import * as anchor from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair } from "@solana/web3.js";
import { setupTestEnvironment, TestSetup, fundWallet, delay } from "../utils/setup";
import { TEST_RESEARCHERS, TEST_CONSTANTS } from "../utils/constants";
import { PDAHelper } from "../utils/helpers";

describe("ResearchDao - Researcher Profiles", () => {
  let setup: TestSetup;
  let testResearcher: Keypair;
  let adminUser: Keypair;
  let pdaHelper: PDAHelper;

  before(async () => {
    setup = await setupTestEnvironment();
    testResearcher = Keypair.generate();
    adminUser = Keypair.generate();
    pdaHelper = new PDAHelper(setup.researchDao.programId);

    await fundWallet(setup.provider, testResearcher.publicKey);
    await fundWallet(setup.provider, adminUser.publicKey);
  });

  describe("Profile Creation", () => {
    it("Should create a researcher profile successfully", async () => {
      const researcher = TEST_RESEARCHERS.ALICE;
      const [profilePda] = pdaHelper.getResearcherProfilePda(testResearcher.publicKey);

      await setup.researchDao.methods
        .createResearcherProfile(
          researcher.name,
          researcher.affiliation,
          researcher.specialization,
          researcher.contactInfo
        )
        .accounts({
          researcherProfile: profilePda,
          researcher: testResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([testResearcher])
        .rpc();

      const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);

      expect(profile.researcher.toString()).to.equal(testResearcher.publicKey.toString());
      expect(profile.name).to.equal(researcher.name);
      expect(profile.affiliation).to.equal(researcher.affiliation);
      expect(profile.specialization).to.equal(researcher.specialization);
      expect(profile.contactInfo).to.equal(researcher.contactInfo);
      expect(profile.verificationStatus).to.deep.equal({ pending: {} });
      expect(profile.reputation).to.equal(0);
      expect(profile.totalProposals).to.equal(0);
      expect(profile.fundedProposals).to.equal(0);
      expect(profile.completedProjects).to.equal(0);
      expect(profile.totalFundingReceived.toNumber()).to.equal(0);
      expect(profile.createdAt.toNumber()).to.be.greaterThan(0);
    });

    it("Should prevent duplicate profile creation", async () => {
      const researcher = TEST_RESEARCHERS.BOB;
      const [profilePda] = pdaHelper.getResearcherProfilePda(testResearcher.publicKey);

      try {
        await setup.researchDao.methods
          .createResearcherProfile(
            researcher.name,
            researcher.affiliation,
            researcher.specialization,
            researcher.contactInfo
          )
          .accounts({
            researcherProfile: profilePda,
            researcher: testResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([testResearcher])
          .rpc();

        expect.fail("Should have thrown error for duplicate profile");
      } catch (error) {
        expect(error.message).to.include("already in use");
      }
    });

    it("Should validate profile input data", async () => {
      const newResearcher = Keypair.generate();
      await fundWallet(setup.provider, newResearcher.publicKey);
      const [profilePda] = pdaHelper.getResearcherProfilePda(newResearcher.publicKey);

      // Test empty name
      try {
        await setup.researchDao.methods
          .createResearcherProfile(
            "", // Empty name
            "University",
            "AI",
            "email@example.com"
          )
          .accounts({
            researcherProfile: profilePda,
            researcher: newResearcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([newResearcher])
          .rpc();

        expect.fail("Should have thrown error for empty name");
      } catch (error) {
        expect(error.message).to.include("InvalidProfileData");
      }
    });

    it("Should handle profile creation with different specializations", async () => {
      const specializations = [
        "Crop Science",
        "Soil Science", 
        "Plant Pathology",
        "Agricultural Engineering",
        "Sustainable Agriculture"
      ];

      for (let i = 0; i < specializations.length; i++) {
        const researcher = Keypair.generate();
        await fundWallet(setup.provider, researcher.publicKey);
        const [profilePda] = pdaHelper.getResearcherProfilePda(researcher.publicKey);

        await setup.researchDao.methods
          .createResearcherProfile(
            `Researcher ${i}`,
            `Institution ${i}`,
            specializations[i],
            `researcher${i}@example.com`
          )
          .accounts({
            researcherProfile: profilePda,
            researcher: researcher.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([researcher])
          .rpc();

        const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
        expect(profile.specialization).to.equal(specializations[i]);
        expect(profile.verificationStatus).to.deep.equal({ pending: {} });
      }
    });
  });

  describe("Profile Verification", () => {
    let unverifiedResearcher: Keypair;
    let unverifiedProfilePda: anchor.web3.PublicKey;

    before(async () => {
      unverifiedResearcher = Keypair.generate();
      await fundWallet(setup.provider, unverifiedResearcher.publicKey);
      [unverifiedProfilePda] = pdaHelper.getResearcherProfilePda(unverifiedResearcher.publicKey);

      // Create unverified profile
      const researcher = TEST_RESEARCHERS.CHARLIE;
      await setup.researchDao.methods
        .createResearcherProfile(
          researcher.name,
          researcher.affiliation,
          researcher.specialization,
          researcher.contactInfo
        )
        .accounts({
          researcherProfile: unverifiedProfilePda,
          researcher: unverifiedResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([unverifiedResearcher])
        .rpc();
    });

    it("Should verify a researcher profile", async () => {
      await setup.researchDao.methods
        .verifyResearcher()
        .accounts({
          researcherProfile: unverifiedProfilePda,
          verifier: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      const profile = await setup.researchDao.account.researcherProfile.fetch(
        unverifiedProfilePda
      );

      expect(profile.verificationStatus).to.deep.equal({ verified: {} });
      expect(profile.verifiedAt.toNumber()).to.be.greaterThan(0);
      expect(profile.verifiedBy.toString()).to.equal(setup.authority.publicKey.toString());
    });

    it("Should prevent unauthorized verification", async () => {
      const unauthorizedUser = Keypair.generate();
      await fundWallet(setup.provider, unauthorizedUser.publicKey);

      const newResearcher = Keypair.generate();
      await fundWallet(setup.provider, newResearcher.publicKey);
      const [profilePda] = pdaHelper.getResearcherProfilePda(newResearcher.publicKey);

      // Create profile
      await setup.researchDao.methods
        .createResearcherProfile(
          "Test Researcher",
          "Test University",
          "Test Field",
          "test@example.com"
        )
        .accounts({
          researcherProfile: profilePda,
          researcher: newResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([newResearcher])
        .rpc();

      // Try to verify with unauthorized user
      try {
        await setup.researchDao.methods
          .verifyResearcher()
          .accounts({
            researcherProfile: profilePda,
            verifier: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unauthorizedUser])
          .rpc();

        expect.fail("Should have thrown error for unauthorized verification");
      } catch (error) {
        expect(error.message).to.include("UnauthorizedVerification");
      }
    });

    it("Should prevent verification of already verified profile", async () => {
      try {
        await setup.researchDao.methods
          .verifyResearcher()
          .accounts({
            researcherProfile: unverifiedProfilePda,
            verifier: setup.authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([setup.authority])
          .rpc();

        expect.fail("Should have thrown error for already verified profile");
      } catch (error) {
        expect(error.message).to.include("AlreadyVerified");
      }
    });
  });

  describe("Profile Data Integrity", () => {
    it("Should maintain profile data after verification", async () => {
      const researcher = TEST_RESEARCHERS.DAVID;
      const newResearcher = Keypair.generate();
      await fundWallet(setup.provider, newResearcher.publicKey);
      const [profilePda] = pdaHelper.getResearcherProfilePda(newResearcher.publicKey);

      // Create profile
      await setup.researchDao.methods
        .createResearcherProfile(
          researcher.name,
          researcher.affiliation,
          researcher.specialization,
          researcher.contactInfo
        )
        .accounts({
          researcherProfile: profilePda,
          researcher: newResearcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([newResearcher])
        .rpc();

      const profileBefore = await setup.researchDao.account.researcherProfile.fetch(profilePda);

      // Verify profile
      await setup.researchDao.methods
        .verifyResearcher()
        .accounts({
          researcherProfile: profilePda,
          verifier: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      const profileAfter = await setup.researchDao.account.researcherProfile.fetch(profilePda);

      // Verify data integrity
      expect(profileAfter.researcher.toString()).to.equal(profileBefore.researcher.toString());
      expect(profileAfter.name).to.equal(profileBefore.name);
      expect(profileAfter.affiliation).to.equal(profileBefore.affiliation);
      expect(profileAfter.specialization).to.equal(profileBefore.specialization);
      expect(profileAfter.contactInfo).to.equal(profileBefore.contactInfo);
      expect(profileAfter.reputation).to.equal(profileBefore.reputation);
      expect(profileAfter.createdAt.toString()).to.equal(profileBefore.createdAt.toString());
    });

    it("Should handle profile state transitions correctly", async () => {
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);
      const [profilePda] = pdaHelper.getResearcherProfilePda(researcher.publicKey);

      // Create profile
      await setup.researchDao.methods
        .createResearcherProfile(
          "State Test",
          "Test Institution",
          "Agricultural Science",
          "state@test.com"
        )
        .accounts({
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      let profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.verificationStatus).to.deep.equal({ pending: {} });

      // Verify profile
      await setup.researchDao.methods
        .verifyResearcher()
        .accounts({
          researcherProfile: profilePda,
          verifier: setup.authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([setup.authority])
        .rpc();

      profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);
      expect(profile.verificationStatus).to.deep.equal({ verified: {} });
      expect(profile.verifiedAt.toNumber()).to.be.greaterThan(0);
      expect(profile.verifiedBy.toString()).to.equal(setup.authority.publicKey.toString());
    });
  });

  describe("Profile Analytics", () => {
    it("Should track profile creation timestamps", async () => {
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);
      const [profilePda] = pdaHelper.getResearcherProfilePda(researcher.publicKey);

      const beforeTime = Math.floor(Date.now() / 1000);

      await setup.researchDao.methods
        .createResearcherProfile(
          "Analytics Test",
          "Test University",
          "Data Science",
          "analytics@test.com"
        )
        .accounts({
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      const afterTime = Math.floor(Date.now() / 1000);
      const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);

      expect(profile.createdAt.toNumber()).to.be.at.least(beforeTime);
      expect(profile.createdAt.toNumber()).to.be.at.most(afterTime + 5); // Allow 5 second buffer
    });

    it("Should initialize metrics correctly", async () => {
      const researcher = Keypair.generate();
      await fundWallet(setup.provider, researcher.publicKey);
      const [profilePda] = pdaHelper.getResearcherProfilePda(researcher.publicKey);

      await setup.researchDao.methods
        .createResearcherProfile(
          "Metrics Test",
          "Test Institution",
          "Research Metrics",
          "metrics@test.com"
        )
        .accounts({
          researcherProfile: profilePda,
          researcher: researcher.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([researcher])
        .rpc();

      const profile = await setup.researchDao.account.researcherProfile.fetch(profilePda);

      expect(profile.reputation).to.equal(0);
      expect(profile.totalProposals).to.equal(0);
      expect(profile.fundedProposals).to.equal(0);
      expect(profile.completedProjects).to.equal(0);
      expect(profile.totalFundingReceived.toNumber()).to.equal(0);
    });
  });
});
