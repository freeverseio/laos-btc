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

mod export;
pub mod info;
mod update;

#[derive(Debug, Parser)]
pub(crate) enum IndexSubcommand {
	#[command(about = "Write inscription numbers and ids to a tab-separated file")]
	Export(export::Export),
	#[command(about = "Print index statistics")]
	Info(info::Info),
	#[command(about = "Update the index", alias = "run")]
	Update,
}

impl IndexSubcommand {
	pub(crate) fn run(self, settings: Settings) -> SubcommandResult {
		match self {
			Self::Export(export) => export.run(settings),
			Self::Info(info) => info.run(settings),
			Self::Update => update::run(settings),
		}
	}
}
