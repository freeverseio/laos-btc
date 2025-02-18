// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use message::Message;

mod message;

pub const COLLECTION_ADDRESS_LENGTH: usize = 20;
pub const REBASEABLE_LENGTH: usize = 1;
pub const PAYLOAD_LENGTH: usize = COLLECTION_ADDRESS_LENGTH + REBASEABLE_LENGTH;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LaosCollection {
	pub message: Message,
}

pub type Payload = [u8; PAYLOAD_LENGTH];

impl LaosCollection {
	pub const MAGIC_NUMBER: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;
	pub const COMMIT_CONFIRMATIONS: u16 = 6;

	pub fn decipher(transaction: &Transaction) -> Option<LaosCollection> {
		let payload = LaosCollection::payload(transaction)?;

		let message = Message::from_payload(payload);

		Some(Self { message })
	}

	pub fn encipher(&self) -> ScriptBuf {
		let mut builder = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(LaosCollection::MAGIC_NUMBER);

		let address_collection: &script::PushBytes = (&self.message.address_collection).into();
		let rebaseable: [u8; 1] = if self.message.rebaseable { [1] } else { [0] };

		builder = builder.push_slice(address_collection);
		builder = builder.push_slice::<&script::PushBytes>((&rebaseable).into());

		builder.into_script()
	}

	fn payload(transaction: &Transaction) -> Option<Payload> {
		// search transaction outputs for payload
		for output in &transaction.output {
			let mut instructions = output.script_pubkey.instructions();

			match instructions.next() {
				Some(Ok(Instruction::Op(opcodes::all::OP_RETURN)))
					if instructions.next() ==
						Some(Ok(Instruction::Op(LaosCollection::MAGIC_NUMBER))) =>
				{
					// construct the payload by concatenating remaining data pushes
					let mut payload = Vec::with_capacity(PAYLOAD_LENGTH);

					for result in instructions {
						match result {
							// Collection address
							Ok(Instruction::PushBytes(push))
								if push.len() == COLLECTION_ADDRESS_LENGTH &&
									payload.is_empty() =>
							{
								payload.extend_from_slice(push.as_bytes());
							},
							// Rebaseable
							Ok(Instruction::PushBytes(push))
								if push.len() == REBASEABLE_LENGTH &&
									payload.len() == COLLECTION_ADDRESS_LENGTH =>
							{
								payload.extend_from_slice(push.as_bytes());
							},
							_ => return None,
						}
					}
					return payload.try_into().ok();
				},
				_ => continue,
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::{
		absolute::LockTime, blockdata::transaction::Version, opcodes, script::Builder, Amount,
		Transaction, TxOut,
	};

	/// Happy path: Encode a `LaosCollection` into a script and then decode it back.
	#[test]
	fn test_encipher_decipher() {
		let address = [0xAA; COLLECTION_ADDRESS_LENGTH];
		let message = Message { address_collection: address, rebaseable: true };
		let collection = LaosCollection { message };

		// Encode the collection into a script.
		let script_buf = collection.encipher();

		// Create a transaction with a single output that contains the script.
		let tx = Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![TxOut { value: Amount::ONE_SAT, script_pubkey: script_buf }],
		};

		// Decode the transaction and verify that the decoded collection matches the original.
		let decoded = LaosCollection::decipher(&tx).expect("Valid payload expected");
		assert_eq!(decoded, collection);
	}

	/// Error: Transaction does not contain a script with the expected payload (script does not
	/// start with OP_RETURN).
	#[test]
	fn test_decipher_no_payload() {
		// Build a script that does not start with OP_RETURN.
		let script_buf = Builder::new().push_opcode(opcodes::all::OP_PUSHNUM_15).into_script();

		let tx = Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![TxOut { value: Amount::ONE_SAT, script_pubkey: script_buf }],
		};

		// Decipher should return None due to the missing proper payload header.
		assert!(LaosCollection::decipher(&tx).is_none());
	}

	/// Error: The payload contains an address push with an incorrect length.
	#[test]
	fn test_decipher_incorrect_address_length() {
		// Build a script with a valid header but an address of 19 bytes instead of 20.
		let wrong_address = [0xBB; COLLECTION_ADDRESS_LENGTH - 1];
		let rebaseable = [0x01; REBASEABLE_LENGTH];
		let script_buf = Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(LaosCollection::MAGIC_NUMBER)
			.push_slice::<&script::PushBytes>((&wrong_address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.into_script();

		let tx = Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![TxOut { value: Amount::ONE_SAT, script_pubkey: script_buf }],
		};

		// Decipher should return None because the address length is incorrect.
		assert!(LaosCollection::decipher(&tx).is_none());
	}

	/// Error: The payload is incomplete (missing the rebaseable flag push).
	#[test]
	fn test_decipher_missing_rebaseable() {
		// Build a script with a valid header and correct address, but missing the rebaseable push.
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let script_buf = Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(LaosCollection::MAGIC_NUMBER)
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();

		let tx = Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![TxOut { value: Amount::ONE_SAT, script_pubkey: script_buf }],
		};

		// Decipher should return None because the rebaseable flag is missing.
		assert!(LaosCollection::decipher(&tx).is_none());
	}

	/// Error: The payload contains extra data (more pushes than expected).
	#[test]
	fn test_decipher_extra_push() {
		// Build a script with the correct payload and add an extra push.
		let address = [0xDD; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x00; REBASEABLE_LENGTH];
		let extra = [0xFF; 10];
		let script_buf = Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(LaosCollection::MAGIC_NUMBER)
			.push_slice::<&script::PushBytes>((&address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.push_slice::<&script::PushBytes>((&extra).into())
			.into_script();

		let tx = Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![TxOut { value: Amount::ONE_SAT, script_pubkey: script_buf }],
		};

		// Decipher should return None due to the extra data push.
		assert!(LaosCollection::decipher(&tx).is_none());
	}
}
