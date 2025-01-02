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

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Copy, Clone, Eq)]
pub struct Edict {
	pub id: RuneId,
	pub amount: u128,
	pub output: u32,
}

impl Edict {
	pub fn from_integers(tx: &Transaction, id: RuneId, amount: u128, output: u128) -> Option<Self> {
		let Ok(output) = u32::try_from(output) else {
			return None;
		};

		// note that this allows `output == tx.output.len()`, which means to divide
		// amount between all non-OP_RETURN outputs
		if output > u32::try_from(tx.output.len()).unwrap() {
			return None;
		}

		Some(Self { id, amount, output })
	}
}
