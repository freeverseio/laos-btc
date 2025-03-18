pub mod address_mapping;
pub mod collection;
pub mod collection_id;
mod operations;
pub mod register_collection;
use opcodes::all::{OP_PUSHBYTES_15, OP_RETURN};

pub use operations::Brc721Operation;

use super::*;

/// The opcode used to identify register collection operations.
pub(crate) const BRC721_INIT_SEQUENCE: [u8; 2] = [OP_RETURN.to_u8(), OP_PUSHBYTES_15.to_u8()];
/// Constant representing the length of a collection address in bytes.
pub const COLLECTION_ADDRESS_LENGTH: usize = 20;

pub fn get_operation(script: &ScriptBuf) -> Option<Brc721Operation> {
	let buffer = script.as_bytes();

	if buffer.len() < 3 {
		return None;
	}

	if buffer[0..2] != BRC721_INIT_SEQUENCE {
		return None;
	}

	Some(Brc721Operation::try_from(buffer[2]).unwrap())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_operation_valid() {
		let script = ScriptBuf::from(vec![
			OP_RETURN.to_u8(), // First byte of BRC721_INIT_SEQUENCE
			0x5f,              // Second byte of BRC721_INIT_SEQUENCE
			0x00,              // Operation byte for RegisterCollection
		]);
		assert_eq!(get_operation(&script), Some(Brc721Operation::RegisterCollection));
	}

	#[test]
	fn test_get_operation_invalid_first_opcode() {
		let script = ScriptBuf::from(vec![
			0xbd, // Invalid first opcode (OP_RETURN_189 in hexadecimal)
			0x5f, // Second byte of BRC721_INIT_SEQUENCE
			0x00, // Operation byte for RegisterCollection
		]);
		assert_eq!(get_operation(&script), None);
	}

	#[test]
	fn test_get_operation_invalid_second_opcode() {
		let script = ScriptBuf::from(vec![
			OP_RETURN.to_u8(),
			0x60, // Invalid second opcode (OP_PUSHNUM_16 in hexadecimal)
			0x00, // Operation byte for RegisterCollection
		]);
		assert_eq!(get_operation(&script), None);
	}

	#[test]
	fn test_get_operation_too_short() {
		let script = ScriptBuf::from(vec![
			OP_RETURN.to_u8(), // Only one byte, too short
		]);
		assert_eq!(get_operation(&script), None);
	}
}
