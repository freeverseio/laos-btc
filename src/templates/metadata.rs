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

pub(crate) struct MetadataHtml<'a>(pub &'a Value);

impl Display for MetadataHtml<'_> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self.0 {
			Value::Array(x) => {
				write!(f, "<ul>")?;
				for element in x {
					write!(f, "<li>{}</li>", MetadataHtml(element))?;
				}
				write!(f, "</ul>")
			},
			Value::Bool(x) => write!(f, "{x}"),
			Value::Bytes(x) => {
				for byte in x {
					write!(f, "{byte:02X}")?;
				}
				Ok(())
			},
			Value::Float(x) => write!(f, "{x}"),
			Value::Integer(x) => write!(f, "{}", i128::from(*x)),
			Value::Map(x) => {
				write!(f, "<dl>")?;
				for (key, value) in x {
					write!(f, "<dt>{}</dt>", MetadataHtml(key))?;
					write!(f, "<dd>{}</dd>", MetadataHtml(value))?;
				}
				write!(f, "</dl>")
			},
			Value::Null => write!(f, "null"),
			Value::Tag(tag, value) => write!(f, "<sup>{tag}</sup>{}", MetadataHtml(value)),
			Value::Text(x) => x.escape(f, false),
			_ => write!(f, "unknown"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn null() {
		assert_eq!(MetadataHtml(&Value::Null).to_string(), "null");
	}

	#[test]
	fn integer() {
		assert_eq!(MetadataHtml(&Value::Integer(100.into())).to_string(), "100");
	}

	#[test]
	fn bool() {
		assert_eq!(MetadataHtml(&Value::Bool(false)).to_string(), "false");
		assert_eq!(MetadataHtml(&Value::Bool(true)).to_string(), "true");
	}

	#[test]
	fn tag() {
		assert_eq!(
			MetadataHtml(&Value::Tag(0, Box::new(Value::Bool(false)))).to_string(),
			"<sup>0</sup>false"
		);
	}

	#[test]
	fn string() {
		assert_eq!(MetadataHtml(&Value::Text("hello".into())).to_string(), "hello");
		assert_eq!(MetadataHtml(&Value::Text("<".into())).to_string(), "&lt;");
	}

	#[test]
	fn bytes() {
		assert_eq!(MetadataHtml(&Value::Bytes(vec![0, 1, 2, 0xFF])).to_string(), "000102FF");
	}

	#[test]
	fn float() {
		assert_eq!(MetadataHtml(&Value::Float(0.5)).to_string(), "0.5");
	}

	#[test]
	fn array() {
		assert_eq!(
			MetadataHtml(&Value::Array(vec![
				Value::Null,
				Value::Null,
				Value::Text("hello".to_string())
			]))
			.to_string(),
			"<ul><li>null</li><li>null</li><li>hello</li></ul>"
		)
	}

	#[test]
	fn map() {
		assert_eq!(
			MetadataHtml(&Value::Map(
				vec![
					(Value::Text("b".to_string()), Value::Null),
					(Value::Text("a".to_string()), Value::Text("hello".to_string()))
				]
				.into_iter()
				.collect()
			))
			.to_string(),
			"<dl><dt>b</dt><dd>null</dd><dt>a</dt><dd>hello</dd></dl>"
		);
		assert_eq!(
			MetadataHtml(&Value::Map(
				vec![(Value::Text("<".to_string()), Value::Null),].into_iter().collect()
			))
			.to_string(),
			"<dl><dt>&lt;</dt><dd>null</dd></dl>"
		);
	}
}
