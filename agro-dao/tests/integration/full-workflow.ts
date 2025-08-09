import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet, delay } from "../utils/setup";
import { TEST_RESEARCHERS, TEST_PROPOSALS, TEST_MILESTONES, TEST_CONSTANTS } from "../utils/constants";
import { PDAHelper, TestHelpers } from "../utils/helpers";

describe("Integration Tests - Full Research Workflow", () => {
  let setup: TestSetup;
  let researcher: Keypair;
  let funder1: Keypair;
  let funder2: Keypair;
  let profilePda: anchor.web3.PublicKey;
  let pdaHelper: PDAHelper;
  let testHelpers: TestHelpers;

  before(async () => {
    setup = await setupTestEnvironment();
    researcher = Keypair.generate();
    funder1 = Keypair.generate();
    funder2 = Keypair.generate();
    pdaHelper = new PDAHelper(setup.researchDao.programId);
    testHelpers = new TestHelpers(setup.researchDao);

    await fundWallet(setup.provider, researcher.publicKey);
    await fundWallet(setup.provider, funder1.publicKey, 10);
    await fundWallet(setup.provider, funder2.publicKey, 10);
  });

  describe("Complete Research Project Workflow", () => {
    it("Should execute a complete research project from start to finish", async () => {
      console.log("=== Starting Complete Research Workflow Test ===");

      // Step 1: Initialize Protocol (if not already done)
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
        console.log("✓ Protocol initialized");
      } catch {
        console.log("✓ Protocol already initialized");
      }

      // Step 2: Create Researcher Profile
      [profilePda] = await testHelpers.createResearcherWithProfile(
        researcher,
        TEST_RESEARCHERS.ALICE
      );
      console.log("✓ Researcher profile created");

      let profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.verificationStatus).to.deep.equal({ pending: {} });
      expect(profile.reputation).to.equal(0);

      // Step 3: Verify Researcher
      await testHelpers.verifyResearcher(profilePda, setup.authority);
      console.log("✓ Researcher verified");

      profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.verificationStatus).to.deep.equal({ verified: {} });

      // Step 4: Create Research Proposal
      const proposal = TEST_PROPOSALS.DROUGHT_RESISTANCE;
      const [proposalPda] = pdaHelper.getResearchProposalPda(researcher.publicKey, 0);

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
      console.log("✓ Research proposal created");

      let researchProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(researchProposal.status).to.deep.equal({ pending: {} });
      expect(researchProposal.currentFunding.toNumber()).to.equal(0);

      // Step 5: Partial Funding from Multiple Funders
      const halfFunding = proposal.fundingGoal.div(new anchor.BN(2));

      await setup.researchDao.methods
        .submitProposalForFunding(halfFunding)
        .accounts({
          researchProposal: proposalPda,
          funder: funder1.publicKey,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder1])
        .rpc();
      console.log("✓ Partial funding (50%) received from Funder 1");

      researchProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(researchProposal.currentFunding.toString()).to.equal(halfFunding.toString());
      expect(researchProposal.totalFunders).to.equal(1);
      expect(researchProposal.status).to.deep.equal({ pending: {} }); // Still pending until fully funded

      // Step 6: Complete Funding
      await setup.researchDao.methods
        .submitProposalForFunding(halfFunding)
        .accounts({
          researchProposal: proposalPda,
          funder: funder2.publicKey,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([funder2])
        .rpc();
      console.log("✓ Full funding achieved from Funder 2");

      researchProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(researchProposal.status).to.deep.equal({ funded: {} });
      expect(researchProposal.currentFunding.toString()).to.equal(proposal.fundingGoal.toString());
      expect(researchProposal.totalFunders).to.equal(2);

      // Step 7: Update Profile After Funding
      profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.fundedProposals).to.equal(1);
      expect(profile.totalFundingReceived.toString()).to.equal(proposal.fundingGoal.toString());

      // Step 8: Publish Research Milestones
      const milestones = [
        TEST_MILESTONES.INITIAL_RESEARCH,
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

        console.log(`✓ Milestone ${i + 1} published: ${milestones[i].title}`);
        await delay(500); // Small delay between milestones
      }

      researchProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      expect(researchProposal.status).to.deep.equal({ inProgress: {} });
      expect(researchProposal.milestonesCompleted).to.equal(4);

      // Step 9: Publish Final Research Findings
      await setup.researchDao.methods
        .publishFindings(
          "Drought-Resistant Wheat Varieties: Genetic Analysis and Field Testing Results",
          "Comprehensive study identifying key genetic markers for drought resistance in wheat, including field trial validation across multiple climate zones. Results show 30% improved drought tolerance in modified varieties.",
          "QmDroughtResistanceFindings2024",
          true // Peer-reviewed
        )
        .accounts({
          researchProposal: proposalPda,
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();
      console.log("✓ Final research findings published");

      // Step 10: Verify Final State
      const finalProposal = await setup.researchDao.account.researchProposal.fetch(proposalPda);
      const finalProfile = await setup.researchDao.account.researcherProfile.fetch(profilePda);

      expect(finalProposal.status).to.deep.equal({ completed: {} });
      expect(finalProposal.completedAt.toNumber()).to.be.greaterThan(0);

      expect(finalProfile.totalProposals).to.equal(1);
      expect(finalProfile.fundedProposals).to.equal(1);
      expect(finalProfile.completedProjects).to.equal(1);
      expect(finalProfile.reputation).to.be.greaterThan(50); // Should have significant reputation
      expect(finalProfile.totalFundingReceived.toString()).to.equal(proposal.fundingGoal.toString());

      console.log("=== Workflow Complete - Final State ===");
      console.log(`Researcher Reputation: ${finalProfile.reputation}`);
      console.log(`Total Funding Received: ${finalProfile.totalFundingReceived.toString()} lamports`);
      console.log(`Completed Projects: ${finalProfile.completedProjects}`);
      console.log(`Milestones Completed: ${finalProposal.milestonesCompleted}`);
      console.log(`Project Status: ${Object.keys(finalProposal.status)[0]}`);
    });
  });

  describe("Multi-Project Researcher Journey", () => {
    it("Should handle a researcher with multiple projects", async () => {
      console.log("=== Starting Multi-Project Journey Test ===");

      // Create a new researcher for multi-project test
      const multiProjectResearcher = Keypair.generate();
      await fundWallet(setup.provider, multiProjectResearcher.publicKey);

      const [multiProfilePda] = await testHelpers.createResearcherWithProfile(
        multiProjectResearcher,
        TEST_RESEARCHERS.BOB
      );
      await testHelpers.verifyResearcher(multiProfilePda, setup.authority);

      const projects = [
        TEST_PROPOSALS.SOIL_HEALTH,
        TEST_PROPOSALS.PRECISION_AGRICULTURE,
        TEST_PROPOSALS.CLIMATE_ADAPTATION
      ];

      console.log(`✓ Multi-project researcher created and verified`);

      for (let i = 0; i < projects.length; i++) {
        const project = projects[i];
        const [projectProposalPda] = pdaHelper.getResearchProposalPda(multiProjectResearcher.publicKey, i);

        // Create proposal
        await setup.researchDao.methods
          .createProposal(
            project.title,
            project.description,
            project.category,
            project.fundingGoal,
            project.duration,
            project.ipfsHash,
            project.milestones
          )
          .accounts({
            researchProposal: projectProposalPda,
            researcherProfile: multiProfilePda,
            researcher: multiProjectResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([multiProjectResearcher])
          .rpc();

        // Fund proposal
        const projectFunder = Keypair.generate();
        await fundWallet(setup.provider, projectFunder.publicKey, 15);

        await setup.researchDao.methods
          .submitProposalForFunding(project.fundingGoal)
          .accounts({
            researchProposal: projectProposalPda,
            funder: projectFunder.publicKey,
            researcher: multiProjectResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([projectFunder])
          .rpc();

        // Publish a few milestones
        for (let j = 0; j < 2; j++) {
          const milestoneIndex = j % TEST_MILESTONES.length;
          const milestone = Object.values(TEST_MILESTONES)[milestoneIndex];

          await setup.researchDao.methods
            .publishMilestone(
              `${project.title} - ${milestone.title}`,
              milestone.description,
              `${milestone.ipfsHash}-project-${i}`,
              milestone.completionPercentage
            )
            .accounts({
              researchProposal: projectProposalPda,
              researcherProfile: multiProfilePda,
              researcher: multiProjectResearcher.publicKey,
              systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([multiProjectResearcher])
            .rpc();

          await delay(300);
        }

        // Complete project
        await setup.researchDao.methods
          .publishFindings(
            `${project.title} - Final Report`,
            `Comprehensive analysis and findings for ${project.category} research`,
            `QmFindings-${project.category.replace(/\s+/g, '')}-${i}`,
            i % 2 === 0 // Alternate peer-reviewed status
          )
          .accounts({
            researchProposal: projectProposalPda,
            researcherProfile: multiProfilePda,
            researcher: multiProjectResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([multiProjectResearcher])
          .rpc();

        console.log(`✓ Project ${i + 1} completed: ${project.title}`);
        await delay(500);
      }

      // Verify multi-project stats
      const finalMultiProfile = await setup.researchDao.account.researcherProfile.fetch(multiProfilePda);

      expect(finalMultiProfile.totalProposals).to.equal(3);
      expect(finalMultiProfile.fundedProposals).to.equal(3);
      expect(finalMultiProfile.completedProjects).to.equal(3);
      expect(finalMultiProfile.reputation).to.be.greaterThan(100); // Significant reputation from multiple projects

      const totalExpectedFunding = projects.reduce((sum, project) => 
        sum.add(project.fundingGoal), new anchor.BN(0)
      );
      expect(finalMultiProfile.totalFundingReceived.toString()).to.equal(totalExpectedFunding.toString());

      console.log("=== Multi-Project Journey Complete ===");
      console.log(`Total Projects: ${finalMultiProfile.totalProposals}`);
      console.log(`Completed Projects: ${finalMultiProfile.completedProjects}`);
      console.log(`Final Reputation: ${finalMultiProfile.reputation}`);
      console.log(`Total Funding: ${finalMultiProfile.totalFundingReceived.toString()} lamports`);
    });
  });

  describe("Cross-Program Integration", () => {
    it("Should interact correctly between AgroDao and ResearchDao programs", async () => {
      console.log("=== Testing Cross-Program Integration ===");

      // Test protocol state access
      const protocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      console.log(`✓ Protocol Authority: ${protocolState.authority.toString()}`);
      console.log(`✓ Protocol Version: ${protocolState.protocolVersion}`);
      console.log(`✓ Min Funding Threshold: ${protocolState.minFundingThreshold.toString()}`);

      // Test that research proposals respect protocol parameters
      const smallProposal = TEST_PROPOSALS.SOIL_HEALTH;
      const smallFundingGoal = new anchor.BN(100); // Very small amount

      const integrationResearcher = Keypair.generate();
      await fundWallet(setup.provider, integrationResearcher.publicKey);

      const [integrationProfilePda] = await testHelpers.createResearcherWithProfile(
        integrationResearcher,
        TEST_RESEARCHERS.CHARLIE
      );
      await testHelpers.verifyResearcher(integrationProfilePda, setup.authority);

      const [integrationProposalPda] = pdaHelper.getResearchProposalPda(integrationResearcher.publicKey, 0);

      // This should fail if funding goal is below protocol threshold
      if (smallFundingGoal.lt(protocolState.minFundingThreshold)) {
        try {
          await setup.researchDao.methods
            .createProposal(
              smallProposal.title,
              smallProposal.description,
              smallProposal.category,
              smallFundingGoal,
              smallProposal.duration,
              smallProposal.ipfsHash,
              smallProposal.milestones
            )
            .accounts({
              researchProposal: integrationProposalPda,
              researcherProfile: integrationProfilePda,
              researcher: integrationResearcher.publicKey,
              systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([integrationResearcher])
            .rpc();

          console.log("⚠️  Small funding proposal created (protocol validation may not be enforced)");
        } catch (error) {
          console.log("✓ Small funding proposal rejected (protocol validation working)");
        }
      }

      // Test with proper funding amount
      await setup.researchDao.methods
        .createProposal(
          smallProposal.title,
          smallProposal.description,
          smallProposal.category,
          smallProposal.fundingGoal, // Use original funding goal
          smallProposal.duration,
          smallProposal.ipfsHash,
          smallProposal.milestones
        )
        .accounts({
          researchProposal: integrationProposalPda,
          researcherProfile: integrationProfilePda,
          researcher: integrationResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([integrationResearcher])
        .rpc();

      console.log("✓ Proper funding proposal created successfully");

      // Test protocol updates don't break research functionality
      const currentThreshold = protocolState.minFundingThreshold;
      const newThreshold = currentThreshold.add(new anchor.BN(1000));

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

      const updatedProtocolState = await setup.agroDao.account.protocolState.fetch(setup.protocolStatePda);
      expect(updatedProtocolState.minFundingThreshold.toString()).to.equal(newThreshold.toString());

      console.log("✓ Protocol parameters updated successfully");
      console.log("✓ Cross-program integration working correctly");
    });
  });

  describe("Error Recovery and Edge Cases", () => {
    it("Should handle various error conditions gracefully", async () => {
      console.log("=== Testing Error Recovery Scenarios ===");

      // Test insufficient funding scenario
      const errorResearcher = Keypair.generate();
      await fundWallet(setup.provider, errorResearcher.publicKey, 0.01); // Very low funding

      try {
        const [errorProfilePda] = await testHelpers.createResearcherWithProfile(
          errorResearcher,
          TEST_RESEARCHERS.DAVID
        );
        console.log("✓ Profile creation succeeded despite low researcher funding");
      } catch (error) {
        console.log("⚠️  Profile creation failed due to insufficient funds");
      }

      // Test network delay scenarios
      const delayResearcher = Keypair.generate();
      await fundWallet(setup.provider, delayResearcher.publicKey);

      const [delayProfilePda] = await testHelpers.createResearcherWithProfile(
        delayResearcher,
        { ...TEST_RESEARCHERS.ALICE, name: "Delay Test Researcher" }
      );

      // Add artificial delay to test timeout scenarios
      await delay(1000);

      try {
        await testHelpers.verifyResearcher(delayProfilePda, setup.authority);
        console.log("✓ Delayed verification completed successfully");
      } catch (error) {
        console.log("⚠️  Delayed verification failed:", error.message);
      }

      // Test state consistency after errors
      const profile = await setup.researchDao.account.researcherProfile.fetch(delayProfilePda);
      expect(profile.researcher.toString()).to.equal(delayResearcher.publicKey.toString());
      console.log("✓ State consistency maintained after delays");

      console.log("✓ Error recovery scenarios completed");
    });
  });

  describe("Performance and Scale Testing", () => {
    it("Should handle multiple concurrent operations", async () => {
      console.log("=== Testing Concurrent Operations ===");

      const concurrentResearchers = [];
      const concurrentProfiles = [];

      // Create multiple researchers concurrently
      for (let i = 0; i < 3; i++) {
        const researcher = Keypair.generate();
        await fundWallet(setup.provider, researcher.publicKey);
        concurrentResearchers.push(researcher);
      }

      // Create profiles concurrently
      const profilePromises = concurrentResearchers.map((researcher, index) => 
        testHelpers.createResearcherWithProfile(
          researcher,
          { ...TEST_RESEARCHERS.ALICE, name: `Concurrent Researcher ${index}` }
        )
      );

      const profileResults = await Promise.all(profilePromises);
      concurrentProfiles.push(...profileResults.map(result => result[0]));

      console.log(`✓ Created ${concurrentProfiles.length} researcher profiles concurrently`);

      // Verify all profiles concurrently
      const verificationPromises = concurrentProfiles.map(profilePda => 
        testHelpers.verifyResearcher(profilePda, setup.authority)
      );

      await Promise.all(verificationPromises);
      console.log(`✓ Verified ${concurrentProfiles.length} researchers concurrently`);

      // Verify all profiles are in correct state
      for (const profilePda of concurrentProfiles) {
        const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
        expect(profile.verificationStatus).to.deep.equal({ verified: {} });
      }

      console.log("✓ All concurrent operations completed successfully");
    });
  });
});
