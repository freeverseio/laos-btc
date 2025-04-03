use super::{Slot, TokenId};
use sp_core::H160;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct TokenIdRange {
	slot_range: RangeInclusive<Slot>,
	registrant: H160,
}

impl TokenIdRange {
	/// Creates a new TokenIdRange with the specified first and last slots and registrant address.
	pub fn new(first: Slot, last: Slot, registrant: H160) -> Self {
		TokenIdRange { slot_range: first..=last, registrant }
	}

	/// Returns the first token ID in this range.
	///
	/// Note: This method consumes self.
	pub fn first_token(&self) -> TokenId {
		TokenId::from((*self.slot_range.start(), self.registrant))
	}

	/// Returns the last token ID in this range.
	///
	/// Note: This method consumes self.
	pub fn last_token(&self) -> TokenId {
		TokenId::from((*self.slot_range.end(), self.registrant))
	}

	/// Checks if the given token ID is contained within this range.
	///
	/// Returns false if the token has a different registrant or if the slot is outside the range.
	pub fn contains(&self, token: TokenId) -> bool {
		if token.registrant() != self.registrant {
			return false;
		}

		self.slot_range.contains(&token.slot())
	}

	/// Checks if this range overlaps with another TokenIdRange.
	pub fn overlaps(&self, other: &TokenIdRange) -> bool {
		// If the registrants are different, the ranges don't overlap
		if self.registrant != other.registrant {
			return false;
		}

		// Check if the slot ranges overlap
		let self_start = *self.slot_range.start();
		let self_end = *self.slot_range.end();
		let other_start = *other.slot_range.start();
		let other_end = *other.slot_range.end();

		// Ranges overlap if one range's start is <= the other's end
		// and one range's end is >= the other's start
		self_start <= other_end && self_end >= other_start
	}
}

impl redb::Value for TokenIdRange {
	type SelfType<'a> = Self;
	type AsBytes<'a> = [u8; 44];

	fn fixed_width() -> Option<usize> {
		// first slot 96b + last slot 96b + registrant 160b
		Some(44)
	}

	fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
	where
		Self: 'a,
	{
		if data.len() != Self::fixed_width().unwrap() {
			unreachable!()
		}

		let first_slot = Slot(data[0..12].try_into().unwrap());
		let last_slot = Slot(data[12..24].try_into().unwrap());
		let registrant_array: [u8; 20] = data[24..44].try_into().unwrap();
		let registrant = H160::from(registrant_array);
		TokenIdRange::new(first_slot, last_slot, registrant)
	}

	fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
	where
		Self: 'b,
	{
		let mut buffer = [0u8; 44];
		buffer[0..12].copy_from_slice(&value.slot_range.start().0);
		buffer[12..24].copy_from_slice(&value.slot_range.end().0);
		buffer[24..44].copy_from_slice(value.registrant.as_bytes());
		buffer
	}

	fn type_name() -> redb::TypeName {
		redb::TypeName::new("brc721::token_id_range")
	}
}

impl redb::Key for TokenIdRange {
	fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
		data1.cmp(data2)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use redb::Value;
	use sp_core::U256;
	use std::str::FromStr;

	fn setup_range(first_slot: u128, last_slot: u128, registrant: &str) -> TokenIdRange {
		let first_slot = Slot::try_from(first_slot).unwrap();
		let last_slot = Slot::try_from(last_slot).unwrap();
		let registrant = H160::from_str(registrant).unwrap();
		TokenIdRange::new(first_slot, last_slot, registrant)
	}

	#[test]
	fn get_first_token_id() {
		let range = setup_range(1, 9, "0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E");
		let first_token: U256 = range.first_token().into();
		assert_eq!(
			hex::encode(first_token.to_big_endian()),
			"000000000000000000000001d4a24fe19b5e0ed77137012b95b4433293e2ff8e"
		);
		assert_eq!(first_token.to_string(), "2675427360108358740834871412060616377274126761870");
	}

	#[test]
	fn get_last_token_id() {
		let range = setup_range(1, 9, "0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E");
		let last_token: U256 = range.last_token().into();
		assert_eq!(
			hex::encode(last_token.to_big_endian()),
			"000000000000000000000009d4a24fe19b5e0ed77137012b95b4433293e2ff8e"
		);
		assert_eq!(last_token.to_string(), "14367440458755582086464350073790880534521587105678");
	}

	#[test]
	fn test_contain() {
		let range = setup_range(1, 9, "0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E");
		let registrant = H160::from_str("0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E").unwrap();

		assert!(!range.contains(TokenId::from((Slot::try_from(0).unwrap(), registrant))));
		assert!(range.contains(TokenId::from((Slot::try_from(1).unwrap(), registrant))));
		assert!(range.contains(TokenId::from((Slot::try_from(7).unwrap(), registrant))));
		assert!(range.contains(TokenId::from((Slot::try_from(9).unwrap(), registrant))));
		assert!(!range.contains(TokenId::from((Slot::try_from(10).unwrap(), registrant))));
	}

	#[test]
	fn test_as_bytes() {
		let range = setup_range(1, 9, "0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E");
		let buffer = TokenIdRange::as_bytes(&range);
		assert_eq!(buffer.len(), TokenIdRange::fixed_width().unwrap());
		assert_eq!(hex::encode(buffer), "000000000000000000000001000000000000000000000009d4a24fe19b5e0ed77137012b95b4433293e2ff8e");
	}

	#[test]
	fn check_width() {
		assert_eq!(TokenIdRange::fixed_width().unwrap(), 44);
	}

	#[test]
	fn test_from_bytes() {
		let buffer = hex::decode("000000000000000000000001000000000000000000000009d4a24fe19b5e0ed77137012b95b4433293e2ff8f").unwrap();
		let range = TokenIdRange::from_bytes(&buffer);
		assert_eq!(range, setup_range(1, 9, "0xd4a24fe19b5e0ed77137012b95b4433293e2ff8f"));
	}
}
