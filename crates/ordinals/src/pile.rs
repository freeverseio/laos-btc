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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub struct Pile {
	pub amount: u128,
	pub divisibility: u8,
	pub symbol: Option<char>,
}

impl Display for Pile {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let cutoff = 10u128.checked_pow(self.divisibility.into()).unwrap();

		let whole = self.amount / cutoff;
		let mut fractional = self.amount % cutoff;

		if fractional == 0 {
			write!(f, "{whole}")?;
		} else {
			let mut width = usize::from(self.divisibility);
			while fractional % 10 == 0 {
				fractional /= 10;
				width -= 1;
			}

			write!(f, "{whole}.{fractional:0>width$}")?;
		}

		write!(f, "\u{A0}{}", self.symbol.unwrap_or('¤'))?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn display() {
		assert_eq!(Pile { amount: 0, divisibility: 0, symbol: None }.to_string(), "0\u{A0}¤");
		assert_eq!(Pile { amount: 25, divisibility: 0, symbol: None }.to_string(), "25\u{A0}¤");
		assert_eq!(Pile { amount: 0, divisibility: 1, symbol: None }.to_string(), "0\u{A0}¤");
		assert_eq!(Pile { amount: 1, divisibility: 1, symbol: None }.to_string(), "0.1\u{A0}¤");
		assert_eq!(Pile { amount: 1, divisibility: 2, symbol: None }.to_string(), "0.01\u{A0}¤");
		assert_eq!(Pile { amount: 10, divisibility: 2, symbol: None }.to_string(), "0.1\u{A0}¤");
		assert_eq!(Pile { amount: 1100, divisibility: 3, symbol: None }.to_string(), "1.1\u{A0}¤");
		assert_eq!(Pile { amount: 100, divisibility: 2, symbol: None }.to_string(), "1\u{A0}¤");
		assert_eq!(Pile { amount: 101, divisibility: 2, symbol: None }.to_string(), "1.01\u{A0}¤");
		assert_eq!(
			Pile { amount: u128::MAX, divisibility: 18, symbol: None }.to_string(),
			"340282366920938463463.374607431768211455\u{A0}¤"
		);
		assert_eq!(
			Pile { amount: u128::MAX, divisibility: 38, symbol: None }.to_string(),
			"3.40282366920938463463374607431768211455\u{A0}¤"
		);
		assert_eq!(Pile { amount: 0, divisibility: 0, symbol: Some('$') }.to_string(), "0\u{A0}$");
	}
}
