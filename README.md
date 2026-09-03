# Settle

**Programmable Settlement Infrastructure on Stellar**

Settle is a comprehensive settlement platform that enables secure, programmable agreements with milestone-based payments, escrow protection, and dispute resolution. Built on Stellar's Soroban smart contracts.

## Architecture

```
                           SETTLE
              Programmable Settlement Infrastructure
                              │
             ┌────────────────┴────────────────┐
             │                                 │
          CLIENTS                          DEVELOPERS
             │                                 │
        Next.js App                       Settle SDK
             │                                 │
             └────────────────┬────────────────┘
                              ↓
                         SETTLE API
                              │
                 ┌────────────┼────────────┐
                 ↓            ↓            ↓
             Agreements    Indexer     Reputation
                 │            │            │
                 └────────────┼────────────┘
                              ↓
                       STELLAR / SOROBAN
                              │
       ┌────────────┬─────────┼─────────┬────────────┐
       ↓            ↓         ↓         ↓            ↓
   Agreement      Escrow   Milestone  Dispute    Settlement
   Contract      Contract   Contract  Contract    Logic
```

## Repository Structure

```
settle/
├── contracts/stellar-contract/    # Soroban smart contracts
├── backend/                       # Rust API server and indexer
├── packages/                      # Shared TypeScript packages
│   ├── sdk/                      # Settle SDK
│   ├── types/                    # Shared type definitions
│   └── stellar/                  # Stellar client utilities
├── apps/
│   └── web/                      # Next.js frontend application
├── integration-tests/             # End-to-end test suite
├── docs/                         # Comprehensive documentation
├── Cargo.toml                    # Rust workspace configuration
└── package.json                  # TypeScript workspace configuration
```

## Core Concepts

### Agreements
Programmable contracts between parties with defined terms, milestones, and payment schedules.

### Escrow
Secure holding of funds with automated release upon milestone completion or dispute resolution.

### Milestones
Trackable progress markers that trigger payment releases and state transitions.

### Disputes
Structured resolution process for disagreements with evidence submission and arbitration.

### Settlement
Final resolution of agreements with complete audit trail and reputation updates.

## Quick Start

### Prerequisites
- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+
- Docker (for local development)
- Stellar CLI (for contract deployment)

### Installation

```bash
# Clone repository
git clone https://github.com/Praizfotos1/Settle.git
cd Settle

# Install dependencies
npm install
cargo build

# Build contracts
npm run contracts:build

# Start development servers
npm run dev
```

### Development

```bash
# Run all tests
npm run test
cargo test

# Start backend API
npm run backend:dev

# Start frontend
cd apps/web && npm run dev

# Run integration tests
npm run integration:test
```

## Smart Contract Architecture

Modular smart contract architecture with domain-specific contracts:

- **Agreement Contract**: Core agreement logic and state management
- **Escrow Contract**: Secure fund management with automated releases
- **Milestone Contract**: Progress tracking and validation
- **Dispute Contract**: Structured resolution process

Each contract follows a layered architecture:
- Contract Entry / Dispatcher
- Domain Management
- Domain Logic + Validation
- Storage / Optimization
- Events / Audit
- Security / Upgrades

## API Architecture

The backend provides:
- RESTful API for agreement management
- Real-time event indexing from Stellar
- WebSocket connections for live updates
- Comprehensive query interface
- Reputation scoring system

## SDK and Integration

The Settle SDK provides:
- TypeScript/JavaScript client library
- React hooks and components
- Stellar wallet integration
- Type-safe contract interactions
- Real-time event subscriptions

## Documentation

- [Architecture Guide](./docs/architecture/)
- [Smart Contracts](./docs/contracts/)
- [API Reference](./docs/api/)
- [SDK Documentation](./docs/sdk/)
- [Integration Guide](./docs/integration/)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Submit pull request

## License

MIT License - see [LICENSE](LICENSE) for details.