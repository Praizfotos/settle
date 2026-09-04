// ===========================================
// Milestones module
// ===========================================

import { Address, nativeToScVal } from "@stellar/stellar-sdk";
import type * as xdr from "@stellar/stellar-sdk/lib/xdr";
import type { Milestone, CreateMilestoneParams } from "../types";
import type { SettleClient } from "../client";
import { decodeMilestone, decodeMilestoneVec } from "../decoder";

interface ModuleContext {
  client: SettleClient;
  config: { contractAddress: string; networkPassphrase: string };
}

export class MilestonesModule {
  private ctx: ModuleContext;

  constructor(ctx: ModuleContext) {
    this.ctx = ctx;
  }

  /**
   * Build a transaction to create a milestone.
   */
  async buildCreate(
    params: CreateMilestoneParams & { agreementId: string },
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(params.id, { type: "string" }),
      nativeToScVal(params.agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      nativeToScVal(params.name, { type: "string" }),
      nativeToScVal(params.description, { type: "string" }),
      nativeToScVal(params.amount.toString(), { type: "i128" }),
      nativeToScVal(params.dueDate.toString(), { type: "u64" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "create_milestone",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to submit a milestone with evidence.
   */
  async buildSubmit(
    milestoneId: string,
    evidence: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(milestoneId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      nativeToScVal(evidence, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "submit_milestone",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to approve a milestone.
   */
  async buildApprove(
    milestoneId: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(milestoneId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "approve_milestone",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to reject a milestone.
   */
  async buildReject(
    milestoneId: string,
    reason: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(milestoneId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      nativeToScVal(reason, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "reject_milestone",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to release milestone payment.
   */
  async buildRelease(
    milestoneId: string,
    recipient: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(milestoneId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      Address.fromString(recipient).toScVal(),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "release_milestone_payment",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Read milestone state from the contract.
   */
  async get(milestoneId: string): Promise<Milestone> {
    const args: xdr.ScVal[] = [
      nativeToScVal(milestoneId, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.readContract({
      method: "get_milestone",
      args,
    });

    return decodeMilestone(result);
  }

  /**
   * Get milestones for an agreement.
   */
  async getByAgreement(
    agreementId: string,
    limit = 50,
    offset = 0
  ): Promise<Milestone[]> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      nativeToScVal(limit, { type: "u32" }),
      nativeToScVal(offset, { type: "u32" }),
    ];

    const result = await this.ctx.client.contract.readContract({
      method: "get_milestones_by_agreement",
      args,
    });

    return decodeMilestoneVec(result);
  }
}
