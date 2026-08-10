# The phase 4/5 specification reconciliation — pointer and census

## Read this first: extract the pointer, not the evidence

The evidence for everything below lives in **`docs/evaluation/phase-4-5-reconciliation.md`**,
a document written for this pass and committed to the repository. That file is the artifact.
This one is a pointer to it plus a census.

**Extract from the pointer, do not copy the evidence into an atom.** A successful ingest
deletes this file from `.kb/_intake/`, and the evidence must survive that deletion. The
atom this file becomes should be short: what the pass was, what classes of defect it found,
how many of each, which commits carry them, and where to read the detail. If an atom ends up
restating the finding-by-finding detail, the KB now holds a second copy of a 50KB document
that will drift from the first, and the drift will be invisible — which is precisely the
failure this pass existed to repair.

Two sibling documents carry the same story at different altitudes and should be linked, not
absorbed:

- `docs/RUNBOOK.md`, section **"Between 5 and 6 — the reconciliation nothing owned"**
  (the plan's own account, with the standing exit criterion it produced).
- `docs/evaluation/review-citation-drift.md`, written independently the same day.

## Where this expects to land

**Layer: none of the three scaffolded ones.** `product/` wants personas and journeys and
`design/` wants interaction patterns; this repository is a Rust event-sourcing library with
no personas, no screens and no interaction surface at all, so forcing either fit would give
a stock answer the standing of a finding. This atom belongs at the **KB root**, or in a
`reference/` layer if one is created. **Kind: `reference`** — it records a dated,
grounded state of the world rather than a decision or a reusable practice.

`source_paths` should include, at minimum:
`docs/evaluation/phase-4-5-reconciliation.md`, `docs/RUNBOOK.md`,
`docs/architecture/SPECIFICATION.md`, `xtask/src/spec_trace.rs`,
`docs/evaluation/review-citation-drift.md`, and this intake file's own path.

## What the pass was

Six commits on branch `redkiln-adoption`, `3c704d3` through `84dcc67`, on top of `3712c9b`,
each landing with `cargo xtask ci` green. It reconciled `docs/architecture/SPECIFICATION.md`
with the tree that phases 4 and 5 actually produced. No phase owned this work; it is recorded
in the runbook as "Not a phase. A pass that had to happen and that this plan had not
scheduled."

## Census, by class of defect

**Documentation obligations that `[FROZEN]` clauses imposed and the code had never met — nine.**
`3c704d3`. The clauses require specific content in `happenstance-core`'s own doc comments;
nine of those requirements were unmet. ES-23's `# Cancellation` section, ES-24's
at-most-once-under-verbatim-reissue statement carrying all three of its limits, ES-19's
correction, VT-15's NFC/NFD note, VT-17's placement, VT-3 / ES-17's `into_parts`, and ES-40's
completeness note. The same commit removed two code comments that were false: `memory.rs`
claimed "ES-6 is deferred" and `ingest.rs` rested on a premise phase 4 had falsified.
`SPECIFICATION.md` cites this SHA in three places as the commit that landed the correction
(around ES-17 and two later clauses).

**Tests that clauses name and the tree did not contain — five.** `e551cdf`. Each was observed
failing against a named wrong implementation before being written, per CLAUDE.md's rule that a
rule no adapter can fail is decorative. The five, from `git show e551cdf`:
`event_new_accepts_a_held_event_type` (`crates/happenstance-core/tests/constructor_ergonomics.rs:63`),
`command_handler_composes_validation_errors` (same file, line 118),
`append_does_not_accept_a_foreign_identity`
(`crates/happenstance-testkit/tests/foreign_identity.rs:51`),
`query_items_is_not_constructible_downstream`
(`crates/happenstance-testkit/src/lib.rs:263`, a `#[cfg(doctest)]` module) and
`provided_method_future_is_send_in_generic_code`
(`crates/happenstance-core/src/memory.rs:777`).

An earlier draft of this file listed `position_next_signals_overflow` among them. It
pre-dates the pass — `e551cdf` does not touch `event.rs` — and the slip is left recorded
because it is the same failure this whole document is about: a plausible name, a real file,
a real line, and wrong. The full roster is in the evidence
document.

**Citations the checker never parsed — 254 of 338.** `a843b99`. `citations()` in
`xtask/src/spec_trace.rs` required a citation to be path-qualified (`path.contains('/')`)
*and* to name a `.rs` or `.toml` file. Of 338 citations in the document, 84 satisfied both;
200 were bare file names and 56 pointed into Markdown, and neither form was ever parsed.
The step had been reporting "no problems found" over a quarter of the corpus. The widening
also added the coverage number to the summary line. This is the finding generalised in
`lesson-a-check-that-verifies-the-address-not-the-referent.md`.

**Conformance rules claimed by no clause — six.** `52105d2`. Check 6 swept `suite.rs` alone;
widening it to all three entries of `RULE_FILES` (`xtask/src/spec_trace.rs:85-89` — `suite.rs`,
`model.rs`, `concurrency.rs`) found six rules that no clause claimed and no clause retired.
Four were attribution errors: ES-25, ES-19, ES-18 and VT-11 each already stated the
proposition and already named the wrong implementation, and only the rule's name was missing.
The remaining two are genuine holes in the specification and are held in
`UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:1968-1997`). The summary line went from
"89 suite rules" to **"95 conformance rules"**, which is what it prints today.

