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
use axum::http::HeaderName;

#[derive(Default)]
pub struct ServerConfig {
	pub chain: Chain,
	pub csp_origin: Option<String>,
	pub decompress: bool,
	pub domain: Option<String>,
	pub index_sats: bool,
	pub json_api_enabled: bool,
	pub proxy: Option<Url>,
}

impl ServerConfig {
	pub(super) fn preview_content_security_policy(
		&self,
		media: Media,
	) -> ServerResult<[(HeaderName, HeaderValue); 1]> {
		let default = match media {
			Media::Audio => "default-src 'self'",
			Media::Code(_) => "script-src-elem 'self' https://cdn.jsdelivr.net",
			Media::Font => "script-src-elem 'self'; style-src 'self' 'unsafe-inline'",
			Media::Iframe =>
				return Err(anyhow!(
					"preview_content_security_policy cannot be called with Media::Iframe"
				)
				.into()),
			Media::Image(_) => "default-src 'self' 'unsafe-inline'",
			Media::Markdown => "script-src-elem 'self' https://cdn.jsdelivr.net",
			Media::Model => "script-src-elem 'self' https://ajax.googleapis.com",
			Media::Pdf => "script-src-elem 'self' https://cdn.jsdelivr.net",
			Media::Text => "default-src 'self'",
			Media::Unknown => "default-src 'self'",
			Media::Video => "default-src 'self'",
		};

		let value = if let Some(csp_origin) = &self.csp_origin {
			default.replace("'self'", csp_origin).parse().map_err(|err| {
				anyhow!("invalid content-security-policy origin `{csp_origin}`: {err}")
			})?
		} else {
			HeaderValue::from_static(default)
		};

		Ok([(header::CONTENT_SECURITY_POLICY, value)])
	}
}
