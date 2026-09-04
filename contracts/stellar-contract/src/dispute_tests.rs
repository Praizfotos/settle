#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String};
    use soroban_sdk::testutils::Address as _;
    use crate::types::*;
    use crate::SettleContract;
    use crate::SettleContractClient;

    fn create_active_agreement(client: &SettleContractClient, env: &Env, id: &str) -> (Address, Address) {
        let opener = Address::generate(env);
        let counterparty = Address::generate(env);
        let agreement_id = String::from_str(env, id);
        let token = Address::generate(env);
        let total_amount = 1000000;
        let expires_at = env.ledger().timestamp() + 86400;
        let milestones = soroban_sdk::Vec::from_array(env, [
            String::from_str(env, "milestone-1"),
        ]);

        client.create_agreement(
            &agreement_id, &opener, &counterparty, &token, &total_amount, &expires_at, &milestones,
        );
        client.fund_agreement(&agreement_id, &opener, &total_amount);
        client.activate_agreement(&agreement_id, &opener);
        (opener, counterparty)
    }

    #[test]
    fn test_open_dispute() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let (opener, _) = create_active_agreement(&client, &env, "agreement-1");
        let agreement_id = String::from_str(&env, "agreement-1");

        let id = String::from_str(&env, "dispute-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");

        let dispute = client.open_dispute(&id, &agreement_id, &opener, &reason, &initial_evidence);
        assert_eq!(dispute.id, id);
        assert_eq!(dispute.agreement_id, agreement_id);
        assert_eq!(dispute.opened_by, opener);
        assert_eq!(dispute.status, DisputeStatus::Open);
    }

    #[test]
    fn test_get_nonexistent_dispute() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let id = String::from_str(&env, "nonexistent");
        let result = client.get_dispute(&id);
        assert!(result.is_none());
    }

    #[test]
    fn test_has_open_dispute() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, SettleContract);
        let client = SettleContractClient::new(&env, &contract_id);

        let (opener, _) = create_active_agreement(&client, &env, "agreement-1");
        let agreement_id = String::from_str(&env, "agreement-1");

        assert!(!client.has_open_dispute(&agreement_id));

        let id = String::from_str(&env, "dispute-1");
        let reason = String::from_str(&env, "Milestone not completed as agreed");
        let initial_evidence = String::from_str(&env, "https://evidence.example.com");
        client.open_dispute(&id, &agreement_id, &opener, &reason, &initial_evidence);

        assert!(client.has_open_dispute(&agreement_id));
    }
}
