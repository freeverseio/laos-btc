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

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721TokenHtml {
	pub entry: Brc721Token,
}

impl PageContent for Brc721TokenHtml {
	fn title(&self) -> String {
		"Brc721Token".to_string()
	}
}

#[cfg(test)]
mod tests {
	use ordinals::brc721::token::UtxoId;
	use sp_core::H160;

	use super::*;

	#[test]
	fn display_utxo_id() {
		assert_eq!(
			Brc721TokenHtml {
				entry: Brc721Token::new(
					None,
					Some(UtxoId { tx_idx: 0, tx_out_idx: 0, utxo_idx: 0 })
				)
			}
			.to_string(),
			"<h1>Token</h1>
<p>0 - 0 - 0</p>"
		);
	}

	#[test]
	fn display_owner() {
		assert_eq!(
			Brc721TokenHtml { entry: Brc721Token::new(Some(H160::zero()), None,) }.to_string(),
			"<h1>Token</h1>
<p>0x0000000000000000000000000000000000000000</p>"
		);
	}

	#[test]
	fn display_none() {
		assert_eq!(
			Brc721TokenHtml { entry: Brc721Token::new(None, None,) }.to_string(),
			"<h1>Token</h1>
<p>unexisting collection</p>"
		);
	}
}
