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
fn dumped_descriptors_match_wallet_descriptors() {
	let core = mockcore::spawn();
	let ord = TestServer::spawn(&core);

	create_wallet(&core, &ord);

	let output = CommandBuilder::new("wallet dump")
		.core(&core)
		.ord(&ord)
		.stderr_regex(".*")
		.run_and_deserialize_output::<ListDescriptorsResult>();

	assert!(core
		.descriptors()
		.iter()
		.zip(output.descriptors.iter())
		.all(|(wallet_descriptor, output_descriptor)| *wallet_descriptor == output_descriptor.desc));
}

#[test]
fn dumped_descriptors_restore() {
	let core = mockcore::spawn();
	let ord = TestServer::spawn(&core);

	create_wallet(&core, &ord);

	let output = CommandBuilder::new("wallet dump")
		.core(&core)
		.ord(&ord)
		.stderr_regex(".*")
		.run_and_deserialize_output::<ListDescriptorsResult>();

	let core = mockcore::spawn();

	CommandBuilder::new("wallet restore --from descriptor")
		.stdin(serde_json::to_string(&output).unwrap().as_bytes().to_vec())
		.core(&core)
		.ord(&ord)
		.run_and_extract_stdout();

	assert!(core
		.descriptors()
		.iter()
		.zip(output.descriptors.iter())
		.all(|(wallet_descriptor, output_descriptor)| *wallet_descriptor == output_descriptor.desc));
}

#[test]
fn dump_and_restore_descriptors_with_minify() {
	let core = mockcore::spawn();
	let ord = TestServer::spawn(&core);

	create_wallet(&core, &ord);

	let output = CommandBuilder::new("--format minify wallet dump")
		.core(&core)
		.ord(&ord)
		.stderr_regex(".*")
		.run_and_deserialize_output::<ListDescriptorsResult>();

	let core = mockcore::spawn();

	CommandBuilder::new("wallet restore --from descriptor")
		.stdin(serde_json::to_string(&output).unwrap().as_bytes().to_vec())
		.core(&core)
		.ord(&ord)
		.run_and_extract_stdout();

	assert!(core
		.descriptors()
		.iter()
		.zip(output.descriptors.iter())
		.all(|(wallet_descriptor, output_descriptor)| *wallet_descriptor == output_descriptor.desc));
}
