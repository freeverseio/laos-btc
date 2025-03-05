use bitcoin::{opcodes, script::Instruction};
use thiserror::Error;

/// Custom error type for errors related to bitcoin script operations.
#[derive(Debug, Error, PartialEq)]
pub enum BitcoinScriptError {
	/// An instruction of the expected type was not found in the script.
	#[error("Instruction not found: `{0}`")]
	InstructionNotFound(String),

	/// An unexpected instruction was encountered during decoding.
	#[error("Unexpected instruction")]
	UnexpectedInstruction,

	/// The length of a push operation in the script does not match the expected size.
	#[error("Invalid length: `{0}`")]
	InvalidLength(String),

	/// An error occurred during decoding.
	#[error("Decoding error: `{0}`")]
	Decode(String),
}

/// Helper function to ensure the next instruction is a specific opcode.
///
/// Returns an error if the expected opcode is not found or if there are no more instructions.
pub fn expect_opcode<'a>(
	instructions: &mut impl Iterator<Item = Result<Instruction<'a>, bitcoin::script::Error>>,
	expected_op: opcodes::Opcode,
	desc: &str,
) -> Result<(), BitcoinScriptError> {
	match instructions
		.next()
		.ok_or_else(|| BitcoinScriptError::InstructionNotFound(desc.into()))?
	{
		Ok(Instruction::Op(op)) if op == expected_op => Ok(()),
		_ => Err(BitcoinScriptError::UnexpectedInstruction),
	}
}

/// Helper function to ensure the next instruction is a push operation of the expected length.
///
/// Returns an error if the expected length is not met or if there are no more instructions.
pub fn expect_push_bytes<'a>(
	instructions: &mut impl Iterator<Item = Result<Instruction<'a>, bitcoin::script::Error>>,
	expected_len: Option<usize>,
	desc: &str,
) -> Result<Vec<u8>, BitcoinScriptError> {
	match (
		instructions
			.next()
			.ok_or_else(|| BitcoinScriptError::InstructionNotFound(desc.into()))?,
		expected_len,
	) {
		(Ok(Instruction::PushBytes(bytes)), Some(expected)) if bytes.len() != expected =>
			Err(BitcoinScriptError::InvalidLength(desc.into())),
		(Ok(Instruction::PushBytes(bytes)), _) => Ok(bytes.as_bytes().to_vec()),
		_ => Err(BitcoinScriptError::UnexpectedInstruction),
	}
}
