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

use ordinals::{btc_address_to_h160, RegisterCollection, RegisterOwnership, SlotsBundle};

use super::*;

pub(crate) type RegisterCollectionValue = ([u8; COLLECTION_ADDRESS_LENGTH], bool);

pub(super) struct Brc721Updater<'a, 'client, 'tx> {
	pub(super) height: u32,
	pub(super) client: &'client Client,
	pub(super) collection_table:
		&'a mut Table<'tx, Brc721CollectionIdValue, RegisterCollectionValue>,
}

impl Brc721Updater<'_, '_, '_> {
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
		if self.collection_table.get(collection_id_value).ok().is_none() {
			return Ok(());
		}
		// If the tx doesn't include enough outputs, there's nothing to index
		if tx.output.len() < register_ownership.slots_bundles.len() + 1 {
			return Ok(());
		}

		let h160_address = if let Ok(address) = self
			.client
			.get_raw_transaction_info(&tx.input[0].previous_output.txid, None)
			.map(|raw_transaction| {
				let script = raw_transaction.vout[tx.input[0].previous_output.vout as usize]
					.script_pub_key
					.clone();
				match script.address {
					Some(address) =>
						if let Ok(address) = btc_address_to_h160(
							address.require_network(Network::Bitcoin).expect("Non BTC address"),
						) {
							Ok(address)
						} else {
							Err(())
						},
					_ => Err(()),
				}
			}) {
			address
		} else {
			return Ok(());
		};

		Ok(())
	}
}
