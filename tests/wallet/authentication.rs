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
use ord::subcommand::wallet::balance::Output;

#[test]
fn authentication() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(
		&core,
		&["--server-username", "foo", "--server-password", "bar"],
		&[],
	);

	create_wallet(&core, &ord);

	assert_eq!(
		CommandBuilder::new("--server-username foo --server-password bar wallet balance")
			.core(&core)
			.ord(&ord)
			.run_and_deserialize_output::<Output>()
			.cardinal,
		0
	);

	core.mine_blocks(1);

	assert_eq!(
		CommandBuilder::new("--server-username foo --server-password bar wallet balance")
			.core(&core)
			.ord(&ord)
			.run_and_deserialize_output::<Output>(),
		Output {
			cardinal: 50 * COIN_VALUE,
			ordinal: 0,
			runic: None,
			runes: None,
			total: 50 * COIN_VALUE,
		}
	);
}
