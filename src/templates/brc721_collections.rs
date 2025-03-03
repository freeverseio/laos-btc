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

type DisplayableAddress = String;

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub struct Brc721CollectionsHtml {
	pub entries: Vec<Brc721Collection>,
	pub more: bool,
	pub prev: Option<usize>,
	pub next: Option<usize>,
}

impl PageContent for Brc721CollectionsHtml {
	fn title(&self) -> String {
		"Brc721Collections".to_string()
	}
}

#[cfg(test)]
mod tests {
	use sp_core::H160;

	use super::*;

	#[test]
	fn display() {
		assert_eq!(
			Brc721CollectionsHtml {
				entries: vec![
					(Brc721Collection(Brc721CollectionId::default(), H160::default(), false))
				],
				more: false,
				prev: None,
				next: None,
			}
			.to_string(),
			"<h1>Brc721 Collections</h1>
<ul>
  <li>0:0 - 0x0000000000000000000000000000000000000000</li>
</ul>
<div class=center>
    prev
      next
  </div>"
		);
	}

	#[test]
	fn with_prev_and_next() {
		assert_eq!(
			Brc721CollectionsHtml {
				entries: vec![
					Brc721Collection(
						Brc721CollectionId::default(),
						H160::from_str("0x0000000000000000000000000000000000000000").unwrap(),
						false,
					),
					Brc721Collection(
						Brc721CollectionId { block: 1, tx: 1 },
						H160::from_low_u64_be(1),
						false
					)
				],
				prev: Some(1),
				next: Some(2),
				more: true,
			}
			.to_string(),
			"<h1>Brc721 Collections</h1>
<ul>
  <li>0:0 - 0x0000000000000000000000000000000000000000</li>
  <li>1:1 - 0x0000000000000000000000000000000000000001</li>
</ul>
<div class=center>
    <a class=prev href=/brc721/collections/1>prev</a>
      <a class=next href=/brc721/collections/2>next</a>
  </div>"
		);
	}
}
