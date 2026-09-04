// ===========================================
// Escrow module
// ===========================================

import { Address, nativeToScVal } from "@stellar/stellar-sdk";
import type * as xdr from "@stellar/stellar-sdk/lib/xdr";
import type { Escrow } from "../types";
import type { SettleClient } from "../client";
import { decodeEscrow } from "../decoder";

interface ModuleContext {
  client: SettleClient;
  config: { contractAddress: string; networkPassphrase: string };
}

export class EscrowModule {
  private ctx: ModuleContext;

  constructor(ctx: ModuleContext) {
    this.ctx = ctx;
  }

  /**
   * Build a transaction to create escrow for an agreement.
   */
  async buildCreate(
    agreementId: string,
    token: string,
    amount: bigint,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      Address.fromString(token).toScVal(),
      nativeToScVal(amount.toString(), { type: "i128" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "create_escrow",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to fund escrow.
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
      method: "fund_escrow",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to lock escrow funds.
   */
  async buildLock(
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
      method: "lock_escrow",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to release escrow funds.
   */
  async buildRelease(
    agreementId: string,
    recipient: string,
    amount: bigint,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      Address.fromString(recipient).toScVal(),
      nativeToScVal(amount.toString(), { type: "i128" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "release_escrow",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Build a transaction to refund escrow funds.
   */
  async buildRefund(
    agreementId: string,
    recipient: string,
    amount: bigint,
    sourceAccount: string
  ): Promise<{ txXdr: string }> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
      Address.fromString(sourceAccount).toScVal(),
      Address.fromString(recipient).toScVal(),
      nativeToScVal(amount.toString(), { type: "i128" }),
    ];

    const result = await this.ctx.client.contract.buildContractCall({
      method: "refund_escrow",
      args,
      sourceAccount,
    });

    return { txXdr: result.txXdr };
  }

  /**
   * Read escrow state from the contract.
   */
  async get(agreementId: string): Promise<Escrow> {
    const args: xdr.ScVal[] = [
      nativeToScVal(agreementId, { type: "string" }),
    ];

    const result = await this.ctx.client.contract.readContract({
      method: "get_escrow",
      args,
    });

    return decodeEscrow(result);
  }
}
