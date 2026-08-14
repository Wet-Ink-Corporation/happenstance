---
item: HS-P0010
stage: implementation
created: 2026-08-14T01:25:19.009Z
updated: 2026-08-14T01:25:19.009Z
template_sig: 4c5f37d6
rendered_sig: e468b480
---

# Slice ledger — Freeze ProjectionStore behind a suite that can fail

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| decisions-and-design-record | approved | ps-clause-pairing-sweep f77f183, projection-decision-atoms 9520b28, projection-api-design-record 0df2c1c | (this commit) |
| projection-port-and-probe | changes-requested | owned-batch-port-shape 2eade38, projection-probe-conformance-feature cb495ee, memory-projection-store 5fd62c6 | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### projection-port-and-probe

- `crates/happenstance-core/src/projection.rs:495-499` publishes a `[dev-dependencies]` block for
  `happenstance-core = { features = ["conformance"] }`, which contradicts the orphan-rule paragraph
  three lines above it (:486-491) and cannot work. Because `impl ProjectionProbe for TheirStore` is
  orphan-rule-rejected in the adapter's `tests/` crate, the impl must live in the adapter's `src/`; a
  dev-dependency does not make `ProjectionProbe` exist for the lib build, and Rust cannot `#[cfg]` on
  a dependency's feature. The story spec anticipated this precisely — AC-002 reads "`[dependencies]`
  (not `[dev-dependencies]`)" — and the ledger records the wrong form as satisfying evidence for
  AC-006 ("with the `[dev-dependencies]` cost stated"). This is published guidance on the crate's
  public page, aimed at the exact persona (P2, the adapter author) the project exists to serve.
  Fix: rewrite the snippet to the shape that actually compiles and still demonstrates "one flag on a
  dependency they already have, no new edge": `[dependencies] happenstance-core = "…"` /
  `[features] conformance = ["happenstance-core/conformance"]` /
  `[dev-dependencies] happenstance-testkit = "…"`. Then correct the AC-006 evidence line in
  `.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/_ledger.md:64`
  so the ledger stops asserting the wrong artefact.

- `crates/happenstance-core/src/projection.rs:504-507` states "The falsifier is the story that builds
  an outside author's fixture from the documentation alone, and it is named here" — but no name
  appears. probe AC-006 requires the trait's doc to be "naming `documented-extension-surface` as the
  story that can falsify it". The sentence is self-refuting as written, and the ledger claims the
  requirement met.
  Fix: either name it — "…the falsifier is `documented-extension-surface` (HS-S0015), which builds an
  outside author's fixture from the documentation alone" — or, if naming an internal backlog slug in
  published rustdoc is judged wrong, drop the "it is named here" clause, record the deviation and its
  reason in the ledger's AC-006 row rather than reporting the AC as met.

- **Added by the orchestrator after the run, not by the slice reviewer.**
  `owned-batch-port-shape`'s checkpoint `2eade38` writes
  `crates/happenstance-ladybug/src/stand_in.rs`, which is outside the boundary its own `spec.md`
  declares. Found by `redkiln verify --item HS-S0004 --grain story`, whose `boundary` check reads
  `links.commits`; the slice review did not report it. The consequence is not cosmetic:
  `redkiln advance HS-S0004 --to report` runs `implement`'s command gate on the way out, the gate is
  red, and the story cannot reach its `report` review gate at all — so **HS-S0004 is held at `plan`
  and no human verdict could be recorded against it**, unlike its two slice-mates.
  Settle which of the two it is, and do not paper over it: either the `spec.md` boundary is too
  narrow for work `owned-batch-port-shape` legitimately owns — in which case widen it deliberately
  and say why, as `cc0f158` did for its own widening — or the implementer reached into the ladybug
  skeleton it should not have, in which case the `stand_in.rs` change belongs to a story that owns
  that file. Widening a boundary to match what was written, with no reason given, converts the check
  into a rubber stamp for whatever the implementer happened to touch.
