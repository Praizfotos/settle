// ===========================================
// Stellar network configuration
// ===========================================

export interface NetworkConfig {
  network: "testnet" | "mainnet";
  networkPassphrase: string;
  rpcUrl: string;
  horizonUrl: string;
}

const TESTNET_CONFIG: NetworkConfig = {
  network: "testnet",
  networkPassphrase: "Test SDF Network ; September 2015",
  rpcUrl: "https://soroban-testnet.stellar.org",
  horizonUrl: "https://horizon-testnet.stellar.org",
};

const MAINNET_CONFIG: NetworkConfig = {
  network: "mainnet",
  networkPassphrase: "Public Global Stellar Network ; September 2015",
  rpcUrl: "https://soroban-mainnet.stellar.org",
  horizonUrl: "https://horizon.stellar.org",
};

/**
 * Get network config from environment or explicit network name.
 * For browser use, reads NEXT_PUBLIC_* env vars.
 * For Node.js, reads process.env directly.
 */
export function networkConfig(
  network?: "testnet" | "mainnet"
): NetworkConfig {
  const envNetwork =
    network ??
    (typeof process !== "undefined"
      ? (process.env.NEXT_PUBLIC_STELLAR_NETWORK ??
        process.env.STELLAR_NETWORK ??
        "testnet")
      : "testnet") as "testnet" | "mainnet";

  const base = envNetwork === "mainnet" ? MAINNET_CONFIG : TESTNET_CONFIG;

  return {
    ...base,
    networkPassphrase:
      (typeof process !== "undefined"
        ? (process.env.NEXT_PUBLIC_STELLAR_NETWORK_PASSPHRASE ??
          process.env.STELLAR_NETWORK_PASSPHRASE)
        : undefined) ?? base.networkPassphrase,
    rpcUrl:
      (typeof process !== "undefined"
        ? (process.env.NEXT_PUBLIC_STELLAR_RPC_URL ??
          process.env.STELLAR_RPC_URL)
        : undefined) ?? base.rpcUrl,
    horizonUrl:
      (typeof process !== "undefined"
        ? (process.env.NEXT_PUBLIC_STELLAR_HORIZON_URL ??
          process.env.STELLAR_HORIZON_URL)
        : undefined) ?? base.horizonUrl,
  };
}

export function getNetworkPassphrase(network?: "testnet" | "mainnet"): string {
  return networkConfig(network).networkPassphrase;
}

export function getRpcUrl(network?: "testnet" | "mainnet"): string {
  return networkConfig(network).rpcUrl;
}

export function getHorizonUrl(network?: "testnet" | "mainnet"): string {
  return networkConfig(network).horizonUrl;
}

export function isTestnet(network?: "testnet" | "mainnet"): boolean {
  return networkConfig(network).network === "testnet";
}

export function isMainnet(network?: "testnet" | "mainnet"): boolean {
  return networkConfig(network).network === "mainnet";
}
