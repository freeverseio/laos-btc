use super::*;
use ord::subcommand::wallet::{brc721::register_ownership, receive};
use ordinals::brc721::{
	address_mapping,
	register_ownership::{RegisterOwnership, SlotsBundle},
};

#[test]
fn using_fixtures_file() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	let mnemonic = "taste pole august obvious estate hurry illness bread match farm ready indicate"
		.to_string();
	CommandBuilder::new(["--regtest", "wallet", "--name", "test", "restore", "--from", "mnemonic"])
		.stdin(mnemonic.into())
		.core(&core)
		.run_and_extract_stdout();

	let output = CommandBuilder::new("--regtest wallet --name test receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<receive::Output>();

	let recipient = output
		.addresses
		.first()
		.unwrap()
		.clone()
		.require_network(Network::Regtest)
		.unwrap();
	assert_eq!(
		recipient.to_string(),
		"bcrt1pswcsgefgmts0esvgvw0hx3w3xf68ce8yf9tmsgu5ltlj5kmrcjlqd402f3"
	);
	let recipìent_h160 = address_mapping::btc_address_to_h160(recipient).unwrap();
	assert_eq!(recipìent_h160.to_string(), "0x0000000000000000000000000000000000000000"); // WIP esta address vendrá del fichero

	let balances = CommandBuilder::new("--regtest wallet --name test balance")
		.core(&core)
		.ord(&ord)
		.expected_exit_code(0)
		.stdout_regex(".*")
		.run_and_extract_stdout();
	let balances: serde_json::Value = serde_json::from_str(&balances).unwrap();
	let cardinal_balance = balances["cardinal"].as_u64().unwrap();
	assert!(cardinal_balance > 0, "Cardinal balance should be greater than 0");

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
		SlotsBundle(vec![
			(340282366920938463463374607431768211455..=340282366920938463463374607431768211455)
		])
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
	)); // TODO check if recipient is also the owner (not only the wallet)
	 // UTXO 3
	assert!(core.state().is_wallet_address(
		&Address::from_script(&tx.output[3].script_pubkey, Network::Regtest).unwrap()
	)); // TODO check if recipient is also the owner (not only the wallet)
}

#[test]
fn invalid_address() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	CommandBuilder::new("--regtest wallet brc721 register-ownership --fee-rate 1 --file tmp.yml")
	.write("tmp.yml", "collection_id: 300:1\noutputs:\n  - slots_bundle: [[0]]\n    owner: 1BitcoinEaterAddressDontSendf59kuE")
	.core(&core)
	.ord(&ord)
	.stderr_regex("(?s).*address 1BitcoinEaterAddressDontSendf59kuE is not valid on regtest.*")
		.expected_exit_code(1)
		.run_and_extract_stdout();
}

#[test]
fn insufficient_utxos() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	CommandBuilder::new("--regtest wallet brc721 register-ownership --fee-rate 1 --file tmp.yml")
	.write("tmp.yml", "collection_id: 300:1\noutputs:\n  - slots_bundle: [[0]]\n    owner: mrEqurom3cKudH7FaDrF3j1DJePLcjAU3m")
	.core(&core)
	.ord(&ord)
	.stderr_regex("(?s).*address 1BitcoinEaterAddressDontSendf59kuE is not valid on regtest.*")
		.expected_exit_code(1)
		.run_and_extract_stdout();
}
