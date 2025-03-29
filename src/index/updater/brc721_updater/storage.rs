use super::{Brc721CollectionId, TokenIdRange};

pub type Result = redb::Result;

pub trait Table<K, V> {
	fn insert(&mut self, key: &K, value: V) -> Result;
	fn get(&self, key: &K) -> Option<V>;
}

struct Storage<T: Table<Brc721CollectionId, Vec<TokenIdRange>>> {
	collection_id_to_token_id_range: T,
}

impl<T: Table<Brc721CollectionId, Vec<TokenIdRange>>> Storage<T> {
	pub fn new(table: T) -> Self {
		Storage { collection_id_to_token_id_range: table }
	}

	fn add_token_id_range(
		&mut self,
		collection_id: &Brc721CollectionId,
		range: TokenIdRange,
	) -> Result {
		//		let current = self.collection_id_to_token_id_range.get(collection_id).ok().unwrap()
		//		self.collection_id_to_token_id_range.insert(collection_id, range)
		Ok(())
	}

	fn get_collection_token_id_ranges(
		&self,
		collection_id: &Brc721CollectionId,
	) -> Option<Vec<TokenIdRange>> {
		self.collection_id_to_token_id_range.get(collection_id)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use sp_core::H160;
	use std::collections::HashMap;

	#[test]
	fn add_token_range_of_unexistent_collection() {
		let mut table: HashMap<Brc721CollectionId, Vec<TokenIdRange>> = HashMap::new();
		let mut storage = Storage::new(table);

		let collection_id = Brc721CollectionId { block: 1, tx: 2 };
		let range =
			TokenIdRange::new(3.try_into().unwrap(), 4.try_into().unwrap(), H160::default());
		storage.add_token_id_range(&collection_id, range);

		let result = storage.get_collection_token_id_ranges(&collection_id).unwrap();
		assert_eq!(result.len(), 1);
	}
}
