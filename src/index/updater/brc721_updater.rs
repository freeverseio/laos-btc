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

use ordinals::{txin_to_h160, RegisterCollection, RegisterOwnership};

use super::*;

pub(super) trait Brc721Table<K, V> {
	fn insert(&mut self, key: K, value: V) -> redb::Result;

	fn get_value(&self, key: K) -> Option<V>;
}

macro_rules! impl_brc721_table {
	($K: ty, $V: ty) => {
		impl Brc721Table<$K, $V> for Table<'_, $K, $V> {
			fn insert(&mut self, key: $K, value: $V) -> redb::Result {
				self.insert(key, value).map(|_| ())
			}

			fn get_value(&self, key: $K) -> Option<$V> {
				let result = self.get(key).ok()?;

				// Convert the AccessGuard to the expected tuple type
				result.map(|guard| guard.value())
			}
		}
	};
}

pub(crate) type RegisterCollectionValue = ([u8; COLLECTION_ADDRESS_LENGTH], bool);

pub(crate) type Brc721TokenId = ([u8; 12], [u8; 20]);
pub(crate) type Brc721TokenInCollection = (Brc721TokenId, Brc721CollectionIdValue);

pub(crate) type TokenScriptOwner = Vec<u8>;

pub(crate) type OwnerUTXOIndex = (String, u128);
pub(crate) type TokenBundles = (Brc721CollectionIdValue, [u8; 20], u128, u128);

impl_brc721_table!(Brc721CollectionIdValue, RegisterCollectionValue);
impl_brc721_table!(Brc721TokenInCollection, TokenScriptOwner);
impl_brc721_table!(OwnerUTXOIndex, TokenBundles);
impl_brc721_table!(String, u128);
impl_brc721_table!((), Vec<TokenScriptOwner>);

pub(super) struct Brc721Updater<'a, T1, T2, T3, T4, T5> {
	pub(super) height: u32,
	pub(super) collection_table: &'a mut T1,
	pub(super) token_owners: &'a mut T2,
	pub(super) token_by_owner: &'a mut T3,
	pub(super) tokens_for_owner: &'a mut T4,
	pub(super) unspent_utxos: &'a mut T5,
}

