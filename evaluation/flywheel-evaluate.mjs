import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const entry = process.env.METAHARNESS_FLYWHEEL_ENTRY;
if (!entry) throw new Error("METAHARNESS_FLYWHEEL_ENTRY is required");
const { makeSigner, runFlywheelGenerations, verifyReplayBundle } = await import(entry);

const repo = resolve(process.argv[2] ?? ".");
const binary = resolve(repo, "target/debug/darwin_evaluator");
const dataset = resolve(repo, "evaluation/frozen-cases.json");
const items = JSON.parse(readFileSync(dataset, "utf8"));
const frozenNoPromotion = () => ({
  promote: false,
  reasons: ["HUMAN_APPROVAL_REQUIRED", "AUTHORITY_NONE"],
});

const proposed = {
  max_ood_regression: "0.02",
  max_cost_increase: "0.25",
};
const proposer = async (_base, target) => ({
  value: proposed[target],
  summary: `bounded deterministic ${target} proposal`,
});
const evaluator = async (policy) => {
  const request = JSON.stringify({ variantId: "flywheel", genome: Object.fromEntries(
    Object.entries(policy).map(([key, value]) => [key, Number(value)]),
  ) });
  const child = spawnSync(binary, [dataset], { input: request, encoding: "utf8" });
  if (child.status !== 0) throw new Error(child.stderr || `evaluator exited ${child.status}`);
  const card = JSON.parse(child.stdout);
  return {
    primary: card.primary,
    noopRate: card.noopRate,
    costPerWin: card.costPerWin,
    regressed: card.regressed,
  };
};

const result = await runFlywheelGenerations({
  rootPolicy: { max_ood_regression: "0.10", max_cost_increase: "0.50" },
  proposer,
  evaluator,
  promotionRule: frozenNoPromotion,
  holdout: { id: "evoseal-frozen-v1", items },
  anchor: { id: "evoseal-anchor-v1", items },
  mutationTargets: ["max_ood_regression", "max_cost_increase"],
  maxGenerations: 2,
  signer: makeSigner(),
  now: (generation) => `generation-${generation}`,
  cacheEvaluations: true,
  dataSource: "FROZEN_LOCAL_FIXTURE",
});
const verdict = verifyReplayBundle(result.replayBundle, { promotionRule: frozenNoPromotion });
const output = {
  package: "@metaharness/flywheel@0.1.12",
  generationsRun: result.generationsRun,
  promotions: result.promotions.length,
  milestoneReached: result.milestoneReached,
  finalPolicy: result.finalPolicy,
  replayVerdict: verdict,
  authority: "none",
};
writeFileSync(resolve(repo, "evidence/flywheel-replay.json"), `${JSON.stringify(result.replayBundle, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(output, null, 2)}\n`);
if (!verdict.pass || result.promotions.length !== 0) process.exitCode = 1;
