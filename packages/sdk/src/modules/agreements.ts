// ===========================================
// Agreements module
// ===========================================

import { Address, nativeToScVal } from "@stellar/stellar-sdk";
import type * as xdr from "@stellar/stellar-sdk/lib/xdr";
import type { Agreement, CreateAgreementParams, PaginatedResult } from "../types";
import type { SettleClient } from "../client";
import { decodeAgreement, decodeAgreementVec } from "../decoder";

export interface ModuleContext {
  client: SettleClient;
  config: {
    contractAddress: string;
    networkPassphrase: string;
  };
}

export class AgreementsModule {
  private ctx: ModuleContext;

  constructor(ctx: ModuleContext) {
    this.ctx = ctx;
  }

  /**
   * Build a transaction to create an agreement.
   * Returns XDR for wallet signing.
   */
  async buildCreate(
    params: CreateAgreementParams,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const milestoneNames = params.milestones.map((m) =>
      nativeToScVal(m.name, { type: "string" })
    );

    const args: xdr.ScVal[] = [
      nativeToScVal(params.id, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      Address.fromString(params.counterparty).toScVal(),
      Address.fromString(params.token).toScVal(),
      nativeToScVal(params.totalAmount.toString(), { type: "i128" }),
      nativeToScVal(params.expiresAt.toString(), { type: "u64" }),
      nativeToScVal(milestoneNames, { type: "vec" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "create_agreement",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to fund an agreement.
   */
  async buildFund(
    agreementId: string,
    amount: bigint,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      nativeToScVal(amount.toString(), { type: "i128" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "fund_agreement",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to activate an agreement.
   */
  async buildActivate(
    agreementId: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "activate_agreement",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to complete an agreement.
   */
  async buildComplete(
    agreementId: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "complete_agreement",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to cancel an agreement.
   */
  async buildCancel(
    agreementId: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "cancel_agreement",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Read agreement state from the contract.
   */
  async get(agreementId: string): Promise<Agreement> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.readContract({
      method: "get_agreement",
      args,
    });

    return decodeAgreement(result);
  }

  /**
   * Get agreements for a participant (read from contract).
   */
  async getByParticipant(
    participant: string,
    limit = 20,
    offset = 0
  ): Promise<Agreement[]> {
    const args: xdr.ScVal[] = [
      Address.fromString(participant).toScVal(),
      nativeToScVal(limit, { type: "u32" }),
      nativeToScVal(offset, { type: "u32" }),
    ];

    const result = await this.ctx.client.contract.readContract({
      method: "get_agreements_by_participant",
      args,
    });

    return decodeAgreementVec(result);
  }
}
