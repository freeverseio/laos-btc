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
pub struct BlocksHtml {
	pub last: u32,
	pub blocks: Vec<BlockHash>,
	pub featured_blocks: BTreeMap<BlockHash, Vec<InscriptionId>>,
}

impl BlocksHtml {
	pub(crate) fn new(
		blocks: Vec<(u32, BlockHash)>,
		featured_blocks: BTreeMap<BlockHash, Vec<InscriptionId>>,
	) -> Self {
		Self {
			last: blocks.first().map(|(height, _)| height).cloned().unwrap_or(0),
			blocks: blocks.into_iter().map(|(_, hash)| hash).collect(),
			featured_blocks,
		}
	}
}

impl PageContent for BlocksHtml {
	fn title(&self) -> String {
		"Blocks".to_string()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn html() {
		let mut feature_blocks = BTreeMap::new();
		feature_blocks.insert(
			"2222222222222222222222222222222222222222222222222222222222222222"
				.parse()
				.unwrap(),
			vec![inscription_id(1), inscription_id(2)],
		);

		assert_regex_match!(
			&BlocksHtml::new(
				vec![
					(
						1260002,
						"2222222222222222222222222222222222222222222222222222222222222222"
							.parse()
							.unwrap()
					),
					(
						1260001,
						"1111111111111111111111111111111111111111111111111111111111111111"
							.parse()
							.unwrap()
					),
					(
						1260000,
						"0000000000000000000000000000000000000000000000000000000000000000"
							.parse()
							.unwrap()
					)
				],
				feature_blocks,
			)
			.to_string()
			.unindent(),
			"<h1>Blocks</h1>
      <div class=block>
        <h2><a href=/block/1260002>Block 1260002</a></h2>
        <div class=thumbnails>
          <a href=/inscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          <a href=/inscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
        </div>
      </div>
      <ol start=1260001 reversed class=block-list>
        <li><a class=collapse href=/block/1{64}>1{64}</a></li>
        <li><a class=collapse href=/block/0{64}>0{64}</a></li>
      </ol>
      "
			.unindent(),
		);
	}
}
