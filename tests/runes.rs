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
use ord::subcommand::runes::Output;

#[test]
fn flag_is_required() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest"], &[]);

	CommandBuilder::new("--regtest runes")
		.core(&core)
		.ord(&ord)
		.expected_exit_code(1)
		.expected_stderr("error: `ord runes` requires index created with `--index-runes` flag\n")
		.run_and_extract_stdout();
}

#[test]
fn no_runes() {
	let core = mockcore::builder().network(Network::Regtest).build();

	assert_eq!(
		CommandBuilder::new("--index-runes --regtest runes")
			.core(&core)
			.run_and_deserialize_output::<Output>(),
		Output { runes: BTreeMap::new() }
	);
}

#[test]
fn one_rune() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-runes"], &[]);

	create_wallet(&core, &ord);

	let etch = etch(&core, &ord, Rune(RUNE));

	pretty_assert_eq!(
		CommandBuilder::new("--index-runes --regtest runes")
			.core(&core)
			.run_and_deserialize_output::<Output>(),
		Output {
			runes: vec![(
				Rune(RUNE),
				RuneInfo {
					block: 7,
					burned: 0,
					divisibility: 0,
					etching: etch.output.reveal,
					id: RuneId { block: 7, tx: 1 },
					terms: None,
					mints: 0,
					number: 0,
					premine: 1000,
					rune: SpacedRune { rune: Rune(RUNE), spacers: 0 },
					supply: 1000,
					symbol: Some('¢'),
					timestamp: ord::timestamp(7),
					turbo: false,
					tx: 1,
				}
			)]
			.into_iter()
			.collect(),
		}
	);
}

#[test]
fn two_runes() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-runes"], &[]);

	create_wallet(&core, &ord);

	let a = etch(&core, &ord, Rune(RUNE));
	let b = etch(&core, &ord, Rune(RUNE + 1));

	pretty_assert_eq!(
		CommandBuilder::new("--index-runes --regtest runes")
			.core(&core)
			.run_and_deserialize_output::<Output>(),
		Output {
			runes: vec![
				(
					Rune(RUNE),
					RuneInfo {
						block: 7,
						burned: 0,
						divisibility: 0,
						etching: a.output.reveal,
						id: RuneId { block: 7, tx: 1 },
						terms: None,
						mints: 0,
						number: 0,
						premine: 1000,
						rune: SpacedRune { rune: Rune(RUNE), spacers: 0 },
						supply: 1000,
						symbol: Some('¢'),
						timestamp: ord::timestamp(7),
						turbo: false,
						tx: 1,
					}
				),
				(
					Rune(RUNE + 1),
					RuneInfo {
						block: 14,
						burned: 0,
						divisibility: 0,
						etching: b.output.reveal,
						id: RuneId { block: 14, tx: 1 },
						terms: None,
						mints: 0,
						number: 1,
						premine: 1000,
						rune: SpacedRune { rune: Rune(RUNE + 1), spacers: 0 },
						supply: 1000,
						symbol: Some('¢'),
						timestamp: ord::timestamp(14),
						turbo: false,
						tx: 1,
					}
				)
			]
			.into_iter()
			.collect(),
		}
	);
}
