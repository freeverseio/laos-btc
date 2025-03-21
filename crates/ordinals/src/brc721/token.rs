use bitcoin::OutPoint;
use sp_core::H160;

use crate::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Brc721TokenOwnership {
	InitialOwner(H160),
	NftId(Brc721Output),
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721Output {
	pub outpoint: OutPoint,
	pub nft_idx: u128,
}

impl fmt::Display for Brc721TokenOwnership {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Brc721TokenOwnership::InitialOwner(owner) => write!(f, "{:?}", owner),
			Brc721TokenOwnership::NftId(utxo_id) => write!(
				f,
				"{} - {} - {}",
				utxo_id.outpoint.txid, utxo_id.outpoint.vout, utxo_id.nft_idx
			),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn brc721_token_display_utxo_id() {
		let token = Brc721TokenOwnership::NftId(Brc721Output::default());
		assert_eq!(format!("{}", token), "0000000000000000000000000000000000000000000000000000000000000000 - 4294967295 - 0");
	}

	#[test]
	fn brc721_token_display_owner() {
		let token = Brc721TokenOwnership::InitialOwner(H160::zero());
		assert_eq!(format!("{}", token), "0x0000000000000000000000000000000000000000");
	}
}
