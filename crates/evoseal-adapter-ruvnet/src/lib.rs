use evoseal_application::{DecisionLedger, IntegrationError, LedgerReceipt, WitnessReceipt, WitnessSealer};
use evoseal_domain::{BoundaryDecision, DecisionStatus, PromotionRequest};
use ruvector_core::{DbOptions, DistanceMetric, HnswConfig, SearchQuery, VectorDB, VectorEntry};
use rvf_crypto::{WitnessEntry, create_witness_chain, shake256_256, verify_witness_chain};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::Path;

pub struct RuVectorDecisionLedger {
    db: VectorDB,
}

impl RuVectorDecisionLedger {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, IntegrationError> {
        let options = DbOptions {
            dimensions: 8,
            distance_metric: DistanceMetric::Cosine,
            storage_path: path.as_ref().to_string_lossy().into_owned(),
            hnsw_config: Some(HnswConfig::default()),
            quantization: None,
        };
        let db = VectorDB::new(options)
            .map_err(|error| IntegrationError::Persistence(error.to_string()))?;
        Ok(Self { db })
    }

    #[allow(clippy::cast_precision_loss)]
    fn features(request: &PromotionRequest, decision: &BoundaryDecision) -> Vec<f32> {
        vec![
            request.incumbent.in_domain.score as f32,
            request.candidate.in_domain.score as f32,
            request.incumbent.out_of_domain.score as f32,
            request.candidate.out_of_domain.score as f32,
            (request.candidate.in_domain.cost / request.incumbent.in_domain.cost) as f32,
            request.candidate.in_domain.safety_failures as f32,
            request.candidate.edits.len() as f32,
            if decision.status == DecisionStatus::Promote { 1.0 } else { -1.0 },
        ]
    }
}

impl DecisionLedger for RuVectorDecisionLedger {
    fn record(
        &self,
        request: &PromotionRequest,
        decision: &BoundaryDecision,
    ) -> Result<LedgerReceipt, IntegrationError> {
        let vector = Self::features(request, decision);
        let query = SearchQuery { vector: vector.clone(), k: 3, filter: None, ef_search: Some(32) };
        let nearest_prior_ids = self.db.search(query)
            .map_err(|error| IntegrationError::Persistence(error.to_string()))?
            .into_iter().map(|result| result.id).collect();
        let mut metadata = HashMap::<String, Value>::new();
        metadata.insert("run_id".into(), json!(request.run_id));
        metadata.insert("candidate_commit".into(), json!(request.candidate.commit));
        metadata.insert("status".into(), json!(format!("{:?}", decision.status)));
        metadata.insert("authority".into(), json!("none"));
        let id = format!("{}:{}", request.run_id, request.candidate.candidate_id);
        let stored = self.db.insert(VectorEntry {
            id: Some(id.clone()),
            vector,
            metadata: Some(metadata),
        }).map_err(|error| IntegrationError::Persistence(error.to_string()))?;
        let read_back = self.db.get(&stored)
            .map_err(|error| IntegrationError::Persistence(error.to_string()))?;
        if read_back.is_none() {
            return Err(IntegrationError::Persistence("write-then-verify failed".into()));
        }
        Ok(LedgerReceipt { history_id: stored, nearest_prior_ids })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RvfWitnessSealer;

impl WitnessSealer for RvfWitnessSealer {
    fn seal(&self, canonical_actions: &[Vec<u8>]) -> Result<WitnessReceipt, IntegrationError> {
        let entries: Vec<WitnessEntry> = canonical_actions.iter().enumerate().map(|(index, action)| WitnessEntry {
            prev_hash: [0; 32],
            action_hash: shake256_256(action),
            timestamp_ns: u64::try_from(index).unwrap_or(u64::MAX),
            witness_type: if index == 0 { 0x01 } else { 0x02 },
        }).collect();
        let chain = create_witness_chain(&entries);
        let verified = verify_witness_chain(&chain)
            .map_err(|error| IntegrationError::Witness(error.to_string()))?;
        let root_hex = hex::encode(shake256_256(&chain));
        Ok(WitnessReceipt { root_hex, entries: verified.len(), verified: verified == entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evoseal_domain::{CandidateEvidence, ComponentEdit, GatePolicy, IncumbentEvidence, MetricSlice};
    use tempfile::tempdir;

    fn fixture() -> PromotionRequest {
        PromotionRequest {
            run_id: "ledger-test".into(),
            incumbent: IncumbentEvidence {
                commit: "a".repeat(40),
                in_domain: MetricSlice { score: 0.7, cost: 100.0, safety_failures: 0 },
                out_of_domain: MetricSlice { score: 0.7, cost: 100.0, safety_failures: 0 },
            },
            candidate: CandidateEvidence {
                candidate_id: "c1".into(),
                commit: "b".repeat(40),
                in_domain: MetricSlice { score: 0.75, cost: 105.0, safety_failures: 0 },
                out_of_domain: MetricSlice { score: 0.72, cost: 105.0, safety_failures: 0 },
                edits: vec![ComponentEdit { component: "planner".into(), hypothesis: "bounded".into() }],
            },
            policy: GatePolicy::default(),
        }
    }

    #[test]
    fn real_ruvector_write_is_read_back() {
        let dir = tempdir().unwrap();
        let ledger = RuVectorDecisionLedger::open(dir.path().join("db.redb")).unwrap();
        let receipt = ledger.record(&fixture(), &BoundaryDecision {
            status: DecisionStatus::Promote,
            reasons: Vec::new(),
        }).unwrap();
        assert_eq!(receipt.history_id, "ledger-test:c1");
    }

    #[test]
    fn real_rvf_chain_verifies_and_tampering_fails() {
        let sealer = RvfWitnessSealer;
        let receipt = sealer.seal(&[b"request".to_vec(), b"decision".to_vec()]).unwrap();
        assert!(receipt.verified);
        assert_eq!(receipt.entries, 2);
        assert_eq!(receipt.root_hex.len(), 64);
        let entries = vec![WitnessEntry {
            prev_hash: [0; 32], action_hash: shake256_256(b"a"), timestamp_ns: 0, witness_type: 1,
        }, WitnessEntry {
            prev_hash: [0; 32], action_hash: shake256_256(b"b"), timestamp_ns: 1, witness_type: 2,
        }];
        let mut chain = create_witness_chain(&entries);
        chain[0] ^= 1;
        assert!(verify_witness_chain(&chain).is_err());
    }
}
