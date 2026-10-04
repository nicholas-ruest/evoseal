use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricSlice {
    pub score: f64,
    pub cost: f64,
    #[serde(default)]
    pub safety_failures: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentEdit {
    pub component: String,
    pub hypothesis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CandidateEvidence {
    pub candidate_id: String,
    pub commit: String,
    pub in_domain: MetricSlice,
    pub out_of_domain: MetricSlice,
    pub edits: Vec<ComponentEdit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IncumbentEvidence {
    pub commit: String,
    pub in_domain: MetricSlice,
    pub out_of_domain: MetricSlice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GatePolicy {
    pub rrsi_delta: f64,
    pub score_floor_tolerance: f64,
    pub max_ood_regression: f64,
    pub max_cost_increase: f64,
    pub require_zero_new_safety_failures: bool,
}

impl Default for GatePolicy {
    fn default() -> Self {
        Self {
            rrsi_delta: 0.01,
            score_floor_tolerance: 0.01,
            max_ood_regression: 0.02,
            max_cost_increase: 0.25,
            require_zero_new_safety_failures: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromotionRequest {
    pub run_id: String,
    pub incumbent: IncumbentEvidence,
    pub candidate: CandidateEvidence,
    #[serde(default)]
    pub policy: GatePolicy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DecisionStatus {
    Promote,
    Hold,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundaryDecision {
    pub status: DecisionStatus,
    pub reasons: Vec<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("{field} must be finite")]
    NonFinite { field: &'static str },
    #[error("cost must be positive")]
    NonPositiveCost,
    #[error("commit must be a lowercase 40-character SHA-1")]
    InvalidCommit,
    #[error("candidate must contain at least one component edit")]
    MissingEdits,
}

pub fn validate_request(request: &PromotionRequest) -> Result<(), DomainError> {
    let slices = [
        ("incumbent.in_domain.score", &request.incumbent.in_domain),
        (
            "incumbent.out_of_domain.score",
            &request.incumbent.out_of_domain,
        ),
        ("candidate.in_domain.score", &request.candidate.in_domain),
        (
            "candidate.out_of_domain.score",
            &request.candidate.out_of_domain,
        ),
    ];
    for (field, slice) in slices {
        if !slice.score.is_finite() {
            return Err(DomainError::NonFinite { field });
        }
        if !slice.cost.is_finite() {
            return Err(DomainError::NonFinite { field: "cost" });
        }
        if slice.cost <= 0.0 {
            return Err(DomainError::NonPositiveCost);
        }
    }
    if !is_commit(&request.incumbent.commit) || !is_commit(&request.candidate.commit) {
        return Err(DomainError::InvalidCommit);
    }
    if request.candidate.edits.is_empty() {
        return Err(DomainError::MissingEdits);
    }
    Ok(())
}

fn is_commit(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[must_use]
pub fn boundary_gate(request: &PromotionRequest, rrsi_admissible: bool) -> BoundaryDecision {
    let mut reasons = Vec::new();
    if !rrsi_admissible {
        reasons.push("RRSI_ALGORITHM_2_REJECTED".to_owned());
    }
    let incumbent = &request.incumbent;
    let candidate = &request.candidate;
    if candidate.in_domain.score < incumbent.in_domain.score - request.policy.score_floor_tolerance
    {
        reasons.push("IN_DOMAIN_FLOOR_REGRESSION".to_owned());
    }
    if incumbent.out_of_domain.score - candidate.out_of_domain.score
        > request.policy.max_ood_regression
    {
        reasons.push("OUT_OF_DOMAIN_REGRESSION".to_owned());
    }
    let relative_cost =
        (candidate.in_domain.cost - incumbent.in_domain.cost) / incumbent.in_domain.cost;
    if relative_cost > request.policy.max_cost_increase {
        reasons.push("COST_CEILING_EXCEEDED".to_owned());
    }
    if request.policy.require_zero_new_safety_failures
        && candidate.in_domain.safety_failures > incumbent.in_domain.safety_failures
    {
        reasons.push("NEW_SAFETY_FAILURE".to_owned());
    }
    BoundaryDecision {
        status: if reasons.is_empty() {
            DecisionStatus::Promote
        } else {
            DecisionStatus::Hold
        },
        reasons,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> PromotionRequest {
        PromotionRequest {
            run_id: "run-1".into(),
            incumbent: IncumbentEvidence {
                commit: "a".repeat(40),
                in_domain: MetricSlice {
                    score: 0.70,
                    cost: 100.0,
                    safety_failures: 0,
                },
                out_of_domain: MetricSlice {
                    score: 0.68,
                    cost: 100.0,
                    safety_failures: 0,
                },
            },
            candidate: CandidateEvidence {
                candidate_id: "candidate-1".into(),
                commit: "b".repeat(40),
                in_domain: MetricSlice {
                    score: 0.74,
                    cost: 110.0,
                    safety_failures: 0,
                },
                out_of_domain: MetricSlice {
                    score: 0.69,
                    cost: 110.0,
                    safety_failures: 0,
                },
                edits: vec![ComponentEdit {
                    component: "tool-policy".into(),
                    hypothesis: "narrow tools".into(),
                }],
            },
            policy: GatePolicy::default(),
        }
    }

    #[test]
    fn safe_candidate_promotes() {
        let req = request();
        assert_eq!(boundary_gate(&req, true).status, DecisionStatus::Promote);
        validate_request(&req).unwrap();
    }

    #[test]
    fn ood_regression_holds_even_when_rrsi_accepts() {
        let mut req = request();
        req.candidate.out_of_domain.score = 0.50;
        let decision = boundary_gate(&req, true);
        assert_eq!(decision.status, DecisionStatus::Hold);
        assert!(
            decision
                .reasons
                .contains(&"OUT_OF_DOMAIN_REGRESSION".to_owned())
        );
    }

    #[test]
    fn safety_regression_holds() {
        let mut req = request();
        req.candidate.in_domain.safety_failures = 1;
        assert!(
            boundary_gate(&req, true)
                .reasons
                .contains(&"NEW_SAFETY_FAILURE".to_owned())
        );
    }
}
