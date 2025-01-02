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
use nix::{
	sys::signal::{self, Signal},
	unistd::Pid,
};

#[test]
fn wallet_pending() {
	let core = mockcore::builder().network(Network::Regtest).build();
	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-runes"], &[]);

	create_wallet(&core, &ord);

	core.mine_blocks(1);

	let batchfile = batch::File {
		etching: Some(batch::Etching {
			divisibility: 0,
			rune: SpacedRune { rune: Rune(RUNE), spacers: 0 },
			supply: "1000".parse().unwrap(),
			premine: "1000".parse().unwrap(),
			symbol: '¢',
			..default()
		}),
		inscriptions: vec![batch::Entry { file: Some("inscription.jpeg".into()), ..default() }],
		..default()
	};

	let tempdir = Arc::new(TempDir::new().unwrap());

	{
		let mut spawn = CommandBuilder::new(
			"--regtest --index-runes wallet batch --fee-rate 0 --batch batch.yaml",
		)
		.temp_dir(tempdir.clone())
		.write("batch.yaml", serde_yaml::to_string(&batchfile).unwrap())
		.write("inscription.jpeg", "inscription")
		.core(&core)
		.ord(&ord)
		.expected_exit_code(1)
		.spawn();

		let mut buffer = String::new();

		BufReader::new(spawn.child.stderr.as_mut().unwrap())
			.read_line(&mut buffer)
			.unwrap();

		assert_regex_match!(
			buffer,
			"Waiting for rune AAAAAAAAAAAAA commitment [[:xdigit:]]{64} to mature…\n"
		);

		core.mine_blocks(1);

		signal::kill(Pid::from_raw(spawn.child.id().try_into().unwrap()), Signal::SIGINT).unwrap();

		buffer.clear();

		BufReader::new(spawn.child.stderr.as_mut().unwrap())
			.read_line(&mut buffer)
			.unwrap();

		assert_eq!(
			buffer,
			"Shutting down gracefully. Press <CTRL-C> again to shutdown immediately.\n"
		);

		spawn.child.wait().unwrap();
	}

	let output = CommandBuilder::new("--regtest --index-runes wallet pending")
		.temp_dir(tempdir)
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<ord::subcommand::wallet::pending::PendingOutput>>();

	assert_eq!(output.first().unwrap().rune.rune, Rune(RUNE));
}
