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

pub(super) struct Message {
	pub(super) flaw: Option<Flaw>,
	pub(super) edicts: Vec<Edict>,
	pub(super) fields: HashMap<u128, VecDeque<u128>>,
}

impl Message {
	pub(super) fn from_integers(tx: &Transaction, payload: &[u128]) -> Self {
		let mut edicts = Vec::new();
		let mut fields = HashMap::<u128, VecDeque<u128>>::new();
		let mut flaw = None;

		for i in (0..payload.len()).step_by(2) {
			let tag = payload[i];

			if Tag::Body == tag {
				let mut id = RuneId::default();
				for chunk in payload[i + 1..].chunks(4) {
					if chunk.len() != 4 {
						flaw.get_or_insert(Flaw::TrailingIntegers);
						break;
					}

					let Some(next) = id.next(chunk[0], chunk[1]) else {
						flaw.get_or_insert(Flaw::EdictRuneId);
						break;
					};

					let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
						flaw.get_or_insert(Flaw::EdictOutput);
						break;
					};

					id = next;
					edicts.push(edict);
				}
				break;
			}

			let Some(&value) = payload.get(i + 1) else {
				flaw.get_or_insert(Flaw::TruncatedField);
				break;
			};

			fields.entry(tag).or_default().push_back(value);
		}

		Self { flaw, edicts, fields }
	}
}
