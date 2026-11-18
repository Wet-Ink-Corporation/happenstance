---
id: kb-decision-0113
title: A declined capability is printed past libtest's capture and listed in each adapter's README
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0113
reversibility: high
phase: 21
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-18 for #145, and not yet accepted: the owner's yes is pending. CF-18 has two
  residuals that declension by inheritance (ADR-0051) left open. First: a default cargo test still
  prints no SKIP line, because RuleOutcome::report uses println!, which libtest captures for a
  passing test. This record does not accept that gap and does not narrow CF-18's frozen Rejects
  prose. RuleOutcome::report instead writes the SKIP line through a locked io::stderr() handle,
  which libtest's capture does not intercept. That is additive testkit behaviour, verified red
  first. If the verification fails, the fallback is to accept the gap. Second: each published
  adapter's README lists the capabilities its conformance fixtures decline, with each reason, as a
  convention stated where adapter authors read. It is not a specification clause. The README lines
  for three in-tree adapters land with this proposal, because they are additive. Cloudflare's
  README says its fixture declines nothing, and that fixture declines READ_FAULT; its correction
  lands on acceptance with the publish-readiness test that checks it. ES-35 stays renewed past 1.0 and is out of scope.
depends_on:
  - kb-decision-0051
  - kb-decision-0066
related:
  - kb-open-question-cf-18-residuals-after-declension-001
  - kb-decision-0042
source_paths:
  - crates/happenstance-testkit/src/contract.rs
  - crates/happenstance-cloudflare/README.md
  - crates/happenstance-sqlite/README.md
  - crates/happenstance-postgres/README.md
  - crates/happenstance-neon/README.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-18
---

# A declined capability is printed past libtest's capture and listed in each adapter's README

**Proposed on 2026-11-18 for #145, and awaiting the owner.** The README lines under Decision 3 have
landed on the proposing branch, because they are additive and need no specification change.
Nothing else has been applied. This atom is the whole record.

## Context

CF-18 (`spec/SPECIFICATION.md:9418`) is `[FROZEN]`. It requires an unmet capability's rule to be
emitted as a test that **reports** the skip with the fixture's reason. Its `Rejects:` paragraph
turns on the reason landing "in the adapter's CI log where a reviewer and a user of the adapter can
both see it" (`:9427-9432`). Two residuals remain (`.kb/open-questions/cf-18-residuals-after-declension-by-inheritance.md`).

**Residual 1, visibility.** `RuleOutcome::report` writes the line with `println!`
(`crates/happenstance-testkit/src/contract.rs:1624-1628`). Its own rustdoc records that libtest
discards a passing test's stdout unless the run passes `--show-output` (`:1609-1610`). A stranger
who runs `cargo test` on an adapter sees green and no `SKIP` line. Since ADR-0051,
`every_declined_capability_is_stated_by_this_fixture` runs in the adapter's own binary
(`SPECIFICATION.md:9443-9446`). That proves a reason exists. It does not show the reason to anyone.

**Residual 3, disclosure.** ADR-0066 defines *conformant* as *"passes `happenstance-testkit`
X.Y"*, stated in the README. No adapter README listed what its fixtures decline. The one with a
capability table, Cloudflare's, was wrong: it says "The fixture declines nothing, so the
conformance run prints no `SKIP` line" (`crates/happenstance-cloudflare/README.md:154`), while
`CloudflareFixture` declines `READ_FAULT` (`crates/happenstance-cloudflare/tests/support/mod.rs:258`).
This is residual 3's hazard showing up in this repository, not in a stranger's.

## Options

**Residual 1.**

- **A. A second channel, the SKIP line written through a locked `io::stderr()` handle (chosen,
  pending verification).** libtest's capture works through the output-capture hook that the
  `print!` and `eprint!` family consult. A write to the `Stderr` handle does not consult it. This
  is the reading to verify. It was not executed here: the session's hook refused a scratch Rust
  file, and the toolchain has no `rust-src` to read. The implementer verifies it red first, as
  below. The lines interleave with libtest's progress output, which is cosmetic. It is two-way.
- B. Accept the gap by record. The reason is reachable under `--show-output`, in the fixture's
  source, and in the README (Decision 3). It is free and honest, and it leaves CF-18's named victim
  in place for anyone who never opens the README.
