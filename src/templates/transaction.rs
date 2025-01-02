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
pub struct TransactionHtml {
	pub chain: Chain,
	pub etching: Option<SpacedRune>,
	pub inscription_count: u32,
	pub transaction: Transaction,
	pub txid: Txid,
}

impl PageContent for TransactionHtml {
	fn title(&self) -> String {
		format!("Transaction {}", self.txid)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::blockdata::script;

	#[test]
	fn html() {
		let transaction = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![TxIn {
				sequence: Default::default(),
				previous_output: Default::default(),
				script_sig: Default::default(),
				witness: Default::default(),
			}],
			output: vec![
				TxOut {
					value: Amount::from_sat(50 * COIN_VALUE),
					script_pubkey: script::Builder::new().push_int(0).into_script(),
				},
				TxOut {
					value: Amount::from_sat(50 * COIN_VALUE),
					script_pubkey: script::Builder::new().push_int(1).into_script(),
				},
			],
		};

		let txid = transaction.compute_txid();

		pretty_assert_eq!(
      TransactionHtml {
        chain: Chain::Mainnet,
        etching: None,
        inscription_count: 0,
        txid: transaction.compute_txid(),
        transaction,
      }.to_string(),
      format!(
        "
        <h1>Transaction <span class=monospace>{txid}</span></h1>
        <dl>
        </dl>
        <h2>1 Input</h2>
        <ul>
          <li><a class=collapse href=/output/0000000000000000000000000000000000000000000000000000000000000000:4294967295>0000000000000000000000000000000000000000000000000000000000000000:4294967295</a></li>
        </ul>
        <h2>2 Outputs</h2>
        <ul class=monospace>
          <li>
            <a href=/output/{txid}:0 class=collapse>
              {txid}:0
            </a>
            <dl>
              <dt>value</dt><dd>5000000000</dd>
              <dt>script pubkey</dt><dd class=monospace>OP_0</dd>
            </dl>
          </li>
          <li>
            <a href=/output/{txid}:1 class=collapse>
              {txid}:1
            </a>
            <dl>
              <dt>value</dt><dd>5000000000</dd>
              <dt>script pubkey</dt><dd class=monospace>OP_PUSHNUM_1</dd>
            </dl>
          </li>
        </ul>
      "
      )
      .unindent()
    );
	}
}
