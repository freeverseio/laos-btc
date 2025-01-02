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

pub(crate) struct AcceptJson(pub(crate) bool);

#[async_trait::async_trait]
impl<S> axum::extract::FromRequestParts<S> for AcceptJson
where
	Arc<ServerConfig>: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(
		parts: &mut http::request::Parts,
		state: &S,
	) -> Result<Self, Self::Rejection> {
		let state = Arc::from_ref(state);
		let json_api_enabled = state.json_api_enabled;
		let json_header = parts
			.headers
			.get("accept")
			.map(|value| value == "application/json")
			.unwrap_or_default();
		if json_header && json_api_enabled {
			Ok(Self(true))
		} else if json_header && !json_api_enabled {
			Err((StatusCode::NOT_ACCEPTABLE, "JSON API disabled"))
		} else {
			Ok(Self(false))
		}
	}
}
