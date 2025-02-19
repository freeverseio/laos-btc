use crate::{fund_raw_transaction, wallet::Wallet, FeeRate, TARGET_POSTAGE};
use anyhow::{anyhow, Ok, Result};
use bitcoin::{
	absolute::LockTime, consensus, transaction::Version, Address, Amount, ScriptBuf, Transaction,
	TxOut, Txid,
};
use bitcoincore_rpc::RpcApi;

pub trait Scriptable {
	fn encipher(&self) -> ScriptBuf;
}

pub(crate) struct BitcoinService {
	pub wallet: Wallet,
}

impl BitcoinService {
	pub fn new(wallet: Wallet) -> Self {
		Self { wallet }
	}
	pub fn build_tx<T: Scriptable>(
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
			output: vec![
				TxOut { value: Amount::from_sat(0), script_pubkey: tx.encipher() },
				TxOut { value: postage.amount, script_pubkey: postage.destination.script_pubkey() },
			],
		};

		let unsigned_transaction =
			fund_raw_transaction(self.wallet.bitcoin_client(), fee_rate, &unfunded_tx)?;

		let signed_transaction = self
			.wallet
			.bitcoin_client()
			.sign_raw_transaction_with_wallet(&unsigned_transaction, None, None)?
			.hex;
		let signed_transaction = consensus::encode::deserialize(&signed_transaction)?;

		Ok(signed_transaction)
	}

	pub fn get_change_address(&self) -> Result<Address> {
		self.wallet.get_change_address()
	}

	pub fn send_tx(&self, signed_tx: &Transaction) -> Result<Txid> {
		let tx_id = self.wallet.bitcoin_client().send_raw_transaction(signed_tx)?;
		Ok(tx_id)
	}
}

pub struct Postage {
	pub amount: Amount,
	pub destination: Address,
}

pub fn calculate_postage(postage: Option<Amount>, destination: Address) -> Result<Postage> {
	let postage = postage.unwrap_or(TARGET_POSTAGE);

	if postage < destination.script_pubkey().minimal_non_dust() {
		return Err(anyhow!(
			"postage below dust limit of {}sat",
			destination.script_pubkey().minimal_non_dust().to_sat()
		));
	}
	Ok(Postage { amount: postage, destination })
}
