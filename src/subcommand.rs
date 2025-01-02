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

pub mod balances;
pub mod decode;
pub mod env;
pub mod epochs;
pub mod find;
pub mod index;
pub mod list;
pub mod parse;
pub mod runes;
pub mod server;
mod settings;
pub mod subsidy;
pub mod supply;
pub mod teleburn;
pub mod traits;
pub mod verify;
pub mod wallet;
pub mod wallets;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
	#[command(about = "List all rune balances")]
	Balances,
	#[command(about = "Decode a transaction")]
	Decode(decode::Decode),
	#[command(about = "Start a regtest ord and bitcoind instance")]
	Env(env::Env),
	#[command(about = "List the first satoshis of each reward epoch")]
	Epochs,
	#[command(about = "Find a satoshi's current location")]
	Find(find::Find),
	#[command(subcommand, about = "Index commands")]
	Index(index::IndexSubcommand),
	#[command(about = "List the satoshis in an output")]
	List(list::List),
	#[command(about = "Parse a satoshi from ordinal notation")]
	Parse(parse::Parse),
	#[command(about = "List all runes")]
	Runes,
	#[command(about = "Run the explorer server")]
	Server(server::Server),
	#[command(about = "Display settings")]
	Settings,
	#[command(about = "Display information about a block's subsidy")]
	Subsidy(subsidy::Subsidy),
	#[command(about = "Display Bitcoin supply information")]
	Supply,
	#[command(about = "Generate teleburn addresses")]
	Teleburn(teleburn::Teleburn),
	#[command(about = "Display satoshi traits")]
	Traits(traits::Traits),
	#[command(about = "Verify BIP322 signature")]
	Verify(verify::Verify),
	#[command(about = "Wallet commands")]
	Wallet(wallet::WalletCommand),
	#[command(about = "List all Bitcoin Core wallets")]
	Wallets,
}

impl Subcommand {
	pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
		match self {
			Self::Balances => balances::run(settings),
			Self::Decode(decode) => decode.run(settings),
			Self::Env(env) => env.run(),
			Self::Epochs => epochs::run(),
			Self::Find(find) => find.run(settings),
			Self::Index(index) => index.run(settings),
			Self::List(list) => list.run(settings),
			Self::Parse(parse) => parse.run(),
			Self::Runes => runes::run(settings),
			Self::Server(server) => {
				let index = Arc::new(Index::open(&settings)?);
				let handle = axum_server::Handle::new();
				LISTENERS.lock().unwrap().push(handle.clone());
				server.run(settings, index, handle)
			},
			Self::Settings => settings::run(settings),
			Self::Subsidy(subsidy) => subsidy.run(),
			Self::Supply => supply::run(),
			Self::Teleburn(teleburn) => teleburn.run(),
			Self::Traits(traits) => traits.run(),
			Self::Verify(verify) => verify.run(),
			Self::Wallet(wallet) => wallet.run(settings),
			Self::Wallets => wallets::run(settings),
		}
	}
}

#[derive(clap::ValueEnum, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum OutputFormat {
	#[default]
	Json,
	Yaml,
	Minify,
}

pub trait Output: Send {
	fn print(&self, format: OutputFormat);
}

impl<T> Output for T
where
	T: Serialize + Send,
{
	fn print(&self, format: OutputFormat) {
		match format {
			OutputFormat::Json => serde_json::to_writer_pretty(io::stdout(), self).ok(),
			OutputFormat::Yaml => serde_yaml::to_writer(io::stdout(), self).ok(),
			OutputFormat::Minify => serde_json::to_writer(io::stdout(), self).ok(),
		};
		println!();
	}
}

pub(crate) type SubcommandResult = Result<Option<Box<dyn Output>>>;
