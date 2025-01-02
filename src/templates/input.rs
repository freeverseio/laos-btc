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
pub(crate) struct InputHtml {
	pub(crate) path: (u32, usize, usize),
	pub(crate) input: TxIn,
}

impl PageContent for InputHtml {
	fn title(&self) -> String {
		format!("Input /{}/{}/{}", self.path.0, self.path.1, self.path.2)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::{blockdata::script, Witness};

	#[test]
	fn input_html() {
		let mut witness = Witness::new();
		witness.push([1]);
		pretty_assert_eq!(
      InputHtml {
        path: (1, 2, 3),
        input: TxIn {
          previous_output: "0000000000000000000000000000000000000000000000000000000000000000:0"
            .parse()
            .unwrap(),
          script_sig: ScriptBuf::builder().push_slice(b"foo").into_script(),
          sequence: Sequence::MAX,
          witness,
        }
      }
      .to_string(),
      "
      <h1>Input /1/2/3</h1>
      <dl>
        <dt>previous output</dt><dd class=collapse>0000000000000000000000000000000000000000000000000000000000000000:0</dd>
        <dt>witness</dt><dd class=monospace>010101</dd>
        <dt>script sig</dt><dd class=monospace>OP_PUSHBYTES_3 666f6f</dd>
        <dt>text</dt><dd>\x03foo</dd>
      </dl>
      "
      .unindent()
    );
	}

	#[test]
	fn skip_empty_items() {
		pretty_assert_eq!(
			InputHtml {
				path: (1, 2, 3),
				input: TxIn {
					previous_output: OutPoint::null(),
					script_sig: script::Builder::new().into_script(),
					sequence: Sequence::MAX,
					witness: Witness::new(),
				}
			}
			.to_string(),
			"
      <h1>Input /1/2/3</h1>
      <dl>
      </dl>
      "
			.unindent()
		);
	}
}
