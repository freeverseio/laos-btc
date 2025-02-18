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

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
	InscriptionCreated {
		block_height: u32,
		charms: u16,
		inscription_id: InscriptionId,
		location: Option<SatPoint>,
		parent_inscription_ids: Vec<InscriptionId>,
		sequence_number: u32,
	},
	InscriptionTransferred {
		block_height: u32,
		inscription_id: InscriptionId,
		new_location: SatPoint,
		old_location: SatPoint,
		sequence_number: u32,
	},
	LaosCollectionCreated {
		collection_id: RuneId,
		txid: Txid,
	},
	RuneBurned {
		amount: u128,
		block_height: u32,
		rune_id: RuneId,
		txid: Txid,
	},
	RuneEtched {
		block_height: u32,
		rune_id: RuneId,
		txid: Txid,
	},
	RuneMinted {
		amount: u128,
		block_height: u32,
		rune_id: RuneId,
		txid: Txid,
	},
	RuneTransferred {
		amount: u128,
		block_height: u32,
		outpoint: OutPoint,
		rune_id: RuneId,
		txid: Txid,
	},
}
