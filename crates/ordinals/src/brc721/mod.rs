pub mod address_mapping;
pub mod collection;
pub mod collection_id;
pub mod register_collection;

use super::*;

/// The opcode used to identify register collection operations.
pub(crate) const BRC721_INIT_SEQUENCE: [u8; 2] = [0x6a, 0x5f];
/// Constant representing the length of a collection address in bytes.
pub const COLLECTION_ADDRESS_LENGTH: usize = 20;

/// Byte to identify which operation of brc721 is used. Just a fancy name to don't remember the
/// numerical byte of each operation. Only fieldless variants are allowed, leading to a compile
/// error otherwise.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Brc721Operation {
	RegisterCollection = 0x00,
}

impl TryFrom<u8> for Brc721Operation {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0x00 => Ok(Brc721Operation::RegisterCollection),
			_ => Err(()),
		}
	}
}

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


// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_is_brc721_script_valid() {
// 		let script = ScriptBuf::from(vec![opcodes::all::OP_RETURN.to_u8(), 0x5f]); // 0x5f is the
// second byte of BRC721_INIT_SEQUENCE 		assert!(is_brc721_script(&script));
// 	}

// 	#[test]
// 	fn test_is_brc721_script_invalid_first_opcode() {
// 		let script = ScriptBuf::from(vec![
// 			0xbd, // Invalid first opcode (OP_RETURN_189 in hexadecimal)
// 			0x5f, // Second byte of BRC721_INIT_SEQUENCE
// 		]);
// 		assert!(!is_brc721_script(&script));
// 	}

// 	#[test]
// 	fn test_is_brc721_script_invalid_second_opcode() {
// 		let script = ScriptBuf::from(vec![
// 			opcodes::all::OP_RETURN.to_u8(),
// 			0x60, // Invalid second opcode (OP_PUSHNUM_16 in hexadecimal)
// 		]);
// 		assert!(!is_brc721_script(&script));
// 	}

// 	#[test]
// 	fn test_is_brc721_script_too_short() {
// 		let script = ScriptBuf::from(vec![
// 			opcodes::all::OP_RETURN.to_u8(), // Only one byte, too short
// 		]);
// 		assert!(!is_brc721_script(&script));
// 	}

// 	#[test]
// 	fn test_is_brc721_script_too_long() {
// 		let script = ScriptBuf::from(vec![
// 			opcodes::all::OP_RETURN.to_u8(),
// 			0x5f, // Second byte of BRC721_INIT_SEQUENCE
// 			0x54, // Extra byte (OP_PUSHNUM_4 in hexadecimal)
// 		]);
// 		assert!(is_brc721_script(&script)); // The function should only check the first two bytes
// 	}
// }
