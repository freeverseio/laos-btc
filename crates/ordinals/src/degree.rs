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
pub struct Degree {
	pub hour: u32,
	pub minute: u32,
	pub second: u32,
	pub third: u64,
}

impl Display for Degree {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}°{}′{}″{}‴", self.hour, self.minute, self.second, self.third)
	}
}

impl From<Sat> for Degree {
	fn from(sat: Sat) -> Self {
		let height = sat.height().n();
		Degree {
			hour: height / (CYCLE_EPOCHS * SUBSIDY_HALVING_INTERVAL),
			minute: height % SUBSIDY_HALVING_INTERVAL,
			second: height % DIFFCHANGE_INTERVAL,
			third: sat.third(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn case(sat: u64, hour: u32, minute: u32, second: u32, third: u64) {
		assert_eq!(Degree::from(Sat(sat)), Degree { hour, minute, second, third });
	}

	#[test]
	fn from() {
		case(0, 0, 0, 0, 0);
		case(1, 0, 0, 0, 1);
		case(5_000_000_000, 0, 1, 1, 0);
		case(5_000_000_000 * u64::from(DIFFCHANGE_INTERVAL), 0, DIFFCHANGE_INTERVAL, 0, 0);
		case(5_000_000_000 * u64::from(SUBSIDY_HALVING_INTERVAL), 0, 0, 336, 0);
		case(
			(5_000_000_000 +
				2_500_000_000 +
				1_250_000_000 +
				625_000_000 + 312_500_000 +
				156_250_000) * u64::from(SUBSIDY_HALVING_INTERVAL),
			1,
			0,
			0,
			0,
		);
	}
}
