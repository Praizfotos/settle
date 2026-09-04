"use client";

import { useEffect, useState, useCallback } from "react";
import { useParams } from "next/navigation";
import { useWallet } from "@/lib/wallet";
import { getAgreement, listMilestones } from "@/lib/api";
import type { ApiAgreement, ApiMilestone } from "@/lib/api";
import { Address, Contract, nativeToScVal, SorobanRpc, TransactionBuilder } from "@stellar/stellar-sdk";
import Link from "next/link";

const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_STELLAR_CONTRACT_ADDRESS ?? "";

type ActionState = "idle" | "signing" | "submitting" | "confirming" | "success" | "error";

function StatusBadge({ status }: { status: string }) {
  const colors: Record<string, string> = {
    Draft: "bg-gray-100 text-gray-600",
    Funded: "bg-blue-100 text-blue-700",
    Active: "bg-green-100 text-green-700",
    Completed: "bg-green-100 text-green-700",
    Disputed: "bg-red-100 text-red-700",
    Expired: "bg-yellow-100 text-yellow-700",
    Cancelled: "bg-gray-100 text-gray-500",
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

async function invokeContract(
  method: string,
  args: any[],
  address: string
): Promise<{ hash: string }> {
  const server = new SorobanRpc.Server(RPC_URL, { allowHttp: true });
  const account = await server.getAccount(address);
  const contract = new Contract(CONTRACT_ADDRESS);

  const txBuilder = new TransactionBuilder(account, {
    fee: "100000",
    networkPassphrase: NETWORK_PASSPHRASE,
  })
    .addOperation(contract.call(method, ...args))
    .setTimeout(300);

  const transaction = txBuilder.build();

  const freighter = await import("@stellar/freighter-api");
  const txXdr = transaction.toXDR("base64");
  const signedResult = await freighter.signTransaction(txXdr, {
    networkPassphrase: NETWORK_PASSPHRASE,
    address,
  });
  const signedXdr = typeof signedResult === "string"
    ? signedResult
    : signedResult.signedTxXdr ?? signedResult.signedTxBase64;

  const sendResponse = await fetch(RPC_URL, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: 1,
      method: "sendTransaction",
      params: { tx: signedXdr },
    }),
  }).then((r) => r.json());

  if (sendResponse.error) {
    throw new Error(`Transaction failed: ${JSON.stringify(sendResponse.error)}`);
  }

  const txHash = sendResponse.result?.hash;
  if (!txHash) {
    throw new Error("No transaction hash in response: " + JSON.stringify(sendResponse));
  }

  // Poll for confirmation
  for (let i = 0; i < 30; i++) {
    await new Promise((r) => setTimeout(r, 2000));
    const txResult = await server.getTransaction(txHash);
    if (txResult.status !== "NOT_FOUND") {
      if (txResult.status === "SUCCESS") {
        return { hash: txHash };
      }
      throw new Error(`Transaction failed: ${txResult.status}`);
    }
  }

  throw new Error("Transaction not confirmed after 60 seconds");
}

