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
use crate::wallet::Maturity;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ResumeOutput {
	pub etchings: Vec<batch::Output>,
}
#[derive(Debug, Parser)]
pub(crate) struct Resume {
	#[arg(long, help = "Don't broadcast transactions.")]
	pub(crate) dry_run: bool,
	#[arg(long, help = "Pending <RUNE> etching to resume.")]
	pub(crate) rune: Option<SpacedRune>,
}

impl Resume {
	pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
		let mut etchings = Vec::new();
		loop {
			if SHUTTING_DOWN.load(atomic::Ordering::Relaxed) {
				break;
			}

			let spaced_rune = self.rune;

			let pending_etchings = if let Some(spaced_rune) = spaced_rune {
				let pending_etching = wallet.load_etching(spaced_rune.rune)?;

				ensure!(
					pending_etching.is_some(),
					"rune {spaced_rune} does not correspond to any pending etching."
				);

				vec![(spaced_rune.rune, pending_etching.unwrap())]
			} else {
				wallet.pending_etchings()?
			};

			for (rune, entry) in pending_etchings {
				if self.dry_run {
					etchings
						.push(batch::Output { reveal_broadcast: false, ..entry.output.clone() });
					continue;
				};

				match wallet.check_maturity(rune, &entry.commit)? {
					Maturity::Mature => etchings.push(wallet.send_etching(rune, &entry)?),
					Maturity::CommitSpent(txid) => {
						eprintln!("Commitment for rune etching {rune} spent in {txid}");
						wallet.clear_etching(rune)?;
					},
					Maturity::CommitNotFound => {},
					Maturity::BelowMinimumHeight(_) => {},
					Maturity::ConfirmationsPending(_) => {},
				}
			}

			if wallet.pending_etchings()?.is_empty() {
				break;
			}

			if self.dry_run {
				break;
			}

			if !wallet.integration_test() {
				thread::sleep(Duration::from_secs(5));
			}
		}

		Ok(Some(Box::new(ResumeOutput { etchings }) as Box<dyn Output>))
	}
}
