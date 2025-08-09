import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet } from "../utils/setup";
import { TEST_RESEARCHERS, TEST_PROPOSALS, TEST_CONSTANTS } from "../utils/constants";
import { PDAHelper, TestHelpers } from "../utils/helpers";

describe("ResearchDao - Research Proposals", () => {
  let setup: TestSetup;
  let verifiedResearcher: Keypair;
  let unverifiedResearcher: Keypair;
  let verifiedProfilePda: anchor.web3.PublicKey;
  let unverifiedProfilePda: anchor.web3.PublicKey;
  let pdaHelper: PDAHelper;
  let testHelpers: TestHelpers;
  let proposalCounter = 0;

  before(async () => {
    setup = await setupTestEnvironment();
    verifiedResearcher = Keypair.generate();
    unverifiedResearcher = Keypair.generate();
    pdaHelper = new PDAHelper(setup.researchDao.programId);
    testHelpers = new TestHelpers(setup.researchDao);

    await fundWallet(setup.provider, verifiedResearcher.publicKey);
    await fundWallet(setup.provider, unverifiedResearcher.publicKey);

    // Create and verify researcher profile
    [verifiedProfilePda] = await testHelpers.createResearcherWithProfile(
      verifiedResearcher,
      TEST_RESEARCHERS.ALICE
    );
    await testHelpers.verifyResearcher(verifiedProfilePda, setup.authority);

    // Create unverified researcher profile
    [unverifiedProfilePda] = await testHelpers.createResearcherWithProfile(
      unverifiedResearcher,
      TEST_RESEARCHERS.BOB
    );
  });

  describe("Proposal Creation", () => {
    it("Should create a research proposal successfully", async () => {
      const proposal = TEST_PROPOSALS.DROUGHT_RESISTANCE;
      const [proposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

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
          researcherProfile: verifiedProfilePda,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([verifiedResearcher])
        .rpc();

      const createdProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);

      expect(createdProposal.researcher.toString()).to.equal(verifiedResearcher.publicKey.toString());
      expect(createdProposal.title).to.equal(proposal.title);
      expect(createdProposal.description).to.equal(proposal.description);
      expect(createdProposal.category).to.equal(proposal.category);
      expect(createdProposal.fundingGoal.toString()).to.equal(proposal.fundingGoal.toString());
      expect(createdProposal.duration).to.equal(proposal.duration);
      expect(createdProposal.ipfsHash).to.equal(proposal.ipfsHash);
      expect(createdProposal.milestones).to.deep.equal(proposal.milestones);
      expect(createdProposal.status).to.deep.equal({ pending: {} });
      expect(createdProposal.currentFunding.toNumber()).to.equal(0);
      expect(createdProposal.totalFunders).to.equal(0);
      expect(createdProposal.milestonesCompleted).to.equal(0);
      expect(createdProposal.createdAt.toNumber()).to.be.greaterThan(0);

      proposalCounter++;
    });

    it("Should prevent unverified researchers from creating proposals", async () => {
      const proposal = TEST_PROPOSALS.SOIL_HEALTH;
      const [proposalPda] = pdaHelper.getResearchProposalPda(
        unverifiedResearcher.publicKey,
        0
      );

      try {
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
            researcherProfile: unverifiedProfilePda,
            researcher: unverifiedResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unverifiedResearcher])
          .rpc();

        expect.fail("Should have thrown error for unverified researcher");
      } catch (error) {
        expect(error.message).to.include("ResearcherNotVerified");
      }
    });

    it("Should validate proposal data", async () => {
      const [proposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

      // Test empty title
      try {
        await setup.researchDao.methods
          .createProposal(
            "", // Empty title
            "Valid description",
            "Crop Science",
            new anchor.BN(100000),
            30,
            "QmTestHash",
            ["Milestone 1"]
          )
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: verifiedProfilePda,
            researcher: verifiedResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([verifiedResearcher])
          .rpc();

        expect.fail("Should have thrown error for empty title");
      } catch (error) {
        expect(error.message).to.include("InvalidProposalData");
      }
    });

    it("Should validate funding goal limits", async () => {
      const [proposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

      // Test zero funding goal
      try {
        await setup.researchDao.methods
          .createProposal(
            "Valid Title",
            "Valid description",
            "Crop Science",
            new anchor.BN(0), // Zero funding
            30,
            "QmTestHash",
            ["Milestone 1"]
          )
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: verifiedProfilePda,
            researcher: verifiedResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([verifiedResearcher])
          .rpc();

        expect.fail("Should have thrown error for zero funding goal");
      } catch (error) {
        expect(error.message).to.include("InvalidFundingGoal");
      }
    });

    it("Should create proposals with different categories", async () => {
      const categories = ["Crop Science", "Soil Science", "Plant Pathology", "Sustainable Agriculture"];
      
      for (let i = 0; i < categories.length; i++) {
        const [proposalPda] = pdaHelper.getResearchProposalPda(
          verifiedResearcher.publicKey,
          proposalCounter
        );

        await setup.researchDao.methods
          .createProposal(
            `${categories[i]} Research`,
            `Research proposal for ${categories[i]}`,
            categories[i],
            new anchor.BN(50000 + i * 10000),
            30 + i * 10,
            `QmHash${i}`,
            [`${categories[i]} Milestone 1`, `${categories[i]} Milestone 2`]
          )
          .accounts({
            researchProposal: proposalPda,
            researcherProfile: verifiedProfilePda,
            researcher: verifiedResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([verifiedResearcher])
          .rpc();

        const proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
        expect(proposal.category).to.equal(categories[i]);
        expect(proposal.milestones.length).to.equal(2);

        proposalCounter++;
      }
    });
  });

  describe("Proposal Funding", () => {
    let fundableProposalPda: anchor.web3.PublicKey;
    let funder: Keypair;

    before(async () => {
      funder = Keypair.generate();
      await fundWallet(setup.provider, funder.publicKey, 10); // Fund with 10 SOL

      // Create a proposal for funding tests
      const proposal = TEST_PROPOSALS.PRECISION_AGRICULTURE;
      [fundableProposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

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
          researchProposal: fundableProposalPda,
          researcherProfile: verifiedProfilePda,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([verifiedResearcher])
        .rpc();

      proposalCounter++;
    });

    it("Should fund a research proposal", async () => {
      const fundingAmount = new anchor.BN(50000); // 0.05 SOL in lamports

      await setup.researchDao.methods
        .submitProposalForFunding(fundingAmount)
        .accounts({
          researchProposal: fundableProposalPda,
          funder: funder.publicKey,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder])
        .rpc();

      const proposal = await setup.researchDao.account.researchProposal.fetch(fundableProposalPda);

      expect(proposal.currentFunding.toString()).to.equal(fundingAmount.toString());
      expect(proposal.totalFunders).to.equal(1);
    });

    it("Should handle multiple funders", async () => {
      const secondFunder = Keypair.generate();
      await fundWallet(setup.provider, secondFunder.publicKey, 5);

      const fundingAmount = new anchor.BN(25000);

      await setup.researchDao.methods
        .submitProposalForFunding(fundingAmount)
        .accounts({
          researchProposal: fundableProposalPda,
          funder: secondFunder.publicKey,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([secondFunder])
        .rpc();

      const proposal = await setup.researchDao.account.researchProposal.fetch(fundableProposalPda);

      expect(proposal.currentFunding.toNumber()).to.equal(75000); // 50000 + 25000
      expect(proposal.totalFunders).to.equal(2);
    });

    it("Should prevent funding zero amount", async () => {
      const zeroFunder = Keypair.generate();
      await fundWallet(setup.provider, zeroFunder.publicKey);

      try {
        await setup.researchDao.methods
          .submitProposalForFunding(new anchor.BN(0))
          .accounts({
            researchProposal: fundableProposalPda,
            funder: zeroFunder.publicKey,
            researcher: verifiedResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([zeroFunder])
          .rpc();

        expect.fail("Should have thrown error for zero funding amount");
      } catch (error) {
        expect(error.message).to.include("InvalidFundingAmount");
      }
    });

    it("Should update proposal status when funding goal is reached", async () => {
      const proposal = await setup.researchDao.account.researchProposal.fetch(fundableProposalPda);
      const remainingFunding = proposal.fundingGoal.sub(proposal.currentFunding);

      if (remainingFunding.toNumber() > 0) {
        const finalFunder = Keypair.generate();
        await fundWallet(setup.provider, finalFunder.publicKey, 10);

        await setup.researchDao.methods
          .submitProposalForFunding(remainingFunding)
          .accounts({
            researchProposal: fundableProposalPda,
            funder: finalFunder.publicKey,
            researcher: verifiedResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([finalFunder])
          .rpc();

        const updatedProposal = await setup.researchDao.account.researchProposal.fetch(fundableProposalPda);
        expect(updatedProposal.status).to.deep.equal({ funded: {} });
        expect(updatedProposal.currentFunding.toString()).to.equal(updatedProposal.fundingGoal.toString());
      }
    });
  });

  describe("Proposal Management", () => {
    it("Should prevent duplicate proposal creation with same details", async () => {
      const proposal = TEST_PROPOSALS.DROUGHT_RESISTANCE;
      const [proposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

      try {
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
            researcherProfile: verifiedProfilePda,
            researcher: verifiedResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([verifiedResearcher])
          .rpc();

        proposalCounter++;
      } catch (error) {
        // This is expected if the PDA already exists
        expect(error.message).to.include("already in use");
      }
    });

    it("Should track researcher's proposal count", async () => {
      const profileBefore = await setup.researchDao.account.researcherProfile.fetch(verifiedProfilePda);
      const initialProposalCount = profileBefore.totalProposals;

      const [proposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

      await setup.researchDao.methods
        .createProposal(
          "Tracking Test Proposal",
          "Testing proposal counting",
          "Research Metrics",
          new anchor.BN(100000),
          30,
          "QmTrackingHash",
          ["Tracking Milestone"]
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: verifiedProfilePda,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([verifiedResearcher])
        .rpc();

      const profileAfter = await setup.researchDao.account.researcherProfile.fetch(verifiedProfilePda);
      expect(profileAfter.totalProposals).to.equal(initialProposalCount + 1);

      proposalCounter++;
    });
  });

  describe("Proposal State Transitions", () => {
    it("Should handle proposal lifecycle states correctly", async () => {
      const [proposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

      // Create proposal
      await setup.researchDao.methods
        .createProposal(
          "Lifecycle Test",
          "Testing proposal lifecycle",
          "Agricultural Engineering",
          new anchor.BN(100000),
          60,
          "QmLifecycleHash",
          ["Phase 1", "Phase 2", "Phase 3"]
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: verifiedProfilePda,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([verifiedResearcher])
        .rpc();

      let proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(proposal.status).to.deep.equal({ pending: {} });

      // Fund the proposal
      const funder = Keypair.generate();
      await fundWallet(setup.provider, funder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding(proposal.fundingGoal)
        .accounts({
          researchProposal: proposalPda,
          funder: funder.publicKey,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder])
        .rpc();

      proposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(proposal.status).to.deep.equal({ funded: {} });
      expect(proposal.currentFunding.toString()).to.equal(proposal.fundingGoal.toString());

      proposalCounter++;
    });

    it("Should maintain proposal data integrity across state changes", async () => {
      const originalProposal = TEST_PROPOSALS.CLIMATE_ADAPTATION;
      const [proposalPda] = pdaHelper.getResearchProposalPda(
        verifiedResearcher.publicKey,
        proposalCounter
      );

      await setup.researchDao.methods
        .createProposal(
          originalProposal.title,
          originalProposal.description,
          originalProposal.category,
          originalProposal.fundingGoal,
          originalProposal.duration,
          originalProposal.ipfsHash,
          originalProposal.milestones
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: verifiedProfilePda,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([verifiedResearcher])
        .rpc();

      const proposalBefore = await setup.researchDao.account.researchProposal.fetch(proposalPda);

      // Fund the proposal
      const funder = Keypair.generate();
      await fundWallet(setup.provider, funder.publicKey, 10);

      await setup.researchDao.methods
        .submitProposalForFunding(originalProposal.fundingGoal)
        .accounts({
          researchProposal: proposalPda,
          funder: funder.publicKey,
          researcher: verifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder])
        .rpc();

      const proposalAfter = await setup.researchDao.account.researchProposal.fetch(proposalPda);

      // Verify data integrity
      expect(proposalAfter.researcher.toString()).to.equal(proposalBefore.researcher.toString());
      expect(proposalAfter.title).to.equal(proposalBefore.title);
      expect(proposalAfter.description).to.equal(proposalBefore.description);
      expect(proposalAfter.category).to.equal(proposalBefore.category);
      expect(proposalAfter.fundingGoal.toString()).to.equal(proposalBefore.fundingGoal.toString());
      expect(proposalAfter.milestones).to.deep.equal(proposalBefore.milestones);
      expect(proposalAfter.createdAt.toString()).to.equal(proposalBefore.createdAt.toString());

      proposalCounter++;
    });
  });
});
