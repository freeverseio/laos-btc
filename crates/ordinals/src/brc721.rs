pub mod address_mapping;
pub mod bitcoin_script;
pub mod collection;
pub mod collection_id;
pub mod operations;
pub mod register_collection;
pub mod register_ownership;
pub mod token;
pub mod token_id;
pub mod token_id_range;

pub use token_id_range::TokenIdRange;

use super::*;

use bitcoin::opcodes;

/// The opcode used to identify register collection operations.
pub(crate) const BRC721_INIT_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

/// Checks if a given script is a BRC721 script.
pub fn is_brc721_script(script: &ScriptBuf) -> bool {
	let buffer = script.as_bytes();
	if buffer.len() < 2 {
		return false;
	}

	buffer[0..2] == [opcodes::all::OP_RETURN.to_u8(), BRC721_INIT_CODE.to_u8()]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_brc721_script_valid() {
		let script =
			ScriptBuf::from(vec![opcodes::all::OP_RETURN.to_u8(), BRC721_INIT_CODE.to_u8()]);
		assert!(is_brc721_script(&script));
	}

	#[test]
	fn test_is_brc721_script_invalid_first_opcode() {
		let script = ScriptBuf::from(vec![
			opcodes::all::OP_RETURN_189.to_u8(), // Invalid first opcode
			BRC721_INIT_CODE.to_u8(),
		]);
		assert!(!is_brc721_script(&script));
	}

	#[test]
	fn test_is_brc721_script_invalid_second_opcode() {
		let script = ScriptBuf::from(vec![
			opcodes::all::OP_RETURN.to_u8(),
			opcodes::all::OP_PUSHNUM_16.to_u8(), // Invalid second opcode
		]);
		assert!(!is_brc721_script(&script));
	}

	#[test]
	fn test_is_brc721_script_too_short() {
		let script = ScriptBuf::from(vec![
			opcodes::all::OP_RETURN.to_u8(), // Only one byte, too short
		]);
		assert!(!is_brc721_script(&script));
	}

	#[test]
	fn test_is_brc721_script_too_long() {
		let script = ScriptBuf::from(vec![
			opcodes::all::OP_RETURN.to_u8(),
			BRC721_INIT_CODE.to_u8(),
			opcodes::all::OP_PUSHNUM_4.to_u8(), // Extra byte, too long
		]);
		assert!(is_brc721_script(&script)); // The function should only check the first two bytes
	}
}
