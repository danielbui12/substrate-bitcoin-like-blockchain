const { u8aToHex } = require('@polkadot/util');
const { Keyring } = require('@polkadot/keyring');
const { ApiPromise, WsProvider } = require('@polkadot/api');

const wsProvider = new WsProvider('ws://127.0.0.1:9944'); // Replace with your endpoint

async function main() {
    // await cryptoWaitReady();
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
    // Register types
    const api = await ApiPromise.create({
        types: {
            TransactionInput: {
                outpoint: 'H256',      // Fixed 32 bytes
                sigscript: 'H512'     // Fixed 64 bytes
            },
            TransactionOutput: {
                value: 'u128',        // 16 bytes
                pubkey: 'H256'        // Fixed 32 bytes
            },
            Transaction: {
                inputs: 'Vec<TransactionInput>',  // Vec type
                outputs: 'Vec<TransactionOutput>' // Vec type
            }
        },
        provider: wsProvider, // Add the provider here
    });

    // create Alice based on the development seed
    const alice = keyring.addFromUri('//Alice');

    const inputs = [{
        // the latest UTXO hash of the account want to spent
        outpoint: "0xdc25c09de55abb8ea4c3d53bd1ca5c26e0501db8cede096d8328cb482fda935a",
        // default simple sigscript
        sigscript: "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    }]
    const outputs = [
        {
            // Value to spend
            value: "50",
            // Bob pubkey
            pubkey: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48",
        },
        {
            // Value to spend
            value: "50",
            // Alice pubkey
            pubkey: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
        },
    ]
    // Encode full transaction
    const encodedTx = api.createType('Transaction', {
        inputs: inputs,
        outputs: outputs
    }).toU8a();
    
    // the encoded transaction in u8 array
    const signature = alice.sign(encodedTx);
    const isValid = alice.verify(encodedTx, signature, alice.publicKey);
    
    // output the result
    console.log(`${u8aToHex(signature)} is ${isValid ? 'valid' : 'invalid'}`);
}

main().then(() => process.exit(0)).catch(error => {
    console.error(error);
    process.exit(1);
});