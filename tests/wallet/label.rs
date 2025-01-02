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
fn label() {
	let core = mockcore::spawn();

	let ord = TestServer::spawn_with_server_args(&core, &["--index-sats"], &[]);

	create_wallet(&core, &ord);

	core.mine_blocks(2);

	let (inscription, _reveal) = inscribe(&core, &ord);

	let output = CommandBuilder::new("wallet label")
		.core(&core)
		.ord(&ord)
		.stdout_regex(".*")
		.run_and_extract_stdout();

	assert!(output
		.contains(r#"\"name\":\"nvtcsezkbth\",\"number\":5000000000,\"rarity\":\"uncommon\""#));

	assert!(output
		.contains(r#"\"name\":\"nvtccadxgaz\",\"number\":10000000000,\"rarity\":\"uncommon\""#));

	assert!(output.contains(&inscription.to_string()));
}
