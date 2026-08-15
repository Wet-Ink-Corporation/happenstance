# Staged for `/redkiln:kb-ingest` — ADR-0030 and the two open questions it closes

**Wave hint:** `2026-08-15-adr-0030-checkpoint-progress`. Date-prefixed and
slug-suffixed so a later wave staging the same subject cannot overwrite this
file's audit trail — the convention the `2026-08-13-projection-adrs` and
`2026-08-13-ps-clause-pairing-sweep` waves used.

**Source of record:**
[`references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md`](../../references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md)
— the full decision record, with the compiler-free evidence, the exposing
implementation and the three rejected alternatives. This file stages the
*atom-side* content only. Nothing under `.kb/` was hand-written by the commit that
carries it; atoms are `/redkiln:kb-ingest`'s to author (CLAUDE.md, *Where the work
lives*; the first attempt at hand-writing them was reverted at `0269720`).

---

## 1. A new decision atom — ADR-0030

**Title:** The checkpoint reports the commits that happened
**kind:** `decision` · **status:** `accepted` · **adr_id:** `ADR-0030` ·
**phase:** 6 · **reversibility:** low · **supersedes:** null

**Summary material.** §4 obliged a `commit` to *couple* its two writes and never
to *advance* anything. PS-1's `MUST` is satisfied by a store that makes neither
write durable — the *"or not at all"* arm — while three rules
(`commit_advances_the_checkpoint`,
`commit_accepts_a_position_the_batch_did_not_write`,
`commit_rejects_a_regressing_position`) and a fourth that does not exist yet
(`fresh_projection_has_no_checkpoint`) all assert progress. The exposing
implementation is a store whose backing state lives per **handle** rather than per
store, which is the fixture bug CLAUDE.md's *"one fixture instance is one isolated
backing store"* exists to forbid, and it is `independent`: no other `PS` `MUST`
rejects it.

The decision mints **PS-38**, `[PROVISIONAL]`, in §4.7 — *a successful `commit`
MUST advance `id`'s checkpoint to `position`, and a `ProjectionId` no successful
`commit` has named MUST read as `Checkpoint::NeverRun`* — falsified by a store
answering `checkpoint` from a replica that may lag its own `commit`. PS-1, PS-19,
PS-21 and PS-22 are **byte-identical** across it; each gains a recorded finding.
Rejected: adding a sentence to `[FROZEN]` PS-1 (a gap, not a repair); splitting
PS-23's *"exactly one"*, which is the only sentence that entails progress today
and does so incidentally on a clause scheduled to be rewritten by an unrelated
falsifier; routing it to phase 7, which PS-1's own paragraph forbids; and folding
all seven of the sweep's defective rows into one record, which is four questions
too many for one ADR title.

**Depends on:** `kb-decision-0007`, `kb-decision-0017`, `kb-decision-0018`.
**Related:** `kb-decision-0019`, `kb-open-question-ps-1-no-progress-obligation-001`,
`kb-open-question-ps-19-scope-narrower-001`,
`kb-playbook-repair-frozen-clause-001`, `kb-playbook-one-decision-per-adr-title-001`.
**source_paths:** this file; `references/adr/0030-…md`;
`references/evaluation/ps-clause-pairing-sweep.md`; `spec/SPECIFICATION.md`.

---

## 2. Two open questions close, and one sentence in each is amended rather than deleted

### `.kb/open-questions/ps-1-states-no-progress-obligation.md`

**Closes**, resolved by ADR-0030. The resolution is *"a clause of its own"*, which
is one of the three candidates the sweep left open; the record names why the other
two lost. The amendment the sweep already staged stands and is repeated here so
the closing atom carries it: the atom's *"stated by no clause's `MUST` anywhere in
the document"* is **not** exactly right — PS-23 states it, incidentally and on a
`[PROVISIONAL]` clause about fan-out scope. The obligation was **misfiled**, and
that is what turned *"add a sentence to PS-1"* into *"put it where the rules can
cite it"*.

### `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md`

**Closes**, resolved by the same record. PS-19 keeps its scope — *after a
successful `reset`* — and the unseen-id half becomes PS-38's second sentence.
`fresh_projection_has_no_checkpoint` is listed against both clauses in §4.11 and
is still unwritten, which is why it renders `†`. Its sub-question 2 (*check the
other 35 `PS` clauses for the same shape*) was already answered **isolated** by
the sweep and by the wave that ingested it; nothing here reopens that.

### `.kb/maps/open-questions-index.md`

Both rows move from open to resolved, each naming ADR-0030.

---

## 3. Four findings that stay open, recorded so the closing waves do not absorb them

These are the sweep's other defective and undetermined rows. Each is now written
into the specification clause it is about, with its exposing implementation, its
strength and its owner. **None of them is ADR-0030's**, and a future wave should
not fold them into it.

| Clause | Shape · strength | Where it now lives | Owner |
|---|---|---|---|
| PS-8 | S1 · dependent — the `MUST` binds the method's *existence*, the rule its *behaviour* | the clause's own recorded finding | ADR-0017's range; the repair is a further decision |
| PS-13 | S2 · dependent — the rule asserts the converse of the `MUST` and runs in a suite where the projection is conformant by construction | the clause's own recorded finding | ADR-0017's range; and §4.11's write-behind/`READS_THROUGH_BATCH` tension |
| PS-28 | S1 · **undetermined** — turns on what *"the last good position"* means, which the rule does not say | the clause's own recorded finding | resolved by defining the phrase when the rule is written (typed-layer phase) |
| PS-29 | S1 · independent — *"without being polled for it"* reaches past *"observable through the API"* | the clause's own recorded finding | ADR-0019 defers the observability design to the typed-layer phase |

**PS-32's correction is still owed and is still not performed.** ADR-0007's
Context says a callback-driven pump *"cannot be written against the port as it
stands — in either crate"*; it can, and was compiled (PRESSURE-TEST §3.4). ADR-0007
is accepted and immutable, so the correction is a **superseding atom's** and never
an edit (`.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59`), and
[ADR-0017](../../references/adr/0017-what-a-projection-batch-owns.md) records it as
owed and states its shape at `:356-361`. Phase 6's clause disposition changes
nothing about that and says so in the clause. A wave that writes the superseding
atom should carry: what cannot be written against the port as it stands is the
**conformance suite**, not the runner, and §4.3 is where the distinction lives.

**Candidate open question, raised and not opened here** (the sweep's inverse-shape
observation, `ps-clause-pairing-sweep.md:433-454`): PS-3, PS-31 and PS-36 share
one shape — *a documentation obligation with no instrument*. The repository
already carries the instrument that would close all three
(`standards/rust/70-rustdoc-obligations.md` and the design record's
`## Visibility and stability` block). Worth an atom; not opened by this story,
which had no mandate to.

---

## 4. One reference-layer fact worth an atom, or a line in an existing one

**`spec-trace`'s `has_suite` is a per-family switch, and a family absent from it
is a family whose rule citations nothing checks.** `PS-` was excluded when the
projection suite did not exist; the exclusion outlived it by two slices, and until
it was flipped a `PS` clause could cite a rule that had never existed with every
gate in the repository green. The general shape — *an exclusion written for a true
reason, kept after the reason expired, and structurally invisible because the
thing it disables is a check* — is the same defect
`.kb/open-questions/es-7-and-vt-9-provisional-markers.md` records one level up
about maturity markers. Flipping it is now held by
`spec_trace::tests::the_projection_family_is_checked_against_its_suite`
(`xtask/src/spec_trace.rs`), and the family that still abstains — `SY` — is held
by the test beside it, so widening the switch to everything is a build failure
rather than a judgement call.
