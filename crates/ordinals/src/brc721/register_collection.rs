use bitcoin::{
	opcodes,
	script::{self, Instruction},
	ScriptBuf,
};
use serde::{Deserialize, Serialize};
use sp_core::H160;
use thiserror::Error;

/// Constant representing the length of a collection address in bytes.
pub const COLLECTION_ADDRESS_LENGTH: usize = 20;

/// Constant representing the length of the rebaseable flag in bytes.
const REBASEABLE_LENGTH: usize = 1;

/// The opcode used to identify register collection operations.
const REGISTER_COLLECTION_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

/// Struct to represent a register collection with an address and rebaseability status.
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RegisterCollection {
	/// The 20-byte Ethereum-style address of the collection.
	pub address: H160,

	/// A boolean flag indicating whether the collection is rebaseable.
	pub rebaseable: bool,
}

impl RegisterCollection {
	/// Encodes a `RegisterCollection` instance into a Bitcoin script.
	///
	/// The encoded script includes an OP_RETURN opcode, the REGISTER_COLLECTION_CODE,
	/// the collection address, and the rebaseable flag.
	pub fn encode(&self) -> ScriptBuf {
		let address: &script::PushBytes =
			self.address.as_bytes().try_into().expect("Conversion failed");
		let rebaseable = [self.rebaseable as u8];

		script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE)
			.push_slice(address)
			.push_slice(rebaseable)
			.into_script()
	}

	/// Decodes a Bitcoin script into a `RegisterCollection` instance.
	///
	/// The function checks for the presence of OP_RETURN, REGISTER_COLLECTION_CODE,
	/// a 20-byte collection address, and a 1-byte rebaseable flag in the script.
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
			rebaseable: rebaseable_bytes[0] > 0, /* any nonzero value is `true` */
		})
	}
}

/// Custom error type for errors related to register collection operations.
#[derive(Debug, Error, PartialEq)]
pub enum RegisterCollectionError {
	/// An instruction of the expected type was not found in the script.
	#[error("Instruction not found: `{0}`")]
	InstructionNotFound(String),

	/// The output does not match the expected format or content.
	#[error("Invalid output")]
	InvalidOutput,

	/// An unexpected instruction was encountered during decoding.
	#[error("Unexpected instruction")]
	UnexpectedInstruction,

	/// The length of a push operation in the script does not match the expected size.
	#[error("Invalid length: `{0}`")]
	InvalidLength(String),

	/// No output was found where one was expected.
	#[error("Output not found")]
	OutputNotFound,
}

/// Helper function to ensure the next instruction is a specific opcode.
///
/// Returns an error if the expected opcode is not found or if there are no more instructions.
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

/// Helper function to ensure the next instruction is a push operation of the expected length.
///
/// Returns an error if the expected length is not met or if there are no more instructions.
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
