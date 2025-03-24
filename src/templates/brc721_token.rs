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

#[derive(Boilerplate, Clone, Copy, Debug, PartialEq)]
pub struct Brc721TokenHtml {
	pub entry: Brc721TokenOwnership,
}

impl PageContent for Brc721TokenHtml {
	fn title(&self) -> String {
		"Brc721Token".to_string()
	}
}

#[cfg(test)]
mod tests {
	use ordinals::brc721::token::Brc721Output;
	use sp_core::H160;

	use super::*;

	#[test]
	fn display_utxo_id() {
		assert_eq!(
			Brc721TokenHtml {
				entry: Brc721TokenOwnership::NftId(Brc721Output {
					outpoint: OutPoint {
						txid: Txid::from_str(
							"c8cdf720db5562a039be5d81c51a07c5120eaf0bf142b2144f1a1eb7a95678d3"
						)
						.unwrap(),
						vout: 0
					},
					nft_idx: 0
				})
			}
			.to_string(),
			"<h1>Token Owner</h1>
<p>c8cdf720db5562a039be5d81c51a07c5120eaf0bf142b2144f1a1eb7a95678d3:0:0</p>"
		);
	}

	#[test]
	fn display_owner() {
		assert_eq!(
			Brc721TokenHtml { entry: Brc721TokenOwnership::InitialOwner(H160::zero()) }.to_string(),
			"<h1>Token Owner</h1>
<p>0x0000000000000000000000000000000000000000</p>"
		);
	}
}
