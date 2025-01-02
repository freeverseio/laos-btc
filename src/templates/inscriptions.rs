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
pub(crate) struct InscriptionsHtml {
	pub(crate) inscriptions: Vec<InscriptionId>,
	pub(crate) prev: Option<u32>,
	pub(crate) next: Option<u32>,
}

impl PageContent for InscriptionsHtml {
	fn title(&self) -> String {
		"Inscriptions".into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn without_prev_and_next() {
		assert_regex_match!(
			InscriptionsHtml {
				inscriptions: vec![inscription_id(1), inscription_id(2)],
				prev: None,
				next: None,
			},
			"
        <h1>All Inscriptions</h1>
        <div class=thumbnails>
          <a href=/inscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          <a href=/inscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
        </div>
        .*
        prev
        next
        .*
      "
			.unindent()
		);
	}

	#[test]
	fn with_prev_and_next() {
		assert_regex_match!(
			InscriptionsHtml {
				inscriptions: vec![inscription_id(1), inscription_id(2)],
				prev: Some(1),
				next: Some(2),
			},
			"
        <h1>All Inscriptions</h1>
        <div class=thumbnails>
          <a href=/inscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>
          <a href=/inscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
        </div>
        .*
        <a class=prev href=/inscriptions/1>prev</a>
        <a class=next href=/inscriptions/2>next</a>
        .*
      "
			.unindent()
		);
	}
}
