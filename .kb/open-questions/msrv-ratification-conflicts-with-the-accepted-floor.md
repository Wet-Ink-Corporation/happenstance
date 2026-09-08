---
id: kb-open-question-msrv-ratification-conflict-001
title: A 2026-09-06 ratification recommends lowering the MSRV; the accepted decision says the number does not move
kind: open_question
status: accepted
authority_tier: note
summary: >-
  kb-decision-0037 is accepted, immutable, and titled around the number not moving - 0.2.0
  publishes at 1.97.1, unchanged from ADR-0029, with only its standing (preference to promise)
  converted. Two days after that atom's last review, ratifications-2026-09-06-pre-publication.md
  records msrv-premise ratified on its own recommendation, and that brief's recommendation is
  Option B: lower the floor to 1.95, the workspace's measured maximum. msrv-premise is not among
  the six items 2026-09-07-ratifications-discharged-and-what-execution-changed.md records as
  executed, so the ratification stands unexecuted: Cargo.toml and rust-toolchain.toml both still
  read 1.97.1 as of 2026-09-07. Compounding it, the brief's own bisection puts cfg_select's
  stabilisation at 1.95, not 1.88 - an error that both ADR-0029's and ADR-0037's accepted bodies
  repeat verbatim, and neither can be edited to correct it. What forces a choice is phase 12,
  where the promise actually binds a consumer; the brief itself names 0.2.0 as the only genuinely
  cheap moment to move the number, because raising a published floor is breaking in practice
  however additive the manifest key claims to be.
depends_on:
  - kb-decision-0037
  - kb-decision-0029
related: []
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/msrv-premise.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
  - Cargo.toml
  - rust-toolchain.toml
  - .github/workflows/ci.yml
last_reviewed: 2026-09-07
---

# A 2026-09-06 ratification recommends lowering the MSRV; the accepted decision says the number does not move

## What is true today

`kb-decision-0037` is `status: accepted` and, under this corpus's immutability rule, cannot be edited — only superseded by a new atom. Its own title states the outcome: *"The MSRV becomes a promise at 0.2.0, and the number does not move."* Its decision text is explicit that `1.97.1` is unchanged from `ADR-0029` and that what changes at `0.2.0` is the floor's standing, from a self-imposed constraint to a promise a real consumer is bound by. `Cargo.toml:26` and `rust-toolchain.toml` both read `1.97.1`, confirmed at `HEAD` on 2026-09-07.

Two days before that atom's `last_reviewed` date, `.kb/_intake/ratifications-2026-09-06-pre-publication.md` lists `msrv-premise` among nine briefs "ratified on the brief's own recommendation, without individual review," on the stated ground that a brief which already argued itself out of its first answer under two rounds of critique is a stronger input than a fresh opinion. `msrv-premise`'s own recommendation, after exactly that revision process, is **Option B**: lower `Cargo.toml`'s `rust-version` to `1.95` — the measured floor of every crate in the workspace, `happenstance-sqlite` included — rather than holding `1.97.1` (Option A) or splitting per-package (Option C). The brief's own words for Option A, the one the accepted decision matches: *"holding a number whose entire recorded justification has expired is the failure mode this remediation is named after."*

That ratification has not been executed. `.kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md` records the queue of six items the ratification pass obliged, discharges all six, and `msrv-premise` is not one of them — the queue covers `op-read-non-exhaustive`, `stringified-throw-visibility`, `codec-foreign-tag-resolution`, `empty-decision-outcome` + `tags-scope-agreement` (landed together), and `cf-18`. Nothing in the discharge record mentions the MSRV.

A third, independent fact sharpens the conflict rather than resolving it: `msrv-premise`'s bisection transcript shows `cfg_select` — the macro whose unavailability is the entire stated reason for raising the floor above `1.85` — stabilising at **1.95**, not `1.88`. `.kb/decisions/0029-msrv-raised-to-1-97-1.md` and `.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md` both state `1.88` as the threshold, in bodies that are accepted and immutable. The error is real, verified by a compiler bisection, and sits inside two records that cannot be edited to fix it.

## What is not decided

Whether the 2026-09-06 ratification of `msrv-premise` — reached by the same batch process that ratified eight other briefs the same day — is a decision that obliges superseding `kb-decision-0037`, or whether it is better read as a recommendation that was recorded but never adopted, standing alongside an accepted atom that says the opposite. `open-questions/README.md` names exactly this shape as its first case: *"an unresolved conflict between two sources, or between a source and the shipped code."* Here both sides are sources — one an accepted decision atom, the other a dated ratification record — and the shipped code (`Cargo.toml`, `rust-toolchain.toml`) currently agrees with the atom, not the ratification.

## What forces it

Phase 12, "Publish `0.2.0`," which `RUNBOOK.md` marks `not started`. `msrv-premise`'s own "Cost of delay" section is direct: *"`0.2.0` is the genuinely cheap moment for this and the only one — the promise begins at the stable release."* Once phase 12 lands, raising the floor again costs a real consumer a build even though the manifest key calls it additive only in the raising direction; lowering stays additive forever. So the choice is cheapest exactly at the moment this open question is being recorded, and gets more expensive to reverse the longer it stays unresolved.

## Ordered sub-questions

1. Does the 2026-09-06 batch ratification of `msrv-premise` count as the kind of decision that must supersede an already-accepted atom, or does a ratification reached "on the brief's own recommendation, without individual review" carry less weight than the individually-reviewed decision that produced `kb-decision-0037`?
2. If the ratification stands, does a new atom supersede `kb-decision-0037` at `1.95`, correcting the `cfg_select`/`1.88` error inherited from both `0029` and `0037` in the same change — per this corpus's precedent that a correction is a new atom, never an edit?
3. If `kb-decision-0037` stands instead, should the ratification record be treated as superseded or withdrawn, so a future reader does not find two live, contradictory answers to the same question?
4. Either way: what number does phase 12 actually publish at, given `Cargo.toml` and `rust-toolchain.toml` both still read `1.97.1` and nothing in the discharge queue touched either file?
