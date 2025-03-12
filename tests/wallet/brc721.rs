mod register_collection;
use super::*;

#[test]
fn brc721_subcommand_requires_brc721_flag() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	CommandBuilder::new(
		"--regtest wallet brc721 register-collection --fee-rate 1 --address 0xfffffffffffffffffffffffe0000000000000006",
	)
	.core(&core)
	.ord(&ord)
    .expected_exit_code(1)
	.expected_stderr("error: brc721 subcommand should be used with --brc721 wallet flag\n")
    .run_and_extract_stdout();
}

#[test]
fn brc721_flag_requires_brc721_descriptors() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	create_wallet(&core, &ord);

	CommandBuilder::new(
		"--regtest wallet --brc721 brc721 register-collection --fee-rate 1 --address 0xfffffffffffffffffffffffe0000000000000006",
	)
	.core(&core)
	.ord(&ord)
    .expected_exit_code(1)
	.expected_stderr("error: wallet \"ord\" contains unexpected output descriptors, and does not appear to be an `ord` brc721 wallet, create a new wallet with `ord wallet --brc721 restore`\n")
    .run_and_extract_stdout();
}

#[test]
fn brc721_flag_only_allows_restore_and_brc721_subcommands() {
	let core = mockcore::builder().network(Network::Regtest).build();

	let ord = TestServer::spawn_with_server_args(&core, &["--regtest", "--index-brc721"], &[]);

	core.mine_blocks(1);

	CommandBuilder::new("--regtest wallet --brc721 create")
		.core(&core)
		.ord(&ord)
		.expected_exit_code(1)
		.expected_stderr(
			"error: Only restore and brc721 commands are supported with --brc721 flag\n",
		)
		.run_and_extract_stdout();
}

#[test]
fn restore_with_wrong_descriptors_fails() {
	let core = mockcore::spawn();

	CommandBuilder::new("wallet --brc721 --name foo restore --from descriptor")
      .stdin(r#"
{
  "wallet_name": "bar",
  "descriptors": [
    {
    	"desc": "wpkh([1bd99ea6/84h/1h/0h]tpubDCBg5D4XDJtgRtFFQNTBUZAqWLCeQDrgXPkvKirFrCXREmX38bHYUUTMT7xVeyzTcgYp7dfZ4RnAScTKNf5h9VmN65aiYizyAzhRYQMvoM9/0/*)#d0v92la2",
        "timestamp": 1741691061,
        "active": true,
        "internal": false,
        "range": [
            0,
            999
        ],
        "next": 0,
        "next_index": 0
    },
    {
        "desc": "wpkh([1bd99ea6/84h/1h/0h]tpubDCBg5D4XDJtgRtFFQNTBUZAqWLCeQDrgXPkvKirFrCXREmX38bHYUUTMT7xVeyzTcgYp7dfZ4RnAScTKNf5h9VmN65aiYizyAzhRYQMvoM9/1/*)#umfyh2dj",
        "timestamp": 1741691061,
        "active": true,
        "internal": true,
        "range": [
            0,
            999
        ],
        "next": 0,
        "next_index": 0
    },
	{
    	"desc": "tr([c0b9536d/86'/1'/0']tprv8fXhtVjj3vb7kgxKuiWXzcUsur44gbLbbtwxL4HKmpzkBNoMrYqbQhMe7MWhrZjLFc9RBpTRYZZkrS8HH1Q3SmD5DkfpjKqtd97q1JWfqzr/1/*)#u6uap67k",
    	"timestamp": 1706047839,
    	"active": true,
    	"internal": true,
    	"range": [
    	  0,
    	  1013
    	],
    	"next": 14
    }
  ]
}"#.into())
    .core(&core)
    .expected_exit_code(1)
    .expected_stderr("error: wallet \"foo\" contains unexpected output descriptors, and does not appear to be an `ord` brc721 wallet, create a new wallet with `ord wallet --brc721 restore`\n")
    .run_and_extract_stdout();
}

#[test]
fn restore_with_mnemonic_fails() {
	let core = mockcore::spawn();

	CommandBuilder::new("wallet --brc721 --name foo restore --from mnemonic")
		.core(&core)
		.expected_exit_code(1)
		.expected_stderr("error: Only descriptor source is supported for brc721 wallets\n")
		.run_and_extract_stdout();
}

#[test]
fn restore_with_descriptors_works() {
	let core = mockcore::spawn();

	CommandBuilder::new("wallet --brc721 --name foo restore --from descriptor")
      .stdin(r#"
{
  "wallet_name": "bar",
  "descriptors": [
    {
    	"desc": "wpkh([1bd99ea6/84h/1h/0h]tpubDCBg5D4XDJtgRtFFQNTBUZAqWLCeQDrgXPkvKirFrCXREmX38bHYUUTMT7xVeyzTcgYp7dfZ4RnAScTKNf5h9VmN65aiYizyAzhRYQMvoM9/0/*)#d0v92la2",
        "timestamp": 1741691061,
        "active": true,
        "internal": false,
        "range": [
            0,
            999
        ],
        "next": 0,
        "next_index": 0
    },
    {
        "desc": "wpkh([1bd99ea6/84h/1h/0h]tpubDCBg5D4XDJtgRtFFQNTBUZAqWLCeQDrgXPkvKirFrCXREmX38bHYUUTMT7xVeyzTcgYp7dfZ4RnAScTKNf5h9VmN65aiYizyAzhRYQMvoM9/1/*)#umfyh2dj",
        "timestamp": 1741691061,
        "active": true,
        "internal": true,
        "range": [
            0,
            999
        ],
        "next": 0,
        "next_index": 0
    }
  ]
}"#.into())
    .core(&core)
    .expected_exit_code(0)
    .run_and_extract_stdout();
}
