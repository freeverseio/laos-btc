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
use ord::subcommand::wallet::{cardinals::CardinalUtxo, outputs::Output};

#[test]
fn cardinals() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &[], &[]);

	create_wallet(&core, &ord);

	inscribe(&core, &ord);

	let all_outputs = CommandBuilder::new("wallet outputs")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<Output>>();

	let cardinal_outputs = CommandBuilder::new("wallet cardinals")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<CardinalUtxo>>();

	assert_eq!(all_outputs.len() - cardinal_outputs.len(), 1);
}

#[test]
fn cardinals_does_not_show_runic_outputs() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-runes"], &[]);

	create_wallet(&core, &ord);

	core.mine_blocks(1);

	batch(
		&core,
		&ord,
		batch::File {
			etching: Some(batch::Etching {
				supply: "1000".parse().unwrap(),
				divisibility: 0,
				terms: None,
				premine: "1000".parse().unwrap(),
				rune: SpacedRune { rune: Rune(RUNE), spacers: 0 },
				symbol: '¢',
				turbo: false,
			}),
			inscriptions: vec![batch::Entry { file: Some("inscription.jpeg".into()), ..default() }],
			..default()
		},
	);

	let all_outputs = CommandBuilder::new("--regtest wallet outputs")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<Output>>();

	let cardinal_outputs = CommandBuilder::new("--regtest wallet cardinals")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<CardinalUtxo>>();

	assert_eq!(all_outputs.len() - cardinal_outputs.len(), 2);
}
