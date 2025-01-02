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

fn re(s: &'static str) -> Regex {
	Regex::new(&format!("^{s}$")).unwrap()
}

lazy_static! {
	pub(crate) static ref ADDRESS: Regex = re(
		r"((bc1|tb1|bcrt1)[qpzry9x8gf2tvdw0s3jn54khce6mua7l]{39,60}|[123][a-km-zA-HJ-NP-Z1-9]{25,34})"
	);
	pub(crate) static ref HASH: Regex = re(r"[[:xdigit:]]{64}");
	pub(crate) static ref INSCRIPTION_ID: Regex = re(r"[[:xdigit:]]{64}i\d+");
	pub(crate) static ref INSCRIPTION_NUMBER: Regex = re(r"-?[0-9]+");
	pub(crate) static ref OUTPOINT: Regex = re(r"[[:xdigit:]]{64}:\d+");
	pub(crate) static ref RUNE_ID: Regex = re(r"[0-9]+:[0-9]+");
	pub(crate) static ref RUNE_NUMBER: Regex = re(r"-?[0-9]+");
	pub(crate) static ref SATPOINT: Regex = re(r"[[:xdigit:]]{64}:\d+:\d+");
	pub(crate) static ref SAT_NAME: Regex = re(r"[a-z]{1,11}");
	pub(crate) static ref SPACED_RUNE: Regex = re(r"[A-Z•.]+");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sat_name() {
		assert!(SAT_NAME.is_match(&Sat(0).name()));
		assert!(SAT_NAME.is_match(&Sat::LAST.name()));
	}
}
