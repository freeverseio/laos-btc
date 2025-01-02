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

#[derive(Debug)]
pub(crate) enum Expected {
	String(String),
	Regex(Regex),
}

impl Expected {
	pub(crate) fn regex(pattern: &str) -> Self {
		Self::Regex(Regex::new(&format!("^(?s){pattern}$")).unwrap())
	}

	#[track_caller]
	pub(crate) fn assert_match(&self, output: &str) {
		match self {
			Self::String(string) => pretty_assert_eq!(output, string),
			Self::Regex(regex) =>
				if !regex.is_match(output) {
					eprintln!("Regex did not match:");
					pretty_assert_eq!(regex.as_str(), output);
				},
		}
	}
}
