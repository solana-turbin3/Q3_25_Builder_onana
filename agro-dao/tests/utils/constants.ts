import { BN } from "@coral-xyz/anchor";

export const TEST_CONSTANTS = {
  // Protocol constants
  MIN_FUNDING_THRESHOLD: new BN(1_000_000), // 0.001 SOL
  RESEARCH_PROPOSAL_FEE: new BN(10_000),    // 0.00001 SOL
  MINIMUM_STAKED_AMOUNT: new BN(100_000),   // 0.0001 SOL
  
  // Researcher test data
  RESEARCHER_NAME: "Dr. Jane Smith",
  RESEARCHER_BIO: "Agricultural researcher with 10 years experience in sustainable farming practices",
  RESEARCHER_SPECIALIZATION: "Sustainable Farming Techniques",
  
  // Proposal test data
  PROPOSAL_TITLE: "Sustainable Crop Rotation Study",
  PROPOSAL_DESCRIPTION: "A comprehensive study on sustainable crop rotation techniques to improve soil health and crop yield while reducing environmental impact",
  FUNDING_TARGET: new BN(5_000_000), // 0.005 SOL
  
  // IPFS hashes (mock data - all values under 255 for u8 arrays)
  MOCK_IPFS_HASH: Array.from({ length: 32 }, (_, i) => (i * 7) % 256),
  MOCK_EVIDENCE_HASH: Array.from({ length: 32 }, (_, i) => (i * 11) % 256),
  MOCK_FINDINGS_HASH: Array.from({ length: 32 }, (_, i) => (i * 13) % 256),
  
  // Time constants
  SECONDS_PER_DAY: 24 * 60 * 60,
  FUNDING_DEADLINE_DAYS: 30,
  
  // Reputation constants
  VERIFICATION_BONUS: 100,
  MILESTONE_BONUS: 5,
  COMPLETION_BONUS: 20,
  FINDINGS_BONUS: 50,
};

export const SAMPLE_MILESTONES = [
  {
    description: "Initial data collection and soil sampling",
    targetDate: new BN(Math.floor(Date.now() / 1000) + (30 * TEST_CONSTANTS.SECONDS_PER_DAY)),
    completionDate: null,
    isCompleted: false,
    ipfsEvidenceHash: null,
  },
  {
    description: "Laboratory analysis and testing phase",
    targetDate: new BN(Math.floor(Date.now() / 1000) + (60 * TEST_CONSTANTS.SECONDS_PER_DAY)),
    completionDate: null,
    isCompleted: false,
    ipfsEvidenceHash: null,
  },
  {
    description: "Field implementation and monitoring",
    targetDate: new BN(Math.floor(Date.now() / 1000) + (90 * TEST_CONSTANTS.SECONDS_PER_DAY)),
    completionDate: null,
    isCompleted: false,
    ipfsEvidenceHash: null,
  },
];

export const RESEARCH_CATEGORIES = {
  CROP_IMPROVEMENT: { cropImprovement: {} },
  SUSTAINABLE_FARMING: { sustainableFarming: {} },
  PEST_CONTROL: { pestControl: {} },
  SOIL_HEALTH: { soilHealth: {} },
  CLIMATE_ADAPTATION: { climateAdaptation: {} },
  WATER_MANAGEMENT: { waterManagement: {} },
};

export const PROPOSAL_STATUSES = {
  DRAFT: { draft: {} },
  SUBMITTED_FOR_FUNDING: { submittedForFunding: {} },
  FUNDING_ACTIVE: { fundingActive: {} },
  FUNDED: { funded: {} },
  IN_PROGRESS: { inProgress: {} },
  COMPLETED: { completed: {} },
  CANCELLED: { cancelled: {} },
};
