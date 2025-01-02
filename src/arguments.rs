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
use clap::builder::styling::{AnsiColor, Effects, Styles};

#[derive(Debug, Parser)]
#[command(
  version,
  styles = Styles::styled()
    .error(AnsiColor::Red.on_default() | Effects::BOLD)
    .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
    .invalid(AnsiColor::Red.on_default())
    .literal(AnsiColor::Blue.on_default())
    .placeholder(AnsiColor::Cyan.on_default())
    .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
    .valid(AnsiColor::Green.on_default()),
)]
pub(crate) struct Arguments {
	#[command(flatten)]
	pub(crate) options: Options,
	#[command(subcommand)]
	pub(crate) subcommand: Subcommand,
}

impl Arguments {
	pub(crate) fn run(self) -> SnafuResult<Option<Box<dyn subcommand::Output>>> {
		let mut env: BTreeMap<String, String> = BTreeMap::new();

		for (variable, value) in env::vars_os() {
			let Some(variable) = variable.to_str() else {
				continue;
			};

			let Some(key) = variable.strip_prefix("ORD_") else {
				continue;
			};

			env.insert(
				key.into(),
				value.into_string().map_err(|value| SnafuError::EnvVarUnicode {
					backtrace: Backtrace::capture(),
					value,
					variable: variable.into(),
				})?,
			);
		}

		Ok(self.subcommand.run(Settings::load(self.options)?)?)
	}
}
