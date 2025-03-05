use crate::brc721::flags::{Brc721Flag, BRC721_FLAG_LENGTH, BRC721_INIT_CODE};
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
	/// The encoded script includes an OP_RETURN opcode, the BRC721_INIT_CODE, the register
	/// collection flag, the collection address, and the rebaseable flag.
	pub fn to_script(&self) -> ScriptBuf {
		let address = self.address.as_fixed_bytes();
		let rebaseable = [self.rebaseable as u8];

		script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterCollection.byte_slice())
			.push_slice(address)
			.push_slice(rebaseable)
			.into_script()
	}

	/// Decodes a Bitcoin script into a `RegisterCollection` instance.
	///
	/// The function checks for the presence of OP_RETURN, BRC721_INIT_CODE, the register collection
	/// flag, a 20-byte collection address, and a 1-byte rebaseable flag in the script.
	pub fn from_script(script: &ScriptBuf) -> Result<Self, RegisterCollectionError> {
		let mut instructions = script.instructions();

		expect_opcode(&mut instructions, opcodes::all::OP_RETURN, "OP_RETURN")?;
		expect_opcode(&mut instructions, BRC721_INIT_CODE, "BRC721_INIT_CODE")?;

		match expect_push_bytes(&mut instructions, BRC721_FLAG_LENGTH, "Register collection flag") {
			Ok(byte) if byte == Brc721Flag::RegisterCollection.byte_slice() => (),
			Err(err) => return Err(err),
			_ => return Err(RegisterCollectionError::UnexpectedInstruction),
		}

		// Expect the collection address (20 bytes)
		let address_bytes =
			expect_push_bytes(&mut instructions, COLLECTION_ADDRESS_LENGTH, "collection address")?;

		// Expect the rebaseable flag (1 byte)
		let rebaseable_bytes =
			expect_push_bytes(&mut instructions, REBASEABLE_LENGTH, "rebaseable")?;

		if rebaseable_bytes[0] > 1 {
			return Err(RegisterCollectionError::UnexpectedInstruction)
		}

		Ok(Self { address: H160::from_slice(&address_bytes), rebaseable: rebaseable_bytes[0] == 1 })
	}
}

/// Custom error type for errors related to register collection operations.
#[derive(Debug, Error, PartialEq)]
pub enum RegisterCollectionError {
	/// An instruction of the expected type was not found in the script.
	#[error("Instruction not found: `{0}`")]
	InstructionNotFound(String),

	/// An unexpected instruction was encountered during decoding.
	#[error("Unexpected instruction")]
	UnexpectedInstruction,

