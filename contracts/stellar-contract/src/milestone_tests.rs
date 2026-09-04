#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String};
    use soroban_sdk::testutils::Address as _;
    use crate::types::*;
    use crate::SettleContract;
    use crate::SettleContractClient;

    #[test]
    fn test_create_milestone() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let counterparty = Address::generate(&env);
        let agreement_id = String::from_str(&env, "agreement-1");
        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let agreement_milestones = soroban_sdk::Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        client.create_agreement(
            &agreement_id, &creator, &counterparty, &token, &total_amount, &expires_at, &agreement_milestones,
        );

        let id = String::from_str(&env, "milestone-1");
        let name = String::from_str(&env, "First Milestone");
        let description = String::from_str(&env, "Complete initial setup");
        let amount = 500000;
        let due_date = env.ledger().timestamp() + 86400;

        let result = client.try_create_milestone(
            &id, &agreement_id, &creator, &name, &description, &amount, &due_date,
        );
        assert!(result.is_ok(), "try_create_milestone host error: {:?}", result.err());
    }

    #[test]
    fn test_get_nonexistent_milestone() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let id = String::from_str(&env, "nonexistent");
        let result = client.get_milestone(&id);
        assert!(result.is_none());
    }
}
