use super::*;
use ord::subcommand::wallet::register_command;
use sp_core::H160;
#[test]
fn register_collection() {
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
	.run_and_deserialize_output::<register_command::Output>();
	println!("Transaction ID: {:?}", output.tx_id);
}
