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

#[derive(Boilerplate)]
pub(crate) struct OutputHtml {
	pub(crate) chain: Chain,
	pub(crate) inscriptions: Option<Vec<InscriptionId>>,
	pub(crate) outpoint: OutPoint,
	pub(crate) output: TxOut,
	pub(crate) runes: Option<BTreeMap<SpacedRune, Pile>>,
	pub(crate) sat_ranges: Option<Vec<(u64, u64)>>,
	pub(crate) spent: bool,
}

impl PageContent for OutputHtml {
	fn title(&self) -> String {
		format!("Output {}", self.outpoint)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::{blockdata::script, PubkeyHash};

	#[test]
	fn unspent_output() {
		assert_regex_match!(
      OutputHtml {
        chain: Chain::Mainnet,
        inscriptions: Some(Vec::new()),
        outpoint: outpoint(1),
        output: TxOut { value: Amount::from_sat(3), script_pubkey: ScriptBuf::new_p2pkh(&PubkeyHash::all_zeros()), },
        runes: Some(BTreeMap::new()),
        sat_ranges: Some(vec![(0, 1), (1, 3)]),
        spent: false,
      },
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>value</dt><dd>3</dd>
          <dt>script pubkey</dt><dd class=monospace>OP_DUP OP_HASH160 OP_PUSHBYTES_20 0{40} OP_EQUALVERIFY OP_CHECKSIG</dd>
          <dt>address</dt><dd><a class=collapse href=/address/1111111111111111111114oLvT2>1111111111111111111114oLvT2</a></dd>
          <dt>transaction</dt><dd><a class=collapse href=/tx/1{64}>1{64}</a></dd>
          <dt>spent</dt><dd>false</dd>
        </dl>
        <h2>2 Sat Ranges</h2>
        <ul class=monospace>
          <li><a href=/sat/0 class=mythic>0</a></li>
          <li><a href=/sat/1 class=common>1</a>-<a href=/sat/2 class=common>2</a> \\(2 sats\\)</li>
        </ul>
      "
      .unindent()
    );
	}

	#[test]
	fn spent_output() {
		assert_regex_match!(
			OutputHtml {
				chain: Chain::Mainnet,
				inscriptions: None,
				outpoint: outpoint(1),
				output: TxOut {
					value: Amount::from_sat(1),
					script_pubkey: script::Builder::new().push_int(0).into_script(),
				},
				runes: None,
				sat_ranges: None,
				spent: true,
			},
			"
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>value</dt><dd>1</dd>
          <dt>script pubkey</dt><dd class=monospace>OP_0</dd>
          <dt>transaction</dt><dd><a class=collapse href=/tx/1{64}>1{64}</a></dd>
          <dt>spent</dt><dd>true</dd>
        </dl>
      "
			.unindent()
		);
	}

	#[test]
	fn spent_output_with_ranges() {
		assert_regex_match!(
      OutputHtml {
        chain: Chain::Mainnet,
        inscriptions: None,
        outpoint: outpoint(1),
        output: TxOut { value: Amount::from_sat(3), script_pubkey: ScriptBuf::new_p2pkh(&PubkeyHash::all_zeros()), },
        runes: None,
        sat_ranges: Some(vec![(0, 1), (1, 3)]),
        spent: true,
      },
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>value</dt><dd>3</dd>
          <dt>script pubkey</dt><dd class=monospace>OP_DUP OP_HASH160 OP_PUSHBYTES_20 0{40} OP_EQUALVERIFY OP_CHECKSIG</dd>
          <dt>address</dt><dd><a class=collapse href=/address/1111111111111111111114oLvT2>1111111111111111111114oLvT2</a></dd>
          <dt>transaction</dt><dd><a class=collapse href=/tx/1{64}>1{64}</a></dd>
          <dt>spent</dt><dd>true</dd>
        </dl>
        <h2>2 Sat Ranges</h2>
        <ul class=monospace>
          <li><a href=/sat/0 class=mythic>0</a></li>
          <li><a href=/sat/1 class=common>1</a>-<a href=/sat/2 class=common>2</a> \\(2 sats\\)</li>
        </ul>
      "
      .unindent()
    );
	}

	#[test]
	fn no_list() {
		assert_regex_match!(
      OutputHtml {
        chain: Chain::Mainnet,
        inscriptions: None,
        outpoint: outpoint(1),
        output: TxOut { value: Amount::from_sat(3), script_pubkey: ScriptBuf::new_p2pkh(&PubkeyHash::all_zeros()), },
        runes: None,
        sat_ranges: None,
        spent: false,
      }
      .to_string(),
      "
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>value</dt><dd>3</dd>
          <dt>script pubkey</dt><dd class=monospace>OP_DUP OP_HASH160 OP_PUSHBYTES_20 0{40} OP_EQUALVERIFY OP_CHECKSIG</dd>
          <dt>address</dt><dd><a class=collapse href=/address/1111111111111111111114oLvT2>1111111111111111111114oLvT2</a></dd>
          <dt>transaction</dt><dd><a class=collapse href=/tx/1{64}>1{64}</a></dd>
          <dt>spent</dt><dd>false</dd>
        </dl>
      "
      .unindent()
    );
	}

	#[test]
	fn with_inscriptions() {
		assert_regex_match!(
			OutputHtml {
				chain: Chain::Mainnet,
				inscriptions: Some(vec![inscription_id(1)]),
				outpoint: outpoint(1),
				output: TxOut {
					value: Amount::from_sat(3),
					script_pubkey: ScriptBuf::new_p2pkh(&PubkeyHash::all_zeros()),
				},
				runes: None,
				sat_ranges: None,
				spent: false,
			},
			"
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>inscriptions</dt>
          <dd class=thumbnails>
            <a href=/inscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          </dd>
          .*
        </dl>
      "
			.unindent()
		);
	}

	#[test]
	fn with_runes() {
		assert_regex_match!(
			OutputHtml {
				chain: Chain::Mainnet,
				inscriptions: None,
				outpoint: outpoint(1),
				output: TxOut {
					value: Amount::from_sat(3),
					script_pubkey: ScriptBuf::new_p2pkh(&PubkeyHash::all_zeros()),
				},
				runes: Some(
					vec![(
						SpacedRune { rune: Rune(26), spacers: 1 },
						Pile { amount: 11, divisibility: 1, symbol: None }
					)]
					.into_iter()
					.collect()
				),
				sat_ranges: None,
				spent: false,
			},
			"
        <h1>Output <span class=monospace>1{64}:1</span></h1>
        <dl>
          <dt>runes</dt>
          <dd>
            <table>
              <tr>
                <th>rune</th>
                <th>balance</th>
              </tr>
              <tr>
                <td><a href=/rune/A•A>A•A</a></td>
                <td>1.1\u{A0}¤</td>
              </tr>
            </table>
          </dd>
          .*
        </dl>
      "
			.unindent()
		);
	}
}
