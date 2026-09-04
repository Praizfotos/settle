"use client";

import { useWallet } from "@/lib/wallet";

export default function SettingsPage() {
  const { connected, address, network, disconnect } = useWallet();

  return (
    <div className="max-w-[640px] mx-auto px-6 py-10">
      <h1 className="text-[28px] font-bold text-gray-900 tracking-tight mb-8">
        Settings
      </h1>

      <div className="bg-white rounded-2xl border border-gray-200 p-6 mb-6">
        <h2 className="text-[15px] font-semibold text-gray-900 mb-4">
          Wallet
        </h2>
        {connected && address ? (
          <div className="space-y-3">
            <div>
              <p className="text-[12px] text-gray-400">Address</p>
              <p className="text-[14px] font-mono text-gray-700">{address}</p>
            </div>
            <div>
              <p className="text-[12px] text-gray-400">Network</p>
              <p className="text-[14px] text-gray-700">{network}</p>
            </div>
            <button
              onClick={disconnect}
              className="mt-2 px-4 py-2 text-[13px] font-medium text-red-600 border border-red-200 rounded-xl hover:bg-red-50 transition-colors"
            >
              Disconnect Wallet
            </button>
          </div>
        ) : (
          <p className="text-[14px] text-gray-400">No wallet connected</p>
        )}
      </div>

      <div className="bg-white rounded-2xl border border-gray-200 p-6">
        <h2 className="text-[15px] font-semibold text-gray-900 mb-4">
          Network Configuration
        </h2>
        <div className="space-y-2">
          <div>
            <p className="text-[12px] text-gray-400">Stellar Network</p>
            <p className="text-[14px] text-gray-700">
              {process.env.NEXT_PUBLIC_STELLAR_NETWORK ?? "testnet"}
            </p>
          </div>
          <div>
            <p className="text-[12px] text-gray-400">RPC URL</p>
            <p className="text-[13px] font-mono text-gray-600 break-all">
              {process.env.NEXT_PUBLIC_STELLAR_RPC_URL ?? "https://soroban-testnet.stellar.org"}
            </p>
          </div>
          <div>
            <p className="text-[12px] text-gray-400">Contract Address</p>
            <p className="text-[13px] font-mono text-gray-600 break-all">
              {process.env.NEXT_PUBLIC_STELLAR_CONTRACT_ADDRESS ?? "Not configured"}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
