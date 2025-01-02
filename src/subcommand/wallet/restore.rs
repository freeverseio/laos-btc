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

#[derive(Debug, Clone)]
pub(crate) struct Timestamp(bitcoincore_rpc::json::Timestamp);

impl FromStr for Timestamp {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(if s == "now" {
			Self(bitcoincore_rpc::json::Timestamp::Now)
		} else {
			Self(bitcoincore_rpc::json::Timestamp::Time(s.parse()?))
		})
	}
}

#[derive(Debug, Parser)]
pub(crate) struct Restore {
	#[clap(value_enum, long, help = "Restore wallet from <SOURCE> on stdin.")]
	from: Source,
	#[arg(long, help = "Use <PASSPHRASE> when deriving wallet.")]
	pub(crate) passphrase: Option<String>,
	#[arg(
		long,
		help = "Scan chain from <TIMESTAMP> onwards. Can be a unix timestamp in \
    seconds or the string `now`, to skip scanning"
	)]
	pub(crate) timestamp: Option<Timestamp>,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum Source {
	Descriptor,
	Mnemonic,
}

impl Restore {
	pub(crate) fn run(self, name: String, settings: &Settings) -> SubcommandResult {
		ensure!(
			!settings
				.bitcoin_rpc_client(None)?
				.list_wallet_dir()?
				.iter()
				.any(|wallet_name| wallet_name == &name),
			"wallet `{}` already exists",
			name
		);

		let mut buffer = String::new();

		match self.from {
			Source::Descriptor => {
				io::stdin().read_to_string(&mut buffer)?;

				ensure!(self.passphrase.is_none(), "descriptor does not take a passphrase");

				ensure!(self.timestamp.is_none(), "descriptor does not take a timestamp");

				let wallet_descriptors: ListDescriptorsResult = serde_json::from_str(&buffer)?;
				Wallet::initialize_from_descriptors(
					name,
					settings,
					wallet_descriptors.descriptors,
				)?;
			},
			Source::Mnemonic => {
				io::stdin().read_line(&mut buffer)?;
				let mnemonic = Mnemonic::from_str(&buffer)?;
				Wallet::initialize(
					name,
					settings,
					mnemonic.to_seed(self.passphrase.unwrap_or_default()),
					self.timestamp
						.unwrap_or(Timestamp(bitcoincore_rpc::json::Timestamp::Time(0)))
						.0,
				)?;
			},
		}

		Ok(None)
	}
}
