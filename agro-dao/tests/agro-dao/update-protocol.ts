import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet } from "../utils/setup";
import { TEST_CONSTANTS } from "../utils/constants";

describe("AgroDao - Protocol Updates", () => {
  let setup: TestSetup;
  let unauthorizedUser: Keypair;

  before(async () => {
    setup = await setupTestEnvironment();
    unauthorizedUser = Keypair.generate();
    await fundWallet(setup.provider, unauthorizedUser.publicKey);

    // Ensure protocol is initialized
    try {
      await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
    } catch {
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
  });

  describe("Parameter Updates", () => {
    it("Should update minimum funding threshold", async () => {
      const newThreshold = TEST_CONSTANTS.MIN_FUNDING_THRESHOLD;

      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: newThreshold,
          researchProposalFee: null,
          minimumStakedAmount: null,
          isPaused: null,
          newAuthority: null,
        })
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      const protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      expect(protocolState.minFundingThreshold.toString()).to.equal(
        newThreshold.toString()
      );
      expect(protocolState.protocolVersion).to.equal(2);
    });

    it("Should update research proposal fee", async () => {
      const newFee = TEST_CONSTANTS.RESEARCH_PROPOSAL_FEE;

      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: newFee,
          minimumStakedAmount: null,
          isPaused: null,
          newAuthority: null,
        })
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      const protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      expect(protocolState.researchProposalFee.toString()).to.equal(
        newFee.toString()
      );
      expect(protocolState.protocolVersion).to.equal(3);
    });

    it("Should update minimum staked amount", async () => {
      const newStakeAmount = TEST_CONSTANTS.MINIMUM_STAKED_AMOUNT;

      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: null,
          minimumStakedAmount: newStakeAmount,
          isPaused: null,
          newAuthority: null,
        })
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      const protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      expect(protocolState.minimumStakedAmount.toString()).to.equal(
        newStakeAmount.toString()
      );
      expect(protocolState.protocolVersion).to.equal(4);
    });

    it("Should pause and unpause protocol", async () => {
      // Pause protocol
      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: null,
          minimumStakedAmount: null,
          isPaused: true,
          newAuthority: null,
        })
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      let protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      expect(protocolState.isPaused).to.be.true;

      // Unpause protocol
      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: null,
          minimumStakedAmount: null,
          isPaused: false,
          newAuthority: null,
        })
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      expect(protocolState.isPaused).to.be.false;
    });
  });

  describe("Authority Management", () => {
    it("Should transfer authority", async () => {
      const newAuthority = Keypair.generate();
      await fundWallet(setup.provider, newAuthority.publicKey);

      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: null,
          minimumStakedAmount: null,
          isPaused: null,
          newAuthority: newAuthority.publicKey,
        })
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      const protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      expect(protocolState.authority.toString()).to.equal(
        newAuthority.publicKey.toString()
      );

      // Update authority for subsequent tests
      setup.authority = newAuthority;
    });

    it("Should reject unauthorized updates", async () => {
      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: TEST_CONSTANTS.MIN_FUNDING_THRESHOLD,
            researchProposalFee: null,
            minimumStakedAmount: null,
            isPaused: null,
            newAuthority: null,
          })
          .accounts({
            protocolState: setup.protocolStatePda,
            authority: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unauthorizedUser])
          .rpc();

        expect.fail("Should have thrown an error for unauthorized update");
      } catch (error) {
        expect(error.message).to.include("UnauthorizedUpdate");
      }
    });
  });

  describe("Final State Verification", () => {
    it("Should verify final protocol state", async () => {
      const protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      console.log("Final Protocol State:");
      console.log("- Authority:", protocolState.authority.toString());
      console.log("- Min Funding Threshold:", protocolState.minFundingThreshold.toString());
      console.log("- Research Proposal Fee:", protocolState.researchProposalFee.toString());
      console.log("- Minimum Staked Amount:", protocolState.minimumStakedAmount.toString());
      console.log("- Is Paused:", protocolState.isPaused);
      console.log("- Protocol Version:", protocolState.protocolVersion);

      // Verify the protocol has been updated multiple times
      expect(protocolState.protocolVersion).to.be.greaterThan(4);
      expect(protocolState.isPaused).to.be.false;
      expect(protocolState.minFundingThreshold.toString()).to.equal(
        TEST_CONSTANTS.MIN_FUNDING_THRESHOLD.toString()
      );
    });
  });
});
