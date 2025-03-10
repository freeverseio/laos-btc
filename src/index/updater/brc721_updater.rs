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

use ordinals::{RegisterCollection, RegisterOwnership, SlotsBundle};

use super::*;

pub(super) trait Brc721Table<K, V> {
	fn insert(&mut self, key: K, value: V) -> redb::Result;

	fn get_value(&self, key: K) -> Option<V>;
}

impl Brc721Table<Brc721CollectionIdValue, RegisterCollectionValue>
	for Table<'_, Brc721CollectionIdValue, RegisterCollectionValue>
{
	fn insert(
		&mut self,
		key: Brc721CollectionIdValue,
		value: RegisterCollectionValue,
	) -> redb::Result {
		self.insert(key, value).map(|_| ())
	}

	fn get_value(&self, key: Brc721CollectionIdValue) -> Option<RegisterCollectionValue> {
		let result = self.get(key).ok()?;

		// Convert the AccessGuard to the expected tuple type
		result.map(|guard| guard.value())
	}
}

pub(crate) type RegisterCollectionValue = ([u8; COLLECTION_ADDRESS_LENGTH], bool);

pub(super) struct Brc721Updater<'a, T> {
	pub(super) height: u32,
	pub(super) collection_table: &'a mut T,
}

impl<T> Brc721Updater<'_, T>
where
	T: Brc721Table<Brc721CollectionIdValue, RegisterCollectionValue>,
{
	/// Indexes a brc721 operation from a transaction
	pub(super) fn index_brc721(&mut self, tx_index: u32, tx: &Transaction) -> Result<()> {
		if tx.output.is_empty() {
			return Ok(())
		}

		let first_output_script = tx.output[0].clone().script_pubkey;

		if let Ok(register_collection) = RegisterCollection::from_script(&first_output_script) {
			self.index_register_collections(tx_index, register_collection)?;
		} else if let Ok(register_ownership) = first_output_script.try_into() {
			self.index_register_ownership(tx_index, tx, register_ownership)?;
		}

		Ok(())
	}

	/// Indexes a register collection operation from a RegisterCollection.
	fn index_register_collections(
		&mut self,
		tx_index: u32,
		register_collection: RegisterCollection,
	) -> Result<()> {
		self.collection_table.insert(
			(self.height.into(), tx_index),
			(register_collection.address.into(), register_collection.rebaseable),
		)?;
		Ok(())
	}

	/// Indexes a register ownership operation from a RegisterOwnership and the related
	/// Transaction.
	fn index_register_ownership(
		&mut self,
		tx_index: u32,
		tx: &Transaction,
		register_ownership: RegisterOwnership,
	) -> Result<()> {
		let collection_id_value =
			(register_ownership.collection_id.block, register_ownership.collection_id.tx);

		// If the collection isn't registered, there's nothing to index
		if self.collection_table.get_value(collection_id_value).is_none() {
			return Ok(());
		}

		// If the tx doesn't include enough outputs, there's nothing to index
		if tx.output.len() < register_ownership.slots_bundles.len() + 1 {
			return Ok(());
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

	impl Brc721Table<Brc721CollectionIdValue, RegisterCollectionValue>
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

		fn get_value(&self, key: Brc721CollectionIdValue) -> Option<RegisterCollectionValue> {
			self.get(&key).copied()
		}
	}

	const COLLECTION_ADDRESS: [u8; COLLECTION_ADDRESS_LENGTH] = [0x2A; COLLECTION_ADDRESS_LENGTH];

	fn brc721_register_collection_tx(rebaseable: bool) -> Transaction {
		let collection =
			RegisterCollection { address: H160::from_slice(&COLLECTION_ADDRESS), rebaseable };

		let output =
			TxOut { value: Amount::ONE_SAT, script_pubkey: collection.clone().to_script() };

		Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![],
			output: vec![output],
		}
	}

	fn brc721_register_ownership_tx(
		collection_id: Brc721CollectionId,
		slots_bundles: Vec<SlotsBundle>,
		owners: Vec<Address>,
	) -> Transaction {
		let output_0 = TxOut {
			value: Amount::ONE_SAT,
			script_pubkey: RegisterOwnership { collection_id, slots_bundles }.into(),
		};

		// Create outputs for each owner.
		// Each owner’s output uses the script generated from the address.
		let owner_outputs: Vec<TxOut> = owners
			.iter()
			.map(|owner| {
				let owner_script = owner.script_pubkey();
				TxOut { value: Amount::ONE_SAT, script_pubkey: owner_script }
			})
			.collect();

		let mut output = vec![output_0];
		output.extend(owner_outputs);

		Transaction {
			version: Version(1),
			lock_time: LockTime::from_height(1000).unwrap(),
			input: vec![], // TODO: Include input asap
			output,
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

		let tx = brc721_register_collection_tx(expected_rebaseable);

		updater.index_brc721(expected_tx_index, &tx).unwrap();

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

		updater.index_brc721(tx_index, &tx).unwrap();

		assert_eq!(id_to_collection.len(), 0);
	}

	#[test]
	fn test_multiple_transactions() {
		let expected_height = 100u32;
		let mut id_to_collection = HashMap::new();

		let mut updater =
			Brc721Updater { height: expected_height, collection_table: &mut id_to_collection };

		let transactions = [
			(0, brc721_register_collection_tx(true)),
			(1, brc721_register_collection_tx(false)),
			(2, empty_tx()),
		];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap();
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

	#[test]
	fn index_register_ownership_for_not_registered_collection() {
		let mut id_to_collection = HashMap::new();
		let mut updater = Brc721Updater { height: 100u32, collection_table: &mut id_to_collection };

		let transactions = [(
			0,
			brc721_register_ownership_tx(Brc721CollectionId { block: 100, tx: 1 }, vec![], vec![]),
		)];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap()
		}

		// TODO: Assert that register ownership tables are untouched, once we define them
	}

	#[test]
	fn index_register_ownership_with_incorrect_number_of_outputs() {
		let mut id_to_collection = HashMap::new();
		let mut updater = Brc721Updater { height: 100u32, collection_table: &mut id_to_collection };

		let transactions = [
			(0, brc721_register_collection_tx(false)),
			(
				1,
				brc721_register_ownership_tx(
					Brc721CollectionId { block: 100, tx: 1 },
					vec![SlotsBundle(vec![(0..=3), (4..=10)])],
					vec![],
				),
			),
		];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap()
		}

		// TODO: Assert that register ownership tables are untouched, once we define them
	}
}
