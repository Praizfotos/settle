#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String, Vec};
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::testutils::Ledger as _;
    use crate::types::*;
    use crate::SettleContract;
    use crate::SettleContractClient;

    fn setup(env: &Env) -> (SettleContractClient, Address, Address, String) {
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(env, &contract_id);
        let creator = Address::generate(env);
        let counterparty = Address::generate(env);
        let id = String::from_str(env, "agreement-1");
        (client, creator, counterparty, id)
    }

    #[test]
    fn test_create_agreement() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, creator, counterparty, id) = setup(&env);

        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
            String::from_str(&env, "milestone-2"),
        ]);

        let agreement = client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        assert_eq!(agreement.id, id);
        assert_eq!(agreement.creator, creator);
        assert_eq!(agreement.counterparty, counterparty);
        assert_eq!(agreement.total_amount, total_amount);
        assert_eq!(agreement.status, AgreementStatus::Draft);
    }

    #[test]
    fn test_create_agreement_with_expired_time() {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().set_timestamp(100);
        env.ledger().set_sequence_number(1000);
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let counterparty = Address::generate(&env);
        let id = String::from_str(&env, "agreement-1");
        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = 50u64;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        let result = client.try_create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_create_agreement_with_zero_amount() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let counterparty = Address::generate(&env);
        let id = String::from_str(&env, "agreement-1");
        let token = Address::generate(&env);
        let total_amount = 0;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        let result = client.try_create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_create_agreement_with_same_creator_and_counterparty() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let id = String::from_str(&env, "agreement-1");
        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        let result = client.try_create_agreement(
            &id, &creator, &creator, &token, &total_amount, &expires_at, &milestones,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_fund_agreement() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, creator, counterparty, id) = setup(&env);

        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );

        let fund_amount = 500000;
        let agreement = client.fund_agreement(&id, &creator, &fund_amount);
        assert_eq!(agreement.funded_amount, fund_amount);
    }

    #[test]
    fn test_activate_agreement() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, creator, counterparty, id) = setup(&env);

        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        client.fund_agreement(&id, &creator, &total_amount);
        let agreement = client.activate_agreement(&id, &creator);
        assert_eq!(agreement.status, AgreementStatus::Active);
    }

    #[test]
    fn test_complete_agreement() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, creator, counterparty, id) = setup(&env);

        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        client.fund_agreement(&id, &creator, &total_amount);
        client.activate_agreement(&id, &creator);
        let agreement = client.complete_agreement(&id, &counterparty);
        assert_eq!(agreement.status, AgreementStatus::Completed);
    }

    #[test]
    fn test_cancel_agreement() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, creator, counterparty, id) = setup(&env);

        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        let agreement = client.cancel_agreement(&id, &creator);
        assert_eq!(agreement.status, AgreementStatus::Cancelled);
    }

    #[test]
    fn test_get_agreement() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, creator, counterparty, id) = setup(&env);

        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        let agreement = client.get_agreement(&id);
        assert!(agreement.is_some());
        assert_eq!(agreement.unwrap().id, id);
    }

    #[test]
    fn test_get_nonexistent_agreement() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let id = String::from_str(&env, "nonexistent");
        let result = client.get_agreement(&id);
        assert!(result.is_none());
    }
}
