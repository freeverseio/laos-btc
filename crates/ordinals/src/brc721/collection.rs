use super::collection_id::Brc721CollectionId;
use crate::{Deserialize, Serialize};
use sp_core::H160;
use std::fmt;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721Collection {
	pub id: Brc721CollectionId,
	#[serde(rename = "LAOS_address")]
	pub laos_address: H160,
	pub rebaseable: bool,
}

impl Brc721Collection {
	// Constructor function
	pub fn new(id: Brc721CollectionId, laos_address: H160, rebaseable: bool) -> Self {
		Brc721Collection { id, laos_address, rebaseable }
	}
}

impl fmt::Display for Brc721Collection {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} - {:?} - {}", self.id, self.laos_address, self.rebaseable)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use derive_more::FromStr;

	#[test]
	fn test_brc721_collection_display() {
		let collection_id = Brc721CollectionId::default(); // Create a collection ID
		let address = H160::from_str("0x0000000000000000000000000000000000000001").unwrap();
		let rebaseable = false;

		let collection = Brc721Collection::new(collection_id, address, rebaseable);

		assert_eq!(
			format!("{}", collection),
			"0:0 - 0x0000000000000000000000000000000000000001 - false"
		);
	}
}
