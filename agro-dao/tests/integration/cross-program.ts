import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet } from "../utils/setup";
import { TEST_RESEARCHERS, TEST_PROPOSALS } from "../utils/constants";
import { PDAHelper, TestHelpers } from "../utils/helpers";

describe("Integration Tests - Cross-Program Interactions", () => {
  let setup: TestSetup;
  let pdaHelper: PDAHelper;
  let testHelpers: TestHelpers;

  before(async () => {
    setup = await setupTestEnvironment();
    pdaHelper = new PDAHelper(setup.researchDao.programId);
  testHelpers = new TestHelpers(setup);
  });

  describe("Protocol-Research Integration", () => {
    it("Should enforce protocol parameters in research operations", async () => {
      // Initialize protocol with specific parameters
      try {
        await setup.agroDao.methods
          .initializeProtocol()
          .accounts({
            protocolState: setup.protocolStatePda,
            authority: setup.authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([setup.authority])
          .rpc();
      } catch {
        // Protocol already initialized
      }

      const protocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      console.log("Protocol Parameters:");
      console.log("- Min Funding Threshold:", protocolState.minFundingThreshold.toString());
      console.log("- Research Proposal Fee:", protocolState.researchProposalFee.toString());
      console.log("- Minimum Staked Amount:", protocolState.minimumStakedAmount.toString());

      // Create researcher and proposal
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);

  const [profilePda] = await testHelpers.createResearcherWithProfile(researcher, TEST_RESEARCHERS.ALICE);
  await testHelpers.verifyResearcher(profilePda, setup.authority);

      // Test proposal with funding below threshold (if implemented)
    const [proposalPda] = await pdaHelper.getNextProposalPdaFromProfile(setup.researchDao as any, researcher.publicKey, profilePda);
      const proposal = TEST_PROPOSALS.DROUGHT_RESISTANCE;

      await setup.researchDao.methods
        .createProposal(
          proposal.title,
          proposal.description,
          proposal.category,
          proposal.fundingGoal,
          proposal.duration,
          proposal.milestones,
          proposal.ipfsHash
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      const createdProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
  expect(createdProposal.fundingTarget.gte(protocolState.minFundingThreshold)).to.be.true;
    });

    it("Should handle protocol updates affecting research operations", async () => {
      const protocolStateBefore = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      const newThreshold = protocolStateBefore.minFundingThreshold.add(new anchor.BN(50000));

      // Update protocol parameters
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
  } as any)
        .signers([setup.authority])
        .rpc();

      const protocolStateAfter = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      expect(protocolStateAfter.minFundingThreshold.toString()).to.equal(newThreshold.toString());

      // Test that new proposals should respect updated threshold
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);

      const [profilePda] = await testHelpers.createResearcherWithProfile(
        researcher,
        TEST_RESEARCHERS.BOB
      );
      await testHelpers.verifyResearcher(profilePda, setup.authority);

      // Create proposal with funding goal meeting new threshold
    const [proposalPda] = await pdaHelper.getNextProposalPdaFromProfile(setup.researchDao as any, researcher.publicKey, profilePda);
      const proposal = {
        ...TEST_PROPOSALS.SOIL_HEALTH,
        fundingGoal: newThreshold.add(new anchor.BN(10000))
      };

      await setup.researchDao.methods
        .createProposal(
          proposal.title,
          proposal.description,
          proposal.category,
          proposal.fundingGoal,
          proposal.duration,
          proposal.milestones,
          proposal.ipfsHash
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      const createdProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
  expect(createdProposal.fundingTarget.gte(protocolStateAfter.minFundingThreshold)).to.be.true;
    });

    it("Should handle protocol pause affecting research operations", async () => {
      // Pause the protocol
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
  } as any)
        .signers([setup.authority])
        .rpc();

      const pausedProtocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      expect(pausedProtocolState.isPaused).to.be.true;

      // Test that operations should still work or be appropriately restricted
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);

      // Profile creation should still work (if not restricted)
      try {
        const [profilePda] = await testHelpers.createResearcherWithProfile(
          researcher,
          TEST_RESEARCHERS.CHARLIE
        );
        console.log("✓ Profile creation allowed during protocol pause");
      } catch (error) {
        console.log("⚠️  Profile creation restricted during protocol pause");
      }

      // Unpause the protocol
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
  } as any)
        .signers([setup.authority])
        .rpc();

      const unpausedProtocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      expect(unpausedProtocolState.isPaused).to.be.false;
    });
  });

  describe("Authority and Permission Integration", () => {
    it("Should coordinate authority between programs", async () => {
      const protocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      
      // Verify that the same authority can perform operations in both programs
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);

      // Create profile
  const [profilePda] = await testHelpers.createResearcherWithProfile(
        researcher,
  TEST_RESEARCHERS.ALICE
      );

      // Authority should be able to verify researcher
      await testHelpers.verifyResearcher(profilePda, setup.authority);

  const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
  expect(profile.isVerified).to.equal(true);
    });

    it("Should handle authority transfer across programs", async () => {
  const newAuthority = Keypair.generate();
      await fundWallet(setup.provider, newAuthority.publicKey);
  const originalAuthority = setup.authority;

      // Transfer authority in protocol
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
  } as any)
        .signers([setup.authority])
        .rpc();

      const updatedProtocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      expect(updatedProtocolState.authority.toString()).to.equal(newAuthority.publicKey.toString());

      // Test that new authority can verify researchers
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);

      const [profilePda] = await testHelpers.createResearcherWithProfile(
        researcher,
        { ...TEST_RESEARCHERS.ALICE, name: "Authority Transfer Test" }
      );

      await testHelpers.verifyResearcher(profilePda, newAuthority);

      const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
  expect(profile.isVerified).to.equal(true);

      // Restore original authority for other tests by transferring back
      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: null,
          minimumStakedAmount: null,
          isPaused: null,
          newAuthority: originalAuthority.publicKey,
        })
        .accounts({
          protocolState: setup.protocolStatePda,
          authority: newAuthority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([newAuthority])
        .rpc();

      setup.authority = originalAuthority;
    });
  });

  describe("Data Consistency Across Programs", () => {
    it("Should maintain consistent state across program interactions", async () => {
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);

      // Create and verify researcher
      const [profilePda] = await testHelpers.createResearcherWithProfile(
        researcher,
        TEST_RESEARCHERS.ALICE
      );
      await testHelpers.verifyResearcher(profilePda, setup.authority);

      // Create proposal
      const proposal = TEST_PROPOSALS.PRECISION_AGRICULTURE;
      const [proposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 0);

      await setup.researchDao.methods
        .createProposal(
          proposal.title,
          proposal.description,
          proposal.category,
          proposal.fundingGoal,
          proposal.duration,
          proposal.milestones,
          proposal.ipfsHash
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      // Submit proposal for funding (no args per IDL)
      await setup.researchDao.methods
        .submitProposalForFunding()
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      // Verify state consistency
      const finalProfile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      const finalProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      const protocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);

      // Check that cross-references are consistent
      expect(finalProposal.researcher.toString()).to.equal(researcher.publicKey.toString());
      expect(finalProfile.researcher.toString()).to.equal(researcher.publicKey.toString());
  expect(finalProfile.isVerified).to.equal(true);
  expect(finalProposal.status).to.deep.equal({ submittedForFunding: {} });
    });

    it("Should handle concurrent operations across programs", async () => {
      const operations = [];

      // Concurrent protocol update
      const protocolOperation = setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: new anchor.BN(50000),
          minimumStakedAmount: null,
          isPaused: null,
          newAuthority: null,
        })
  .accounts({
          protocolState: setup.protocolStatePda,
          authority: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
  } as any)
        .signers([setup.authority])
        .rpc();

      operations.push(protocolOperation);

      // Concurrent researcher operations
      for (let i = 0; i < 2; i++) {
        const researcher = Keypair.generate();
        await fundWallet(setup.provider, researcher.publicKey);

        const researcherOperation = (async () => {
          const [profilePda] = await testHelpers.createResearcherWithProfile(
            researcher,
            { ...TEST_RESEARCHERS.ALICE, name: `Concurrent Researcher ${i}` }
          );
          await testHelpers.verifyResearcher(profilePda, setup.authority);
          return profilePda;
        })();

        operations.push(researcherOperation);
      }

      // Wait for all operations to complete
      const results = await Promise.all(operations);
      
      // Verify all operations completed successfully
      expect(results).to.have.length(3); // 1 protocol update + 2 researcher operations

      // Verify final state consistency
      const finalProtocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      expect(finalProtocolState.researchProposalFee.toNumber()).to.equal(50000);

      // Verify researcher profiles
      for (let i = 1; i < results.length; i++) {
        if (results[i] instanceof anchor.web3.PublicKey) {
          const profile = await setup.researchDao.account.researcherProfile.fetch(results[i] as anchor.web3.PublicKey);
          expect(profile.isVerified).to.equal(true);
        }
      }
    });
  });

  describe("Event and State Synchronization", () => {
    it("Should handle event ordering across programs", async () => {
      const events: string[] = [];

      // Protocol event
      const stateBefore = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      const targetMinStaked = new anchor.BN(100000);
      const newMinStaked = stateBefore.minimumStakedAmount.eq(targetMinStaked)
        ? targetMinStaked.add(new anchor.BN(1))
        : targetMinStaked;

      await setup.agroDao.methods
        .updateProtocol({
          minFundingThreshold: null,
          researchProposalFee: null,
          minimumStakedAmount: newMinStaked,
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

      events.push("Protocol Updated");

      // Research events
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);

      const [profilePda] = await testHelpers.createResearcherWithProfile(
        researcher,
        TEST_RESEARCHERS.ALICE
      );
      events.push("Researcher Profile Created");

      await testHelpers.verifyResearcher(profilePda, setup.authority);
      events.push("Researcher Verified");

      // Create proposal
  const proposal = TEST_PROPOSALS.SOIL_HEALTH;
      const [proposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 0);

      await setup.researchDao.methods
        .createProposal(
          proposal.title,
          proposal.description,
          proposal.category,
          proposal.fundingGoal,
          proposal.duration,
          proposal.milestones,
          proposal.ipfsHash
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
  } as any)
        .signers([researcher])
        .rpc();

      events.push("Research Proposal Created");

      // Verify event ordering and state consistency
      const finalProtocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      const finalProfile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      const finalProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);

      expect(events).to.deep.equal([
        "Protocol Updated",
        "Researcher Profile Created",
        "Researcher Verified",
        "Research Proposal Created"
      ]);

  expect(finalProtocolState.minimumStakedAmount.toString()).to.equal(newMinStaked.toString());
  expect(finalProfile.isVerified).to.equal(true);
  expect(finalProposal.status).to.deep.equal({ draft: {} });

      console.log("Event sequence:", events);
      console.log("✓ Event ordering and state synchronization verified");
    });
  });
});
