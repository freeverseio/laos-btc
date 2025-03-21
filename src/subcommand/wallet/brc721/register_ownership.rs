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

use crate::wallet::calculate_postage;

use super::*;
use ordinals::brc721::register_ownership::{RegisterOwnership, SlotsBundle};
use serde::{de::Error as DeError, Deserialize, Deserializer};

#[derive(Debug, Parser)]
pub(crate) struct RegisterOwnershipCmd {
	#[arg(
		long,
		help = "Register multiple slots defined in YAML <OWNERSHIP_FILE>.",
		value_name = "OWNERSHIP_FILE"
	)]
	pub(crate) file: PathBuf,
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
		let file = File::load(&self.file)?;

		let mut slots_bundles = Vec::<SlotsBundle>::new();
		let mut recipients = Vec::<Address>::new();

		let initial_owner = file.initial_owner.clone().require_network(wallet.chain().into())?;

		for output in file.outputs {
			slots_bundles.push(output.slots_bundle.clone());
			let recipient = match &output.recipient {
				Some(recipient) => recipient.clone().require_network(wallet.chain().into())?,
				None => initial_owner.clone(),
			};
			recipients.push(recipient);
		}

		let postage = calculate_postage(self.postage, wallet.get_change_address()?)?;

		let register_ownership =
			RegisterOwnership { collection_id: file.collection_id, slots_bundles };

		let bitcoin_tx = wallet.build_brc721_register_ownership_tx(
			register_ownership,
			recipients,
			initial_owner,
			self.fee_rate,
			postage,
		)?;

		let tx_id = wallet.bitcoin_client().send_raw_transaction(&bitcoin_tx)?;

		Ok(Some(Box::new(Output { tx_id })))
	}
}

#[derive(Debug, Deserialize)]
pub struct File {
	#[serde(deserialize_with = "deserialize_collection_id")]
	pub collection_id: Brc721CollectionId,
	pub outputs: Vec<SlotsOwnership>,
	#[serde(deserialize_with = "deserialize_initial_owner")]
	pub initial_owner: Address<NetworkUnchecked>,
}

#[derive(Debug, Deserialize)]
pub struct SlotsOwnership {
	#[serde(deserialize_with = "deserialize_slots_bundle")]
	slots_bundle: SlotsBundle,
	#[serde(default, deserialize_with = "deserialize_recipient")]
	recipient: Option<Address<NetworkUnchecked>>,
}

impl File {
	pub fn load(path: &Path) -> Result<Self> {
		let file: Self = serde_yaml::from_reader(fs::File::open(path)?)?;
		ensure!(
			!file.outputs.is_empty(),
			"register ownership file must contain at least one output",
		);

		// Check overlapping ranges
		for (index, output) in file.outputs.iter().enumerate() {
			let mut sorted_ranges = output.slots_bundle.clone();
			sorted_ranges.0.sort_by_key(|r| *r.start());

			if sorted_ranges.0.windows(2).any(|pair| ranges_overlap(&pair[0], &pair[1])) {
				return Err(anyhow::anyhow!(
					"overlapping ranges detected in output {}: {:?}",
					index,
					sorted_ranges
				));
			}
		}
		Ok(file)
	}
}

/// Returns true if the two ranges overlap. Two ranges overlap if they share any value.
fn ranges_overlap(
	r1: &std::ops::RangeInclusive<u128>,
	r2: &std::ops::RangeInclusive<u128>,
) -> bool {
	// They do not overlap if one finishes before the other starts.
	!(r1.end() < r2.start() || r2.end() < r1.start())
}

fn deserialize_slots_bundle<'de, D>(deserializer: D) -> Result<SlotsBundle, D::Error>
where
	D: Deserializer<'de>,
{
	let slots_bundle = Vec::<Vec<u128>>::deserialize(deserializer)?;

	if slots_bundle.is_empty() {
		return Err(D::Error::custom("slots_bundle cannot be empty"));
	}

	let mut ranges = SlotsBundle(Vec::with_capacity(slots_bundle.len()));
	for (i, range) in slots_bundle.into_iter().enumerate() {
		let range = match range.len() {
			0 => return Err(D::Error::custom(format!("range at index {} cannot be empty", i))),
			1 => {
				// Interpret [x] as x..=x
				let x = range[0];
				x..=x
			},
			2 => {
				let start = range[0];
				let end = range[1];
				if start > end {
					return Err(D::Error::custom(format!(
						"range at index {} has start {} greater than end {}",
						i, start, end
					)));
				}
				start..=end
			},
			other =>
				return Err(D::Error::custom(format!(
					"range at index {} must have 1 or 2 elements, got {}",
					i, other
				))),
		};
		ranges.0.push(range);
	}
	Ok(ranges)
}

fn deserialize_recipient<'de, D>(
	deserializer: D,
) -> Result<Option<Address<NetworkUnchecked>>, D::Error>
where
	D: Deserializer<'de>,
{
	match Option::<String>::deserialize(deserializer)? {
		Some(address) =>
			address.parse::<Address<NetworkUnchecked>>().map(Some).map_err(D::Error::custom),
		None => Ok(None),
	}
}

fn deserialize_initial_owner<'de, D>(deserializer: D) -> Result<Address<NetworkUnchecked>, D::Error>
where
	D: Deserializer<'de>,
{
	String::deserialize(deserializer)?
		.parse::<Address<NetworkUnchecked>>()
		.map_err(D::Error::custom)
}

