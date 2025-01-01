## Pre-install packages

## Address

- Convert `SS58 account` to `sr25519::Pubkey`: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/utilities
- Alice: from `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY` to `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`
- Bob: from `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty` to `0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48` 

## Steps

### 1. Alice faucets UTXO 100 unit

_image here_

**Input**
- `to`: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d 
- `amount`: 100

**Submit signed transaction**

_image here_

**Event**

_image here_

- `hash`: 0xdc25c09de55abb8ea4c3d53bd1ca5c26e0501db8cede096d8328cb482fda935a


### 2. Alice transfers UTXO 50 unit to Bob, get back UXTO 50 unit

**Input**

- `inputs[0].outpoint`: 0xdc25c09de55abb8ea4c3d53bd1ca5c26e0501db8cede096d8328cb482fda935a
- `inputs[0].sigscript`: 0x68bde80b38ff66e6765e765933eda5f095037edc01d822681900e4d0f03c490c59fda186ebddb92b54b2083292fbde4e0b10cc2a26d1f2a09897eb66a609b083

- `outputs[0].value`: 50
- `outputs[0].pubkey`: 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
- `outputs[1].value`: 50
- `outputs[1].pubkey`: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d

_image here_

**Submit unsigned transaction**

_image here_

**Events**

_image here_


## FAQ

### How do I get `sigscript` ?

Sigscript is signed data from _Alice_. Look at `runtime/src/utxo.rs:346` and `scripts/generate-signature/generate-signature.js:14` to explore how it works.

### How does it maintain balance of wallet?

Every time a wallet makes transfer UTXO, the `input(s) UTXO` will be removed, replaced for `outputs UTXO`. Read [the scenarios and flow](TODO).


In this case:
- Alice UTXO balance (0xdc25c09de55abb8ea4c3d53bd1ca5c26e0501db8cede096d8328cb482fda935a) was removed
- After invoke `spend` extrinsic, state contains 2 new UTXOs, 1 for Alice rest UTXO balance, 1 for Bob new UTXO balance




