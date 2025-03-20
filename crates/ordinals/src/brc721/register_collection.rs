use crate::brc721::operations::Brc721Operation;
use bitcoin::{
	opcodes::all::{OP_PUSHNUM_15, OP_RETURN},
	script, ScriptBuf,
};
use serde::{Deserialize, Serialize};
use sp_core::H160;

use super::bitcoin_script::{expect_opcode, expect_push_bytes, BitcoinScriptError};

/// Constant representing the length of a collection address in bytes.
pub const COLLECTION_ADDRESS_LENGTH: usize = 20;

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
		let mut script = ScriptBuf::new();
		script.push_opcode(OP_RETURN);
		script.push_opcode(OP_PUSHNUM_15);
		let mut buffer = Vec::new();
		buffer.push(Brc721Operation::RegisterCollection as u8);
		buffer.extend_from_slice(self.address.as_bytes());
		buffer.push(self.rebaseable as u8);
		let buffer: &script::PushBytes = buffer.as_slice().try_into().unwrap();
		script.push_slice(buffer);
		script
	}

	/// Decodes a Bitcoin script into a `RegisterCollection` instance.
	///
	/// The function checks for the presence of OP_RETURN, BRC721_INIT_CODE, the register collection
	/// flag, a 20-byte collection address, and a 1-byte rebaseable flag in the script.
	pub fn from_script(script: &ScriptBuf) -> Result<Self, BitcoinScriptError> {
		let mut instructions = script.instructions();

		expect_opcode(&mut instructions, OP_RETURN, "OP_RETURN")?;
		expect_opcode(&mut instructions, OP_PUSHNUM_15, "BRC721_INIT_CODE")?;

		let buffer = expect_push_bytes(&mut instructions, None, "Register ownership operation")?;

		if buffer.len() < 22 {
			return Err(BitcoinScriptError::InvalidLength("short script".to_string()));
		}

		if buffer[0] != Brc721Operation::RegisterCollection as u8 {
			return Err(BitcoinScriptError::UnexpectedInstruction);
		}

		let address = H160::from_slice(&buffer[1..21]);
		let flags = buffer[21];

		if flags != 0x00 && flags != 0x01 {
			return Err(BitcoinScriptError::UnexpectedInstruction);
		}

		let rebaseable = flags == 0x01;

		Ok(Self { address, rebaseable })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;

	#[test]
	fn test_serialized_default() {
		let cmd = RegisterCollection::default();
		let buf = cmd.to_script();
		assert_eq!(buf.len(), 25);
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f1600000000000000000000000000000000000000000000"
		);
	}

	#[test]
	fn register_collection_serialize_correctly() {
		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: true };
		let buf = cmd.to_script();
		assert_eq!(
			hex::encode(buf.into_bytes()),
			"6a5f1600abcffffffffffffffffffffffffffffffffffcba01"
		);
	}

	#[test]
	fn register_collection_deserialize_correctly() {
		let address = H160::from_str("0xabcffffffffffffffffffffffffffffffffffcba").unwrap();
		let cmd = RegisterCollection { address, rebaseable: true };
		let buf = cmd.to_script();
		let result = RegisterCollection::from_script(&buf).unwrap();
		assert_eq!(cmd, result);
	}

	#[test]
	fn register_collection_decode_ignores_extra_bytes() {
		let buf = ScriptBuf::from_bytes(
			hex::decode("6a5f1600abcffffffffffffffffffffffffffffffffffcba01FFFF").unwrap(),
		);
		RegisterCollection::from_script(&buf).unwrap();
	}

	#[test]
	fn register_collection_decode_empty_script_returns_error() {
		let script = script::Builder::new().into_script();
		assert_eq!(script.len(), 0);

		let result = RegisterCollection::from_script(&script);
		assert_eq!(
			result.unwrap_err(),
			BitcoinScriptError::InvalidLength("script is too short".to_string())
		);
	}
}
