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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SplitfileUnchecked {
	outputs: Vec<OutputUnchecked>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OutputUnchecked {
	address: Address<NetworkUnchecked>,
	value: Option<DeserializeFromStr<Amount>>,
	runes: BTreeMap<SpacedRune, Decimal>,
}

pub(crate) struct Splitfile {
	pub(crate) outputs: Vec<Output>,
	pub(crate) rune_info: BTreeMap<Rune, RuneInfo>,
}

pub(crate) struct Output {
	pub(crate) address: Address,
	pub(crate) value: Option<Amount>,
	pub(crate) runes: BTreeMap<Rune, u128>,
}

#[derive(Clone, Copy)]
pub(crate) struct RuneInfo {
	pub(crate) divisibility: u8,
	pub(crate) id: RuneId,
	pub(crate) spaced_rune: SpacedRune,
	pub(crate) symbol: Option<char>,
}

impl Splitfile {
	pub(crate) fn load(path: &Path, wallet: &Wallet) -> Result<Self> {
		let network = wallet.chain().network();

		let unchecked = Self::load_unchecked(path)?;

		let mut rune_info = BTreeMap::<Rune, RuneInfo>::new();

		let mut outputs = Vec::new();

		for output in unchecked.outputs {
			let mut runes = BTreeMap::new();

			for (spaced_rune, decimal) in output.runes {
				let info = if let Some(info) = rune_info.get(&spaced_rune.rune) {
					info
				} else {
					let (id, entry, _parent) =
						wallet.get_rune(spaced_rune.rune)?.with_context(|| {
							format!("rune `{}` has not been etched", spaced_rune.rune)
						})?;
					rune_info.insert(
						spaced_rune.rune,
						RuneInfo {
							divisibility: entry.divisibility,
							id,
							spaced_rune: entry.spaced_rune,
							symbol: entry.symbol,
						},
					);
					rune_info.get(&spaced_rune.rune).unwrap()
				};

				let amount = decimal.to_integer(info.divisibility)?;

				runes.insert(spaced_rune.rune, amount);
			}

			outputs.push(Output {
				address: output.address.require_network(network)?,
				value: output.value.map(|DeserializeFromStr(value)| value),
				runes,
			});
		}

		Ok(Self { outputs, rune_info })
	}

	fn load_unchecked(path: &Path) -> Result<SplitfileUnchecked> {
		Ok(serde_yaml::from_reader(File::open(path)?)?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example_split_file_is_valid() {
		Splitfile::load_unchecked("tests/fixtures/splits.yml".as_ref()).unwrap();
	}
}
