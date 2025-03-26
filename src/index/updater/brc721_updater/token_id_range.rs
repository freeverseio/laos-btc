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
	use std::str::FromStr;

	#[test]
	fn get_first_token_id() {
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::from_str("0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E").unwrap();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);
		let first_token: U256 = range.first_token().into();
		assert_eq!(hex::encode(first_token.to_big_endian()), "000000000000000000000001d4a24fe19b5e0ed77137012b95b4433293e2ff8e");
		assert_eq!(first_token.to_string(), "2675427360108358740834871412060616377274126761870");
	}

	#[test]
	fn get_last_token_id() {
		let first_slot = Slot::try_from(1).unwrap();
		let last_slot = Slot::try_from(9).unwrap();
		let registrant = H160::from_str("0xD4a24FE19b5e0ED77137012B95b4433293E2Ff8E").unwrap();
		let range = TokenIdRange::new(first_slot, last_slot, registrant);
		let last_token: U256 = range.last_token().into();
		assert_eq!(hex::encode(last_token.to_big_endian()), "000000000000000000000009d4a24fe19b5e0ed77137012b95b4433293e2ff8e");
		assert_eq!(last_token.to_string(), "14367440458755582086464350073790880534521587105678");
	}
}
