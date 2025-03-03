use super::collection_id::Brc721CollectionId;
use crate::{Deserialize, Serialize};
use sp_core::H160;

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721Collection(pub Brc721CollectionId, pub H160, pub bool);
