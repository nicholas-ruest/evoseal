#!/usr/bin/env python3
"""Bounded JSON bridge to Google RRSI's real schedule and Algorithm 2 APIs."""
import json
import sys

from rrsi.config import RRSIConfig
from rrsi.evaluate import EvalResult
from rrsi.schedule import edit_budget
from rrsi.selection import Candidate, select_round


def metric(job, data):
    return EvalResult(
        job=job,
        k=1,
        per_task={},
        S=float(data["score"]),
        C=float(data["cost"]),
        n_expected=1,
        missing=0,
        extra={"safety_failures": int(data.get("safety_failures", 0))},
    )


def main():
    request = json.load(sys.stdin)
    incumbent = metric("incumbent", request["incumbent"]["in_domain"])
    candidate = Candidate(
        variant=request["candidate"]["candidate_id"],
        edits=request["candidate"]["edits"],
        ev=metric("candidate", request["candidate"]["in_domain"]),
        commit=request["candidate"]["commit"],
    )
    cfg = RRSIConfig(delta=float(request["policy"]["rrsi_delta"]))
    counts = {edit["component"]: 1 for edit in request["candidate"]["edits"]}
    winner, decisions = select_round(
        [candidate], incumbent, incumbent.S, cfg.delta, cfg, counts
    )
    decision = decisions[0]
    json.dump(
        {
            "admissible": winner is not None and decision.admissible,
            "reason": decision.reason,
            "edit_budget": edit_budget(1, cfg.T, cfg.b_min, cfg.b_max),
        },
        sys.stdout,
        sort_keys=True,
    )


if __name__ == "__main__":
    main()

