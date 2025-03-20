use super::*;
use ord::{
	subcommand::wallet::{
		brc721::{register_collection, register_ownership},
		receive,
	},
	templates::{Brc721CollectionsHtml, Brc721TokenHtml},
};
use ordinals::{
	brc721::{
		address_mapping,
		register_ownership::{RegisterOwnership, SlotsBundle},
		token::UtxoId,
	},
	Brc721Collection, Brc721Token,
};
use sp_core::{H160, U256};

#[test]
fn fixtures_file() {
	let core = mockcore::builder().network(Network::Regtest).build();
	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	// Restore `test` wallet
	let mnemonic = "taste pole august obvious estate hurry illness bread match farm ready indicate"
		.to_string();
	CommandBuilder::new(["--regtest", "wallet", "--name", "test", "restore", "--from", "mnemonic"])
		.stdin(mnemonic.into())
		.core(&core)
		.run_and_extract_stdout();

	// Get initial owner address
	let output = CommandBuilder::new("--regtest wallet --name test receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<receive::Output>();

	let initial_owner = output
		.addresses
		.first()
		.unwrap()
		.clone()
		.require_network(Network::Regtest)
		.unwrap();

	assert_eq!(
		initial_owner.to_string(),
		"bcrt1pswcsgefgmts0esvgvw0hx3w3xf68ce8yf9tmsgu5ltlj5kmrcjlqd402f3"
	);
	let initial_owner_h160 = address_mapping::btc_address_to_h160(initial_owner.clone()).unwrap();
	assert_eq!(
		initial_owner_h160,
		H160::from_slice(&hex::decode("4e7b5ee0272b429056a8c7de8d464c67aa17facf").unwrap())
	);

	// Fund initial owner address
	core.mine_blocks_to(3, initial_owner.clone());

	// Call register ownership
	let file_path =
		format!("{}/tests/fixtures/brc721_register_ownership.yml", env!("CARGO_MANIFEST_DIR"));
	let output = CommandBuilder::new(format!(
		"--regtest wallet --name test brc721 register-ownership --fee-rate 1 --file {}",
		file_path
	))
	.core(&core)
	.ord(&ord)
	.expected_exit_code(0)
	.run_and_deserialize_output::<register_ownership::Output>();

	core.mine_blocks(1);

	let tx = core.tx_by_id(output.tx_id);
	assert_eq!(tx.output.len(), 4);

	// UTXO 0
	let register_ownership =
		RegisterOwnership::try_from(tx.output[0].script_pubkey.clone()).unwrap();
	assert_eq!(register_ownership.collection_id, Brc721CollectionId::from_str("300:1").unwrap());
	assert_eq!(register_ownership.slots_bundles.len(), 2);
	assert_eq!(register_ownership.slots_bundles[0], SlotsBundle(vec![(0..=3), (4..=10)]));
	assert_eq!(
		register_ownership.slots_bundles[1],
		SlotsBundle(vec![(79228162514264337593543950335..=79228162514264337593543950335)])
	);
	// UTXO 1
	assert_eq!(
		Address::from_script(&tx.output[1].script_pubkey, Network::Regtest)
			.unwrap()
			.to_string(),
		"mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m"
	);
	// UTXO 2
	assert!(core.state().is_wallet_address(
		&Address::from_script(&tx.output[2].script_pubkey, Network::Regtest).unwrap()
	));
	assert_eq!(
		Address::from_script(&tx.output[2].script_pubkey, Network::Regtest)
			.unwrap()
			.to_string(),
		initial_owner.to_string()
	);
	// UTXO 3 (postage)
	assert!(core.state().is_wallet_address(
		&Address::from_script(&tx.output[3].script_pubkey, Network::Regtest).unwrap()
	));
}

#[test]
fn invalid_recipient() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	CommandBuilder::new("--regtest wallet brc721 register-ownership --fee-rate 1 --file tmp.yml")
	.write("tmp.yml", "collection_id: 300:1\ninitial_owner: bcrt1pe3p3nce9x258cuttetd4jl5f7398xge4mmafs3kxcfuqvuxec8rq63wsae\noutputs:\n  - slots_bundle: [[0]]\n    recipient: 1BitcoinEaterAddressDontSendf59kuE")
	.core(&core)
	.ord(&ord)
	.stderr_regex("(?s).*address 1BitcoinEaterAddressDontSendf59kuE is not valid on regtest.*")
		.expected_exit_code(1)
		.run_and_extract_stdout();
}

#[test]
fn invalid_initial_owner() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	CommandBuilder::new("--regtest wallet brc721 register-ownership --fee-rate 1 --file tmp.yml")
	.write("tmp.yml", "collection_id: 300:1\ninitial_owner: bc1p0zp08hxlum5p5ghmf3qsjvk3q0my3jqwmzq2zz04hpqykmh0mvlq35z3xd\noutputs:\n  - slots_bundle: [[0]]\n    recipient: mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m")
	.core(&core)
	.ord(&ord)
	.stderr_regex("(?s).*address bc1p0zp08hxlum5p5ghmf3qsjvk3q0my3jqwmzq2zz04hpqykmh0mvlq35z3xd is not valid on regtest.*")
		.expected_exit_code(1)
		.run_and_extract_stdout();
}

