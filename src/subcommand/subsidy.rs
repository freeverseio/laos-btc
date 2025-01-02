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

#[derive(Debug, Parser)]
pub(crate) struct Subsidy {
	#[arg(help = "List sats in subsidy at <HEIGHT>.")]
	height: Height,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
	pub first: u64,
	pub subsidy: u64,
	pub name: String,
}

impl Subsidy {
	pub(crate) fn run(self) -> SubcommandResult {
		let first = self.height.starting_sat();

		let subsidy = self.height.subsidy();

		if subsidy == 0 {
			bail!("block {} has no subsidy", self.height);
		}

		Ok(Some(Box::new(Output { first: first.0, subsidy, name: first.name() })))
	}
}
