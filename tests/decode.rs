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
use bitcoin::{
	absolute::LockTime, consensus::Encodable, opcodes, script, transaction::Version, ScriptBuf,
	Sequence, Transaction, TxIn, TxOut, Witness,
};
use ord::{
	subcommand::decode::{CompactInscription, CompactOutput, RawOutput},
	Envelope, Inscription,
};

fn transaction() -> Vec<u8> {
	let script = script::Builder::new()
		.push_opcode(opcodes::OP_FALSE)
		.push_opcode(opcodes::all::OP_IF)
		.push_slice(b"ord")
		.push_slice([1])
		.push_slice(b"text/plain;charset=utf-8")
		.push_slice([])
		.push_slice([0, 1, 2, 3])
		.push_opcode(opcodes::all::OP_ENDIF)
		.into_script();

	let mut witness = Witness::new();

	witness.push(script);
	witness.push([]);

	let transaction = Transaction {
		version: Version(2),
		lock_time: LockTime::ZERO,
		input: vec![TxIn {
			previous_output: OutPoint::null(),
			script_sig: ScriptBuf::new(),
			sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
			witness,
		}],
		output: vec![TxOut {
			script_pubkey: Runestone::default().encipher(),
			value: Amount::from_sat(0),
		}],
	};

	let mut buffer = Vec::new();

	transaction.consensus_encode(&mut buffer).unwrap();

	buffer
}

#[test]
fn from_file() {
	pretty_assert_eq!(
		CommandBuilder::new("decode --file transaction.bin")
			.write("transaction.bin", transaction())
			.run_and_deserialize_output::<RawOutput>(),
		RawOutput {
			inscriptions: vec![Envelope {
				payload: Inscription {
					body: Some(vec![0, 1, 2, 3]),
					content_type: Some(b"text/plain;charset=utf-8".into()),
					..default()
				},
				input: 0,
				offset: 0,
				pushnum: false,
				stutter: false,
			}],
			runestone: Some(Artifact::Runestone(Runestone::default())),
		},
	);
}

#[test]
fn from_stdin() {
	pretty_assert_eq!(
		CommandBuilder::new("decode")
			.stdin(transaction())
			.run_and_deserialize_output::<RawOutput>(),
		RawOutput {
			inscriptions: vec![Envelope {
				payload: Inscription {
					body: Some(vec![0, 1, 2, 3]),
					content_type: Some(b"text/plain;charset=utf-8".into()),
					..default()
				},
				input: 0,
				offset: 0,
				pushnum: false,
				stutter: false,
			}],
			runestone: Some(Artifact::Runestone(Runestone::default())),
		},
	);
}

#[test]
fn from_core() {
	let core = mockcore::spawn();
	let ord = TestServer::spawn(&core);

	create_wallet(&core, &ord);

	core.mine_blocks(1);

	let (_inscription, reveal) = inscribe(&core, &ord);

	pretty_assert_eq!(
		CommandBuilder::new(format!("decode --txid {reveal}"))
			.core(&core)
			.run_and_deserialize_output::<RawOutput>(),
		RawOutput {
			inscriptions: vec![Envelope {
				payload: Inscription {
					body: Some(b"FOO".into()),
					content_type: Some(b"text/plain;charset=utf-8".into()),
					..default()
				},
				input: 0,
				offset: 0,
				pushnum: false,
				stutter: false,
			}],
			runestone: None,
		},
	);
}

#[test]
fn compact() {
	pretty_assert_eq!(
		CommandBuilder::new("decode --compact --file transaction.bin")
			.write("transaction.bin", transaction())
			.run_and_deserialize_output::<CompactOutput>(),
		CompactOutput {
			inscriptions: vec![CompactInscription {
				body: Some("00010203".into()),
				content_encoding: None,
				content_type: Some("text/plain;charset=utf-8".into()),
				duplicate_field: false,
				incomplete_field: false,
				metadata: None,
				metaprotocol: None,
				parents: Vec::new(),
				pointer: None,
				unrecognized_even_field: false,
			}],
			runestone: Some(Artifact::Runestone(Runestone::default())),
		},
	);
}