**Clauses stating things about the code that were false — sixteen.** `89bb966`. Re-tensed.
The runbook names three: `AppendCondition` described as having public fields after VT-30 made
`guards` private; `ReadOptions` described as having no upper bound beside a `to` that had
shipped; `SequencePosition::next` described as `saturating_add` after it became `checked_add`.
The same commit recounted six self-referential numbers the document states about itself,
refreshed §5's inventory for the wire module, and re-anchored roughly 130 citations.

**Citations that resolve and point at the wrong thing — the content anchor.** `84dcc67`.
Existence-and-bounds was never the property worth checking; the property is that a citation
is *about* the thing the sentence attributes to it. The check derives the subject from the
prose and searches a window around the cited range. It found two more defects immediately —
both in citations the sweep in `89bb966` had just repaired into the wrong place. Its design
and the four attempts it took to get there are in
`lesson-anchoring-citations-in-a-long-lived-document.md`.

## The state the tool reports today

`cargo run -p xtask -- spec-trace`, run against the working tree on 2026-08-10:

> 200 clauses (139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 95 conformance
> rules, 58 e2e cases, 358 citations checked (69 anchored to their subject, 12 external)
>
> 2 rule(s) claimed by no clause and owing a decision: …
>
> traceability: no problems found; §7.1–§7.2 matches the checker

Note the two numbers that moved and the one that did not: coverage rose from 84 parsed
citations to 358 checked, but only 69 of those 358 carry a derivable subject and are therefore
anchored to it. **81% of the corpus is still verified for addressing only.** That is the
honest ceiling of a derived anchor and it is stated rather than hidden — see the anchoring
lesson for why raising it by guessing was measured and rejected.

## The independent corroboration

`docs/evaluation/review-citation-drift.md` (dated 2026-08-10, pinned to `3712c9b`) was written
as a byproduct of building `docs/rust/`, with no knowledge of this pass. It found six stale
citations, diagnosed the identical root cause, and recommended the identical remedy — porting
`parse_citation` / `check_citations` from `xtask/src/lint_constitution.rs`, which it estimated
at "about forty lines" (`docs/evaluation/review-citation-drift.md:69`). Five of its six
`SPECIFICATION.md` citations are discharged by this pass, and so is the one into
`docs/adr/0009` — by an incidental out-of-scope edit rather than by a sweep, kept under
ADR-0006's *rewrite the referent, never the reasoning* rule and recorded in the evaluation
document. The other sixteen ADRs carry the same exposure and nothing checks them. Its §4.1 — `query.rs` naming the one
pattern spelling that does not compile — is discharged by `e551cdf`. Its §2 is gap 5 in
`gaps-owed-a-decision.md` and remains open.

**Two independent passes converging on the same root cause and the same remedy is itself the
evidence that the defect is structural rather than incidental.** Record that as the finding,
not as a coincidence.

## What the pass deliberately did not do

It wrote no ADR. Seven findings needed one and were recorded rather than decided. Six are in
`gaps-owed-a-decision.md`; the seventh is
`open-question-nothing-owns-the-post-phase-reconciliation.md`. Two of the six are additionally
held in `UNCLAIMED_PENDING_ADR` so the gate prints them on every green run.
