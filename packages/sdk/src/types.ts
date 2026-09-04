// ===========================================
// @settle/sdk — Type definitions
// ===========================================

import type {
  Agreement,
  AgreementStatus,
  CreateAgreementParams,
  Milestone,
  MilestoneStatus,
  CreateMilestoneParams,
  Escrow,
  EscrowStatus,
  Dispute,
  DisputeStatus,
  ReputationScore,
  PaginatedResult,
  TxResult,
  SettleConfig,
} from "@settle/types";

export type {
  Agreement,
  AgreementStatus,
  CreateAgreementParams,
  Milestone,
  MilestoneStatus,
  CreateMilestoneParams,
  Escrow,
  EscrowStatus,
  Dispute,
  DisputeStatus,
  ReputationScore,
  PaginatedResult,
  TxResult,
  SettleConfig,
};

export interface SettleClientConfig extends SettleConfig {
  apiUrl?: string;
}
