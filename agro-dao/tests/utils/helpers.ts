import { PublicKey, Keypair } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import { TestSetup } from "./setup";

export class PDAHelper {
  static getProtocolStatePda(programId: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_state")],
      programId
    );
  }

  static getResearcherProfilePda(
    researcher: PublicKey,
    programId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("researcher"), researcher.toBuffer()],
      programId
    );
  }

  static getResearchProposalPda(
    researcher: PublicKey,
    proposalId: BN,
    programId: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("proposal"),
        researcher.toBuffer(),
        proposalId.toArrayLike(Buffer, "le", 8),
      ],
      programId
    );
  }
}

export class TestHelpers {
  static async createResearcherWithProfile(
    setup: TestSetup,
    name: string = "Test Researcher",
    bio: string = "Test bio",
    specialization: string = "Test specialization"
  ): Promise<{ researcher: Keypair; profilePda: PublicKey }> {
    const researcher = Keypair.generate();
    
    // Fund researcher account
    const signature = await setup.provider.connection.requestAirdrop(
      researcher.publicKey,
      2 * 1_000_000_000 // 2 SOL
    );
    await setup.provider.connection.confirmTransaction(signature, "confirmed");

    const [profilePda] = PDAHelper.getResearcherProfilePda(
      researcher.publicKey,
      setup.researchDao.programId
    );

    await setup.researchDao.methods
      .createResearcherProfile(name, bio, specialization)
      .accounts({
        researcherProfile: profilePda,
        researcher: researcher.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([researcher])
      .rpc();

    return { researcher, profilePda };
  }

  static async verifyResearcher(
    setup: TestSetup,
    profilePda: PublicKey,
    authority?: Keypair
  ): Promise<void> {
    const signer = authority || setup.authority;
    
    await setup.researchDao.methods
      .verifyResearcher()
      .accounts({
        researcherProfile: profilePda,
        authority: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([signer])
      .rpc();
  }

  static getCurrentTimestamp(): BN {
    return new BN(Math.floor(Date.now() / 1000));
  }

  static getFutureTimestamp(daysFromNow: number): BN {
    const secondsFromNow = daysFromNow * 24 * 60 * 60;
    return new BN(Math.floor(Date.now() / 1000) + secondsFromNow);
  }

  static async expectError(
    promise: Promise<any>,
    expectedErrorMessage: string
  ): Promise<void> {
    try {
      await promise;
      throw new Error("Expected an error to be thrown");
    } catch (error) {
      if (!error.message.includes(expectedErrorMessage)) {
        throw new Error(
          `Expected error message to include "${expectedErrorMessage}", but got: ${error.message}`
        );
      }
    }
  }
}
