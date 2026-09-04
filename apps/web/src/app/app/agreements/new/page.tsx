"use client";

import { useState, useCallback } from "react";
import { useWallet } from "@/lib/wallet";
import { useRouter } from "next/navigation";
import { Address, Contract, nativeToScVal, StrKey, TransactionBuilder, xdr } from "@stellar/stellar-sdk";
import * as SorobanRpc from "@stellar/stellar-sdk/rpc";

const RPC_URL = "https://soroban-testnet.stellar.org";
const NETWORK_PASSPHRASE = "Test SDF Network ; September 2015";
const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_STELLAR_CONTRACT_ADDRESS ?? "";

type TxState =
  | "idle"
  | "building"
  | "signing"
  | "submitting"
  | "confirming"
  | "success"
  | "error";

export default function NewAgreementPage() {
  const { connected, address } = useWallet();
  const router = useRouter();

  const [counterparty, setCounterparty] = useState("");
  const [token, setToken] = useState(
    "CCQPQBKJ3D6WTU3CFINUZ5XLQFMMVXO6NJTWTSQJSMSCGTYLOEO4K7EZ"
  );
  const [totalAmount, setTotalAmount] = useState("");
  const [title, setTitle] = useState("");
  const [milestoneInput, setMilestoneInput] = useState("");
  const [txState, setTxState] = useState<TxState>("idle");
  const [txHash, setTxHash] = useState<string | null>(null);
  const [txError, setTxError] = useState<string | null>(null);

  function toAddressScVal(val: string) {
    if (val.startsWith("C")) {
      const rawBytes = StrKey.decodeContract(val);
      return Address.contract(Buffer.from(rawBytes)).toScVal();
    }
    return Address.fromString(val).toScVal();
  }

  const generateId = useCallback(() => {
    const chars = "abcdefghijklmnopqrstuvwxyz0123456789";
    let id = "agr-";
    for (let i = 0; i < 12; i++) {
      id += chars[Math.floor(Math.random() * chars.length)];
    }
    return id;
  }, []);

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
    if (!address) return;

    const agreementId = generateId();
    const amountBigInt = BigInt(Math.round(parseFloat(totalAmount) * 1_000_000));
    const expiresAt = BigInt(Math.floor(Date.now() / 1000) + 86400 * 30);
    const milestoneNames = milestoneInput
      ? milestoneInput.split(",").map((m) => m.trim()).filter(Boolean)
      : ["default-milestone"];

    setTxState("building");
    setTxHash(null);
    setTxError(null);

    try {
      const server = new SorobanRpc.Server(RPC_URL, { allowHttp: true });
      const account = await server.getAccount(address);
      const contract = new Contract(CONTRACT_ADDRESS);

      const milestoneScVals = milestoneNames.map((name) =>
        nativeToScVal(name, { type: "string" })
      );

      const args = [
        nativeToScVal(agreementId, { type: "string" }),
        toAddressScVal(address),
        toAddressScVal(counterparty),
        toAddressScVal(token),
        nativeToScVal(amountBigInt.toString(), { type: "i128" }),
        nativeToScVal(expiresAt.toString(), { type: "u64" }),
        xdr.ScVal.scvVec(milestoneScVals),
      ];

      const txBuilder = new TransactionBuilder(account, {
        fee: "100000",
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(contract.call("create_agreement", ...args))
        .setTimeout(300);

      const transaction = txBuilder.build();

      setTxState("signing");

      // Use Freighter to sign
      const freighter = await import("@stellar/freighter-api");
      const txXdr = transaction.toXDR("base64");
      const signedResult = await freighter.signTransaction(txXdr, {
        networkPassphrase: NETWORK_PASSPHRASE,
        address,
      });
      const signedXdr = typeof signedResult === "string"
        ? signedResult
        : signedResult.signedTxXdr ?? signedResult.signedTxBase64 ?? JSON.stringify(signedResult);

      if (!signedXdr || typeof signedXdr !== "string") {
        throw new Error("Freighter did not return signed XDR. Got: " + JSON.stringify(signedResult));
      }

      setTxState("submitting");

      const sendResult = await fetch(RPC_URL, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          jsonrpc: "2.0",
          id: 1,
          method: "sendTransaction",
          params: {
            tx: signedXdr,
          },
        }),
      }).then((r) => r.json());

      if (sendResult.error) {
        throw new Error(`Transaction failed: ${JSON.stringify(sendResult.error)}`);
      }

      const txHash = sendResult.result?.hash ?? sendResult.hash;
      setTxHash(txHash);
      setTxState("confirming");

      // Poll for confirmation
      for (let i = 0; i < 30; i++) {
        await new Promise((r) => setTimeout(r, 2000));
        const txResult = await server.getTransaction(txHash);
        if (txResult.status !== "NOT_FOUND") {
          if (txResult.status === "SUCCESS") {
            setTxState("success");
            setTimeout(() => router.push("/app/agreements"), 3000);
            return;
          } else {
            throw new Error(`Transaction failed: ${txResult.status}`);
          }
        }
      }

      throw new Error("Transaction not confirmed after 60 seconds");
    } catch (err: any) {
      setTxState("error");
      setTxError(err?.message ?? "Unknown error");
    }
  }

  return (
    <div className="max-w-[640px] mx-auto px-6 py-10">
      <h1 className="text-[28px] font-bold text-gray-900 tracking-tight mb-2">
        Create Agreement
      </h1>
      <p className="text-[14px] text-gray-500 mb-8">
        Deploy a programmable settlement agreement on Stellar Testnet.
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

        <div>
          <label className="block text-[13px] font-medium text-gray-700 mb-1.5">
            Milestones (comma-separated names)
          </label>
          <input
            type="text"
            value={milestoneInput}
            onChange={(e) => setMilestoneInput(e.target.value)}
            placeholder="Design, Development, Launch"
            className="w-full px-4 py-2.5 text-[14px] border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#1254D8] focus:border-transparent"
          />
        </div>

        <button
          type="submit"
          disabled={
            txState !== "idle" ||
            !counterparty ||
            !totalAmount
          }
          className="w-full py-3 text-[14px] font-semibold text-white rounded-[10px] transition-all hover:scale-[1.01] active:scale-[0.99] disabled:opacity-50 disabled:cursor-not-allowed"
          style={{
            background: "linear-gradient(135deg, #1254D8 0%, #2F70E8 100%)",
          }}
        >
          {txState === "idle" && "Create & Sign Agreement"}
          {txState === "building" && "Building transaction..."}
          {txState === "signing" && "Please sign in Freighter..."}
          {txState === "submitting" && "Submitting to Testnet..."}
          {txState === "confirming" && "Waiting for confirmation..."}
          {txState === "success" && "Agreement created!"}
          {txState === "error" && "Try again"}
        </button>
      </form>

      {txHash && (
        <div className="mt-6 p-4 rounded-xl bg-blue-50 border border-blue-200 text-blue-700 text-[13px]">
          <span className="font-medium">Tx Hash:</span>{" "}
          <a
            href={`https://stellar.expert/explorer/testnet/tx/${txHash}`}
            target="_blank"
            rel="noopener noreferrer"
            className="underline font-mono"
          >
            {txHash}
          </a>
        </div>
      )}

      {txError && (
        <div className="mt-6 p-4 rounded-xl bg-red-50 border border-red-200 text-red-700 text-[13px]">
          {txError}
        </div>
      )}

      <p className="mt-6 text-[12px] text-gray-400 text-center">
        This deploys a real agreement on Stellar Testnet via Soroban smart contract.
      </p>
    </div>
  );
}
