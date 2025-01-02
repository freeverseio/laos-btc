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
pub(crate) struct ChildrenHtml {
	pub(crate) parent: InscriptionId,
	pub(crate) parent_number: i32,
	pub(crate) children: Vec<InscriptionId>,
	pub(crate) prev_page: Option<usize>,
	pub(crate) next_page: Option<usize>,
}

impl PageContent for ChildrenHtml {
	fn title(&self) -> String {
		format!("Inscription {} Children", self.parent_number)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn without_prev_and_next() {
		assert_regex_match!(
			ChildrenHtml {
				parent: inscription_id(1),
				parent_number: 0,
				children: vec![inscription_id(2), inscription_id(3)],
				prev_page: None,
				next_page: None,
			},
			"
        <h1><a href=/inscription/1{64}i1>Inscription 0</a> Children</h1>
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
			ChildrenHtml {
				parent: inscription_id(1),
				parent_number: 0,
				children: vec![inscription_id(2), inscription_id(3)],
				next_page: Some(3),
				prev_page: Some(1),
			},
			"
        <h1><a href=/inscription/1{64}i1>Inscription 0</a> Children</h1>
        <div class=thumbnails>
          <a href=/inscription/2{64}i2><iframe .* src=/preview/2{64}i2></iframe></a>
          <a href=/inscription/3{64}i3><iframe .* src=/preview/3{64}i3></iframe></a>
        </div>
        .*
          <a class=prev href=/children/1{64}i1/1>prev</a>
          <a class=next href=/children/1{64}i1/3>next</a>
        .*
      "
			.unindent()
		);
	}
}
