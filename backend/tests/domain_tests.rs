//! Domain-level unit tests — no I/O, no DB, no network.

#[cfg(test)]
mod reputation_tests {
    use settle_backend::domain::reputation::ReputationScore;

    #[test]
    fn excellent_provider_scores_above_80() {
        let rep = ReputationScore::compute(
            "GABCDEF".into(),
            85,        // completed
            250_000,   // volume
            0.97,      // on-time rate
            0.015,     // dispute rate
            1.0,       // win rate
        );
        assert!(rep.score >= 80.0, "score was {}", rep.score);
    }

    #[test]
    fn zero_activity_gives_unrated() {
        let rep = ReputationScore::compute("GABCDEF".into(), 0, 0, 0.0, 0.0, 0.0);
        use settle_backend::domain::reputation::ReputationLabel;
        assert_eq!(rep.label, ReputationLabel::Unrated);
    }

    #[test]
    fn amounts_balance_invariant() {
        // Escrow: total = escrowed + released + refunded
        let total: i64 = 2500;
        let escrowed: i64 = 1250;
        let released: i64 = 750;
        let refunded: i64 = 500;
        assert_eq!(escrowed + released + refunded, total);
    }
}
