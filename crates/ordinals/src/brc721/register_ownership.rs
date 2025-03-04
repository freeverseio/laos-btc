use std::{ops::Add, str::FromStr};

use bitcoin::{
	opcodes,
	script::{self, PushBytes},
	Address, ScriptBuf,
};

use super::register_collection::{expect_opcode, expect_push_bytes, RegisterCollectionError};
use crate::{varint::encode_to_vec, Brc721CollectionId};
use bitcoin::script::Instruction;
use serde::{Deserialize, Serialize};
use sp_core::H160;
use thiserror::Error;

#[derive(Clone, PartialEq, Debug)]
pub struct Ranges(pub Vec<std::ops::RangeInclusive<u128>>);

#[derive(Clone, PartialEq, Debug, Default)]
pub struct RegisterOwnership {
	pub collection_id: Brc721CollectionId,
	pub slots: Vec<Ranges>,
	pub owners: Vec<Address>,
}

impl Ranges {
	pub fn to_leb128(&self) -> Vec<u8> {
		let mut value = Vec::new();
		// let packed: u128 = ((self.block as u128) << 32) | (self.tx as u128);
		// For each slot group, encode its length and then each slot_range value.
		// payload.extend(encode_leb128(group.len() as u128));
		// for &slot_range in group {
		// 	payload.extend(encode_leb128(slot_range));
		// }
		// varint::encode_to_vec(packed, &mut value);
		value
	}

	// pub fn from_leb128(value: &Vec<u8>) -> Result<Self, Error> {
	// let (n, _consumed) = varint::decode(value).map_err(|e| Error::Decode(e))?;
	// // Extract block from the upper 64 bits of the lower 96 bits
	// let block = n >> 32;
	// // Extract tx from the lower 32 bits
	// let tx = n & 0xFFFF_FFFF;
	// Ok(Brc721CollectionId { block: block as u64, tx: tx as u32 })
	// }
}

const REGISTER_OWNERSHIP_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_16; // TODO

impl From<RegisterOwnership> for ScriptBuf {
	fn from(register_ownership: RegisterOwnership) -> Self {
		let collection_id = register_ownership.collection_id.to_leb128();
		let collection_id: &PushBytes = collection_id
			.as_slice()
			.try_into()
			.expect("qed; collection_id slice should convert to PushBytes");

		// encode_to_vec(register_ownership.slots.len() as u128, &mut payload);
		// for encode each bundle

		script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_OWNERSHIP_CODE)
			.push_slice(&collection_id)
			// push slots
			.into_script()
	}
}

impl TryFrom<ScriptBuf> for RegisterOwnership {
	type Error = RegisterCollectionError;
	fn try_from(payload: ScriptBuf) -> Result<Self, RegisterCollectionError> {
		let mut instructions = payload.instructions();

		expect_opcode(&mut instructions, opcodes::all::OP_RETURN, "OP_RETURN")?;
		expect_opcode(&mut instructions, REGISTER_OWNERSHIP_CODE, "REGISTER_OWNERSHIP_CODE")?;

		let collection_id = expect_push_bytes(&mut instructions, None, "collection id")?;
		let collection_id = Brc721CollectionId::from_leb128(&collection_id)
			.map_err(|e| RegisterCollectionError::InstructionNotFound(e.to_string()))?; // TODO error

		// get slots value and decode

		Ok(RegisterOwnership { collection_id, ..Default::default() })
	}
}

#[cfg(test)]
mod tests {
	use std::{collections::btree_map::Range, default};

	use crate::varint::{self, encode};

	use super::*;

	use bitcoin::{
		secp256k1::{rand, Secp256k1},
		Address, Network, PublicKey,
	};
	fn get_random_address() -> Address {
		// Generate random key pair.
		let s = Secp256k1::new();
		let public_key = PublicKey::new(s.generate_keypair(&mut rand::thread_rng()).1);

		// Generate pay-to-pubkey-hash address.
		Address::p2pkh(&public_key, Network::Regtest)
	}

	#[test]
	fn register_ownership_encodes_decodes_correctly() {
		let command = RegisterOwnership {
			owners: vec![get_random_address()],
			collection_id: Brc721CollectionId::from_str("1:1").unwrap(),
			slots: vec![Ranges(vec![(0..=3), (4..=10)]), Ranges(vec![(15..=15), (40..=50)])],
		};
		let encoded = ScriptBuf::from(command.clone());
		assert_eq!(command, encoded.try_into().unwrap());
	}
}
