use anyhow::{Context, Result, bail};
use evoseal_domain::DecisionStatus;
use evoseal_evaluation::{FrozenCase, score};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env, fs,
    io::{self, Read},
};

#[derive(Debug, Deserialize)]
struct DarwinInput {
    #[serde(rename = "variantId")]
    variant_id: String,
    genome: Value,
}

#[derive(Debug, Serialize)]
struct DarwinScoreCard {
    #[serde(rename = "variantId")]
    variant_id: String,
    primary: f64,
    regressed: bool,
    #[serde(rename = "noopRate")]
    noop_rate: f64,
    #[serde(rename = "costPerWin")]
    cost_per_win: f64,
    raw: Value,
}

fn parameter(genome: &Value, name: &str) -> Result<f64> {
    genome
        .get(name)
        .and_then(Value::as_f64)
        .with_context(|| format!("missing numeric genome parameter {name}"))
}

fn main() -> Result<()> {
    let dataset = env::args()
        .nth(1)
        .context("dataset path argument required")?;
    let cases: Vec<FrozenCase> =
        serde_json::from_slice(&fs::read(&dataset).with_context(|| format!("read {dataset}"))?)?;
    if cases.is_empty() {
        bail!("frozen dataset must not be empty");
    }

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let DarwinInput { variant_id, genome } = serde_json::from_str(&input)?;
    let max_ood_regression = parameter(&genome, "max_ood_regression")?;
    let max_cost_increase = parameter(&genome, "max_cost_increase")?;

    let mut tuned = cases;
    for case in &mut tuned {
        case.request.policy.max_ood_regression = max_ood_regression;
        case.request.policy.max_cost_increase = max_cost_increase;
    }
    let result = score(&tuned, "evoseal");
    let total = f64::from(u32::try_from(result.total).context("dataset too large")?);
    let correct = f64::from(u32::try_from(result.correct).context("correct count too large")?);
    let primary = correct / total;
    let promoted = tuned
        .iter()
        .filter(|case| {
            evoseal_domain::boundary_gate(&case.request, case.rrsi_admissible).status
                == DecisionStatus::Promote
        })
        .count();
    let card = DarwinScoreCard {
        variant_id,
        primary,
        regressed: result.unsafe_promotions > 0,
        noop_rate: 1.0
            - f64::from(u32::try_from(promoted).context("promotion count too large")?) / total,
        cost_per_win: if result.correct == 0 {
            max_cost_increase * total
        } else {
            (max_cost_increase * total) / correct
        },
        raw: serde_json::json!({
            "correct": result.correct,
            "total": result.total,
            "unsafe_promotions": result.unsafe_promotions,
            "genome": genome,
            "authority": "none"
        }),
    };
    println!("{}", serde_json::to_string(&card)?);
    Ok(())
}
