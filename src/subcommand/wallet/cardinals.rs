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

#[derive(Serialize, Deserialize)]
pub struct CardinalUtxo {
	pub output: OutPoint,
	pub amount: u64,
}

pub(crate) fn run(wallet: Wallet) -> SubcommandResult {
	let unspent_outputs = wallet.utxos();

	let inscribed_utxos = wallet
		.inscriptions()
		.keys()
		.map(|satpoint| satpoint.outpoint)
		.collect::<BTreeSet<OutPoint>>();

	let runic_utxos = wallet.get_runic_outputs()?.unwrap_or_default();

	let cardinal_utxos = unspent_outputs
		.iter()
		.filter_map(|(output, txout)| {
			if inscribed_utxos.contains(output) || runic_utxos.contains(output) {
				None
			} else {
				Some(CardinalUtxo { output: *output, amount: txout.value.to_sat() })
			}
		})
		.collect::<Vec<CardinalUtxo>>();

	Ok(Some(Box::new(cardinal_utxos)))
}
