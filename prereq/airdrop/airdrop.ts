import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

// Recreate the Keypair from the secret key
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Connect to Solana devnet
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    // Request 2 SOL airdrop
    const txhash = await connection.requestAirdrop(
      keypair.publicKey,
      2 * LAMPORTS_PER_SOL
    );

    console.log(`Success! Check out your TX here:`);
    console.log(`https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error("Oops, something went wrong:", e);
  }
})();

// https://explorer.solana.com/tx/4WgXj1NMatwvG2srJAiqLCkvSDssSGFKFF5kfJWXfMjLX4esJTQgzuXpcweDoiktZkKLYn1mMe3shL681ouxgN7j?cluster=devnet