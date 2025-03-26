use sp_core::{H160, U256};

/// TokenId type
/// every slot is identified by a unique `asset_id = concat(slot #, owner_address)`
#[derive(Clone, Debug, PartialEq)]
pub struct TokenId(pub ([u8; 12], [u8; 20]));

impl TokenId {
	pub fn registrant(&self) -> H160 {
		H160::from(self.0 .1)
	}

	pub fn slot(&self) -> Slot {
		Slot(self.0 .0)
	}
}

/// Slot type - 96-bit unsigned integer
#[derive(Eq, PartialEq, Clone, Copy, Default, PartialOrd, Ord, Hash, Debug)]
pub struct Slot(pub [u8; 12]);
impl Slot {
	/// Maximum value for a 96-bit unsigned integer
	pub const MAX_SLOT: Slot = Slot([0xFF; 12]);

	pub fn new(bytes: [u8; 12]) -> Self {
		Slot(bytes)
	}
}

impl TryFrom<u128> for Slot {
	type Error = &'static str;

	fn try_from(value: u128) -> Result<Self, Self::Error> {
		if value > ((1u128 << 96) - 1) {
			Err("Value exceeds 96-bit limit")
		} else {
			let bytes = value.to_be_bytes();
			let slot_bytes: [u8; 12] =
				bytes[4..].try_into().map_err(|_| "Slice conversion failed")?;
			Ok(Slot(slot_bytes))
		}
	}
}

impl From<Slot> for u128 {
	fn from(slot: Slot) -> u128 {
		let mut bytes = [0u8; 16];
		bytes[4..].copy_from_slice(&slot.0);
		u128::from_be_bytes(bytes)
	}
}

impl From<(Slot, H160)> for TokenId {
	fn from(input: (Slot, H160)) -> Self {
		Self((input.0 .0, input.1 .0))
	}
}

impl From<TokenId> for U256 {
	fn from(input: TokenId) -> Self {
		let mut bytes = [0u8; 32];

		let slot_bytes = input.0 .0;
		let owner = input.0 .1;
		// Copy the slot into the first 12 bytes of the array
		bytes[..12].copy_from_slice(&slot_bytes);
		// Copy the owner address bytes into the array
		bytes[12..].copy_from_slice(&owner);

		Self::from_big_endian(&bytes)
	}
}

impl From<U256> for TokenId {
	fn from(input: U256) -> Self {
		let num = input.to_big_endian();
		let mut slot = [0u8; 12];
		slot.copy_from_slice(&num[..12]);
		let mut owner: [u8; 20] = [0u8; 20];
		owner.copy_from_slice(&num[12..]);
		Self((slot, owner))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::str::FromStr;

	#[test]
	fn slot_from_u128_within_limit() {
		let value = 123456789012345678901234567u128;
		let slot = Slot::try_from(value).unwrap();
		let result: u128 = slot.into();
		assert_eq!(result, value);
	}

	#[test]
	fn slot_from_u128_exceeds_limit() {
		let value = 1u128 << 100;
		let result = Slot::try_from(value);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), "Value exceeds 96-bit limit");
	}

	#[test]
	fn max_slot() {
		let max_value: u128 = Slot::MAX_SLOT.into();
		assert_eq!(max_value, (1u128 << 96) - 1);
	}

	#[test]
	fn token_id_conversions() {
		let slot = Slot::MAX_SLOT;
		let owner = H160::from_str("0x8000000000000000000000000000000000000001").unwrap();
		let token_id = TokenId::from((slot, owner));
		assert_eq!(token_id.clone(), TokenId((slot.0, owner.0)));

		let token_id_as_u256 = U256::from(token_id.clone());
		assert_eq!(
			token_id_as_u256.to_string(),
			"115792089237316195423570985007957157034604533206538721623099442498085163368449"
		);

		assert_eq!(
			format!("0x{:x}", token_id_as_u256),
			"0xffffffffffffffffffffffff8000000000000000000000000000000000000001"
		);

		let recovered_token_id = TokenId::from(token_id_as_u256);
		assert_eq!(recovered_token_id, token_id);
	}
}
