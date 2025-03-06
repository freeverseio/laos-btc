use super::*;
use ord::subcommand::wallet::brc721::register_ownership;
use ordinals::brc721::register_ownership::{RegisterOwnership, SlotsBundle};

#[test]
fn register_ownership_using_fixtures_file() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

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
	));
	// UTXO 3
	assert!(core.state().is_wallet_address(
		&Address::from_script(&tx.output[3].script_pubkey, Network::Regtest).unwrap()
	));
}
