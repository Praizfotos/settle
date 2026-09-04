# Settle Architecture

This document describes the system architecture of Settle, a programmable settlement protocol built on Stellar/Soroban.

## System Overview

Settle is a **Stellar-native** protocol. All core state lives in Soroban smart contracts on the Stellar network. The backend indexes contract events to provide efficient queries. The frontend uses the SDK to build Soroban transactions, which the user's Freighter wallet signs and submits.

```
┌─────────────────────────────────────────────────────────┐
│                    USER INTERFACE                        │
│                                                         │
│  Landing Page ─── Dashboard ─── Agreements ─── Settings  │
│       │              │              │                   │
│       └──────────────┴──────────────┘                   │
│                      │                                  │
│               Freighter Wallet                           │
│          (signs Soroban transactions)                   │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                   @settle/sdk                            │
│                                                         │
│  AgreementsModule  MilestonesModule  EscrowModule        │
│  DisputesModule    ReputationModule                      │
│                                                         │
│  Builds Soroban XDR transactions                        │
│  Decodes contract simulation results                    │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│              STELLAR / SOROBAN                           │
│                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐ │
│  │  Agreement   │  │   Escrow    │  │   Milestone     │ │
│  │  Contract    │  │  Contract   │  │   Contract      │ │
│  └─────────────┘  └─────────────┘  └─────────────────┘ │
│  ┌─────────────┐  ┌─────────────────────────────────┐   │
│  │  Dispute    │  │  EventBuilder (20 event types)  │   │
│  │  Contract   │  │  Emits structured XDR events    │   │
│  └─────────────┘  └─────────────────────────────────┘   │
└──────────────────────┬──────────────────────────────────┘
                       │
          Contract events emitted on-chain
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                   INDEXER                               │
│                                                         │
│  StellarClient.getEvents()                              │
│       │                                                 │
│       ▼                                                 │
│  decoder.rs (XDR → SettleEvent enum)                    │
│       │                                                 │
│       ▼                                                 │
│  processor.rs (SettleEvent → PostgreSQL writes)          │
│       │                                                 │
│       ▼                                                 │
│  Cursor persistence (idempotent restart)                │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                  PostgreSQL                              │
│                                                         │
│  agreements │ milestones │ disputes │ escrow             │
│  settlement_events (audit trail)                        │
│  indexer_cursor (restart position)                       │
└──────────────────────┬──────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────┐
│                  RUST API (Axum)                         │
│                                                         │
│  GET /api/v1/agreements                                 │
│  GET /api/v1/agreements/:id                             │
│  GET /api/v1/agreements/:id/milestones                  │
│  GET /api/v1/disputes                                   │
│  GET /api/v1/reputation/:address                        │
│  POST /api/v1/agreements (draft creation)               │
└─────────────────────────────────────────────────────────┘
```

## Data Flow

### Command Path (Write)

```
1. User fills form in Next.js
2. Frontend calls SDK method (e.g., agreements.buildCreate)
3. SDK constructs Soroban InvokeHostFunction XDR
4. XDR passed to Freighter for user signing
5. Signed XDR submitted to Stellar RPC
6. Transaction confirmed on-chain
7. Contract emits structured event
```

### Query Path (Read)

```
1. Indexer polls Stellar RPC for new events
2. Events decoded from XDR (topics + data)
3. Decoded events processed into PostgreSQL
4. Frontend queries backend REST API
5. Backend reads from PostgreSQL projections
6. Frontend renders real on-chain data
```

## Signing Model

Settle uses a **non-custodial** signing model:

- **User-initiated writes**: The frontend builds Soroban XDR, passes it to Freighter for signing. Private keys never leave the user's wallet.
- **Backend-initiated reads**: The indexer uses RPC `simulateTransaction` with a dummy account for read-only contract queries. No signing required.
- **Backend writes**: Reserved for future automated operations. Would use `STELLAR_SECRET_KEY` env var. Currently unused.

## Security Model

- All privileged contract operations call `require_auth()` on the relevant address
- State transitions are validated by the contract before acceptance
- The backend API is read-only for contract state (projections from indexed events)
- No private keys are stored in the repository or frontend
- Environment variables are used for all configuration
- The `.gitignore` excludes `.env` files

## Event System

The contract emits 20 event types via `EventBuilder`:

| Domain | Events |
|--------|--------|
| Agreement | Created, Funded, Activated, Completed, Expired, Cancelled |
| Milestone | Created, Submitted, Approved, Rejected, Released |
| Escrow | Funded, Locked, Released, Refunded |
| Dispute | Opened, EvidenceSubmitted, Resolved, Closed |
| Reputation | Updated |

Each event contains:
- Topics: `["settle_event", EventType, participant]`
- Data: SettleEvent struct with amounts, IDs, timestamps (little-endian encoded)

The indexer decodes events using `ScVal` XDR parsing and projects them into PostgreSQL tables.

## Indexer Reliability

- **Cursor persistence**: Last processed ledger stored in `indexer_cursor` table
- **At-least-once delivery**: On restart, events from the last committed cursor are reprocessed
- **Idempotent writes**: Most database operations use `ON CONFLICT DO UPDATE`
- **Audit trail**: All events stored in `settlement_events` with `ON CONFLICT DO NOTHING`
- **Unknown events**: Logged and skipped, cursor advances

## Environment

See [README.md](../README.md#environment-variables) for the complete list of environment variables.

### Local Development

```bash
# Backend: Rust API + Indexer
DATABASE_URL=postgresql://settle:settle@localhost:5432/settle
STELLAR_NETWORK=testnet
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_CONTRACT_ADDRESS=<deployed contract>

# Frontend: Next.js
NEXT_PUBLIC_STELLAR_NETWORK=testnet
NEXT_PUBLIC_STELLAR_RPC_URL=https://soroban-testnet.stellar.org
NEXT_PUBLIC_STELLAR_CONTRACT_ADDRESS=<deployed contract>
NEXT_PUBLIC_API_URL=http://localhost:3000
```

### Testnet Deployment

Contracts are deployed to Stellar Testnet. The default RPC URL is `https://soroban-testnet.stellar.org`. The default network passphrase is `Test SDF Network ; September 2015`.

## Troubleshooting

### Backend won't start

- Ensure PostgreSQL is running and `DATABASE_URL` is correct
- Run `cargo sqlx migrate run` to apply migrations
- Check that `STELLAR_CONTRACT_ADDRESS` is set

### Frontend can't connect to wallet

- Install the [Freighter](https://freighter.app/) browser extension
- Ensure Freighter is set to Testnet
- Refresh the page after installing

### Indexer not picking up events

- Check that `STELLAR_RPC_URL` is accessible
- Verify `STELLAR_CONTRACT_ADDRESS` matches the deployed contract
- Check backend logs for RPC errors
