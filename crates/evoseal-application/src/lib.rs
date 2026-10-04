use evoseal_domain::{BoundaryDecision, DecisionStatus, PromotionRequest, boundary_gate, validate_request};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RrsiResult {
    pub admissible: bool,
    pub reason: String,
    pub edit_budget: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LedgerReceipt {
    pub history_id: String,
    pub nearest_prior_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WitnessReceipt {
    pub root_hex: String,
    pub entries: usize,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromotionReport {
    pub run_id: String,
    pub candidate_id: String,
    pub status: DecisionStatus,
    pub reasons: Vec<String>,
    pub rrsi: RrsiResult,
    pub ledger: LedgerReceipt,
    pub witness: WitnessReceipt,
    pub authority: String,
}

pub trait RrsiSelector {
    fn select(&self, request: &PromotionRequest) -> Result<RrsiResult, IntegrationError>;
}

pub trait DecisionLedger {
    fn record(&self, request: &PromotionRequest, decision: &BoundaryDecision)
        -> Result<LedgerReceipt, IntegrationError>;
}

pub trait WitnessSealer {
    fn seal(&self, canonical_actions: &[Vec<u8>]) -> Result<WitnessReceipt, IntegrationError>;
}

#[derive(Debug, Error)]
pub enum IntegrationError {
    #[error("upstream rejected request: {0}")]
    Upstream(String),
    #[error("upstream command timed out")]
    Timeout,
    #[error("persistence boundary failed: {0}")]
    Persistence(String),
    #[error("witness verification failed: {0}")]
    Witness(String),
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] evoseal_domain::DomainError),
    #[error(transparent)]
    Integration(#[from] IntegrationError),
    #[error("canonical serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub struct PromotionService<R, L, W> {
    rrsi: R,
    ledger: L,
    witness: W,
}

impl<R, L, W> PromotionService<R, L, W>
where
    R: RrsiSelector,
    L: DecisionLedger,
    W: WitnessSealer,
{
    pub fn new(rrsi: R, ledger: L, witness: W) -> Self {
        Self { rrsi, ledger, witness }
    }

    pub fn evaluate(&self, request: &PromotionRequest) -> Result<PromotionReport, ApplicationError> {
        validate_request(request)?;
        let rrsi = self.rrsi.select(request)?;
        let decision = boundary_gate(request, rrsi.admissible);
        let request_bytes = serde_json::to_vec(request)?;
        let decision_bytes = serde_json::to_vec(&decision)?;
        let witness = self.witness.seal(&[request_bytes, decision_bytes])?;
        if !witness.verified {
            return Err(IntegrationError::Witness("adapter returned unverified chain".into()).into());
        }
        let ledger = self.ledger.record(request, &decision)?;
        Ok(PromotionReport {
            run_id: request.run_id.clone(),
            candidate_id: request.candidate.candidate_id.clone(),
            status: decision.status,
            reasons: decision.reasons,
            rrsi,
            ledger,
            witness,
            authority: "none".to_owned(),
        })
    }
}

