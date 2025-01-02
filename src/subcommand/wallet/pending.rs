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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct PendingOutput {
	pub commit: Txid,
	pub rune: SpacedRune,
}
#[derive(Debug, Parser)]
pub(crate) struct Pending {}

impl Pending {
	pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
		let etchings = wallet
			.pending_etchings()?
			.into_iter()
			.map(|(_, entry)| {
				let spaced_rune = entry.output.rune.unwrap().rune;

				PendingOutput { rune: spaced_rune, commit: entry.commit.compute_txid() }
			})
			.collect::<Vec<PendingOutput>>();

		Ok(Some(Box::new(etchings) as Box<dyn Output>))
	}
}
