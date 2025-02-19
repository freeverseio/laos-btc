use super::*;
use ord::subcommand::wallet::register_collection;
use sp_core::H160;
#[test]
fn register_collection_returns_tx_id() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let alice = H160::from_slice(&[0; 20]);

	let output = CommandBuilder::new(format!(
		"--regtest wallet register --fee-rate 1 --collection-address {:x} --rebaseable",
		alice
	))
	.core(&core)
	.ord(&ord)
	.expected_exit_code(0)
	.run_and_deserialize_output::<register_collection::Output>();
	assert_eq!(output.tx_id, core.mempool()[0].compute_txid());

	core.mine_blocks(1);

	let tx = core.tx_by_id(output.tx_id);
	let register_collection = RegisterCollection::decipher(&tx).unwrap();
	assert!(register_collection.rebaseable);
	assert_eq!(register_collection.address, alice);
}

#[test]
fn rebaseable_is_false_by_default() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let alice = format!("{:x}", H160::from_slice(&[0; 20]));

	let output = CommandBuilder::new(format!(
		"--regtest wallet register --fee-rate 1 --collection-address {}",
		alice
	))
	.core(&core)
	.ord(&ord)
	.expected_exit_code(0)
	.run_and_deserialize_output::<register_collection::Output>();

	core.mine_blocks(1);

	let tx = core.tx_by_id(output.tx_id);
	let register_collection = RegisterCollection::decipher(&tx).unwrap();
	assert!(!register_collection.rebaseable);
}
