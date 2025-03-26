use super::{Slot, TokenId};
use redb::{TypeName, Value};
use sp_core::{H160, U256};
use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
pub struct TokenIdRange {
	slot_range: RangeInclusive<Slot>,
	registrant: H160,
}

impl TokenIdRange {
	pub fn new(first: Slot, last: Slot, registrant: H160) -> Self {
		TokenIdRange { slot_range: first..=last, registrant }
	}

	pub fn first_token(self) -> TokenId {
		TokenId::from((self.slot_range.start().clone(), self.registrant))
	}

	pub fn last_token(self) -> TokenId {
		TokenId::from((self.slot_range.end().clone(), self.registrant))
	}

	pub fn contains(&self, token: TokenId) -> bool {
		if token.registrant() != self.registrant {
			return false;
		}

		self.slot_range.contains(&token.slot())
	}
}

impl Value for TokenIdRange {
	type SelfType<'a> = Self;
	type AsBytes<'a> = [u8; 44];

	fn fixed_width() -> Option<usize> {
		// first slot 96b + last slot 96b + registrant 160b
		Some(352)
	}

	fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
	where
		Self: 'a,
	{
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::from_str("0x000000000000000000000000FFFFFFFFFFFFFFFF").unwrap();
		TokenIdRange::new(first_slot, last_slot, registrant)
	}

	fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
	where
		Self: 'b,
	{
		let mut buffer = [0u8; 44];
		buffer[0..12].copy_from_slice(&value.slot_range.start().0);
		buffer[12..24].copy_from_slice(&value.slot_range.end().0);
		buffer[24..44].copy_from_slice(&value.registrant.as_bytes());
		buffer
	}

	fn type_name() -> TypeName {
		TypeName::new("token_id_range")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use redb::Value;

	#[test]
	fn get_first_token_id() {
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::from_str("0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E").unwrap();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);
		let first_token: U256 = range.first_token().into();
		assert_eq!(
			hex::encode(first_token.to_big_endian()),
			"000000000000000000000001d4a24fe19b5e0ed77137012b95b4433293e2ff8e"
		);
		assert_eq!(first_token.to_string(), "2675427360108358740834871412060616377274126761870");
	}

	#[test]
	fn get_last_token_id() {
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::from_str("0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E").unwrap();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);
		let last_token: U256 = range.last_token().into();
		assert_eq!(
			hex::encode(last_token.to_big_endian()),
			"000000000000000000000009d4a24fe19b5e0ed77137012b95b4433293e2ff8e"
		);
		assert_eq!(last_token.to_string(), "14367440458755582086464350073790880534521587105678");
	}

	#[test]
	fn test_contain() {
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::from_str("0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E").unwrap();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);

		assert!(!range.contains(TokenId::from((Slot::try_from(0).unwrap(), registrant))));
		assert!(range.contains(TokenId::from((first_slot, registrant))));
		assert!(range.contains(TokenId::from((Slot::try_from(7).unwrap(), registrant))));
		assert!(range.contains(TokenId::from((last_slot, registrant))));
		assert!(!range.contains(TokenId::from((Slot::try_from(10).unwrap(), registrant))));
	}

	#[test]
	fn test_as_bytes() {
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::from_str("0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E").unwrap();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);

		let buffer = TokenIdRange::as_bytes(&range);
		assert_eq!(hex::encode(buffer), "000000000000000000000001000000000000000000000009d4a24fe19b5e0ed77137012b95b4433293e2ff8e");
	}
}
