# Frozen implementation contract

The JSON shapes in `evoseal-domain` and the following policy order are frozen for the bounded evaluation:

1. validate input;
2. call RRSI once with the supplied candidate and incumbent;
3. HOLD if RRSI rejects;
4. HOLD on in-domain floor regression;
5. HOLD on OOD regression greater than policy ceiling;
6. HOLD on relative cost above ceiling;
7. HOLD on any new safety failure when enabled;
8. seal and verify request plus decision;
9. record and read back the decision;
10. emit `authority: "none"`.

Candidate strategies must share the same frozen cases:

- `score-only`: promotes on positive in-domain delta;
- `rrsi-only`: follows upstream admissibility only;
- `evoseal`: follows the complete contract above.

No strategy may change datasets, incumbent measurements, thresholds, or expected outcomes.

