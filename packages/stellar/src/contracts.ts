// ===========================================
// Soroban contract interaction utilities
// ===========================================

import * as SorobanRpc from "@stellar/stellar-sdk/rpc";
import * as xdr from "@stellar/stellar-sdk/lib/xdr";
import { Address, Contract } from "@stellar/stellar-sdk";
import { networkConfig } from "./network";

/**
 * Client for interacting with Settle Soroban contracts.
 */
export class SorobanContractClient {
  private server: SorobanRpc.Server;
  private contractAddress: string;
  private networkPassphrase: string;

  constructor(opts: {
    rpcUrl: string;
    contractAddress: string;
    networkPassphrase: string;
  }) {
    this.server = new SorobanRpc.Server(opts.rpcUrl, { allowHttp: true });
    this.contractAddress = opts.contractAddress;
    this.networkPassphrase = opts.networkPassphrase;
  }

  /**
   * Create a client from environment config.
   */
  static fromEnv(network?: "testnet" | "mainnet"): SorobanContractClient {
    const config = networkConfig(network);
    const contractAddress =
      (typeof process !== "undefined"
        ? (process.env.NEXT_PUBLIC_STELLAR_CONTRACT_ADDRESS ??
          process.env.STELLAR_CONTRACT_ADDRESS)
        : undefined) ?? "";

    return new SorobanContractClient({
      rpcUrl: config.rpcUrl,
      contractAddress,
      networkPassphrase: config.networkPassphrase,
    });
  }

  get contract(): Contract {
    return new Contract(this.contractAddress);
  }

  /**
   * Build a Soroban transaction that invokes a contract method.
   * Returns the transaction XDR string for signing.
   */
  async buildContractCall(opts: {
    method: string;
    args: xdr.ScVal[];
    sourceAccount: string;
    simulate?: boolean;
  }): Promise<{
    txXdr: string;
    simulation?: SorobanRpc.Api.SimulateTransactionResponse;
  }> {
    const account = await this.server.getAccount(opts.sourceAccount);

    const txBuilder = new SorobanRpc.TransactionBuilder(account, {
      fee: "100000",
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(
        this.contract.call(opts.method, ...opts.args)
      )
      .setTimeout(300);

    const transaction = txBuilder.build();

    let simulation: SorobanRpc.Api.SimulateTransactionResponse | undefined;
    if (opts.simulate !== false) {
      simulation = await this.server.simulateTransaction(transaction);
    }

    return {
      txXdr: transaction.toXDR(),
      simulation,
    };
  }

  /**
   * Submit a signed transaction to the network.
   */
  async submitTransaction(
    signedTxXdr: string
  ): Promise<SorobanRpc.Api.SendTransactionResponse> {
    const transaction = xdr.Transaction.fromXDR(
      signedTxXdr,
      this.networkPassphrase
    );
    return this.server.sendTransaction(transaction);
  }

  /**
   * Poll for transaction completion.
   */
  async waitForTransaction(
    hash: string,
    maxAttempts = 30,
    intervalMs = 2000
  ): Promise<SorobanRpc.Api.GetTransactionResponse> {
    for (let i = 0; i < maxAttempts; i++) {
      const result = await this.server.getTransaction(hash);
      if (result.status !== "NOT_FOUND") {
        return result;
      }
      await new Promise((resolve) => setTimeout(resolve, intervalMs));
    }
    throw new Error(`Transaction ${hash} not confirmed after ${maxAttempts} attempts`);
  }

  /**
   * Read contract state via simulateTransaction with readOnly invocation.
   */
  async readContract(opts: {
    method: string;
    args: xdr.ScVal[];
  }): Promise<xdr.ScVal> {
    const address = Address.fromString(this.contractAddress);
    const invoke = new xdr.InvokeHostFunctionOp({
      hostFunction: new xdr.HostFunction.hostFunctionTypeInvoke(
        new xdr.InvokeContractArgs({
          contractAddress: address.toScAddress(),
          function: new xdr.ScSymbol(opts.method),
          args: opts.args,
        })
      ),
      auth: [],
    });

    // For read-only, we build a transaction from a non-existent account
    // and simulate only. The server handles this.
    const dummyAccount = new SorobanRpc.Account(
      "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
      "0"
    );

    const tx = new SorobanRpc.TransactionBuilder(dummyAccount, {
      fee: "0",
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(invoke)
      .build();

    const result = await this.server.simulateTransaction(tx);
    if (
      result.status === "SUCCESS" &&
      "result" in result &&
      result.result
    ) {
      return result.result.retval;
    }
    throw new Error(`Contract read failed: ${JSON.stringify(result)}`);
  }
}

/**
 * Helper to build contract call arguments.
 */
export function contractInvocation(contractAddress: string) {
  const contract = new Contract(contractAddress);
  return {
    contract,
    call: (method: string, ...args: xdr.ScVal[]) =>
      contract.call(method, ...args),
  };
}

/**
 * Build a Soroban contract invocation operation.
 */
export function buildContractCall(opts: {
  contractAddress: string;
  method: string;
  args: xdr.ScVal[];
}): xdr.Operation {
  const contract = new Contract(opts.contractAddress);
  return new xdr.Operation({
    body: xdr.OperationBody.invokeHostFunction(
      new xdr.InvokeHostFunctionOp({
        hostFunction: new xdr.HostFunction.hostFunctionTypeInvoke(
          new xdr.InvokeContractArgs({
            contractAddress: Address.fromString(opts.contractAddress).toScAddress(),
            function: new xdr.ScSymbol(opts.method),
            args: opts.args,
          })
        ),
        auth: [],
      })
    ),
    sourceAccount: undefined,
    muxedAccount: undefined,
    muxedAccountMed25519: undefined,
  });
}

/**
 * Get the contract ID from a deployed contract address.
 */
export function contractId(address: string): Address {
  return Address.fromString(address);
}
