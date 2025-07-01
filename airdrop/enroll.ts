import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
// Make sure this path is correct for your project structure
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
// Make sure you have created this file and it contains your secret key
import wallet from "./Turbin3-wallet.json";

// --- Constants ---
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const mintTs = Keypair.generate(); 
const mintCollection = new 
PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"); 

// --- Setup ---

// Import our dev wallet keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet.wallet));

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com", "confirmed");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

// Create our program
const program: Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const account_seeds = [
  Buffer.from("prereqs"),
  keypair.publicKey.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(
  account_seeds,
  program.programId
);

// --- Transaction Execution ---
(async () => {
  try {
    console.log("Initializing account...");
    const txhash = await program.methods
      // 1. Pass your GitHub username as the argument
      .initialize("ENuel20")
      // 2. Use the .accounts() method
      .accounts({
        user: keypair.publicKey,
        account: account_key,
        // 3. Use the correctly imported SystemProgram.programId
        systemProgram: SystemProgram.programId,
      })
      // 4. Sign with the keypair that is paying for the transaction
      .signers([keypair])
      .rpc();
      
    console.log(`Success! Your account has been initialized.`);
    console.log(`Check out your transaction here:`);
    console.log(`https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();

// Execute the submitTs transaction 
(async () => { 
    try { 
        const txhash = await program.methods 
            .submitTs() 
            .accounts({ 
                user: keypair.publicKey, 
                account: account_key, 
                mint: mintTs.publicKey, 
                collection: mintCollection, 
                mpl_core_program: MPL_CORE_PROGRAM_ID, 
                system_program: SystemProgram.programId, 
            }) 
            .signers([keypair, mintTs]) 
            .rpc(); 
        console.log(`Success! Check out your TX here: 
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();