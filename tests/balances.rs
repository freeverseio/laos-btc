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

#[test]
fn flag_is_required() {
	let core = mockcore::builder().network(Network::Regtest).build();

	CommandBuilder::new("--regtest balances")
		.core(&core)
		.expected_exit_code(1)
		.expected_stderr("error: `ord balances` requires index created with `--index-runes` flag\n")
		.run_and_extract_stdout();
}

#[test]
fn no_runes() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let output = CommandBuilder::new("--regtest --index-runes balances")
		.core(&core)
		.run_and_deserialize_output::<Balances>();

	assert_eq!(output, Balances { runes: BTreeMap::new() });
}

#[test]
fn with_runes() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-runes"], &[]);

	create_wallet(&core, &ord);

	let a = etch(&core, &ord, Rune(RUNE));
	let b = etch(&core, &ord, Rune(RUNE + 1));

	let output = CommandBuilder::new("--regtest --index-runes balances")
		.core(&core)
		.run_and_deserialize_output::<Balances>();

	assert_eq!(
		output,
		Balances {
			runes: [
				(
					SpacedRune::new(Rune(RUNE), 0),
					[(
						OutPoint { txid: a.output.reveal, vout: 1 },
						Pile { amount: 1000, divisibility: 0, symbol: Some('¢') },
					)]
					.into()
				),
				(
					SpacedRune::new(Rune(RUNE + 1), 0),
					[(
						OutPoint { txid: b.output.reveal, vout: 1 },
						Pile { amount: 1000, divisibility: 0, symbol: Some('¢') },
					)]
					.into()
				),
			]
			.into()
		}
	);
}
