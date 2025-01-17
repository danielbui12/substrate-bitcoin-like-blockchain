# Substrate Bitcoin-like Blockchain with UTXO account model

## Step 1: Set up environment

> [!Note]
> Make sure you have followed this [document](https://docs.substrate.io/install/) to set up rust, wasm environment.


- Build project:

```sh
cd substrate-bitcoin-like-blockchain
cargo build --release
```

- Run local blockchain node in development mode:

```sh
./target/release/academy-pow --dev --tmp


# Stdout:

2025-01-17 20:42:29 Academy PoW Chain    
2025-01-17 20:42:29 ✌️  version 3.0.0-d2db5382991    
2025-01-17 20:42:29 ❤️  by @danielbui12, 2025-2025    
2025-01-17 20:42:29 📋 Chain specification: Development    
2025-01-17 20:42:29 🏷  Node name: rapid-opinion-7325    
2025-01-17 20:42:29 👤 Role: AUTHORITY    
2025-01-17 20:42:29 💾 Database: ParityDb at /var/folders/n1/4b940c8s1qddng2m8xgxwb280000gn/T/substratebnsOsb/chains/dev/paritydb/full    
2025-01-17 20:42:29 🔨 Initializing Genesis block/state (state: 0x6118…d3ad, header-hash: 0xad3e…d8b4)    
2025-01-17 20:42:29 Using default protocol ID "sup" because none is configured in the chain specs    
2025-01-17 20:42:29 Local node identity is: 12D3KooWEWeNVgFshX9KJ2cKKJgz1UqmRMneA5vRAPgGdUNZ8J9Z    
2025-01-17 20:42:29 Running litep2p network backend    
2025-01-17 20:42:29 💻 Operating system: macos    
2025-01-17 20:42:29 💻 CPU architecture: aarch64    
2025-01-17 20:42:29 📦 Highest known block at #0    
2025-01-17 20:42:29 〽️ Prometheus exporter started at 127.0.0.1:9615    
2025-01-17 20:42:29 Running JSON-RPC server: addr=127.0.0.1:9944, allowed origins=["*"]    
2025-01-17 20:42:34 💤 Idle (0 peers), best: #0 (0xad3e…d8b4), finalized #0 (0xad3e…d8b4), ⬇ 0 ⬆ 0    
2025-01-17 20:42:39 💤 Idle (0 peers), best: #0 (0xad3e…d8b4), finalized #0 (0xad3e…d8b4), ⬇ 0 ⬆ 0    
2025-01-17 20:42:39 🙌 Starting consensus session on top of parent 0xad3e45470dd74feceeecf2e11a9bb18ed97c813c53af0160baea0b81d23fd8b4 (#0)    
2025-01-17 20:42:39 🎁 Prepared block for proposing at 1 (3 ms) [hash: 0x6d9c8eecd409c90365b22b5c3ad63f0d3e1a78466d79daa9b1b62bbee4c7fed6; parent_hash: 0xad3e…d8b4; extrinsics (2): [0x0bca…1ac2, 0x6591…3bcd]    
2025-01-17 20:42:40 🏆 Imported #1 (0xad3e…d8b4 → 0xa4f4…4268)    
2025-01-17 20:42:40 ✅ Successfully mined block on top of: 0xad3e…d8b4    
2025-01-17 20:42:40 🙌 Starting consensus session on top of parent 0xa4f476563afdedac4198e0f87b33e616f52e414e7eb0093687bbb566e3a64268 (#1)    
```


✅ Congrats, you've completed step 1. Let's move on step 2 by running this:

```sh
git checkout step-2-data-structure
```





