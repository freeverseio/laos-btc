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

#[derive(Boilerplate, Debug, PartialEq, Serialize)]
pub struct RuneNotFoundHtml {
	pub rune: Rune,
	pub unlock_height: Option<Height>,
}

impl PageContent for RuneNotFoundHtml {
	fn title(&self) -> String {
		format!("Rune {}", self.rune)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display() {
		assert_regex_match!(
			RuneNotFoundHtml { rune: Rune(u128::MAX), unlock_height: Some(Height(111)) },
			"<h1>BCGDENLQRQWDSLRUGSNLBTMFIJAV</h1>
<dl>
  <dt>unlock height</dt>
  <dd>111</dd>
  <dt>reserved</dt>
  <dd>false</dd>
</dl>
"
		);
	}

	#[test]
	fn display_reserved() {
		assert_regex_match!(
			RuneNotFoundHtml { rune: Rune(Rune::RESERVED), unlock_height: None },
			"<h1>AAAAAAAAAAAAAAAAAAAAAAAAAAA</h1>
<dl>
  <dt>unlock height</dt>
  <dd>none</dd>
  <dt>reserved</dt>
  <dd>true</dd>
</dl>
"
		);
	}
}
