# Add Milestone Release Event Indexing

## Problem
The backend indexer doesn't currently handle milestone release events from the Stellar network. This means milestone payments aren't being tracked in the database.

## Context
When a milestone is approved and payment is released, the contract emits a `MilestoneReleased` event. The indexer should capture this event and update the database accordingly.

## Scope
- Add event handler for `MilestoneReleased` events
- Update milestone status in database
- Add proper error handling for malformed events

## Implementation Guidance

1. Add event type to indexer event decoder
2. Create handler function for milestone release events
3. Update milestone record in database
4. Add idempotency checks
5. Handle edge cases (event replay, duplicate events)

## Acceptance Criteria
- [ ] `MilestoneReleased` events are captured
- [ ] Milestone status is updated in database
- [ ] Event processing is idempotent
- [ ] Malformed events are handled gracefully
- [ ] Tests for event processing

## Files/Components Affected
- `backend/src/indexer/`
- `backend/src/domain/`
- `backend/src/database/`

## Testing Requirements
- Unit tests for event decoding
- Integration tests for event processing
- Tests for duplicate event handling

## Dependencies
None

## Difficulty
High