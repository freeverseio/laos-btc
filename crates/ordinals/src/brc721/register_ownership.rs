use bitcoin::{opcodes, script::PushBytes, ScriptBuf};

use crate::{
	varint::{self},
	Brc721CollectionId,
};

use super::{
	bitcoin_script::{expect_opcode, expect_push_bytes, BitcoinScriptError},
	operations::Brc721Operation,
	BRC721_INIT_CODE,
};

#[derive(Clone, PartialEq, Debug)]
pub struct RegisterOwnership {
	pub collection_id: Brc721CollectionId,
	pub slots_bundles: Vec<SlotsBundle>,
}

const MIN_BUFFER_SIZE: usize = 7;

impl From<RegisterOwnership> for ScriptBuf {
	fn from(register_ownership: RegisterOwnership) -> Self {
		let collection_id = register_ownership.collection_id.to_leb128();

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

		let mut script = ScriptBuf::new();
		script.push_opcode(opcodes::all::OP_RETURN);
		script.push_opcode(BRC721_INIT_CODE);
		let mut buffer = Vec::<u8>::new();
		buffer.push(Brc721Operation::RegisterOwnership as u8);
		buffer.extend_from_slice(&collection_id);
		buffer.extend_from_slice(&slots_bundles);
		let buffer: &PushBytes = buffer.as_slice().try_into().unwrap();
		script.push_slice(buffer);
		script
	}
}

