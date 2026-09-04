#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String};
    use soroban_sdk::testutils::Address as _;
    use crate::types::*;
    use crate::SettleContract;
    use crate::SettleContractClient;

    fn setup(env: &Env) -> (Address, Address, String) {
        let creator = Address::generate(env);
        let counterparty = Address::generate(env);
        let id = String::from_str(env, "agreement-1");
        let token = Address::generate(env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = soroban_sdk::Vec::from_array(env, [
            String::from_str(env, "milestone-1"),
        ]);

        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(env, &contract_id);
        client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        (creator, counterparty, id)
    }

    #[test]
    fn test_create_escrow() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let creator = Address::generate(&env);
        let counterparty = Address::generate(&env);
        let id = String::from_str(&env, "agreement-1");
        let token = Address::generate(&env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = soroban_sdk::Vec::from_array(&env, [
            String::from_str(&env, "milestone-1"),
        ]);

        client.create_agreement(
            &id, &creator, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );

        let escrow = client.create_escrow(&id, &token, &total_amount);
        assert_eq!(escrow.agreement_id, id);
        assert_eq!(escrow.amount, total_amount);
        assert_eq!(escrow.status, EscrowStatus::Empty);
    }

    #[test]
    fn test_get_nonexistent_escrow() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let agreement_id = String::from_str(&env, "nonexistent");
        let result = client.get_escrow(&agreement_id);
        assert!(result.is_none());
    }
}
