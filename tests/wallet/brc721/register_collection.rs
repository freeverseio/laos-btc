use super::*;
use ord::{subcommand::wallet::brc721::register_collection, templates::Brc721CollectionsHtml};
use ordinals::Brc721Collection;
use sp_core::H160;

#[test]
fn register_collection_returns_tx_id() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let alice = H160::from_slice(&[0; 20]);

	let output = CommandBuilder::new(format!(
		"--regtest wallet brc721 register-collection --fee-rate 1 --address {:x} --rebaseable",
		alice
	))
	.core(&core)
	.ord(&ord)
	.expected_exit_code(0)
	.run_and_deserialize_output::<register_collection::Output>();
	assert_eq!(output.tx_id, core.mempool()[0].compute_txid());

	core.mine_blocks(1);

	let tx = core.tx_by_id(output.tx_id);
	assert_eq!(tx.output.len(), 3);
	let register_collection = RegisterCollection::from_script(&tx.output[0].script_pubkey).unwrap();
	assert!(register_collection.rebaseable);
	assert_eq!(register_collection.address, alice);
}

#[test]
fn rebaseable_is_false_by_default() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let alice = format!("{:x}", H160::from_slice(&[0; 20]));

	let output = CommandBuilder::new(format!(
		"--regtest wallet brc721 register-collection --fee-rate 1 --address {}",
		alice
	))
	.core(&core)
	.ord(&ord)
	.expected_exit_code(0)
	.run_and_deserialize_output::<register_collection::Output>();

	core.mine_blocks(1);

	let tx = core.tx_by_id(output.tx_id);
	assert_eq!(tx.output.len(), 3);
	let register_collection = RegisterCollection::from_script(&tx.output[0].script_pubkey).unwrap();
	assert!(!register_collection.rebaseable);
}

#[test]
fn register_collection_command_indexer_integration() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	let alice = H160::from_slice(&[0; 20]);

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
}
