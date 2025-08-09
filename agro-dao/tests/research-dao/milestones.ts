import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet, delay } from "../utils/setup";
import { TEST_RESEARCHERS, TEST_PROPOSALS, TEST_MILESTONES } from "../utils/constants";
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
    testHelpers = new TestHelpers(setup.researchDao);

    await fundWallet(setup.provider, researcher.publicKey);

    // Create and verify researcher
    [profilePda] = await testHelpers.createResearcherWithProfile(
      researcher,
      TEST_RESEARCHERS.ALICE
    );
    await testHelpers.verifyResearcher(profilePda, setup.authority);

    // Create a funded proposal for milestone testing
    const proposal = TEST_PROPOSALS.DROUGHT_RESISTANCE;
    [proposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 0);

    await setup.researchDao.methods
      .createProposal(
        proposal.title,
        proposal.description,
        proposal.category,
        proposal.fundingGoal,
        proposal.duration,
        proposal.ipfsHash,
        proposal.milestones
      )
      .accounts({
        researchProposal: proposalPda,
        researcherProfile: profilePda,
        researcher: researcher.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([researcher])
      .rpc();

    // Fund the proposal
    const funder = Keypair.generate();
    await fundWallet(setup.provider, funder.publicKey, 10);

    await setup.researchDao.methods
      .submitProposalForFunding(proposal.fundingGoal)
      .accounts({
        researchProposal: proposalPda,
        funder: funder.publicKey,
        researcher: researcher.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([funder])
      .rpc();
  });

  describe("Milestone Publication", () => {
    it("Should publish a milestone successfully", async () => {
      const milestone = TEST_MILESTONES.INITIAL_RESEARCH;

      await setup.researchDao.methods
        .publishMilestone(
          milestone.title,
          milestone.description,
          milestone.ipfsHash,
          milestone.completionPercentage
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      const proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(proposal.milestonesCompleted).to.equal(1);
      expect(proposal.status).to.deep.equal({ inProgress: {} });
      
      const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.reputation).to.be.greaterThan(0);
    });

    it("Should prevent milestone publication by non-researcher", async () => {
      const unauthorizedUser = Keypair.generate();
      await fundWallet(setup.provider, unauthorizedUser.publicKey);
      
      const milestone = TEST_MILESTONES.DATA_COLLECTION;

      try {
        await setup.researchDao.methods
          .publishMilestone(
            milestone.title,
            milestone.description,
            milestone.ipfsHash,
            milestone.completionPercentage
          )
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: profilePda,
            researcher: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unauthorizedUser])
          .rpc();

        expect.fail("Should have thrown error for unauthorized milestone publication");
      } catch (error) {
        expect(error.message).to.include("UnauthorizedMilestonePublication");
      }
    });

    it("Should validate milestone data", async () => {
      try {
        await setup.researchDao.methods
          .publishMilestone(
            "", // Empty title
            "Valid description",
            "QmValidHash",
            50
          )
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([researcher])
          .rpc();

        expect.fail("Should have thrown error for invalid milestone data");
      } catch (error) {
        expect(error.message).to.include("InvalidMilestoneData");
      }
    });

    it("Should publish multiple milestones", async () => {
      const milestones = [
        TEST_MILESTONES.DATA_COLLECTION,
        TEST_MILESTONES.ANALYSIS_PHASE,
        TEST_MILESTONES.FIELD_TRIALS
      ];

      for (let i = 0; i < milestones.length; i++) {
        await setup.researchDao.methods
          .publishMilestone(
            milestones[i].title,
            milestones[i].description,
            milestones[i].ipfsHash,
            milestones[i].completionPercentage
          )
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([researcher])
          .rpc();

        await delay(1000); // Add delay between milestones
      }

      const proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(proposal.milestonesCompleted).to.equal(4); // 1 + 3 new milestones
      
      const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.reputation).to.be.greaterThan(30); // Should have accumulated reputation
    });

    it("Should prevent milestone publication on unfunded proposals", async () => {
      // Create an unfunded proposal
      const unfundedProposal = TEST_PROPOSALS.SOIL_HEALTH;
      const [unfundedProposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 1);

      await setup.researchDao.methods
        .createProposal(
          unfundedProposal.title,
          unfundedProposal.description,
          unfundedProposal.category,
          unfundedProposal.fundingGoal,
          unfundedProposal.duration,
          unfundedProposal.ipfsHash,
          unfundedProposal.milestones
        )
        .accounts({
          researchProposal: unfundedProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      const milestone = TEST_MILESTONES.INITIAL_RESEARCH;

      try {
        await setup.researchDao.methods
          .publishMilestone(
            milestone.title,
            milestone.description,
            milestone.ipfsHash,
            milestone.completionPercentage
          )
          .accounts({
            researchProposal: unfundedProposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([researcher])
          .rpc();

        expect.fail("Should have thrown error for milestone on unfunded proposal");
      } catch (error) {
        expect(error.message).to.include("ProposalNotFunded");
      }
    });
  });

  describe("Research Findings Publication", () => {
    it("Should publish research findings successfully", async () => {
      const findings = {
        title: "Drought Resistance Gene Discovery",
        summary: "Identified key genetic markers for drought resistance in wheat varieties",
        ipfsHash: "QmFindingsHash123",
        peerReviewed: false
      };

      await setup.researchDao.methods
        .publishFindings(
          findings.title,
          findings.summary,
          findings.ipfsHash,
          findings.peerReviewed
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      const proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(proposal.status).to.deep.equal({ completed: {} });
      expect(proposal.completedAt.toNumber()).to.be.greaterThan(0);
      
      const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.completedProjects).to.equal(1);
      expect(profile.reputation).to.be.greaterThan(50); // Completion bonus
    });

    it("Should prevent findings publication by non-researcher", async () => {
      // Create another proposal for this test
      const newProposal = TEST_PROPOSALS.PRECISION_AGRICULTURE;
      const [newProposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 2);

      await setup.researchDao.methods
        .createProposal(
          newProposal.title,
          newProposal.description,
          newProposal.category,
          newProposal.fundingGoal,
          newProposal.duration,
          newProposal.ipfsHash,
          newProposal.milestones
        )
        .accounts({
          researchProposal: newProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      // Fund the proposal
      const funder = Keypair.generate();
      await fundWallet(setup.provider, funder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding(newProposal.fundingGoal)
        .accounts({
          researchProposal: newProposalPda,
          funder: funder.publicKey,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder])
        .rpc();

      // Try to publish findings with unauthorized user
      const unauthorizedUser = Keypair.generate();
      await fundWallet(setup.provider, unauthorizedUser.publicKey);

      try {
        await setup.researchDao.methods
          .publishFindings(
            "Unauthorized Findings",
            "These findings are unauthorized",
            "QmUnauthorizedHash",
            false
          )
          .accounts({
            researchProposal: newProposalPda,
            researcherProfile: profilePda,
            researcher: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unauthorizedUser])
          .rpc();

        expect.fail("Should have thrown error for unauthorized findings publication");
      } catch (error) {
        expect(error.message).to.include("UnauthorizedFindingsPublication");
      }
    });

    it("Should validate findings data", async () => {
      // Create another proposal for validation tests
      const validationProposal = TEST_PROPOSALS.CLIMATE_ADAPTATION;
      const [validationProposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 3);

      await setup.researchDao.methods
        .createProposal(
          validationProposal.title,
          validationProposal.description,
          validationProposal.category,
          validationProposal.fundingGoal,
          validationProposal.duration,
          validationProposal.ipfsHash,
          validationProposal.milestones
        )
        .accounts({
          researchProposal: validationProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      // Fund the proposal
      const funder = Keypair.generate();
      await fundWallet(setup.provider, funder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding(validationProposal.fundingGoal)
        .accounts({
          researchProposal: validationProposalPda,
          funder: funder.publicKey,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder])
        .rpc();

      try {
        await setup.researchDao.methods
          .publishFindings(
            "", // Empty title
            "Valid summary",
            "QmValidHash",
            false
          )
          .accounts({
            researchProposal: validationProposalPda,
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([researcher])
          .rpc();

        expect.fail("Should have thrown error for invalid findings data");
      } catch (error) {
        expect(error.message).to.include("InvalidFindingsData");
      }
    });

    it("Should handle peer-reviewed vs non-peer-reviewed findings", async () => {
      // Create proposal for peer-reviewed findings
      const peerReviewProposal = TEST_PROPOSALS.SUSTAINABLE_PRACTICES;
      const [peerReviewProposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 4);

      await setup.researchDao.methods
        .createProposal(
          peerReviewProposal.title,
          peerReviewProposal.description,
          peerReviewProposal.category,
          peerReviewProposal.fundingGoal,
          peerReviewProposal.duration,
          peerReviewProposal.ipfsHash,
          peerReviewProposal.milestones
        )
        .accounts({
          researchProposal: peerReviewProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      // Fund the proposal
      const funder = Keypair.generate();
      await fundWallet(setup.provider, funder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding(peerReviewProposal.fundingGoal)
        .accounts({
          researchProposal: peerReviewProposalPda,
          funder: funder.publicKey,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder])
        .rpc();

      const reputationBefore = (await setup.researchDao.account.researcherProfile.fetch(profilePda)).reputation;

      // Publish peer-reviewed findings
      await setup.researchDao.methods
        .publishFindings(
          "Peer-Reviewed Sustainable Practices Study",
          "Comprehensive analysis of sustainable agricultural practices with peer review",
          "QmPeerReviewedHash",
          true // Peer-reviewed
        )
        .accounts({
          researchProposal: peerReviewProposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      const reputationAfter = (await setup.researchDao.account.researcherProfile.fetch(profilePda)).reputation;
      
      // Peer-reviewed findings should give more reputation bonus
      expect(reputationAfter).to.be.greaterThan(reputationBefore + 10);
    });
  });

  describe("Research Lifecycle Integration", () => {
    it("Should track complete research lifecycle", async () => {
      // Create a new researcher for full lifecycle test
      const lifecycleResearcher = Keypair.generate();
      await fundWallet(setup.provider, lifecycleResearcher.publicKey);

      const [lifecycleProfilePda] = await testHelpers.createResearcherWithProfile(
        lifecycleResearcher,
        TEST_RESEARCHERS.DAVID
      );
      await testHelpers.verifyResearcher(lifecycleProfilePda, setup.authority);

      // Create proposal
      const lifecycleProposal = TEST_PROPOSALS.SMART_FARMING;
      const [lifecycleProposalPda] = pdaHelper.getResearchProposalPda(lifecycleResearcher.publicKey, 0);

      await setup.researchDao.methods
        .createProposal(
          lifecycleProposal.title,
          lifecycleProposal.description,
          lifecycleProposal.category,
          lifecycleProposal.fundingGoal,
          lifecycleProposal.duration,
          lifecycleProposal.ipfsHash,
          lifecycleProposal.milestones
        )
        .accounts({
          researchProposal: lifecycleProposalPda,
          researcherProfile: lifecycleProfilePda,
          researcher: lifecycleResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([lifecycleResearcher])
        .rpc();

      // Fund proposal
      const lifecycleFunder = Keypair.generate();
      await fundWallet(setup.provider, lifecycleFunder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding(lifecycleProposal.fundingGoal)
        .accounts({
          researchProposal: lifecycleProposalPda,
          funder: lifecycleFunder.publicKey,
          researcher: lifecycleResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([lifecycleFunder])
        .rpc();

      // Publish milestones
      for (const milestone of [TEST_MILESTONES.INITIAL_RESEARCH, TEST_MILESTONES.DATA_COLLECTION]) {
        await setup.researchDao.methods
          .publishMilestone(
            milestone.title,
            milestone.description,
            milestone.ipfsHash,
            milestone.completionPercentage
          )
          .accounts({
            researchProposal: lifecycleProposalPda,
            researcherProfile: lifecycleProfilePda,
            researcher: lifecycleResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([lifecycleResearcher])
          .rpc();

        await delay(500);
      }

      // Publish findings
      await setup.researchDao.methods
        .publishFindings(
          "Smart Farming Technology Assessment",
          "Comprehensive evaluation of IoT and AI technologies in modern agriculture",
          "QmSmartFarmingHash",
          true
        )
        .accounts({
          researchProposal: lifecycleProposalPda,
          researcherProfile: lifecycleProfilePda,
          researcher: lifecycleResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([lifecycleResearcher])
        .rpc();

      // Verify final state
      const finalProposal = await setup.researchDao.account.researchProposal.fetch(lifecycleProposalPda);
      const finalProfile = await setup.researchDao.account.researcherProfile.fetch(lifecycleProfilePda);

      expect(finalProposal.status).to.deep.equal({ completed: {} });
      expect(finalProposal.milestonesCompleted).to.equal(2);
      expect(finalProposal.completedAt.toNumber()).to.be.greaterThan(0);
      
      expect(finalProfile.totalProposals).to.equal(1);
      expect(finalProfile.fundedProposals).to.equal(1);
      expect(finalProfile.completedProjects).to.equal(1);
      expect(finalProfile.reputation).to.be.greaterThan(20);
      expect(finalProfile.totalFundingReceived.toString()).to.equal(lifecycleProposal.fundingGoal.toString());

      console.log("Lifecycle Test Results:");
      console.log("- Final Reputation:", finalProfile.reputation);
      console.log("- Total Funding Received:", finalProfile.totalFundingReceived.toString());
      console.log("- Milestones Completed:", finalProposal.milestonesCompleted);
      console.log("- Proposal Status:", Object.keys(finalProposal.status)[0]);
    });
  });
});
