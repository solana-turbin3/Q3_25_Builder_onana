import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { AgroDao } from "../../target/types/agro_dao";
import { ResearchDao } from "../../target/types/research_dao";

export interface TestSetup {
  provider: anchor.AnchorProvider;
  agroDao: Program<AgroDao>;
  researchDao: Program<ResearchDao>;
  authority: Keypair;
  protocolStatePda: PublicKey;
  protocolStateBump: number;
}

export async function setupTestEnvironment(): Promise<TestSetup> {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const agroDao = anchor.workspace.AgroDao as Program<AgroDao>;
  const researchDao = anchor.workspace.ResearchDao as Program<ResearchDao>;
  
  const authority = (provider as anchor.AnchorProvider).wallet.payer;
  
  const [protocolStatePda, protocolStateBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("protocol_state")],
    agroDao.programId
  );

  return {
    provider,
    agroDao,
    researchDao,
    authority,
    protocolStatePda,
    protocolStateBump,
  };
}

export async function fundWallet(
  provider: anchor.AnchorProvider,
  publicKey: PublicKey,
  amount: number = 2 * LAMPORTS_PER_SOL
): Promise<void> {
  const signature = await provider.connection.requestAirdrop(publicKey, amount);
  await provider.connection.confirmTransaction(signature, "confirmed");
}

export async function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// Backward-compat alias used by some tests
export const delay = sleep;
