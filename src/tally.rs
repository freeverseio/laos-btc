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

pub(crate) trait Tally {
	fn tally(self, count: usize) -> Tallied;
}

impl Tally for &'static str {
	fn tally(self, count: usize) -> Tallied {
		Tallied { noun: self, count }
	}
}

pub(crate) struct Tallied {
	count: usize,
	noun: &'static str,
}

impl Display for Tallied {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if self.count == 1 {
			write!(f, "{} {}", self.count, self.noun)
		} else {
			write!(f, "{} {}s", self.count, self.noun)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn zero() {
		assert_eq!("foo".tally(0).to_string(), "0 foos")
	}

	#[test]
	fn one() {
		assert_eq!("foo".tally(1).to_string(), "1 foo")
	}

	#[test]
	fn two() {
		assert_eq!("foo".tally(2).to_string(), "2 foos")
	}
}
