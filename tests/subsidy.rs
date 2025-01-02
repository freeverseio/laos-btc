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
use ord::subcommand::subsidy::Output;

#[test]
fn genesis() {
	assert_eq!(
		CommandBuilder::new("subsidy 0").run_and_deserialize_output::<Output>(),
		Output { first: 0, subsidy: 5000000000, name: "nvtdijuwxlp".into() }
	);
}

#[test]
fn second_block() {
	assert_eq!(
		CommandBuilder::new("subsidy 1").run_and_deserialize_output::<Output>(),
		Output { first: 5000000000, subsidy: 5000000000, name: "nvtcsezkbth".into() }
	);
}

#[test]
fn second_to_last_block_with_subsidy() {
	assert_eq!(
		CommandBuilder::new("subsidy 6929998").run_and_deserialize_output::<Output>(),
		Output { first: 2099999997689998, subsidy: 1, name: "b".into() }
	);
}

#[test]
fn last_block_with_subsidy() {
	assert_eq!(
		CommandBuilder::new("subsidy 6929999").run_and_deserialize_output::<Output>(),
		Output { first: 2099999997689999, subsidy: 1, name: "a".into() }
	);
}

#[test]
fn first_block_without_subsidy() {
	CommandBuilder::new("subsidy 6930000")
		.expected_stderr("error: block 6930000 has no subsidy\n")
		.expected_exit_code(1)
		.run_and_extract_stdout();
}