impl<T1, T2, T3, T4, T5> Brc721Updater<'_, T1, T2, T3, T4, T5>
where
	T1: Brc721Table<Brc721CollectionIdValue, RegisterCollectionValue>,
	T2: Brc721Table<Brc721TokenInCollection, TokenScriptOwner>,
	T3: Brc721Table<OwnerUTXOIndex, TokenBundles>,
	T4: Brc721Table<String, u128>,
	T5: Brc721Table<(), Vec<TokenScriptOwner>>,
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
			self.index_register_ownership(tx, register_ownership)?;
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

		// Get the h160 address contained in the first input or return Ok if it's not
		// P2PKH/P2WPKH(nothing to index)
		let h160_address = if let Ok(address) = txin_to_h160(&tx.input[0]) {
			address
		} else {
			return Ok(());
		};

		for (index, slot_bundle) in register_ownership.slots_bundles.into_iter().enumerate() {
			for slot_range in slot_bundle.0.into_iter() {
				let owner_bytes = tx.output[index + 1].clone().script_pubkey.into_bytes();
				let hex_encoded_owner = hex::encode(&owner_bytes);
				let slot_range_owned_by_owner =
					self.tokens_for_owner.get_value(hex_encoded_owner.clone()).unwrap_or(0);

				let slot_start = *slot_range.start();
				let slot_end = *slot_range.end();

				for slot in slot_range {
					let mut slot_bytes = [0u8; 12];
					slot_bytes.copy_from_slice(&slot.to_le_bytes()[..12]);
					let token_id = (slot_bytes, h160_address.0);
					if self.token_owners.get_value((token_id, collection_id_value)).is_some() {
						return Err(anyhow::anyhow!(format!(
							"Token {:?} already registered",
							(token_id, collection_id_value)
						)));
					}

					self.token_owners
						.insert((token_id, collection_id_value), owner_bytes.clone())?;
				}

				self.token_by_owner.insert(
					(hex_encoded_owner.clone(), slot_range_owned_by_owner),
					(collection_id_value, h160_address.0, slot_start, slot_end),
				)?;

				self.tokens_for_owner.insert(hex_encoded_owner, slot_range_owned_by_owner + 1)?;

				let mut unspent_utxos = self.unspent_utxos.get_value(()).unwrap_or(vec![]);
				if !unspent_utxos.contains(&owner_bytes) {
					unspent_utxos.push(owner_bytes);
					self.unspent_utxos.insert((), unspent_utxos)?;
				}
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::Transaction;
	use ordinals::{btc_address_to_h160, SlotsBundle};
	use sp_core::H160;
	use std::collections::HashMap;

	macro_rules! impl_brc721_table_hashmap {
		($K: ty, $V: ty) => {
			impl Brc721Table<$K, $V> for HashMap<$K, $V> {
				fn insert(&mut self, key: $K, value: $V) -> redb::Result<()> {
					HashMap::insert(self, key, value);
					Ok(())
				}

				fn get_value(&self, key: $K) -> Option<$V> {
					self.get(&key).cloned()
				}
			}
		};
	}

	impl_brc721_table_hashmap!(Brc721CollectionIdValue, RegisterCollectionValue);
	impl_brc721_table_hashmap!(Brc721TokenInCollection, TokenScriptOwner);
	impl_brc721_table_hashmap!(OwnerUTXOIndex, TokenBundles);
	impl_brc721_table_hashmap!(String, u128);
	impl_brc721_table_hashmap!((), Vec<TokenScriptOwner>);

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
		input: Option<Vec<TxIn>>,
	) -> Transaction {
		let input = if let Some(input) = input {
			input
		} else {
			// If not specified, include a valid P2PWKH input
			let pubkey_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
			let pubkey_bytes = hex::decode(pubkey_hex).unwrap();
			// Dummy signature.
			let signature = vec![0x30, 0x45, 0x02, 0x21];
			vec![TxIn {
				previous_output: Default::default(),
				script_sig: Script::new().into(),
				sequence: bitcoin::Sequence(0xffffffff),
				witness: vec![signature, pubkey_bytes.clone()].into(),
			}]
		};

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
			input,
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
		let mut token_owners = HashMap::new();
		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};

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
		let mut token_owners = HashMap::new();
		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};
		let tx_index = 5;
		let tx = empty_tx();

		updater.index_brc721(tx_index, &tx).unwrap();

		assert_eq!(id_to_collection.len(), 0);
	}

	#[test]
	fn test_multiple_transactions() {
		let expected_height = 100u32;
		let mut id_to_collection = HashMap::new();
		let mut token_owners = HashMap::new();

		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};

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
	fn index_register_ownership() {
		let expected_height = 100;
		let mut id_to_collection = HashMap::new();
		let mut token_owners = HashMap::new();

		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};

		let addr_str = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";

		let owner_address =
			Address::from_str(addr_str).unwrap().require_network(Network::Bitcoin).unwrap();

		let h160_signer = btc_address_to_h160(owner_address.clone()).unwrap();

		let transactions = [
			(1, brc721_register_collection_tx(false)),
			(
				2,
				brc721_register_ownership_tx(
					Brc721CollectionId { block: 100, tx: 1 },
					vec![SlotsBundle(vec![(0..=3), (4..=10)])],
					vec![owner_address.clone()],
					None,
				),
			),
		];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap();
		}

		let existing_token_id = ([3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], h160_signer.0);

		let owner_script_buf = ScriptBuf::from_bytes(
			token_owners.get(&(existing_token_id, (100, 1))).unwrap().clone(),
		);

		assert_eq!(owner_script_buf, owner_address.script_pubkey());

		let hex_encoded_owner = hex::encode(owner_address.script_pubkey());
		assert_eq!(
			token_by_owner.get(&(hex_encoded_owner.clone(), 0)).unwrap(),
			&((100u64, 1u32), h160_signer.0, 0u128, 3u128)
		);

		assert_eq!(*tokens_for_owner.get(&hex_encoded_owner).unwrap(), 2u128);
		assert_eq!(
			unspent_utxos.get(&()).unwrap(),
			&vec![owner_address.script_pubkey().into_bytes()]
		);
	}

	#[test]
	fn index_register_ownership_for_not_registered_collection() {
		let expected_height = 100;
		let mut id_to_collection = HashMap::new();
		let mut token_owners = HashMap::new();
		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};

		let transactions = [(
			1,
			brc721_register_ownership_tx(
				Brc721CollectionId { block: 100, tx: 1 },
				vec![],
				vec![],
				None,
			),
		)];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap()
		}

		assert_eq!(token_owners, HashMap::new());
		assert_eq!(tokens_for_owner, HashMap::new());
		assert_eq!(token_by_owner, HashMap::new());
	}

	#[test]
	fn index_register_ownership_with_incorrect_number_of_outputs() {
		let expected_height = 100;
		let mut id_to_collection = HashMap::new();
		let mut token_owners = HashMap::new();

		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};

		let transactions = [
			(1, brc721_register_collection_tx(false)),
			(
				2,
				brc721_register_ownership_tx(
					Brc721CollectionId { block: 100, tx: 1 },
					vec![SlotsBundle(vec![(0..=3), (4..=10)])],
					vec![],
					None,
				),
			),
		];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap()
		}

		assert_eq!(token_owners, HashMap::new());
		assert_eq!(tokens_for_owner, HashMap::new());
		assert_eq!(token_by_owner, HashMap::new());
	}

	#[test]
	fn index_register_ownership_with_invalid_input() {
		let expected_height = 100;
		let mut id_to_collection = HashMap::new();
		let mut token_owners = HashMap::new();

		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};
		let addr_str = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
		let owner_address =
			Address::from_str(addr_str).unwrap().require_network(Network::Bitcoin).unwrap();

		let txin = TxIn {
			previous_output: Default::default(),
			script_sig: Script::new().into(),
			sequence: bitcoin::Sequence(0xffffffff),
			witness: vec![vec![0x30, 0x45, 0x02, 0x21]].into(), // Only one element.
		};

		let transactions = [
			(1, brc721_register_collection_tx(false)),
			(
				2,
				brc721_register_ownership_tx(
					Brc721CollectionId { block: 100, tx: 1 },
					vec![SlotsBundle(vec![(0..=3), (4..=10)])],
					vec![owner_address.clone(), owner_address],
					Some(vec![txin]),
				),
			),
		];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap()
		}

		assert_eq!(token_owners, HashMap::new());

		assert_eq!(tokens_for_owner, HashMap::new());
		assert_eq!(token_by_owner, HashMap::new());
	}

	#[test]
	fn index_register_ownership_already_registered_ownership() {
		let expected_height = 100;
		let mut id_to_collection = HashMap::new();
		let mut token_owners = HashMap::new();

		let mut tokens_for_owner = HashMap::new();
		let mut token_by_owner = HashMap::new();
		let mut unspent_utxos = HashMap::new();

		let mut updater = Brc721Updater {
			height: expected_height,
			collection_table: &mut id_to_collection,
			token_owners: &mut token_owners,
			tokens_for_owner: &mut tokens_for_owner,
			token_by_owner: &mut token_by_owner,
			unspent_utxos: &mut unspent_utxos,
		};

		let addr_str = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
		let owner_address =
			Address::from_str(addr_str).unwrap().require_network(Network::Bitcoin).unwrap();

		let transactions = [
			(1, brc721_register_collection_tx(false)),
			(
				2,
				brc721_register_ownership_tx(
					Brc721CollectionId { block: 100, tx: 1 },
					vec![SlotsBundle(vec![(0..=3), (4..=10)])],
					vec![owner_address.clone()],
					None,
				),
			),
		];

		for (tx_index, tx) in transactions.iter() {
			updater.index_brc721(*tx_index, tx).unwrap()
		}

		assert!(updater
			.index_brc721(
				3,
				&brc721_register_ownership_tx(
					Brc721CollectionId { block: 100, tx: 1 },
					vec![SlotsBundle(vec![std::ops::RangeInclusive::new(3, 3)])],
					vec![owner_address.clone()],
					None,
				)
			)
			.is_err());
	}
}
