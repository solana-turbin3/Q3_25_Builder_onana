import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet } from "../utils/setup";
import { TEST_CONSTANTS } from "../utils/constants";

describe("AgroDao - Protocol Error Conditions", () => {
  let setup: TestSetup;
  let unauthorizedUser: Keypair;

  before(async () => {
    setup = await setupTestEnvironment();
    unauthorizedUser = Keypair.generate();
    await fundWallet(setup.provider, unauthorizedUser.publicKey);

    // Initialize protocol for testing
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
    } catch {
      // Protocol already initialized
    }
  });

  describe("Initialization Errors", () => {
    it("Should prevent multiple protocol initializations", async () => {
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

        expect.fail("Should have thrown error for duplicate initialization");
      } catch (error) {
        expect(error.message).to.include("already in use");
      }
    });
  });

  describe("Authority Errors", () => {
    it("Should reject protocol updates from unauthorized users", async () => {
      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: new anchor.BN(5000),
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

        expect.fail("Should have thrown error for unauthorized update");
      } catch (error) {
        expect(error.message).to.include("UnauthorizedUpdate");
      }
    });

    it("Should reject authority transfer from non-authority", async () => {
      const newAuthority = Keypair.generate();
      await fundWallet(setup.provider, newAuthority.publicKey);

      try {
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
            authority: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unauthorizedUser])
          .rpc();

        expect.fail("Should have thrown error for unauthorized authority transfer");
      } catch (error) {
        expect(error.message).to.include("UnauthorizedUpdate");
      }
    });
  });

  describe("Parameter Validation Errors", () => {
    it("Should reject invalid minimum funding threshold", async () => {
      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: new anchor.BN(0), // Invalid: zero threshold
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

        expect.fail("Should have thrown error for invalid funding threshold");
      } catch (error) {
        expect(error.message).to.include("InvalidParameterValue");
      }
    });

    it("Should reject invalid research proposal fee", async () => {
      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: null,
            researchProposalFee: new anchor.BN(0), // Invalid: zero fee
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

        expect.fail("Should have thrown error for invalid proposal fee");
      } catch (error) {
        expect(error.message).to.include("InvalidParameterValue");
      }
    });

    it("Should reject invalid minimum staked amount", async () => {
      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: null,
            researchProposalFee: null,
            minimumStakedAmount: new anchor.BN(0), // Invalid: zero stake
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

        expect.fail("Should have thrown error for invalid staked amount");
      } catch (error) {
        expect(error.message).to.include("InvalidParameterValue");
      }
    });

    it("Should reject excessively large parameter values", async () => {
      const maxU64 = new anchor.BN("18446744073709551615"); // Max u64

      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: maxU64,
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

        expect.fail("Should have thrown error for excessive parameter value");
      } catch (error) {
        expect(error.message).to.include("InvalidParameterValue");
      }
    });
  });

  describe("State Consistency Errors", () => {
    it("Should handle no-update scenario gracefully", async () => {
      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: null,
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

        expect.fail("Should have thrown error for no-update scenario");
      } catch (error) {
        expect(error.message).to.include("NoUpdatesProvided");
      }
    });

    it("Should handle protocol state corruption gracefully", async () => {
      // This test simulates scenarios where the protocol state might be in an inconsistent state
      const protocolState = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      // Verify state is consistent after all operations
      expect(protocolState.authority).to.exist;
      expect(protocolState.protocolVersion).to.be.greaterThan(0);
      expect(protocolState.minFundingThreshold.toNumber()).to.be.greaterThan(0);
      expect(protocolState.researchProposalFee.toNumber()).to.be.greaterThan(0);
      expect(protocolState.minimumStakedAmount.toNumber()).to.be.greaterThan(0);
    });
  });

  describe("Account Validation Errors", () => {
    it("Should reject invalid protocol state PDA", async () => {
      const [invalidPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("invalid"), setup.authority.publicKey.toBuffer()],
        setup.agroDao.programId
      );

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
            protocolState: invalidPda,
            authority: setup.authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([setup.authority])
          .rpc();

        expect.fail("Should have thrown error for invalid PDA");
      } catch (error) {
        expect(error.message).to.include("AccountNotInitialized");
      }
    });

    it("Should reject incorrect system program", async () => {
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
            authority: setup.authority.publicKey,
            systemProgram: setup.agroDao.programId, // Wrong program
          })
          .signers([setup.authority])
          .rpc();

        expect.fail("Should have thrown error for incorrect system program");
      } catch (error) {
        expect(error.message).to.include("InvalidProgramId");
      }
    });
  });

  describe("State Validation", () => {
    it("Should maintain state consistency after errors", async () => {
      const protocolStateBefore = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      // Attempt an invalid operation
      try {
        await setup.agroDao.methods
          .updateProtocol({
            minFundingThreshold: new anchor.BN(0),
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
      } catch {
        // Expected to fail
      }

      const protocolStateAfter = await setup.agroDao.account.protocolState.fetch(
        setup.protocolStatePda
      );

      // Verify state hasn't changed after failed operation
      expect(protocolStateAfter.authority.toString()).to.equal(
        protocolStateBefore.authority.toString()
      );
      expect(protocolStateAfter.protocolVersion).to.equal(
        protocolStateBefore.protocolVersion
      );
      expect(protocolStateAfter.minFundingThreshold.toString()).to.equal(
        protocolStateBefore.minFundingThreshold.toString()
      );
    });
  });
});
