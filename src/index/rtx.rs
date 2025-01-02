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

pub(crate) struct Rtx(pub(crate) redb::ReadTransaction);

impl Rtx {
	pub(crate) fn block_height(&self) -> Result<Option<Height>> {
		Ok(self
			.0
			.open_table(HEIGHT_TO_BLOCK_HEADER)?
			.range(0..)?
			.next_back()
			.transpose()?
			.map(|(height, _header)| Height(height.value())))
	}

	pub(crate) fn block_count(&self) -> Result<u32> {
		Ok(self
			.0
			.open_table(HEIGHT_TO_BLOCK_HEADER)?
			.range(0..)?
			.next_back()
			.transpose()?
			.map(|(height, _header)| height.value() + 1)
			.unwrap_or(0))
	}

	pub(crate) fn block_hash(&self, height: Option<u32>) -> Result<Option<BlockHash>> {
		let height_to_block_header = self.0.open_table(HEIGHT_TO_BLOCK_HEADER)?;

		Ok(match height {
			Some(height) => height_to_block_header.get(height)?,
			None => height_to_block_header
				.range(0..)?
				.next_back()
				.transpose()?
				.map(|(_height, header)| header),
		}
		.map(|header| Header::load(*header.value()).block_hash()))
	}
}
