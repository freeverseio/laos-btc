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
pub(crate) struct Traits {
	#[arg(help = "Show traits for <SAT>.")]
	sat: Sat,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
	pub number: u64,
	pub decimal: String,
	pub degree: String,
	pub name: String,
	pub height: u32,
	pub cycle: u32,
	pub epoch: u32,
	pub period: u32,
	pub offset: u64,
	pub rarity: Rarity,
}

impl Traits {
	pub(crate) fn run(self) -> SubcommandResult {
		Ok(Some(Box::new(Output {
			number: self.sat.n(),
			decimal: self.sat.decimal().to_string(),
			degree: self.sat.degree().to_string(),
			name: self.sat.name(),
			height: self.sat.height().0,
			cycle: self.sat.cycle(),
			epoch: self.sat.epoch().0,
			period: self.sat.period(),
			offset: self.sat.third(),
			rarity: self.sat.rarity(),
		})))
	}
}
