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
pub(crate) struct Find {
	#[arg(help = "Find output and offset of <SAT>.")]
	sat: Sat,
	#[clap(help = "Find output and offset of all sats in the range [<SAT>, <END>).")]
	end: Option<Sat>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Output {
	pub satpoint: SatPoint,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FindRangeOutput {
	pub start: u64,
	pub size: u64,
	pub satpoint: SatPoint,
}

impl Find {
	pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
		let index = Index::open(&settings)?;

		if !index.has_sat_index() {
			bail!("find requires index created with `--index-sats` flag");
		}

		index.update()?;

		match self.end {
			Some(end) => match index.find_range(self.sat, end)? {
				Some(mut results) => {
					results.sort_by_key(|find_range_output| find_range_output.start);
					Ok(Some(Box::new(results)))
				},
				None => Err(anyhow!("range has not been mined as of index height")),
			},
			None => match index.find(self.sat)? {
				Some(satpoint) => Ok(Some(Box::new(Output { satpoint }))),
				None => Err(anyhow!("sat has not been mined as of index height")),
			},
		}
	}
}
