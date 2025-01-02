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

pub(super) enum Flag {
	Etching = 0,
	Terms = 1,
	Turbo = 2,
	#[allow(unused)]
	Cenotaph = 127,
}

impl Flag {
	pub(super) fn mask(self) -> u128 {
		1 << self as u128
	}

	pub(super) fn take(self, flags: &mut u128) -> bool {
		let mask = self.mask();
		let set = *flags & mask != 0;
		*flags &= !mask;
		set
	}

	pub(super) fn set(self, flags: &mut u128) {
		*flags |= self.mask()
	}
}

impl From<Flag> for u128 {
	fn from(flag: Flag) -> Self {
		flag.mask()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn mask() {
		assert_eq!(Flag::Etching.mask(), 0b1);
		assert_eq!(Flag::Cenotaph.mask(), 1 << 127);
	}

	#[test]
	fn take() {
		let mut flags = 1;
		assert!(Flag::Etching.take(&mut flags));
		assert_eq!(flags, 0);

		let mut flags = 0;
		assert!(!Flag::Etching.take(&mut flags));
		assert_eq!(flags, 0);
	}

	#[test]
	fn set() {
		let mut flags = 0;
		Flag::Etching.set(&mut flags);
		assert_eq!(flags, 1);
	}
}
