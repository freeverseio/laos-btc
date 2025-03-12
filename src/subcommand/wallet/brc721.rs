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
use crate::wallet_brc721::WalletBrc721;

pub mod register_collection;
pub mod register_ownership;

#[derive(Debug, Parser)]
pub(crate) struct Brc721Command {
	#[command(subcommand)]
	pub(crate) subcommand: Subcommand,
}

#[derive(Debug, Parser)]
#[allow(clippy::large_enum_variant)]
pub(crate) enum Subcommand {
	#[command(about = "Register Collection", visible_alias = "rc")]
	RegisterCollection(register_collection::RegisterCollectionCmd),
	#[command(about = "Register Ownership", visible_alias = "ro")]
	RegisterOwnership(register_ownership::RegisterOwnershipCmd),
}

impl Brc721Command {
	pub(crate) fn run(self, wallet: WalletBrc721) -> SubcommandResult {
		match self.subcommand {
			Subcommand::RegisterCollection(register) => register.run(wallet),
			Subcommand::RegisterOwnership(cmd) => cmd.run(wallet),
		}
	}
}
