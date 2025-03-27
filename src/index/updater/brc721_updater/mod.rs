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

mod token_id_range;

use ordinals::{
	brc721::is_brc721_script, btc_address_to_h160, RegisterCollection, RegisterOwnership, Slot,
	TokenId,
};
use token_id_range::TokenIdRange;

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
// [u8; 32] (token_bundle.2) is the tx hash
// u32      (token_bundle.3) is the tx output index
// u128     (token_bundle.4) is the slot start
// u128     (token_bundle.5) is the slot end
pub(crate) type TokenBundles = (Brc721CollectionIdValue, [u8; 20], [u8; 32], u32, u128, u128);

impl_brc721_table!(Brc721CollectionIdValue, RegisterCollectionValue);
impl_brc721_table!(Brc721TokenInCollection, TokenScriptOwner);
impl_brc721_table!(OwnerUTXOIndex, TokenBundles);
impl_brc721_table!(String, u128);
impl_brc721_table!((), Vec<TokenScriptOwner>);
impl_brc721_table!(Brc721CollectionId, Vec<TokenIdRange>);

pub(super) struct Brc721Updater<'a, 'client, T1, T2, T3, T4, T5> {
	pub(super) height: u32,
	pub(super) client: &'client Client,
	pub(super) collection_table: &'a mut T1,
	pub(super) token_owners: &'a mut T2,
	pub(super) token_by_owner: &'a mut T3,
	pub(super) tokens_for_owner: &'a mut T4,
	pub(super) unspent_utxos: &'a mut T5,
}

impl<T1, T2, T3, T4, T5> Brc721Updater<'_, '_, T1, T2, T3, T4, T5>
where
	T1: Brc721Table<Brc721CollectionIdValue, RegisterCollectionValue>,
	T2: Brc721Table<Brc721TokenInCollection, TokenScriptOwner>,
	T3: Brc721Table<OwnerUTXOIndex, TokenBundles>,
	T4: Brc721Table<String, u128>,
	T5: Brc721Table<(), Vec<TokenScriptOwner>>,
{
	pub(super) fn index_brc721(&mut self, tx_index: u32, tx: &Transaction) -> Result<()> {
		if tx.output.is_empty() {
			return Ok(());
		}

		let first_output_script = tx.output[0].clone().script_pubkey;

		// early return if the script is not a BRC721 script.
		if !is_brc721_script(&first_output_script) {
			return Ok(());
		}

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

		let tx_info = if let Ok(tx_info) =
			self.client.get_raw_transaction_info(&tx.input[0].previous_output.txid, None)
		{
			tx_info
		} else {
			return Ok(());
		};

		let script_buf = tx_info.vout[tx.input[0].previous_output.vout.into_usize()]
			.script_pub_key
			.script()?;

		let network = match self.client.get_blockchain_info() {
			Ok(blockchain_info) => blockchain_info.chain,
			_ => Network::Bitcoin,
		};

		let address = if let Ok(address) = Address::from_script(&script_buf, network) {
			address
		} else {
			return Ok(());
		};

		// Get the h160 address contained in the first input or return Ok if it's not
		// P2PKH/P2WPKH/P2TR(nothing to index)
		let h160_address = if let Ok(address) = btc_address_to_h160(address) {
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
					let token_id = TokenId::from((
						Slot::try_from(slot).map_err(|err| anyhow::anyhow!(err))?,
						h160_address,
					));
					if self.token_owners.get_value((token_id.0, collection_id_value)).is_some() {
						return Err(anyhow::anyhow!(format!(
							"Token {:?} already registered",
							(token_id, collection_id_value)
						)));
					}

					self.token_owners
						.insert((token_id.0, collection_id_value), owner_bytes.clone())?;
				}

				let tx_out_idx = u32::try_from(index + 1)?;
				self.token_by_owner.insert(
					(hex_encoded_owner.clone(), slot_range_owned_by_owner),
					(
						collection_id_value,
						h160_address.0,
						*tx.compute_txid().as_ref(),
						tx_out_idx,
						slot_start,
						slot_end,
					),
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
