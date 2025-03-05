use bitcoin::{
	opcodes,
	script::{self, PushBytes},
	ScriptBuf,
};

use crate::{
	varint::{self},
	Brc721CollectionId,
};

use super::{
	bitcoin_script::{expect_opcode, expect_push_bytes, BitcoinScriptError},
	flags::{Brc721Flag, BRC721_FLAG_LENGTH, BRC721_INIT_CODE},
};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct RegisterOwnership {
	pub collection_id: Brc721CollectionId,
	pub slots_bundles: Vec<SlotsBundle>,
}

impl From<RegisterOwnership> for ScriptBuf {
	fn from(register_ownership: RegisterOwnership) -> Self {
		let collection_id = register_ownership.collection_id.to_leb128();
		let collection_id: &PushBytes = collection_id
			.as_slice()
			.try_into()
			.expect("qed; collection_id slice should convert to PushBytes");

		let mut slots_bundles = Vec::<u8>::new();
		varint::encode_to_vec(
			register_ownership
				.slots_bundles
				.len()
				.try_into()
				.expect("qed; usize conversion to u128 failed"),
			&mut slots_bundles,
		);
		for slots_bundle in &register_ownership.slots_bundles {
			slots_bundles.extend_from_slice(&slots_bundle.to_leb128());
		}
		let slots_bundles: &PushBytes = slots_bundles
			.as_slice()
			.try_into()
			.expect("qed; slots slice should convert to PushBytes");

		script::Builder::new()
			.push_opcode(opcodes::all::OP_RETURN)
			.push_opcode(BRC721_INIT_CODE)
			.push_slice(Brc721Flag::RegisterOwnership.byte_slice())
			.push_slice(collection_id)
			.push_slice(slots_bundles)
			.into_script()
	}
}

impl TryFrom<ScriptBuf> for RegisterOwnership {
	type Error = BitcoinScriptError;
	fn try_from(payload: ScriptBuf) -> Result<Self, BitcoinScriptError> {
		let mut instructions = payload.instructions();

		expect_opcode(&mut instructions, opcodes::all::OP_RETURN, "OP_RETURN")?;
		expect_opcode(&mut instructions, BRC721_INIT_CODE, "BRC721_INIT_CODE")?;

		match expect_push_bytes(
			&mut instructions,
			Some(BRC721_FLAG_LENGTH),
			"Register ownership flag",
		) {
			Ok(byte) if byte == Brc721Flag::RegisterCollection.byte_slice() => (),
			Err(err) => return Err(err),
			_ => return Err(BitcoinScriptError::UnexpectedInstruction),
		}

		let collection_id = expect_push_bytes(&mut instructions, None, "collection id")?;
		let collection_id = Brc721CollectionId::from_leb128(&collection_id)
			.map_err(|e| BitcoinScriptError::Decode(e.to_string()))?;

		let mut slots_bundles_encoded: Vec<u8> =
			expect_push_bytes(&mut instructions, None, "slots bundles")?;
		let (num_bundles, consumed) = varint::decode(&slots_bundles_encoded)
			.map_err(|e| BitcoinScriptError::Decode(e.to_string()))?;
		slots_bundles_encoded.drain(0..consumed);
		let mut slots_bundles: Vec<SlotsBundle> = Vec::with_capacity(num_bundles as usize);
		for _ in 0..num_bundles {
			let ranges = SlotsBundle::from_leb128(&mut slots_bundles_encoded).unwrap();
			slots_bundles.push(ranges);
		}

		Ok(RegisterOwnership { collection_id, slots_bundles })
	}
}

#[derive(Clone, PartialEq, Debug)]
pub struct SlotsBundle(pub Vec<std::ops::RangeInclusive<u128>>);

impl SlotsBundle {
	pub fn to_leb128(&self) -> Vec<u8> {
		let mut encoded = Vec::new();

		varint::encode_to_vec(
			self.0.len().try_into().expect("qed; usize conversion to u128 failed"),
			&mut encoded,
		);

		for range in &self.0 {
			varint::encode_to_vec(*range.start(), &mut encoded);
			varint::encode_to_vec(*range.end(), &mut encoded);
		}
		encoded
	}

	pub fn from_leb128(encoded: &mut Vec<u8>) -> Result<Self, varint::Error> {
		let mut slots_bundle = Vec::<std::ops::RangeInclusive<u128>>::new();
		let (num_ranges, consumed) = varint::decode(encoded)?;
		encoded.drain(0..consumed);
		for _ in 0..num_ranges {
			let (start, consumed) = varint::decode(encoded)?;
			encoded.drain(0..consumed);
			let (end, consumed) = varint::decode(encoded)?;
			encoded.drain(0..consumed);
			slots_bundle.push(start..=end);
		}
		Ok(SlotsBundle(slots_bundle))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::FromStr;

	#[test]
	fn register_ownership_encodes_decodes_correctly() {
		let command = RegisterOwnership {
			collection_id: Brc721CollectionId::from_str("1:1").unwrap(),
			slots_bundles: vec![
				SlotsBundle(vec![(0..=3), (4..=10)]),
				SlotsBundle(vec![(15..=15), (40..=50)]),
			],
		};
		let encoded = ScriptBuf::from(command.clone());
		assert_eq!(encoded.len(), 20);
		let decoded = RegisterOwnership::try_from(encoded).unwrap();
		assert_eq!(command.collection_id, decoded.collection_id);
		assert_eq!(command.slots_bundles, decoded.slots_bundles);
	}
}
