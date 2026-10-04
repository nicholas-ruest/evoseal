use anyhow::{Context, Result};
use clap::Parser;
use evoseal_adapter_rrsi::PythonRrsiSelector;
use evoseal_adapter_ruvnet::{RuVectorDecisionLedger, RvfWitnessSealer};
use evoseal_application::PromotionService;
use evoseal_domain::PromotionRequest;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(about = "Evaluate an agent-harness candidate without granting promotion authority")]
struct Args {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long, env = "RRSI_SOURCE")]
    rrsi_source: PathBuf,
    #[arg(long, default_value = "python3")]
    python: PathBuf,
    #[arg(long, default_value = "evidence/runtime/ruvector.redb")]
    ledger: PathBuf,
    #[arg(long, default_value_t = 30)]
    timeout_seconds: u64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = fs::read(&args.input)
        .with_context(|| format!("read {}", args.input.display()))?;
    let request: PromotionRequest = serde_json::from_slice(&input).context("parse input JSON")?;
    if let Some(parent) = args.ledger.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let bridge = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../integrations/rrsi_bridge.py");
    let selector = PythonRrsiSelector::new(
        args.python,
        args.rrsi_source,
        bridge,
        Duration::from_secs(args.timeout_seconds),
    );
    let ledger = RuVectorDecisionLedger::open(&args.ledger)?;
    let report = PromotionService::new(selector, ledger, RvfWitnessSealer).evaluate(&request)?;
    let rendered = serde_json::to_string_pretty(&report)? + "\n";
    if let Some(output) = args.output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        }
        fs::write(&output, rendered).with_context(|| format!("write {}", output.display()))?;
    } else {
        print!("{rendered}");
    }
    Ok(())
}

