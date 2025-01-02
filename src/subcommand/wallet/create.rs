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
use bitcoin::secp256k1::rand::{self, RngCore};

#[derive(Serialize, Deserialize)]
pub struct Output {
	pub mnemonic: Mnemonic,
	pub passphrase: Option<String>,
}

#[derive(Debug, Parser)]
pub(crate) struct Create {
	#[arg(long, default_value = "", help = "Use <PASSPHRASE> to derive wallet seed.")]
	pub(crate) passphrase: String,
}

impl Create {
	pub(crate) fn run(self, name: String, settings: &Settings) -> SubcommandResult {
		let mut entropy = [0; 16];
		rand::thread_rng().fill_bytes(&mut entropy);

		let mnemonic = Mnemonic::from_entropy(&entropy)?;

		Wallet::initialize(
			name,
			settings,
			mnemonic.to_seed(&self.passphrase),
			bitcoincore_rpc::json::Timestamp::Now,
		)?;

		Ok(Some(Box::new(Output { mnemonic, passphrase: Some(self.passphrase) })))
	}
}
