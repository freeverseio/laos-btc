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

pub struct DeserializeFromStr<T: FromStr>(pub T);

impl<'de, T: FromStr> Deserialize<'de> for DeserializeFromStr<T>
where
	T::Err: Display,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(Self(
			FromStr::from_str(&String::deserialize(deserializer)?)
				.map_err(serde::de::Error::custom)?,
		))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deserialize_from_str() {
		assert_eq!(serde_json::from_str::<DeserializeFromStr<u64>>("\"1\"").unwrap().0, 1,);
	}
}
