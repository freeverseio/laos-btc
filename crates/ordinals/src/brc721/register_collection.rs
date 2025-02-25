use bitcoin::{
	opcodes,
	script::{self, Instruction},
	ScriptBuf, Transaction,
};
use serde::{Deserialize, Serialize};
use sp_core::H160;
use thiserror::Error;

pub const COLLECTION_ADDRESS_LENGTH: usize = 20;
const REBASEABLE_LENGTH: usize = 1;
const PAYLOAD_LENGTH: usize = COLLECTION_ADDRESS_LENGTH + REBASEABLE_LENGTH;
const REGISTER_COLLECTION_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RegisterCollection {
	pub address: H160,
	pub rebaseable: bool,
}

impl RegisterCollection {
	pub fn encode(&self) -> ScriptBuf {
		let address: &script::PushBytes =
			self.address.as_bytes().try_into().expect("Conversion failed");
		let rebaseable = [self.rebaseable as u8];

		script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice(address)
			.push_slice(&rebaseable)
			.into_script()
	}

	pub fn decode(script: &ScriptBuf) -> Result<Self, RegisterCollectionError> {
		let mut instructions = script.instructions();
		match instructions
			.next()
			.ok_or(RegisterCollectionError::InstructionNotFound("OP_RETURN".into()))?
		{
			Ok(Instruction::Op(opcodes::all::OP_RETURN)) => {},
			_ => return Err(RegisterCollectionError::UnexpectedInstruction),
		}

		match instructions.next().ok_or(RegisterCollectionError::InstructionNotFound(
			"REGISTER_COLLECTION_CODE".into(),
		))? {
			Ok(Instruction::Op(REGISTER_COLLECTION_CODE)) => {},
			_ => return Err(RegisterCollectionError::UnexpectedInstruction),
		}

		// Construct the payload by concatenating remaining data pushes
		let mut payload = Vec::with_capacity(PAYLOAD_LENGTH);

		match instructions
			.next()
			.ok_or(RegisterCollectionError::InstructionNotFound("collection address".into()))?
		{
			Ok(Instruction::PushBytes(push)) if push.len() == COLLECTION_ADDRESS_LENGTH => {
				payload.extend_from_slice(push.as_bytes());
			},
			Ok(Instruction::PushBytes(_)) => {
				return Err(RegisterCollectionError::InvalidLength("collection address".into()));
			},
			_ => return Err(RegisterCollectionError::UnexpectedInstruction),
		}

		match instructions
			.next()
			.ok_or(RegisterCollectionError::InstructionNotFound("rebaseable".into()))?
		{
			Ok(Instruction::PushBytes(push)) if push.len() == REBASEABLE_LENGTH => {
				payload.extend_from_slice(push.as_bytes());
			},
			Ok(Instruction::PushBytes(_)) => {
				return Err(RegisterCollectionError::InvalidLength("rebaseable".into()));
			},
			_ => return Err(RegisterCollectionError::UnexpectedInstruction),
		}

		Ok(Self {
			address: H160::from_slice(&payload[..COLLECTION_ADDRESS_LENGTH]),
			rebaseable: payload[COLLECTION_ADDRESS_LENGTH] > 0, // any value > 0 indicates `true`
		})
	}

	pub fn from_tx(transaction: Transaction) -> Result<Self, RegisterCollectionError> {
		let output = transaction.output.first().ok_or(RegisterCollectionError::OutputNotFound)?;

		RegisterCollection::decode(&output.script_pubkey)
	}
}

#[derive(Debug, Error, PartialEq)]
pub enum RegisterCollectionError {
	#[error("Instruction not found: `{0}`")]
	InstructionNotFound(String),
	#[error("Invalid output")]
	InvalidOutput,
	#[error("Unexpected instruction")]
	UnexpectedInstruction,
	#[error("Invalid lenght: `{0}`")]
	InvalidLength(String),
	#[error("Output not found")]
	OutputNotFound,
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use bitcoin::{absolute::LockTime, transaction::Version, Amount, TxOut};

	use super::*;

	#[test]
	fn decode_transaction_no_output() {
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![],
		};

		assert_eq!(
			RegisterCollection::from_tx(tx).unwrap_err(),
			RegisterCollectionError::OutputNotFound
		);
	}

	#[test]
	fn decode_transaction_with_output_but_no_op_return() {
		let script_buf =
			script::Builder::new().push_opcode(opcodes::all::OP_PUSHNUM_15).into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert_eq!(
			RegisterCollection::from_tx(tx).unwrap_err().to_string(),
			"Unexpected instruction"
		);
	}

	#[test]
	fn decode_transaction_with_op_return_but_wrong_op_code() {
		let script_buf = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(opcodes::all::OP_PUSHNUM_13)
			.into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert_eq!(
			RegisterCollection::from_tx(tx).unwrap_err().to_string(),
			"Unexpected instruction"
		);
	}

	#[test]
	fn decode_transaction_incorrect_address_length() {
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

		assert_eq!(
			RegisterCollection::from_tx(tx).unwrap_err().to_string(),
			"Invalid lenght: `collection address`"
		);
	}

	#[test]
	fn decode_transaction_missing_rebasable() {
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

		assert_eq!(
			RegisterCollection::from_tx(tx).unwrap_err().to_string(),
			"Instruction not found: `rebaseable`"
		);
	}

	#[test]
	fn decode_transaction_extra_push_is_ignored() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x00; REBASEABLE_LENGTH];
		let extra = [0xFF; 1];
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

		let register_collection_decoded = RegisterCollection::from_tx(tx).unwrap();
		assert_eq!(register_collection_decoded.address, address.into());
		assert!(!register_collection_decoded.rebaseable);
	}

	#[test]
	fn encode_decode_register_collection_transaction() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let register_collection =
			RegisterCollection { address: H160::from(address), rebaseable: false };

		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut {
				value: Amount::ZERO,
				script_pubkey: register_collection.clone().encode(),
			}],
		};

		let register_collection_decoded = RegisterCollection::from_tx(tx).unwrap();

		assert_eq!(register_collection, register_collection_decoded);
	}

	#[test]
	fn test_register_collection_encode() {
		let cmd = RegisterCollection::default();
		let buf = cmd.encode();
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f1400000000000000000000000000000000000000000100"
		);

		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: false };
		let buf = cmd.encode();
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f14abcffffffffffffffffffffffffffffffffffcba0100"
		);

		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: true };
		let buf = cmd.encode();
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f14abcffffffffffffffffffffffffffffffffffcba0101"
		);
	}

	#[test]
	fn test_register_collection_decode() {
		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: true };
		let buf = cmd.encode();
		let result = RegisterCollection::decode(&buf).unwrap();
		assert_eq!(cmd, result);
	}
}
