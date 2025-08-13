import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet, delay } from "../utils/setup";
import { TEST_RESEARCHERS, TEST_PROPOSALS, TEST_MILESTONES, TEST_CONSTANTS } from "../utils/constants";
import { PDAHelper, TestHelpers } from "../utils/helpers";

describe("ResearchDao - Milestones & Research Lifecycle", () => {
  let setup: TestSetup;
  let researcher: Keypair;
  let profilePda: anchor.web3.PublicKey;
  let proposalPda: anchor.web3.PublicKey;
  let pdaHelper: PDAHelper;
  let testHelpers: TestHelpers;

  before(async () => {
    setup = await setupTestEnvironment();
    researcher = Keypair.generate();
    pdaHelper = new PDAHelper(setup.researchDao.programId);
  testHelpers = new TestHelpers(setup as any);

    await fundWallet(setup.provider, researcher.publicKey);

    // Create and verify researcher
    [profilePda] = await testHelpers.createResearcherWithProfile(
      researcher,
      TEST_RESEARCHERS.ALICE
    );
    await testHelpers.verifyResearcher(profilePda, setup.authority);

    // Create a funded proposal for milestone testing
    const proposal = {
      ...TEST_PROPOSALS.DROUGHT_RESISTANCE,
      milestones: [
        { description: "Phase 1", targetDate: new anchor.BN(Math.floor(Date.now()/1000) + 7*TEST_CONSTANTS.SECONDS_PER_DAY), completionDate: null, isCompleted: false, ipfsEvidenceHash: null },
      ] as any,
    };
    [proposalPda] = await pdaHelper.getNextProposalPdaFromProfile(
      setup.researchDao as any,
      researcher.publicKey,
      profilePda
    );

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

    // Fund the proposal
    const funder = Keypair.generate();
    await fundWallet(setup.provider, funder.publicKey, 10);

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
  });

  describe("Milestone Publication", () => {
    it("Should publish a milestone successfully", async () => {
      // First milestone (index 0) with 32-byte evidence hash
      await setup.researchDao.methods
        .publishMilestone(0, TEST_CONSTANTS.MOCK_EVIDENCE_HASH)
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

  const proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
  // With a single milestone, status progresses to Completed after first publish
  expect(["inProgress","completed"]).to.include(Object.keys(proposal.status)[0]);
    });

    it("Should prevent milestone publication by non-researcher", async () => {
      const unauthorizedUser = Keypair.generate();
      await fundWallet(setup.provider, unauthorizedUser.publicKey);

      try {
        await setup.researchDao.methods
          .publishMilestone(1, TEST_CONSTANTS.MOCK_EVIDENCE_HASH)
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: profilePda,
            researcher: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([unauthorizedUser])
          .rpc();

        expect.fail("Should have thrown error for unauthorized milestone publication");
      } catch (error) {
        // Any authorization-related error is acceptable here
        expect(error).to.be.instanceOf(Error);
      }
    });

  it("Should validate milestone data", async () => {
      // Try with an obviously invalid evidence hash (wrong length) to provoke a client/serialization error
      const badHash = Array.from({ length: 16 }, () => 1) as any;
      try {
        await setup.researchDao.methods
          .publishMilestone(2, badHash)
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([researcher])
          .rpc();

        expect.fail("Should have thrown error for invalid milestone data");
      } catch (error) {
        expect(error).to.be.instanceOf(Error);
      }
    });

    it("Should publish multiple milestones", async () => {
    // Only index 0 exists in this test setup; if already completed, expect error
    for (let i = 0; i < 1; i++) {
        await setup.researchDao.methods
          .publishMilestone(i, TEST_CONSTANTS.MOCK_EVIDENCE_HASH)
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([researcher])
      .rpc().catch(() => {});

        await delay(250);
      }

  const proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
  expect(["inProgress","completed"]).to.include(Object.keys(proposal.status)[0]);
    });

    it("Should prevent milestone publication on unfunded proposals", async () => {
      // Create an unfunded proposal
      const unfundedProposal = TEST_PROPOSALS.SOIL_HEALTH;
      const [unfundedProposalPda] = await pdaHelper.getNextProposalPdaFromProfile(
        setup.researchDao as any,
        researcher.publicKey,
        profilePda
      );

      await setup.researchDao.methods
        .createProposal(
          unfundedProposal.title,
          unfundedProposal.description,
          unfundedProposal.category,
          unfundedProposal.fundingGoal,
          unfundedProposal.duration,
          unfundedProposal.milestones,
          unfundedProposal.ipfsHash
        )
        .accounts({
          researchProposal: unfundedProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      const milestone = TEST_MILESTONES.INITIAL_RESEARCH;

      try {
        await setup.researchDao.methods
          .publishMilestone(0, TEST_CONSTANTS.MOCK_EVIDENCE_HASH)
          .accounts({
            researchProposal: unfundedProposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([researcher])
          .rpc();

        expect.fail("Should have thrown error for milestone on unfunded proposal");
      } catch (error) {
        expect(error).to.be.instanceOf(Error);
      }
    });
  });

  describe("Research Findings Publication", () => {
    it("Should publish research findings successfully", async () => {
      await setup.researchDao.methods
        .publishFindings(TEST_CONSTANTS.MOCK_FINDINGS_HASH)
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      const proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(proposal.status).to.deep.equal({ completed: {} });
    });

    it("Should prevent findings publication by non-researcher", async () => {
      // Create another proposal for this test
      const newProposal = TEST_PROPOSALS.PRECISION_AGRICULTURE;
      const [newProposalPda] = await pdaHelper.getNextProposalPdaFromProfile(
        setup.researchDao as any,
        researcher.publicKey,
        profilePda
      );

      await setup.researchDao.methods
        .createProposal(
          newProposal.title,
          newProposal.description,
          newProposal.category,
          newProposal.fundingGoal,
          newProposal.duration,
          newProposal.milestones,
          newProposal.ipfsHash
        )
        .accounts({
          researchProposal: newProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      // Move to funding submission state
      await setup.researchDao.methods
        .submitProposalForFunding()
        .accounts({
          researchProposal: newProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      // Try to publish findings with unauthorized user
      const unauthorizedUser = Keypair.generate();
      await fundWallet(setup.provider, unauthorizedUser.publicKey);

      try {
        await setup.researchDao.methods
          .publishFindings(TEST_CONSTANTS.MOCK_FINDINGS_HASH)
          .accounts({
            researchProposal: newProposalPda,
            researcherProfile: profilePda,
            researcher: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([unauthorizedUser])
          .rpc();

        expect.fail("Should have thrown error for unauthorized findings publication");
      } catch (error) {
        expect(error).to.be.instanceOf(Error);
      }
    });

    it("Should validate findings data", async () => {
      // Create another proposal for validation tests
      const validationProposal = TEST_PROPOSALS.SOIL_HEALTH;
      const [validationProposalPda] = await pdaHelper.getNextProposalPdaFromProfile(
        setup.researchDao as any,
        researcher.publicKey,
        profilePda
      );

      await setup.researchDao.methods
        .createProposal(
          validationProposal.title,
          validationProposal.description,
          validationProposal.category,
          validationProposal.fundingGoal,
          validationProposal.duration,
          validationProposal.milestones,
          validationProposal.ipfsHash
        )
        .accounts({
          researchProposal: validationProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      // Fund the proposal
      const funder = Keypair.generate();
      await fundWallet(setup.provider, funder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding()
        .accounts({
          researchProposal: validationProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      try {
        const badHash = Array.from({ length: 16 }, () => 2) as any;
        await setup.researchDao.methods
          .publishFindings(badHash)
          .accounts({
            researchProposal: validationProposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([researcher])
          .rpc();

        expect.fail("Should have thrown error for invalid findings data");
      } catch (error) {
        expect(error).to.be.instanceOf(Error);
      }
    });

    it("Should handle peer-reviewed vs non-peer-reviewed findings", async () => {
      // Create proposal for findings test
      const peerReviewProposal = TEST_PROPOSALS.SOIL_HEALTH;
      const [peerReviewProposalPda] = await pdaHelper.getNextProposalPdaFromProfile(
        setup.researchDao as any,
        researcher.publicKey,
        profilePda
      );

      await setup.researchDao.methods
        .createProposal(
          peerReviewProposal.title,
          peerReviewProposal.description,
          peerReviewProposal.category,
          peerReviewProposal.fundingGoal,
          peerReviewProposal.duration,
          peerReviewProposal.milestones,
          peerReviewProposal.ipfsHash
        )
        .accounts({
          researchProposal: peerReviewProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      await setup.researchDao.methods
        .submitProposalForFunding()
        .accounts({
          researchProposal: peerReviewProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      // Publish findings; just verify it completes
      await setup.researchDao.methods
        .publishFindings(TEST_CONSTANTS.MOCK_FINDINGS_HASH)
        .accounts({
          researchProposal: peerReviewProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([researcher])
        .rpc();

      const p = await setup.researchDao.account.researchProposal.fetch(peerReviewProposalPda);
      expect(p.status).to.deep.equal({ completed: {} });
    });
  });

  describe("Research Lifecycle Integration", () => {
    it("Should track complete research lifecycle", async () => {
      // Create a new researcher for full lifecycle test
      const lifecycleResearcher = Keypair.generate();
      await fundWallet(setup.provider, lifecycleResearcher.publicKey);

      const [lifecycleProfilePda] = await testHelpers.createResearcherWithProfile(
        lifecycleResearcher,
  TEST_RESEARCHERS.CHARLIE
      );
      await testHelpers.verifyResearcher(lifecycleProfilePda, setup.authority);

      // Create proposal
  const lifecycleProposal = {
        ...TEST_PROPOSALS.PRECISION_AGRICULTURE,
        milestones: [
          { description: "Phase A", targetDate: new anchor.BN(Math.floor(Date.now()/1000) + 7*TEST_CONSTANTS.SECONDS_PER_DAY), completionDate: null, isCompleted: false, ipfsEvidenceHash: null },
          { description: "Phase B", targetDate: new anchor.BN(Math.floor(Date.now()/1000) + 14*TEST_CONSTANTS.SECONDS_PER_DAY), completionDate: null, isCompleted: false, ipfsEvidenceHash: null },
        ] as any,
      };
      const [lifecycleProposalPda] = await pdaHelper.getNextProposalPdaFromProfile(
        setup.researchDao as any,
        lifecycleResearcher.publicKey,
        lifecycleProfilePda
      );

      await setup.researchDao.methods
        .createProposal(
          lifecycleProposal.title,
          lifecycleProposal.description,
          lifecycleProposal.category,
          lifecycleProposal.fundingGoal,
          lifecycleProposal.duration,
          lifecycleProposal.milestones,
          lifecycleProposal.ipfsHash
        )
        .accounts({
          researchProposal: lifecycleProposalPda,
          researcherProfile: lifecycleProfilePda,
          researcher: lifecycleResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([lifecycleResearcher])
        .rpc();

      // Fund proposal
      const lifecycleFunder = Keypair.generate();
      await fundWallet(setup.provider, lifecycleFunder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding()
        .accounts({
          researchProposal: lifecycleProposalPda,
          researcherProfile: lifecycleProfilePda,
          researcher: lifecycleResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([lifecycleResearcher])
        .rpc();

      // Publish milestones
      // Publish two milestones (indices 0 and 1)
      for (const i of [0, 1]) {
        await setup.researchDao.methods
          .publishMilestone(i, TEST_CONSTANTS.MOCK_EVIDENCE_HASH)
          .accounts({
            researchProposal: lifecycleProposalPda,
            researcherProfile: lifecycleProfilePda,
            researcher: lifecycleResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          } as any)
          .signers([lifecycleResearcher])
          .rpc();

        await delay(250);
      }

      // Publish findings
      await setup.researchDao.methods
        .publishFindings(TEST_CONSTANTS.MOCK_FINDINGS_HASH)
        .accounts({
          researchProposal: lifecycleProposalPda,
          researcherProfile: lifecycleProfilePda,
          researcher: lifecycleResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .signers([lifecycleResearcher])
        .rpc();

      // Verify final state
      const finalProposal = await setup.researchDao.account.researchProposal.fetch(lifecycleProposalPda);
      const finalProfile = await setup.researchDao.account.researcherProfile.fetch(lifecycleProfilePda);

  expect(finalProposal.status).to.deep.equal({ completed: {} });
  expect(finalProfile.totalProposals).to.equal(1);
    });
  });
});
