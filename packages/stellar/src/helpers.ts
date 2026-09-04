// ===========================================
// Stellar address and explorer helpers
// ===========================================

/**
 * Shorten a Stellar address for display: GABC...XYZ
 */
export function shortAddress(address: string, chars = 4): string {
  if (address.length <= chars * 2 + 3) return address;
  return `${address.slice(0, chars)}...${address.slice(-chars)}`;
}

/**
 * Validate a Stellar address format (starts with G or S, 56 chars).
 */
export function isValidStellarAddress(address: string): boolean {
  return /^[GS][A-Z0-9]{55}$/.test(address);
}

/**
 * Get the Stellar explorer base URL for a network.
 */
export function explorerBaseUrl(network: "testnet" | "mainnet" = "testnet"): string {
  return network === "mainnet"
    ? "https://stellar.expert/explorer/public"
    : "https://stellar.expert/explorer/testnet";
}

/**
 * Get the explorer URL for a transaction hash.
 */
export function txExplorerUrl(
  txHash: string,
  network: "testnet" | "mainnet" = "testnet"
): string {
  return `${explorerBaseUrl(network)}/tx/${txHash}`;
}

/**
 * Get the explorer URL for a contract.
 */
export function contractExplorerUrl(
  contractAddress: string,
  network: "testnet" | "mainnet" = "testnet"
): string {
  return `${explorerBaseUrl(network)}/contract/${contractAddress}`;
}

/**
 * Get the explorer URL for an account.
 */
export function accountExplorerUrl(
  address: string,
  network: "testnet" | "mainnet" = "testnet"
): string {
  return `${explorerBaseUrl(network)}/account/${address}`;
}
