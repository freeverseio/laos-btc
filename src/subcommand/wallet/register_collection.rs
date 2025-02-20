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
use crate::wallet::calculate_postage;
use brc721::register_collection::RegisterCollection;
use sp_core::H160;

#[derive(Debug, Parser)]
pub(crate) struct Register {
	#[clap(long, help = "Use <FEE_RATE> sats/vbyte for register collection transaction.")]
	fee_rate: FeeRate,
	#[clap(
		long,
		help = "Register Collection <COLLECTION_ADDRESS>. 20-byte Ethereum address: 0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
	)]
	collection_address: H160,
	#[arg(
		long,
		help = "Enable the existence of unique, non-fungible tokens, the content of which can be edited by their owners at any given time [default: false]"
	)]
	rebaseable: bool,
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

impl Register {
	pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
		let destination = wallet.get_change_address()?;

		let postage = calculate_postage(self.postage, destination)?;

		let register_collection_tx =
			RegisterCollection { address: self.collection_address, rebaseable: self.rebaseable };

		let bitcoin_tx = wallet.build_tx(register_collection_tx, self.fee_rate, postage)?;

		let tx_id = wallet.bitcoin_client().send_raw_transaction(&bitcoin_tx)?;

		Ok(Some(Box::new(Output { tx_id })))
	}
}
