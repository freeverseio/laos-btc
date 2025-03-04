use super::collection_id::Brc721CollectionId;
use crate::{Deserialize, Serialize};
use sp_core::H160;
use std::fmt;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721Collection {
	pub collection_id: Brc721CollectionId,
	pub address: H160,
	pub rebaseable: bool,
}

impl Brc721Collection {
	// Constructor function
	pub fn new(collection_id: Brc721CollectionId, address: H160, rebaseable: bool) -> Self {
		Brc721Collection { collection_id, address, rebaseable }
	}
}

impl fmt::Display for Brc721Collection {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} - {:?} - {}", self.collection_id, self.address, self.rebaseable)
	}
}
