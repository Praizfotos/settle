# Contributing to Settle

Thank you for your interest in contributing to Settle! This document provides guidelines and information for contributors.

## Prerequisites

- Rust 1.70+ with `wasm32-unknown-unknown` target
- Node.js 18+
- Docker (for local development)
- Stellar CLI (for contract deployment)

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/Praizfotos/settle.git
   cd Settle
   ```

2. Install dependencies:
   ```bash
   npm install
   cargo build
   ```

3. Build contracts:
   ```bash
   npm run contracts:build
   ```

4. Start development servers:
   ```bash
   npm run dev
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

## Development Workflow

### Smart Contracts

1. Navigate to `contracts/stellar-contract/`
2. Make changes to Rust files
3. Run tests: `cargo test`
4. Build for WASM: `npm run contracts:build`

### Backend API

1. Navigate to `backend/`
2. Make changes to Rust files
3. Run tests: `cargo test`
4. Start development server: `npm run backend:dev`

### Frontend

1. Navigate to `apps/web/`
2. Make changes to TypeScript/React files
3. Run tests: `npm test`
4. Start development server: `cd apps/web && npm run dev`

### SDK

1. Navigate to `packages/sdk/`
2. Make changes to TypeScript files
3. Run tests: `npm test`

## Testing

### Run All Tests

```bash
npm run test
cargo test
```

### Run Specific Tests

```bash
# Contract tests
cargo test --manifest-path contracts/stellar-contract/Cargo.toml

# Backend tests
cargo test --manifest-path backend/Cargo.toml

# Frontend tests
cd apps/web && npm test

# Integration tests
npm run integration:test
```

## Code Style

### Rust

- Follow standard Rust formatting
- Use `cargo fmt` to format code
- Use `cargo clippy` for linting

### TypeScript

- Use Prettier for formatting
- Follow ESLint rules
- Use TypeScript strict mode

## Branch Naming

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `test/description` - Test additions or improvements

## Commit Messages

Use conventional commits:

- `feat: add new agreement feature`
- `fix: resolve escrow release issue`
- `docs: update architecture documentation`
- `test: add contract authorization tests`

## Pull Requests

1. Create a feature branch from `main`
2. Make your changes
3. Add tests if applicable
4. Ensure all tests pass
5. Submit a pull request

## Contract Development

### State Transitions

Ensure all state transitions are:
- Explicitly defined
- Properly validated
- Tested for valid and invalid cases

### Authorization

Every privileged operation must have:
- Clear authorization checks
- Tests for authorized and unauthorized access

### Events

All important state changes should emit events for:
- Indexer integration
- Audit trail
- Real-time updates

## Indexer Development

### Event Processing

- Ensure idempotency
- Handle duplicate events
- Implement proper error handling

### Database Operations

- Use transactions where required
- Implement proper error handling
- Ensure data consistency

## Issue Workflow

1. Find an issue you'd like to work on
2. Comment on the issue to indicate you're working on it
3. Create a branch for your work
4. Make changes and submit a PR
5. Address any review feedback
6. Your contribution will be merged once approved

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in existing issues
- Review documentation in `docs/`

## Code of Conduct

Please be respectful and constructive in all interactions. We're building a welcoming community for open-source contributors.

## License

By contributing to Settle, you agree that your contributions will be licensed under the MIT License.