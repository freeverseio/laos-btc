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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
	pub supply: u64,
	pub first: u64,
	pub last: u64,
	pub last_mined_in_block: u32,
}

pub(crate) fn run() -> SubcommandResult {
	let mut last = 0;

	loop {
		if Height(last + 1).subsidy() == 0 {
			break;
		}
		last += 1;
	}

	Ok(Some(Box::new(Output {
		supply: Sat::SUPPLY,
		first: 0,
		last: Sat::SUPPLY - 1,
		last_mined_in_block: last,
	})))
}
