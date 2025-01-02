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

#[derive(Deserialize, Serialize)]
pub struct Output {
	pub addresses: Vec<Address<NetworkUnchecked>>,
}

#[derive(Debug, Parser)]
pub(crate) struct Receive {
	#[arg(short, long, help = "Generate <NUMBER> addresses.")]
	number: Option<u64>,
}

impl Receive {
	pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
		let mut addresses: Vec<Address<NetworkUnchecked>> = Vec::new();

		for _ in 0..self.number.unwrap_or(1) {
			addresses.push(
				wallet
					.bitcoin_client()
					.get_new_address(None, Some(bitcoincore_rpc::json::AddressType::Bech32m))?,
			);
		}

		Ok(Some(Box::new(Output { addresses })))
	}
}
