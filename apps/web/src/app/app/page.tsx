"use client";

import { useEffect, useState } from "react";
import { useWallet } from "@/lib/wallet";
import { listAgreements, listSettlements, getReputation } from "@/lib/api";
import type { ApiAgreement, ApiSettlement, ApiReputation } from "@/lib/api";

function StatCard({
  label,
  value,
  sub,
}: {
  label: string;
  value: string;
  sub?: string;
}) {
  return (
    <div className="bg-white rounded-2xl border border-gray-200 p-6">
      <p className="text-[13px] font-medium text-gray-500 mb-1">{label}</p>
      <p className="text-[28px] font-bold text-gray-900 tracking-tight">
        {value}
      </p>
      {sub && <p className="text-[12px] text-gray-400 mt-1">{sub}</p>}
    </div>
  );
}

function StatusBadge({ status }: { status: string }) {
  const colors: Record<string, string> = {
    DRAFT: "bg-gray-100 text-gray-600",
    FUNDED: "bg-blue-100 text-blue-700",
    ACTIVE: "bg-green-100 text-green-700",
    COMPLETED: "bg-green-100 text-green-700",
    DISPUTED: "bg-red-100 text-red-700",
    EXPIRED: "bg-yellow-100 text-yellow-700",
    CANCELLED: "bg-gray-100 text-gray-500",
  };
  return (
    <span
      className={`px-2 py-0.5 text-[11px] font-medium rounded-full ${colors[status] ?? "bg-gray-100 text-gray-600"}`}
    >
      {status}
    </span>
  );
}

export default function DashboardPage() {
  const { connected, address, connecting, connect } = useWallet();
  const [agreements, setAgreements] = useState<ApiAgreement[]>([]);
  const [settlements, setSettlements] = useState<ApiSettlement[]>([]);
  const [reputation, setReputation] = useState<ApiReputation | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!connected || !address) {
      setLoading(false);
      return;
    }

    async function load() {
      try {
        const [agrRes, setRes, repRes] = await Promise.all([
          listAgreements(address!, 10, 0),
          listSettlements(),
          getReputation(address!),
        ]);
        setAgreements(agrRes.agreements);
        setSettlements(setRes.settlements);
        setReputation(repRes);
      } catch (err: any) {
        setError(err?.message ?? "Failed to load data");
      } finally {
        setLoading(false);
      }
    }
    load();
  }, [connected, address]);

  if (!connected) {
    return (
      <div className="max-w-[1280px] mx-auto px-6 md:px-10 py-16">
        <div className="text-center py-24">
          <h1 className="text-[32px] font-bold text-gray-900 mb-4">
            Welcome to Settle
          </h1>
          <p className="text-[16px] text-gray-500 mb-8 max-w-[480px] mx-auto">
            Connect your Stellar wallet to view agreements, manage escrow, and
            track settlements.
          </p>
          <div className="flex items-center justify-center gap-3">
            <button
              onClick={connect}
              disabled={connecting}
              className="px-5 py-2.5 text-[14px] font-semibold text-white rounded-xl transition-all duration-150 hover:scale-[1.02] active:scale-[0.98] disabled:opacity-50"
              style={{
                background: "linear-gradient(135deg, #1254D8 0%, #2F70E8 100%)",
                boxShadow: "0 1px 3px rgba(18,84,216,0.35)",
              }}
            >
              {connecting ? "Connecting..." : "Connect Wallet"}
            </button>
            <a
              href="/"
              className="px-5 py-2.5 text-[14px] font-medium text-gray-600 border border-gray-200 rounded-xl hover:bg-gray-50 transition-colors"
            >
              Back to Home
            </a>
          </div>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="max-w-[1280px] mx-auto px-6 md:px-10 py-16">
        <div className="animate-pulse space-y-6">
          <div className="h-8 w-48 bg-gray-200 rounded" />
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            {[1, 2, 3, 4].map((i) => (
              <div key={i} className="h-28 bg-gray-200 rounded-2xl" />
            ))}
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-[1280px] mx-auto px-6 md:px-10 py-10">
      <div className="mb-8">
        <h1 className="text-[28px] font-bold text-gray-900 tracking-tight">
          Dashboard
        </h1>
        <p className="text-[14px] text-gray-500 mt-1">
          Your settlement overview
        </p>
      </div>

      {error && (
        <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-xl text-[13px] text-red-700">
          {error}
        </div>
      )}

      {/* Stats */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <StatCard
          label="Total Agreements"
          value={String(agreements.length)}
        />
        <StatCard
          label="Active"
          value={String(agreements.filter((a) => a.status === "ACTIVE").length)}
        />
        <StatCard
          label="Completed"
          value={String(
            agreements.filter((a) => a.status === "COMPLETED").length
          )}
        />
        <StatCard
          label="Reputation"
          value={reputation ? `${reputation.score.toFixed(0)}` : "—"}
          sub={reputation?.label}
        />
      </div>

      {/* Recent Agreements */}
      <div className="bg-white rounded-2xl border border-gray-200 overflow-hidden mb-8">
        <div className="px-6 py-4 border-b border-gray-100 flex items-center justify-between">
          <h2 className="text-[15px] font-semibold text-gray-900">
            Recent Agreements
          </h2>
          <a
            href="/app/agreements"
            className="text-[13px] font-medium text-[#1254D8] hover:text-[#0D4FD7]"
          >
            View all
          </a>
        </div>
        {agreements.length === 0 ? (
          <div className="px-6 py-12 text-center">
            <p className="text-[14px] text-gray-400">
              No agreements yet. Create your first agreement to get started.
            </p>
          </div>
        ) : (
          <div className="divide-y divide-gray-100">
            {agreements.slice(0, 5).map((a) => (
              <div
                key={a.id}
                className="px-6 py-4 flex items-center justify-between hover:bg-gray-50 transition-colors"
              >
                <div className="flex-1 min-w-0">
                  <p className="text-[14px] font-medium text-gray-900 truncate">
                    {a.title ?? `Agreement ${a.on_chain_id}`}
                  </p>
                  <p className="text-[12px] text-gray-400 mt-0.5">
                    {a.client.slice(0, 8)}... → {a.provider.slice(0, 8)}...
                  </p>
                </div>
                <div className="flex items-center gap-3 ml-4">
                  <StatusBadge status={a.status} />
                  <span className="text-[13px] font-mono text-gray-500">
                    {(a.total_amount / 1_000_000).toFixed(2)} XLM
                  </span>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Recent Activity */}
      <div className="bg-white rounded-2xl border border-gray-200 overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-100">
          <h2 className="text-[15px] font-semibold text-gray-900">
            Recent Activity
          </h2>
        </div>
        {settlements.length === 0 ? (
          <div className="px-6 py-12 text-center">
            <p className="text-[14px] text-gray-400">
              No settlement activity yet.
            </p>
          </div>
        ) : (
          <div className="divide-y divide-gray-100">
            {settlements.slice(0, 5).map((s) => (
              <div
                key={s.id}
                className="px-6 py-4 flex items-center gap-4"
              >
                <div className="w-2 h-2 rounded-full bg-[#1254D8] flex-shrink-0" />
                <div className="flex-1 min-w-0">
                  <p className="text-[13px] text-gray-700">
                    <span className="font-medium">{s.event_type}</span>
                    {" · "}
                    <span className="font-mono text-gray-500">
                      {s.participant.slice(0, 8)}...
                    </span>
                  </p>
                </div>
                <span className="text-[12px] text-gray-400 flex-shrink-0">
                  {new Date(s.timestamp).toLocaleDateString()}
                </span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
