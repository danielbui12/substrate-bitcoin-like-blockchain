# FAQ

### How do I get `sigscript` ?

Sigscript is signed data from _Alice_. Look at `runtime/src/utxo.rs:346` and `scripts/generate-signature/generate-signature.js:14` to explore how it works.

### How does it maintain balance of wallet?

Every time a wallet makes transfer UTXO, the `input(s) UTXO` will be removed, replaced for `outputs UTXO`. Please read the [scenario](demo-usage.md).

In this case:
- Alice UTXO balance (0xdc25c09de55abb8ea4c3d53bd1ca5c26e0501db8cede096d8328cb482fda935a) was removed
- After invoke `spend` extrinsic, state contains 2 new UTXOs, 1 for Alice rest UTXO balance, 1 for Bob new UTXO balance




