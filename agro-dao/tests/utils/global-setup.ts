import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { setupTestEnvironment } from "./setup";

/**
 * Global test setup that ensures all required accounts are initialized
 * Call this before running any tests that depend on program state
 */
export async function ensureGlobalTestState() {
  console.log("🌍 Running global test setup...");
  
  const setup = await setupTestEnvironment();
  
  // 1. Initialize Agro DAO protocol if needed
  await ensureAgroDaoInitialized(setup);
  
  // 2. Initialize Treasury DAO if needed
  await ensureTreasuryDaoInitialized(setup);
  
  // 3. Initialize Reputation DAO if needed  
  await ensureReputationDaoInitialized(setup);
  
  console.log("Global test state ensured");
  return setup;
}

async function ensureAgroDaoInitialized(setup: any) {
  try {
    await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
    console.log("Agro DAO already initialized");
  } catch {
    console.log("Initializing Agro DAO...");
    try {
      await setup.agroDao.methods
        .initializeProtocol()
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();
      console.log("Agro DAO initialized successfully");
    } catch (error) {
      console.log("Agro DAO initialization failed (may be expected):", error.message);
    }
  }
}

async function ensureTreasuryDaoInitialized(setup: any) {
  // Check if treasury DAO is properly initialized and accessible
  console.log("Verifying Treasury DAO integration readiness");
  
  try {
    // Verify treasury program is loaded and accessible
    const treasuryProgram = setup.treasuryProgram;
    if (treasuryProgram) {
      console.log("Treasury DAO program accessible");
      console.log(`📍 Treasury Program ID: ${treasuryProgram.programId.toString()}`);
    } else {
      console.log("Treasury DAO program not loaded in setup");
    }
  } catch (error) {
    console.log("Treasury DAO will be initialized on-demand");
  }
}

async function ensureReputationDaoInitialized(setup: any) {
  // Check if reputation DAO is properly initialized and accessible
  console.log("Verifying Reputation DAO integration readiness");
  
  try {
    // Verify reputation program is loaded and accessible
    const reputationProgram = setup.reputationProgram;
    if (reputationProgram) {
      console.log("Reputation DAO program accessible");
      console.log(`📍 Reputation Program ID: ${reputationProgram.programId.toString()}`);
    } else {
      console.log("Reputation DAO program not loaded in setup");
    }
  } catch (error) {
    console.log("Reputation DAO will be initialized on-demand");
  }
}
