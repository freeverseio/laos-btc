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

#[derive(Serialize, Eq, PartialEq, Deserialize, Debug)]
pub enum Artifact {
	Cenotaph(Cenotaph),
	Runestone(Runestone),
}

impl Artifact {
	pub fn mint(&self) -> Option<RuneId> {
		match self {
			Self::Cenotaph(cenotaph) => cenotaph.mint,
			Self::Runestone(runestone) => runestone.mint,
		}
	}
}
