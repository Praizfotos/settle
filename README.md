# Settle

**Programmable Settlement Infrastructure on Stellar/Soroban**

Settle is a programmable settlement protocol built natively on Stellar and Soroban smart contracts. It enables secure agreements with milestone-based payments, escrow protection, and dispute resolution — all enforced on-chain.

## Why Stellar?

Stellar provides the ideal foundation for programmable settlement:

- **Low fees**: Sub-cent transaction costs make micropayments viable
- **Fast finality**: 3-5 second consensus means near-instant settlement
- **Soroban smart contracts**: Turing-complete contract runtime for complex settlement logic
- **Built-in token ecosystem**: Native asset support and AMM for token liquidity
- **Freighter wallet**: Mature browser wallet for seamless UX
- **Event system**: Contract events enable real-time off-chain indexing

Settle is not "Stellar as a payment rail." The entire protocol — agreements, escrow, milestones, disputes, and reputation — lives in Soroban contracts on Stellar.

## Current Status

**Testnet development.** Contracts are built and the backend/indexer are functional. The frontend reads from the backend API. The on-chain transaction flow (wallet signing -> contract invocation -> confirmation) is in progress.

### What works now

- Soroban smart contracts (agreement, escrow, milestone, dispute)
- Rust/Axum backend API with PostgreSQL
- Event indexer polling Stellar RPC for contract events
- Next.js frontend with Freighter wallet connection
- Dashboard showing real backend data
- Agreement draft creation (off-chain)
- SDK with contract read/write methods
- Domain model with state machines

### In progress

- Frontend SDK integration (building and signing Soroban transactions)
- Complete on-chain transaction flow (create, fund, activate, complete)
- Escrow operations via frontend
- Dispute operations via frontend
- Real-time transaction status UX

## Architecture

```
USER
 ↓
Next.js (apps/web)
 ↓
@settle/sdk (packages/sdk)
 ↓
Freighter (browser wallet)
 ↓
Stellar Network (Soroban Contracts)
 ↓
Contract Events
 ↓
Indexer (backend/indexer)
 ↓
PostgreSQL
 ↓
Rust API (backend/api)
 ↓
Next.js (frontend reads)
```

The protocol is Stellar-native: all state transitions happen in Soroban contracts. The backend indexes contract events to provide efficient queries. The frontend uses the SDK to build Soroban transactions, which Freighter signs and submits.

## Repository Structure

```
settle/
├── contracts/stellar-contract/    # Soroban smart contracts (Rust)
├── backend/                       # Rust API server + event indexer
│   ├── src/api/                   # REST API endpoints
│   ├── src/indexer/               # Event polling, decoding, processing
│   ├── src/stellar/               # Stellar RPC client, XDR construction
│   └── src/services/              # Business logic layer
├── packages/                      # Shared TypeScript packages
│   ├── sdk/                       # @settle/sdk — Soroban contract client
│   ├── types/                     # @settle/types — shared type definitions
│   └── stellar/                   # @settle/stellar — network utilities
├── apps/
│   └── web/                       # Next.js frontend (Freighter integration)
├── docs/                          # Architecture and protocol documentation
├── Cargo.toml                     # Rust workspace
└── package.json                   # TypeScript workspace
```

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Smart contracts | Soroban (Rust) on Stellar |
| Backend | Rust, Axum, SQLx, PostgreSQL |
| Indexer | Rust, Stellar JSON-RPC, event polling |
| SDK | TypeScript, @stellar/stellar-sdk |
| Frontend | Next.js, React, Tailwind CSS |
| Wallet | Freighter (Stellar browser extension) |
| Types | TypeScript, shared @settle/types package |

## Quick Start

### Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+
- PostgreSQL 14+
- [Freighter](https://freighter.app/) browser extension (for wallet)

### 1. Clone and install

```bash
git clone https://github.com/Praizfotos/settle.git
cd settle
npm install
```

### 2. Configure environment

```bash
# Copy example env files
cp .env.example .env
cp apps/web/.env.example apps/web/.env.local

# Edit .env with your PostgreSQL connection string and Stellar config
# Default: postgresql://settle:settle@localhost:5432/settle
# Default: STELLAR_RPC_URL=https://soroban-testnet.stellar.org
```

### 3. Set up database

```bash
# Create the database
createdb settle

# Run migrations (if using sqlx)
cargo sqlx migrate run
```

### 4. Build contracts

```bash
cd contracts/stellar-contract
cargo build --target wasm32-unknown-unknown --release
```

### 5. Start development

```bash
# Terminal 1: Backend API + Indexer
cd backend
cargo run

# Terminal 2: Frontend
cd apps/web
npm run dev
```

Open [http://localhost:3001](http://localhost:3001) to see the landing page.

## Environment Variables

### Backend

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://settle:settle@localhost:5432/settle` |
| `STELLAR_NETWORK` | `testnet` or `mainnet` | `testnet` |
| `STELLAR_RPC_URL` | Soroban RPC endpoint | `https://soroban-testnet.stellar.org` |
| `STELLAR_CONTRACT_ADDRESS` | Deployed contract address | (required) |
| `STELLAR_SECRET_KEY` | Backend signing key (optional) | (none) |
| `SERVER_PORT` | API server port | `3000` |

### Frontend

| Variable | Description | Default |
|----------|-------------|---------|
| `NEXT_PUBLIC_STELLAR_NETWORK` | `testnet` or `mainnet` | `testnet` |
| `NEXT_PUBLIC_STELLAR_RPC_URL` | Soroban RPC endpoint | `https://soroban-testnet.stellar.org` |
| `NEXT_PUBLIC_STELLAR_CONTRACT_ADDRESS` | Deployed contract address | (required) |
| `NEXT_PUBLIC_API_URL` | Backend API URL | `http://localhost:3000` |

## Smart Contracts

The Soroban contracts are in `contracts/stellar-contract/` and implement:

- **AgreementContract**: Create, fund, activate, complete, cancel agreements
- **EscrowContract**: Create, fund, lock, release, refund escrow
- **MilestoneContract**: Create, submit, approve, reject, release milestones
- **DisputeContract**: Open disputes, submit evidence, resolve, close

Each contract emits structured events that the backend indexer processes into PostgreSQL.

## API

The Rust backend provides a REST API:

- `GET /api/v1/agreements` — List agreements
- `POST /api/v1/agreements` — Create agreement draft
- `GET /api/v1/agreements/:id` — Get agreement details
- `GET /api/v1/agreements/:id/milestones` — List milestones
- `GET /api/v1/disputes` — List disputes
- `GET /api/v1/reputation/:address` — Get reputation score

On-chain write operations (fund, activate, complete) are in progress and currently return `501 Not Implemented`.

## SDK

The `@settle/sdk` provides TypeScript methods for all contract operations:

```typescript
import { SettleClient } from "@settle/sdk";

const client = SettleClient.fromEnv();

// Build a Soroban transaction (returns XDR for wallet signing)
const { txXdr } = await client.agreements.buildCreate({
  counterparty: "G...",
  token: "C...",
  totalAmount: "1000000000", // 100 XLM in stroops
  expiresAt: BigInt(Math.floor(Date.now() / 1000) + 86400),
}, sourceAccount);

// Sign with Freighter
const signedXdr = await signTransaction(txXdr);

// Submit to Stellar network
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## Security

See [SECURITY.md](SECURITY.md) for the security policy.

## License

MIT License — see [LICENSE](LICENSE) for details.
