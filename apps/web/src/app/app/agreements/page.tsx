"use client";

import { useEffect, useState } from "react";
import { useWallet } from "@/lib/wallet";
import { listAgreements } from "@/lib/api";
import type { ApiAgreement } from "@/lib/api";
import Link from "next/link";

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

export default function AgreementsPage() {
  const { connected, address } = useWallet();
  const [agreements, setAgreements] = useState<ApiAgreement[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!connected || !address) {
      setLoading(false);
      return;
    }
    listAgreements(address, 50, 0)
      .then((res) => setAgreements(res.agreements))
      .catch(() => {})
      .finally(() => setLoading(false));
  }, [connected, address]);

  return (
    <div className="max-w-[1280px] mx-auto px-6 md:px-10 py-10">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-[28px] font-bold text-gray-900 tracking-tight">
            Agreements
          </h1>
          <p className="text-[14px] text-gray-500 mt-1">
            Manage your settlement agreements
          </p>
        </div>
        <Link
          href="/app/agreements/new"
          className="px-5 py-2.5 text-[14px] font-semibold text-white rounded-[10px] transition-all hover:scale-[1.02] active:scale-[0.98]"
          style={{
            background: "linear-gradient(135deg, #1254D8 0%, #2F70E8 100%)",
          }}
        >
          Create Agreement
        </Link>
      </div>

      {!connected ? (
        <div className="text-center py-24">
          <p className="text-[16px] text-gray-500">
            Connect your wallet to view agreements.
          </p>
        </div>
      ) : loading ? (
        <div className="space-y-4">
          {[1, 2, 3].map((i) => (
            <div key={i} className="h-20 bg-gray-200 rounded-2xl animate-pulse" />
          ))}
        </div>
      ) : agreements.length === 0 ? (
        <div className="text-center py-24 bg-white rounded-2xl border border-gray-200">
          <p className="text-[16px] text-gray-400 mb-4">
            No agreements yet
          </p>
          <Link
            href="/app/agreements/new"
            className="text-[14px] font-medium text-[#1254D8] hover:text-[#0D4FD7]"
          >
            Create your first agreement →
          </Link>
        </div>
      ) : (
        <div className="space-y-3">
          {agreements.map((a) => (
            <Link
              key={a.id}
              href={`/app/agreements/${a.on_chain_id}`}
              className="block bg-white rounded-2xl border border-gray-200 p-5 hover:shadow-md transition-shadow"
            >
              <div className="flex items-center justify-between">
                <div className="flex-1 min-w-0">
                  <h3 className="text-[15px] font-semibold text-gray-900 truncate">
                    {a.title ?? `Agreement ${a.on_chain_id}`}
                  </h3>
                  <p className="text-[12px] text-gray-400 mt-1">
                    {a.client.slice(0, 10)}... → {a.provider.slice(0, 10)}...
                  </p>
                </div>
                <div className="flex items-center gap-4 ml-4">
                  <StatusBadge status={a.status} />
                  <div className="text-right">
                    <p className="text-[14px] font-semibold text-gray-900">
                      {(a.total_amount / 1_000_000).toFixed(2)}
                    </p>
                    <p className="text-[11px] text-gray-400">
                      {a.funded_amount > 0
                        ? `${((a.funded_amount / a.total_amount) * 100).toFixed(0)}% funded`
                        : "Not funded"}
                    </p>
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}
