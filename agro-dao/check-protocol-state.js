const anchor = require("@coral-xyz/anchor");
const { Connection, PublicKey } = require("@solana/web3.js");

async function checkProtocolState() {
  try {
    const connection = new Connection("https://api.devnet.solana.com");
    const programId = new PublicKey("HWjwngNibn1coAzqLZhg4huw5pH5gNZY8zxJaK7s3Hbj");
    
    // Derive protocol state PDA
    const [protocolStatePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("protocol")],
      programId
    );
    
    console.log("📍 Protocol State PDA:", protocolStatePda.toString());
    
    const accountInfo = await connection.getAccountInfo(protocolStatePda);
    if (accountInfo) {
      console.log("✅ Protocol account exists");
      console.log("📊 Account data length:", accountInfo.data.length);
      
      // Simple parsing to check pause state (assuming it's at a specific offset)
      const data = accountInfo.data;
      console.log("🔍 Raw data (first 100 bytes):", data.slice(0, 100).toString('hex'));
    } else {
      console.log("❌ Protocol account not found");
    }
  } catch (error) {
    console.error("Error:", error.message);
  }
}

checkProtocolState();
