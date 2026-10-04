use evoseal_adapter_rrsi::PythonRrsiSelector;
use evoseal_application::RrsiSelector;
use evoseal_domain::{CandidateEvidence, ComponentEdit, GatePolicy, IncumbentEvidence, MetricSlice, PromotionRequest};
use std::{env, path::PathBuf, time::Duration};

#[test]
#[ignore = "requires pinned Google RRSI checkout; run with RRSI_SOURCE"]
fn invokes_google_rrsi_algorithm_two() {
    let source = PathBuf::from(env::var("RRSI_SOURCE").expect("RRSI_SOURCE required"));
    let bridge = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../integrations/rrsi_bridge.py");
    let selector = PythonRrsiSelector::new("python3", source, bridge, Duration::from_secs(15));
    let request = PromotionRequest {
        run_id: "upstream-rrsi".into(),
        incumbent: IncumbentEvidence {
            commit: "a".repeat(40),
            in_domain: MetricSlice { score: 0.70, cost: 100.0, safety_failures: 0 },
            out_of_domain: MetricSlice { score: 0.70, cost: 100.0, safety_failures: 0 },
        },
        candidate: CandidateEvidence {
            candidate_id: "better".into(),
            commit: "b".repeat(40),
            in_domain: MetricSlice { score: 0.75, cost: 110.0, safety_failures: 0 },
            out_of_domain: MetricSlice { score: 0.74, cost: 110.0, safety_failures: 0 },
            edits: vec![ComponentEdit { component: "tool_policy".into(), hypothesis: "narrower".into() }],
        },
        policy: GatePolicy::default(),
    };
    let result = selector.select(&request).expect("real RRSI call succeeds");
    assert!(result.admissible, "{}", result.reason);
    assert!(result.edit_budget > 0);
}
