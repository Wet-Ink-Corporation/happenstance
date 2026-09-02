---
item: HS-P0011
stage: implementation
created: 2026-08-16T00:19:48.476Z
updated: 2026-08-16T00:19:48.476Z
template_sig: 4c5f37d6
rendered_sig: 96fcc01e
---

# Slice ledger — The typed layer, the worked example, and 0.2.0-alpha.1

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| decision-records | approved | adr-0020-fold-query-agreement c011143, adr-0021-payload-evolution-and-codec-tag e33dc9f | (this commit) |
| typed-vocabulary | approved | domain-event-and-decision-model 996853f, decision-model-composition 4dc6aeb | (this commit) |
| codec-and-command-loop | approved | codec-and-feature-forwarding da530cd, command-loop 9971600 | (this commit) |
| testing-surface | approved | misbehaving-testkit-stores 6c59c46, given-when-then-dsl bd054c0 | (this commit) |
| projection-runner | approved | projection-trait-and-runner 60b8072, projection-clause-verdicts 55a2370, polling-cost-measurement 049d513 | (this commit) |
| worked-example-and-proof | approved | worked-example-on-typed-layer 597a20e, compile-fail-proof-artefact 1044a95 | (this commit) |
| alpha-release | approved | edge-flavour-and-wasm-claim 0a010c9, defect-log-and-macros-verdict 344f2c0, publish-0-2-0-alpha-1 448e1ac | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

## Deviations from the signed-off `_design.md`

Where a shipped surface is not spelled the way `_design.md`'s *Signatures* block spells it. The
design's risk table calls a shape difference *"blocking: escalate rather than adapt silently"*, so
each row here is the escalation: disclosed at the time, carried forward so the next milestone that
reads *Signatures* as binding meets the shipped signature rather than the written one.

| Slice | Item | Design says | Shipped | Why, and what it costs |
| ----- | ---- | ----------- | ------- | ---------------------- |
| codec-and-command-loop | `happenstance::commit`, `happenstance::commit_with` (`crates/happenstance/src/command.rs:221`, `:261`) | `B: Boundary` | `B: Boundary + Clone` | The design's own *Shape decision* requires the loop to re-fold from a pristine model on every retry, and gives `DecisionModel` a `Clone` supertrait for exactly that — but `Boundary` is sealed and carries no `Clone`, so the bound has to be written at the entry points. The alternative, adding `Clone` to the sealed trait, edits M2's frozen surface. It costs a caller nothing: every `DecisionModel` is already `Clone` and tuples of `Clone` are `Clone`. Disclosed in `command-loop/implementation-report.md` and `report.md` claim 4 when it landed; **no code change is owed** — this row exists so `_design.md` and the shipped signature do not diverge silently. |
