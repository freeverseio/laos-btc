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
use ord::subcommand::index::info::TransactionsOutput;

#[test]
fn json_with_satoshi_index() {
	let core = mockcore::spawn();

	let (tempdir, _) = CommandBuilder::new("--index-sats index update").core(&core).run();

	CommandBuilder::new("--index-sats index info")
		.temp_dir(tempdir)
		.core(&core)
		.stdout_regex(
			r#"\{
  "blocks_indexed": 1,
  "branch_pages": \d+,
  "fragmented_bytes": \d+,
  "index_file_size": \d+,
  "index_path": ".*\.redb",
  "leaf_pages": \d+,
  "metadata_bytes": \d+,
  "outputs_traversed": 1,
  "page_size": \d+,
  "sat_ranges": 1,
  "stored_bytes": \d+,
  "tables": .*,
  "total_bytes": \d+,
  "transactions": \[
    \{
      "starting_block_count": 0,
      "starting_timestamp": \d+
    \}
  \],
  "tree_height": \d+,
  "utxos_indexed": 1
\}
"#,
		)
		.run_and_extract_stdout();
}

#[test]
fn json_without_satoshi_index() {
	let core = mockcore::spawn();

	let (tempdir, _) = CommandBuilder::new("index update").core(&core).run();

	CommandBuilder::new("index info")
		.core(&core)
		.temp_dir(tempdir)
		.stdout_regex(
			r#"\{
  "blocks_indexed": 1,
  "branch_pages": \d+,
  "fragmented_bytes": \d+,
  "index_file_size": \d+,
  "index_path": ".*\.redb",
  "leaf_pages": \d+,
  "metadata_bytes": \d+,
  "outputs_traversed": 0,
  "page_size": \d+,
  "sat_ranges": 0,
  "stored_bytes": \d+,
  "tables": .*,
  "total_bytes": \d+,
  "transactions": \[
    \{
      "starting_block_count": 0,
      "starting_timestamp": \d+
    \}
  \],
  "tree_height": \d+,
  "utxos_indexed": 1
\}
"#,
		)
		.run_and_extract_stdout();
}

#[test]
fn transactions() {
	let core = mockcore::spawn();

	let (tempdir, _) = CommandBuilder::new("index update").core(&core).run();

	let output = CommandBuilder::new("index info --transactions")
		.temp_dir(tempdir.clone())
		.core(&core)
		.run_and_deserialize_output::<Vec<TransactionsOutput>>();

	assert!(output.is_empty());

	core.mine_blocks(10);

	CommandBuilder::new("index update").temp_dir(tempdir.clone()).core(&core).run();

	let output = CommandBuilder::new("index info --transactions")
		.temp_dir(tempdir.clone())
		.core(&core)
		.stdout_regex(".*")
		.run_and_deserialize_output::<Vec<TransactionsOutput>>();

	assert_eq!(output[0].start, 0);
	assert_eq!(output[0].end, 1);
	assert_eq!(output[0].count, 1);

	core.mine_blocks(10);

	CommandBuilder::new("index update").temp_dir(tempdir.clone()).core(&core).run();

	let output = CommandBuilder::new("index info --transactions")
		.temp_dir(tempdir.clone())
		.core(&core)
		.run_and_deserialize_output::<Vec<TransactionsOutput>>();

	assert_eq!(output[1].start, 1);
	assert_eq!(output[1].end, 11);
	assert_eq!(output[1].count, 10);
}
