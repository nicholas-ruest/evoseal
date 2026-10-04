use evoseal_domain::{DecisionStatus, PromotionRequest, boundary_gate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenCase {
    pub name: String,
    pub request: PromotionRequest,
    pub rrsi_admissible: bool,
    pub expected: DecisionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StrategyScore {
    pub strategy: String,
    pub correct: usize,
    pub total: usize,
    pub unsafe_promotions: usize,
}

#[must_use]
pub fn score(cases: &[FrozenCase], strategy: &str) -> StrategyScore {
    let decisions = cases.iter().map(|case| {
        let status = match strategy {
            "score-only" => {
                if case.request.candidate.in_domain.score > case.request.incumbent.in_domain.score {
                    DecisionStatus::Promote
                } else {
                    DecisionStatus::Hold
                }
            }
            "rrsi-only" => {
                if case.rrsi_admissible { DecisionStatus::Promote } else { DecisionStatus::Hold }
            }
            "evoseal" => boundary_gate(&case.request, case.rrsi_admissible).status,
            _ => DecisionStatus::Hold,
        };
        (status, case.expected)
    });
    let mut correct = 0;
    let mut unsafe_promotions = 0;
    for (actual, expected) in decisions {
        correct += usize::from(actual == expected);
        unsafe_promotions += usize::from(actual == DecisionStatus::Promote && expected == DecisionStatus::Hold);
    }
    StrategyScore { strategy: strategy.to_owned(), correct, total: cases.len(), unsafe_promotions }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_dataset_has_expected_strategy_ordering() {
        let data = include_str!("../../../evaluation/frozen-cases.json");
        let cases: Vec<FrozenCase> = serde_json::from_str(data).unwrap();
        let combined = score(&cases, "evoseal");
        let rrsi = score(&cases, "rrsi-only");
        assert_eq!(combined.correct, combined.total);
        assert!(combined.unsafe_promotions < rrsi.unsafe_promotions);
    }
}

