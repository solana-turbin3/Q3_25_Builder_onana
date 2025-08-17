const anchor = require("@coral-xyz/anchor");
const { Connection, PublicKey } = require("@solana/web3.js");

async function checkCorrectPDA() {
  try {
    const connection = new Connection("https://api.devnet.solana.com");
    const agroDAOProgramId = new PublicKey("HWjwngNibn1coAzqLZhg4huw5pH5gNZY8zxJaK7s3Hbj");
    
    // Use the same PDA derivation as in the tests (looking at setup.ts)
    const [protocolStatePda, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      agroDAOProgramId
    );
    
    console.log("📍 Correct Protocol State PDA:", protocolStatePda.toString());
    console.log("📍 Bump:", bump);
    
    const accountInfo = await connection.getAccountInfo(protocolStatePda);
    if (accountInfo) {
      console.log("✅ Protocol account exists");
      console.log("📊 Account data length:", accountInfo.data.length);
      console.log("🔍 Owner:", accountInfo.owner.toString());
      
      // Check if it matches the AgroDAO program
      if (accountInfo.owner.equals(agroDAOProgramId)) {
        console.log("✅ Account owned by AgroDAO program");
        
        // Try to parse the pause state (it should be around byte 40-50 based on struct layout)
        const data = accountInfo.data;
        console.log("📋 Account data (hex):", data.toString('hex'));
        
        // Look for the isPaused boolean (likely at the end after other fields)
        const lastBytes = data.slice(-20);
        console.log("🔍 Last 20 bytes (where isPaused might be):", lastBytes.toString('hex'));
      } else {
        console.log("❌ Account not owned by AgroDAO program");
      }
    } else {
      console.log("❌ Protocol account not found at this PDA");
    }
  } catch (error) {
    console.error("Error:", error.message);
  }
}

checkCorrectPDA();