impl TryFrom<ScriptBuf> for RegisterOwnership {
	type Error = BitcoinScriptError;
	fn try_from(payload: ScriptBuf) -> Result<Self, BitcoinScriptError> {
		let mut instructions = payload.instructions();

		expect_opcode(&mut instructions, opcodes::all::OP_RETURN, "OP_RETURN")?;
		expect_opcode(&mut instructions, BRC721_INIT_CODE, "BRC721_INIT_CODE")?;

		let buffer = expect_push_bytes(&mut instructions, "Register ownership operation")?;

		if buffer.len() < MIN_BUFFER_SIZE {
			return Err(BitcoinScriptError::InvalidLength("script is too short".to_string()));
		}

		if buffer[0] != Brc721Operation::RegisterOwnership as u8 {
			return Err(BitcoinScriptError::UnexpectedInstruction);
		}

		let mut buffer = buffer[1..].to_vec();
		let collection_id = Brc721CollectionId::from_leb128(&mut buffer).map_err(
			|e: super::collection_id::Error| {
				BitcoinScriptError::Decode(format!("{} while extracting collection_id", e))
			},
		)?;

		let (num_bundles, consumed) = varint::decode(&buffer).map_err(|e| {
			BitcoinScriptError::Decode(format!("{} while extracting num_bundles", e))
		})?;
		buffer.drain(0..consumed);
		let mut slots_bundles: Vec<SlotsBundle> = Vec::with_capacity(num_bundles as usize);
		for i in 0..num_bundles {
			let ranges = SlotsBundle::from_leb128(&mut buffer).map_err(|e: varint::Error| {
				BitcoinScriptError::Decode(format!("{} while extracting range {}", e, i))
			})?;
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
			if start >= (1u128 << 96) {
				return Err(varint::Error::Overflow);
			}
			encoded.drain(0..consumed);
			let (end, consumed) = varint::decode(encoded)?;
			if end >= (1u128 << 96) {
				return Err(varint::Error::Overflow);
			}
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
	fn slots_bundle_to_leb128() {
		let bundle = SlotsBundle(vec![(10..=11), (2..=5), (8..=8)]);
		let encoded = bundle.to_leb128();
		assert_eq!("030a0b02050808", hex::encode(encoded));

		let bundle = SlotsBundle(vec![(20..=22)]);
		let encoded = bundle.to_leb128();
		assert_eq!("011416", hex::encode(encoded));
	}

	#[test]
	fn slots_bundle_from_leb128() {
		let encoded = "030a0b02050808";
		let bundle = SlotsBundle::from_leb128(&mut hex::decode(encoded).unwrap()).unwrap();
		assert_eq!(bundle, SlotsBundle(vec![(10..=11), (2..=5), (8..=8)]));

		let encoded = "011416";
		let bundle = SlotsBundle::from_leb128(&mut hex::decode(encoded).unwrap()).unwrap();
		assert_eq!(bundle, SlotsBundle(vec![(20..=22)]));
	}

	#[test]
	fn register_ownership_from_script_fails_short_script() {
		let mut script = ScriptBuf::new();
		script.push_opcode(opcodes::all::OP_RETURN);
		script.push_opcode(BRC721_INIT_CODE);
		let buffer = vec![Brc721Operation::RegisterOwnership as u8];
		let buffer: &PushBytes = buffer.as_slice().try_into().unwrap();
		script.push_slice(buffer);

		let decoded = RegisterOwnership::try_from(script);
		assert_eq!(decoded.unwrap_err().to_string(), "Invalid length: `script is too short`");
	}

	#[test]
	fn script_from_register_ownership_and_back() {
		let command = RegisterOwnership {
			collection_id: Brc721CollectionId::from_str("5:7").unwrap(),
			slots_bundles: vec![
				SlotsBundle(vec![(16..=17), (2..=5), (8..=8)]),
				SlotsBundle(vec![(32..=34)]),
			],
		};
		let encoded = ScriptBuf::from(command.clone());

		assert_eq!("6a5f0e0105070203101102050808012022", encoded.to_hex_string());
		assert_eq!(encoded.len(), 17);

		let decoded = RegisterOwnership::try_from(encoded).unwrap();
		assert_eq!(command.collection_id, decoded.collection_id);
		assert_eq!(command.slots_bundles, decoded.slots_bundles);
	}

	#[test]
	fn register_ownership_from_script_and_back_extra_bytes_are_ignored() {
		let command = RegisterOwnership {
			collection_id: Brc721CollectionId::from_str("5:7").unwrap(),
			slots_bundles: vec![
				SlotsBundle(vec![(16..=17), (2..=5), (8..=8)]),
				SlotsBundle(vec![(32..=34)]),
			],
		};
		let encoded = ScriptBuf::from(command.clone());
		assert_eq!("6a5f0e0105070203101102050808012022", encoded.to_hex_string());
		assert_eq!(encoded.len(), 17);
		let mut encoded_with_extra_bytes = encoded.clone().into_bytes();
		encoded_with_extra_bytes.extend_from_slice(&[0x00; 10]);
		let encoded_with_extra_bytes = ScriptBuf::from_bytes(encoded_with_extra_bytes);
		assert_eq!(
			"6a5f0e010507020310110205080801202200000000000000000000",
			encoded_with_extra_bytes.to_hex_string()
		);

		let decoded = RegisterOwnership::try_from(encoded_with_extra_bytes).unwrap();
		assert_eq!(command.collection_id, decoded.collection_id);
		assert_eq!(command.slots_bundles, decoded.slots_bundles);
	}

	#[test]
	fn register_ownership_from_script_and_back_big_numbers() {
		let block = u64::MAX;
		let tx = u32::MAX;
		let collection_id = Brc721CollectionId::new(block, tx).unwrap();
		let command = RegisterOwnership {
			collection_id,
			slots_bundles: {
				// Create a vector with single-number slots from 0 to 2^96
				let mut bundles = Vec::new();
				let slots = vec![
					0u128,             // Min value
					1u128 << 32,       // 2^32
					1u128 << 64,       // 2^64
					1u128 << 95,       // 2^95
					(1u128 << 96) - 1, // Max allowed value (2^96 - 1)
				];
				for slot in slots {
					bundles.push(SlotsBundle(vec![(slot..=slot)]));
				}
				bundles
			},
		};
		let encoded = ScriptBuf::from(command.clone());
		assert_eq!(encoded.len(), 114);
		assert_eq!("6a5f4c6e01ffffffffffffffffff01ffffffff0f050100000180808080108080808010018080808080808080800280808080808080808002018080808080808080808080808010808080808080808080808080801001ffffffffffffffffffffffffff1fffffffffffffffffffffffffff1f", encoded.to_hex_string());
		let decoded = RegisterOwnership::try_from(encoded).unwrap();
		assert_eq!(command.collection_id, decoded.collection_id);
		assert_eq!(command.slots_bundles, decoded.slots_bundles);
	}

	#[test]
	fn slot_start_overflow() {
		let command = RegisterOwnership {
			collection_id: Brc721CollectionId::from_str("5:7").unwrap(),
			slots_bundles: vec![SlotsBundle(vec![1u128 << 96..=1u128 << 96])],
		};
		let encoded = ScriptBuf::from(command.clone());
		assert_eq!(
			"6a5f21010507010180808080808080808080808080208080808080808080808080808020",
			encoded.to_hex_string()
		);
		let decoded = RegisterOwnership::try_from(encoded);
		assert_eq!(
			decoded.unwrap_err().to_string(),
			"Decoding error: `overflow while extracting range 0`"
		);
	}

	#[test]
	fn slot_end_overflow() {
		let command = RegisterOwnership {
			collection_id: Brc721CollectionId::from_str("5:7").unwrap(),
			slots_bundles: vec![SlotsBundle(vec![(1u128 << (96 - 1))..=1u128 << 96])],
		};
		let encoded = ScriptBuf::from(command.clone());
		assert_eq!(
			"6a5f21010507010180808080808080808080808080108080808080808080808080808020",
			encoded.to_hex_string()
		);
		let decoded = RegisterOwnership::try_from(encoded);
		assert_eq!(
			decoded.unwrap_err().to_string(),
			"Decoding error: `overflow while extracting range 0`"
		);
	}
}
