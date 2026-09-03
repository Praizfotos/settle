# Add Agreement State Transition Validation

## Problem
The agreement contract currently doesn't validate state transitions properly. Some invalid transitions may be allowed, which could lead to inconsistent state.

## Context
Settle's agreement lifecycle follows a specific state machine:
- Draft → Funded → Active → Completed/Cancelled

Invalid transitions like going from Completed back to Active should be prevented.

## Scope
- Add explicit state transition validation in `agreement.rs`
- Define valid transitions in a state machine pattern
- Add tests for all valid and invalid transitions

## Implementation Guidance

1. Define valid transitions:
```rust
fn is_valid_transition(from: AgreementStatus, to: AgreementStatus) -> bool {
    match (from, to) {
        (AgreementStatus::Draft, AgreementStatus::Funded) => true,
        (AgreementStatus::Funded, AgreementStatus::Active) => true,
        (AgreementStatus::Active, AgreementStatus::Completed) => true,
        (AgreementStatus::Draft, AgreementStatus::Cancelled) => true,
        (AgreementStatus::Funded, AgreementStatus::Cancelled) => true,
        (AgreementStatus::Active, AgreementStatus::Cancelled) => true,
        _ => false,
    }
}
```

2. Add validation before each state change
3. Return `SettleError::InvalidStateTransition` for invalid transitions
4. Add comprehensive tests

## Acceptance Criteria
- [ ] All valid transitions are allowed
- [ ] All invalid transitions are rejected with appropriate error
- [ ] Tests cover every state transition
- [ ] Documentation updated to reflect state machine

## Files/Components Affected
- `contracts/stellar-contract/src/agreement.rs`
- `contracts/stellar-contract/src/types.rs`
- `contracts/stellar-contract/src/errors.rs`
- `contracts/stellar-contract/src/agreement_tests.rs`

## Testing Requirements
- Unit tests for each valid transition
- Unit tests for each invalid transition
- Integration tests for complete lifecycle

## Dependencies
None

## Difficulty
Medium