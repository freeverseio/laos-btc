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
use serde_hex::{SerHex, Strict};

pub use crate::{
	subcommand::decode::RawOutput as Decode,
	templates::{
		BlocksHtml as Blocks, RuneHtml as Rune, RunesHtml as Runes, StatusHtml as Status,
		TransactionHtml as Transaction,
	},
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
	pub best_height: u32,
	pub hash: BlockHash,
	pub height: u32,
	pub inscriptions: Vec<InscriptionId>,
	pub runes: Vec<SpacedRune>,
	pub target: BlockHash,
	pub transactions: Vec<bitcoin::blockdata::transaction::Transaction>,
}

impl Block {
	pub(crate) fn new(
		block: bitcoin::Block,
		height: Height,
		best_height: Height,
		inscriptions: Vec<InscriptionId>,
		runes: Vec<SpacedRune>,
	) -> Self {
		Self {
			hash: block.header.block_hash(),
			target: target_as_block_hash(block.header.target()),
			height: height.0,
			best_height: best_height.0,
			inscriptions,
			runes,
			transactions: block.txdata,
		}
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockInfo {
	pub average_fee: u64,
	pub average_fee_rate: u64,
	pub bits: u32,
	#[serde(with = "SerHex::<Strict>")]
	pub chainwork: [u8; 32],
	pub confirmations: i32,
	pub difficulty: f64,
	pub hash: BlockHash,
	pub feerate_percentiles: [u64; 5],
	pub height: u32,
	pub max_fee: u64,
	pub max_fee_rate: u64,
	pub max_tx_size: u32,
	pub median_fee: u64,
	pub median_time: Option<u64>,
	pub merkle_root: TxMerkleNode,
	pub min_fee: u64,
	pub min_fee_rate: u64,
	pub next_block: Option<BlockHash>,
	pub nonce: u32,
	pub previous_block: Option<BlockHash>,
	pub subsidy: u64,
	pub target: BlockHash,
	pub timestamp: u64,
	pub total_fee: u64,
	pub total_size: usize,
	pub total_weight: usize,
	pub transaction_count: u64,
	pub version: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Children {
	pub ids: Vec<InscriptionId>,
	pub more: bool,
	pub page: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChildInscriptions {
	pub children: Vec<ChildInscriptionRecursive>,
	pub more: bool,
	pub page: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Inscription {
	pub address: Option<String>,
	pub charms: Vec<Charm>,
	pub child_count: u64,
	pub children: Vec<InscriptionId>,
	pub content_length: Option<usize>,
	pub content_type: Option<String>,
	pub effective_content_type: Option<String>,
	pub fee: u64,
	pub height: u32,
	pub id: InscriptionId,
	pub next: Option<InscriptionId>,
	pub number: i32,
	pub parents: Vec<InscriptionId>,
	pub previous: Option<InscriptionId>,
	pub rune: Option<SpacedRune>,
	pub sat: Option<ordinals::Sat>,
	pub satpoint: SatPoint,
	pub timestamp: i64,
	pub value: Option<u64>,
	pub metaprotocol: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InscriptionRecursive {
	pub charms: Vec<Charm>,
	pub content_type: Option<String>,
	pub content_length: Option<usize>,
	pub delegate: Option<InscriptionId>,
	pub fee: u64,
	pub height: u32,
	pub id: InscriptionId,
	pub number: i32,
	pub output: OutPoint,
	pub sat: Option<ordinals::Sat>,
	pub satpoint: SatPoint,
	pub timestamp: i64,
	pub value: Option<u64>,
	pub address: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChildInscriptionRecursive {
	pub charms: Vec<Charm>,
	pub fee: u64,
	pub height: u32,
	pub id: InscriptionId,
	pub number: i32,
	pub output: OutPoint,
	pub sat: Option<ordinals::Sat>,
	pub satpoint: SatPoint,
	pub timestamp: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Inscriptions {
	pub ids: Vec<InscriptionId>,
	pub more: bool,
	pub page_index: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UtxoRecursive {
	pub inscriptions: Option<Vec<InscriptionId>>,
	pub runes: Option<BTreeMap<SpacedRune, Pile>>,
	pub sat_ranges: Option<Vec<(u64, u64)>>,
	pub value: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Output {
	pub address: Option<Address<NetworkUnchecked>>,
	pub indexed: bool,
	pub inscriptions: Option<Vec<InscriptionId>>,
	pub outpoint: OutPoint,
	pub runes: Option<BTreeMap<SpacedRune, Pile>>,
	pub sat_ranges: Option<Vec<(u64, u64)>>,
	pub script_pubkey: ScriptBuf,
	pub spent: bool,
	pub transaction: Txid,
	pub value: u64,
}

impl Output {
	pub fn new(
		chain: Chain,
		inscriptions: Option<Vec<InscriptionId>>,
		outpoint: OutPoint,
		tx_out: TxOut,
		indexed: bool,
		runes: Option<BTreeMap<SpacedRune, Pile>>,
		sat_ranges: Option<Vec<(u64, u64)>>,
		spent: bool,
	) -> Self {
		Self {
			address: chain
				.address_from_script(&tx_out.script_pubkey)
				.ok()
				.map(|address| uncheck(&address)),
			indexed,
			inscriptions,
			outpoint,
			runes,
			sat_ranges,
			script_pubkey: tx_out.script_pubkey,
			spent,
			transaction: outpoint.txid,
			value: tx_out.value.to_sat(),
		}
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sat {
	pub address: Option<String>,
	pub block: u32,
	pub charms: Vec<Charm>,
	pub cycle: u32,
	pub decimal: String,
	pub degree: String,
	pub epoch: u32,
	pub inscriptions: Vec<InscriptionId>,
	pub name: String,
	pub number: u64,
	pub offset: u64,
	pub percentile: String,
	pub period: u32,
	pub rarity: Rarity,
	pub satpoint: Option<SatPoint>,
	pub timestamp: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SatInscription {
	pub id: Option<InscriptionId>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SatInscriptions {
	pub ids: Vec<InscriptionId>,
	pub more: bool,
	pub page: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressInfo {
	pub outputs: Vec<OutPoint>,
	pub inscriptions: Option<Vec<InscriptionId>>,
	pub sat_balance: u64,
	pub runes_balances: Option<Vec<(SpacedRune, Decimal, Option<char>)>>,
}
