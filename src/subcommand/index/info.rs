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
pub(crate) struct Info {
	#[arg(long)]
	transactions: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsOutput {
	pub start: u32,
	pub end: u32,
	pub count: u32,
	pub elapsed: f64,
}

impl Info {
	pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
		let index = Index::open(&settings)?;

		let info = index.info()?;

		if self.transactions {
			let mut output = Vec::new();
			for window in info.transactions.windows(2) {
				let start = &window[0];
				let end = &window[1];
				output.push(TransactionsOutput {
					start: start.starting_block_count,
					end: end.starting_block_count,
					count: end.starting_block_count - start.starting_block_count,
					elapsed: (end.starting_timestamp - start.starting_timestamp) as f64 /
						1000.0 / 60.0,
				});
			}
			Ok(Some(Box::new(output)))
		} else {
			Ok(Some(Box::new(info)))
		}
	}
}
