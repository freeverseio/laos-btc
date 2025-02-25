// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use ordinals::RegisterCollection;

use super::*;

pub(super) trait Insertable<K, V> {
	fn insert(&mut self, key: K, value: V) -> redb::Result;
}

impl Insertable<Brc721CollectionIdValue, RegisterCollectionValue>
	for Table<'_, Brc721CollectionIdValue, RegisterCollectionValue>
{
	fn insert(
		&mut self,
		key: Brc721CollectionIdValue,
		value: RegisterCollectionValue,
	) -> redb::Result {
		self.insert(key, value).map(|_| ())
	}
}

pub(crate) type RegisterCollectionValue = ([u8; COLLECTION_ADDRESS_LENGTH], bool);

pub(super) struct Brc721Updater<'a, T> {
	pub(super) height: u32,
	pub(super) collection_table: &'a mut T,
}

impl<T> Brc721Updater<'_, T>
where
	T: Insertable<Brc721CollectionIdValue, RegisterCollectionValue>,
{
	/// Indexes collections from a transaction.
	///
	/// # Arguments
	/// * `tx_index` - The index of the transaction within its block.
	/// * `tx` - The transaction to process.
	pub(super) fn index_collections(&mut self, tx_index: u32, tx: &Transaction) -> Result<()> {
		// Ensure the transaction has at least one output.
		if tx.output.is_empty() {
			log::warn!("Failed to decode register collection: Output not found");
			return Ok(());
		}

		// TODO what if the output.let() > 1 ???? we just skip the others ?
		let first_output = tx.output[0].clone();

		// Decode the register collection from the first output's script public key.
		match RegisterCollection::decode(&first_output.script_pubkey) {
			Ok(register_collection) => {
				self.collection_table.insert(
					(self.height.into(), tx_index),
					(register_collection.address.into(), register_collection.rebaseable),
				)?;
			},
			Err(e) => {
				log::warn!("Failed to decode register collection: {:?}", e);
			},
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::Transaction;
	use sp_core::H160;
	use std::collections::HashMap;

	impl Insertable<Brc721CollectionIdValue, RegisterCollectionValue>
		for HashMap<Brc721CollectionIdValue, RegisterCollectionValue>
	{
		fn insert(
			&mut self,
			key: Brc721CollectionIdValue,
			value: RegisterCollectionValue,
		) -> redb::Result<()> {
			HashMap::insert(self, key, value);
			Ok(())
		}
	}

	const COLLECTION_ADDRESS: [u8; COLLECTION_ADDRESS_LENGTH] = [0x2A; COLLECTION_ADDRESS_LENGTH];

	fn brc721_collection_tx(rebaseable: bool) -> Transaction {
		let collection =
			RegisterCollection { address: H160::from_slice(&COLLECTION_ADDRESS), rebaseable };

		let output = TxOut { value: Amount::ONE_SAT, script_pubkey: collection.clone().encode() };

		Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![output],
		}
	}

	fn empty_tx() -> Transaction {
		Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![],
		}
	}

	#[test]
	fn test_one_collection() {
		let expected_height = 100u32;
		let expected_rebaseable = true;
		let expected_tx_index = 5;

		let mut id_to_collection = HashMap::new();

		let mut updater =
			Brc721Updater { height: expected_height, collection_table: &mut id_to_collection };

		let tx = brc721_collection_tx(expected_rebaseable);

		updater.index_collections(expected_tx_index, &tx).unwrap();

		assert_eq!(id_to_collection.len(), 1);
		let key = (expected_height.into(), expected_tx_index);
		assert!(id_to_collection.contains_key(&key));

		let (address, rebaseable) = id_to_collection.get(&key).unwrap();
		assert_eq!(*address, COLLECTION_ADDRESS);
		assert_eq!(*rebaseable, expected_rebaseable);
	}

	#[test]
	fn test_no_collections() {
		let expected_height = 100u32;
		let mut id_to_collection = HashMap::new();

		let mut updater =
			Brc721Updater { height: expected_height, collection_table: &mut id_to_collection };

		let tx_index = 5;
		let tx = empty_tx();

		updater.index_collections(tx_index, &tx).unwrap();

		assert_eq!(id_to_collection.len(), 0);
	}

	#[test]
	fn test_multiple_transactions() {
		let expected_height = 100u32;
		let mut id_to_collection = HashMap::new();

		let mut updater =
			Brc721Updater { height: expected_height, collection_table: &mut id_to_collection };

		let transactions =
			[(0, brc721_collection_tx(true)), (1, brc721_collection_tx(false)), (2, empty_tx())];

		for (tx_index, tx) in transactions.iter() {
			updater.index_collections(*tx_index, tx).unwrap();
		}

		assert_eq!(id_to_collection.len(), 2);
		assert!(id_to_collection.contains_key(&(expected_height.into(), 0)));
		assert!(id_to_collection.contains_key(&(expected_height.into(), 1)));
		assert!(!id_to_collection.contains_key(&(expected_height.into(), 2)));

		assert!(id_to_collection.get(&(expected_height.into(), 0)).unwrap().1);
		assert_eq!(
			id_to_collection.get(&(expected_height.into(), 0)).unwrap().0,
			COLLECTION_ADDRESS
		);
		assert!(!id_to_collection.get(&(expected_height.into(), 1)).unwrap().1);
		assert_eq!(
			id_to_collection.get(&(expected_height.into(), 0)).unwrap().0,
			COLLECTION_ADDRESS
		);
	}
}
