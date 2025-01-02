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
use ord::subcommand::wallet::sats::{OutputAll, OutputRare, OutputTsv};

#[test]
fn requires_sat_index() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &[], &[]);

	create_wallet(&core, &ord);

	CommandBuilder::new("wallet sats")
		.core(&core)
		.ord(&ord)
		.expected_exit_code(1)
		.expected_stderr("error: sats requires index created with `--index-sats` flag\n")
		.run_and_extract_stdout();
}

#[test]
fn sats() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &["--index-sats"], &[]);

	create_wallet(&core, &ord);

	let second_coinbase = core.mine_blocks(1)[0].txdata[0].compute_txid();

	let output = CommandBuilder::new("--index-sats wallet sats")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<OutputRare>>();

	assert_eq!(output[0].sat, 50 * COIN_VALUE);
	assert_eq!(output[0].output.to_string(), format!("{second_coinbase}:0"));
}

#[test]
fn sats_from_tsv_success() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &["--index-sats"], &[]);

	create_wallet(&core, &ord);

	let second_coinbase = core.mine_blocks(1)[0].txdata[0].compute_txid();

	let output = CommandBuilder::new("--index-sats wallet sats --tsv foo.tsv")
		.write("foo.tsv", "nvtcsezkbtg")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<OutputTsv>();

	assert_eq!(output.found["nvtcsezkbtg"].to_string(), format!("{second_coinbase}:0:1"));
}

#[test]
fn sats_from_tsv_parse_error() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &["--index-sats"], &[]);

	create_wallet(&core, &ord);

	CommandBuilder::new("--index-sats wallet sats --tsv foo.tsv")
    .write("foo.tsv", "===")
    .core(&core)
    .ord(&ord)
    .expected_exit_code(1)
    .expected_stderr(
      "error: failed to parse sat from string \"===\" on line 1: failed to parse sat `===`: invalid integer: invalid digit found in string\n",
    )
    .run_and_extract_stdout();
}

#[test]
fn sats_from_tsv_file_not_found() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &["--index-sats"], &[]);

	create_wallet(&core, &ord);

	CommandBuilder::new("--index-sats wallet sats --tsv foo.tsv")
		.core(&core)
		.ord(&ord)
		.expected_exit_code(1)
		.stderr_regex("error: I/O error reading `.*`\n\nbecause:.*")
		.run_and_extract_stdout();
}

#[test]
fn sats_all() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &["--index-sats"], &[]);

	create_wallet(&core, &ord);

	let second_coinbase = core.mine_blocks(1)[0].txdata[0].compute_txid();

	let output = CommandBuilder::new("--index-sats wallet sats --all")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<OutputAll>>();

	assert_eq!(
		output,
		vec![OutputAll {
			output: format!("{second_coinbase}:0").parse::<OutPoint>().unwrap(),
			ranges: vec![format!("{}-{}", 50 * COIN_VALUE, 100 * COIN_VALUE)],
		}]
		.into_iter()
		.collect::<Vec<OutputAll>>()
	);
}
