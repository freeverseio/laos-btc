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

pub(crate) trait IntoUsize {
	fn into_usize(self) -> usize;
}

impl IntoUsize for u32 {
	fn into_usize(self) -> usize {
		self.try_into().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn into_usize() {
		u32::MAX.into_usize();
	}
}