- C. Narrow CF-18's `Rejects:` prose to "reachable under a flag". This is a frozen-clause change
  that weakens the promise. One-way.
- D. Emit the skipped rule as `#[ignore = "…"]`. Not available: the attribute needs a literal at
  expansion, and the reason is an associated `const` the macro cannot read. It would also put
  `#[ignore]` into every adapter's binary, which the house standard treats as a red flag.

**Residual 3.**

- **E. Each published adapter's README carries a capability table: every capability its
  conformance fixtures decline, and the reason (chosen).** The convention is stated in
  `docs/adapter-reading-order.md`, where an adapter author reads. Two-way.
- F. A specification clause in §6.6 obliging it. That puts a documentation duty on third parties
  through a frozen clause, which no rule can check.
- G. An `xtask` lint that matches README tables against fixture `const`s. It would need a Rust
  parser over `tests/support`. Brittle, and deferred until the convention has drifted once under
  review.
- H. No obligation. Cloudflare's README is the evidence against this.

## Decision

1. **Residual 1 is not accepted.** `RuleOutcome::report` writes its `SKIP` line with
   `writeln!(io::stderr().lock(), …)`. It handles the write's `Result` by ignoring only a closed
   pipe, because a report must never fail a passing rule, and it says so in a comment (H-10). The
   wasm emitter is unchanged, because it already goes through `console_log!`. CF-18's MUST,
   marker and `Rejects:` do not move.
2. **Verification gates Decision 1.** First, red: run a default `cargo test` on
   `happenstance-sqlite`'s conformance target, which declines `MID_BATCH_FAULT` and `READ_FAULT`,
   and record that no `SKIP` line appears. Then green: the same command shows both. If libtest
   captures the handle write after all, Decision 1 falls back to option B, and the record says so
   when it is accepted.
3. **Residual 3: option E.** The four in-tree adapter READMEs list their declines now:
   - `happenstance-sqlite`: `MID_BATCH_FAULT` (by scope) and `READ_FAULT` (incapacity).
   - `happenstance-postgres` and `happenstance-neon`: `READ_YOUR_OWN_WRITES` on the event store,
     and `RESET_REFUSAL` on the projection stores.
   - `happenstance-cloudflare`: `READ_FAULT` (by scope), with the false sentence corrected, on
     acceptance rather than with this proposal. `tests/publish_readiness.rs`'s
     `the_conformance_claim_carries_every_qualifier_the_run_produces` compares the README's
     no-`SKIP` sentence against only three of the fixture's capabilities (`SECOND_HANDLE`,
     `REOPEN`, `MID_BATCH_FAULT`), so correcting the README alone fails that test. The repair
     widens the test to every capability the fixture declares, `READ_FAULT`, `REMOVAL`,
     `READ_YOUR_OWN_WRITES` and `HELD_ABOVE_FRONTIER` among them, with each declined reason
     carried verbatim in the README. That is residual 3's hazard a second time: the check that
     guards the sentence had the same blind spot as the sentence.
4. ES-35 stays `renew-past-1.0` (`kb-decision-0066`). Sub-question 2 of the atom is already
   answered.

## What acceptance changes

- **`happenstance-testkit`:** `RuleOutcome::report` as in Decision 1, and its rustdoc's "Two
  honest limitations" rewritten. A unit test drives a `report_to<W: Write>` seam, so the line's
  text is checked without stdio. The uncaptured property is the manual check in Decision 2,
  recorded in the PR. Ships in the next testkit minor.
- **`spec/SPECIFICATION.md`:** CF-18's non-normative paragraph saying `RuleOutcome::report`
  "writes to a stdout libtest discards" is updated to describe the handle write. This is prose
  only, and the admitted set is unchanged.
- **`docs/adapter-reading-order.md`:** one paragraph stating the README capability-table
  convention, pointing at Cloudflare's table as the model.
- **`.kb/open-questions/cf-18-residuals-after-declension-by-inheritance.md`:** closed, citing this
  record.
- #145 closes.

## Door

Two-way. No frozen text moves, and both mechanisms can be reverted in a minor.
