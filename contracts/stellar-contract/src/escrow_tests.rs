#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address, String};
    use crate::types::*;
    use crate::errors::*;
    use crate::escrow::EscrowContract;

    fn setup() -> (Env, Address, Address) {
        let env = Env::default();
        let creator = Address::random(&env);
        let counterparty = Address::random(&env);
        (env, creator, counterparty)
    }

    #[test]
    fn test_create_escrow() {
        let (env, _, _) = setup();
        env.mock_all_auths();

        let agreement_id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let amount = 1000000;

        let result = EscrowContract::create_escrow(
            env.clone(),
            agreement_id.clone(),
            token,
            amount,
        );

        assert!(result.is_ok());
        let escrow = result.unwrap();
        assert_eq!(escrow.agreement_id, agreement_id);
        assert_eq!(escrow.total_amount, amount);
        assert_eq!(escrow.status, EscrowStatus::Empty);
    }

    #[test]
    fn test_fund_escrow() {
        let (env, _, _) = setup();
        env.mock_all_auths();

        let agreement_id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let amount = 1000000;

        // Create escrow first
        EscrowContract::create_escrow(
            env.clone(),
            agreement_id.clone(),
            token,
            amount,
        ).unwrap();

        // Fund escrow
        let funder = Address::random(&env);
        let fund_amount = 500000;

        let result = EscrowContract::fund_escrow(
            env.clone(),
            agreement_id.clone(),
            funder,
            fund_amount,
        );

        assert!(result.is_ok());
        let escrow = result.unwrap();
        assert_eq!(escrow.funded_amount, fund_amount);
        assert_eq!(escrow.status, EscrowStatus::Funded);
    }

    #[test]
    fn test_lock_escrow() {
        let (env, _, _) = setup();
        env.mock_all_auths();

        let agreement_id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let amount = 1000000;

        // Create escrow
        EscrowContract::create_escrow(
            env.clone(),
            agreement_id.clone(),
            token,
            amount,
        ).unwrap();

        // Fund escrow
        let funder = Address::random(&env);
        EscrowContract::fund_escrow(
            env.clone(),
            agreement_id.clone(),
            funder,
            amount,
        ).unwrap();

        // Lock escrow
        let locker = Address::random(&env);
        let lock_amount = 500000;

        let result = EscrowContract::lock_escrow(
            env.clone(),
            agreement_id.clone(),
            locker,
            lock_amount,
        );

        assert!(result.is_ok());
        let escrow = result.unwrap();
        assert_eq!(escrow.locked_amount, lock_amount);
        assert_eq!(escrow.status, EscrowStatus::Locked);
    }

    #[test]
    fn test_release_escrow() {
        let (env, _, _) = setup();
        env.mock_all_auths();

        let agreement_id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let amount = 1000000;

        // Create escrow
        EscrowContract::create_escrow(
            env.clone(),
            agreement_id.clone(),
            token,
            amount,
        ).unwrap();

        // Fund escrow
        let funder = Address::random(&env);
        EscrowContract::fund_escrow(
            env.clone(),
            agreement_id.clone(),
            funder,
            amount,
        ).unwrap();

        // Lock escrow
        let locker = Address::random(&env);
        EscrowContract::lock_escrow(
            env.clone(),
            agreement_id.clone(),
            locker,
            amount,
        ).unwrap();

        // Release escrow
        let releaser = Address::random(&env);
        let recipient = Address::random(&env);
        let release_amount = 500000;

        let result = EscrowContract::release_escrow(
            env.clone(),
            agreement_id.clone(),
            releaser,
            recipient,
            release_amount,
        );

        assert!(result.is_ok());
        let escrow = result.unwrap();
        assert_eq!(escrow.released_amount, release_amount);
    }

    #[test]
    fn test_refund_escrow() {
        let (env, _, _) = setup();
        env.mock_all_auths();

        let agreement_id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let amount = 1000000;

        // Create escrow
        EscrowContract::create_escrow(
            env.clone(),
            agreement_id.clone(),
            token,
            amount,
        ).unwrap();

        // Fund escrow
        let funder = Address::random(&env);
        EscrowContract::fund_escrow(
            env.clone(),
            agreement_id.clone(),
            funder,
            amount,
        ).unwrap();

        // Refund escrow
        let refunder = Address::random(&env);
        let recipient = Address::random(&env);
        let refund_amount = 500000;

        let result = EscrowContract::refund_escrow(
            env.clone(),
            agreement_id.clone(),
            refunder,
            recipient,
            refund_amount,
        );

        assert!(result.is_ok());
        let escrow = result.unwrap();
        assert_eq!(escrow.refunded_amount, refund_amount);
    }

    #[test]
    fn test_get_escrow() {
        let (env, _, _) = setup();
        env.mock_all_auths();

        let agreement_id = String::from_str(&env, "agreement-1");
        let token = Address::random(&env);
        let amount = 1000000;

        // Create escrow
        EscrowContract::create_escrow(
            env.clone(),
            agreement_id.clone(),
            token,
            amount,
        ).unwrap();

        // Get escrow
        let result = EscrowContract::get_escrow(env.clone(), agreement_id.clone());

        assert!(result.is_some());
        let escrow = result.unwrap();
        assert_eq!(escrow.agreement_id, agreement_id);
    }

    #[test]
    fn test_get_nonexistent_escrow() {
        let env = Env::default();
        let agreement_id = String::from_str(&env, "nonexistent");

        let result = EscrowContract::get_escrow(env, agreement_id);

        assert!(result.is_none());
    }
}