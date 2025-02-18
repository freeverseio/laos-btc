use crate::fund_raw_transaction;
use crate::wallet::Wallet;
use crate::FeeRate;
use crate::TARGET_POSTAGE;
use anyhow::{anyhow, Ok, Result};
use bitcoin::consensus;
use bitcoin::Txid;
use bitcoin::{absolute::LockTime, transaction::Version, Transaction, TxOut};
use bitcoin::{Address, Amount};
use bitcoincore_rpc::Client;
use bitcoincore_rpc::RpcApi;

pub trait TxOutable {
	fn as_output(&self) -> TxOut;
}

pub struct BitcoinService {
	wallet: Wallet,
	client: Client,
}

impl BitcoinService {
	pub fn build_tx<T: TxOutable>(
		&self,
		tx: T,
		fee_rate: FeeRate,
		postage: Postage,
	) -> Result<Transaction> {
		self.wallet.lock_non_cardinal_outputs()?;

		let unfunded_tx = Transaction {
			version: Version(2),
			lock_time: LockTime::ZERO,
			input: vec![],
			output: vec![tx.as_output(), postage_as_output(postage)],
		};

		let unsigned_transaction = fund_raw_transaction(&self.client, fee_rate, &unfunded_tx)?;

		let signed_transaction = self
			.client
			.sign_raw_transaction_with_wallet(&unsigned_transaction, None, None)?
			.hex;
		let signed_transaction = consensus::encode::deserialize(&signed_transaction)?;

		Ok(signed_transaction)
	}

	pub fn get_change_address(&self) -> Result<Address> {
		self.wallet.get_change_address()
	}

	pub fn send_tx(&self, signed_tx: &Transaction) -> Result<Txid> {
		let tx_id = self.client.send_raw_transaction(signed_tx)?;
		Ok(tx_id)
	}
}

pub struct Postage {
	pub amount: Amount,
	pub destination: Address,
}

pub fn get_postage(postage: Option<Amount>, destination: Address) -> Result<Postage> {
	let postage = postage.unwrap_or(TARGET_POSTAGE);

	if destination.script_pubkey().minimal_non_dust() > postage {
		return Err(anyhow!(
			"postage below dust limit of {}sat",
			destination.script_pubkey().minimal_non_dust().to_sat()
		));
	}
	Ok(Postage { amount: postage, destination })
}

// TODO move to TxOutable as RegisterCollection?
fn postage_as_output(postage: Postage) -> TxOut {
	TxOut { value: postage.amount, script_pubkey: postage.destination.script_pubkey() }
}

// #[test]
// tx greater than maximum: runestone greater than maximum OP_RE
// postage below dust limit
// fund_raw_tx
// check that i dont spend ordinals output when sending tx
// inspirarme en mas tests de mint
// some any other check from laos domain (if so maybe should be in other file)
// check assert_eq!(Runestone::decipher(&signed_transaction), Some(Artifact::Runestone(runestone)),);
