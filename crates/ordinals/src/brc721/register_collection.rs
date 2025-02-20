#![allow(dead_code)] // TODO remove when it is used
use bitcoin::{
	absolute::LockTime,
	opcodes,
	script::{self, Instruction},
	transaction::Version,
	Amount, ScriptBuf, Transaction, TxOut,
};
use serde::{Deserialize, Serialize};
use sp_core::H160;
use thiserror::Error;

const COLLECTION_ADDRESS_LENGTH: usize = 20;
const REBASEABLE_LENGTH: usize = 1;
const PAYLOAD_LENGTH: usize = COLLECTION_ADDRESS_LENGTH + REBASEABLE_LENGTH;
const REGISTER_COLLECTION_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RegisterCollection {
	pub address: H160,
	pub rebaseable: bool,
}

type Payload = [u8; PAYLOAD_LENGTH];

// risger collection
// payload

// script buf includes payload (and register)
// transaction includes script

impl From<RegisterCollection> for ScriptBuf {
	fn from(register_collection: RegisterCollection) -> Self {
		let mut builder = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE);

		let address: &script::PushBytes =
			register_collection.address.as_bytes().try_into().expect("Conversion failed");
		let rebaseable: [u8; 1] = if register_collection.rebaseable { [1] } else { [0] };

		builder = builder.push_slice(address);
		builder = builder.push_slice::<&script::PushBytes>((&rebaseable).into());

		builder.into_script()
	}
}

// // Check that the first instruction is OP_RETURN.
// match instructions.next().ok_or(RegisterCollectionError::NoOutput)? {
//     Ok(Instruction::Op(opcodes::all::OP_RETURN)) => {},
//     _ => return Err(RegisterCollectionError::InvalidScript),
// }

// // Check that the second instruction is REGISTER_COLLECTION_CODE.
// match instructions.next().ok_or(RegisterCollectionError::MissingOpcode)? {
//     Ok(Instruction::Op(REGISTER_COLLECTION_CODE)) => {},
//     _ => return Err(RegisterCollectionError::InvalidScript),
// }

// // Now create a fixed-capacity payload container (using Vec or a fixed array).
// let mut payload = Vec::with_capacity(PAYLOAD_LENGTH);

// // Expect the next instruction to be the collection address.
// match instructions.next().ok_or(RegisterCollectionError::MissingCollectionAddress)? {
//     Ok(Instruction::PushBytes(push)) if push.len() == COLLECTION_ADDRESS_LENGTH => {
//         payload.extend_from_slice(push.as_bytes());
//     },
//     Ok(Instruction::PushBytes(push)) => {
//         log::warn!("Invalid address length: {}", push.len());
//         return Err(RegisterCollectionError::InvalidAddressLength);
//     },
//     _ => {
//         log::warn!("REGISTER_COLLECTION_CODE found but not followed by push bytes instruction for collection address");
//         return Err(RegisterCollectionError::InvalidScript);
//     }
// }

// // Expect the next instruction to be the rebaseable flag.
// match instructions.next().ok_or(RegisterCollectionError::MissingRebaseable)? {
//     Ok(Instruction::PushBytes(push)) if push.len() == REBASEABLE_LENGTH => {
//         payload.extend_from_slice(push.as_bytes());
//     },
//     Ok(Instruction::PushBytes(push)) => {
//         log::warn!("Invalid rebaseable length: {}", push.len());
//         return Err(RegisterCollectionError::InvalidRebaseableLength);
//     },
//     _ => {
//         log::warn!("REGISTER_COLLECTION_CODE not followed by push bytes instruction for rebaseable flag");
//         return Err(RegisterCollectionError::InvalidScript);
//     }
// }

#[derive(Debug, Error)]
pub enum RegisterCollectionError {
	#[error("Instruction expected")]
	NoInstruction,
	#[error("Invalid output")]
	InvalidOutput,
	#[error("Invalid OP_CODE")]
	InvalidOpcode,
	// Add other error variants as needed.
}

impl TryFrom<Transaction> for RegisterCollection {
	type Error = RegisterCollectionError;

	fn try_from(transaction: Transaction) -> Result<Self, Self::Error> {
		let output = transaction
			.output
			.first()
			.ok_or(RegisterCollectionError::NoInstruction)
			.unwrap();
		let mut instructions = output.script_pubkey.instructions();

		match instructions.next().ok_or(RegisterCollectionError::NoInstruction)? {
			Ok(Instruction::Op(opcodes::all::OP_RETURN)) => {},
			_ => return Err(RegisterCollectionError::NoInstruction),
		}

		match instructions.next().ok_or(RegisterCollectionError::NoInstruction)? {
			Ok(Instruction::Op(REGISTER_COLLECTION_CODE)) => {},
			_ => return Err(RegisterCollectionError::MissingOpcode),
		}

		if instructions.next()? != Ok(Instruction::Op(REGISTER_COLLECTION_CODE)) {
			return None;
		}

		// Construct the payload by concatenating remaining data pushes
		let mut payload = Vec::with_capacity(PAYLOAD_LENGTH);

		// Expect the first push to be the collection address
		if let Some(Ok(Instruction::PushBytes(push))) = instructions.next() {
			if push.len() == COLLECTION_ADDRESS_LENGTH {
				payload.extend_from_slice(push.as_bytes());
			} else {
				log::warn!("Invalid address length: {}", push.len());
				return None;
			}
		} else {
			log::warn!("REGISTER_COLLECTION_CODE found but not followed by push bytes instruction");
			return None;
		}

		// Expect the second push to be the rebaseable flag
		if let Some(Ok(Instruction::PushBytes(push))) = instructions.next() {
			if push.len() == REBASEABLE_LENGTH {
				payload.extend_from_slice(push.as_bytes());
			} else {
				log::warn!("Invalid rebasable length {}", push.len());
				return None;
			}
		} else {
			log::warn!("REGISTER_COLLECTION_CODE followed by push byte instruction for collection addres but not followed by push bytes instruction for rebaseable");
			return None;
		}

		payload.try_into().ok()
	}
}

