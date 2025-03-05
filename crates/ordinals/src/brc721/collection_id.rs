// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use super::*;

#[derive(
	Debug,
	PartialEq,
	Copy,
	Clone,
	Hash,
	Eq,
	Ord,
	PartialOrd,
	Default,
	DeserializeFromStr,
	SerializeDisplay,
)]
pub struct Brc721CollectionId {
	pub block: u64,
	pub tx: u32,
}

impl Brc721CollectionId {
	pub fn new(block: u64, tx: u32) -> Option<Brc721CollectionId> {
		let id = Brc721CollectionId { block, tx };

		if id.block == 0 && id.tx > 0 {
			return None;
		}

		Some(id)
	}

	pub fn delta(self, next: Brc721CollectionId) -> Option<(u128, u128)> {
		let block = next.block.checked_sub(self.block)?;

		let tx = if block == 0 { next.tx.checked_sub(self.tx)? } else { next.tx };

		Some((block.into(), tx.into()))
	}

	pub fn next(self: Brc721CollectionId, block: u128, tx: u128) -> Option<Brc721CollectionId> {
		Brc721CollectionId::new(
			self.block.checked_add(block.try_into().ok()?)?,
			if block == 0 {
				self.tx.checked_add(tx.try_into().ok()?)?
			} else {
				tx.try_into().ok()?
			},
		)
	}

	pub fn to_leb128(&self) -> Vec<u8> {
		let mut value = Vec::new();
		let packed: u128 = ((self.block as u128) << 32) | (self.tx as u128);
		varint::encode_to_vec(packed, &mut value);
		value
	}

	pub fn from_leb128(value: &[u8]) -> Result<Self, Error> {
		let (n, _consumed) = varint::decode(value).map_err(Error::Decode)?;
		// Extract block from the upper 64 bits of the lower 96 bits
		let block = n >> 32;
		// Extract tx from the lower 32 bits
		let tx = n & 0xFFFF_FFFF;
		Ok(Brc721CollectionId { block: block as u64, tx: tx as u32 })
	}
}

impl Display for Brc721CollectionId {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}:{}", self.block, self.tx)
	}
}

impl FromStr for Brc721CollectionId {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (height, index) = s.split_once(':').ok_or(Error::Separator)?;

		Ok(Self {
			block: height.parse().map_err(Error::Block)?,
			tx: index.parse().map_err(Error::Transaction)?,
		})
	}
}

#[derive(Debug, PartialEq)]
pub enum Error {
	Separator,
	Block(ParseIntError),
	Transaction(ParseIntError),
	Decode(varint::Error),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Separator => write!(f, "missing separator"),
			Self::Block(err) => write!(f, "invalid height: {err}"),
			Self::Transaction(err) => write!(f, "invalid index: {err}"),
			Self::Decode(err) => write!(f, "decoding error: {err}"),
		}
	}
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn leb128_encode_decode_small_number() {
		let collection_id = Brc721CollectionId::from_str("1:1").unwrap();
		let encoded = collection_id.to_leb128();
		let decoded = Brc721CollectionId::from_leb128(&encoded).unwrap();
		assert_eq!(decoded, collection_id);
		assert_eq!("1:1".as_bytes().len(), 3);
		assert_eq!(encoded.len(), 5);
	}

	#[test]
	fn leb128_encode_decode_big_number() {
		let block = u64::MAX;
		let tx = u32::MAX;
		let collection_id = Brc721CollectionId::new(block, tx).unwrap();
		let encoded = collection_id.to_leb128();
		let decoded = Brc721CollectionId::from_leb128(&encoded).unwrap();
		assert_eq!(decoded, collection_id);
		assert_eq!("18446744073709551615:4294967295".as_bytes().len(), 31);
		assert_eq!(encoded.len(), 14);
	}

	#[test]
	fn delta() {
		let mut expected = [
			Brc721CollectionId { block: 3, tx: 1 },
			Brc721CollectionId { block: 4, tx: 2 },
			Brc721CollectionId { block: 1, tx: 2 },
			Brc721CollectionId { block: 1, tx: 1 },
			Brc721CollectionId { block: 3, tx: 1 },
			Brc721CollectionId { block: 2, tx: 0 },
		];

		expected.sort();

		assert_eq!(
			expected,
			[
				Brc721CollectionId { block: 1, tx: 1 },
				Brc721CollectionId { block: 1, tx: 2 },
				Brc721CollectionId { block: 2, tx: 0 },
				Brc721CollectionId { block: 3, tx: 1 },
				Brc721CollectionId { block: 3, tx: 1 },
				Brc721CollectionId { block: 4, tx: 2 },
			]
		);

		let mut previous = Brc721CollectionId::default();
		let mut deltas = Vec::new();
		for id in expected {
			deltas.push(previous.delta(id).unwrap());
			previous = id;
		}

		assert_eq!(deltas, [(1, 1), (0, 1), (1, 0), (1, 1), (0, 0), (1, 2)]);

		let mut previous = Brc721CollectionId::default();
		let mut actual = Vec::new();
		for (block, tx) in deltas {
			let next = previous.next(block, tx).unwrap();
			actual.push(next);
			previous = next;
		}

		assert_eq!(actual, expected);
	}

	#[test]
	fn display() {
		assert_eq!(Brc721CollectionId { block: 1, tx: 2 }.to_string(), "1:2");
	}

	#[test]
	fn from_str() {
		assert!(matches!("123".parse::<Brc721CollectionId>(), Err(Error::Separator)));
		assert!(matches!(":".parse::<Brc721CollectionId>(), Err(Error::Block(_))));
		assert!(matches!("1:".parse::<Brc721CollectionId>(), Err(Error::Transaction(_))));
		assert!(matches!(":2".parse::<Brc721CollectionId>(), Err(Error::Block(_))));
		assert!(matches!("a:2".parse::<Brc721CollectionId>(), Err(Error::Block(_))));
		assert!(matches!("1:a".parse::<Brc721CollectionId>(), Err(Error::Transaction(_)),));
		assert_eq!(
			"1:2".parse::<Brc721CollectionId>().unwrap(),
			Brc721CollectionId { block: 1, tx: 2 }
		);
	}

	#[test]
	fn serde() {
		let rune_id = Brc721CollectionId { block: 1, tx: 2 };
		let json = "\"1:2\"";
		assert_eq!(serde_json::to_string(&rune_id).unwrap(), json);
		assert_eq!(serde_json::from_str::<Brc721CollectionId>(json).unwrap(), rune_id);
	}
}
