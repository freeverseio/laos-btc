use bitcoin::{
	opcodes,
	script::{self, PushBytes},
	Address, ScriptBuf,
};

use super::register_collection::{expect_opcode, expect_push_bytes, RegisterCollectionError};
use crate::{
	varint::{self},
	Brc721CollectionId,
};

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

		varint::encode_to_vec(
			self.0.len().try_into().expect("qed; usize conversion to u128 failed"),
			&mut value,
		);

		for range in &self.0 {
			varint::encode_to_vec(*range.start(), &mut value);
			varint::encode_to_vec(*range.end(), &mut value);
		}
		value
	}

	pub fn from_leb128(value: &mut Vec<u8>) -> Result<Self, varint::Error> {
		let mut ranges = Vec::<std::ops::RangeInclusive<u128>>::new();
		let (num_ranges, consumed) = varint::decode(value)?;
		value.drain(0..consumed);
		for _ in 0..num_ranges {
			let (start, consumed) = varint::decode(value)?;
			value.drain(0..consumed);
			let (end, consumed) = varint::decode(value)?;
			value.drain(0..consumed);
			ranges.push(start..=end);
		}
		Ok(Ranges(ranges))
	}
}

const REGISTER_OWNERSHIP_CODE: opcodes::Opcode = opcodes::all::OP_PUSHNUM_16; // TODO

impl From<RegisterOwnership> for ScriptBuf {
	fn from(register_ownership: RegisterOwnership) -> Self {
		let collection_id = register_ownership.collection_id.to_leb128();
		let collection_id: &PushBytes = collection_id
			.as_slice()
			.try_into()
			.expect("qed; collection_id slice should convert to PushBytes");

		let mut slots = Vec::<u8>::new();
		varint::encode_to_vec(
			register_ownership
				.slots
				.len()
				.try_into()
				.expect("qed; usize conversion to u128 failed"),
			&mut slots,
		);
		for slot_ranges in &register_ownership.slots {
			slots.extend_from_slice(&slot_ranges.to_leb128());
		}
		let slots: &PushBytes = slots
			.as_slice()
			.try_into()
			.expect("qed; slots slice should convert to PushBytes");

		script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(REGISTER_OWNERSHIP_CODE)
			.push_slice(&collection_id)
			.push_slice(&slots)
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

		// TODO rename slots bundles and so on
		let mut slots_bundles: Vec<u8> =
			expect_push_bytes(&mut instructions, None, "slots bundles")?;
		let (num_bundles, consumed) = varint::decode(&slots_bundles)
			.map_err(|e| RegisterCollectionError::InvalidLength(e.to_string()))?; // TODO error
		slots_bundles.drain(0..consumed);
		let mut slots: Vec<Ranges> = Vec::with_capacity(num_bundles as usize);
		for _ in 0..num_bundles {
			let ranges = Ranges::from_leb128(&mut slots_bundles).unwrap();
			slots.push(ranges);
		}

		Ok(RegisterOwnership { collection_id, slots, ..Default::default() }) // TODO ignore owners
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;

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
		assert_eq!(encoded.len(), 20);
		let decoded = RegisterOwnership::try_from(encoded).unwrap();
		assert_eq!(command.collection_id, decoded.collection_id);
		assert_eq!(command.slots, decoded.slots);
	}
}
