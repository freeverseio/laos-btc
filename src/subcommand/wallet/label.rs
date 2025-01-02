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

#[derive(Serialize)]
struct Label {
	first_sat: SatLabel,
	inscriptions: BTreeMap<u64, BTreeSet<InscriptionId>>,
}

#[derive(Serialize)]
struct SatLabel {
	name: String,
	number: u64,
	rarity: Rarity,
}

#[derive(Serialize)]
struct Line {
	label: String,
	r#ref: String,
	r#type: String,
}

pub(crate) fn run(wallet: Wallet) -> SubcommandResult {
	let mut lines: Vec<Line> = Vec::new();

	let sat_ranges = wallet.get_wallet_sat_ranges()?;

	let mut inscriptions_by_output: BTreeMap<OutPoint, BTreeMap<u64, Vec<InscriptionId>>> =
		BTreeMap::new();

	for (satpoint, inscriptions) in wallet.inscriptions() {
		inscriptions_by_output
			.entry(satpoint.outpoint)
			.or_default()
			.insert(satpoint.offset, inscriptions.clone());
	}

	for (output, ranges) in sat_ranges {
		let sat = Sat(ranges[0].0);
		let mut inscriptions = BTreeMap::<u64, BTreeSet<InscriptionId>>::new();

		if let Some(output_inscriptions) = inscriptions_by_output.get(&output) {
			for (&offset, offset_inscriptions) in output_inscriptions {
				inscriptions.entry(offset).or_default().extend(offset_inscriptions);
			}
		}

		lines.push(Line {
			label: serde_json::to_string(&Label {
				first_sat: SatLabel { name: sat.name(), number: sat.n(), rarity: sat.rarity() },
				inscriptions,
			})?,
			r#ref: output.to_string(),
			r#type: "output".into(),
		});
	}

	for line in lines {
		serde_json::to_writer(io::stdout(), &line)?;
		println!();
	}

	Ok(None)
}
