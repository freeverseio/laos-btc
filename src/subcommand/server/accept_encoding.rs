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
use axum::extract::FromRef;

#[derive(Default, Debug)]
pub(crate) struct AcceptEncoding(pub(crate) Option<String>);

#[async_trait::async_trait]
impl<S> axum::extract::FromRequestParts<S> for AcceptEncoding
where
	Arc<ServerConfig>: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(
		parts: &mut http::request::Parts,
		_state: &S,
	) -> Result<Self, Self::Rejection> {
		Ok(Self(
			parts
				.headers
				.get("accept-encoding")
				.map(|value| value.to_str().unwrap_or_default().to_owned()),
		))
	}
}

impl AcceptEncoding {
	pub(crate) fn is_acceptable(&self, encoding: &HeaderValue) -> bool {
		let Ok(encoding) = encoding.to_str() else {
			return false;
		};

		self.0
			.clone()
			.unwrap_or_default()
			.split(',')
			.any(|value| value.split(';').next().unwrap_or_default().trim() == encoding)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::{extract::FromRequestParts, http::Request};
	use http::header::ACCEPT_ENCODING;

	#[tokio::test]
	async fn single_encoding() {
		let req = Request::builder().header(ACCEPT_ENCODING, "gzip").body(()).unwrap();

		let encodings = AcceptEncoding::from_request_parts(
			&mut req.into_parts().0,
			&Arc::new(ServerConfig { json_api_enabled: false, decompress: false, ..default() }),
		)
		.await
		.unwrap();

		assert_eq!(encodings.0, Some("gzip".to_string()));
	}

	#[tokio::test]
	async fn accepts_encoding_with_qvalues() {
		let req = Request::builder()
			.header(ACCEPT_ENCODING, "deflate;q=0.5, gzip;q=1.0, br;q=0.8")
			.body(())
			.unwrap();

		let encodings = AcceptEncoding::from_request_parts(
			&mut req.into_parts().0,
			&Arc::new(ServerConfig { json_api_enabled: false, decompress: false, ..default() }),
		)
		.await
		.unwrap();

		assert_eq!(encodings.0, Some("deflate;q=0.5, gzip;q=1.0, br;q=0.8".to_string()));

		assert!(encodings.is_acceptable(&HeaderValue::from_static("deflate")));
		assert!(encodings.is_acceptable(&HeaderValue::from_static("gzip")));
		assert!(encodings.is_acceptable(&HeaderValue::from_static("br")));
		assert!(!encodings.is_acceptable(&HeaderValue::from_static("bzip2")));
	}

	#[tokio::test]
	async fn accepts_encoding_without_qvalues() {
		let req = Request::builder()
			.header(ACCEPT_ENCODING, "gzip, deflate, br")
			.body(())
			.unwrap();

		let encodings = AcceptEncoding::from_request_parts(
			&mut req.into_parts().0,
			&Arc::new(ServerConfig { json_api_enabled: false, decompress: false, ..default() }),
		)
		.await
		.unwrap();

		assert_eq!(encodings.0, Some("gzip, deflate, br".to_string()));

		assert!(encodings.is_acceptable(&HeaderValue::from_static("deflate")));
		assert!(encodings.is_acceptable(&HeaderValue::from_static("gzip")));
		assert!(encodings.is_acceptable(&HeaderValue::from_static("br")));
		assert!(!encodings.is_acceptable(&HeaderValue::from_static("bzip2")));
	}
}
