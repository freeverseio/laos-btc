use super::collection_id::Brc721CollectionId;
use crate::{Deserialize, Serialize};
use sp_core::H160;
use std::fmt;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721Collection(pub Brc721CollectionId, pub H160, pub bool);

impl fmt::Display for Brc721Collection {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} - {:?} - {}", self.0, self.1, self.2)
	}
}
