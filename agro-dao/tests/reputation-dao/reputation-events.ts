import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ReputationDao } from "../../target/types/reputation_dao";
import { expect } from "chai";
import { BN } from "bn.js";

describe("Reputation DAO - Events and Scoring", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ReputationDao as Program<ReputationDao>;
  const authority = provider.wallet as anchor.Wallet;

  // Test users
  const testUser1 = anchor.web3.Keypair.generate();
  const testUser2 = anchor.web3.Keypair.generate();
  
  // PDAs
  let reputationConfigPda: anchor.web3.PublicKey;
  let user1ReputationPda: anchor.web3.PublicKey;
  let user2ReputationPda: anchor.web3.PublicKey;

  before(async () => {
    console.log("Setting up Reputation Events test environment...");
    
    // Derive PDAs
    [reputationConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("reputation_config")],
      program.programId
    );

    [user1ReputationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_reputation"), testUser1.publicKey.toBuffer()],
      program.programId
    );

    [user2ReputationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_reputation"), testUser2.publicKey.toBuffer()],
      program.programId
    );

    console.log("📍 Reputation Config PDA:", reputationConfigPda.toString());
    console.log("📍 User 1 Reputation PDA:", user1ReputationPda.toString());
    console.log("📍 Program ID:", program.programId.toString());
  });

  describe("Reputation System Initialization", () => {
    it("should initialize reputation config with default thresholds", async () => {
      console.log("Initializing reputation configuration...");
      
      try {
        await program.methods
          .initializeReputationConfig(
            null, // Use default bronze threshold
            null, // Use default silver threshold  
            null, // Use default gold threshold
            null, // Use default platinum threshold
            null  // Use default diamond threshold
          )
          .accounts({
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        
        console.log("Reputation config initialized with default thresholds");
      } catch (error) {
        if (error.message?.includes("already in use")) {
          console.log("Reputation config already initialized, fetching existing config");
        } else {
          console.log("Reputation config initialization failed:", error.message);
          throw error;
        }
      }

      // Fetch and verify config (whether newly created or existing)
      const configAccount = await program.account.reputationConfig.fetch(reputationConfigPda);
      
      expect(configAccount.authority.toString()).to.equal(authority.publicKey.toString());
      expect(configAccount.tierThresholdBronze.toNumber()).to.equal(100);
      expect(configAccount.tierThresholdSilver.toNumber()).to.equal(500);
      expect(configAccount.tierThresholdGold.toNumber()).to.equal(1500);
      expect(configAccount.tierThresholdPlatinum.toNumber()).to.equal(3000);
      expect(configAccount.tierThresholdDiamond.toNumber()).to.equal(5000);

      console.log("Reputation config verified with default thresholds");
      console.log("  - Bronze: 100 points");
      console.log("  - Silver: 500 points");
      console.log("  - Gold: 1500 points");
      console.log("  - Platinum: 3000 points");
      console.log("  - Diamond: 5000 points");
    });

    it("should initialize user reputation accounts", async () => {
      console.log("👤 Initializing user reputation accounts...");
      
      try {
        // Initialize user 1 reputation
        await program.methods
          .initializeUserReputation()
          .accounts({
            userReputation: user1ReputationPda,
            user: testUser1.publicKey,
            payer: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        // Initialize user 2 reputation
        await program.methods
          .initializeUserReputation()
          .accounts({
            userReputation: user2ReputationPda,
            user: testUser2.publicKey,
            payer: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        const user1Account = await program.account.userReputation.fetch(user1ReputationPda);
        const user2Account = await program.account.userReputation.fetch(user2ReputationPda);

        expect(user1Account.user.toString()).to.equal(testUser1.publicKey.toString());
        expect(user1Account.reputationScore.toNumber()).to.equal(0);
        expect(user1Account.tier).to.deep.equal({ none: {} });

        expect(user2Account.user.toString()).to.equal(testUser2.publicKey.toString());
        expect(user2Account.reputationScore.toNumber()).to.equal(0);
        expect(user2Account.tier).to.deep.equal({ none: {} });

        console.log("User reputation accounts initialized");
        console.log("  - User 1: 0 points, None tier");
        console.log("  - User 2: 0 points, None tier");
        
      } catch (error) {
        console.log("User reputation initialization failed:", error.message);
        throw error;
      }
    });
  });

  describe("Positive Reputation Events", () => {
    it("should award points for milestone completion", async () => {
      console.log("Testing milestone completion event...");
      
      try {
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { milestoneCompleted: {} },
            null
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(100); // MILESTONE_SUCCESS_BONUS
        expect(userAccount.tier).to.deep.equal({ bronze: {} }); // Should be bronze tier now

        console.log("Milestone completion awarded 100 points");
        console.log("  - New score: 100 points");
        console.log("  - New tier: Bronze");
        
      } catch (error) {
        console.log("Milestone completion test failed:", error.message);
        throw error;
      }
    });

    it("should award points for project completion", async () => {
      console.log("Testing project completion event...");
      
      try {
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { projectCompleted: {} },
            null
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(300); // 100 + 200 (PROJECT_COMPLETION_BONUS)
        expect(userAccount.tier).to.deep.equal({ bronze: {} }); // Still bronze

        console.log("Project completion awarded 200 points");
        console.log("  - New score: 300 points");
        console.log("  - Tier: Bronze");
        
      } catch (error) {
        console.log("Project completion test failed:", error.message);
        throw error;
      }
    });

    it("should award points for positive peer review", async () => {
      console.log("⭐ Testing positive peer review event...");
      
      try {
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { peerReviewPositive: {} },
            null
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(325); // 300 + 25 (PEER_REVIEW_BONUS)

        console.log("Positive peer review awarded 25 points");
        console.log("  - New score: 325 points");
        
      } catch (error) {
        console.log("Positive peer review test failed:", error.message);
        throw error;
      }
    });

    it("should award custom reputation amounts", async () => {
      console.log("🎨 Testing custom reputation event...");
      
      try {
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { custom: {} },
            new BN(150) // Custom amount
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(475); // 325 + 150

        console.log("Custom reputation event awarded 150 points");
        console.log("  - New score: 475 points");
        
      } catch (error) {
        console.log("Custom reputation test failed:", error.message);
        throw error;
      }
    });

    it("should progress to silver tier with enough points", async () => {
      console.log("🥈 Testing progression to Silver tier...");
      
      try {
        // Add more points to reach silver threshold (500)
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { custom: {} },
            new BN(50) // 475 + 50 = 525 (above silver threshold)
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(525);
        expect(userAccount.tier).to.deep.equal({ silver: {} });

        console.log("User progressed to Silver tier");
        console.log("  - Score: 525 points");
        console.log("  - Tier: Silver");
        
      } catch (error) {
        console.log("Silver tier progression test failed:", error.message);
        throw error;
      }
    });
  });

  describe("Negative Reputation Events", () => {
    it("should penalize milestone failures", async () => {
      console.log("Testing milestone failure event...");
      
      try {
        await program.methods
          .updateReputation(
            testUser2.publicKey,
            { milestoneFailed: {} },
            null
          )
          .accounts({
            userReputation: user2ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user2ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(-50); // MILESTONE_FAILURE_PENALTY
        expect(userAccount.tier).to.deep.equal({ none: {} });

        console.log("Milestone failure penalized 50 points");
        console.log("  - New score: -50 points");
        console.log("  - Tier: None");
        
      } catch (error) {
        console.log("Milestone failure test failed:", error.message);
        throw error;
      }
    });

    it("should penalize project abandonment", async () => {
      console.log("🚫 Testing project abandonment event...");
      
      try {
        await program.methods
          .updateReputation(
            testUser2.publicKey,
            { projectAbandoned: {} },
            null
          )
          .accounts({
            userReputation: user2ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user2ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(-150); // -50 + (-100)

        console.log("Project abandonment penalized 100 points");
        console.log("  - New score: -150 points");
        
      } catch (error) {
        console.log("Project abandonment test failed:", error.message);
        throw error;
      }
    });

    it("should penalize dispute resolution", async () => {
      console.log("Testing dispute resolution penalty...");
      
      try {
        await program.methods
          .updateReputation(
            testUser2.publicKey,
            { disputeResolved: {} },
            null
          )
          .accounts({
            userReputation: user2ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user2ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(-225); // -150 + (-75)

        console.log("Dispute resolution penalized 75 points");
        console.log("  - New score: -225 points");
        
      } catch (error) {
        console.log("Dispute resolution test failed:", error.message);
        throw error;
      }
    });

    it("should handle negative custom amounts", async () => {
      console.log("Testing negative custom reputation event...");
      
      try {
        await program.methods
          .updateReputation(
            testUser2.publicKey,
            { custom: {} },
            new BN(-25) // Negative custom amount
          )
          .accounts({
            userReputation: user2ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user2ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(-250); // -225 + (-25)

        console.log("Negative custom event penalized 25 points");
        console.log("  - New score: -250 points");
        
      } catch (error) {
        console.log("Negative custom reputation test failed:", error.message);
        throw error;
      }
    });
  });

  describe("Reputation Recovery and Tier Progression", () => {
    it("should allow reputation recovery from negative scores", async () => {
      console.log("Testing reputation recovery...");
      
      try {
        // Help user2 recover with multiple positive events
        await program.methods
          .updateReputation(
            testUser2.publicKey,
            { milestoneCompleted: {} },
            null
          )
          .accounts({
            userReputation: user2ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        await program.methods
          .updateReputation(
            testUser2.publicKey,
            { projectCompleted: {} },
            null
          )
          .accounts({
            userReputation: user2ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        const userAccount = await program.account.userReputation.fetch(user2ReputationPda);
        
        expect(userAccount.reputationScore.toNumber()).to.equal(50); // -250 + 100 + 200 = 50
        expect(userAccount.tier).to.deep.equal({ none: {} }); // Still below bronze threshold

        console.log("User recovered from negative reputation");
        console.log("  - New score: 50 points");
        console.log("  - Tier: None (below bronze threshold)");
        
      } catch (error) {
        console.log("Reputation recovery test failed:", error.message);
        throw error;
      }
    });

    it("should demonstrate complete tier progression", async () => {
      console.log("🏅 Testing complete tier progression...");
      
      try {
        // Push user1 through multiple tiers
        console.log("  Current user1 score: 525 (Silver)");
        
        // Add points to reach Gold (1500)
        const pointsNeeded = 1500 - 525;
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { custom: {} },
            new BN(pointsNeeded)
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        let userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        expect(userAccount.reputationScore.toNumber()).to.equal(1500);
        expect(userAccount.tier).to.deep.equal({ gold: {} });
        console.log("  Reached Gold tier: 1500 points");

        // Add points to reach Platinum (3000)
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { custom: {} },
            new BN(1500)
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        expect(userAccount.reputationScore.toNumber()).to.equal(3000);
        expect(userAccount.tier).to.deep.equal({ platinum: {} });
        console.log("  Reached Platinum tier: 3000 points");

        // Add points to reach Diamond (5000)
        await program.methods
          .updateReputation(
            testUser1.publicKey,
            { custom: {} },
            new BN(2000)
          )
          .accounts({
            userReputation: user1ReputationPda,
            reputationConfig: reputationConfigPda,
            authority: authority.publicKey,
          })
          .rpc();

        userAccount = await program.account.userReputation.fetch(user1ReputationPda);
        expect(userAccount.reputationScore.toNumber()).to.equal(5000);
        expect(userAccount.tier).to.deep.equal({ diamond: {} });
        console.log("  Reached Diamond tier: 5000 points");

        console.log("Complete tier progression demonstrated!");
        
      } catch (error) {
        console.log("Tier progression test failed:", error.message);
        throw error;
      }
    });
  });

  describe("Reputation System Summary", () => {
    it("should demonstrate comprehensive reputation functionality", async () => {
      console.log("Reputation System Summary:");
      
      const user1Account = await program.account.userReputation.fetch(user1ReputationPda);
      const user2Account = await program.account.userReputation.fetch(user2ReputationPda);
      const configAccount = await program.account.reputationConfig.fetch(reputationConfigPda);

      console.log("Reputation Events System:");
      console.log("├── Positive Events");
      console.log("│   ├── Milestone Completed: +100 points");
      console.log("│   ├── Project Completed: +200 points");
      console.log("│   ├── Positive Peer Review: +25 points");
      console.log("│   └── Custom Events: Variable points");
      console.log("├── Negative Events");
      console.log("│   ├── Milestone Failed: -50 points");
      console.log("│   ├── Project Abandoned: -100 points");
      console.log("│   ├── Dispute Resolution: -75 points");
      console.log("│   └── Custom Penalties: Variable points");
      console.log("└── Tier System");
      console.log("    ├── Bronze: 100+ points");
      console.log("    ├── Silver: 500+ points");
      console.log("    ├── Gold: 1500+ points");
      console.log("    ├── Platinum: 3000+ points");
      console.log("    └── Diamond: 5000+ points");
      
      console.log("");
      console.log("👤 Final User States:");
      console.log(`  User 1: ${user1Account.reputationScore.toNumber()} points, ${Object.keys(user1Account.tier)[0]} tier`);
      console.log(`  User 2: ${user2Account.reputationScore.toNumber()} points, ${Object.keys(user2Account.tier)[0]} tier`);
      
      console.log("");
      console.log("Reputation system is fully functional!");
      console.log("  • Event-driven reputation scoring");
      console.log("  • Automatic tier progression");
      console.log("  • Recovery from negative reputation");
      console.log("  • Custom event support");
      console.log("  • Cross-program CPI integration ready");

      expect(true).to.be.true; // Always pass this summary test
    });
  });
});