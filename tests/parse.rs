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
use ord::{subcommand::parse::Output, Object};

#[test]
fn name() {
	assert_eq!(
		CommandBuilder::new("parse a").run_and_deserialize_output::<Output>(),
		Output { object: Object::Integer(2099999997689999) }
	);
}

#[test]
fn hash() {
	assert_eq!(
		CommandBuilder::new(
			"parse 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
		)
		.run_and_deserialize_output::<Output>(),
		Output {
			object: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
				.parse::<Object>()
				.unwrap(),
		}
	);
}

#[test]
fn unrecognized_object() {
	CommandBuilder::new("parse Az")
		.stderr_regex(r"error: .*: Unrecognized representation.*")
		.expected_exit_code(2)
		.run_and_extract_stdout();
}
