// ===========================================
// ScVal decoder for Settle contract types
// ===========================================

import * as xdr from "@stellar/stellar-sdk/lib/xdr";
import type {
  Agreement,
  AgreementStatus,
  Milestone,
  MilestoneStatus,
  Dispute,
  DisputeStatus,
  Escrow,
  EscrowStatus,
  ReputationScore,
} from "./types";

// ── Helpers ──────────────────────────────────────────────────────────

function field(struct: xdr.ScVal, index: number): xdr.ScVal | undefined {
  if (struct.switch() !== xdr.ScValType.struct()) return undefined;
  const fields = struct.struct();
  return fields?.[index];
}

function optString(val: xdr.ScVal | undefined): string | undefined {
  if (!val || val.switch() !== xdr.ScValType.opt()) return undefined;
  const inner = val.opt()?.[0];
  if (!inner) return undefined;
  return scValToString(inner);
}

function scValToString(val: xdr.ScVal): string {
  if (val.switch() === xdr.ScValType.string()) {
    return val.string().toString();
  }
  if (val.switch() === xdr.ScValType.symbol()) {
    return val.symbol().toString();
  }
  return String(val.toXDR("base64"));
}

function scValToBytes(val: xdr.ScVal): string {
  if (val.switch() === xdr.ScValType.bytes()) {
    return val.bytes().toString("hex");
  }
  return String(val.toXDR("base64"));
}

function scValToI128(val: xdr.ScVal): bigint {
  if (val.switch() === xdr.ScValType.i128()) {
    const parts = val.i128();
    const hi = BigInt(parts.hi()) << 64n;
    const lo = BigInt(parts.lo());
    return hi | lo;
  }
  return 0n;
}

function scValToU64(val: xdr.ScVal): bigint {
  if (val.switch() === xdr.ScValType.u64()) {
    return BigInt(val.u64());
  }
  if (val.switch() === xdr.ScValType.timepoint()) {
    return BigInt(val.timepoint());
  }
  return 0n;
}

function scValToU32(val: xdr.ScVal): number {
  if (val.switch() === xdr.ScValType.u32()) {
    return val.u32();
  }
  return 0;
}

function scValToAddress(val: xdr.ScVal): string {
  if (val.switch() === xdr.ScValType.address()) {
    return val.address().toString();
  }
  return "";
}

function scValToVec(val: xdr.ScVal): xdr.ScVal[] {
  if (val.switch() === xdr.ScValType.vec()) {
    return val.vec() ?? [];
  }
  return [];
}

function scValToBool(val: xdr.ScVal): boolean {
  if (val.switch() === xdr.ScValType.bool()) {
    return val.bool();
  }
  return false;
}

// ── Agreement ────────────────────────────────────────────────────────

function decodeAgreementStatus(val: xdr.ScVal): AgreementStatus {
  const symbol = scValToString(val).toLowerCase();
  return symbol as AgreementStatus;
}

export function decodeAgreement(val: xdr.ScVal): Agreement {
  const f = (i: number) => field(val, i);

  return {
    id: scValToBytes(f(0)!),
    creator: scValToAddress(f(1)!),
    counterparty: scValToAddress(f(2)!),
    token: scValToAddress(f(3)!),
    totalAmount: scValToI128(f(4)!),
    fundedAmount: scValToI128(f(5)!),
    releasedAmount: scValToI128(f(6)!),
    refundedAmount: scValToI128(f(7)!),
    status: decodeAgreementStatus(f(8)!),
    createdAt: Number(scValToU64(f(9)!)),
    expiresAt: Number(scValToU64(f(10)!)),
    milestones: scValToVec(f(11)!).map((m) => scValToBytes(m)),
  };
}

export function decodeAgreementVec(val: xdr.ScVal): Agreement[] {
  return scValToVec(val).map(decodeAgreement);
}

// ── Milestone ────────────────────────────────────────────────────────

function decodeMilestoneStatus(val: xdr.ScVal): MilestoneStatus {
  const symbol = scValToString(val).toLowerCase();
  return symbol as MilestoneStatus;
}

export function decodeMilestone(val: xdr.ScVal): Milestone {
  const f = (i: number) => field(val, i);

  return {
    id: scValToBytes(f(0)!),
    agreementId: scValToBytes(f(1)!),
    name: scValToString(f(2)!),
    description: scValToString(f(3)!),
    amount: scValToI128(f(4)!),
    status: decodeMilestoneStatus(f(5)!),
    dueDate: Number(scValToU64(f(6)!)),
    submittedAt: optString(f(7)) ? Number(BigInt(optString(f(7))!)) : undefined,
    approvedAt: optString(f(8)) ? Number(BigInt(optString(f(8))!)) : undefined,
    evidence: optString(f(9)),
  };
}

export function decodeMilestoneVec(val: xdr.ScVal): Milestone[] {
  return scValToVec(val).map(decodeMilestone);
}

// ── Escrow ───────────────────────────────────────────────────────────

function decodeEscrowStatus(val: xdr.ScVal): EscrowStatus {
  const symbol = scValToString(val).toLowerCase();
  return symbol as EscrowStatus;
}

export function decodeEscrow(val: xdr.ScVal): Escrow {
  const f = (i: number) => field(val, i);

  return {
    agreementId: scValToBytes(f(0)!),
    token: scValToAddress(f(1)!),
    amount: scValToI128(f(2)!),
    lockedAmount: scValToI128(f(3)!),
    releasedAmount: scValToI128(f(4)!),
    status: decodeEscrowStatus(f(5)!),
    createdAt: Number(scValToU64(f(6)!)),
    lastActionAt: Number(scValToU64(f(7)!)),
  };
}

// ── Dispute ──────────────────────────────────────────────────────────

function decodeDisputeStatus(val: xdr.ScVal): DisputeStatus {
  const symbol = scValToString(val).toLowerCase();
  return symbol as DisputeStatus;
}

export function decodeDispute(val: xdr.ScVal): Dispute {
  const f = (i: number) => field(val, i);

  return {
    id: scValToBytes(f(0)!),
    agreementId: scValToBytes(f(1)!),
    openedBy: scValToAddress(f(2)!),
    reason: scValToString(f(3)!),
    evidence: scValToVec(f(4)).map((e) => scValToString(e)),
    status: decodeDisputeStatus(f(5)!),
    resolution: optString(f(6)),
    openedAt: Number(scValToU64(f(7)!)),
    resolvedAt: optString(f(8)) ? Number(BigInt(optString(f(8))!)) : undefined,
    arbitrator: optString(f(9)),
  };
}

// ── Reputation ───────────────────────────────────────────────────────

export function decodeReputationScore(
  participant: string,
  val: xdr.ScVal
): ReputationScore {
  const f = (i: number) => field(val, i);

  return {
    participant,
    score: scValToU32(f(0)!),
    label: scValToString(f(1)!) as ReputationScore["label"],
    totalAgreements: scValToU32(f(2)!),
    successfulAgreements: scValToU32(f(3)!),
    disputedAgreements: scValToU32(f(4)!),
    totalVolume: scValToI128(f(5)!),
    lastUpdated: Number(scValToU64(f(6)!)),
  };
}
