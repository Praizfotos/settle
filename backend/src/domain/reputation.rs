//! Reputation is derived entirely from on-chain settlement history.
//! No self-reported data is accepted.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationScore {
    pub address: String,
    /// Composite score 0.0–100.0.
    pub score: f64,
    pub label: ReputationLabel,
    pub completed_agreements: i32,
    /// Total volume settled in token base units.
    pub settled_volume: i64,
    /// Percentage 0.0–100.0.
    pub on_time_completion_rate: f64,
    /// Percentage 0.0–100.0.
    pub dispute_rate: f64,
    /// Disputes the address opened that resolved in their favour.
    pub dispute_win_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReputationLabel {
    Unrated,
    New,
    Developing,
    Good,
    Excellent,
}

impl ReputationScore {
    /// Derive a reputation score from raw settlement statistics.
    ///
    /// Formula (v1):
    ///   score = 0.40 × on_time_rate
    ///         + 0.30 × (1 - dispute_rate)
    ///         + 0.20 × volume_factor   (log-scaled, capped at 1.0)
    ///         + 0.10 × agreement_count_factor (log-scaled, capped at 1.0)
    pub fn compute(
        address: String,
        completed: i32,
        settled_volume: i64,
        on_time_rate: f64,
        dispute_rate: f64,
        dispute_win_rate: f64,
    ) -> Self {
        let volume_factor = (settled_volume as f64 / 100_000.0).ln_1p().min(1.0);
        let count_factor  = (completed as f64 / 50.0).ln_1p().min(1.0);

        let raw = 0.40 * on_time_rate
            + 0.30 * (1.0 - dispute_rate)
            + 0.20 * volume_factor
            + 0.10 * count_factor;

        let score = (raw * 100.0).clamp(0.0, 100.0);

        let label = match score as u32 {
            0..=9   => ReputationLabel::Unrated,
            10..=29 => ReputationLabel::New,
            30..=59 => ReputationLabel::Developing,
            60..=84 => ReputationLabel::Good,
            _       => ReputationLabel::Excellent,
        };

        Self {
            address,
            score,
            label,
            completed_agreements: completed,
            settled_volume,
            on_time_completion_rate: on_time_rate,
            dispute_rate,
            dispute_win_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excellent_score_for_clean_history() {
        let rep = ReputationScore::compute(
            "GABC...".into(),
            100,
            500_000,
            0.98,
            0.01,
            1.0,
        );
        assert!(rep.score >= 80.0);
        assert_eq!(rep.label, ReputationLabel::Excellent);
    }

    #[test]
    fn new_score_for_zero_activity() {
        let rep = ReputationScore::compute("GABC...".into(), 0, 0, 0.0, 0.0, 0.0);
        assert_eq!(rep.label, ReputationLabel::Unrated);
    }
}
