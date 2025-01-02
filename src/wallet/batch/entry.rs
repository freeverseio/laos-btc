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

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, Default, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Entry {
	pub file: Option<PathBuf>,
	pub delegate: Option<InscriptionId>,
	pub satpoint: Option<SatPoint>,
	pub destination: Option<Address<NetworkUnchecked>>,
	pub metadata: Option<serde_yaml::Value>,
	pub metaprotocol: Option<String>,
}

impl Entry {
	pub(crate) fn metadata(&self) -> Result<Option<Vec<u8>>> {
		Ok(match &self.metadata {
			None => None,
			Some(metadata) => {
				let mut cbor = Vec::new();
				ciborium::into_writer(&metadata, &mut cbor)?;
				Some(cbor)
			},
		})
	}
}