export default function AgreementDetailPage() {
  const params = useParams();
  const id = params.id as string;
  const { connected, address } = useWallet();
  const [agreement, setAgreement] = useState<ApiAgreement | null>(null);
  const [milestones, setMilestones] = useState<ApiMilestone[]>([]);
  const [loading, setLoading] = useState(true);
  const [actionState, setActionState] = useState<ActionState>("idle");
  const [actionName, setActionName] = useState<string>("");
  const [txHash, setTxHash] = useState<string | null>(null);
  const [actionError, setActionError] = useState<string | null>(null);

  const fetchData = useCallback(() => {
    if (!id) return;
    Promise.all([
      getAgreement(id).catch(() => null),
      listMilestones(id).catch(() => ({ milestones: [] })),
    ])
      .then(([agr, mil]) => {
        setAgreement(agr);
        setMilestones(mil.milestones);
      })
      .finally(() => setLoading(false));
  }, [id]);

  useEffect(() => {
    fetchData();
  }, [fetchData]);

  async function executeAction(method: string, args: any[], name: string) {
    if (!address) return;
    setActionState("signing");
    setActionName(name);
    setTxHash(null);
    setActionError(null);

    try {
      const result = await invokeContract(method, args, address);
      setTxHash(result.hash);
      setActionState("success");
      fetchData();
    } catch (err: any) {
      setActionState("error");
      setActionError(err?.message ?? "Unknown error");
    }
  }

  function handleFund() {
    if (!agreement) return;
    const amount = BigInt(agreement.total_amount - agreement.funded_amount);
    executeAction(
      "fund_agreement",
      [
        nativeToScVal(agreement.on_chain_id, { type: "string" }),
        Address.fromString(address!).toScVal(),
        nativeToScVal(amount.toString(), { type: "i128" }),
      ],
      "Fund Agreement"
    );
  }

  function handleActivate() {
    if (!agreement) return;
    executeAction(
      "activate_agreement",
      [
        nativeToScVal(agreement.on_chain_id, { type: "string" }),
        Address.fromString(address!).toScVal(),
      ],
      "Activate Agreement"
    );
  }

  function handleComplete() {
    if (!agreement) return;
    executeAction(
      "complete_agreement",
      [
        nativeToScVal(agreement.on_chain_id, { type: "string" }),
        Address.fromString(address!).toScVal(),
      ],
      "Complete Agreement"
    );
  }

  function handleCancel() {
    if (!agreement) return;
    executeAction(
      "cancel_agreement",
      [
        nativeToScVal(agreement.on_chain_id, { type: "string" }),
        Address.fromString(address!).toScVal(),
      ],
      "Cancel Agreement"
    );
  }

  if (loading) {
    return (
      <div className="max-w-[800px] mx-auto px-6 py-10">
        <div className="animate-pulse space-y-4">
          <div className="h-8 w-64 bg-gray-200 rounded" />
          <div className="h-40 bg-gray-200 rounded-2xl" />
        </div>
      </div>
    );
  }

  if (!agreement) {
    return (
      <div className="max-w-[800px] mx-auto px-6 py-16 text-center">
        <p className="text-[16px] text-gray-500">Agreement not found.</p>
        <Link
          href="/app/agreements"
          className="text-[14px] text-[#1254D8] hover:underline mt-2 inline-block"
        >
          ← Back to agreements
        </Link>
      </div>
    );
  }

  const fundedPct =
    agreement.total_amount > 0
      ? (agreement.funded_amount / agreement.total_amount) * 100
      : 0;

  const statusLower = agreement.status?.toLowerCase() ?? "";
  const isCreator = address && agreement.client === address;
  const isCounterparty = address && agreement.provider === address;

  return (
    <div className="max-w-[800px] mx-auto px-6 py-10">
      <Link
        href="/app/agreements"
        className="text-[13px] text-gray-400 hover:text-gray-600 mb-4 inline-block"
      >
        ← Agreements
      </Link>

      <div className="flex items-start justify-between mb-8">
        <div>
          <h1 className="text-[24px] font-bold text-gray-900 tracking-tight">
            {agreement.title ?? `Agreement ${agreement.on_chain_id}`}
          </h1>
          <p className="text-[13px] text-gray-400 mt-1 font-mono">
            Chain ID: {agreement.on_chain_id}
          </p>
        </div>
        <StatusBadge status={agreement.status} />
      </div>

      {/* Action Buttons */}
      {connected && (
        <div className="bg-white rounded-2xl border border-gray-200 p-4 mb-6">
          <div className="flex items-center gap-3 flex-wrap">
            {statusLower === "draft" && isCreator && (
              <button
                onClick={handleFund}
                disabled={actionState !== "idle"}
                className="px-4 py-2 text-[13px] font-semibold text-white rounded-lg bg-[#1254D8] hover:bg-[#0D4FD7] disabled:opacity-50 transition-colors"
              >
                {actionName === "Fund Agreement" && actionState !== "idle"
                  ? `${actionState}...`
                  : "Fund Agreement"}
              </button>
            )}
            {statusLower === "funded" && isCreator && (
              <button
                onClick={handleActivate}
                disabled={actionState !== "idle"}
                className="px-4 py-2 text-[13px] font-semibold text-white rounded-lg bg-green-600 hover:bg-green-700 disabled:opacity-50 transition-colors"
              >
                {actionName === "Activate Agreement" && actionState !== "idle"
                  ? `${actionState}...`
                  : "Activate Agreement"}
              </button>
            )}
            {statusLower === "active" && isCounterparty && (
              <button
                onClick={handleComplete}
                disabled={actionState !== "idle"}
                className="px-4 py-2 text-[13px] font-semibold text-white rounded-lg bg-green-600 hover:bg-green-700 disabled:opacity-50 transition-colors"
              >
                {actionName === "Complete Agreement" && actionState !== "idle"
                  ? `${actionState}...`
                  : "Complete Agreement"}
              </button>
            )}
            {(statusLower === "draft" || statusLower === "funded") && isCreator && (
              <button
                onClick={handleCancel}
                disabled={actionState !== "idle"}
                className="px-4 py-2 text-[13px] font-semibold text-gray-600 rounded-lg border border-gray-300 hover:bg-gray-50 disabled:opacity-50 transition-colors"
              >
                {actionName === "Cancel Agreement" && actionState !== "idle"
                  ? `${actionState}...`
                  : "Cancel"}
              </button>
            )}
          </div>

          {txHash && (
            <div className="mt-3 p-3 rounded-lg bg-blue-50 border border-blue-200 text-blue-700 text-[12px]">
              <span className="font-medium">Tx:</span>{" "}
              <a
                href={`https://stellar.expert/explorer/testnet/tx/${txHash}`}
                target="_blank"
                rel="noopener noreferrer"
                className="underline font-mono"
              >
                {txHash.slice(0, 16)}...
              </a>
            </div>
          )}
          {actionError && (
            <div className="mt-3 p-3 rounded-lg bg-red-50 border border-red-200 text-red-700 text-[12px]">
              {actionError}
            </div>
          )}
        </div>
      )}

      {/* Overview */}
      <div className="bg-white rounded-2xl border border-gray-200 p-6 mb-6">
        <h2 className="text-[15px] font-semibold text-gray-900 mb-4">
          Overview
        </h2>
        <div className="grid grid-cols-2 gap-4">
          <div>
            <p className="text-[12px] text-gray-400">Total Amount</p>
            <p className="text-[18px] font-semibold text-gray-900">
              {(agreement.total_amount / 1_000_000).toFixed(2)} XLM
            </p>
          </div>
          <div>
            <p className="text-[12px] text-gray-400">Funded</p>
            <p className="text-[18px] font-semibold text-gray-900">
              {(agreement.funded_amount / 1_000_000).toFixed(2)} XLM
            </p>
          </div>
          <div>
            <p className="text-[12px] text-gray-400">Creator</p>
            <p className="text-[13px] font-mono text-gray-600 truncate">
              {agreement.client}
            </p>
          </div>
          <div>
            <p className="text-[12px] text-gray-400">Counterparty</p>
            <p className="text-[13px] font-mono text-gray-600 truncate">
              {agreement.provider}
            </p>
          </div>
        </div>

        <div className="mt-4">
          <div className="h-2 bg-gray-100 rounded-full overflow-hidden">
            <div
              className="h-full bg-[#1254D8] rounded-full transition-all"
              style={{ width: `${Math.min(fundedPct, 100)}%` }}
            />
          </div>
          <p className="text-[11px] text-gray-400 mt-1">
            {fundedPct.toFixed(0)}% funded
          </p>
        </div>
      </div>

      {/* Milestones */}
      <div className="bg-white rounded-2xl border border-gray-200 overflow-hidden">
        <div className="px-6 py-4 border-b border-gray-100">
          <h2 className="text-[15px] font-semibold text-gray-900">
            Milestones
          </h2>
        </div>
        {milestones.length === 0 ? (
          <div className="px-6 py-8 text-center">
            <p className="text-[13px] text-gray-400">
              No milestones defined for this agreement.
            </p>
          </div>
        ) : (
          <div className="divide-y divide-gray-100">
            {milestones.map((m) => (
              <div key={m.id} className="px-6 py-4">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-[14px] font-medium text-gray-900">
                      {m.name}
                    </p>
                    <p className="text-[12px] text-gray-400 mt-0.5">
                      {m.description}
                    </p>
                  </div>
                  <div className="flex items-center gap-3">
                    <StatusBadge status={m.status} />
                    <span className="text-[13px] font-mono text-gray-500">
                      {(m.amount / 1_000_000).toFixed(2)}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
