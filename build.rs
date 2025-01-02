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

use std::{process::Command, str};

fn git_branch() -> Option<String> {
	str::from_utf8(
		&Command::new("git")
			.args(["rev-parse", "--abbrev-ref", "HEAD"])
			.output()
			.ok()?
			.stdout,
	)
	.ok()
	.map(|branch| branch.into())
}

fn git_commit() -> Option<String> {
	str::from_utf8(
		&Command::new("git")
			.args(["rev-parse", "--verify", "HEAD"])
			.output()
			.ok()?
			.stdout,
	)
	.ok()
	.map(|branch| branch.into())
}

fn main() {
	println!("cargo:rustc-env=GIT_BRANCH={}", git_branch().unwrap_or_default());
	println!("cargo:rustc-env=GIT_COMMIT={}", git_commit().unwrap_or_default());
}