#[test]
fn insufficient_utxos() {
	let core = mockcore::builder().network(Network::Regtest).build();
	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);
	create_wallet(&core, &ord);

	let output = CommandBuilder::new("--regtest wallet receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<receive::Output>();

	let initial_owner = output
		.addresses
		.first()
		.unwrap()
		.clone()
		.require_network(Network::Regtest)
		.unwrap();

	assert!(core.state().is_wallet_address(&initial_owner));
	assert_eq!(
		initial_owner.to_string(),
		"bcrt1phe7th6w2q6na0v4p0345la9ctvmze0xhacvhpk4xdha2sdxltxsqtkttl7"
	);

	CommandBuilder::new("--regtest wallet brc721 register-ownership --fee-rate 1 --file tmp.yml")
	.write("tmp.yml", "collection_id: 300:1\ninitial_owner: bcrt1phe7th6w2q6na0v4p0345la9ctvmze0xhacvhpk4xdha2sdxltxsqtkttl7\noutputs:\n  - slots_bundle: [[0]]\n    recipient: mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m")
	.core(&core)
	.ord(&ord)
	.stderr_regex("(?s).*error: No available UTXOs found for address bcrt1phe7th6w2q6na0v4p0345la9ctvmze0xhacvhpk4xdha2sdxltxsqtkttl7.*")
		.expected_exit_code(1)
		.run_and_extract_stdout();
}

#[test]
fn initial_owner_not_in_wallet() {
	let core = mockcore::builder().network(Network::Regtest).build();
	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);
	create_wallet(&core, &ord);

	assert!(!core.state().is_wallet_address(
		&Address::from_str("bcrt1pe3p3nce9x258cuttetd4jl5f7398xge4mmafs3kxcfuqvuxec8rq63wsae")
			.unwrap()
			.assume_checked()
	));

	CommandBuilder::new("--regtest wallet brc721 register-ownership --fee-rate 1 --file tmp.yml")
	.write("tmp.yml", "collection_id: 300:1\ninitial_owner: bcrt1pe3p3nce9x258cuttetd4jl5f7398xge4mmafs3kxcfuqvuxec8rq63wsae\noutputs:\n  - slots_bundle: [[0]]\n    recipient: mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m")
	.core(&core)
	.ord(&ord)
	.stderr_regex("(?s).*error: initial owner address bcrt1pe3p3nce9x258cuttetd4jl5f7398xge4mmafs3kxcfuqvuxec8rq63wsae is not controlled by wallet.*")
		.expected_exit_code(1)
		.run_and_extract_stdout();
}

#[test]
fn register_ownership_command_indexer_integration() {
	let core = mockcore::builder().network(Network::Regtest).build();
	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let alice = H160::from_slice(&[0; 20]);

	// Get initial owner address
	let output = CommandBuilder::new("--regtest wallet receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<receive::Output>();

	let initial_owner = output
		.addresses
		.first()
		.unwrap()
		.clone()
		.require_network(Network::Regtest)
		.unwrap();

	assert_eq!(
		initial_owner.to_string(),
		"bcrt1pswcsgefgmts0esvgvw0hx3w3xf68ce8yf9tmsgu5ltlj5kmrcjlqd402f3"
	);

	CommandBuilder::new(format!(
		"--regtest wallet brc721 register-collection --fee-rate 1 --address {:x} --rebaseable",
		alice
	))
	.core(&core)
	.ord(&ord)
	.expected_exit_code(0)
	.run_and_deserialize_output::<register_collection::Output>();

	core.mine_blocks(1);
	ord.assert_html(
		"/brc721/collections",
		Chain::Regtest,
		Brc721CollectionsHtml {
			entries: vec![Brc721Collection::new(
				Brc721CollectionId { block: 2, tx: 1 },
				H160::from_str("0x0000000000000000000000000000000000000000").unwrap(),
				true,
			)],
			more: false,
			prev: None,
			next: None,
		},
	);

	let initial_owner_h160 = address_mapping::btc_address_to_h160(initial_owner.clone()).unwrap();
	assert_eq!(
		initial_owner_h160,
		H160::from_slice(&hex::decode("4e7b5ee0272b429056a8c7de8d464c67aa17facf").unwrap())
	);

	let expected_owner = Address::from_str("mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m")
		.unwrap()
		.require_network(Network::Regtest)
		.unwrap();

	// Fund initial owner address
	core.mine_blocks_to(3, initial_owner.clone());

	// Call register ownership
	let file_path =
		format!("{}/tests/fixtures/brc721_register_ownership.yml", env!("CARGO_MANIFEST_DIR"));
	let output = CommandBuilder::new(format!(
		"--regtest wallet brc721 register-ownership --fee-rate 1 --file {}",
		file_path
	))
	.core(&core)
	.ord(&ord)
	.expected_exit_code(0)
	.run_and_deserialize_output::<register_ownership::Output>();

	core.mine_blocks(1);

	let token_id_parts = ([3u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], initial_owner_h160.0);

	let mut raw_token_id = [0u8; 32];
	raw_token_id[..12].copy_from_slice(&token_id_parts.0);
	raw_token_id[12..].copy_from_slice(&token_id_parts.1);

	let expected_token_id = U256::from_little_endian(&raw_token_id);

	ord.assert_html(
		format!("/brc721/token/2:1/{expected_token_id}"),
		Chain::Regtest,
		Brc721TokenHtml {
			entry: Brc721Token::new(None, Some(UtxoId { tx_idx: 1, tx_out_idx: 1, utxo_idx: 0 })),
		},
	);

	let tx = core.tx_by_id(output.tx_id);

	let actual_owner = Address::from_script(&tx.output[1].script_pubkey, Network::Regtest).unwrap();

	assert_eq!(expected_owner, actual_owner);
}
