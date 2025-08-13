import * as anchor from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import BN from "bn.js";
import { TestSetup } from "./setup";

export class PDAHelper {
  public programId: PublicKey;
  constructor(programId: PublicKey) {
    this.programId = programId;
  }

  static getProtocolStatePda(programId: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_state")],
      programId
    );
  }

  getResearcherProfilePda(
    researcher: PublicKey
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("researcher"), researcher.toBuffer()],
      this.programId
    );
  }

  getResearchProposalPda(
    researcher: PublicKey,
    proposalId: number | BN
  ): [PublicKey, number] {
    const idBn = BN.isBN(proposalId) ? (proposalId as BN) : new BN(proposalId);
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("proposal"),
        researcher.toBuffer(),
        idBn.toArrayLike(Buffer, "le", 8),
      ],
      this.programId
    );
  }

  async getNextProposalPdaFromProfile(
    researchDao: any,
    researcher: PublicKey,
    researcherProfilePda: PublicKey
  ): Promise<[PublicKey, number]> {
    const profile = await researchDao.account.researcherProfile.fetch(researcherProfilePda);
    const nextId = new BN(profile.totalProposals);
    return this.getResearchProposalPda(researcher, nextId);
  }
}

export class TestHelpers {
  private setup: TestSetup;
  constructor(setup: TestSetup) {
    this.setup = setup;
  }

  async createResearcherWithProfile(
    researcherKeypair: Keypair,
    researcherData: { name: string; affiliation: string; specialization: string; contactInfo?: string }
  ): Promise<[PublicKey, number]> {
    // Fund account
    const sig = await this.setup.provider.connection.requestAirdrop(
      researcherKeypair.publicKey,
      2 * 1_000_000_000
    );
    await this.setup.provider.connection.confirmTransaction(sig, "confirmed");

    const pdaHelper = new PDAHelper(this.setup.researchDao.programId);
    const [profilePda, bump] = pdaHelper.getResearcherProfilePda(researcherKeypair.publicKey);

    await this.setup.researchDao.methods
      .createResearcherProfile(
        researcherData.name,
        `${researcherData.affiliation}${researcherData.contactInfo ? " | " + researcherData.contactInfo : ""}`,
        researcherData.specialization
      )
      .accounts({
        researcherProfile: profilePda,
        researcher: researcherKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
  } as any)
      .signers([researcherKeypair])
      .rpc();

    return [profilePda, bump];
  }

  async verifyResearcher(
    profilePda: PublicKey,
    authority?: Keypair
  ): Promise<void> {
    const signer = authority || this.setup.authority;

    await this.setup.researchDao.methods
      .verifyResearcher()
      .accounts({
        researcherProfile: profilePda,
  authority: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
  } as any)
      .signers([signer])
      .rpc();
  }

  getCurrentTimestamp(): BN {
    return new BN(Math.floor(Date.now() / 1000));
  }

  getFutureTimestamp(daysFromNow: number): BN {
    const secondsFromNow = daysFromNow * 24 * 60 * 60;
    return new BN(Math.floor(Date.now() / 1000) + secondsFromNow);
  }

  async expectError(
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
