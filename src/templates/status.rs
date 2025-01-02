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

#[derive(Boilerplate, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusHtml {
	pub address_index: bool,
	pub blessed_inscriptions: u64,
	pub chain: Chain,
	pub cursed_inscriptions: u64,
	pub height: Option<u32>,
	pub initial_sync_time: Duration,
	pub inscription_index: bool,
	pub inscriptions: u64,
	pub json_api: bool,
	pub lost_sats: u64,
	pub minimum_rune_for_next_block: Rune,
	pub rune_index: bool,
	pub runes: u64,
	pub sat_index: bool,
	pub started: DateTime<Utc>,
	pub transaction_index: bool,
	pub unrecoverably_reorged: bool,
	pub uptime: Duration,
}

impl PageContent for StatusHtml {
	fn title(&self) -> String {
		"Status".into()
	}
}
