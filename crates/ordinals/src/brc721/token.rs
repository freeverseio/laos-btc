use sp_core::H160;

use crate::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Brc721Token {
	pub owner: Option<H160>,
	pub utxo_id: Option<UtxoId>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UtxoId {
	pub tx_idx: u32,
	pub tx_out_idx: u128,
	pub utxo_idx: u128,
}

impl Brc721Token {
	pub fn new(owner: Option<H160>, utxo_id: Option<UtxoId>) -> Self {
		Brc721Token { owner, utxo_id }
	}
}

impl fmt::Display for Brc721Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(owner) = &self.owner {
			write!(f, "{:?}", owner)
		} else if let Some(utxo_id) = &self.utxo_id {
			write!(f, "{} - {} - {}", utxo_id.tx_idx, utxo_id.tx_out_idx, utxo_id.utxo_idx)
		} else {
			write!(f, "")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn brc721_token_display_none() {
		let token = Brc721Token::new(None, None);

		assert_eq!(format!("{}", token), "");
	}

	#[test]
	fn brc721_token_display_utxo_id() {
		let token = Brc721Token::new(None, Some(UtxoId::default()));

		assert_eq!(format!("{}", token), "0 - 0 - 0");
	}

	#[test]
	fn brc721_token_display_owner() {
		let token = Brc721Token::new(Some(H160::zero()), None);

		assert_eq!(format!("{}", token), "0x0000000000000000000000000000000000000000");
	}
}
