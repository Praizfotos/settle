"use client";

import { useWallet } from "@/lib/wallet";
import { SettleLogo } from "@/components/layout/SettleLogo";
import Link from "next/link";
import { usePathname } from "next/navigation";

function shortAddress(addr: string, chars = 4): string {
  return `${addr.slice(0, chars)}...${addr.slice(-chars)}`;
}

export function AppNavigation() {
  const { connected, address, network, connecting, connect, disconnect } =
    useWallet();
  const pathname = usePathname();

  const links = [
    { label: "Dashboard", href: "/app" },
    { label: "Agreements", href: "/app/agreements" },
    { label: "Disputes", href: "/app/disputes" },
  ];

  return (
    <header className="fixed top-0 left-0 right-0 z-50 bg-white/95 backdrop-blur-md shadow-[0_1px_0_0_#E5E7EB]">
      <div className="max-w-[1280px] mx-auto px-6 md:px-10 h-[60px] flex items-center justify-between gap-8">
        <div className="flex items-center gap-8">
          <Link href="/" aria-label="Settle home" className="flex-shrink-0">
            <SettleLogo />
          </Link>
          <nav className="hidden md:flex items-center gap-1">
            {links.map((link) => (
              <Link
                key={link.href}
                href={link.href}
                className={`px-3.5 py-1.5 text-[13.5px] font-medium rounded-lg transition-colors duration-150 ${
                  pathname === link.href
                    ? "text-gray-900 bg-gray-100"
                    : "text-gray-500 hover:text-gray-900 hover:bg-gray-50"
                }`}
              >
                {link.label}
              </Link>
            ))}
          </nav>
        </div>

        <div className="flex items-center gap-3">
          {network && (
            <span
              className={`px-2 py-0.5 text-[11px] font-medium rounded-full ${
                network === "testnet"
                  ? "bg-yellow-100 text-yellow-800"
                  : "bg-green-100 text-green-800"
              }`}
            >
              {network}
            </span>
          )}

          {connected && address ? (
            <div className="flex items-center gap-2">
              <span className="text-[13px] font-mono text-gray-600">
                {shortAddress(address)}
              </span>
              <button
                onClick={disconnect}
                className="px-3 py-1.5 text-[13px] font-medium text-gray-500 hover:text-gray-900 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors"
              >
                Disconnect
              </button>
            </div>
          ) : (
            <button
              onClick={connect}
              disabled={connecting}
              className="px-4 py-2 text-[13.5px] font-semibold text-white rounded-[10px] transition-all duration-150 hover:scale-[1.02] active:scale-[0.98] disabled:opacity-50"
              style={{
                background: "linear-gradient(135deg, #1254D8 0%, #2F70E8 100%)",
                boxShadow: "0 1px 3px rgba(18,84,216,0.35)",
              }}
            >
              {connecting ? "Connecting..." : "Connect Wallet"}
            </button>
          )}
        </div>
      </div>
    </header>
  );
}
