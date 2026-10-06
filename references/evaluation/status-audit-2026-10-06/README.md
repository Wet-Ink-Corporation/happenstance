# Status-quo audit, 2026-10-06: the evidence

The working files behind
[`../status-audit-2026-10-06.md`](../status-audit-2026-10-06.md), pinned to
`1f92d08`. Like the report, they are immutable: superseded, never edited. They are
intermediate material. Where a map and the report disagree, the report wins,
because it carries the verifiers' corrections.

## How the audit ran

1. **Measure.** `cargo xtask ci` ran in a detached worktree at `1f92d08`, and so did
   five of the seven examples. The result is in [`gate-summary.md`](gate-summary.md).
   The raw logs are not kept, because the same command reproduces them. The lines the report cites from the second run are kept in [`gate2-excerpts.txt`](gate2-excerpts.txt), under their original line numbers.
2. **Map.** Nine independent read-only readers each covered one area, and each
   cited `path:line`, a commit or a command for every claim: vision, plan, spec,
   core, adapters, sync, docs, history and governance. Their output is the
   `map-*.md` files.
3. **Assess.** Eight assessors, one per dimension, rated and scored each area from
   the maps, spot-checking against the tree. Their output is the `assess-*.md`
   files.
4. **Verify.** Every high- and medium-severity finding went to two verifiers. One
   checked the evidence against primary sources; the other tried to refute the
   finding. The votes and their corrected wording are in
   [`verified.json`](verified.json): 27 confirmed, 26 partly true and reworded,
   none refuted outright.
5. **Estimate.** An independent per-phase effort range is in
   [`estimate.md`](estimate.md), calibrated on phase 17's re-estimate.
6. **Synthesize and critique.** Two critics, one for completeness and one for
   accuracy, reviewed the report draft, and a revision pass fixed what they found.

## What was not verified here

- The live Postgres and Neon suites. They are `#[ignore]`d locally, so CI's
  results were read instead.
- The deployed `workerd` leg.
- The steps the local gate skips when a tool is absent: `cargo hack`,
  `cargo deny`, and the nightly docs.rs builds.
- Git history before 2026-09-07. The clone starts at a squashed import.

Paths in these files that read `<repo>` or `<audit scratch>` stand for the audit's
worktree and its scratch directory. They were not part of the repository.
