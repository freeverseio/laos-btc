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
use ord::subcommand::wallet::outputs::Output;

#[test]
fn outputs() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &[], &[]);

	create_wallet(&core, &ord);

	let coinbase_tx = &core.mine_blocks_with_subsidy(1, 1_000_000)[0].txdata[0];
	let outpoint = OutPoint::new(coinbase_tx.compute_txid(), 0);
	let amount = coinbase_tx.output[0].value;

	let output = CommandBuilder::new("wallet outputs")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<Output>>();

	assert_eq!(output[0].output, outpoint);
	assert_eq!(output[0].amount, amount.to_sat());
	assert!(output[0].sat_ranges.is_none());
}

#[test]
fn outputs_includes_locked_outputs() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &[], &[]);

	create_wallet(&core, &ord);

	let coinbase_tx = &core.mine_blocks_with_subsidy(1, 1_000_000)[0].txdata[0];
	let outpoint = OutPoint::new(coinbase_tx.compute_txid(), 0);
	let amount = coinbase_tx.output[0].value;

	core.lock(outpoint);

	let output = CommandBuilder::new("wallet outputs")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<Output>>();

	assert_eq!(output[0].output, outpoint);
	assert_eq!(output[0].amount, amount.to_sat());
	assert!(output[0].sat_ranges.is_none());
}

#[test]
fn outputs_includes_unbound_outputs() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &[], &[]);

	create_wallet(&core, &ord);

	let coinbase_tx = &core.mine_blocks_with_subsidy(1, 1_000_000)[0].txdata[0];
	let outpoint = OutPoint::new(coinbase_tx.compute_txid(), 0);
	let amount = coinbase_tx.output[0].value;

	core.lock(outpoint);

	let output = CommandBuilder::new("wallet outputs")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<Output>>();

	assert_eq!(output[0].output, outpoint);
	assert_eq!(output[0].amount, amount.to_sat());
	assert!(output[0].sat_ranges.is_none());
}

#[test]
fn outputs_includes_sat_ranges() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &["--index-sats"], &[]);

	create_wallet(&core, &ord);

	let coinbase_tx = &core.mine_blocks_with_subsidy(1, 1_000_000)[0].txdata[0];
	let outpoint = OutPoint::new(coinbase_tx.compute_txid(), 0);
	let amount = coinbase_tx.output[0].value;

	let output = CommandBuilder::new("wallet outputs --ranges")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<Output>>();

	assert_eq!(output[0].output, outpoint);
	assert_eq!(output[0].amount, amount.to_sat());
	assert_eq!(output[0].sat_ranges, Some(vec!["5000000000-5001000000".to_string()]));
}

#[test]
fn outputs_includes_runes_and_inscriptions() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-runes"], &[]);

	create_wallet(&core, &ord);

	let rune = Rune(RUNE);

	let etched = batch(
		&core,
		&ord,
		batch::File {
			etching: Some(batch::Etching {
				divisibility: 3,
				premine: "1.111".parse().unwrap(),
				rune: SpacedRune { rune, spacers: 1 },
				supply: "2.222".parse().unwrap(),
				symbol: '¢',
				terms: Some(batch::Terms { amount: "1.111".parse().unwrap(), cap: 1, ..default() }),
				turbo: false,
			}),
			inscriptions: vec![batch::Entry { file: Some("inscription.jpeg".into()), ..default() }],
			..default()
		},
	);

	let output = CommandBuilder::new("--regtest --index-runes wallet outputs")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<Vec<Output>>();

	assert!(output.contains(&Output {
		output: etched.output.rune.clone().unwrap().location.unwrap(),
		address: etched.output.rune.unwrap().destination,
		amount: 10000,
		inscriptions: Some(Vec::new()),
		runes: Some(
			vec![(
				SpacedRune { rune, spacers: 1 },
				ord::decimal::Decimal { value: 1111, scale: 3 }
			)]
			.into_iter()
			.collect()
		),
		sat_ranges: None,
	}));

	assert!(output.contains(&Output {
		output: etched.output.inscriptions[0].location.outpoint,
		address: Some(etched.output.inscriptions[0].destination.clone()),
		amount: 10000,
		inscriptions: Some(vec![etched.output.inscriptions[0].id]),
		runes: Some(BTreeMap::new()),
		sat_ranges: None,
	}));
}
