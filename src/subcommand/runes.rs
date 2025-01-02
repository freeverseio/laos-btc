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
	pub runes: BTreeMap<Rune, RuneInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RuneInfo {
	pub block: u64,
	pub burned: u128,
	pub divisibility: u8,
	pub etching: Txid,
	pub id: RuneId,
	pub mints: u128,
	pub number: u64,
	pub premine: u128,
	pub rune: SpacedRune,
	pub supply: u128,
	pub symbol: Option<char>,
	pub terms: Option<Terms>,
	pub timestamp: DateTime<Utc>,
	pub turbo: bool,
	pub tx: u32,
}

pub(crate) fn run(settings: Settings) -> SubcommandResult {
	let index = Index::open(&settings)?;

	ensure!(index.has_rune_index(), "`ord runes` requires index created with `--index-runes` flag",);

	index.update()?;

	Ok(Some(Box::new(Output {
		runes: index
			.runes()?
			.into_iter()
			.map(
				|(
					id,
					entry @ RuneEntry {
						block,
						burned,
						divisibility,
						etching,
						mints,
						number,
						premine,
						spaced_rune,
						symbol,
						terms,
						timestamp,
						turbo,
					},
				)| {
					(
						spaced_rune.rune,
						RuneInfo {
							block,
							burned,
							divisibility,
							etching,
							id,
							mints,
							number,
							premine,
							rune: spaced_rune,
							supply: entry.supply(),
							symbol,
							terms,
							timestamp: crate::timestamp(timestamp),
							turbo,
							tx: id.tx,
						},
					)
				},
			)
			.collect::<BTreeMap<Rune, RuneInfo>>(),
	})))
}
