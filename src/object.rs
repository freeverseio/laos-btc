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

#[derive(Debug, PartialEq, Clone, DeserializeFromStr, SerializeDisplay)]
pub enum Object {
	Address(Address<NetworkUnchecked>),
	Hash([u8; 32]),
	InscriptionId(InscriptionId),
	Integer(u128),
	OutPoint(OutPoint),
	Rune(SpacedRune),
	Sat(Sat),
	SatPoint(SatPoint),
}

impl FromStr for Object {
	type Err = SnafuError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		use Representation::*;

		match input.parse::<Representation>()? {
			Address =>
				Ok(Self::Address(input.parse().snafu_context(error::AddressParse { input })?)),
			Decimal | Degree | Percentile | Name =>
				Ok(Self::Sat(input.parse().snafu_context(error::SatParse { input })?)),
			Hash => Ok(Self::Hash(
				bitcoin::hashes::sha256::Hash::from_str(input)
					.snafu_context(error::HashParse { input })?
					.to_byte_array(),
			)),
			InscriptionId => Ok(Self::InscriptionId(
				input.parse().snafu_context(error::InscriptionIdParse { input })?,
			)),
			Integer =>
				Ok(Self::Integer(input.parse().snafu_context(error::IntegerParse { input })?)),
			OutPoint =>
				Ok(Self::OutPoint(input.parse().snafu_context(error::OutPointParse { input })?)),
			Rune => Ok(Self::Rune(input.parse().snafu_context(error::RuneParse { input })?)),
			SatPoint =>
				Ok(Self::SatPoint(input.parse().snafu_context(error::SatPointParse { input })?)),
		}
	}
}

impl Display for Object {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Self::Address(address) => write!(f, "{}", address.clone().assume_checked()),
			Self::Hash(hash) => {
				for byte in hash {
					write!(f, "{byte:02x}")?;
				}
				Ok(())
			},
			Self::InscriptionId(inscription_id) => write!(f, "{inscription_id}"),
			Self::Integer(integer) => write!(f, "{integer}"),
			Self::OutPoint(outpoint) => write!(f, "{outpoint}"),
			Self::Rune(rune) => write!(f, "{rune}"),
			Self::Sat(sat) => write!(f, "{sat}"),
			Self::SatPoint(satpoint) => write!(f, "{satpoint}"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_str() {
		#[track_caller]
		fn case(s: &str, expected: Object) {
			let actual = s.parse::<Object>().unwrap();
			assert_eq!(actual, expected);
			let round_trip = actual.to_string().parse::<Object>().unwrap();
			assert_eq!(round_trip, expected);
		}

		assert_eq!("nvtdijuwxlp".parse::<Object>().unwrap(), Object::Sat(Sat(0)));
		assert_eq!("a".parse::<Object>().unwrap(), Object::Sat(Sat::LAST));
		assert_eq!("1.1".parse::<Object>().unwrap(), Object::Sat(Sat(50 * COIN_VALUE + 1)));
		assert_eq!("1°0′0″0‴".parse::<Object>().unwrap(), Object::Sat(Sat(2067187500000000)));
		assert_eq!("0%".parse::<Object>().unwrap(), Object::Sat(Sat(0)));

		case("0", Object::Integer(0));

		case(
			"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefi1",
			Object::InscriptionId(
				"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdefi1"
					.parse()
					.unwrap(),
			),
		);

		case(
			"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
			Object::Hash([
				0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab,
				0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67,
				0x89, 0xab, 0xcd, 0xef,
			]),
		);
		case(
			"bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4",
			Object::Address("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4".parse().unwrap()),
		);
		case(
			"BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4",
			Object::Address("BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4".parse().unwrap()),
		);
		case(
			"tb1qqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesrxh6hy",
			Object::Address(
				"tb1qqqqqp399et2xygdj5xreqhjjvcmzhxw4aywxecjdzew6hylgvsesrxh6hy"
					.parse()
					.unwrap(),
			),
		);
		case(
			"TB1QQQQQP399ET2XYGDJ5XREQHJJVCMZHXW4AYWXECJDZEW6HYLGVSESRXH6HY",
			Object::Address(
				"TB1QQQQQP399ET2XYGDJ5XREQHJJVCMZHXW4AYWXECJDZEW6HYLGVSESRXH6HY"
					.parse()
					.unwrap(),
			),
		);
		case(
			"bcrt1qs758ursh4q9z627kt3pp5yysm78ddny6txaqgw",
			Object::Address("bcrt1qs758ursh4q9z627kt3pp5yysm78ddny6txaqgw".parse().unwrap()),
		);
		case(
			"BCRT1QS758URSH4Q9Z627KT3PP5YYSM78DDNY6TXAQGW",
			Object::Address("BCRT1QS758URSH4Q9Z627KT3PP5YYSM78DDNY6TXAQGW".parse().unwrap()),
		);
		case(
			"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123",
			Object::OutPoint(
				"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123"
					.parse()
					.unwrap(),
			),
		);
		case(
			"0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123",
			Object::OutPoint(
				"0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123"
					.parse()
					.unwrap(),
			),
		);
		case(
			"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123:456",
			Object::SatPoint(
				"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef:123:456"
					.parse()
					.unwrap(),
			),
		);
		case(
			"0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123:456",
			Object::SatPoint(
				"0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF:123:456"
					.parse()
					.unwrap(),
			),
		);
		case("A", Object::Rune(SpacedRune { rune: Rune(0), spacers: 0 }));
		case("A•A", Object::Rune(SpacedRune { rune: Rune(26), spacers: 1 }));
	}
}
