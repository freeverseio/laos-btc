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

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub(super) struct Message {
	pub(super) address_collection: [u8; COLLECTION_ADDRESS_LENGTH],
	pub(super) rebaseable: bool,
}

impl Message {
	pub(super) fn from_payload(payload: Payload) -> Self {
		Self {
			address_collection: payload[..COLLECTION_ADDRESS_LENGTH].try_into().expect("The length is correct; qed;"),
			rebaseable: payload[COLLECTION_ADDRESS_LENGTH] > 0,
		}
	}
}
