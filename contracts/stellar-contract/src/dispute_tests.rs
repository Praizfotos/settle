#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String};
    use crate::types::*;
    use crate::errors::*;
    use crate::dispute::DisputeContract;

    fn setup() -> (Env, Address, Address) {
        let env = Env::default();
        let opener = Address::random(&env);
        let arbitrator = Address::random(&env);
        (env, opener, arbitrator)
    }

    #[test]
    fn test_open_dispute() {
        let (env, opener, _) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "dispute-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");

        let result = DisputeContract::open_dispute(
            env.clone(),
            id.clone(),
            agreement_id.clone(),
            opener.clone(),
            reason,
            initial_evidence,
        );

        assert!(result.is_ok());
        let dispute = result.unwrap();
        assert_eq!(dispute.id, id);
        assert_eq!(dispute.agreement_id, agreement_id);
        assert_eq!(dispute.opener, opener);
        assert_eq!(dispute.status, DisputeStatus::Open);
    }

    #[test]
    fn test_submit_evidence() {
        let (env, opener, _) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "dispute-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");

        // Open dispute
        DisputeContract::open_dispute(
            env.clone(),
            id.clone(),
            agreement_id,
            opener,
            reason,
            initial_evidence,
        ).unwrap();

        // Submit additional evidence
        let submitter = Address::random(&env);
        let evidence = String::from_str(&env, "https://additional-evidence.example.com");

        let result = DisputeContract::submit_evidence(
            env.clone(),
            id.clone(),
            submitter,
            evidence,
        );

        assert!(result.is_ok());
        let dispute = result.unwrap();
        assert_eq!(dispute.status, DisputeStatus::EvidenceSubmission);
    }

    #[test]
    fn test_resolve_dispute() {
        let (env, opener, arbitrator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "dispute-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");

        // Open dispute
        DisputeContract::open_dispute(
            env.clone(),
            id.clone(),
            agreement_id,
            opener.clone(),
            reason,
            initial_evidence,
        ).unwrap();

        // Submit evidence
        let submitter = Address::random(&env);
        let evidence = String::from_str(&env, "https://additional-evidence.example.com");
        DisputeContract::submit_evidence(
            env.clone(),
            id.clone(),
            submitter,
            evidence,
        ).unwrap();

        // Resolve dispute
        let resolution = String::from_str(&env, "Dispute resolved in favor of opener");
        let winner = opener;
        let compensation_amount = 500000;

        let result = DisputeContract::resolve_dispute(
            env.clone(),
            id.clone(),
            arbitrator,
            resolution,
            winner,
            compensation_amount,
        );

        assert!(result.is_ok());
        let dispute = result.unwrap();
        assert_eq!(dispute.status, DisputeStatus::Resolved);
    }

    #[test]
    fn test_close_dispute() {
        let (env, opener, arbitrator) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "dispute-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");

        // Open dispute
        DisputeContract::open_dispute(
            env.clone(),
            id.clone(),
            agreement_id,
            opener,
            reason,
            initial_evidence,
        ).unwrap();

        // Submit evidence
        let submitter = Address::random(&env);
        let evidence = String::from_str(&env, "https://additional-evidence.example.com");
        DisputeContract::submit_evidence(
            env.clone(),
            id.clone(),
            submitter,
            evidence,
        ).unwrap();

        // Resolve dispute
        let arbitrator_addr = Address::random(&env);
        let resolution = String::from_str(&env, "Dispute resolved in favor of opener");
        let winner = Address::random(&env);
        let compensation_amount = 500000;
        DisputeContract::resolve_dispute(
            env.clone(),
            id.clone(),
            arbitrator_addr,
            resolution,
            winner,
            compensation_amount,
        ).unwrap();

        // Close dispute
        let closer = Address::random(&env);
        let result = DisputeContract::close_dispute(
            env.clone(),
            id.clone(),
            closer,
        );

        assert!(result.is_ok());
        let dispute = result.unwrap();
        assert_eq!(dispute.status, DisputeStatus::Closed);
    }

    #[test]
    fn test_get_dispute() {
        let (env, opener, _) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "dispute-1");
        let agreement_id = String::from_str(&env, "agreement-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");

        // Open dispute
        DisputeContract::open_dispute(
            env.clone(),
            id.clone(),
            agreement_id,
            opener,
            reason,
            initial_evidence,
        ).unwrap();

        // Get dispute
        let result = DisputeContract::get_dispute(env.clone(), id.clone());

        assert!(result.is_some());
        let dispute = result.unwrap();
        assert_eq!(dispute.id, id);
    }

    #[test]
    fn test_get_nonexistent_dispute() {
        let env = Env::default();
        let id = String::from_str(&env, "nonexistent");

        let result = DisputeContract::get_dispute(env, id);

        assert!(result.is_none());
    }

    #[test]
    fn test_has_open_dispute() {
        let (env, opener, _) = setup();
        env.mock_all_auths();

        let agreement_id = String::from_str(&env, "agreement-1");

        // Check for open dispute (should be false)
        let result = DisputeContract::has_open_dispute(env.clone(), agreement_id.clone());
        assert!(!result);

        // Open dispute
        let id = String::from_str(&env, "dispute-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");
        DisputeContract::open_dispute(
            env.clone(),
            id,
            agreement_id.clone(),
            opener,
            reason,
            initial_evidence,
        ).unwrap();

        // Check for open dispute (should be true)
        let result = DisputeContract::has_open_dispute(env, agreement_id);
        assert!(result);
    }
}