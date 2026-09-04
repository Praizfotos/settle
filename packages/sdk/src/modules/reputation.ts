// ===========================================
// Reputation module
// ===========================================

import type { ReputationScore } from "../types";
import type { SettleClient } from "../client";

interface ModuleContext {
  client: SettleClient;
  config: { contractAddress: string; networkPassphrase: string };
}

export class ReputationModule {
  private ctx: ModuleContext;
  private apiUrl: string;

  constructor(ctx: ModuleContext) {
    this.ctx = ctx;
    this.apiUrl =
      process.env.NEXT_PUBLIC_API_URL ??
      process.env.STELLAR_API_URL ??
      "http://localhost:3000";
  }

  /**
   * Get reputation score for a participant.
   * Reads from the backend API (indexed data).
   */
  async getScore(address: string): Promise<ReputationScore> {
    const response = await fetch(
      `${this.apiUrl}/api/v1/reputation/${address}`
    );

    if (!response.ok) {
      throw new Error(`Failed to fetch reputation: ${response.statusText}`);
    }

    return response.json();
  }
}
