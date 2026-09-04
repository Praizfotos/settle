"use client";

import { useState } from "react";
import { useWallet } from "@/lib/wallet";
import { useRouter } from "next/navigation";

export default function NewAgreementPage() {
  const { connected, address } = useWallet();
  const router = useRouter();

  const [counterparty, setCounterparty] = useState("");
  const [token, setToken] = useState(
    "CBIELTK6YQZD7SPHE7G4DHLMGIQHW7T3M2YBWUHQKNPK5KXHSP5BF7YB"
  );
  const [totalAmount, setTotalAmount] = useState("");
  const [title, setTitle] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [result, setResult] = useState<string | null>(null);

  if (!connected) {
    return (
      <div className="max-w-[640px] mx-auto px-6 py-16 text-center">
        <p className="text-[16px] text-gray-500">
          Connect your wallet to create an agreement.
        </p>
      </div>
    );
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setSubmitting(true);
    setResult(null);

    try {
      const apiUrl = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:3000";
      const res = await fetch(`${apiUrl}/api/v1/agreements`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          counterparty,
          token,
          total_amount: Math.round(parseFloat(totalAmount) * 1_000_000),
          title: title || undefined,
        }),
      });

      const data = await res.json();
      if (res.ok) {
        setResult(
          "Agreement draft created in the database. On-chain submission via Soroban contract is coming soon."
        );
        setTimeout(() => router.push("/app/agreements"), 2000);
      } else {
        setResult(data.error ?? "Failed to create agreement");
      }
    } catch (err: any) {
      setResult(err?.message ?? "Network error");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <div className="max-w-[640px] mx-auto px-6 py-10">
      <h1 className="text-[28px] font-bold text-gray-900 tracking-tight mb-2">
        Create Agreement
      </h1>
      <p className="text-[14px] text-gray-500 mb-8">
        Set up a new programmable settlement agreement on Stellar.
      </p>

      <form onSubmit={handleSubmit} className="space-y-5">
        <div>
          <label className="block text-[13px] font-medium text-gray-700 mb-1.5">
            Title
          </label>
          <input
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
            placeholder="Website redesign project"
            className="w-full px-4 py-2.5 text-[14px] border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#1254D8] focus:border-transparent"
          />
        </div>

        <div>
          <label className="block text-[13px] font-medium text-gray-700 mb-1.5">
            Counterparty Address *
          </label>
          <input
            type="text"
            value={counterparty}
            onChange={(e) => setCounterparty(e.target.value)}
            placeholder="G..."
            required
            className="w-full px-4 py-2.5 text-[14px] font-mono border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#1254D8] focus:border-transparent"
          />
        </div>

        <div>
          <label className="block text-[13px] font-medium text-gray-700 mb-1.5">
            Token Contract
          </label>
          <input
            type="text"
            value={token}
            onChange={(e) => setToken(e.target.value)}
            placeholder="C..."
            className="w-full px-4 py-2.5 text-[14px] font-mono border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#1254D8] focus:border-transparent"
          />
        </div>

        <div>
          <label className="block text-[13px] font-medium text-gray-700 mb-1.5">
            Total Amount (XLM) *
          </label>
          <input
            type="number"
            step="0.01"
            min="0"
            value={totalAmount}
            onChange={(e) => setTotalAmount(e.target.value)}
            placeholder="100.00"
            required
            className="w-full px-4 py-2.5 text-[14px] border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#1254D8] focus:border-transparent"
          />
        </div>

        <button
          type="submit"
          disabled={submitting || !counterparty || !totalAmount}
          className="w-full py-3 text-[14px] font-semibold text-white rounded-[10px] transition-all hover:scale-[1.01] active:scale-[0.99] disabled:opacity-50 disabled:cursor-not-allowed"
          style={{
            background: "linear-gradient(135deg, #1254D8 0%, #2F70E8 100%)",
          }}
        >
          {submitting ? "Creating..." : "Create Agreement"}
        </button>
      </form>

      {result && (
        <div
          className={`mt-6 p-4 rounded-xl text-[13px] ${
            result.includes("Failed") || result.includes("error")
              ? "bg-red-50 border border-red-200 text-red-700"
              : "bg-green-50 border border-green-200 text-green-700"
          }`}
        >
          {result}
        </div>
      )}

      <p className="mt-6 text-[12px] text-gray-400 text-center">
        This creates an off-chain draft. On-chain Soroban contract integration is in progress.
      </p>
    </div>
  );
}
