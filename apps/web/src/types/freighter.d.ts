// Freighter API type declarations
// Matches @stellar/freighter-api v6.x

/* eslint-disable @typescript-eslint/no-explicit-any */

interface FreighterApiError {
  message?: string;
  code?: string;
}

declare module "@stellar/freighter-api" {
  export function isConnected(): Promise<boolean>;
  export function getAddress(): Promise<{ address: string; error?: FreighterApiError }>;
  export function requestAccess(): Promise<{ address: string; error?: FreighterApiError }>;
  export function getNetwork(): Promise<string>;
  export function getNetworkDetails(): Promise<{ networkPassphrase: string; [key: string]: any }>;
  export function isAllowed(): Promise<boolean>;
  export function setAllowed(allowed: boolean): Promise<void>;
  export function signTransaction(
    transactionXdr: string,
    opts?: { networkPassphrase?: string; address?: string }
  ): Promise<{ signedTxXdr: string; signerAddress: string; error?: FreighterApiError }>;
  export function signMessage(
    message: string,
    opts?: { networkPassphrase?: string; address?: string }
  ): Promise<{ signedMessage: string | Buffer | null; signerAddress: string; error?: FreighterApiError }>;
  export function signAuthEntry(
    entryXdr: string,
    opts?: { networkPassphrase?: string; address?: string }
  ): Promise<{ signedAuthEntry: string; signerAddress: string; error?: FreighterApiError }>;
  export function addToken(args: {
    contractId: string;
    networkPassphrase?: string;
  }): Promise<{ contractId: string; error?: FreighterApiError }>;
  export function WatchWalletChanges(callback: (wallet: any) => void): { stop: () => void };

  const _default: {
    isConnected: typeof isConnected;
    getAddress: typeof getAddress;
    requestAccess: typeof requestAccess;
    getNetwork: typeof getNetwork;
    getNetworkDetails: typeof getNetworkDetails;
    isAllowed: typeof isAllowed;
    setAllowed: typeof setAllowed;
    signTransaction: typeof signTransaction;
    signMessage: typeof signMessage;
    signAuthEntry: typeof signAuthEntry;
    addToken: typeof addToken;
    WatchWalletChanges: typeof WatchWalletChanges;
  };
  export default _default;
}

// Allow window.freighter from the browser extension
interface Window {
  freighter?: {
    isConnected(): Promise<boolean>;
    getAddress(): Promise<{ address: string }>;
    requestAccess(): Promise<{ address: string }>;
    getNetwork(): Promise<string>;
    signTransaction(
      txXdr: string,
      opts?: { networkPassphrase?: string; address?: string }
    ): Promise<{ signedTxXdr: string }>;
  };
}