fn deserialize_collection_id<'de, D>(deserializer: D) -> Result<Brc721CollectionId, D::Error>
where
	D: Deserializer<'de>,
{
	let collection_id = String::deserialize(deserializer)?;
	Brc721CollectionId::from_str(&collection_id).map_err(D::Error::custom)
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
collection_id: 1:1
initial_owner: 1BitcoinEaterAddressDontSendf59kuE
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
	fn load_file_wrong_initial_owner() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
initial_owner: a
outputs:
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"base58 error at line 2 column 1"
		);
	}

	#[test]
	fn load_file_wrong_slot_range_three_elements() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
outputs:
  - slots_bundle: [[0,0,0]]
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"outputs[0]: range at index 0 must have 1 or 2 elements, got 3 at line 4 column 5"
		);
	}

	#[test]
	fn load_file_wrong_slot_range_start_greater() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
outputs:
  - slots_bundle: [[1,0]]
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"outputs[0]: range at index 0 has start 1 greater than end 0 at line 4 column 5"
		);
	}

	#[test]
	fn load_file_overlapping_slots() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
initial_owner: 1BitcoinEaterAddressDontSendf59kuE
outputs:
  - slots_bundle: [[0,20],[21],[20]]
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"overlapping ranges detected in output 0: SlotsBundle([0..=20, 20..=20, 21..=21])"
		);
	}

	#[test]
	fn load_file_slot_bundle_one_element_no_recipient() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
initial_owner: mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m
outputs:
  - slots_bundle: [[0]]
"#,
		)
		.unwrap();

		let file = File::load(batch_file.as_path()).unwrap();
		assert_eq!(file.outputs.len(), 1);
		assert_eq!(file.outputs[0].slots_bundle.0.len(), 1);
		assert_eq!(
			file.initial_owner.clone().assume_checked().to_string(),
			"mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m"
		);
		assert_eq!(file.collection_id, Brc721CollectionId::new(1, 1).unwrap());
		let range = &file.outputs[0].slots_bundle.0[0];
		// For a one-element range, start == end.
		assert_eq!(range.start(), range.end());
		// And the only element is 0.
		assert_eq!(*range.start(), 0);
	}

	#[test]
	fn load_file_slot_bundle_one_element_with_recipient_wrong_address() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
outputs:
  - slots_bundle: [[0]]
    recipient: asd
"#,
		)
		.unwrap();

		assert_eq!(
			File::load(batch_file.as_path()).unwrap_err().to_string(),
			"outputs[0]: base58 error at line 4 column 5"
		);
	}

	#[test]
	fn load_file_slot_bundle_one_element_with_recipient() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
initial_owner: mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m
outputs:
  - slots_bundle: [[0]]
    recipient: 1BitcoinEaterAddressDontSendf59kuE
"#,
		)
		.unwrap();

		let file = File::load(batch_file.as_path()).unwrap();
		assert_eq!(file.outputs.len(), 1);
		assert_eq!(file.outputs[0].slots_bundle.0.len(), 1);
		let range = &file.outputs[0].slots_bundle.0[0];
		// For a one-element range, start == end.
		assert_eq!(range.start(), range.end());
		// And the only element is 0.
		assert_eq!(*range.start(), 0);
		assert_eq!(
			file.outputs[0].recipient.as_ref().unwrap().clone().assume_checked().to_string(),
			"1BitcoinEaterAddressDontSendf59kuE"
		);
	}

	#[test]
	fn load_file_multiple_slots_bundles_with_owner() {
		let tempdir = TempDir::new().unwrap();
		let batch_file = tempdir.path().join("temp.yaml");
		fs::write(
			batch_file.clone(),
			r#"
collection_id: 1:1
initial_owner: 1BitcoinEaterAddressDontSendf59kuE
outputs:
  - slots_bundle: [[0]]
    recipient: 1BitcoinEaterAddressDontSendf59kuE
  - slots_bundle: [[0],[2], [4,6]]
    recipient: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
"#,
		)
		.unwrap();

		let file = File::load(batch_file.as_path()).unwrap();
		assert_eq!(file.outputs.len(), 2);
		assert_eq!(
			file.outputs[0].recipient.as_ref().unwrap().clone().assume_checked().to_string(),
			"1BitcoinEaterAddressDontSendf59kuE"
		);
		// OUTPUT 0
		assert_eq!(file.outputs[0].slots_bundle.0.len(), 1);
		let bundle0 = &file.outputs[0].slots_bundle.0[0];
		// For a single-element slot, start should equal end and be 0.
		assert_eq!(*bundle0.start(), 0);
		assert_eq!(*bundle0.end(), 0);
		assert_eq!(
			file.outputs[1].recipient.as_ref().unwrap().clone().assume_checked().to_string(),
			"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
		);
		// OUTPUT 1
		assert_eq!(file.outputs[1].slots_bundle.0.len(), 3);
		let bundle0 = &file.outputs[1].slots_bundle.0[0];
		assert_eq!(*bundle0.start(), 0);
		assert_eq!(*bundle0.end(), 0);
		let bundle1 = &file.outputs[1].slots_bundle.0[1];
		assert_eq!(*bundle1.start(), 2);
		assert_eq!(*bundle1.end(), 2);
		let bundle3 = &file.outputs[1].slots_bundle.0[2];
		assert_eq!(*bundle3.start(), 4);
		assert_eq!(*bundle3.end(), 6);
	}
}
