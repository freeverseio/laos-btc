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
		let mut ranges = match self.collection_id_to_token_id_range.get(collection_id) {
			Some(existing_ranges) => existing_ranges,
			None => Vec::new(),
		};
		ranges.push(range);
		self.collection_id_to_token_id_range.insert(collection_id, ranges)
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
	use super::{super::mock, *};
	use sp_core::H160;

	fn create_storage() -> Storage<mock::CollectionIdToTokenIdsRange> {
		let collaction_ranges = mock::CollectionIdToTokenIdsRange::new();
		Storage::new(collaction_ranges)
	}

	#[test]
	fn get_registered_ranges_of_unexistent_collection_should_return_none() {
		let storage = create_storage();
		let collection_id = Brc721CollectionId { block: 1, tx: 2 };
		assert!(storage.get_collection_token_id_ranges(&collection_id).is_none());
	}

	#[test]
	fn add_token_range_of_unexistent_collection() {
		let mut storage = create_storage();
		let collection_id = Brc721CollectionId { block: 1, tx: 2 };

		let range =
			TokenIdRange::new(3.try_into().unwrap(), 4.try_into().unwrap(), H160::default());
		assert!(storage.add_token_id_range(&collection_id, range).is_ok());

		let result = storage.get_collection_token_id_ranges(&collection_id).unwrap();
		assert_eq!(result.len(), 1);
	}

	#[test]
	fn add_token_range_to_existing_collection() {
		let mut storage = create_storage();

		let collection_id = Brc721CollectionId { block: 1, tx: 2 };

		// Add first range
		let range1 =
			TokenIdRange::new(3.try_into().unwrap(), 4.try_into().unwrap(), H160::default());
		assert!(storage.add_token_id_range(&collection_id, range1.clone()).is_ok());

		// Add second range to the same collection
		let range2 =
			TokenIdRange::new(5.try_into().unwrap(), 6.try_into().unwrap(), H160::default());
		assert!(storage.add_token_id_range(&collection_id, range2.clone()).is_ok());

		// Verify both ranges are stored
		let result = storage.get_collection_token_id_ranges(&collection_id).unwrap();
		assert_eq!(result.len(), 2);

		// Verify the ranges are stored in the correct order
		assert_eq!(result[0], range1);
		assert_eq!(result[1], range2);
	}
}

