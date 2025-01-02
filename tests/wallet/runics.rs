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
use ord::{decimal::Decimal, subcommand::wallet::runics::RunicUtxo};

#[test]
fn wallet_runics() {
	let core = mockcore::builder().network(Network::Regtest).build();
	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-runes"], &[]);

	create_wallet(&core, &ord);

	let rune = Rune(RUNE);

	batch(
		&core,
		&ord,
		batch::File {
			etching: Some(batch::Etching {
				divisibility: 0,
				premine: "1000".parse().unwrap(),
				rune: SpacedRune { rune, spacers: 1 },
				supply: "1000".parse().unwrap(),
				symbol: '¢',
				terms: None,
				turbo: false,
			}),
			inscriptions: vec![batch::Entry { file: Some("inscription.jpeg".into()), ..default() }],
			..default()
		},
	);

	pretty_assert_eq!(
		CommandBuilder::new("--regtest --index-runes wallet runics")
			.core(&core)
			.ord(&ord)
			.run_and_deserialize_output::<Vec<RunicUtxo>>()
			.first()
			.unwrap()
			.runes,
		vec![(SpacedRune { rune, spacers: 1 }, Decimal { value: 1000, scale: 0 })]
			.into_iter()
			.collect()
	);
}
