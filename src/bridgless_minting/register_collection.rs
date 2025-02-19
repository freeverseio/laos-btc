use super::bitcoin_service::TxOutable;
use bitcoin::{
	opcodes,
	script::{self, Instruction},
	Amount, ScriptBuf, Transaction, TxOut,
};
use serde::{Deserialize, Serialize};
use sp_core::H160;

#[allow(dead_code)] // TODO Remove when is used
const COLLECTION_ADDRESS_LENGTH: usize = 20;
#[allow(dead_code)] // TODO Remove when is used
const REBASEABLE_LENGTH: usize = 1;
#[allow(dead_code)] // TODO Remove when is used
const PAYLOAD_LENGTH: usize = COLLECTION_ADDRESS_LENGTH + REBASEABLE_LENGTH;
const REGISTER_COLLECTION_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_15;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RegisterCollection {
	// TODO make them private?
	pub address: H160,
	pub rebaseable: bool,
}
// TODO add other trait so every transactionable has to implement as output + encipher + decipher
impl TxOutable for RegisterCollection {
	fn as_output(&self) -> TxOut {
		TxOut { value: Amount::from_sat(0), script_pubkey: self.encipher() }
	}
}
#[allow(dead_code)] // TODO Remove when is used
type Payload = [u8; PAYLOAD_LENGTH];

#[allow(dead_code)] // TODO Remove when is used
impl RegisterCollection {
	pub fn decipher(transaction: &Transaction) -> Option<RegisterCollection> {
		let payload = RegisterCollection::payload(transaction)?;
		// TODO should be return error if payload is None because missing things?

		Some(Self::from_payload(payload))
	}

	fn payload(transaction: &Transaction) -> Option<Payload> {
		let output = transaction.output.first()?;
		let mut instructions = output.script_pubkey.instructions();

		if instructions.next()? != Ok(Instruction::Op(opcodes::all::OP_RETURN)) {
			return None;
		}
		if instructions.next()? != Ok(Instruction::Op(REGISTER_COLLECTION_CODE)) {
			return None;
		}

		// construct the payload by concatenating remaining data pushes
		let mut payload = Vec::with_capacity(PAYLOAD_LENGTH);

		// Expect the first push to be the collection address
		if let Some(Ok(Instruction::PushBytes(push))) = instructions.next() {
			if push.len() == COLLECTION_ADDRESS_LENGTH {
				payload.extend_from_slice(push.as_bytes());
			} else {
				log::warn!("Invalid address length");
				return None;
			}
		} else {
			log::warn!("REGISTER_COLLECTION_CODE found but not followed by push bytes instruction");
			// TODO should we panic instead?
			return None;
		}

		// Expect the second push to be the rebaseable flag
		if let Some(Ok(Instruction::PushBytes(push))) = instructions.next() {
			if push.len() == REBASEABLE_LENGTH {
				payload.extend_from_slice(push.as_bytes());
			} else {
				log::warn!("Invalid rebasable length");
				return None;
			}
		} else {
			log::warn!("REGISTER_COLLECTION_CODE followed by push byte for collection addres but not followed by push bytes instruction for rebaseable");
			// TODO should we panic instead?
			return None;
		}

		payload.try_into().ok()
	}

	fn encipher(&self) -> ScriptBuf {
		let mut builder = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_COLLECTION_CODE);

		let address: &script::PushBytes =
			self.address.as_bytes().try_into().expect("Conversion failed");
		let rebaseable: [u8; 1] = if self.rebaseable { [1] } else { [0] };

		builder = builder.push_slice(address);
		builder = builder.push_slice::<&script::PushBytes>((&rebaseable).into());

		builder.into_script()
	}

	fn from_payload(payload: Payload) -> Self {
		Self {
			address: H160::from_slice(&payload[..COLLECTION_ADDRESS_LENGTH]),
			rebaseable: payload[COLLECTION_ADDRESS_LENGTH] > 0, // any value > 0 indicates `true`
		}
	}
}

#[cfg(test)]
mod tests {
	use bitcoin::{absolute::LockTime, transaction::Version};

	use super::*;

	#[test]
	fn register_collection_as_output() {
		let alice = H160::from([0; 20]);
		let register_collection = RegisterCollection { address: alice, rebaseable: false };
		assert!(register_collection.encipher().is_op_return());
		assert!(register_collection.as_output().script_pubkey == register_collection.encipher());
		assert!(register_collection.as_output().value == Amount::from_sat(0));
	}

	#[test]
	fn encipher_and_decipher() {
		let alice = H160::from([0; 20]);
		let register_collection = RegisterCollection { address: alice, rebaseable: false };
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![register_collection.as_output()],
		};

		let deciphered = RegisterCollection::decipher(&tx).unwrap();
		assert_eq!(deciphered, register_collection);
	}

	#[test]
	fn payload_no_output() {
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_with_output_but_no_op_return() {
		let script_buf =
			script::Builder::new().push_opcode(opcodes::all::OP_PUSHNUM_15).into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_with_op_return_but_wrong_op_code() {
		let script_buf = script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(opcodes::all::OP_PUSHNUM_15)
			.into_script();
		let tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![TxOut { value: Amount::ZERO, script_pubkey: script_buf }],
		};

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_incorrect_address_length() {
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

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_missing_rebasable() {
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

		assert!(RegisterCollection::payload(&tx).is_none());
	}

	#[test]
	fn payload_extra_push_is_ignored() {
		let address = [0xCC; COLLECTION_ADDRESS_LENGTH];
		let rebaseable = [0x00; REBASEABLE_LENGTH];
		let extra = [0xFF; 10];
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

		let payload = RegisterCollection::payload(&tx).unwrap();
		assert_eq!(payload[..COLLECTION_ADDRESS_LENGTH], address);
		assert_eq!(payload[COLLECTION_ADDRESS_LENGTH], rebaseable[0]);
	}

	#[test]
	#[should_panic(
		expected = "source slice length (21) does not match destination slice length (20)"
	)]
	fn from_payload_invalid_address_length() {
		let address = [0xEE; COLLECTION_ADDRESS_LENGTH + 1];
		let rebaseable_flag = 0x10;
		let mut payload = [0u8; PAYLOAD_LENGTH];
		payload[..COLLECTION_ADDRESS_LENGTH].copy_from_slice(&address);
		payload[COLLECTION_ADDRESS_LENGTH] = rebaseable_flag;

		let _ = RegisterCollection::from_payload(payload);
	}

	#[test]
	fn from_payload_happy_path() {
		let address = [0xEE; COLLECTION_ADDRESS_LENGTH];
		let rebaseable_flag = 0x10;
		let mut payload = [0u8; PAYLOAD_LENGTH];
		payload[..COLLECTION_ADDRESS_LENGTH].copy_from_slice(&address);
		payload[COLLECTION_ADDRESS_LENGTH] = rebaseable_flag;

		let register_collection = RegisterCollection::from_payload(payload);
		assert_eq!(register_collection.address, address.into());
		assert!(register_collection.rebaseable);
	}
}
