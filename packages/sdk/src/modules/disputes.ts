// ===========================================
// Disputes module
// ===========================================

import { Address, nativeToScVal } from "@stellar/stellar-sdk";
import type * as xdr from "@stellar/stellar-sdk/lib/xdr";
import type { Dispute } from "../types";
import type { SettleClient } from "../client";
import { decodeDispute } from "../decoder";

interface ModuleContext {
  client: SettleClient;
  config: { contractAddress: string; networkPassphrase: string };
}

export class DisputesModule {
  private ctx: ModuleContext;

  constructor(ctx: ModuleContext) {
    this.ctx = ctx;
  }

  /**
   * Build a transaction to open a dispute.
   */
  async buildOpen(
    params: {
      id: string;
      agreementId: string;
      reason: string;
      initialEvidence: string;
    },
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(params.id, { type: "string" }),
      nativeToScVal(params.agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      nativeToScVal(params.reason, { type: "string" }),
      nativeToScVal(params.initialEvidence, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "open_dispute",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to submit evidence.
   */
  async buildSubmitEvidence(
    disputeId: string,
    evidence: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(disputeId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      nativeToScVal(evidence, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "submit_evidence",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to resolve a dispute (arbitrator only).
   */
  async buildResolve(
    params: {
      id: string;
      resolution: string;
      winner: string;
      compensationAmount: bigint;
    },
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(params.id, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      nativeToScVal(params.resolution, { type: "string" }),
      Address.fromString(params.winner).toScVal(),
      nativeToScVal(params.compensationAmount.toString(), { type: "i128" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "resolve_dispute",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to close a dispute.
   */
  async buildClose(
    disputeId: string,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(disputeId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "close_dispute",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Read dispute state from the contract.
   */
  async get(disputeId: string): Promise<Dispute> {
    const args: xdr.ScVal[] = [
      nativeToScVal(disputeId, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.readContract({
      method: "get_dispute",
      args,
    });

    return decodeDispute(result);
  }

  /**
   * Check if an agreement has an open dispute.
   */
  async hasOpenDispute(agreementId: string): Promise<boolean> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.readContract({
      method: "has_open_dispute",
      args,
    });

    if (result.switch() === xdr.ScValType.bool()) {
      return result.bool();
    }
    return false;
  }
}
