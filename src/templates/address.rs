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
pub(crate) struct AddressHtml {
	pub(crate) address: Address,
	pub(crate) outputs: Vec<OutPoint>,
	pub(crate) inscriptions: Option<Vec<InscriptionId>>,
	pub(crate) sat_balance: u64,
	pub(crate) runes_balances: Option<Vec<(SpacedRune, Decimal, Option<char>)>>,
}

impl PageContent for AddressHtml {
	fn title(&self) -> String {
		format!("Address {}", self.address)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn setup() -> AddressHtml {
		AddressHtml {
			address: Address::from_str(
				"bc1phuq0vkls6w926zdaem6x9n02z2gg7j2xfudgwddyey7uyquarlgsh40ev8",
			)
			.unwrap()
			.require_network(Network::Bitcoin)
			.unwrap(),
			outputs: vec![outpoint(1), outpoint(2)],
			inscriptions: Some(vec![inscription_id(1)]),
			sat_balance: 99,
			runes_balances: Some(vec![
				(
					SpacedRune { rune: Rune::from_str("TEEEEEEEEESTRUNE").unwrap(), spacers: 0 },
					Decimal { scale: 0, value: 20000 },
					Some('R'),
				),
				(
					SpacedRune { rune: Rune::from_str("ANOTHERTEESTRUNE").unwrap(), spacers: 0 },
					Decimal { scale: 0, value: 10000 },
					Some('F'),
				),
			]),
		}
	}

	#[test]
	fn test_address_rendering() {
		let address_html = setup();
		let expected_pattern = r#".*<h1>Address bc1phuq0vkls6w926zdaem6x9n02z2gg7j2xfudgwddyey7uyquarlgsh40ev8</h1>.*"#;
		assert_regex_match!(address_html, expected_pattern);
	}

	#[test]
	fn test_sat_balance_rendering() {
		let address_html = setup();
		let expected_pattern = r#".*<dt>sat balance</dt>\n\s*<dd>99</dd>.*"#;
		assert_regex_match!(address_html, expected_pattern);
	}

	#[test]
	fn test_inscriptions_rendering() {
		let address_html = setup();
		let expected_pattern = r#".*<dt>inscriptions</dt>\n\s*<dd class=thumbnails>.*<a href=/inscription/1{64}i1><iframe .* src=/preview/1{64}i1></iframe></a>.*</dd>.*"#;
		assert_regex_match!(address_html, expected_pattern);
	}

	#[test]
	fn test_runes_balances_rendering() {
		let address_html = setup();
		let expected_pattern = r#".*<dt>rune balances</dt>\n\s*<dd><a class=monospace href=/rune/TEEEEEEEEESTRUNE>TEEEEEEEEESTRUNE</a>: 20000R</dd>\n\s*<dd><a class=monospace href=/rune/ANOTHERTEESTRUNE>ANOTHERTEESTRUNE</a>: 10000F</dd>.*"#;
		assert_regex_match!(address_html, expected_pattern);
	}

	#[test]
	fn test_outputs_rendering() {
		let address_html = setup();
		let expected_pattern = r#".*<dt>outputs</dt>\n\s*<dd>\n\s*<ul>\n\s*<li><a class=collapse href=/output/1{64}:1>1{64}:1</a></li>\n\s*<li><a class=collapse href=/output/2{64}:2>2{64}:2</a></li>\n\s*</ul>\n\s*</dd>.*"#;
		assert_regex_match!(address_html, expected_pattern);
	}
}
