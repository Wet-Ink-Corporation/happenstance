# Staged: the `happenstance-macros` verdict

**This is raw material, not an atom.** `redkiln validate --kb` skips
`_`-prefixed directories by design, so nothing here is held to `KbFrontmatter`.
The frontmatter below is **proposed** — a starting point for
`/redkiln:kb-ingest` to author from and adjudicate.

```yaml
# PROPOSED — for the ingest wave to adjudicate, not to copy verbatim.
proposed_kind: decision
proposed_title: happenstance-macros is out of scope for 0.1
proposed_status: accepted
proposed_authority_tier: decision
source_paths:
  - references/evaluation/phase-7-macros-verdict.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/defect-log-and-macros-verdict/spec.md
related:
  - .kb/decisions/0020-fold-query-agreement.md    # confirm slug at ingest
pinned_to: 78a2170c1d06bad5eec34915b0b3682f524ec91f
date: 2026-08-16
```

## The one thing the wave must not do

**Do not edit ADR-0020's atom.** `RUNBOOK.md:525`'s decision-table row names
0020 in its ADR column, which makes an edit the obvious move and the wrong one:
ADR-0020 is an **accepted** decision atom and therefore immutable, validation
checks each one against `HEAD`, and ADR-0020's own spec assigns AC-013's verdict
to *"a later obligation owned by the project's closeout… not settled here"*
(`adr-0020-fold-query-agreement/spec.md:323`).

Whether this claim becomes a **new** decision atom or a **supersession** of
ADR-0020 is the wave's adjudication to make from the material below. It is not
this document's, and it is not a hand-edit either way.

## The claim

**`happenstance-macros` is out of scope for 0.1.** The rewritten worked example
does not carry more mapping boilerplate than domain logic; it carries roughly
half as much at worst and an eighth as much at best.

## Why, in the form that makes it checkable

The criterion is mechanical: *if the rewritten example carries more mapping
boilerplate than domain logic, the derive is in scope* (`RUNBOOK.md:4079-4083`).
More means the ratio exceeds one; exactly 1.0 is out.

The judgement is therefore not in the threshold — it is entirely in the
**classification**, which is why the classification is published line range by
line range in the long form and the totals are derived by summing the rows.

**Substrate:** `examples/course-subscriptions/src/main.rs` at `78a2170`, 532
lines, as `worked-example-on-typed-layer` left it. Checked plain before
counting: no `macro_rules!`, and `impl DomainEvent for Enrolment` written out by
hand. Not the design's doctest, which `_design.md:1110-1111` forbids
substituting.

**Result:**

| Bucket | Lines |
| --- | ---: |
| ceremony (`impl DomainEvent`, `:209-247`) | 40 |
| domain | 249 |
| neither (`main`, transcript, imports, call plumbing) | 158 |
| contested (`CourseId` / `StudentId`, `:98-182`) | 85 |

The 29 published ranges are contiguous and exhaust the file: 532 of 532.

| Contested assigned to | ceremony : domain | verdict |
| --- | --- | --- |
| ceremony | 125 : 249 = **0.50 : 1** | out |
| domain | 40 : 334 = **0.12 : 1** | out |

Both extremes agree, so the verdict stands and the contested block is a footnote
(EC-007).

## The prediction it contradicts, named

`_design.md:1104-1111` predicted **in**, at 2.4 : 1, over its own doctest. The
example returns 0.50 : 1 at worst — about a factor of five the other way. **The
contradiction is the point**: the design wrote the number down as falsifiable so
that this measurement could falsify it, and a wave that reconciles the two by
preferring the prior has undone that.

**Why they differ**, which is the durable part and the reason this is worth an
atom rather than a note: the `DomainEvent` impl is a **fixed cost** that barely
grows with the domain — 26 lines for two variants, 40 for three — while the
domain grows with the number of consistency concerns, refusals and handlers. So
the ceremony ratio is a function of how much domain an artefact contains, and a
minimal doctest measures it at the one point where there is almost none. **Both
numbers are true and they answer different questions.** A first-program page
should still be held to 2.4 : 1 (that is AC-U01); a scope decision should not be
taken on it (that is AC-013).

## What a derive would have bought, and the condition that could reopen this

40 lines of 532 — **7.5%** of the file — against a fourth published crate and a
proc-macro in every consumer's build graph.

**Reopen if, and only if, defect C2 is settled with an infallible `Tags` path.**
A derive that also handled *tags from runtime values* would reach into the
contested 85 lines rather than only the 40 — but that is a materially larger
derive than the one this criterion asked about, and its precondition is the
`DomainEvent::tags` defect staged in `contract-defect-log-phase-7.md`. Even
then, an 85-line swing does not reach the threshold from 0.50 : 1, so it stays a
post-0.1 question.

## Consequences already carried out

- No `crates/happenstance-macros/` was created. An **out** verdict escalates
  nothing: `_decomposition.md:428` routes only an *in* verdict to the runbook as
  a scope change.
- `RUNBOOK.md`'s phase-7 session log cites the long-form record; the macros exit
  box is ticked; the `:525` decision-table row moves off `open` to the verdict.
- `publish-0-2-0-alpha-1` is unblocked: it depends on the **record**, never on a
  crate.
