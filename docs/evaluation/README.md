# The evaluation record

The fourteen dated review and research documents [`docs/RUNBOOK.md`](../RUNBOOK.md)
was derived from. They are kept as **evidence, not instructions**: each records what
was found, when, and against which commit, so a decision the runbook now states in
one line can be traced back to the measurement that forced it.

## Every crate name in here is one rename out of date

All fourteen were written against commits `9fd2337` and `2a65d76`, before `7d6c1b0`
executed the rename [ADR-0006](../adr/0006-bare-name-to-the-typed-layer.md) decided.
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
