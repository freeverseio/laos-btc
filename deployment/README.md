Use this website to generate the appropriate rpcauth string, given username and password:  https://jlopp.github.io/bitcoin-core-rpc-auth-generator/.

example deploying digital ocean production environment:

`helmfile apply --set tag="v0.1.0" --environment prod-digitalocean`