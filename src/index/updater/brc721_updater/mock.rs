use super::{
	storage::{Result, Table},
	Brc721CollectionId, TokenIdRange,
};
use std::collections::HashMap;

pub type CollectionIdToTokenIdsRange = HashMap<Brc721CollectionId, Vec<TokenIdRange>>;

impl Table<Brc721CollectionId, Vec<TokenIdRange>> for CollectionIdToTokenIdsRange {
	fn insert(&mut self, key: &Brc721CollectionId, value: Vec<TokenIdRange>) -> Result {
		self.insert(key.clone(), value);
		Ok(())
	}
	fn get(&self, key: &Brc721CollectionId) -> Option<Vec<TokenIdRange>> {
		self.get(key).cloned()
	}
}
