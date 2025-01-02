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
pub(crate) struct ParentsHtml {
	pub(crate) id: InscriptionId,
	pub(crate) number: i32,
	pub(crate) parents: Vec<InscriptionId>,
	pub(crate) prev_page: Option<usize>,
	pub(crate) next_page: Option<usize>,
}

impl PageContent for ParentsHtml {
	fn title(&self) -> String {
		format!("Inscription {} Parents", self.number)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn without_prev_and_next() {
		assert_regex_match!(
			ParentsHtml {
				id: inscription_id(1),
				number: 0,
				parents: vec![inscription_id(2), inscription_id(3)],
				prev_page: None,
				next_page: None,
			},
			"
        <h1><a href=/inscription/1{64}i1>Inscription 0</a> Parents</h1>
        <div class=thumbnails>
          <a href=/inscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
          <a href=/inscription/3{64}i3><iframe .* src=/preview/3{64}i3></iframe></a>
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
			ParentsHtml {
				id: inscription_id(1),
				number: 0,
				parents: vec![inscription_id(2), inscription_id(3)],
				next_page: Some(3),
				prev_page: Some(1),
			},
			"
        <h1><a href=/inscription/1{64}i1>Inscription 0</a> Parents</h1>
        <div class=thumbnails>
          <a href=/inscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
          <a href=/inscription/3{64}i3><iframe .* src=/preview/3{64}i3></iframe></a>
        </div>
        .*
          <a class=prev href=/parents/1{64}i1/1>prev</a>
          <a class=next href=/parents/1{64}i1/3>next</a>
        .*
      "
			.unindent()
		);
	}
}
