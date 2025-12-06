import { Keypair } from "@solana/web3.js";

// Generate a new keypair
const keypair = Keypair.generate();

// Log the public key (address) in Base58 format
console.log(`The public key is: ${keypair.publicKey.toBase58()}`);

// Log the secret key (a Uint8Array of 64 bytes)
console.log(`${keypair.secretKey}`);