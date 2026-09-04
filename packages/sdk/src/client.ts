// ===========================================
// @settle/sdk — SettleClient
// Main entry point for the Settle SDK
// ===========================================

import { SorobanContractClient } from "@settle/stellar";
import { AgreementsModule } from "./modules/agreements";
import { MilestonesModule } from "./modules/milestones";
import { DisputesModule } from "./modules/disputes";
import { EscrowModule } from "./modules/escrow";
import { ReputationModule } from "./modules/reputation";
import type { SettleClientConfig } from "./types";

export class SettleClient {
  public readonly agreements: AgreementsModule;
  public readonly milestones: MilestonesModule;
  public readonly disputes: DisputesModule;
  public readonly escrow: EscrowModule;
  public readonly reputation: ReputationModule;
  public readonly contract: SorobanContractClient;
  public readonly config: SettleClientConfig;

  constructor(config: SettleClientConfig) {
    this.config = config;

    this.contract = new SorobanContractClient({
      rpcUrl: config.rpcUrl,
      contractAddress: config.contractAddress,
      networkPassphrase: config.networkPassphrase,
    });

    const ctx = { client: this, config };

    this.agreements = new AgreementsModule(ctx);
    this.milestones = new MilestonesModule(ctx);
    this.disputes = new DisputesModule(ctx);
    this.escrow = new EscrowModule(ctx);
    this.reputation = new ReputationModule(ctx);
  }

  /**
   * Create a SettleClient from environment variables.
   */
  static fromEnv(network?: "testnet" | "mainnet"): SettleClient {
    const net = network ?? (process.env.NEXT_PUBLIC_STELLAR_NETWORK as "testnet" | "mainnet") ?? "testnet";

    return new SettleClient({
      network: net,
      rpcUrl:
        process.env.NEXT_PUBLIC_STELLAR_RPC_URL ??
        process.env.STELLAR_RPC_URL ??
        "https://soroban-testnet.stellar.org",
      networkPassphrase:
        process.env.NEXT_PUBLIC_STELLAR_NETWORK_PASSPHRASE ??
        process.env.STELLAR_NETWORK_PASSPHRASE ??
        "Test SDF Network ; September 2015",
      contractAddress:
        process.env.NEXT_PUBLIC_STELLAR_CONTRACT_ADDRESS ??
        process.env.STELLAR_CONTRACT_ADDRESS ??
        "",
      apiUrl:
        process.env.NEXT_PUBLIC_API_URL ??
        "http://localhost:3000",
    });
  }
}
