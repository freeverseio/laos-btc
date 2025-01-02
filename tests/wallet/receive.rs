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
use ord::subcommand::wallet::receive;

#[test]
fn receive() {
	let core = mockcore::spawn();
	let ord = TestServer::spawn(&core);

	create_wallet(&core, &ord);

	let output = CommandBuilder::new("wallet receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<receive::Output>();

	assert!(output.addresses.first().unwrap().is_valid_for_network(Network::Bitcoin));
}
