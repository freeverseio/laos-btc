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
use std::{
	cmp::{PartialEq, PartialOrd},
	ops::{Add, AddAssign, Div, Rem, Sub, SubAssign},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct Lot(pub u128);

impl Lot {
	#[cfg(test)]
	const MAX: Self = Self(u128::MAX);

	pub(super) fn n(self) -> u128 {
		self.0
	}

	fn checked_add(self, rhs: Self) -> Option<Self> {
		Some(Self(self.0.checked_add(rhs.0)?))
	}

	fn checked_sub(self, rhs: Self) -> Option<Self> {
		Some(Self(self.0.checked_sub(rhs.0)?))
	}
}

impl TryFrom<Lot> for usize {
	type Error = <usize as TryFrom<u128>>::Error;
	fn try_from(lot: Lot) -> Result<Self, Self::Error> {
		usize::try_from(lot.0)
	}
}

impl Add for Lot {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		self.checked_add(other).expect("lot overflow")
	}
}

impl AddAssign for Lot {
	fn add_assign(&mut self, other: Self) {
		*self = *self + other;
	}
}

impl Add<u128> for Lot {
	type Output = Self;
	fn add(self, other: u128) -> Self::Output {
		self + Lot(other)
	}
}

impl AddAssign<u128> for Lot {
	fn add_assign(&mut self, other: u128) {
		*self += Lot(other);
	}
}

impl Sub for Lot {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		self.checked_sub(other).expect("lot underflow")
	}
}

impl SubAssign for Lot {
	fn sub_assign(&mut self, other: Self) {
		*self = *self - other;
	}
}

impl Div<u128> for Lot {
	type Output = Self;
	fn div(self, other: u128) -> Self::Output {
		Lot(self.0 / other)
	}
}

impl Rem<u128> for Lot {
	type Output = Self;
	fn rem(self, other: u128) -> Self::Output {
		Lot(self.0 % other)
	}
}

impl PartialEq<u128> for Lot {
	fn eq(&self, other: &u128) -> bool {
		self.0 == *other
	}
}

impl PartialOrd<u128> for Lot {
	fn partial_cmp(&self, other: &u128) -> Option<std::cmp::Ordering> {
		self.0.partial_cmp(other)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic(expected = "lot overflow")]
	fn add() {
		let _ = Lot::MAX + 1;
	}

	#[test]
	#[should_panic(expected = "lot overflow")]
	fn add_assign() {
		let mut l = Lot::MAX;
		l += Lot(1);
	}

	#[test]
	#[should_panic(expected = "lot overflow")]
	fn add_u128() {
		let _ = Lot::MAX + 1;
	}

	#[test]
	#[should_panic(expected = "lot overflow")]
	fn add_assign_u128() {
		let mut l = Lot::MAX;
		l += 1;
	}

	#[test]
	#[should_panic(expected = "lot underflow")]
	fn sub() {
		let _ = Lot(0) - Lot(1);
	}

	#[test]
	#[should_panic(expected = "lot underflow")]
	fn sub_assign() {
		let mut l = Lot(0);
		l -= Lot(1);
	}

	#[test]
	fn div() {
		assert_eq!(Lot(100) / 2, Lot(50));
	}

	#[test]
	fn rem() {
		assert_eq!(Lot(77) % 8, Lot(5));
	}

	#[test]
	fn partial_eq() {
		assert_eq!(Lot(100), 100);
	}

	#[test]
	fn partial_ord() {
		assert!(Lot(100) > 10);
	}
}
