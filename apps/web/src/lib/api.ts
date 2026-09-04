"use client";

const API_URL = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3000";

async function apiFetch<T>(
  path: string,
  options?: RequestInit
): Promise<T> {
  const res = await fetch(`${API_URL}${path}`, {
    headers: {
      "Content-Type": "application/json",
      ...options?.headers,
    },
    ...options,
  });

  if (!res.ok) {
    const body = await res.json().catch(() => ({}));
    throw new Error(body.error ?? `API error ${res.status}`);
  }

  return res.json();
}

// ── Agreements ──────────────────────────────────────────────────────

export interface ApiAgreement {
  id: string;
  on_chain_id: string;
  client: string;
  provider: string;
  token: string;
  total_amount: number;
  funded_amount: number;
  released_amount: number;
  refunded_amount: number;
  title: string | null;
  status: string;
  created_at: string;
  updated_at: string;
  tx_hash: string | null;
}

export async function listAgreements(
  address?: string,
  limit = 20,
  offset = 0
): Promise<{ agreements: ApiAgreement[] }> {
  const params = new URLSearchParams({
    limit: String(limit),
    offset: String(offset),
  });
  if (address) params.set("address", address);
  return apiFetch(`/api/v1/agreements?${params}`);
}

export async function getAgreement(
  id: string
): Promise<ApiAgreement> {
  return apiFetch(`/api/v1/agreements/${id}`);
}

// ── Milestones ──────────────────────────────────────────────────────

export interface ApiMilestone {
  id: string;
  on_chain_id: string;
  agreement_id: string;
  name: string;
  description: string;
  amount: number;
  status: string;
  due_date: string;
  submitted_at: string | null;
  approved_at: string | null;
  evidence: string | null;
}

export async function listMilestones(
  agreementId: string
): Promise<{ milestones: ApiMilestone[] }> {
  return apiFetch(`/api/v1/milestones/${agreementId}`);
}

// ── Disputes ────────────────────────────────────────────────────────

export interface ApiDispute {
  id: string;
  on_chain_id: number;
  agreement_id: string;
  opener: string;
  reason: string;
  status: string;
  resolution: string;
  opened_at: string;
  resolved_at: string | null;
}

export async function listDisputes(): Promise<{ disputes: ApiDispute[] }> {
  return apiFetch(`/api/v1/disputes`);
}

export async function getDispute(
  id: string
): Promise<ApiDispute> {
  return apiFetch(`/api/v1/disputes/${id}`);
}

// ── Settlements ─────────────────────────────────────────────────────

export interface ApiSettlement {
  id: string;
  event_type: string;
  agreement_id: string;
  participant: string;
  amount: number;
  ledger: number;
  tx_hash: string;
  timestamp: string;
}

export async function listSettlements(): Promise<{
  settlements: ApiSettlement[];
}> {
  return apiFetch(`/api/v1/settlements`);
}

// ── Reputation ──────────────────────────────────────────────────────

export interface ApiReputation {
  address: string;
  score: number;
  label: string;
  completed_agreements: number;
  settled_volume: number;
  on_time_completion_rate: number;
  dispute_rate: number;
  dispute_win_rate: number;
}

export async function getReputation(
  address: string
): Promise<ApiReputation> {
  return apiFetch(`/api/v1/reputation/${address}`);
}

// ── Health ──────────────────────────────────────────────────────────

export async function getHealth(): Promise<{
  status: string;
  database: boolean;
  service: string;
  version: string;
}> {
  return apiFetch(`/api/v1/health`);
}
