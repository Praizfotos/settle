"use client";

import { useEffect, useState } from "react";
import { useWallet } from "@/lib/wallet";
import { listDisputes } from "@/lib/api";
import type { ApiDispute } from "@/lib/api";

function StatusBadge({ status }: { status: string }) {
  const colors: Record<string, string> = {
    OPEN: "bg-red-100 text-red-700",
    EVIDENCE_SUBMISSION: "bg-yellow-100 text-yellow-700",
    UNDER_REVIEW: "bg-blue-100 text-blue-700",
    RESOLVED: "bg-green-100 text-green-700",
    CLOSED: "bg-gray-100 text-gray-500",
  };
  return (
    <span
      className={`px-2 py-0.5 text-[11px] font-medium rounded-full ${colors[status] ?? "bg-gray-100 text-gray-600"}`}
    >
      {status}
    </span>
  );
}

export default function DisputesPage() {
  const { connected } = useWallet();
  const [disputes, setDisputes] = useState<ApiDispute[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    listDisputes()
      .then((res) => setDisputes(res.disputes))
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="max-w-[1280px] mx-auto px-6 md:px-10 py-10">
      <div className="mb-8">
        <h1 className="text-[28px] font-bold text-gray-900 tracking-tight">
          Disputes
        </h1>
        <p className="text-[14px] text-gray-500 mt-1">
          Track and resolve agreement disputes
        </p>
      </div>

      {!connected ? (
        <div className="text-center py-24">
          <p className="text-[16px] text-gray-500">
            Connect your wallet to view disputes.
          </p>
        </div>
      ) : loading ? (
        <div className="space-y-4">
          {[1, 2].map((i) => (
            <div key={i} className="h-24 bg-gray-200 rounded-2xl animate-pulse" />
          ))}
        </div>
      ) : disputes.length === 0 ? (
        <div className="text-center py-24 bg-white rounded-2xl border border-gray-200">
          <p className="text-[16px] text-gray-400">
            No disputes. All agreements are running smoothly.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {disputes.map((d) => (
            <div
              key={d.id}
              className="bg-white rounded-2xl border border-gray-200 p-5"
            >
              <div className="flex items-start justify-between">
                <div className="flex-1 min-w-0">
                  <p className="text-[14px] font-medium text-gray-900">
                    {d.reason}
                  </p>
                  <p className="text-[12px] text-gray-400 mt-1">
                    Opened by {d.opener.slice(0, 10)}... ·{" "}
                    {new Date(d.opened_at).toLocaleDateString()}
                  </p>
                </div>
                <StatusBadge status={d.status} />
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
