// ===========================================
// @settle/types
// Shared TypeScript types for the Settle protocol.
// Mirrors contracts/stellar-contract/src/types.rs
// ===========================================

// --- Agreement ---

export type AgreementStatus =
  | "draft"
  | "funded"
  | "active"
  | "completed"
  | "disputed"
  | "resolved"
  | "expired"
  | "cancelled";

export interface Agreement {
  id: string;
  creator: string;          // Stellar address
  counterparty: string;     // Stellar address
  token: string;            // Soroban token contract address
  totalAmount: bigint;
  fundedAmount: bigint;
  releasedAmount: bigint;
  refundedAmount: bigint;
  status: AgreementStatus;
  createdAt: bigint;        // unix timestamp (seconds)
  expiresAt: bigint;
  milestones: string[];     // milestone IDs
}

export interface CreateAgreementParams {
  id: string;
  counterparty: string;
  token: string;
  totalAmount: bigint;
  expiresAt: bigint;
  milestones: CreateMilestoneParams[];
}

// --- Milestone ---

export type MilestoneStatus =
  | "pending"
  | "submitted"
  | "approved"
  | "rejected"
  | "released";

export interface Milestone {
  id: string;
  agreementId: string;
  name: string;
  description: string;
  amount: bigint;
  status: MilestoneStatus;
  dueDate: bigint;
  submittedAt: bigint | null;
  approvedAt: bigint | null;
  evidence: string | null;
}

export interface CreateMilestoneParams {
  id: string;
  name: string;
  description: string;
  amount: bigint;
  dueDate: bigint;
}

// --- Escrow ---

export type EscrowStatus =
  | "empty"
  | "funded"
  | "locked"
  | "released"
  | "refunded";

export interface Escrow {
  agreementId: string;
  token: string;
  amount: bigint;
  lockedAmount: bigint;
  releasedAmount: bigint;
  status: EscrowStatus;
  createdAt: bigint;
  lastActionAt: bigint;
}

// --- Dispute ---

export type DisputeStatus =
  | "open"
  | "evidence_submission"
  | "under_review"
  | "resolved"
  | "closed";

export interface Dispute {
  id: string;
  agreementId: string;
  openedBy: string;         // Stellar address
  reason: string;
  evidence: string[];
  status: DisputeStatus;
  resolution: string | null;
  openedAt: bigint;
  resolvedAt: bigint | null;
  arbitrator: string | null;
}

// --- Participant / Reputation ---

export interface Participant {
  address: string;
  reputationScore: number;
  totalAgreements: number;
  successfulAgreements: number;
  disputedAgreements: number;
  totalVolume: bigint;
  joinedAt: bigint;
  lastActivity: bigint;
}

export interface ReputationScore {
  address: string;
  score: number;
  totalAgreements: number;
  successfulAgreements: number;
  disputedAgreements: number;
  totalVolume: bigint;
}

// --- Events ---

export type EventType =
  | "agreement_created"
  | "agreement_funded"
  | "agreement_activated"
  | "agreement_completed"
  | "agreement_expired"
  | "agreement_cancelled"
  | "milestone_created"
  | "milestone_submitted"
  | "milestone_approved"
  | "milestone_rejected"
  | "milestone_released"
  | "escrow_funded"
  | "escrow_locked"
  | "escrow_released"
  | "escrow_refunded"
  | "dispute_opened"
  | "evidence_submitted"
  | "dispute_resolved"
  | "dispute_closed"
  | "reputation_updated";

export interface SettleEvent {
  eventType: EventType;
  agreementId: string | null;
  milestoneId: string | null;
  disputeId: string | null;
  participant: string;
  data: string;             // base64 encoded
  timestamp: bigint;
  blockHeight: number;
}

// --- API / SDK通用 types ---

export interface PaginatedResult<T> {
  data: T[];
  total: number;
  page: number;
  pageSize: number;
  hasMore: boolean;
}

export interface TxResult {
  hash: string;
  ledger: number;
  success: boolean;
  error?: string;
}

export interface SettleConfig {
  network: "testnet" | "mainnet";
  rpcUrl: string;
  networkPassphrase: string;
  contractAddress: string;
  apiUrl?: string;
}

// --- Error codes (mirror contract errors) ---

export enum SettleErrorCode {
  // Authorization (1000-1099)
  Unauthorized = 1001,
  NotAgreementParty = 1002,
  NotArbitrator = 1003,

  // Agreement (2000-2099)
  AgreementNotFound = 2001,
  AgreementAlreadyExists = 2002,
  AgreementAlreadyFunded = 2003,
  AgreementNotFunded = 2004,
  AgreementExpired = 2005,
  AgreementCancelled = 2006,
  AgreementNotActive = 2007,
  InvalidAgreementStatus = 2008,

  // Funding (3000-3099)
  InsufficientFunds = 3001,
  InsufficientEscrowBalance = 3002,
  EscrowAlreadyFunded = 3003,
  EscrowNotFunded = 3004,
  TransferFailed = 3005,
  InvalidAmount = 3006,

  // Milestone (4000-4099)
  MilestoneNotFound = 4001,
  MilestoneAlreadyExists = 4002,
  MilestoneNotSubmitted = 4003,
  MilestoneAlreadyApproved = 4004,
  MilestoneExpired = 4005,
  InvalidMilestoneStatus = 4006,

  // Dispute (5000-5099)
  DisputeNotFound = 5001,
  DisputeAlreadyExists = 5002,
  DisputeNotOpen = 5003,
  InvalidDisputeStatus = 5004,

  // Validation (6000-6099)
  InvalidParticipant = 6001,
  InvalidDeadline = 6002,
  InvalidEvidence = 6003,
  InvalidTimestamp = 6004,
  InvalidPercentage = 6005,

  // State (7000-7099)
  InvalidStateTransition = 7001,

  // System (9000-9099)
  SystemError = 9001,
}
