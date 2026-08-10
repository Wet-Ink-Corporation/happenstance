# The evaluation record

The fourteen dated review and research documents [`docs/RUNBOOK.md`](../RUNBOOK.md)
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

## Every crate name in here is one rename out of date

All fourteen were written against commits `9fd2337` and `2a65d76`, before `7d6c1b0`
executed the rename [ADR-0006](../../.kb/decision/0006-bare-name-to-the-typed-layer.md) decided.
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

[`docs/architecture/SPECIFICATION.md`](../architecture/SPECIFICATION.md) says what is
true now, clause by clause; [`docs/RUNBOOK.md`](../RUNBOOK.md) says what happens next
and who settles it. Where either disagrees with a document here, the document here is
the older observation and loses.

The three exploration documents lose to both for a different reason: they are not
observations at all. They describe something that does not exist, built on a
dependency that has not been accepted, to serve an architecture that has not been
spiked. Their value is the questions they sharpen and the mistakes they have already
made in private — not the answers they reach.
