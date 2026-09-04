/**
 * @settle/sdk
 *
 * TypeScript SDK for the Settle programmable agreement and settlement protocol.
 * Built on Stellar / Soroban.
 */

export { SettleClient } from "./client";
export { AgreementsModule } from "./modules/agreements";
export { MilestonesModule } from "./modules/milestones";
export { DisputesModule } from "./modules/disputes";
export { EscrowModule } from "./modules/escrow";
export { ReputationModule } from "./modules/reputation";

export type {
  SettleConfig,
  Agreement,
  CreateAgreementParams,
  Milestone,
  Dispute,
  Escrow,
  ReputationScore,
  PaginatedResult,
  TxResult,
} from "./types";

export { SettleError, SettleErrorCode } from "./errors";
