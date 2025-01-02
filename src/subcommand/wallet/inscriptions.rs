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
pub struct Output {
	pub inscription: InscriptionId,
	pub location: SatPoint,
	pub explorer: String,
	pub postage: u64,
}

pub(crate) fn run(wallet: Wallet) -> SubcommandResult {
	let explorer = match wallet.chain() {
		Chain::Mainnet => "https://ordinals.com/inscription/",
		Chain::Regtest => "http://localhost/inscription/",
		Chain::Signet => "https://signet.ordinals.com/inscription/",
		Chain::Testnet => "https://testnet.ordinals.com/inscription/",
		Chain::Testnet4 => "https://testnet4.ordinals.com/inscription/",
	};

	let mut output = Vec::new();

	for (location, inscriptions) in wallet.inscriptions() {
		if let Some(txout) = wallet.utxos().get(&location.outpoint) {
			for inscription in inscriptions {
				output.push(Output {
					location: *location,
					inscription: *inscription,
					explorer: format!("{explorer}{inscription}"),
					postage: txout.value.to_sat(),
				})
			}
		}
	}

	Ok(Some(Box::new(output)))
}
