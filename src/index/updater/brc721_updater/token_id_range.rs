use super::{Slot, TokenId};
use sp_core::{H160, U256};
use std::ops::RangeInclusive;

struct TokenIdRange {
	slot_rage: RangeInclusive<Slot>,
	registrant: H160,
}

impl TokenIdRange {
	fn new(first: Slot, last: Slot, registrant: H160) -> Self {
		TokenIdRange { slot_rage: first..=last, registrant }
	}

	fn first_token(self) -> TokenId {
		TokenId::from((self.slot_rage.start().clone(), self.registrant))
	}

	fn last_token(self) -> TokenId {
		TokenId::from((self.slot_rage.end().clone(), self.registrant))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_first_token_id() {
		// crate the Range
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::default();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);

		let first_token: U256 = range.first_token().into();

		let hex_string = hex::encode(first_token.to_big_endian());
		assert_eq!(hex_string, "0000000000000000000000010000000000000000000000000000000000000000");
		assert_eq!(first_token.to_string(), "1461501637330902918203684832716283019655932542976");
	}

	#[test]
	fn get_last_token_id() {
		// Create the Range
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::default();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);

		let last_token: U256 = range.last_token().into();

		let hex_string = hex::encode(last_token.to_big_endian());
		assert_eq!(hex_string, "0000000000000000000000090000000000000000000000000000000000000000");
		assert_eq!(last_token.to_string(), "13153514735978126263833163494446547176903392886784");
	}
}
