use super::*;
use ord::{bridgeless_minting::bitcoin_service, subcommand::wallet::register_collection};
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

#[test]
fn calculate_postage_amount_by_default() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let address = CommandBuilder::new("--regtest wallet receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<ord::subcommand::wallet::receive::Output>()
		.addresses
		.into_iter()
		.next()
		.unwrap();
	let address = address.require_network(core.state().network).unwrap();

	let postage = bitcoin_service::calculate_postage(None, address.clone()).unwrap();
	assert_eq!(postage.amount, Amount::from_sat(10_000));
	assert_eq!(postage.destination, address);
}

#[test]
fn calculate_postage_when_amount_is_provided() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let address = CommandBuilder::new("--regtest wallet receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<ord::subcommand::wallet::receive::Output>()
		.addresses
		.into_iter()
		.next()
		.unwrap();
	let address = address.require_network(core.state().network).unwrap();

	let amount = Amount::from_sat(20_000);
	let postage = bitcoin_service::calculate_postage(Some(amount), address.clone()).unwrap();
	assert_eq!(postage.amount, amount);
	assert_eq!(postage.destination, address);
}

#[test]
fn calculate_postage_when_low_amount_is_provided() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let address = CommandBuilder::new("--regtest wallet receive")
		.core(&core)
		.ord(&ord)
		.run_and_deserialize_output::<ord::subcommand::wallet::receive::Output>()
		.addresses
		.into_iter()
		.next()
		.unwrap();
	let address = address.require_network(core.state().network).unwrap();

	let amount = Amount::from_sat(1);
	assert!(bitcoin_service::calculate_postage(Some(amount), address.clone()).is_err(), "postage below dust limit of 330sat");
}
