use redb::StorageError;

use super::{Brc721CollectionId, RangeData, TokenIdRange};

pub type Result = redb::Result;

pub trait Table<K, V> {
	fn insert(&mut self, key: &K, value: V) -> Result;
	fn get(&self, key: &K) -> Option<V>;
}

struct Storage<T0, T1>
where
	T0: Table<Brc721CollectionId, Vec<TokenIdRange>>,
	T1: Table<TokenIdRange, RangeData>,
{
	collection_id_to_token_id_range: T0,
	token_id_range_to_range_data: T1,
}

impl<T0: Table<Brc721CollectionId, Vec<TokenIdRange>>, T1: Table<TokenIdRange, RangeData>>
	Storage<T0, T1>
{
	pub fn new(t0: T0, t1: T1) -> Self {
		Storage { collection_id_to_token_id_range: t0, token_id_range_to_range_data: t1 }
	}

	fn add_token_id_range(
		&mut self,
		collection_id: &Brc721CollectionId,
		range: TokenIdRange,
	) -> Result {
		let mut ranges =
			self.collection_id_to_token_id_range.get(collection_id).unwrap_or_else(Vec::new);

		// Check for intersection with existing ranges.  Return an error if an intersection exists.
		for existing_range in &ranges {
			if existing_range.overlaps(&range) {
				return Err(StorageError::PreviousIo); // TODO create a proper error
			}
		}

		ranges.push(range.clone());

		let range_data = RangeData {};

		self.token_id_range_to_range_data.insert(&range, range_data)?;
		self.collection_id_to_token_id_range.insert(collection_id, ranges)
	}

	fn get_token_id_ranges(&self, collection_id: &Brc721CollectionId) -> Option<Vec<TokenIdRange>> {
		self.collection_id_to_token_id_range.get(collection_id)
	}
}

#[cfg(test)]
mod test {
	use super::{super::mock, *};
	use sp_core::H160;

	fn create_storage() -> Storage<mock::CollectionIdToTokenIdsRange, mock::TokenIdsRangeToData> {
		let collaction_ranges = mock::CollectionIdToTokenIdsRange::new();
		let range_data = mock::TokenIdsRangeToData::new();
		Storage::new(collaction_ranges, range_data)
	}

	#[test]
	fn get_registered_ranges_of_unexistent_collection_should_return_none() {
		let storage = create_storage();
		let collection_id = Brc721CollectionId { block: 1, tx: 2 };
		assert!(storage.get_token_id_ranges(&collection_id).is_none());
	}

	#[test]
	fn add_token_range_of_unexistent_collection() {
		let mut storage = create_storage();
		let collection_id = Brc721CollectionId { block: 1, tx: 2 };

		let range =
			TokenIdRange::new(3.try_into().unwrap(), 4.try_into().unwrap(), H160::default());
		assert!(storage.add_token_id_range(&collection_id, range).is_ok());

		let result = storage.get_token_id_ranges(&collection_id).unwrap();
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
		let result = storage.get_token_id_ranges(&collection_id).unwrap();
		assert_eq!(result.len(), 2);

		// Verify the ranges are stored in the correct order
		assert_eq!(result[0], range1);
		assert_eq!(result[1], range2);
	}

	#[test]
	fn add_token_range_with_already_registered_tokens() {
		let mut storage = create_storage();

		let collection_id = Brc721CollectionId { block: 1, tx: 2 };

		// Add first range
		let range1 =
			TokenIdRange::new(3.try_into().unwrap(), 8.try_into().unwrap(), H160::default());
		assert!(storage.add_token_id_range(&collection_id, range1.clone()).is_ok());

		// Add second range to the same collection
		let range2 =
			TokenIdRange::new(8.try_into().unwrap(), 9.try_into().unwrap(), H160::default());
		assert!(storage.add_token_id_range(&collection_id, range2.clone()).is_err());

		let result = storage.get_token_id_ranges(&collection_id).unwrap();
		assert_eq!(result.len(), 1);

		assert_eq!(result[0], range1);
	}
}
