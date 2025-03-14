pub mod address_mapping;
pub mod collection;
pub mod collection_id;
pub(crate) mod operations;
pub mod register_collection;

use super::*;

use bitcoin::opcodes;

/// The opcode used to identify register collection operations.
pub(crate) const BRC721_INIT_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

/// Checks if a given script is a BRC721 script.
pub fn is_brc721_script(script: &ScriptBuf) -> bool {
	// Create an iterator for the instructions in the script.
	let mut instructions = script.instructions().peekable();

	// Expect the first instruction to be OP_RETURN, which signals the start of a script.
	match instructions.next() {
		Some(Ok(Instruction::Op(op))) if op == bitcoin::opcodes::all::OP_RETURN => (),
		_ => return false, // If it's not OP_RETURN, return false.
	}

	// Check for the next instruction to see if it matches the BRC721 initialization code.
	matches!(instructions.next(), Some(Ok(Instruction::Op(op))) if op == BRC721_INIT_CODE)
}

/// Tests for the BRC721 script functions.
#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::{blockdata::script::Builder, opcodes};

	#[test]
	fn test_valid_brc721_script() {
		let script = Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.into_script();

		assert!(is_brc721_script(&script));
	}

	#[test]
	fn test_invalid_script_op_return_missing() {
		// Create a script without OP_RETURN
		let script = Builder::new()
            .push_opcode(BRC721_INIT_CODE) // Invalid because it must start with OP_RETURN
            .into_script();

		assert!(!is_brc721_script(&script));
	}

	#[test]
	fn test_invalid_script_wrong_op_return() {
		// Create a script that has OP_RETURN but doesn't follow it with BRC721_INIT_CODE
		let script = Builder::new()
            .push_opcode(opcodes::all::OP_RETURN)
            .push_opcode(opcodes::all::OP_PUSHNUM_14) // Wrong opcode
            .into_script();

		assert!(!is_brc721_script(&script));
	}

	#[test]
	fn test_invalid_script_empty() {
		// Test an empty script
		let script = Builder::new().into_script();

		assert!(!is_brc721_script(&script));
	}
}
