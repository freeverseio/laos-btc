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

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Output {
	pub output: OutPoint,
	pub amount: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub inscriptions: Option<Vec<InscriptionId>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub runes: Option<BTreeMap<SpacedRune, Decimal>>,
}

pub(crate) fn run(wallet: Wallet) -> SubcommandResult {
	let mut addresses: BTreeMap<Address<NetworkUnchecked>, Vec<Output>> = BTreeMap::new();

	for (output, txout) in wallet.utxos() {
		let address = wallet.chain().address_from_script(&txout.script_pubkey)?;

		let inscriptions = wallet.get_inscriptions_in_output(output);

		let runes = wallet.get_runes_balances_in_output(output)?.map(|balances| {
			balances
				.iter()
				.map(|(rune, pile)| {
					(*rune, Decimal { value: pile.amount, scale: pile.divisibility })
				})
				.collect()
		});

		let output = Output { output: *output, amount: txout.value.to_sat(), inscriptions, runes };

		addresses.entry(address.as_unchecked().clone()).or_default().push(output);
	}

	Ok(Some(Box::new(addresses)))
}
