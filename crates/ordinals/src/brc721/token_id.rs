use sp_core::{H160, U256};

/// TokenId type
/// every slot is identified by a unique `asset_id = concat(slot #, owner_address)`
#[allow(dead_code)] // TODO: remove this when used
pub type TokenId = U256;

/// Slot type - 96-bit unsigned integer
#[derive(Eq, PartialEq, Clone, Copy, Default, PartialOrd, Ord, Hash, Debug)]
pub struct Slot([u8; 12]);
impl Slot {
	/// Maximum value for a 96-bit unsigned integer
	pub const MAX_SLOT: Slot = Slot([0xFF; 12]);

	pub fn new(bytes: [u8; 12]) -> Self {
		Slot(bytes)
	}

	pub fn to_be_bytes(&self) -> [u8; 12] {
		let mut bytes = [0u8; 12];
		let slot_u128: u128 = (*self).into();
		let slot_bytes = slot_u128.to_be_bytes();
		bytes.copy_from_slice(&slot_bytes[4..]); // Copy the last 12 bytes
		bytes
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

/// Converts `Slot` and `H160` to `TokenId`
///
/// Every slot is identified by a unique `token_id` where `token_id = concat(slot #,
/// owner_address)`
///
/// Returns `Slot`
#[allow(dead_code)] // TODO: remove this when used
fn slot_and_owner_to_token_id(slot: Slot, owner: H160) -> TokenId {
	let mut bytes = [0u8; 32];

	let slot_bytes = slot.to_be_bytes();

	// Copy the slot into the first 12 bytes of the array
	bytes[..12].copy_from_slice(&slot_bytes);
	// Copy the owner address bytes into the array
	bytes[12..].copy_from_slice(&owner.0);

	TokenId::from_big_endian(&bytes)
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
	fn slot_and_owner_to_token_id_works() {
		let slot = Slot::try_from(1).unwrap();
		let owner = H160::from_str("0xf2188656f04bc18138144c734bed1bf3782e59b8").unwrap();
		let token_id = slot_and_owner_to_token_id(slot, owner);
		assert_eq!(
			format!("0x{:064x}", token_id),
			"0x000000000000000000000001f2188656f04bc18138144c734bed1bf3782e59b8"
		);
		assert_eq!(token_id.to_string(), "2843624324385043295092873104609319246794557643192");
	}
}
