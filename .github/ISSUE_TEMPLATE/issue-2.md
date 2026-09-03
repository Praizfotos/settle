# Add Escrow Authorization Tests

## Problem
The escrow contract has authorization checks but lacks comprehensive tests to verify that unauthorized users cannot perform privileged operations.

## Context
Escrow operations like locking, releasing, and refunding should only be performed by authorized parties (typically the agreement creator or counterparty).

## Scope
- Add tests for authorized operations
- Add tests for unauthorized operations
- Verify authorization checks are working correctly

## Implementation Guidance

1. Test that the agreement creator can lock escrow
2. Test that random users cannot lock escrow
3. Test that only authorized users can release funds
4. Test that only authorized users can refund funds
5. Use `env.mock_all_auths()` appropriately

## Acceptance Criteria
- [ ] Tests for authorized lock operations
- [ ] Tests for unauthorized lock operations
- [ ] Tests for authorized release operations
- [ ] Tests for unauthorized release operations
- [ ] Tests for authorized refund operations
- [ ] Tests for unauthorized refund operations

## Files/Components Affected
- `contracts/stellar-contract/src/escrow_tests.rs`
- `contracts/stellar-contract/src/escrow.rs`

## Testing Requirements
- Unit tests for authorization checks
- Integration tests with actual auth contexts

## Dependencies
None

## Difficulty
Medium