use ordinals::brc721::RangeData;

use super::{
	storage::{Result, Table},
	Brc721CollectionId, TokenIdRange,
};
use std::collections::HashMap;

pub type CollectionIdToTokenIdsRange = HashMap<Brc721CollectionId, Vec<TokenIdRange>>;

impl Table<Brc721CollectionId, Vec<TokenIdRange>> for CollectionIdToTokenIdsRange {
	fn insert(&mut self, key: &Brc721CollectionId, value: Vec<TokenIdRange>) -> Result {
		HashMap::insert(self, key.clone(), value);
		Ok(())
	}
	fn get(&self, key: &Brc721CollectionId) -> Option<Vec<TokenIdRange>> {
		HashMap::get(self, key).cloned()
	}
}
pub type TokenIdsRangeToData = HashMap<TokenIdRange, RangeData>;

impl Table<TokenIdRange, RangeData> for TokenIdsRangeToData {
	fn insert(&mut self, key: &TokenIdRange, value: RangeData) -> Result {
		HashMap::insert(self, key.clone(), value);
		Ok(())
	}
	fn get(&self, key: &TokenIdRange) -> Option<RangeData> {
		HashMap::get(self, key).cloned()
	}
}
