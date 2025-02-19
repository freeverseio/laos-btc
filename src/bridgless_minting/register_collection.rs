use bitcoin::{Amount, ScriptBuf, TxOut};
use sp_core::H160;

use super::bitcoin_service::TxOutable;

pub struct RegisterCollection {
	// TODO make them private?
	pub laos_collection_address: H160,
	pub rebasable: bool,
}

impl TxOutable for RegisterCollection {
	fn as_output(&self) -> TxOut {
		TxOut { value: Amount::from_sat(0), script_pubkey: self.encipher() }
	}
}

impl RegisterCollection {
	pub fn encipher(&self) -> ScriptBuf {
		ScriptBuf::default()
	}
}

#[cfg(test)]

mod tests {
	use super::*;

	#[test]
	fn register_collection_as_output() {
		let alice = H160::from([0; 20]);
		let register_collection_tx =
			RegisterCollection { laos_collection_address: alice, rebasable: false };
		assert!(register_collection_tx.encipher().is_empty() == false);
		assert!(
			register_collection_tx.as_output().script_pubkey == register_collection_tx.encipher()
		);
		assert!(register_collection_tx.as_output().value == Amount::from_sat(0));
	}
}
