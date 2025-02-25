use bitcoin::{
	opcodes,
	script::{self, Instruction},
	ScriptBuf,
};
use serde::{Deserialize, Serialize};
use sp_core::H160;
use thiserror::Error;

pub const COLLECTION_ADDRESS_LENGTH: usize = 20;
const REBASEABLE_LENGTH: usize = 1;
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

		expect_opcode(&mut instructions, opcodes::all::OP_RETURN, "OP_RETURN")?;
		expect_opcode(&mut instructions, REGISTER_COLLECTION_CODE, "REGISTER_COLLECTION_CODE")?;
		// Expect the collection address (20 bytes)
		let address_bytes =
			expect_push_bytes(&mut instructions, COLLECTION_ADDRESS_LENGTH, "collection address")?;
		// Expect the rebaseable flag (1 byte)
		let rebaseable_bytes =
			expect_push_bytes(&mut instructions, REBASEABLE_LENGTH, "rebaseable")?;

		Ok(Self {
			address: H160::from_slice(&address_bytes),
			rebaseable: rebaseable_bytes[0] > 0, /* any nonzero value is `true` TODO check the
			                                      * whitepaper */
		})
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

/// Helper to ensure the next instruction is a specific opcode.
fn expect_opcode<'a>(
	instructions: &mut impl Iterator<Item = Result<Instruction<'a>, bitcoin::script::Error>>,
	expected_op: opcodes::Opcode,
	desc: &str,
) -> Result<(), RegisterCollectionError> {
	match instructions
		.next()
		.ok_or_else(|| RegisterCollectionError::InstructionNotFound(desc.into()))?
	{
		Ok(Instruction::Op(op)) if op == expected_op => Ok(()),
		_ => Err(RegisterCollectionError::UnexpectedInstruction),
	}
}

fn expect_push_bytes<'a>(
	instructions: &mut impl Iterator<Item = Result<Instruction<'a>, bitcoin::script::Error>>,
	expected_len: usize,
	desc: &str,
) -> Result<Vec<u8>, RegisterCollectionError> {
	match instructions
		.next()
		.ok_or_else(|| RegisterCollectionError::InstructionNotFound(desc.into()))?
	{
		Ok(Instruction::PushBytes(bytes)) if bytes.len() == expected_len =>
			Ok(bytes.as_bytes().into()),
		Ok(Instruction::PushBytes(_)) => Err(RegisterCollectionError::InvalidLength(desc.into())),
		_ => Err(RegisterCollectionError::UnexpectedInstruction),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;

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

	#[test]
	fn test_decode_empty_script() {
		let script = script::Builder::new().into_script();

		assert_eq!(script.len(), 0);

		let result = RegisterCollection::decode(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("OP_RETURN".to_string())
		);
	}

	#[test]
	fn test_decode_script_with_only_op_return() {
		let script = script::Builder::new().push_opcode(opcodes::all::OP_RETURN).into_script();

		let result = RegisterCollection::decode(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("REGISTER_COLLECTION_CODE".to_string())
		);
	}

	#[test]
	fn test_decode_script_with_op_return_and_wrong_opcode() {
		let script = script::Builder::new()
        .push_opcode(opcodes::all::OP_RETURN)
        .push_opcode(opcodes::all::OP_PUSHNUM_13) // Wrong opcode
        .into_script();

		let result = RegisterCollection::decode(&script);
		assert_eq!(result.unwrap_err(), RegisterCollectionError::UnexpectedInstruction);
	}

	#[test]
	fn test_decode_script_with_op_return_and_correct_opcode_but_no_address() {
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.into_script();

		let result = RegisterCollection::decode(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("collection address".to_string())
		);
	}

	#[test]
	fn test_decode_script_with_op_return_correct_opcode_and_short_address() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH - 1];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();

		let result = RegisterCollection::decode(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InvalidLength("collection address".to_string())
		);
	}

	#[test]
	fn test_decode_script_with_op_return_correct_opcode_and_long_address() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH + 1];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();

		let result = RegisterCollection::decode(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InvalidLength("collection address".to_string())
		);
	}

	#[test]
	fn test_decode_script_with_op_return_correct_opcode_and_address_but_no_rebaseable() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();

		let result = RegisterCollection::decode(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("rebaseable".to_string())
		);
	}

	#[test]
	fn test_decode_script_with_op_return_correct_opcode_address_and_rebaseable() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x01; REBASEABLE_LENGTH];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.into_script();

		let result = RegisterCollection::decode(&script).unwrap();
		assert_eq!(result.address, H160::from(address));
		assert!(result.rebaseable);
	}

	#[test]
	fn test_decode_script_with_op_return_correct_opcode_address_and_rebaseable_extra_data() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x01; REBASEABLE_LENGTH];
		let extra_data = [0xFF; 10]; // Extra data that should be ignored
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice::<&script::PushBytes>((&address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.push_slice::<&script::PushBytes>((&extra_data).into())
			.into_script();

		let result = RegisterCollection::decode(&script).unwrap();
		assert_eq!(result.address, H160::from(address));
		assert!(result.rebaseable);
	}
}
