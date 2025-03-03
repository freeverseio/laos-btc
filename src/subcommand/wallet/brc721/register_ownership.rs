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

use std::ops::Add;

use super::*;
use crate::wallet::calculate_postage;
// use anyhow::Ok;
use ordinals::brc721::register_ownership::{Ranges, RegisterOwnership};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer};
use sp_core::H160;
#[derive(Debug, Parser)]
pub(crate) struct RegisterOwnershipCmd {
	#[arg(
		long,
		help = "Register multiple slots defined in YAML <OWNERSHIP_FILE>.",
		value_name = "OWNERSHIP_FILE"
	)]
	pub(crate) batch: PathBuf,
	#[clap(long, help = "Use <FEE_RATE> sats/vbyte for register collection transaction.")]
	fee_rate: FeeRate,
	#[clap(
		long,
		help = "Include <AMOUNT> postage with register collection output. [default: 10000sat]"
	)]
	postage: Option<Amount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
	pub tx_id: Txid,
}

impl RegisterOwnershipCmd {
	pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
		// let destination = wallet.get_change_address()?;

		// let postage = calculate_postage(self.postage, destination)?;

		// let register_collection =
		// 	RegisterOwnership { address: self.address, rebaseable: self.rebaseable };

		// let bitcoin_tx =
		// 	wallet.build_brc721_tx(register_collection.to_script(), self.fee_rate, postage)?;

		// let tx_id = wallet.bitcoin_client().send_raw_transaction(&bitcoin_tx)?;

		// Ok(Some(Box::new(Output { tx_id })))
		Ok(None)
	}
}
#[derive(Debug, Deserialize)]
pub struct File {
	pub outputs: Vec<SlotsOwnership>,
}

#[derive(Debug, Deserialize)]
pub struct SlotsOwnership {
	#[serde(deserialize_with = "deserialize_slots_bundle")]
	slots_bundle: Ranges,
	#[serde(default, deserialize_with = "deserialize_owner")]
	owner: Option<Address<NetworkUnchecked>>,
}

impl File {
	pub fn load(path: &Path) -> Result<Self> {
		let file: Self = serde_yaml::from_reader(fs::File::open(path)?)?;
		ensure!(
			!file.outputs.is_empty(),
			"register ownership file must contain at least one output",
		);
		Ok(file)
	}
}

fn deserialize_slots_bundle<'de, D>(deserializer: D) -> Result<Ranges, D::Error>
where
	D: Deserializer<'de>,
{
	let slots_bundle = Ranges::deserialize(deserializer)?;

	if slots_bundle.is_empty() {
		return Err(D::Error::custom("slots_bundle cannot be empty"));
	}

	// Ensure each slot has exactly 2 elements.
    for (i, range) in slots_bundle.iter().enumerate() {
        if range.is_empty() {
            return Err(D::Error::custom(format!("range at index {} cannot be empty", i)));
        }
        if range.len() > 2 {
            return Err(D::Error::custom(format!(
                "range at index {} must have 1 or 2 elements, got {}",
                i,
                range.len()
            )));
        }
    }

	Ok(slots_bundle)
}

fn deserialize_owner<'de, D>(deserializer: D) -> Result<Option<Address<NetworkUnchecked>>, D::Error>
where
	D: Deserializer<'de>,
{
	match Option::<String>::deserialize(deserializer)? {
		Some(s) => {
			// Attempt to parse the string as a Bitcoin address.
			s.parse::<Address<NetworkUnchecked>>().map(Some).map_err(D::Error::custom)
		},
		None => Ok(None),
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn load_file_no_outputs() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
outputs:
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"register ownership file must contain at least one output"
		);
	}

	#[test]
	fn load_file_wrong_slot_range() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
outputs:
  - slots_bundle: [[0,0,0]]
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"outputs[0]: range at index 0 must have 1 or 2 elements, got 3 at line 3 column 5"
		);
	}

	#[test]
	fn load_file_slot_bundle_one_element_no_owner() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
outputs:
  - slots_bundle: [[0]]
"#,
		)
		.unwrap();

		let file = File::load(batch_file.as_path()).unwrap();
		assert_eq!(file.outputs.len(), 1);
		assert_eq!(file.outputs[0].slots_bundle.len(), 1);
		assert_eq!(file.outputs[0].slots_bundle[0].len(), 1);
		assert_eq!(file.outputs[0].slots_bundle[0][0], 0);
	}

	#[test]
	fn load_file_slot_bundle_one_element_with_owner_wrong_address() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
outputs:
  - slots_bundle: [[0]]
    owner: asd
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"outputs[0]: base58 error at line 3 column 5"
		);
	}

	#[test]
	fn load_file_slot_bundle_one_element_with_owner() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
outputs:
  - slots_bundle: [[0]]
    owner: 1BitcoinEaterAddressDontSendf59kuE
"#,
		)
		.unwrap();

		let file = File::load(batch_file.as_path()).unwrap();
		assert_eq!(file.outputs.len(), 1);
		assert_eq!(file.outputs[0].slots_bundle.len(), 1);
		assert_eq!(file.outputs[0].slots_bundle[0].len(), 1);
		assert_eq!(file.outputs[0].slots_bundle[0][0], 0);
		assert_eq!(file.outputs[0].owner.as_ref().unwrap().clone().assume_checked().to_string(), "1BitcoinEaterAddressDontSendf59kuE");
	}

	#[test]
	fn load_file_multiple_slots_bundles_with_owner() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
outputs:
  - slots_bundle: [[0]]
    owner: 1BitcoinEaterAddressDontSendf59kuE
  - slots_bundle: [[0],[2], [4,6]]
    owner: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
"#,
		)
		.unwrap();

		let file = File::load(batch_file.as_path()).unwrap();
		assert_eq!(file.outputs.len(), 2);
		assert_eq!(file.outputs[0].owner.as_ref().unwrap().clone().assume_checked().to_string(), "1BitcoinEaterAddressDontSendf59kuE");
		assert_eq!(file.outputs[0].slots_bundle.len(), 1);
		assert_eq!(file.outputs[0].slots_bundle[0].len(), 1);
		assert_eq!(file.outputs[0].slots_bundle[0][0], 0);
		assert_eq!(file.outputs[1].owner.as_ref().unwrap().clone().assume_checked().to_string(), "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
		assert_eq!(file.outputs[1].slots_bundle.len(), 3);
		assert_eq!(file.outputs[1].slots_bundle[0].len(), 1);
		assert_eq!(file.outputs[1].slots_bundle[0][0], 0);
		assert_eq!(file.outputs[1].slots_bundle[1].len(), 1);
		assert_eq!(file.outputs[1].slots_bundle[1][0], 2);
		assert_eq!(file.outputs[1].slots_bundle[2].len(), 2);
		assert_eq!(file.outputs[1].slots_bundle[2][0], 4);
		assert_eq!(file.outputs[1].slots_bundle[2][1], 6);
	}
}
