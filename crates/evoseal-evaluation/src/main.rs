use anyhow::{Context, Result};
use clap::Parser;
use evoseal_evaluation::{FrozenCase, score};
use serde::Serialize;
use std::{fs, path::PathBuf, time::Instant};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value = "evaluation/frozen-cases.json")]
    dataset: PathBuf,
    #[arg(long, default_value_t = 10_000)]
    iterations: usize,
}

#[derive(Serialize)]
struct EvaluationReport {
    dataset: String,
    iterations: usize,
    elapsed_ns: u128,
    strategies: Vec<evoseal_evaluation::StrategyScore>,
    claim: &'static str,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let bytes = fs::read(&args.dataset).with_context(|| format!("read {}", args.dataset.display()))?;
    let cases: Vec<FrozenCase> = serde_json::from_slice(&bytes).context("parse frozen dataset")?;
    let start = Instant::now();
    for _ in 0..args.iterations {
        std::hint::black_box(score(&cases, "evoseal"));
    }
    let report = EvaluationReport {
        dataset: args.dataset.display().to_string(),
        iterations: args.iterations,
        elapsed_ns: start.elapsed().as_nanos(),
        strategies: ["score-only", "rrsi-only", "evoseal"].iter().map(|name| score(&cases, name)).collect(),
        claim: "mechanism fixture only; not a production performance claim",
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

