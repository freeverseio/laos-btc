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

//! Types for interoperating with ordinals, inscriptions, and runes.
#![allow(clippy::large_enum_variant)]

use bitcoin::{
	consensus::{Decodable, Encodable},
	constants::{DIFFCHANGE_INTERVAL, SUBSIDY_HALVING_INTERVAL},
	opcodes,
	script::{self, Instruction},
	Network, OutPoint, ScriptBuf, Transaction,
};
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
	cmp,
	collections::{HashMap, VecDeque},
	fmt::{self, Formatter},
	num::ParseIntError,
	ops::{Add, AddAssign, Sub},
};
use thiserror::Error;

pub use artifact::Artifact;
pub use brc721::{
	collection_id::Brc721CollectionId,
	register_collection::{
		RegisterCollection, RegisterCollectionPayload, COLLECTION_ADDRESS_LENGTH,
	},
};
pub use cenotaph::Cenotaph;
pub use charm::Charm;
pub use decimal_sat::DecimalSat;
pub use degree::Degree;
pub use edict::Edict;
pub use epoch::Epoch;
pub use etching::Etching;
pub use flaw::Flaw;
pub use height::Height;
pub use pile::Pile;
pub use rarity::Rarity;
pub use rune::Rune;
pub use rune_id::RuneId;
pub use runestone::Runestone;
pub use sat::Sat;
pub use sat_point::SatPoint;
pub use spaced_rune::SpacedRune;
pub use terms::Terms;

pub const COIN_VALUE: u64 = 100_000_000;
pub const CYCLE_EPOCHS: u32 = 6;

fn default<T: Default>() -> T {
	Default::default()
}

mod artifact;
pub mod brc721;
mod cenotaph;
mod charm;
mod decimal_sat;
mod degree;
mod edict;
mod epoch;
mod etching;
mod flaw;
mod height;
mod pile;
mod rarity;
mod rune;
mod rune_id;
mod runestone;
pub mod sat;
pub mod sat_point;
pub mod spaced_rune;
mod terms;
pub mod varint;
