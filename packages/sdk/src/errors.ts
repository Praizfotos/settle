// ===========================================
// @settle/sdk — Error types
// ===========================================

import { SettleErrorCode } from "@settle/types";

export { SettleErrorCode };

export class SettleError extends Error {
  public code: SettleErrorCode;
  public details?: Record<string, unknown>;

  constructor(
    message: string,
    code: SettleErrorCode,
    details?: Record<string, unknown>
  ) {
    super(message);
    this.name = "SettleError";
    this.code = code;
    this.details = details;
  }

  static fromContractError(
    contractCode: number,
    details?: Record<string, unknown>
  ): SettleError {
    const code = contractCode as SettleErrorCode;
    const message = CONTRACT_ERROR_MESSAGES[code] ?? `Contract error ${contractCode}`;
    return new SettleError(message, code, details);
  }

  static walletDisconnected(): SettleError {
    return new SettleError(
      "Wallet not connected. Please connect your Stellar wallet.",
      SettleErrorCode.Unauthorized
    );
  }

  static networkError(cause?: unknown): SettleError {
    return new SettleError(
      "Network error. Please check your connection and try again.",
      SettleErrorCode.SystemError,
      { cause }
    );
  }

  static transactionRejected(): SettleError {
    return new SettleError(
      "Transaction was rejected by your wallet.",
      SettleErrorCode.Unauthorized
    );
  }

  static insufficientFunds(): SettleError {
    return new SettleError(
      "Insufficient funds to complete this transaction.",
      SettleErrorCode.InsufficientFunds
    );
  }
}

const CONTRACT_ERROR_MESSAGES: Record<number, string> = {
  [SettleErrorCode.Unauthorized]: "You are not authorized to perform this action.",
  [SettleErrorCode.NotAgreementParty]: "You are not a party to this agreement.",
  [SettleErrorCode.NotArbitrator]: "You are not the arbitrator for this dispute.",
  [SettleErrorCode.AgreementNotFound]: "Agreement not found.",
  [SettleErrorCode.AgreementAlreadyExists]: "An agreement with this ID already exists.",
  [SettleErrorCode.AgreementAlreadyFunded]: "This agreement has already been funded.",
  [SettleErrorCode.AgreementNotFunded]: "This agreement has not been funded yet.",
  [SettleErrorCode.AgreementExpired]: "This agreement has expired.",
  [SettleErrorCode.AgreementCancelled]: "This agreement has been cancelled.",
  [SettleErrorCode.AgreementNotActive]: "This agreement is not active.",
  [SettleErrorCode.InvalidAgreementStatus]: "Invalid agreement status for this operation.",
  [SettleErrorCode.InsufficientFunds]: "Insufficient funds.",
  [SettleErrorCode.InsufficientEscrowBalance]: "Insufficient escrow balance.",
  [SettleErrorCode.EscrowAlreadyFunded]: "Escrow is already funded.",
  [SettleErrorCode.EscrowNotFunded]: "Escrow has not been funded.",
  [SettleErrorCode.TransferFailed]: "Token transfer failed.",
  [SettleErrorCode.InvalidAmount]: "Invalid amount.",
  [SettleErrorCode.MilestoneNotFound]: "Milestone not found.",
  [SettleErrorCode.MilestoneAlreadyExists]: "A milestone with this ID already exists.",
  [SettleErrorCode.MilestoneNotSubmitted]: "This milestone has not been submitted.",
  [SettleErrorCode.MilestoneAlreadyApproved]: "This milestone has already been approved.",
  [SettleErrorCode.MilestoneExpired]: "This milestone has expired.",
  [SettleErrorCode.InvalidMilestoneStatus]: "Invalid milestone status for this operation.",
  [SettleErrorCode.DisputeNotFound]: "Dispute not found.",
  [SettleErrorCode.DisputeAlreadyExists]: "A dispute already exists for this agreement.",
  [SettleErrorCode.DisputeNotOpen]: "This dispute is not open.",
  [SettleErrorCode.InvalidDisputeStatus]: "Invalid dispute status for this operation.",
  [SettleErrorCode.InvalidParticipant]: "Invalid participant address.",
  [SettleErrorCode.InvalidDeadline]: "Invalid deadline.",
  [SettleErrorCode.InvalidEvidence]: "Invalid evidence.",
  [SettleErrorCode.InvalidTimestamp]: "Invalid timestamp.",
  [SettleErrorCode.InvalidPercentage]: "Invalid percentage.",
  [SettleErrorCode.InvalidStateTransition]: "Invalid state transition.",
  [SettleErrorCode.SystemError]: "A system error occurred. Please try again.",
};
