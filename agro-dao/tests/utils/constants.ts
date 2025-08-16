import BN from "bn.js";

// Reputation system constants
export const REPUTATION_CONFIG_SEED = "reputation_config";
export const DEFAULT_MIN_REPUTATION = 0;
export const DEFAULT_MAX_REPUTATION = 50000;
export const DEFAULT_DECAY_RATE = 5;
export const DEFAULT_COOLDOWN_PERIOD = 86400; // 24 hours in seconds

// Reputation event values
export const REPUTATION_VALUES = {
  RESEARCHER_VERIFICATION: 50,
  PROPOSAL_SUBMISSION: 10,
  MILESTONE_COMPLETION_ON_TIME: 25,
  MILESTONE_COMPLETION_LATE: 10,
  PROJECT_COMPLETION: 100,
  PEER_REVIEW_PARTICIPATION: 15,
  VOTE_PARTICIPATION: 5,
  PROPOSAL_CREATION: 20,
  SUCCESSFUL_PROPOSAL: 50,
  FIRST_TIME_STAKING: 30,
  QUARTERLY_STAKING: 15,
  REPUTATION_PER_1000_AGRO: 5,
  // Negative events
  MISSED_DEADLINE: -20,
  PROJECT_ABANDONMENT: -75,
  COMMUNITY_VIOLATION: -50,
  FRAUDULENT_ACTIVITY: -200,
};

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
  
  // IPFS content hashes (deterministic test data)
  SAMPLE_IPFS_HASH: Array.from({ length: 32 }, (_, i) => (i * 7) % 256),
  SAMPLE_EVIDENCE_HASH: Array.from({ length: 32 }, (_, i) => (i * 11) % 256),
  SAMPLE_FINDINGS_HASH: Array.from({ length: 32 }, (_, i) => (i * 13) % 256),
  
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

// Common 32-byte content hash for testing
export const SAMPLE_HASH = Array.from({ length: 32 }, (_, i) => (i * 17) % 256);

export const TEST_MILESTONES = {
  INITIAL_RESEARCH: {
    title: "Initial Research",
    description: "Kickoff and baseline study",
    ipfsHash: "QmMilestoneInitial",
    completionPercentage: 25,
  },
  FIELD_TRIALS: {
    title: "Field Trials",
    description: "Run field experiments",
    ipfsHash: "QmMilestoneField",
    completionPercentage: 50,
  },
  ANALYSIS: {
    title: "Analysis",
    description: "Analyze collected data",
    ipfsHash: "QmMilestoneAnalysis",
    completionPercentage: 75,
  },
};

export const TEST_PROPOSALS = {
  DROUGHT_RESISTANCE: {
    title: "Drought Resistance Study",
    description: "Identify drought resistant traits",
    category: RESEARCH_CATEGORIES.CROP_IMPROVEMENT,
    fundingGoal: new BN(1_000_000),
    duration: new BN(Math.floor(Date.now() / 1000) + 30 * TEST_CONSTANTS.SECONDS_PER_DAY),
    ipfsHash: TEST_CONSTANTS.SAMPLE_IPFS_HASH,
    milestones: [],
  },
  SOIL_HEALTH: {
    title: "Soil Health Improvement",
    description: "Improve soil microbiome",
    category: RESEARCH_CATEGORIES.SOIL_HEALTH,
    fundingGoal: new BN(2_000_000),
    duration: new BN(Math.floor(Date.now() / 1000) + 45 * TEST_CONSTANTS.SECONDS_PER_DAY),
    ipfsHash: TEST_CONSTANTS.SAMPLE_EVIDENCE_HASH,
    milestones: [],
  },
  PRECISION_AGRICULTURE: {
    title: "Precision Agriculture Sensors",
    description: "Deploy IoT sensors for precision ag",
    category: RESEARCH_CATEGORIES.WATER_MANAGEMENT,
    fundingGoal: new BN(3_000_000),
    duration: new BN(Math.floor(Date.now() / 1000) + 60 * TEST_CONSTANTS.SECONDS_PER_DAY),
    ipfsHash: TEST_CONSTANTS.SAMPLE_FINDINGS_HASH,
    milestones: [],
  },
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

export const TEST_RESEARCHERS = {
  ALICE: {
    name: "Dr. Alice Johnson",
    affiliation: "Agricultural University",
    specialization: "Sustainable Farming",
    contactInfo: "alice@agri-university.edu"
  },
  BOB: {
    name: "Dr. Bob Chen",
    affiliation: "Research Institute",
    specialization: "Crop Science",
    contactInfo: "bob@research-institute.org"
  },
  CHARLIE: {
    name: "Dr. Charlie Williams",
    affiliation: "Tech University",
    specialization: "Agricultural Technology",
    contactInfo: "charlie@tech-university.edu"
  }
};
