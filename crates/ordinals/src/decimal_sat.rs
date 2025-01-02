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

#[derive(PartialEq, Debug)]
pub struct DecimalSat {
	pub height: Height,
	pub offset: u64,
}

impl From<Sat> for DecimalSat {
	fn from(sat: Sat) -> Self {
		Self { height: sat.height(), offset: sat.third() }
	}
}

impl Display for DecimalSat {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}.{}", self.height, self.offset)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decimal() {
		assert_eq!(Sat(0).decimal(), DecimalSat { height: Height(0), offset: 0 });
		assert_eq!(Sat(1).decimal(), DecimalSat { height: Height(0), offset: 1 });
		assert_eq!(
			Sat(2099999997689999).decimal(),
			DecimalSat { height: Height(6929999), offset: 0 }
		);
	}
}
