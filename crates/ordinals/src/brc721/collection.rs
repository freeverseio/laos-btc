use super::collection_id::Brc721CollectionId;
use sp_core::H160;

pub struct Brc721Collection(pub Brc721CollectionId, pub H160, pub bool);
