import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { setupTestEnvironment, TestSetup } from "../utils/setup";
import { TEST_CONSTANTS } from "../utils/constants";

describe("AgroDao - Protocol Initialization", () => {
  let setup: TestSetup;

  before(async () => {
    setup = await setupTestEnvironment();
  });

  it("Should initialize protocol successfully", async () => {
    console.log("Attempting to initialize protocol...");
    console.log("📍 Protocol State PDA:", setup.protocolStatePda.toString());
    console.log("📍 Authority:", setup.authority.publicKey.toString());
    console.log("📍 Agro DAO Program ID:", setup.agroDao.programId.toString());

    // Check if protocol already exists
    try {
      const existingProtocol = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );
      console.log("Protocol already initialized, skipping initialization test");
      console.log("Existing protocol state:", {
        authority: existingProtocol.authority.toString(),
        version: existingProtocol.protocolVersion,
        isPaused: existingProtocol.isPaused
      });
      return;
    } catch (error) {
      console.log("Protocol doesn't exist, proceeding with initialization");
      console.log("Error details:", error.message);
    }

    try {
      const tx = await setup.agroDao.methods
        .initializeProtocol()
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      console.log("Protocol initialized successfully, TX:", tx);
    } catch (error) {
      console.log("Protocol initialization failed:", error.message);
      throw error;
    }

    // Verify initialization
    const protocolState = await setup.agroDao.account.protocolState.fetch(
      setup.protocolStatePda
    );

    expect(protocolState.authority.toString()).to.equal(
      setup.authority.publicKey.toString()
    );
    expect(protocolState.proposalIdCounter.toNumber()).to.equal(0);
    expect(protocolState.minFundingThreshold.toNumber()).to.equal(0);
    expect(protocolState.researchProposalFee.toNumber()).to.equal(0);
    expect(protocolState.minimumStakedAmount.toNumber()).to.equal(0);
    expect(protocolState.protocolVersion).to.equal(1);
    expect(protocolState.isPaused).to.be.false;
    expect(protocolState.researchDataCounter.toNumber()).to.equal(0);
  });

  it("Should prevent duplicate initialization", async () => {
    // Ensure protocol is initialized first
    try {
      const protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );
      // Protocol exists, test duplicate initialization
    } catch {
      // Initialize first
      await setup.agroDao.methods
        .initializeProtocol()
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();
    }

    // Attempt duplicate initialization
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
      
      expect.fail("Should have thrown an error for duplicate initialization");
    } catch (error) {
      expect(error.message).to.include("already in use");
    }
  });

  it("Should verify initial protocol state", async () => {
    const protocolState = await setup.agroDao.account.protocolState.fetch(
      setup.protocolStatePda
    );

    console.log("Initial Protocol State:");
    console.log("- Authority:", protocolState.authority.toString());
    console.log("- Proposal ID Counter:", protocolState.proposalIdCounter.toNumber());
    console.log("- Min Funding Threshold:", protocolState.minFundingThreshold.toNumber());
    console.log("- Research Proposal Fee:", protocolState.researchProposalFee.toNumber());
    console.log("- Minimum Staked Amount:", protocolState.minimumStakedAmount.toNumber());
    console.log("- Protocol Version:", protocolState.protocolVersion);
    console.log("- Is Paused:", protocolState.isPaused);
    console.log("- Research Data Counter:", protocolState.researchDataCounter.toNumber());

    // Verify default values
    expect(protocolState.proposalIdCounter.toNumber()).to.equal(0);
    expect(protocolState.protocolVersion).to.equal(1);
    expect(protocolState.isPaused).to.be.false;
    expect(protocolState.researchDataCounter.toNumber()).to.equal(0);
  });
});
