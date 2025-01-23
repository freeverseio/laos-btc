# laos-btc

`laos-btc` is an index, block explorer, and command-line wallet. It is experimental
software with no warranty. See [LICENSE](LICENSE) for more details.

Ordinal theory imbues satoshis with numismatic value, allowing them to
be collected and traded as curios.

Ordinal numbers are serial numbers for satoshis, assigned in the order in which
they are mined, and preserved across transactions.

## Wallet

`laos-btc` relies on Bitcoin Core for private key management and transaction signing.
This has a number of implications that you must understand in order to use
`laos-btc` wallet commands safely:

- Bitcoin Core is not aware of inscriptions and does not perform sat
  control. Using `bitcoin-cli` commands and RPC calls with `laos-btc` wallets may
  lead to loss of inscriptions.

- `laos-btc wallet` commands automatically load the `laos-btc` wallet given by the
  `--name` option, which defaults to 'laos-btc'. Keep in mind that after running
  an `laos-btc wallet` command, an `laos-btc` wallet may be loaded.

- Because `laos-btc` has access to your Bitcoin Core wallets, `laos-btc` should not be
  used with wallets that contain a material amount of funds. Keep ordinal and
  cardinal wallets segregated.

## Installation

`laos-btc` is written in Rust and can be built from
[source](https://github.com/freeverseio/laos-btc).

Once `laos-btc` is installed, you should be able to run `laos-btc --version` on the
command line.

### Building

On Linux, `laos-btc` requires `libssl-dev` when building from source.

On Debian-derived Linux distributions (including Ubuntu):
```shell
sudo apt-get install pkg-config libssl-dev build-essential
```

On Red Hat-derived Linux distributions:
```shell
yum install -y pkgconfig openssl-devel
yum groupinstall "Development Tools"
```

You'll also need Rust:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the `laos-btc` repo:
```shell
git clone https://github.com/freeverseio/laos-btc.git
cd laos-btc
```

And finally to actually build `laos-btc`:
```shell
cargo build --release
```

Once built, the `laos-btc` binary can be found at `./target/release/laos-btc`.

### Docker
A Docker image can be built with:
```shell
docker build -t laos-btc .
```

## Syncing

`laos-btc` requires a synced `bitcoind` node with `-txindex` to build the index of
satoshi locations. `laos-btc` communicates with `bitcoind` via RPC.

If `bitcoind` is run locally by the same user, without additional
configuration, `laos-btc` should find it automatically by reading the `.cookie` file
from `bitcoind`'s datadir, and connecting using the default RPC port.

If `bitcoind` is not on mainnet, is not run by the same user, has a non-default
datadir, or a non-default port, you'll need to pass additional flags to `laos-btc`.
See `laos-btc --help` for details.

### `bitcoind` RPC Authentication

`laos-btc` makes RPC calls to `bitcoind`, which usually requires a username and
password.

By default, `laos-btc` looks a username and password in the cookie file created by
`bitcoind`.

The cookie file path can be configured using `--cookie-file`:

```shell
laos-btc --cookie-file /path/to/cookie/file server
```

Alternatively, `laos-btc` can be supplied with a username and password on the
command line:
```shell
laos-btc --bitcoin-rpc-username foo --bitcoin-rpc-password bar server
```

Using environment variables:
```shell
export ORD_BITCOIN_RPC_USERNAME=foo
export ORD_BITCOIN_RPC_PASSWORD=bar
laos-btc server
```

Or in the config file:
```yaml
bitcoin_rpc_username: foo
bitcoin_rpc_password: bar
```

## Logging
`laos-btc` uses [env_logger](https://docs.rs/env_logger/latest/env_logger/). Set the
`RUST_LOG` environment variable in order to turn on logging. For example, run
the server and show `info`-level log messages and above:
```shell
$ RUST_LOG=info cargo run server
```

Set the `RUST_BACKTRACE` environment variable in order to turn on full rust
backtrace. For example, run the server and turn on debugging and full backtrace:
```shell
$ RUST_BACKTRACE=1 RUST_LOG=debug laos-btc server
```
