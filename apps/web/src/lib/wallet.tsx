"use client";

import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";

// ── Types ────────────────────────────────────────────────────────────

export type WalletStatus = "disconnected" | "connecting" | "connected" | "error";

export interface WalletState {
  status: WalletStatus;
  address: string | null;
  network: "testnet" | "mainnet" | null;
  error: string | null;
  /** True once the initial mount-time check has completed. */
  ready: boolean;
}

interface WalletContextValue extends WalletState {
  /** Derived: true when status === "connected" */
  connected: boolean;
  /** Derived: true when status === "connecting" */
  connecting: boolean;
  connect: () => Promise<void>;
  disconnect: () => void;
  signTransaction: (txXdr: string) => Promise<string>;
}

// ── Context ──────────────────────────────────────────────────────────

const WalletContext = createContext<WalletContextValue | null>(null);

export function useWallet(): WalletContextValue {
  const ctx = useContext(WalletContext);
  if (!ctx) throw new Error("useWallet must be used within WalletProvider");
  return ctx;
}

// ── Freighter API resolution ─────────────────────────────────────────

/**
 * Resolve the Freighter API.
 *
 * 1. Check `window.freighter` (browser extension injection)
 * 2. Fall back to `@stellar/freighter-api` npm package (dynamic import)
 *
 * Returns null if neither is available.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
async function getFreighterApi(): Promise<any | null> {
  if (typeof window === "undefined") return null;

  // Path 1: Browser extension global
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const win = window as any;
  if (win.freighter) {
    return win.freighter;
  }

  // Path 2: npm package (requires @stellar/freighter-api installed)
  try {
    const mod = await import("@stellar/freighter-api");
    // The package exports functions both as named exports and on default
    if (mod && typeof mod.requestAccess === "function") {
      return mod;
    }
    if (mod?.default && typeof mod.default.requestAccess === "function") {
      return mod.default;
    }
    return mod;
  } catch {
    return null;
  }
}

// ── Network helpers ──────────────────────────────────────────────────

function parseNetwork(raw: string): "testnet" | "mainnet" {
  const lower = raw.toLowerCase();
  if (lower.includes("testnet")) return "testnet";
  return "mainnet";
}

const REQUIRED_NETWORK: "testnet" | "mainnet" =
  (process.env.NEXT_PUBLIC_STELLAR_NETWORK as "testnet" | "mainnet") ?? "testnet";

// ── Provider ─────────────────────────────────────────────────────────

const INITIAL_STATE: WalletState = {
  status: "disconnected",
  address: null,
  network: null,
  error: null,
  ready: false,
};

export function WalletProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<WalletState>(INITIAL_STATE);
  const mountedRef = useRef(false);

  // Check for existing connection on mount (browser only)
  useEffect(() => {
    if (mountedRef.current) return;
    mountedRef.current = true;

    async function checkExisting() {
      try {
        const freighter = await getFreighterApi();
        if (!freighter) {
          setState((s) => ({ ...s, ready: true }));
          return;
        }

        // Check if already connected
        if (typeof freighter.isConnected === "function") {
          const connected = await freighter.isConnected();
          if (connected && typeof freighter.getAddress === "function") {
            const { address } = await freighter.getAddress();
            const networkRaw =
              typeof freighter.getNetwork === "function"
                ? await freighter.getNetwork()
                : "testnet";
            const network = parseNetwork(String(networkRaw));

            setState({
              status: "connected",
              address,
              network,
              error: null,
              ready: true,
            });
            return;
          }
        }
      } catch {
        // Not connected or Freighter not available — that's fine
      }
      setState((s) => ({ ...s, ready: true }));
    }

    checkExisting();
  }, []);

  // ── connect ──────────────────────────────────────────────────────

  const connect = useCallback(async () => {
    setState((s) => ({ ...s, status: "connecting", error: null }));

    try {
      const freighter = await getFreighterApi();
      if (!freighter) {
        setState((s) => ({
          ...s,
          status: "error",
          error:
            "Stellar wallet not found. Install the Freighter browser extension from https://freighter.app",
        }));
        return;
      }

      // Request access — this opens the Freighter popup
      let address: string;
      if (typeof freighter.requestAccess === "function") {
        const result = await freighter.requestAccess();
        address = result.address ?? result;
      } else if (typeof freighter.getAddress === "function") {
        const result = await freighter.getAddress();
        address = result.address ?? result;
      } else {
        throw new Error("Freighter API does not support requestAccess or getAddress");
      }

      if (!address || typeof address !== "string") {
        throw new Error("No address returned from wallet");
      }

      // Get network
      let network: "testnet" | "mainnet" = "testnet";
      if (typeof freighter.getNetwork === "function") {
        const networkRaw = await freighter.getNetwork();
        network = parseNetwork(String(networkRaw));
      }

      // Check network match
      let error: string | null = null;
      if (network !== REQUIRED_NETWORK) {
        error = `Wallet is on ${network}, but Settle is configured for ${REQUIRED_NETWORK}. Switch your Freighter network to ${REQUIRED_NETWORK}.`;
      }

      setState({
        status: "connected",
        address,
        network,
        error,
        ready: true,
      });
    } catch (err: unknown) {
      // User rejected or Freighter error
      let message = "Failed to connect wallet";
      if (err && typeof err === "object") {
        const e = err as Record<string, unknown>;
        if (e.error && typeof e.error === "object") {
          const inner = e.error as Record<string, unknown>;
          if (typeof inner.message === "string") message = inner.message;
        } else if (typeof e.message === "string") {
          message = e.message;
        }
      } else if (typeof err === "string") {
        message = err;
      }

      // Detect user rejection
      const isRejected =
        message.toLowerCase().includes("reject") ||
        message.toLowerCase().includes("cancel") ||
        message.toLowerCase().includes("denied");

      setState((s) => ({
        ...s,
        status: isRejected ? "disconnected" : "error",
        error: isRejected ? "Wallet connection was cancelled." : message,
        ready: true,
      }));
    }
  }, []);

  // ── disconnect ───────────────────────────────────────────────────

  const disconnect = useCallback(() => {
    setState({
      status: "disconnected",
      address: null,
      network: null,
      error: null,
      ready: true,
    });
  }, []);

  // ── signTransaction ──────────────────────────────────────────────

  const signTransaction = useCallback(
    async (txXdr: string): Promise<string> => {
      const freighter = await getFreighterApi();
      if (!freighter) throw new Error("Wallet not connected");
      if (!state.address) throw new Error("Wallet not connected");

      const passphrase =
        state.network === "mainnet"
          ? "Public Global Stellar Network ; September 2015"
          : "Test SDF Network ; September 2015";

      // Freighter v6+ uses `address` param
      const result = await freighter.signTransaction(txXdr, {
        networkPassphrase: passphrase,
        address: state.address,
      });

      if (result.error) {
        throw new Error(result.error.message ?? "Transaction signing failed");
      }

      return result.signedTxXdr;
    },
    [state.address, state.network]
  );

  // ── Derived convenience fields ──────────────────────────────────

  const connected = state.status === "connected";
  const connecting = state.status === "connecting";

  return (
    <WalletContext.Provider
      value={{
        ...state,
        connected,
        connecting,
        connect,
        disconnect,
        signTransaction,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
}
