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

use crate::wallet::{Postage, Wallet};

use super::*;
use bitcoincore_rpc::json::ImportDescriptors;
use fee_rate::FeeRate;
use redb::Database;

pub(crate) struct WalletBrc721<'a> {
	inner: &'a Wallet,
}

impl<'a> WalletBrc721<'a> {
	pub(crate) fn bitcoin_client(&self) -> &Client {
		&self.inner.bitcoin_client()
	}

	pub(crate) fn lock_non_cardinal_outputs(&self) -> Result {
		self.inner.lock_non_cardinal_outputs()
	}

	pub(crate) fn get_change_address(&self) -> Result<Address> {
		Ok(self
			.inner
			.bitcoin_client()
			.call::<Address<NetworkUnchecked>>("getrawchangeaddress", &["bech32".into()])
			.context("could not get change addresses from wallet")?
			.require_network(self.chain().network())?)
	}

	pub(crate) fn has_brc721_index(&self) -> bool {
		self.inner.has_brc721_index()
	}

	pub(crate) fn chain(&self) -> Chain {
		self.inner.settings().chain()
	}

	pub(crate) fn check_descriptors(
		wallet_name: &str,
		descriptors: Vec<crate::wallet::Descriptor>,
	) -> Result<Vec<crate::wallet::Descriptor>> {
		let wpkh = descriptors
			.iter()
			.filter(|descriptor| descriptor.desc.starts_with("wpkh("))
			.count();

		if wpkh != 2 || descriptors.len() != 2 {
			bail!("wallet \"{}\" contains unexpected output descriptors, and does not appear to be an `ord` brc721 wallet, create a new wallet with `ord wallet --brc721 restore`", wallet_name);
		}

		Ok(descriptors)
	}

	pub(crate) fn initialize_from_descriptors(
		name: String,
		settings: &Settings,
		descriptors: Vec<crate::wallet::Descriptor>,
	) -> Result {
		let client = Self::check_version(settings.bitcoin_rpc_client(Some(name.clone()))?)?;

		let descriptors = Self::check_descriptors(&name, descriptors)?;

		client.create_wallet(&name, None, Some(true), None, None)?;

		let descriptors = descriptors
			.into_iter()
			.map(|descriptor| ImportDescriptors {
				descriptor: descriptor.desc.clone(),
				timestamp: descriptor.timestamp,
				active: Some(true),
				range: descriptor.range.map(|(start, end)| {
					(usize::try_from(start).unwrap_or(0), usize::try_from(end).unwrap_or(0))
				}),
				next_index: descriptor.next.map(|next| usize::try_from(next).unwrap_or(0)),
				internal: descriptor.internal,
				label: None,
			})
			.collect::<Vec<ImportDescriptors>>();

		client.call::<serde_json::Value>(
			"importdescriptors",
			&[serde_json::to_value(descriptors)?],
		)?;

		Ok(())
	}

	pub(crate) fn check_version(client: Client) -> Result<Client> {
		wallet::Wallet::check_version(client)
	}

	pub(crate) fn open_database(wallet_name: &String, settings: &Settings) -> Result<Database> {
		wallet::Wallet::open_database(wallet_name, settings)
	}

	pub(crate) fn build_brc721_tx<T: Into<ScriptBuf>>(
		&self,
		tx: T,
		fee_rate: FeeRate,
		postage: Postage,
	) -> Result<Transaction> {
		ensure!(
			self.has_brc721_index(),
			"creating brc721 collections with `laos-btc wallet brc721 rc` requires index created with `--index-brc721` flag",
		);

		self.lock_non_cardinal_outputs()?;

		let unfunded_tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![
				TxOut { value: Amount::from_sat(0), script_pubkey: tx.into() },
				TxOut { value: postage.amount, script_pubkey: postage.destination.script_pubkey() },
			],
		};

		let unsigned_transaction =
			fund_raw_transaction(self.inner.bitcoin_client(), fee_rate, &unfunded_tx)?;

		let signed_transaction = self
			.inner
			.bitcoin_client()
			.sign_raw_transaction_with_wallet(&unsigned_transaction, None, None)?
			.hex;
		let signed_transaction = consensus::encode::deserialize(&signed_transaction)?;

		Ok(signed_transaction)
	}

	pub(crate) fn from_wallet(wallet: &'a Wallet) -> Self {
		Self { inner: wallet }
	}
}
