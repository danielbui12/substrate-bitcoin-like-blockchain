# How to run the node

## Building the Node

First you will need a [Substrate developer environment](https://docs.substrate.io/install/).

1. Cloning this repo
2. Running `cargo build --release`

## Run a Single Development Node

You can use a native binary if you built it in the previous section. Otherwise you can use Docker.

```sh
./target/release/academy-pow --dev --tmp --mining-algo md5
```

## MultiNode Testnet

When using the local networking, you can use the `--discover-local` flag to discover peers on your local network.

```sh
# Start the first node.
# Same as a single node network above.
./target/release/academy-pow --dev --mining-algo md5

# Start the second node
# Choose a non-default rpc port because the default port is taken by the first node.
./target/release/academy-pow --dev --mining-algo md5 --rpc-port 9944 --discover-local
```

## More Help

```sh
academy-pow --help
```