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

impl From<RegisterCollection> for ScriptBuf {
	fn from(register_collection: RegisterCollection) -> Self {
		let mut builder = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE);

		let address: &script::PushBytes =
			register_collection.address.as_bytes().try_into().expect("Conversion failed");
		let rebaseable: [u8; 1] = if register_collection.rebaseable { [1] } else { [0] };

		builder = builder.push_slice(address);
		builder = builder.push_slice::<&script::PushBytes>((&rebaseable).into());

		builder.into_script()
	}
}

#[derive(Debug, Error, PartialEq)]
pub enum RegisterCollectionError {
	#[error("Instruction not found: `{0}`")]
	InstructionNotFound(String),
	#[error("Invalid output")]
	InvalidOutput,
	#[error("Unexpected instruction")]
	UnexpectedIntruction,
	#[error("Invalid lenght: `{0}`")]
	InvalidLength(String),
	#[error("Output not found")]
	OutputNotFound,
}

impl TryFrom<Transaction> for RegisterCollection {
	type Error = RegisterCollectionError;
	fn try_from(transaction: Transaction) -> Result<Self, Self::Error> {
		let output = transaction.output.first().ok_or(RegisterCollectionError::OutputNotFound)?;

		let mut instructions = output.script_pubkey.instructions();
		match instructions
			.next()
			.ok_or(RegisterCollectionError::InstructionNotFound("OP_RETURN".into()))?
		{
			Ok(Instruction::Op(opcodes::all::OP_RETURN)) => {},
			_ => return Err(RegisterCollectionError::UnexpectedIntruction),
		}

		match instructions.next().ok_or(RegisterCollectionError::InstructionNotFound(
			"REGISTER_COLLECTION_CODE".into(),
		))? {
			Ok(Instruction::Op(REGISTER_COLLECTION_CODE)) => {},
			_ => return Err(RegisterCollectionError::UnexpectedIntruction),
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
			_ => return Err(RegisterCollectionError::UnexpectedIntruction),
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
			_ => return Err(RegisterCollectionError::UnexpectedIntruction),
		}

		Ok(Self {
			address: H160::from_slice(&payload[..COLLECTION_ADDRESS_LENGTH]),
			rebaseable: payload[COLLECTION_ADDRESS_LENGTH] > 0, // any value > 0 indicates `true`
		})
	}
}

#[cfg(test)]
mod tests {
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
			RegisterCollection::try_from(tx).unwrap_err(),
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
			RegisterCollection::try_from(tx).unwrap_err().to_string(),
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
			RegisterCollection::try_from(tx).unwrap_err().to_string(),
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
			RegisterCollection::try_from(tx).unwrap_err().to_string(),
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
			RegisterCollection::try_from(tx).unwrap_err().to_string(),
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

		let register_collection_decoded: RegisterCollection = tx.try_into().unwrap();
		assert_eq!(register_collection_decoded.address, address.into());
		assert_eq!(register_collection_decoded.rebaseable, false);
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
				script_pubkey: register_collection.clone().into(),
			}],
		};

		let register_collection_decoded: RegisterCollection = tx.try_into().unwrap();

		assert_eq!(register_collection, register_collection_decoded);
	}
}
