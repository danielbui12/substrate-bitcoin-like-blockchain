# Substrate Bitcoin-like Blockchain with UTXO account model

## Academy-PoW

This repository is generated from [Academy-PoW](https://github.com/Polkadot-Blockchain-Academy/Academy-PoW) repository. 


However, during implement Bitcoin-like blockchain, I've to update and fix a lot to make sure to code run smoothly.


Significantly, I've bumped Polkadot-SDK to stable-2407. That's quite complected but I've made it thanks to comprehensive document of OpenGuild's Polkadot SDK course.
- [Bump Polkadot SDK versions lesson doc.](https://bootcamp.openguild.wtf/building-a-blockchain-with-polkadot-sdk/polkadot-sdk/substrate/bump-polkadot-sdk-versions)
- [Bump Polkadot SDK versions lesson video.](https://www.youtube.com/watch?v=6nhIZmE1Nck&list=PLnhzaKpksqOKiqu9DDjGnmZWB0hYTaOUC&index=15)
- My commits: [be5c66](https://github.com/danielbui12/substrate-bitcoin-like-blockchain/commit/be5c665779dc23e2ac2c710ce5ada95975b4d5d2)

## Proof of Work

- Discover [Proof-of-Work (PoW)](docs/pow/pow.md)
- Understanding overview of PoW in Substrate by reading [code breakdown](docs/pow/code-breakdown.md).


## UTXO

- Discover [What is UTXO account model?](docs/utxo/utxo.md)
- Understanding overview of UTXO in Substrate by reading [code breakdown](docs/utxo/code-breakdown.md).


## Prerequisites

Before proceeding with this interactive tutorial, make sure you complete the initial tutorials on Substrate development from the official documentation. If you haven't gone through them yet, please review them first to build a solid foundation.

- [Substrate develop environment: Select your environment and complete `Install required packages and Rust`)](https://docs.substrate.io/install/)
- [Substrate Tutorial](https://polkadot.study/tutorials/interactive-substrate-tutorials-rusty-crewmates/)
- [Polkadot SDK doc](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/index.html)
- [Substrate simulate a network](https://docs.substrate.io/tutorials/build-a-blockchain/simulate-network/)


## How to run the node

- Refer to [How to run the node](docs/how-to-run-the-node.md) to choose development environment and boost up your first local blockchain node.

## Demo usage

- Please follow exactly these [instructions](docs/demo-usage.md) to get best experience.


## Build your own UTXO account model

Let's build your own Bitcoin-like blockchain with UTXO account model.

```sh
git checkout start_here
cargo build --release
```

## FAQ

- Read the [FAQ](docs/faq.md)