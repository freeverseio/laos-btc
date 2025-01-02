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

#[derive(Debug, PartialEq, Clone, DeserializeFromStr)]
pub(crate) enum Signer {
	Address(Address<NetworkUnchecked>),
	Inscription(InscriptionId),
	Output(OutPoint),
}

impl FromStr for Signer {
	type Err = SnafuError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		if re::ADDRESS.is_match(input) {
			Ok(Signer::Address(input.parse().snafu_context(error::AddressParse { input })?))
		} else if re::OUTPOINT.is_match(input) {
			Ok(Signer::Output(input.parse().snafu_context(error::OutPointParse { input })?))
		} else if re::INSCRIPTION_ID.is_match(input) {
			Ok(Signer::Inscription(
				input.parse().snafu_context(error::InscriptionIdParse { input })?,
			))
		} else {
			Err(SnafuError::SignerParse { input: input.to_string() })
		}
	}
}
