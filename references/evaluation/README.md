# The evaluation record

The fourteen dated review and research documents [`RUNBOOK.md`](../../RUNBOOK.md)
was derived from. They are kept as **evidence, not instructions**: each records what
was found, when, and against which commit, so a decision the runbook now states in
one line can be traced back to the measurement that forced it.

## Two kinds of document live here, and they have opposite lifecycles

Everything described below this section — the original fourteen — is **immutable
evidence**. It is dated, pinned to a commit, cited by `file:line` from the runbook,
and must be superseded rather than edited.

Four documents added in August 2026 are the opposite. They are **mutable
speculation**, they are pinned to nothing, nothing cites them, and they have already
been revised several times in place. They explore an *application framework* built
above this library — not the library's contract — and they exist to be argued with
and rewritten. Read in order.

| Document | Explores |
|---|---|
| [`research-crux-integration.md`](research-crux-integration.md) | How happenstance and a [Crux](https://github.com/redbadger/crux) core meet: where the store sits, what an `AppendCondition` means across a peer set, what convergence asks of a projection |
| [`research-crux-composition.md`](research-crux-composition.md) | How an application composes without collapsing: vertical slices, the log as the mediator between them, what five other Elm-shaped ecosystems already paid for |
| [`research-crux-stack.md`](research-crux-stack.md) | What the thing is made of, whether Crux should be under it, and the supporting toolchain |
| [`research-crux-layer4-shape.md`](research-crux-layer4-shape.md) | What the framework would feel like to build against, worked against one small application |

Three rules for this second set, and the first one is the load-bearing one:

1. **Nothing in them is an ADR, and nothing in them should become one yet.** No
   decision has been taken. Where they carry a recommendation, it is a recommendation
   with revisit triggers attached, not a settled position.
2. They may be edited freely, unlike the fourteen. Each carries its own revision
   record where an earlier draft was wrong in a way worth remembering — see
   `research-crux-integration.md` §1, which records two corrections and why each
   error was the kind a reader would otherwise repeat.
3. They constrain nothing in `happenstance-core`. Where they identify a gap in the
   contract, the gap is stated as a question for the specification to answer, not as
   a requirement the framework imposes.

## Later additions, which are neither

[`review-citation-drift.md`](review-citation-drift.md) (2026-08-10, pinned to
`3712c9b`) belongs to the first lifecycle — dated, pinned, immutable, supersede
rather than edit — but it is **not one of the fourteen**, and the runbook was not
derived from it and does not cite it. It is a byproduct: defects surfaced while
building [`standards/rust/`](../../standards/rust/README.md), each re-verified directly afterwards,
recorded because the alternative was losing them. It needs no rename
substitution; it was written after `7d6c1b0` and uses today's crate names.

Its §1 is the one worth knowing about from here, because it is about this
directory's own failure mode: six `file:line` citations in `SPECIFICATION.md` and
`.kb/decisions/0009` resolve, pass `spec-trace`, and point at the wrong line.
[`RUNBOOK.md:1713-1716`](../../RUNBOOK.md) predicted exactly that gap; the document
records that it has already recurred since phase 2 closed it, and names the
forty-line check that now catches the same class in `standards/rust/`.

[`ps-clause-pairing-sweep.md`](ps-clause-pairing-sweep.md) (2026-08-13, pinned to
`2136dde`) is the second of these, and carries the same lifecycle — dated,
pinned, **immutable, superseded rather than edited**. It is a byproduct of phase 6
rather than one of the fourteen: a census of every clause PS-1 – PS-37 in
[`spec/SPECIFICATION.md`](../../spec/SPECIFICATION.md) §4, asking of each whether
an implementation exists that satisfies the clause's `MUST` verbatim and fails a
conformance rule the tables assign to it — the question
[`xtask/src/spec_trace.rs`](../../xtask/src/spec_trace.rs) states in its own
header that it cannot answer. It **decides nothing**: seven clauses come out of it
carrying a defect and none is repaired there, because the pass that discovers and
the pass that decides were deliberately kept apart
([`repairing-a-frozen-clause-without-amending-it.md`](../../.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md)).

Its verdict is **isolated** — the hypothesis that §4.11's rule table was populated
from a systematic assumption is not supported — and its headline is that one
supporting sentence of `.kb/open-questions/ps-1-states-no-progress-obligation.md`
does not survive re-derivation. The knowledge-base side of that is now **written**:
the `2026-08-13-projection-adrs` ingest wave consumed
`.kb/_intake/2026-08-13-ps-clause-pairing-sweep.md` and amended both
[`ps-1-states-no-progress-obligation.md`](../../.kb/open-questions/ps-1-states-no-progress-obligation.md)
and [`ps-19-scope-narrower-than-its-rule.md`](../../.kb/open-questions/ps-19-scope-narrower-than-its-rule.md),
each keeping its prior body byte-for-byte and gaining one dated section. Both
questions stay `status: accepted` with their owners unchanged: the sweep answered
the *"check the other 35 for the same shape"* sub-question of each and settled
neither question.

One correction has been made to the document since, and it is the exception the
immutable lifecycle above allows only because it is written down: an **erratum**
at the foot of the document, dated 2026-08-13, correcting the Src column for the
five integration-level rules (PS-26 – PS-30) and for PS-20. It changes no verdict,
no tally and no strength; it exists because a reader re-deriving the verdict from
that column alone would have reached `systematic` where the prose derives
`isolated`.

[`projection-batch-shape-evidence.md`](projection-batch-shape-evidence.md)
(2026-08-14, pinned to `cfd9231`) is the third, and carries the same lifecycle —
dated, pinned, **immutable, superseded rather than edited**. It is a byproduct of
phase 6 rather than one of the fourteen, and the runbook was not derived from it:
it answers one question, *did the two batch shapes disagree, and where?*, off a
single `cargo xtask ci` run in which two conformance harnesses drove the same
sixteen projection rules against `MemoryProjectionStore` and against a buffering,
replay-at-commit store written for the comparison. It carries a disagreement
vocabulary fixed **before** the ledger was filled, and a per-rule ledger covering
the whole of `for_each_projection_store_rule!` rather than the rules the author
remembered.

It **decides nothing**, and that is load-bearing rather than modest: the PS-3
exposure call belongs to `publication-and-positioning` and the freeze verdict to
`ladybug-projection-store`, so a document here that recommended either would be
this project authoring another's decision. Its two substantive results are that
fourteen rules agreed and two differ only in which fixture declines a capability,
and that the axis the pair spans is **narrower** than PS-2's — because
`MemoryProjectionStore` is itself already a deferred write set, which §4.11's
assignment of the CF-5 variant had assumed otherwise. It is cited from PS-3's
clause body in [`spec/SPECIFICATION.md`](../../spec/SPECIFICATION.md), so
`cargo xtask spec-trace` resolves it on every gate run.

[`phase-6-projection-proof.md`](phase-6-projection-proof.md) (2026-08-15, pinned to
`7620481`) is the fourth, and carries the same lifecycle — dated, pinned,
**immutable, superseded rather than edited**. A later run is a later document that
names this one; a correction never lands in place. It is phase 6's **proof
artefact**: the record of one `cargo xtask ci`, run whole rather than `--fast`, on
a working tree with no uncommitted changes, after both of the phase's last two
changes had landed.

It records **nouns**, because the sentence it exists to refuse is
`cargo xtask ci`: green. That sentence is true and is consistent with a
`CheckpointOnlyStore` quietly dropped from the registry, a second batch shape that
is the oracle wearing a hat, and a `wasm32` harness running two rules out of
sixteen. So the document names the conformance rule the deliberately wrong store
fails (`commit_is_atomic_with_the_read_model`) **and** the meta-test that asserts
it fails exactly there — two names, and it says which is which — both fixtures
that pass and how each was observed, every `OPTIONAL` gate step as ran or skipped
(four of four **ran**), and what the run does not cover: the MSRV, `wasm32`
*execution*, and PS-2's bar, which two testkit-side instruments do not clear.

It **decides nothing**, on the same reasoning as the two documents above: no
freeze verdict, no `unstable-projection` exposure verdict, no restatement of the
PS-3 finding, and no ratio over the mutant set in any form (ADR-0010). Three of
its claims are machine-held rather than reviewed — `xtask/src/proof.rs`'s own
`#[cfg(test)]` module reads this document and fails if either test name, either
fixture name or the limits section's two CI jobs are absent from it.

## Every crate name in here is one rename out of date

All fourteen were written against commits `9fd2337` and `2a65d76`, before `7d6c1b0`
executed the rename [ADR-0006](../../.kb/decisions/0006-bare-name-to-the-typed-layer.md) decided.
Read them with the substitution applied: **`happenstance`** means the contract crate,
now `happenstance-core`, and **`happenstance-runtime`** means the typed layer, now
`happenstance`. Six of them — `ARCHITECTURAL-EVALUATION.md`, `PRESSURE-TEST.md`,
`revised-runway.md`, `review-packaging-semver.md`, `review-docs-adr.md` and
`review-dx-ergonomics.md` — also say `happenstance-core`, because each was written
while the rename was still being argued, so both names appear in them and mean one
crate at two different moments. `happenstance-macros` and `happenstance-cloudflare`
were only ever proposals and exist nowhere.

The substitution has one place it must **not** be applied mechanically: a migration
table. `review-packaging-semver.md` enumerates the rename file by file, in rows of
the form `happenstance` → `happenstance-core`, and rewriting the left-hand column
collapses the row into a tautology and destroys the only thing it recorded.

## Why the bodies are not corrected

A rewritten review is no longer the review that was performed. These documents are
dated and pinned to a commit; editing their vocabulary to match today's would make
them agree with a world they were written to criticise, and destroy the evidence
that the criticism came first. The runbook also cites `PRESSURE-TEST.md` by
`file:line` about twenty times, and those citations are load-bearing. This is the
rule ADR-0006 applied to ADR-0005's body and ADR-0007 applied to ADR-0006's:
supersede, never edit. The one permitted exception is repointing a `file:line`
citation at the file it already named — commit `3d43804` did this three times —
because it changes no claim, only whether a reader can follow one.

## None of this is current truth

[`spec/SPECIFICATION.md`](../../spec/SPECIFICATION.md) says what is
true now, clause by clause; [`RUNBOOK.md`](../../RUNBOOK.md) says what happens next
and who settles it. Where either disagrees with a document here, the document here is
the older observation and loses.

The three exploration documents lose to both for a different reason: they are not
observations at all. They describe something that does not exist, built on a
dependency that has not been accepted, to serve an architecture that has not been
spiked. Their value is the questions they sharpen and the mistakes they have already
made in private — not the answers they reach.