impl RegisterCollection {
	pub fn encipher(&self) -> ScriptBuf {
		let mut builder = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE);

		let address: &script::PushBytes =
			self.address.as_bytes().try_into().expect("Conversion failed");
		let rebaseable: [u8; 1] = if self.rebaseable { [1] } else { [0] };

		builder = builder.push_slice(address);
		builder = builder.push_slice::<&script::PushBytes>((&rebaseable).into());

		builder.into_script()
	}

	pub fn decipher(transaction: &Transaction) -> Option<RegisterCollection> {
		let payload = RegisterCollection::payload(transaction)?;
		Some(Self::from_payload(payload))
	}

	fn payload(transaction: &Transaction) -> Option<Payload> {
		let output = transaction.output.first()?;
		let mut instructions = output.script_pubkey.instructions();

		if instructions.next()? != Ok(Instruction::Op(opcodes::all::OP_RETURN)) {
			return None;
		}
		if instructions.next()? != Ok(Instruction::Op(REGISTER_COLLECTION_CODE)) {
			return None;
		}

		// Construct the payload by concatenating remaining data pushes
		let mut payload = Vec::with_capacity(PAYLOAD_LENGTH);

		// Expect the first push to be the collection address
		if let Some(Ok(Instruction::PushBytes(push))) = instructions.next() {
			if push.len() == COLLECTION_ADDRESS_LENGTH {
				payload.extend_from_slice(push.as_bytes());
			} else {
				log::warn!("Invalid address length: {}", push.len());
				return None;
			}
		} else {
			log::warn!("REGISTER_COLLECTION_CODE found but not followed by push bytes instruction");
			return None;
		}

		// Expect the second push to be the rebaseable flag
		if let Some(Ok(Instruction::PushBytes(push))) = instructions.next() {
			if push.len() == REBASEABLE_LENGTH {
				payload.extend_from_slice(push.as_bytes());
			} else {
				log::warn!("Invalid rebasable length {}", push.len());
				return None;
			}
		} else {
			log::warn!("REGISTER_COLLECTION_CODE followed by push byte instruction for collection addres but not followed by push bytes instruction for rebaseable");
			return None;
		}

		payload.try_into().ok()
	}

	fn from_payload(payload: Payload) -> Self {
		Self {
			address: H160::from_slice(&payload[..COLLECTION_ADDRESS_LENGTH]),
			rebaseable: payload[COLLECTION_ADDRESS_LENGTH] > 0, // any value > 0 indicates `true`
		}
	}
}

#[cfg(test)]
mod tests {
	use bitcoin::{absolute::LockTime, transaction::Version, Amount, TxOut};
	use std::str::FromStr;

	use super::*;

	#[test]
	fn encipher_and_decipher() {
		let alice = H160::from([0; 20]);
		let register_collection = RegisterCollection { address: alice, rebaseable: false };
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut {
				value: Amount::from_sat(0),
				script_pubkey: register_collection.encipher(),
			}],
		};

		let deciphered = RegisterCollection::decipher(&tx).unwrap();
		assert_eq!(deciphered, register_collection);
	}

	#[test]
	fn payload_no_output() {
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_with_output_but_no_op_return() {
		let script_buf =
			script::Builder::new().push_opcode(opcodes::all::OP_PUSHNUM_15).into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_with_op_return_but_wrong_op_code() {
		let script_buf = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(opcodes::all::OP_PUSHNUM_15)
			.into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_incorrect_address_length() {
		let wrong_address = [0xBB; COLLECTION_ADDRESS_LENGTH + 10];
		let rebaseable = [0x00; REBASEABLE_LENGTH];
		let script_buf = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&wrong_address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_missing_rebasable() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let script_buf = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_extra_push_is_ignored() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x00; REBASEABLE_LENGTH];
		let extra = [0xFF; 10];
		let script_buf = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.push_slice::<&script::PushBytes>((&extra).into())
			.into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		let payload = RegisterCollection::payload(&tx).unwrap();
		assert_eq!(payload[..COLLECTION_ADDRESS_LENGTH], address);
		assert_eq!(payload[COLLECTION_ADDRESS_LENGTH], rebaseable[0]);
	}

	#[test]
	fn from_payload() {
		let address = [0xEE; COLLECTION_ADDRESS_LENGTH];
		let rebaseable_flag = 0x10;
		let mut payload = [0u8; PAYLOAD_LENGTH];
		payload[..COLLECTION_ADDRESS_LENGTH].copy_from_slice(&address);
		payload[COLLECTION_ADDRESS_LENGTH] = rebaseable_flag;

		let register_collection = RegisterCollection::from_payload(payload);
		assert_eq!(register_collection.address, address.into());
		assert!(register_collection.rebaseable);
	}

	#[test]
	fn from_payload_address_contains_invalid_hex_char() {
		let address = [b'z'; COLLECTION_ADDRESS_LENGTH];
		let rebaseable_flag = 0x10;
		let mut payload = [0u8; PAYLOAD_LENGTH];
		payload[..COLLECTION_ADDRESS_LENGTH].copy_from_slice(&address);
		payload[COLLECTION_ADDRESS_LENGTH] = rebaseable_flag;
		let register_collection = RegisterCollection::from_payload(payload);
		assert_eq!(
			register_collection.address,
			H160::from_str("0x7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a7a").unwrap()
		);
		assert!(register_collection.rebaseable);
	}
}
