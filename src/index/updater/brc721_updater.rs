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

impl Insertable<RuneIdValue, RegisterCollectionValue>
	for Table<'_, RuneIdValue, RegisterCollectionValue>
{
	fn insert(&mut self, key: RuneIdValue, value: RegisterCollectionValue) -> redb::Result {
		self.insert(key, value).map(|_| ())
	}
}

pub(crate) type RegisterCollectionValue = ([u8; COLLECTION_ADDRESS_LENGTH], bool);

pub(super) struct Brc721CollectionUpdater<'a, T> {
	pub(super) event_sender: Option<&'a mpsc::Sender<Event>>,
	pub(super) height: u32,
	pub(super) id_to_collection: &'a mut T,
}

impl<T> Brc721CollectionUpdater<'_, T>
where
	T: Insertable<RuneIdValue, RegisterCollectionValue>,
{
	pub(super) fn index_collections(
		&mut self,
		tx_index: u32,
		tx: &Transaction,
		txid: Txid,
	) -> Result<()> {
		if let Some(register_collection) = RegisterCollection::decipher(tx) {
			self.id_to_collection.insert(
				(self.height.into(), tx_index),
				(register_collection.address.into(), register_collection.rebaseable),
			)?;

			if let Some(sender) = self.event_sender {
				sender.blocking_send(Event::Brc721CollectionCreated {
					txid,
					collection_id: RuneId { block: self.height.into(), tx: tx_index },
				})?;
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::{Transaction, Txid};
	use sp_core::H160;
	use std::collections::HashMap;
	use tokio::sync::mpsc;

	impl Insertable<RuneIdValue, RegisterCollectionValue>
		for HashMap<RuneIdValue, RegisterCollectionValue>
	{
		fn insert(&mut self, key: RuneIdValue, value: RegisterCollectionValue) -> redb::Result<()> {
			HashMap::insert(self, key, value);
			Ok(())
		}
	}

	const COLLECTION_ADDRESS: [u8; COLLECTION_ADDRESS_LENGTH] = [0x2A; COLLECTION_ADDRESS_LENGTH];

	fn brc721_collection_tx(rebaseable: bool) -> Transaction {
		let collection =
			RegisterCollection { address: H160::from_slice(&COLLECTION_ADDRESS), rebaseable };

		let script_buf = collection.encipher();

		let output = TxOut { value: Amount::ONE_SAT, script_pubkey: script_buf };

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
		let expected_txid = Txid::all_zeros();

		let (sender, mut receiver) = mpsc::channel(1000);
		let mut id_to_collection = HashMap::new();

		let mut updater = Brc721CollectionUpdater {
			event_sender: Some(&sender),
			height: expected_height,
			id_to_collection: &mut id_to_collection,
		};

		let tx = brc721_collection_tx(expected_rebaseable);

		updater.index_collections(expected_tx_index, &tx, expected_txid).unwrap();

		assert_eq!(id_to_collection.len(), 1);
		let key = (expected_height.into(), expected_tx_index);
		assert!(id_to_collection.contains_key(&key));

		let (address, rebaseable) = id_to_collection.get(&key).unwrap();
		assert_eq!(*address, COLLECTION_ADDRESS);
		assert_eq!(*rebaseable, expected_rebaseable);

		let event = receiver.try_recv().unwrap();
		match event {
			Event::Brc721CollectionCreated { txid: event_txid, collection_id } => {
				assert_eq!(event_txid, expected_txid);
				assert_eq!(collection_id.block, u64::from(expected_height));
				assert_eq!(collection_id.tx, expected_tx_index);
			},
			_ => panic!("Unexpected event type"),
		}
	}

	#[test]
	fn test_no_collections() {
		let expected_height = 100u32;

		let (sender, mut receiver) = mpsc::channel(1000);
		let mut id_to_collection = HashMap::new();

		let mut updater = Brc721CollectionUpdater {
			event_sender: Some(&sender),
			height: expected_height,
			id_to_collection: &mut id_to_collection,
		};

		let tx_index = 5;
		let tx = empty_tx();
		let txid = Txid::all_zeros();

		updater.index_collections(tx_index, &tx, txid).unwrap();

		assert_eq!(id_to_collection.len(), 0);

		assert!(receiver.try_recv().is_err());
	}

	#[test]
	fn test_multiple_transactions() {
		let expected_height = 100u32;
		let expected_txid = Txid::all_zeros();
		let (sender, mut receiver) = mpsc::channel(1000);
		let mut id_to_collection = HashMap::new();

		let mut updater = Brc721CollectionUpdater {
			event_sender: Some(&sender),
			height: expected_height,
			id_to_collection: &mut id_to_collection,
		};

		let transactions =
			[(0, brc721_collection_tx(true)), (1, brc721_collection_tx(false)), (2, empty_tx())];

		for (tx_index, tx) in transactions.iter() {
			updater.index_collections(*tx_index, tx, expected_txid).unwrap();
		}

		assert_eq!(id_to_collection.len(), 2);
		assert!(id_to_collection.contains_key(&(expected_height.into(), 0)));
		assert!(id_to_collection.contains_key(&(expected_height.into(), 1)));
		assert!(!id_to_collection.contains_key(&(expected_height.into(), 2)));

		assert!(id_to_collection.get(&(expected_height.into(), 0)).unwrap().1);
		assert!(!id_to_collection.get(&(expected_height.into(), 1)).unwrap().1);

		let event = receiver.try_recv().unwrap();
		match event {
			Event::Brc721CollectionCreated { txid: event_txid, collection_id } => {
				assert_eq!(event_txid, expected_txid);
				assert_eq!(collection_id.block, u64::from(expected_height));
				assert_eq!(collection_id.tx, 0);
			},
			_ => panic!("Unexpected event type"),
		}
		let event = receiver.try_recv().unwrap();
		match event {
			Event::Brc721CollectionCreated { txid: event_txid, collection_id } => {
				assert_eq!(event_txid, expected_txid);
				assert_eq!(collection_id.block, u64::from(expected_height));
				assert_eq!(collection_id.tx, 1);
			},
			_ => panic!("Unexpected event type"),
		}
	}

	#[test]
	fn test_event_sending_failure() {
		// receiver dropped to simulate sending failure
		let (sender, _) = mpsc::channel::<Event>(1000);
		let mut id_to_collection = HashMap::new();

		let mut updater = Brc721CollectionUpdater {
			event_sender: Some(&sender),
			height: 100,
			id_to_collection: &mut id_to_collection,
		};

		let tx_index = 5;
		let tx = brc721_collection_tx(true);
		let txid = Txid::all_zeros();

		let result = updater.index_collections(tx_index, &tx, txid);

		assert!(result.is_err());
		assert_eq!(id_to_collection.len(), 1);
	}
}
