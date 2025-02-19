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

use super::*;

pub(super) trait Insertable<K, V> {
	fn insert(&mut self, key: K, value: V) -> redb::Result;
}

impl Insertable<RuneIdValue, LaosCollectionValue> for Table<'_, RuneIdValue, LaosCollectionValue> {
	fn insert(&mut self, key: RuneIdValue, value: LaosCollectionValue) -> redb::Result {
		self.insert(key, value).map(|_| ())
	}
}

pub(crate) type LaosCollectionValue = ([u8; COLLECTION_ADDRESS_LENGTH], bool);

pub(super) struct LaosCollectionUpdater<'a, T> {
	pub(super) event_sender: Option<&'a mpsc::Sender<Event>>,
	pub(super) height: u32,
	pub(super) id_to_collection: &'a mut T,
}

impl<T> LaosCollectionUpdater<'_, T>
where
	T: Insertable<RuneIdValue, LaosCollectionValue>,
{
	pub(super) fn index_collections(
		&mut self,
		tx_index: u32,
		tx: &Transaction,
		txid: Txid,
	) -> Result<()> {
		if let Some(LaosCollection { message }) = LaosCollection::decipher(tx) {
			self.id_to_collection.insert(
				(self.height.into(), tx_index),
				(message.address_collection, message.rebaseable),
			)?;

			if let Some(sender) = self.event_sender {
				sender.blocking_send(Event::LaosCollectionCreated {
					txid,
					collection_id: RuneId { block: self.height.into(), tx: tx_index },
				})?;
			}
		}

		Ok(())
	}
}
