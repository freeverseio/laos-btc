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
fn verify() {
	assert_eq!(
    CommandBuilder::new([
      "verify",
      "--address", "bc1q9vza2e8x573nczrlzms0wvx3gsqjx7vavgkx0l",
      "--text", "Hello World",
      "--witness", "AkcwRAIgZRfIY3p7/DoVTty6YZbWS71bc5Vct9p9Fia83eRmw2QCICK/ENGfwLtptFluMGs2KsqoNSk89pO7F29zJLUx9a/sASECx/EgAxlkQpQ9hYjgGu6EBCPMVPwVIVJqO4XCsMvViHI="
    ])
    .run_and_extract_stdout(),
    ""
  );
}

#[test]
fn verify_fails() {
	CommandBuilder::new([
      "verify",
      "--address", "bc1q9vza2e8x573nczrlzms0wvx3gsqjx7vavgkx0l",
      "--text", "Hello World - this should fail",
      "--witness", "AkcwRAIgZRfIY3p7/DoVTty6YZbWS71bc5Vct9p9Fia83eRmw2QCICK/ENGfwLtptFluMGs2KsqoNSk89pO7F29zJLUx9a/sASECx/EgAxlkQpQ9hYjgGu6EBCPMVPwVIVJqO4XCsMvViHI="
  ])
  .expected_exit_code(1)
  .stderr_regex("error: Invalid signature.*")
  .run_and_extract_stdout();
}

#[test]
fn witness_and_transaction_conflict() {
	CommandBuilder::new([
      "verify",
      "--address", "bc1q9vza2e8x573nczrlzms0wvx3gsqjx7vavgkx0l",
      "--text", "Hello World",
      "--transaction", "asdf",
      "--witness", "AkcwRAIgZRfIY3p7/DoVTty6YZbWS71bc5Vct9p9Fia83eRmw2QCICK/ENGfwLtptFluMGs2KsqoNSk89pO7F29zJLUx9a/sASECx/EgAxlkQpQ9hYjgGu6EBCPMVPwVIVJqO4XCsMvViHI="
  ])
  .expected_exit_code(2)
  .stderr_regex(".*error.*")
  .run_and_extract_stdout();
}

#[test]
fn verify_with_transaction() {
	let tx = bip322::sign_full_encoded(
		"bc1q9vza2e8x573nczrlzms0wvx3gsqjx7vavgkx0l",
		"Hello World",
		"L3VFeEujGtevx9w18HD1fhRbCH67Az2dpCymeRE1SoPK6XQtaN2k",
	)
	.unwrap();

	assert_eq!(
		CommandBuilder::new([
			"verify",
			"--address",
			"bc1q9vza2e8x573nczrlzms0wvx3gsqjx7vavgkx0l",
			"--text",
			"Hello World",
			"--transaction",
			&tx,
		])
		.run_and_extract_stdout(),
		""
	);
}
