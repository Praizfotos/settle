// ===========================================
// @settle/stellar
// Stellar/Soroban network utilities
// ===========================================

export {
  networkConfig,
  getNetworkPassphrase,
  getRpcUrl,
  getHorizonUrl,
  isTestnet,
  isMainnet,
} from "./network";

export {
  contractInvocation,
  buildContractCall,
  contractId,
  SorobanContractClient,
} from "./contracts";

export {
  shortAddress,
  isValidStellarAddress,
  explorerBaseUrl,
  txExplorerUrl,
} from "./helpers";
