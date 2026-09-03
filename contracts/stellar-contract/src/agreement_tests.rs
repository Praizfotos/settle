#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String, Vec};
    use crate::types::*;
    use crate::errors::*;
    use crate::agreement::AgreementContract;

    fn setup() -> (Env, Address, Address, Address) {
        let env = Env::default();
        let admin = Address::random(&env);
        let creator = Address::random(&env);
        let counterparty = Address::random(&env);
        (env, admin, creator, counterparty)
    }

    #[test]
    fn test_create_agreement() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
            String::from_str(&env, "milestone-2"),
        ]);

        let result = AgreementContract::create_agreement(
            env.clone(),
            id.clone(),
            creator.clone(),
            counterparty.clone(),
            token,
            total_amount,
            expires_at,
            milestones,
        );

        assert!(result.is_ok());
        let agreement = result.unwrap();
        assert_eq!(agreement.id, id);
        assert_eq!(agreement.creator, creator);
        assert_eq!(agreement.counterparty, counterparty);
        assert_eq!(agreement.total_amount, total_amount);
        assert_eq!(agreement.status, AgreementStatus::Draft);
    }

    #[test]
    fn test_create_agreement_with_expired_time() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() - 1; // Already expired
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        let result = AgreementContract::create_agreement(
            env.clone(),
            id,
            creator,
            counterparty,
            token,
            total_amount,
            expires_at,
            milestones,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SettleError::InvalidExpiration);
    }

    #[test]
    fn test_create_agreement_with_zero_amount() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 0;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        let result = AgreementContract::create_agreement(
            env.clone(),
            id,
            creator,
            counterparty,
            token,
            total_amount,
            expires_at,
            milestones,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SettleError::InvalidAmount);
    }

    #[test]
    fn test_create_agreement_with_same_creator_and_counterparty() {
        let (env, _, creator, _) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        let result = AgreementContract::create_agreement(
            env.clone(),
            id,
            creator.clone(),
            creator, // Same as creator
            token,
            total_amount,
            expires_at,
            milestones,
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SettleError::InvalidParticipants);
    }

    #[test]
    fn test_fund_agreement() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        // Create agreement first
        AgreementContract::create_agreement(
            env.clone(),
            id.clone(),
            creator.clone(),
            counterparty.clone(),
            token,
            total_amount,
            expires_at,
            milestones,
        ).unwrap();

        // Fund agreement
        let funder = Address::random(&env);
        let fund_amount = 500000;

        let result = AgreementContract::fund_agreement(
            env.clone(),
            id.clone(),
            funder,
            fund_amount,
        );

        assert!(result.is_ok());
        let agreement = result.unwrap();
        assert_eq!(agreement.funded_amount, fund_amount);
        assert_eq!(agreement.status, AgreementStatus::Funded);
    }

    #[test]
    fn test_activate_agreement() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        // Create agreement
        AgreementContract::create_agreement(
            env.clone(),
            id.clone(),
            creator.clone(),
            counterparty.clone(),
            token,
            total_amount,
            expires_at,
            milestones,
        ).unwrap();

        // Fund agreement
        let funder = Address::random(&env);
        AgreementContract::fund_agreement(
            env.clone(),
            id.clone(),
            funder,
            total_amount,
        ).unwrap();

        // Activate agreement
        let result = AgreementContract::activate_agreement(
            env.clone(),
            id.clone(),
            creator,
        );

        assert!(result.is_ok());
        let agreement = result.unwrap();
        assert_eq!(agreement.status, AgreementStatus::Active);
    }

    #[test]
    fn test_complete_agreement() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        // Create agreement
        AgreementContract::create_agreement(
            env.clone(),
            id.clone(),
            creator.clone(),
            counterparty.clone(),
            token,
            total_amount,
            expires_at,
            milestones,
        ).unwrap();

        // Fund agreement
        let funder = Address::random(&env);
        AgreementContract::fund_agreement(
            env.clone(),
            id.clone(),
            funder,
            total_amount,
        ).unwrap();

        // Activate agreement
        AgreementContract::activate_agreement(
            env.clone(),
            id.clone(),
            creator.clone(),
        ).unwrap();

        // Complete agreement
        let result = AgreementContract::complete_agreement(
            env.clone(),
            id.clone(),
            counterparty,
        );

        assert!(result.is_ok());
        let agreement = result.unwrap();
        assert_eq!(agreement.status, AgreementStatus::Completed);
    }

    #[test]
    fn test_cancel_agreement() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        // Create agreement
        AgreementContract::create_agreement(
            env.clone(),
            id.clone(),
            creator.clone(),
            counterparty.clone(),
            token,
            total_amount,
            expires_at,
            milestones,
        ).unwrap();

        // Cancel agreement
        let result = AgreementContract::cancel_agreement(
            env.clone(),
            id.clone(),
            creator,
        );

        assert!(result.is_ok());
        let agreement = result.unwrap();
        assert_eq!(agreement.status, AgreementStatus::Cancelled);
    }

    #[test]
    fn test_get_agreement() {
        let (env, _, creator, counterparty) = setup();
        env.mock_all_auths();

        let id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        // Create agreement
        AgreementContract::create_agreement(
            env.clone(),
            id.clone(),
            creator,
            counterparty,
            token,
            total_amount,
            expires_at,
            milestones,
        ).unwrap();

        // Get agreement
        let result = AgreementContract::get_agreement(env.clone(), id.clone());

        assert!(result.is_some());
        let agreement = result.unwrap();
        assert_eq!(agreement.id, id);
    }

    #[test]
    fn test_get_nonexistent_agreement() {
        let env = Env::default();
        let id = String::from_str(&env, "nonexistent");

        let result = AgreementContract::get_agreement(env, id);

        assert!(result.is_none());
    }
}