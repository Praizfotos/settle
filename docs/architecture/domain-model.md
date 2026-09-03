# Settle Domain Model

Settle's architecture employs clean separation of domain concerns centered around programmable settlement agreements.

## Core Domain Hierarchy

```text
Agreement (Central Domain Object)
   │
   ├── Milestones
   │   └── Progress tracking and payment triggers
   │
   ├── Escrow
   │   └── Secure fund management
   │
   ├── Dispute
   │   └── Conflict resolution process
   │
   └── Participants
       └── Reputation and activity tracking
```

## Domain State Machines

### Agreement State Machine

The Agreement is the central domain primitive that drives the entire protocol:

```text
DRAFT
  ↓ (fund_agreement)
FUNDED
  ↓ (activate_agreement)
ACTIVE
  ↓ (complete_agreement OR dispute timeout)
 ┌───────────────┐
 ↓               ↓
COMPLETED     DISPUTED
                 ↓ (resolve_dispute)
          ┌──────┴──────┐
          ↓             ↓
       RESOLVED      CANCELLED
          ↓
     (auto-expire)
        ↓
      EXPIRED
```

**Key Properties:**
- `Agreement` contains total amounts, parties, timeline, and milestone references
- State transitions are enforced by the contract
- Each Agreement has exactly one Escrow and can have multiple Milestones
- Disputes can be opened on any Agreement in ACTIVE or COMPLETED state

### Escrow State Machine

Escrow provides secure fund management with automated release triggers:

```text
EMPTY
  ↓ (fund_escrow)
FUNDED
  ↓ (lock_escrow) 
LOCKED
  ↓ (milestone approval OR dispute resolution)
 ┌───────────────┐
 ↓               ↓
RELEASED      REFUNDED
```

**Key Properties:**
- One Escrow per Agreement
- Tracks total, locked, and released amounts
- Automatic release upon milestone completion
- Refund capability for dispute resolutions

### Milestone State Machine

Milestones provide granular progress tracking and payment triggers:

```text
PENDING
  ↓ (submit_milestone)
SUBMITTED
  ↓ (approve_milestone OR reject_milestone)
 ┌───────────────┐
 ↓               ↓
APPROVED       REJECTED
  ↓              ↓
(release_milestone_payment)  (re-submit OR dispute)
  ↓
RELEASED
```

**Key Properties:**
- Multiple Milestones per Agreement
- Evidence submission and approval workflow
- Automatic escrow release upon approval
- Rejection triggers re-work or dispute process

### Dispute State Machine

Disputes provide structured conflict resolution:

```text
OPEN
  ↓ (submit_evidence)
EVIDENCE_SUBMISSION
  ↓ (automatic OR manual transition)
UNDER_REVIEW
  ↓ (resolve_dispute)
RESOLVED
  ↓ (close_dispute)
CLOSED
```

**Key Properties:**
- One Dispute per Agreement maximum
- Evidence collection from both parties
- Arbitrator-based resolution
- Compensation determination affecting escrow

## Domain Model Implementation

### Agreement (Central Object)

```rust
pub struct Agreement {
    pub id: String,
    pub creator: Address,
    pub counterparty: Address,
    pub token: Address,
    pub total_amount: i128,
    pub funded_amount: i128,
    pub released_amount: i128,
    pub refunded_amount: i128,
    pub status: AgreementStatus,
    pub created_at: u64,
    pub expires_at: u64,
    pub milestones: Vec<String>,  // Milestone IDs
}
```

**Invariants:**
- `funded_amount <= total_amount`
- `released_amount + refunded_amount <= funded_amount`
- `creator != counterparty`
- `expires_at > created_at`

### Domain Logic Separation

Each domain has separated concerns:

1. **Contract Entry/Dispatcher** (`lib.rs`)
   - Public API surface
   - Authorization checks
   - Cross-domain orchestration

2. **Domain Logic** (`agreement.rs`, `escrow.rs`, etc.)
   - Business rules enforcement
   - State transition validation
   - Domain-specific operations

3. **Storage/Persistence** (`storage.rs`)
   - Data serialization
   - Key generation
   - Indexing support

4. **Events/Audit** (`events.rs`)
   - Structured event emission
   - Indexer integration
   - Audit trail creation

5. **Validation** (`validation.rs`)
   - Input sanitization
   - State validation
   - Constraint enforcement

## Event-Driven Architecture

All domain state changes emit structured events:

```rust
pub enum EventType {
    // Agreement lifecycle
    AgreementCreated,
    AgreementFunded, 
    AgreementActivated,
    AgreementCompleted,
    
    // Milestone progress
    MilestoneSubmitted,
    MilestoneApproved,
    MilestoneReleased,
    
    // Escrow management
    EscrowLocked,
    EscrowReleased,
    EscrowRefunded,
    
    // Dispute resolution
    DisputeOpened,
    DisputeResolved,
    
    // Reputation updates
    ReputationUpdated,
}
```

This enables the backend indexer to maintain real-time views and provide rich queries without on-chain computation costs.

## Architecture Benefits

1. **Single Source of Truth**: Contract state is authoritative
2. **Event-Driven Updates**: Backend stays synchronized via events
3. **Domain Separation**: Clear boundaries between concerns
4. **State Machine Enforcement**: Invalid transitions are prevented
5. **Audit Trail**: Complete history via event log
6. **Query Optimization**: Backend provides efficient queries
7. **Real-time Updates**: WebSocket subscriptions via event processing

This domain model provides the foundation for a robust settlement infrastructure with engineering discipline while implementing Settle's unique protocol requirements.