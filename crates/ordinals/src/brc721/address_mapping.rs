use bitcoin::{Address, Network, WitnessProgram, WitnessVersion};
use bitcoin_hashes::{hash160::Hash as BTCH160, Hash};
use sp_core::H160;
use thiserror::Error;
/// Custom error type for errors related to the address mapping.
#[derive(Debug, Error, PartialEq)]
pub enum AddressMappingError {
	/// Invalid address error
	#[error("Invalid BTC address: `{0}`. Only P2PKH and P2WPKH supported.")]
	InvalidAddress(Address),
}

pub fn btc_address_to_h160(address: Address) -> Result<H160, AddressMappingError> {
	if let Some(program) = address.witness_program() {
		if program.is_p2wpkh() {
			let h160_bytes: Vec<u8> = program.program().as_bytes().to_vec();
			return Ok(H160::from_slice(&h160_bytes));
		}

		if program.is_p2tr() {
			let pubkey = program.program().as_bytes();
			let hash: BTCH160 = BTCH160::hash(pubkey);
			return Ok(H160::from_slice(hash.as_byte_array()));
		}
	} else {
		let script_pubkey = address.script_pubkey();
		let script_bytes = script_pubkey.as_bytes();

		if script_bytes.len() == 25 && script_bytes[0] == 0x76  // OP_DUP
            && script_bytes[1] == 0xa9  // OP_HASH160
            && script_bytes[2] == 0x14  // H160 length (20 bytes)
            && script_bytes[23] == 0x88 // OP_EQUALVERIFY
            && script_bytes[24] == 0xac
		// OP_CHECKSIG
		{
			return Ok(H160::from_slice(&script_bytes[3..23]));
		}
	}

	Err(AddressMappingError::InvalidAddress(address))
}

pub fn h160_to_btc_address(
	h160: H160,
	network: Network,
	is_segwit: bool,
) -> Result<Address, AddressMappingError> {
	let bytes = h160.as_bytes();

	if is_segwit {
		// P2WPKH address
		let program = WitnessProgram::new(WitnessVersion::V0, bytes)
			.expect("H160 contains exactly 20 bytes; qed;");
		Ok(Address::from_witness_program(program, network))
	} else {
		// P2PKH address
		let hash: BTCH160 =
			BTCH160::from_slice(bytes).expect("H160 contains exactly 20 bytes; qed;");
		Ok(Address::p2pkh(hash, network))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::Network;
	use std::str::FromStr;

	#[test]
	fn test_p2wpkh_conversion() {
		let addr_str = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4";
		let address =
			Address::from_str(addr_str).unwrap().require_network(Network::Bitcoin).unwrap();

		let expected_h160 =
			H160::from_slice(&hex::decode("751e76e8199196d454941c45d1b3a323f1433bd6").unwrap());

		let h160 = btc_address_to_h160(address.clone())
			.expect("Valid P2WPKH address should be mapped correctly");
		assert_eq!(h160, expected_h160);

		let back_address = h160_to_btc_address(h160, Network::Bitcoin, true)
			.expect("Valid H160 should be mapped back to P2WPKH");
		assert_eq!(address, back_address);
	}

	#[test]
	fn test_p2pkh_conversion() {
		let addr_str = "1BgGZ9tcN4rm9KBzDn7KprQz87SZ26SAMH";
		let address =
			Address::from_str(addr_str).unwrap().require_network(Network::Bitcoin).unwrap();

		let expected_h160 =
			H160::from_slice(&hex::decode("751e76e8199196d454941c45d1b3a323f1433bd6").unwrap());

		let h160 = btc_address_to_h160(address.clone())
			.expect("Valid P2PKH address should be mapped correctly");
		assert_eq!(h160, expected_h160);

		let back_address = h160_to_btc_address(h160, Network::Bitcoin, false)
			.expect("Valid H160 should be mapped back to P2PKH");
		assert_eq!(address, back_address);
	}

	#[test]
	fn p2tr_conversion() {
		let addr_str = "bcrt1pswcsgefgmts0esvgvw0hx3w3xf68ce8yf9tmsgu5ltlj5kmrcjlqd402f3";
		let address =
			Address::from_str(addr_str).unwrap().require_network(Network::Regtest).unwrap();

		let expected_h160 =
			H160::from_slice(&hex::decode("4e7b5ee0272b429056a8c7de8d464c67aa17facf").unwrap());

		let h160 = btc_address_to_h160(address.clone())
			.expect("Valid P2TR address should be mapped correctly");
		assert_eq!(h160, expected_h160);

		let back_address = h160_to_btc_address(h160, Network::Regtest, false)
			.expect("Valid H160 should be mapped back to P2TR");
		assert!(address != back_address);
	}

	#[test]
	fn test_invalid_address_error() {
		// This is a typical P2SH address which is not supported by our conversion functions.
		let addr_str = "3QJmV3qfvL9SuYo34YihAf3sRCW3qSinyC";
		let address =
			Address::from_str(addr_str).unwrap().require_network(Network::Bitcoin).unwrap();

		let result = btc_address_to_h160(address.clone());
		match result {
			Err(AddressMappingError::InvalidAddress(addr)) => {
				assert_eq!(addr, address);
			},
			_ => panic!("Expected InvalidAddress error variant"),
		}
	}
}