	/// The length of a push operation in the script does not match the expected size.
	#[error("Invalid length: `{0}`")]
	InvalidLength(String),
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
	fn register_collection_encode_encodes_correctly() {
		let cmd = RegisterCollection::default();
		let buf = cmd.to_script();
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f01001400000000000000000000000000000000000000000100"
		);

		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: false };
		let buf = cmd.to_script();
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f010014abcffffffffffffffffffffffffffffffffffcba0100"
		);

		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: true };
		let buf = cmd.to_script();
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f010014abcffffffffffffffffffffffffffffffffffcba0101"
		);
	}

	#[test]
	fn register_collection_decode_decodes_correctly() {
		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: true };
		let buf = cmd.to_script();
		let result = RegisterCollection::from_script(&buf).unwrap();
		assert_eq!(cmd, result);
	}

	#[test]
	fn register_collection_decode_ignores_extra_bytes() {
		let buf = ScriptBuf::from_bytes(
			hex::decode("6a5f010014abcffffffffffffffffffffffffffffffffffcba0101").unwrap(),
		);
		RegisterCollection::from_script(&buf).unwrap();

		let buf = ScriptBuf::from_bytes(
			hex::decode("6a5f010014abcffffffffffffffffffffffffffffffffffcba0101FFFFFF").unwrap(),
		);
		RegisterCollection::from_script(&buf).unwrap();
	}

	#[test]
	fn register_collection_decode_treats_nonzero_as_true() {
		let buf = ScriptBuf::from_bytes(
			hex::decode("6a5f010014abcffffffffffffffffffffffffffffffffffcba0101").unwrap(),
		);
		let rc = RegisterCollection::from_script(&buf).unwrap();
		assert!(rc.rebaseable);

		let buf = ScriptBuf::from_bytes(
			hex::decode("6a5f010014abcffffffffffffffffffffffffffffffffffcba0102").unwrap(),
		);
		let result = RegisterCollection::from_script(&buf);
		assert_eq!(result.unwrap_err(), RegisterCollectionError::UnexpectedInstruction,);

		let buf = ScriptBuf::from_bytes(
			hex::decode("6a5f010014abcffffffffffffffffffffffffffffffffffcba01ff").unwrap(),
		);
		let result = RegisterCollection::from_script(&buf);
		assert_eq!(result.unwrap_err(), RegisterCollectionError::UnexpectedInstruction,);
	}

	#[test]
	fn register_collection_decode_empty_script_returns_error() {
		let script = script::Builder::new().into_script();
		assert_eq!(script.len(), 0);

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("OP_RETURN".to_string())
		);
	}

	#[test]
	fn register_collection_decode_only_op_return_returns_error() {
		let script = script::Builder::new().push_opcode(opcodes::all::OP_RETURN).into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("BRC721_INIT_CODE".to_string())
		);
	}

	#[test]
	fn register_collection_decode_wrong_opcode_returns_error() {
		let script = script::Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_opcode(opcodes::all::OP_PUSHNUM_13) // Wrong opcode
            .into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(result.unwrap_err(), RegisterCollectionError::UnexpectedInstruction);
	}

	#[test]
	fn register_collection_decode_missing_register_collection_flag_returns_error() {
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("Register collection flag".to_string())
		);
	}

	#[test]
	fn register_collection_decode_wrong_flag_returns_error() {
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterOwnership.byte_slice())
			.into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(result.unwrap_err(), RegisterCollectionError::UnexpectedInstruction);
	}

	#[test]
	fn register_collection_decode_missing_address_returns_error() {
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterCollection.byte_slice())
			.into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("collection address".to_string())
		);
	}

	#[test]
	fn register_collection_decode_short_address_returns_error() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH - 1];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterCollection.byte_slice())
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InvalidLength("collection address".to_string())
		);
	}

	#[test]
	fn register_collection_decode_long_address_returns_error() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH + 1];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterCollection.byte_slice())
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InvalidLength("collection address".to_string())
		);
	}

	#[test]
	fn register_collection_decode_missing_rebaseable_returns_error() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterCollection.byte_slice())
			.push_slice::<&script::PushBytes>((&address).into())
			.into_script();

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			RegisterCollectionError::InstructionNotFound("rebaseable".to_string())
		);
	}

	#[test]
	fn register_collection_decode_valid_script_decodes_correctly() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x01; REBASEABLE_LENGTH];
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterCollection.byte_slice())
			.push_slice::<&script::PushBytes>((&address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.into_script();

		let result = RegisterCollection::from_script(&script).unwrap();
		assert_eq!(result.address, H160::from(address));
		assert!(result.rebaseable);
	}

	#[test]
	fn register_collection_decode_ignores_extra_data_after_valid_fields() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x01; REBASEABLE_LENGTH];
		let extra_data = [0xFF; 10]; // Extra data that should be ignored
		let script = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterCollection.byte_slice())
			.push_slice::<&script::PushBytes>((&address).into())
			.push_slice::<&script::PushBytes>((&rebaseable).into())
			.push_slice::<&script::PushBytes>((&extra_data).into())
			.into_script();

		let result = RegisterCollection::from_script(&script).unwrap();
		assert_eq!(result.address, H160::from(address));
		assert!(result.rebaseable);
	}
}
