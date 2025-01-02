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

#[derive(Debug, Parser)]
pub(super) struct SharedArgs {
	#[arg(
		long,
		help = "Use <COMMIT_FEE_RATE> sats/vbyte for commit transaction.\nDefaults to <FEE_RATE> if unset."
	)]
	pub(crate) commit_fee_rate: Option<FeeRate>,
	#[arg(long, help = "Compress inscription content with brotli.")]
	pub(crate) compress: bool,
	#[arg(long, help = "Use fee rate of <FEE_RATE> sats/vB.")]
	pub(crate) fee_rate: FeeRate,
	#[arg(long, help = "Don't sign or broadcast transactions.")]
	pub(crate) dry_run: bool,
	#[arg(long, alias = "nobackup", help = "Do not back up recovery key.")]
	pub(crate) no_backup: bool,
	#[arg(
		long,
		alias = "nolimit",
		help = "Allow transactions larger than MAX_STANDARD_TX_WEIGHT of 400,000 weight units and \
    OP_RETURNs greater than 83 bytes. Transactions over this limit are nonstandard and will not be \
    relayed by bitcoind in its default configuration. Do not use this flag unless you understand \
    the implications."
	)]
	pub(crate) no_limit: bool,
}
