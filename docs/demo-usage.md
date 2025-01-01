## Adding custom type

Head over [Polkadot JS](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/settings/developer), and then add this to additional types as a json file

```json
{
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "Value": "u128",
  "TransactionInput": {
    "outpoint": "Hash",
    "sigscript": "H512"
  },
  "TransactionOutput": {
    "value": "Value",
    "pubkey": "Hash"
  },
  "Transaction": {
    "inputs": "Vec<TransactionInput>",
    "outputs": "Vec<TransactionOutput>"
  },
  "Difficulty": "U256",
  "DifficultyAndTimestamp": {
    "difficulty": "Difficulty",
    "timestamp": "Moment"
  },
  "Public": "H256"
}
```



## Pre-install packages

## Address

- Convert `SS58 account` to `sr25519::Pubkey`: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/utilities
- Alice: from `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY` to `0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d`
- Bob: from `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty` to `0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48` 

```
# faucet
to: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
amount: 100

# event
hash: 0xdc25c09de55abb8ea4c3d53bd1ca5c26e0501db8cede096d8328cb482fda935a
```

```
outpoint: 0xdc25c09de55abb8ea4c3d53bd1ca5c26e0501db8cede096d8328cb482fda935a
sigscript: 0x68bde80b38ff66e6765e765933eda5f095037edc01d822681900e4d0f03c490c59fda186ebddb92b54b2083292fbde4e0b10cc2a26d1f2a09897eb66a609b083
value: 50
pubkey: 0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
```