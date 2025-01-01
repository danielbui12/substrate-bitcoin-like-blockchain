const { u8aToHex } = require('@polkadot/util');
const { Keyring } = require('@polkadot/keyring');
const { cryptoWaitReady } = require('@polkadot/util-crypto')

async function main() {
    await cryptoWaitReady();
    const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
    
    // create Alice based on the development seed
    const alice = keyring.addFromUri('//Alice');

    // the encoded transaction in u8 array
    const message = new Uint8Array([4, 220, 37, 192, 157, 229, 90, 187, 142, 164, 195, 213, 59, 209, 202, 92, 38, 224, 80, 29, 184, 206, 222, 9, 109, 131, 40, 203, 72, 47, 218, 147, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125]);
    const signature = alice.sign(message);
    const isValid = alice.verify(message, signature, alice.publicKey);
    
    // output the result
    console.log(`${u8aToHex(signature)} is ${isValid ? 'valid' : 'invalid'}`);
}

main();