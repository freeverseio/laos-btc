use bitcoin::{script::Instruction, Address, Network, TxIn, WitnessProgram, WitnessVersion};
use bitcoin_hashes::{hash160::Hash as BTCH160, ripemd160, sha256, Hash};
use sp_core::H160;
use thiserror::Error;

/// Custom error type for errors related to the address mapping.
#[derive(Debug, Error, PartialEq)]
pub enum AddressMappingError {
	/// Invalid address error
	#[error("Invalid BTC address: `{0}`. Only P2PKH and P2WPKH supported.")]
	InvalidAddress(Address),

	#[error("Invalid TxIn. Only P2PKH and P2WPKH supported.")]
	InvalidInput,
}

pub fn btc_address_to_h160(address: Address) -> Result<H160, AddressMappingError> {
	if let Some(program) = address.witness_program() {
		if program.is_p2wpkh() {
			let h160_bytes: Vec<u8> = program.program().as_bytes().to_vec();
			return Ok(H160::from_slice(&h160_bytes));
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

pub fn txin_to_h160(txin: &TxIn) -> Result<H160, AddressMappingError> {
	// Check for segwit: P2WPKH (or P2SH-P2WPKH) should have a witness stack.
	if !txin.witness.is_empty() {
		// For P2WPKH the witness must contain exactly two elements.
		if txin.witness.len() == 2 {
			let pubkey_bytes = &txin.witness[1];
			let sha256_hash = sha256::Hash::hash(pubkey_bytes);
			let ripemd160_hash = ripemd160::Hash::hash(&sha256_hash.to_byte_array());
			return Ok(H160::from_slice(&ripemd160_hash.to_byte_array()));
		} else {
			return Err(AddressMappingError::InvalidInput);
		}
	}

	// If no witness is present, then expect a legacy P2PKH.
	// The scriptSig must consist of exactly two pushes: <signature> <pubkey>
	let instructions = txin
		.script_sig
		.instructions()
		.collect::<Result<Vec<_>, _>>()
		.map_err(|_| AddressMappingError::InvalidInput)?;

	if instructions.len() != 2 {
		return Err(AddressMappingError::InvalidInput);
	}

	// Verify that the second push is the public key.
	let pubkey_bytes = if let Instruction::PushBytes(bytes) = instructions[1] {
		bytes
	} else {
		return Err(AddressMappingError::InvalidInput);
	};

	let sha256_hash = sha256::Hash::hash(pubkey_bytes.as_bytes());
	let ripemd160_hash = ripemd160::Hash::hash(&sha256_hash.to_byte_array());
	Ok(H160::from_slice(&ripemd160_hash.to_byte_array()))
}

#[cfg(test)]
mod tests {
	use super::*;
	use bitcoin::{
		blockdata::opcodes::all::OP_CHECKSIG, script::Builder, Network, Script, Witness,
	};
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

	#[test]
	fn test_txin_to_h160_segwit() {
		let pubkey_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
		let pubkey_bytes = hex::decode(pubkey_hex).unwrap();
		// Dummy signature.
		let signature = vec![0x30, 0x45, 0x02, 0x21];
		let txin = TxIn {
			previous_output: Default::default(),
			script_sig: Script::new().into(),
			sequence: bitcoin::Sequence(0xffffffff),
			witness: vec![signature, pubkey_bytes.clone()].into(),
		};

		let h160 = txin_to_h160(&txin).expect("Valid segwit TxIn should return H160");
		let expected_h160 =
			H160::from_slice(&hex::decode("751e76e8199196d454941c45d1b3a323f1433bd6").unwrap());
		assert_eq!(h160, expected_h160);
	}

	#[test]
	fn test_txin_to_h160_legacy() {
		let pubkey_hex = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
		let mut pubkey_bytes = [0; 33];
		hex::decode_to_slice(pubkey_hex, &mut pubkey_bytes).expect("The size is correct; qed;");
		// Dummy signature.
		let signature = [0x30, 0x45, 0x02, 0x21];
		let script_sig =
			Builder::new().push_slice(signature).push_slice(pubkey_bytes).into_script();

		let txin = TxIn {
			previous_output: Default::default(),
			script_sig,
			sequence: bitcoin::Sequence(0xffffffff),
			witness: Witness::new(),
		};

		let h160 = txin_to_h160(&txin).expect("Valid legacy TxIn should return H160");
		let expected_h160 =
			H160::from_slice(&hex::decode("751e76e8199196d454941c45d1b3a323f1433bd6").unwrap());
		assert_eq!(h160, expected_h160);
	}

	#[test]
	fn test_txin_to_h160_segwit_invalid_witness_length() {
		let txin = TxIn {
			previous_output: Default::default(),
			script_sig: Script::new().into(),
			sequence: bitcoin::Sequence(0xffffffff),
			witness: vec![vec![0x30, 0x45, 0x02, 0x21]].into(), // Only one element.
		};

		let err = txin_to_h160(&txin)
			.expect_err("Expected error for segwit TxIn with invalid witness length");
		assert_eq!(err, AddressMappingError::InvalidInput);
	}

	#[test]
	fn test_txin_to_h160_legacy_invalid_push_count() {
		// Create a legacy TxIn with a scriptSig that doesn't have exactly two pushes.
		// Here, we only push a dummy signature and omit the pubkey.
		let script_sig = Builder::new().push_slice([0x30, 0x45, 0x02, 0x21]).into_script();

		let txin = TxIn {
			previous_output: Default::default(),
			script_sig,
			sequence: bitcoin::Sequence(0xffffffff),
			witness: Witness::new(),
		};

		let err = txin_to_h160(&txin)
			.expect_err("Expected error for legacy TxIn with invalid push count");
		assert_eq!(err, AddressMappingError::InvalidInput);
	}

	#[test]
	fn test_txin_to_h160_legacy_invalid_push_type() {
		// Create a legacy TxIn with a scriptSig where the second instruction is not a push of
		// bytes. Instead of a pubkey push, we insert an opcode.
		let signature = [0x30, 0x45, 0x02, 0x21];
		let script_sig = Builder::new()
            .push_slice(signature)
            .push_opcode(OP_CHECKSIG) // Not a push bytes, should trigger an error.
            .into_script();

		let txin = TxIn {
			previous_output: Default::default(),
			script_sig,
			sequence: bitcoin::Sequence(0xffffffff),
			witness: Witness::new(),
		};

		let err =
			txin_to_h160(&txin).expect_err("Expected error for legacy TxIn with invalid push type");
		assert_eq!(err, AddressMappingError::InvalidInput);
	}
}
