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
use regex::RegexSet;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Representation {
	Address,
	Decimal,
	Degree,
	Hash,
	InscriptionId,
	Integer,
	Name,
	OutPoint,
	Percentile,
	Rune,
	SatPoint,
}

impl Representation {
	const fn pattern(self) -> (Self, &'static str) {
		(
			self,
			match self {
				Self::Address => r"^(bc|BC|tb|TB|bcrt|BCRT)1.*$",
				Self::Decimal => r"^.*\..*$",
				Self::Degree => r"^.*°.*′.*″(.*‴)?$",
				Self::Hash => r"^[[:xdigit:]]{64}$",
				Self::InscriptionId => r"^[[:xdigit:]]{64}i\d+$",
				Self::Integer => r"^[0-9]*$",
				Self::Name => r"^[a-z]{1,11}$",
				Self::OutPoint => r"^[[:xdigit:]]{64}:\d+$",
				Self::Percentile => r"^.*%$",
				Self::Rune => r"^[A-Z•.]+$",
				Self::SatPoint => r"^[[:xdigit:]]{64}:\d+:\d+$",
			},
		)
	}
}

impl FromStr for Representation {
	type Err = SnafuError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		if let Some(i) = REGEX_SET.matches(input).into_iter().next() {
			Ok(PATTERNS[i].0)
		} else {
			Err(error::UnrecognizedRepresentation { input }.build())
		}
	}
}

const PATTERNS: &[(Representation, &str)] = &[
	Representation::Address.pattern(),
	Representation::Decimal.pattern(),
	Representation::Degree.pattern(),
	Representation::Hash.pattern(),
	Representation::InscriptionId.pattern(),
	Representation::Integer.pattern(),
	Representation::Name.pattern(),
	Representation::OutPoint.pattern(),
	Representation::Percentile.pattern(),
	Representation::Rune.pattern(),
	Representation::SatPoint.pattern(),
];

lazy_static! {
	static ref REGEX_SET: RegexSet =
		RegexSet::new(PATTERNS.iter().map(|(_representation, pattern)| pattern),).unwrap();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn all_patterns_are_anchored() {
		assert!(PATTERNS
			.iter()
			.all(|(_representation, pattern)| pattern.starts_with('^') && pattern.ends_with('$')));
	}
}
