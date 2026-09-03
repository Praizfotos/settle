#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String};
    use crate::types::*;
    use crate::errors::*;
    use crate::milestone::MilestoneContract;

    fn setup() -> (Env, Address) {
        let env = Env::default();
        let creator = Address::random(&env);
        (env, creator)
    }

    #[test]
    fn test_create_milestone() {
        let (env, creator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "milestone-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let name = String::from_str(&env, "First Milestone");
        let description = String::from_str(&env, "Complete initial setup");
        let amount = 500000;
        let due_date = env.ledger().timestamp() + 86400;

        let result = MilestoneContract::create_milestone(
            env.clone(),
            id.clone(),
            agreement_id.clone(),
            creator.clone(),
            name,
            description,
            amount,
            due_date,
        );

        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.id, id);
        assert_eq!(milestone.agreement_id, agreement_id);
        assert_eq!(milestone.creator, creator);
        assert_eq!(milestone.amount, amount);
        assert_eq!(milestone.status, MilestoneStatus::Pending);
    }

    #[test]
    fn test_submit_milestone() {
        let (env, creator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "milestone-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let name = String::from_str(&env, "First Milestone");
        let description = String::from_str(&env, "Complete initial setup");
        let amount = 500000;
        let due_date = env.ledger().timestamp() + 86400;

        // Create milestone
        MilestoneContract::create_milestone(
            env.clone(),
            id.clone(),
            agreement_id,
            creator.clone(),
            name,
            description,
            amount,
            due_date,
        ).unwrap();

        // Submit milestone
        let submitter = Address::random(&env);
        let evidence = String::from_str(&env, "https://evidence.example.com");

        let result = MilestoneContract::submit_milestone(
            env.clone(),
            id.clone(),
            submitter,
            evidence,
        );

        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.status, MilestoneStatus::Submitted);
    }

    #[test]
    fn test_approve_milestone() {
        let (env, creator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "milestone-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let name = String::from_str(&env, "First Milestone");
        let description = String::from_str(&env, "Complete initial setup");
        let amount = 500000;
        let due_date = env.ledger().timestamp() + 86400;

        // Create milestone
        MilestoneContract::create_milestone(
            env.clone(),
            id.clone(),
            agreement_id,
            creator,
            name,
            description,
            amount,
            due_date,
        ).unwrap();

        // Submit milestone
        let submitter = Address::random(&env);
        let evidence = String::from_str(&env, "https://evidence.example.com");
        MilestoneContract::submit_milestone(
            env.clone(),
            id.clone(),
            submitter,
            evidence,
        ).unwrap();

        // Approve milestone
        let approver = Address::random(&env);
        let result = MilestoneContract::approve_milestone(
            env.clone(),
            id.clone(),
            approver,
        );

        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.status, MilestoneStatus::Approved);
    }

    #[test]
    fn test_reject_milestone() {
        let (env, creator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "milestone-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let name = String::from_str(&env, "First Milestone");
        let description = String::from_str(&env, "Complete initial setup");
        let amount = 500000;
        let due_date = env.ledger().timestamp() + 86400;

        // Create milestone
        MilestoneContract::create_milestone(
            env.clone(),
            id.clone(),
            agreement_id,
            creator,
            name,
            description,
            amount,
            due_date,
        ).unwrap();

        // Submit milestone
        let submitter = Address::random(&env);
        let evidence = String::from_str(&env, "https://evidence.example.com");
        MilestoneContract::submit_milestone(
            env.clone(),
            id.clone(),
            submitter,
            evidence,
        ).unwrap();

        // Reject milestone
        let rejecter = Address::random(&env);
        let reason = String::from_str(&env, "Insufficient evidence");

        let result = MilestoneContract::reject_milestone(
            env.clone(),
            id.clone(),
            rejecter,
            reason,
        );

        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.status, MilestoneStatus::Rejected);
    }

    #[test]
    fn test_release_milestone_payment() {
        let (env, creator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "milestone-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let name = String::from_str(&env, "First Milestone");
        let description = String::from_str(&env, "Complete initial setup");
        let amount = 500000;
        let due_date = env.ledger().timestamp() + 86400;

        // Create milestone
        MilestoneContract::create_milestone(
            env.clone(),
            id.clone(),
            agreement_id,
            creator,
            name,
            description,
            amount,
            due_date,
        ).unwrap();

        // Submit milestone
        let submitter = Address::random(&env);
        let evidence = String::from_str(&env, "https://evidence.example.com");
        MilestoneContract::submit_milestone(
            env.clone(),
            id.clone(),
            submitter,
            evidence,
        ).unwrap();

        // Approve milestone
        let approver = Address::random(&env);
        MilestoneContract::approve_milestone(
            env.clone(),
            id.clone(),
            approver,
        ).unwrap();

        // Release payment
        let releaser = Address::random(&env);
        let recipient = Address::random(&env);

        let result = MilestoneContract::release_milestone_payment(
            env.clone(),
            id.clone(),
            releaser,
            recipient,
        );

        assert!(result.is_ok());
        let milestone = result.unwrap();
        assert_eq!(milestone.status, MilestoneStatus::Released);
    }

    #[test]
    fn test_get_milestone() {
        let (env, creator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "milestone-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let name = String::from_str(&env, "First Milestone");
        let description = String::from_str(&env, "Complete initial setup");
        let amount = 500000;
        let due_date = env.ledger().timestamp() + 86400;

        // Create milestone
        MilestoneContract::create_milestone(
            env.clone(),
            id.clone(),
            agreement_id,
            creator,
            name,
            description,
            amount,
            due_date,
        ).unwrap();

        // Get milestone
        let result = MilestoneContract::get_milestone(env.clone(), id.clone());

        assert!(result.is_some());
        let milestone = result.unwrap();
        assert_eq!(milestone.id, id);
    }

    #[test]
    fn test_get_nonexistent_milestone() {
        let env = Env::default();
        let id = String::from_str(&env, "nonexistent");

        let result = MilestoneContract::get_milestone(env, id);

        assert!(result.is_none());
    }
}