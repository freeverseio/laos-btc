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
use std::fmt::Write;

#[derive(Debug)]
pub(super) enum ServerError {
	BadRequest(String),
	Internal(Error),
	NotAcceptable { accept_encoding: AcceptEncoding, content_encoding: HeaderValue },
	NotFound(String),
}

pub(super) type ServerResult<T = Response> = Result<T, ServerError>;

impl IntoResponse for ServerError {
	fn into_response(self) -> Response {
		match self {
			Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
			Self::Internal(error) => {
				eprintln!("error serving request: {error}");
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					StatusCode::INTERNAL_SERVER_ERROR.canonical_reason().unwrap_or_default(),
				)
					.into_response()
			},
			Self::NotAcceptable { accept_encoding, content_encoding } => {
				let mut message = format!(
					"inscription content encoding `{}` is not acceptable.",
					String::from_utf8_lossy(content_encoding.as_bytes())
				);

				if let Some(accept_encoding) = accept_encoding.0 {
					write!(message, " `Accept-Encoding` header: `{accept_encoding}`").unwrap();
				} else {
					write!(message, " `Accept-Encoding` header not present").unwrap();
				};

				(StatusCode::NOT_ACCEPTABLE, message).into_response()
			},
			Self::NotFound(message) => (
				StatusCode::NOT_FOUND,
				[(header::CACHE_CONTROL, HeaderValue::from_static("no-store"))],
				message,
			)
				.into_response(),
		}
	}
}

pub(super) trait OptionExt<T> {
	fn ok_or_not_found<F: FnOnce() -> S, S: Into<String>>(self, f: F) -> ServerResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
	fn ok_or_not_found<F: FnOnce() -> S, S: Into<String>>(self, f: F) -> ServerResult<T> {
		match self {
			Some(value) => Ok(value),
			None => Err(ServerError::NotFound(f().into() + " not found")),
		}
	}
}

impl From<Error> for ServerError {
	fn from(error: Error) -> Self {
		Self::Internal(error)
	}
}
