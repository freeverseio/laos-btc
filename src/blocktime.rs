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

#[derive(Copy, Clone)]
pub enum Blocktime {
	Confirmed(DateTime<Utc>),
	Expected(DateTime<Utc>),
}

impl Blocktime {
	pub(crate) fn confirmed(seconds: u32) -> Self {
		Self::Confirmed(timestamp(seconds.into()))
	}

	pub(crate) fn timestamp(self) -> DateTime<Utc> {
		match self {
			Self::Confirmed(timestamp) | Self::Expected(timestamp) => timestamp,
		}
	}

	pub(crate) fn unix_timestamp(self) -> i64 {
		match self {
			Self::Confirmed(timestamp) | Self::Expected(timestamp) => timestamp.timestamp(),
		}
	}

	pub(crate) fn suffix(self) -> &'static str {
		match self {
			Self::Confirmed(_) => "",
			Self::Expected(_) => " (expected)",
		}
	}
}
