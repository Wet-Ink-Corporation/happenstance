# Wave `2026-09-11-intake` — placement and adjudication

**Fifteen operations**, ordered so that every atom a later operation links to exists before the
link is written. Five staged files, roughly forty claims, and **three cross-file clusters** —
every one of which collapses to a single destination atom. `00-corpus-match.md` establishes why.

**What this wave does to the accepted decision corpus: nothing destructive.** Three decision
atoms are added. **No accepted body is edited, no `status` is flipped on any decision, and no
decision is superseded** — sixth wave running. Two accepted decisions, `kb-decision-0036` and
`kb-decision-0060`, are *discharged* by ADR-0063 on the shape ADR-0037 set against ADR-0004: a
decision conditional on an event is not wrong when the event arrives. One accepted decision,
`kb-decision-0058`, was repaired in place *before* this wave under the referent rule, and the wave
records the repair rather than repeating or reversing it.

**And one thing this wave does that no previous wave has had to:** it mints two decision atoms
for decisions whose evidence is not in the tree it can see. Adjudication 1 is the argument, the
tense discipline the atoms are held to, and the alternative that lost. It is also the first entry
in `unresolved[]`.

## Ordering

```
Op   1      reference atom       — the clocksource evidence, before the decision that rests on it
Op   2      playbook atom        — the method that produced the evidence; links op 1
Op   3      decision atom        — ADR-0064, the host; links ops 1 and 2
Ops  4– 5   decision atoms       — ADR-0062, then ADR-0063, which depends_on it
Ops  6– 7   open-question atoms  — op 6 links ops 4 and 5; op 7 stands alone
Ops  8–15   merges into existing atoms
            ── the Maps phase runs after all fifteen ──
```

ADR-0064 precedes ADR-0062 for no reason of dependency — the two share nothing — but because its
evidence is on disk and its provenance is clean, and the integrator should meet the wave's easy
case before its hard one. ADR-0062 precedes ADR-0063 because 0063's `depends_on` names 0062 and
its own header says *"Acts on: ADR-0062"*. Merges run last because every one writes a link to an
atom created earlier in the same wave. Reciprocal backlinks from new atoms *to* other new atoms
(op 1 → op 3, op 4 → op 6) are the Maps phase's, so no create writes a forward reference to an
atom that does not yet exist.

---

## The operation table

`adr-0004` = `2026-09-08-adr-0004-msrv-becomes-a-promise-at-publication.md` ·
`citation-scan` = `2026-09-08-intake-is-outside-the-citation-scan.md` ·
`host` = `2026-09-09-the-measurement-host-and-its-clock.md` ·
`adr-0062` = `2026-09-10-adr-0062-the-probe-seam-moves.md` ·
`adr-0063` = `2026-09-11-adr-0063-the-projection-port-is-frozen.md`

| # | Op | Kind | Destination | Sources | Class |
| --- | --- | --- | --- | --- | --- |
| 1 | `create_new` | reference | `reference/host-clocksource-tsc-vs-hpet-2026-09.md` | `host` | extends |
| 2 | `create_new` | playbook | `playbooks/a-control-that-can-fire-on-the-instrument.md` | `host` | extends |
| 3 | `create_new` | decision | `decisions/0064-the-measurement-host-has-declared-conditions.md` | `host` | requires-new-decision |
| 4 | `create_new` | decision | `decisions/0062-the-probe-seam-moves-and-the-far-end-is-built.md` | `adr-0062` | requires-new-decision (conflicts with the tree at `HEAD`) |
| 5 | `create_new` | decision | `decisions/0063-the-projection-port-is-frozen.md` | `adr-0063` | requires-new-decision (conflicts with the tree at `HEAD`) |
| 6 | `defer_open_question` | open_question | `open-questions/projection-apply-is-synchronous-against-a-live-store.md` | `adr-0062`, `adr-0063` | extends |
| 7 | `defer_open_question` | open_question | `open-questions/accepted-atom-immutability-check-is-pre-commit-only.md` | `citation-scan` | extends |
| 8 | `merge_existing` | open_question | `open-questions/probe-read-through-signature-and-live-transaction-seam.md` — **resolved** | `adr-0062` | extends |
| 9 | `merge_existing` | open_question | `open-questions/es-7-and-vt-9-provisional-markers.md` | `adr-0062`, `adr-0063` | extends |
| 10 | `merge_existing` | open_question | `open-questions/testkit-projection-module-unstable-projection-exemption-scope.md` | `adr-0063` | extends |
| 11 | `merge_existing` | open_question | `open-questions/cf-33-cf-34-scope-outside-the-testkit.md` | `host` | extends |
| 12 | `merge_existing` | open_question | `open-questions/docs-citation-anchor-form-and-clause-contradiction-check.md` | `citation-scan`, `host` | extends |
| 13 | `merge_existing` | governance | `governance/rewrite-the-referent-never-the-reasoning.md` | `citation-scan` | extends |
| 14 | `merge_existing` | playbook | `playbooks/require-the-property-not-the-mechanism.md` | `adr-0062` | extends |
| 15 | `merge_existing` | open_question | `open-questions/stale-0-0-0-name-reservations.md` | `adr-0004` | extends |

On every `merge_existing` row, **keys not listed in the frontmatter block below are carried
forward untouched.** `KbFrontmatter` is a passthrough object and several of these atoms carry
governance keys redkiln neither owns nor validates; stripping one to make an atom "conform"
breaks every atom that references it, and nothing brings the key back.

**Two citation rules bind every atom this wave writes, and they are the `host` file's own
request applied to all five.** First, every `path:line` is re-anchored against `86a410c`:
CF-34 is `spec/SPECIFICATION.md:9021` (not `:8747`), `CITATION_SCAN_EXCLUDE` is
`xtask/src/lints.rs:2488` (not `:2003` or `:2007`), and `Projection::apply` is
`crates/happenstance/src/domain.rs:249`. Second — and this is new — **no atom cites a line into a
file that is absent from this worktree or into code the brief says has moved and this tree says
has not.** Ops 4 and 5 cite `references/adr/0062-…` and `0063-…` by name only, as *"the long form
the brief names"*, and cite `projection.rs`, `Cargo.toml` and `SPECIFICATION.md` only at the
lines that carry the *before* state.

---

## The adjudications that were not obvious

### Adjudication 1 — two decisions recorded ahead of the tree that binds them

**Ops 4 and 5** · `create_new` · `.kb/decisions/0062-…` and `.kb/decisions/0063-…` · this
wave's most contestable call, and the first entry in `unresolved[]`

**What was verified, and what was not.** `00`'s provenance block has the full list. In one line:
neither ADR's long-form record exists here, `ProjectionStore::begin` is synchronous at
`crates/happenstance-core/src/projection.rs:460`, PS-6's MUST is unrewritten at
`spec/SPECIFICATION.md:5183`, `conformance = ["unstable-projection"]` stands at
`crates/happenstance-core/Cargo.toml:110`, the typed layer still forwards the feature at
`crates/happenstance/Cargo.toml:142`, there is no `LivePostgresProjectionStore` in
`happenstance-postgres`, and the census is 138/46/12/5. `lane/projection-probe-seam` exists, is
the main checkout's branch, and sits at `86a410c` — the same commit as this worktree and as
`main`. Whatever carries the work is uncommitted, and under this wave's hard constraint the other
checkout is not read for authority.

**Three dispositions were available.** *(a)* Hold both files out — mint nothing, leave them in
`_intake`, and let the wave after the lane lands take them. *(b)* Mint both as `status: proposed`,
which the schema permits and the corpus has never used, and flip them later. *(c)* Mint both as
`status: accepted` — a decision record is *history*, and the choice was made — with the tree's
state at `HEAD` stated in the first clause of the summary and the brief's evidence attributed to
the brief.

**The wave takes (c), on the corpus's own stated convention, and records the argument for (a).**

For (c): `.kb/decisions/README.md` says what a decision atom *is* — *"history — why a choice was
made, and when. `spec/SPECIFICATION.md` says what is true now, and where the two disagree, the
specification wins."* ADR-0062 was written on 2026-09-10 and ADR-0063 on 2026-09-11; the choices
were made and the briefs are their owner's statement of them. `kb-open-question-adr-status-vocabulary-001`
records the corpus's convention for a decision the code has not yet voted on — *status accepted,
with the qualification and its falsifier in the first clause of the summary* — applied to nine
imported ADRs and never revisited. And `kb-decision-0037` is the direct precedent: an accepted
atom for a decision whose event had not happened, carrying *"this atom states what `0.2.0` **will**
bind when phase 12 lands, not a fact already true of the alpha"* in its body. The two new atoms
carry the same sentence in the same place, with the tree's line numbers beside it.

For (a), and it is the stronger counter-argument this wave has met: the `adr-0004` file in this
same batch argues, about a different marker, that *"lifting the marker now would put a promise in
the record before the thing that makes it a promise exists"* — and an accepted atom is immutable
the moment it is committed. If the lane never lands as described — if `begin`'s move costs a
buffering adapter a round trip and ADR-0062's own falsifier fires during the lane — the corpus
holds two accepted decisions about a port that never existed, and the only repair is a
supersession pair. That risk is real, it is the reason this is in `unresolved[]`, and the
integrator or the owner can take (a) by dropping ops 4, 5, 6, 8, 9, 10 and 14 from the plan
without touching the rest.

Why (c) still wins here: 0037's shape differs from the `adr-0004` objection in exactly the way
that matters. The objection is to asserting *the event has happened*. Neither atom asserts that.
Each asserts *the decision was taken*, states what the tree at `86a410c` carries, and says the
lane is where the record lands. Nothing in either atom is false at `HEAD` — which is the test
`kb-governance-referent-not-reasoning-001` applies to every edit and the test these two creates
are held to.

**Four authoring constraints follow, and they are not optional.**

1. The first clause of each summary states the tree's state at `86a410c` and that the record is
   the brief's — before the decision is stated.
2. Every count and result — 20 of 20, eight impls, 117 sites, `Ran` on two rules, the census delta
   — is written as *"the brief records …"*, never as a fact this wave verified.
3. `source_paths` names only files that exist here. `references/adr/0062-…` and `0063-…` appear in
   the body as *"the long form the brief names, absent at this worktree's `HEAD`"* and nowhere in
   frontmatter.
4. No `path:line` into the moved code. `projection.rs:460`, `SPECIFICATION.md:5183`,
   `Cargo.toml:110` and `:142` are cited *as the before state*, which is what they are.

```yaml
id: kb-decision-0062
title: The probe seam moves, begin moves with it, and the far end is built
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0062
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
depends_on:
  - kb-decision-0060
  - kb-decision-0036
  - kb-decision-0017
related:
  - kb-open-question-probe-read-through-signature-001
  - kb-open-question-provisional-falsifiers-001
  - kb-playbook-require-the-property-001
  - kb-playbook-repair-frozen-clause-001
  - kb-decision-0025
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/2026-09-10-adr-0062-the-probe-seam-moves.md
  - crates/happenstance-core/src/projection.rs
  - crates/happenstance-core/tests/probe_live_transaction_shape.rs
  - crates/happenstance-postgres/src/projection_store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-11
```

**The sentence a summary would drop, and must not.** *"The phase-10b `sqlx` refutation was a
property of `begin`'s signature, not of the axis."* ADR-0060 read the far end as *unobservable
rather than unbuilt*, and stopped at the probe; ADR-0062 finds that moving the probe alone leaves
a live store able to report itself and still unable to exist for `sqlx`, because `begin` is where
`BEGIN` has to happen. That is why `begin` moves *with* the seam rather than after it, and why
0060's *"not sufficient alone"* is confirmed from the other direction.

**PS-6's falsifier fired and was acted on.** The atom carries this as a distinct paragraph,
because `kb-open-question-provisional-falsifiers-001` is built on falsifiers that fired *without*
consequence or *never could*, and this is the opposite arm: the falsifier named `sqlx`'s `BEGIN`
by description, it was met, and the MUST was rewritten to the property the signature protected
(a buffering adapter's `begin` resolves at its first poll and never fails), now held by two tests
the brief names. Op 14 carries the general lesson.

**`phase: 12` is a judgement call and is recorded as one.** The port is phase 6's; its
re-evaluation was phase 11's (0060); this work is pre-publication, gates `0.2.0`, and is owned by
no numbered phase's checklist. The field on this corpus records the phase that owned the work,
and the twenty-one atoms at `12` are the pre-publication set. `12` is the consistent reading; the
decision map's row should say so.

**`mapsImpact`:** `decisionMap` (a new row, and *"acted on by ADR-0062"* against 0060's row),
`domainMap`. Op 8 carries the `openQuestionIndex` half.

```yaml
id: kb-decision-0063
title: The projection port is frozen, and the typed layer keeps a gate of the same name
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0063
reversibility: low
phase: 12
supersedes: null
superseded_by: null
depends_on:
  - kb-decision-0062
  - kb-decision-0036
  - kb-decision-0060
  - kb-decision-0017
related:
  - kb-decision-0007
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-projection-module-exemption-scope-001
  - kb-open-question-projection-runner-chunk-observation-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/2026-09-11-adr-0063-the-projection-port-is-frozen.md
  - crates/happenstance-core/Cargo.toml
  - crates/happenstance/Cargo.toml
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-09-11
```

**`reversibility: low`, against 0036's `medium`.** 0036 *declined* a freeze, which is reversible
by freezing later — and was. 0063 *makes* one: a semver promise on a published crate that a
consumer can pin to, and reversing it is a breaking change with a decision record behind it,
which is the definition the atom itself gives of *frozen*. `low` is what the corpus uses for
ADR-0017 and the wire-format freezes, and it is the honest field here.

**The two halves a summary would merge, and must not.** The port is frozen **and** the runner
stays gated, and the second is not a hedge on the first. `Projection::apply` is synchronous
(`crates/happenstance/src/domain.rs:249`); a projection that writes rows cannot issue a statement
into a live transaction from inside it; so the live store the port was just proved against is
one the *runner* cannot drive. Freezing `apply` today would freeze a shape proved at one end of
its axis — the exact mistake ADR-0060 refused for the port. The feature keeps its name on both
crates for two different reasons, and the atom states both.

**`mapsImpact`:** `decisionMap` (a new row, and *"discharged by ADR-0063"* against the rows for
0036 and 0060), `domainMap`. Ops 9 and 10 carry the `openQuestionIndex` half.

### Adjudication 2 — the open question is resolved by the decision, not merged into it

**Op 8** · `merge_existing` · `.kb/open-questions/probe-read-through-signature-and-live-transaction-seam.md`
· `status: accepted` → **`superseded`**

`kb-open-question-probe-read-through-signature-001` scores 88 against `adr-0062` — the highest
score in the wave — and the digest's `merge_existing` is right about the destination and wrong
about the direction. The question was *does the signature move, or does the adapter get built
against today's shape?* The brief's header answers in as many words: *"Resolves:
kb-open-question-probe-read-through-signature-001 — the seam moved, at seam grain, and `begin`
with it."* A decision does not merge into the question it closes; the question records the
answer and closes.

The corpus's convention for an answered question is fixed by five prior instances
(`projection-store-in-adapter-default-features`, `projection-store-batch-has-no-apply-seam`,
`ps-1-states-no-progress-obligation`, …): `status: superseded`, **no** `superseded_by` when a
decision rather than another question resolved it, the summary extended with a dated *Resolved*
clause, and a `## Resolved 2026-09-10 — status superseded; …` section appended below the existing
body. The body above the line is untouched: its compiled table, its E0596/E0728 pins and its
*"not sufficient alone"* paragraph are the argument the decision rests on.

**The resolution is wider than the recommendation.** The atom recommended moving
`probe_read_through` alone, then widened to *"the whole probe seam"* last wave. ADR-0062 moves the
seam **and `begin`**, which the atom never proposed — and it is `begin`'s move that lets a
`sqlx` store exist at all. The resolution section says so, because a reader who takes the atom's
recommendation as the fix will under-build.

**The corollary question closes with it.** *Can PS-2's live-transaction bar be judged met by an
adapter that declares the capability false?* No longer arises: the brief records that
`LivePostgresProjectionStore` declares `READS_THROUGH_BATCH = true` as a true statement — the
falsifier `kb-decision-0060` named, met on purpose.

**Same tense discipline as Adjudication 1.** The resolution section states that at `86a410c`
`begin` is unmoved and the long form is absent, and that the decision is recorded on the lane.

```yaml
id: kb-open-question-probe-read-through-signature-001
title: probe_read_through's signature cannot be implemented correctly by a live transaction
kind: open_question
status: superseded                       # was accepted
authority_tier: note
related:
  - kb-decision-0036
  - kb-decision-0060
  - kb-decision-0062                     # added
  - kb-decision-0017
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-ps-19-scope-narrower-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/probe-read-through-and-the-live-transaction-end.md
  - .kb/_intake/2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md
  - .kb/_intake/2026-09-10-adr-0062-the-probe-seam-moves.md                        # added
last_reviewed: 2026-09-11
```

**`mapsImpact`:** `openQuestionIndex` — the bullet flips from *Open* to *Resolved by ADR-0062*,
listed and annotated rather than removed, per the index's own update rule.

### Adjudication 3 — the host brief is three atoms, and the brief asked for two

**Ops 1–3** · `create_new` · reference, playbook, decision

The brief's own framing: *"Two atoms are probably owed rather than one … the decision … and a
reference atom for the clocksource finding."* Both are right, and `decisions/README.md`'s
separation rule — *"the evidence … is a `reference` atom that this one cites"* — is why they are
two rather than one. The wave adds a third and the argument is the brief's own sentence: *"The
transferable lesson is not about clocks."*

**Op 1 — the reference.** A dated, one-host measurement with the exact shape
`kb-reference-busy-timeout-margin-001` has: a headline number (62,493 backwards reads per million
migrations on `tsc`, 0 in 500,000 on `hpet`), the A/B that makes it evidence rather than artefact,
the proportions that give the mechanism its shape (1/16, 1/4, 1/2; CPU0 and CPU1 6.46 ms apart),
the closure (`tsc_adjust` absent; only firmware could fix it), and Conditions. The atom is a
*pointer* to `ops/host/README.md:143-230`, and it carries two things the brief left out that the
README carries: the paired runner's own control **fails** on `hpet` (`instruments_work.rs`'s
`the_paired_sampler_sees_a_difference_it_was_given`, both arms 4,679 ns, both `TIMER-DOMINATED`),
and a pinned-single-CPU regime that would repair it is measured (0 in 500,000, 20 ns timer) and
**not taken**. A reference atom that quoted the 1,390 ns tax without the failed control would be
the kind of summary that reads as *one arm affected* when the harness's own instrument says *the
ratio half of the harness is unusable here*.

```yaml
id: kb-reference-host-clocksource-tsc-hpet-001
title: The measurement host's TSC skews by a fixed per-CPU offset, and hpet costs its timer 73x
kind: reference
status: accepted
authority_tier: note
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
  - kb-open-question-cf-33-cf-34-scope-001
source_paths:
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - ops/host/README.md
  - ops/host/host.env
  - ops/host/probes/tsc-coherence.c
  - ops/host/probes/tsc-pairwise.c
  - ops/host/probes/tsc-migrate.c
  - benchmarks/src/paired.rs
  - benchmarks/tests/instruments_work.rs
last_reviewed: 2026-09-11
```

Backlinks to `kb-decision-0064` and `kb-playbook-control-fires-on-instrument-001` are the Maps
phase's, since both are created after this atom.

**Op 2 — the playbook.** `playbooks/README.md` asks for *a method with a stated claim, the
measurement that supports it, and the conditions under which it stops holding* and *a technique
arrived at by discarding cheaper alternatives, with those alternatives named*. The brief has all
three. **Claim:** a control that can only fire on a machine fault is worth less than one that can
fire on the instrument. **Measurement:** three instruments; the first (sixteen threads against a
shared high-water mark) reported 154,588,122 violations because its read and compare were
separable and it measured its own scheduling latency; the second (a ping-pong handshake) carried
the SMT-sibling control — CPU0 and CPU1 are one physical core reading one counter and *cannot*
disagree — and reported 299,999 of 300,000 on that pair, so it was discarded; the third
(`tsc-migrate.c`, one thread, migrating, comparing consecutive reads) had no lock, no baton, no
shared state, and its A/B distinguished the columns. **Where it stops holding:** when no row of
the instrument is analytically forced — the SMT-sibling row works because the physics forbids a
backwards read there; a control that merely *usually* passes is a threshold, and CF-34 says what
a threshold is. `benchmarks/tests/controls_fire.rs` already encodes the habit for the harness's
*arms*; the brief's point is that it was needed one layer down, in the probe, and *"was not there
until it was needed twice."*

No owner exists. `kb-decision-0010` states the principle for the conformance suite (*a rule no
adapter can fail is decorative*) and `kb-decision-0022` restates it for an experiment's controls;
neither is a method for building the control, and neither names an instrument that was killed by
one. `kb-playbook-assert-execution-not-discovery-001` is the nearest sibling — a check that can
only pass — and goes in `related`.

```yaml
id: kb-playbook-control-fires-on-instrument-001
title: A control that can fire on the instrument, not only on the fault
kind: playbook
status: accepted
authority_tier: guideline
depends_on: []
related:
  - kb-reference-host-clocksource-tsc-hpet-001
  - kb-decision-0010
  - kb-decision-0022
  - kb-playbook-assert-execution-not-discovery-001
  - kb-playbook-verify-referent-report-coverage-001
source_paths:
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - ops/host/README.md
  - ops/host/probes/tsc-coherence.c
  - ops/host/probes/tsc-pairwise.c
  - ops/host/probes/tsc-migrate.c
  - benchmarks/tests/controls_fire.rs
  - benchmarks/tests/instruments_work.rs
last_reviewed: 2026-09-11
```

**Op 3 — the decision.** A commitment with alternatives (prose in a README and a ritual in a
shell history) and two load-bearing properties stated as rules: **one script writes, another only
reads**, and **`--restore` restores captured values, never assumed defaults**. Nothing accepted
covers the machine — `kb-decision-0022` and the two benchmark reference atoms *name* a host's
conditions and decide nothing about what a host must be. The atom has the structure
`kb-decision-0023` and `kb-decision-0058` use: `## Decision`, `## The CF-34 boundary`,
`## Alternatives rejected`, `## What this decision does not settle` — the last carrying the
brief's four explicit non-decisions verbatim in substance, because the second of them (the
Windows-host figures are **not** re-taken) is what keeps two existing reference atoms and four
`[PROVISIONAL]` clauses true.

**The number.** No artefact on disk carries an ADR number for this decision and the brief
proposes none. Wave 5's rule: highest-taken + 1. The two ADR briefs in this wave carry `0062` and
`0063` in their titles; `0064` is next. `0026`–`0028` stay reserved by subject at
`RUNBOOK.md:391-395`.

**`phase: null`.** The corpus's three `null` decisions are the brand set — decisions owned by no
RUNBOOK phase. This is a fourth: the host is infrastructure for every phase's measurement and is
on no phase's checklist. `12` was considered and rejected: the work does not gate `0.2.0`, and the
brief's *"not re-taken"* bullet is the explicit refusal to make it do so.

```yaml
id: kb-decision-0064
title: The measurement host has declared conditions, and its preflight is unreachable from the gate
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0064
reversibility: medium
phase: null
supersedes: null
superseded_by: null
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-host-clocksource-tsc-hpet-001
  - kb-playbook-control-fires-on-instrument-001
  - kb-open-question-cf-33-cf-34-scope-001
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
source_paths:
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - ops/host/README.md
  - ops/host/host.env
  - ops/host/preflight.sh
  - ops/host/00-system.sh
  - ops/host/cpu-tuning.sh
  - ops/host/happenstance-bench-tuning.service
  - xtask/src/affected.rs
  - benchmarks/src/paired.rs
  - benchmarks/src/cpu.rs
  - benchmarks/README.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-11
```

**The CF-34 boundary is in the decision and in the open question, and the two carry different
halves.** The decision carries the *argument* — every value `preflight.sh` reads exists before
the first sample, which is what separates an environment assertion from a budget, and
`paired.rs:71-73` draws the same line from the other side — and the *residual risk* in the
brief's words: a self-hosted runner would make the preflight reachable from a merge-blocking job,
and *"that is a decision, not a configuration change."* Op 11 carries the *instance*: a third
clock-adjacent assertion outside the testkit, and the first built to be unreachable, for the open
question that collects them.

**`mapsImpact`:** `decisionMap`, `domainMap` — and the domain map has **no section** for
measurement, benchmarks or the host today. The Maps phase decides whether ops 1–3 and 11 justify
one or land under an existing heading; this plan does not pre-empt it.

### Adjudication 4 — the deferral two ADRs make in the same words is one open question

**Op 6** · `defer_open_question` · `.kb/open-questions/projection-apply-is-synchronous-against-a-live-store.md`

This is the wave's cross-file dedup, and the two digests disagreed about it. The `adr-0062`
digest routed it to *"`.kb/decisions/0017-…` (annotate sub-question 3, no new atom warranted)"*;
the `adr-0063` digest routed it to a new open question. The first is not available and the second
is right.

Not available: `kb-decision-0017` is accepted and immutable, and its *"sub-question 3"* is a
pointer into `kb-open-question-projection-batch-no-apply-001` (superseded), whose third
sub-question asks whether closing the batch question *retroactively validates ADR-0006's
encoding-versus-orchestration discriminator*. That is a question about where the runner lives.
This is a question about what `apply`'s signature can drive. Different question, and the
superseded atom is not a home for a live one.

Right: the deferral is made twice, by two consecutive ADRs, in nearly the same words —
`adr-0062`: *"whether `apply` moves is the typed layer's axis, not this port's … the next
record's subject"*; `adr-0063`: *"freezing `apply` now would freeze a shape proved at one end of
its axis, which is the mistake ADR-0060 refused for the port."* A deferral made twice with no
owner is exactly what `open-questions/README.md` calls *"a deferral, where a decision was possible
but was consciously postponed, and what would force the choice."*

**What is true today** (at `86a410c`): `Projection::apply(&mut self, event: Self::Event)` is
synchronous at `crates/happenstance/src/domain.rs:249`; the runner in `crates/happenstance/src/runner.rs`
folds events through it and hands the batch to the store; a projection that writes rows can push
into an owned buffered batch and cannot `await` a statement into a live transaction. **What is
not decided:** whether `apply` moves (to `async`, to a batch-handle parameter, or not at all),
and whether the runner's `unstable-projection` gate — which the brief says now gates the runner
alone — comes off with it. **What forces it:** the first projection an application needs to run
against `LivePostgresProjectionStore` through the runner rather than through the probe, or a
decision to publish the runner ungated. **Ordered sub-questions:** does `apply` need to change at
all, or is a live store an *instrument* for the port's freeze and never a runner target (the
brief's own framing); if it changes, does the batch shape (`&mut Self::Batch`, now `async` at the
probe seam) become `apply`'s parameter; and does 0063's falsifier — *"`apply` moving to a shape
that requires the port to move with it"* — fire on any of those answers.

```yaml
id: kb-open-question-apply-synchronous-live-store-001
title: Projection::apply is synchronous, so the runner cannot drive the live store the port was frozen against
kind: open_question
status: accepted
authority_tier: note
depends_on: []
related:
  - kb-decision-0062
  - kb-decision-0063
  - kb-decision-0060
  - kb-decision-0017
  - kb-decision-0007
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-projection-runner-chunk-observation-001
  - kb-open-question-provisional-falsifiers-001
source_paths:
  - .kb/_intake/2026-09-10-adr-0062-the-probe-seam-moves.md
  - .kb/_intake/2026-09-11-adr-0063-the-projection-port-is-frozen.md
  - crates/happenstance/src/domain.rs
  - crates/happenstance/src/runner.rs
  - crates/happenstance/Cargo.toml
last_reviewed: 2026-09-11
```

**Same tense discipline.** The atom's *"What is true today"* is true of `86a410c` and says so;
the runner's gate *"no longer forwards to the contract crate"* is the brief's statement about
the lane and is attributed.

**`mapsImpact`:** `openQuestionIndex`.

### Adjudication 5 — ADR-0036 and ADR-0060 are discharged, and neither is superseded

**Op 5**, and the second-most contestable call after Adjudication 1.

Both `kb-decision-0036` (*"not frozen at `0.2.0`, ships behind `unstable-projection`"*) and
`kb-decision-0060` (*"keeps its gate, and the reason has expired"*) are accepted, and ADR-0063
freezes the port. Read as bare verdicts, 0063 reverses both, and the reading that would supersede
them is not unreasonable.

It is wrong, and the brief says so in its header: *"Resolves the decision of ADR-0036 and
ADR-0060 — both said gate the port; this record says the reason is discharged. Neither is
superseded in its reasoning, which was right when written."* Three things follow, each checked.

**Both decisions were conditional, and the condition is named in the clause they applied.**
PS-3's SHOULD reads *"**Until PS-2's bar is met** the port SHOULD ship behind an off-by-default
feature"* (`spec/SPECIFICATION.md:5056`). 0036 evaluated that bar and found part 2 unmet; 0060
found part 2 *unmeetable through the port's own signatures* and kept the gate on that ground.
ADR-0062 met the bar. A conditional decision whose condition arrives has run its course; it has
not been found wrong.

**Supersession is for correction, and nothing is corrected.** `decisions/README.md`: *"To
correct one, write a new atom carrying `supersedes`."* 0036's finding — one adapter, one end —
was true on 2026-09-02. 0060's finding — the far end forbidden by `begin`'s signature — was true
on 2026-09-08 and is *why* ADR-0062 moved `begin`. Marking either `superseded` would tell a later
reader the finding was mistaken, which is the lie `kb-governance-referent-not-reasoning-001`'s
ADR-0002 instance names.

**The precedent is ADR-0037 over ADR-0004, twice run.** ADR-0004 said the floor is a preference
*until first publish*; ADR-0037 said the preference has become a promise; both stay accepted and
byte-identical, and the map row carries the lineage. The 2026-09-04 wave's Adjudication 10 is the
record and this wave applies it without variation: `supersedes: null` on 0063, `superseded_by:
null` on 0036 and 0060, zero changed bytes in either predecessor, `depends_on` naming both, and
two *"discharged by ADR-0063"* annotations on the decision map.

**What is deliberately not done:** a governance merge for this shape. The `adr-0063` digest
proposed one. `kb-governance-referent-not-reasoning-001` already carries the ADR-0037 lineage in
`related` and generalised the *decision-stands-reason-expires* case last wave; a
*decision-conditional-condition-met* paragraph would be a third restatement of the same test, and
the atom's own summary is already 480 words. The decision map row is the record.

### Adjudication 6 — the citation-scan file's first finding has an exact owner

**Op 12** · `merge_existing` · `.kb/open-questions/docs-citation-anchor-form-and-clause-contradiction-check.md`
· sources `citation-scan` **and** `host` (its *"Note to the ingest"*)

`kb-open-question-docs-citation-anchor-contradiction-001` scores 82 and its **sub-question 2**
is the question the file answers, nearly verbatim: *"Given the postscript's measurement, is the
`.kb/_intake/` scan exclusion re-examined now rather than waiting for a future ingest, and does
the fix look like scanning intake after all, a pre-ratification repointing step, or something
narrower?"* The file: *"Decided at review: re-anchor at promotion. The owner chose repair 2."*
The merge does four things the atom cannot currently say.

**The predicted event arrived, at the moment the postscript said was not the one that matters.**
The postscript argued the load-bearing moment is *ratification*, not ingest. The 2026-09-07 wave
showed ingest is *also* one: `kb-decision-0058:33` inherited three ranges from a 2026-09-04 brief,
one landed on a blank line, and `cargo xtask ci` was red on `main` from `6acdf24` until the
`0.2.0` closeout session's first command. `a4616ca` had repointed fifteen citations the merge
moved and correctly touched nothing under `.kb/` — nothing there was in scope yet.

**Sub-question 2 has an answer, and the answer is not this repository's to implement.** Repair 2
— `/redkiln:kb-ingest` resolves every `path:line` against `HEAD` as it authors, and refuses the
*wave* rather than the atom when one does not — lives in the redkiln plugin, whose backlog
already carries `fix-kb-ingest-defects`. `.redkiln/` here holds config, packs, templates and
telemetry, and no workflow. The alternative *not* taken, scanning `_intake` as a warning, was
declined by name: it *"puts the check on the wrong side of the boundary"* — this repository
watching for that tool's defect on every run, forever.

**The interim is a habit, and the atom names it.** Until the fix lands, run `cargo xtask ci`
immediately after an ingest wave merges rather than trusting the pre-merge green. The wave that
caused the red was the last commit on `main`, and nobody ran the gate after it.

**Two of this wave's own files drifted while staged, in the section that talks about drift.**
`citation-scan` cites `lints.rs:2003`, `host` cites `:2007`, the constant is at `:2488`; `host`
cites CF-34 at `:8747`, which is now inside CF-38's `Rule`. None would redden the gate —
`check_citation` passes an in-range line in a non-Rust target — which is the milder half of what
the atom already records about `citation_ranges_resolve`. Recorded as a fifth instance, not a
new finding.

The question does **not** close. Sub-questions 1 and 3 stand, the fix has not landed, and the
hole is open until it does. `status` stays `accepted`.

```yaml
id: kb-open-question-docs-citation-anchor-contradiction-001
title: A docs/ page cited a real clause and said the opposite of it, and nothing mechanical would have caught that
kind: open_question
status: accepted
authority_tier: note
related:
  - kb-open-question-rustdoc-citation-form-001
  - kb-playbook-anchoring-citations-001
  - kb-reference-intake-citation-drift-census-001         # added
  - kb-decision-0058                                      # added
  - kb-open-question-immutability-check-pre-commit-001    # added
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/docs-citation-form-and-clause-content.md
  - .kb/_intake/2026-09-08-intake-is-outside-the-citation-scan.md    # added
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md     # added
  - xtask/src/lints.rs                                                # added
last_reviewed: 2026-09-11
```

**Why not the census.** `kb-reference-intake-citation-drift-census-001` scores 78 and is the
atom that *predicted* this. It is a dated measurement at two named commits and stays one;
`reference/README.md`'s dating rule has held for eight waves and holds here. The census is added
to the OQ's `related` so the prediction and its arrival are one hop apart.

**`mapsImpact`:** `openQuestionIndex`.

### Adjudication 7 — the second finding is "worth an atom of its own", and it is

**Op 7** · `defer_open_question` · `.kb/open-questions/accepted-atom-immutability-check-is-pre-commit-only.md`
**Op 13** · `merge_existing` · `.kb/governance/rewrite-the-referent-never-the-reasoning.md`

The file: *"What the attempt revealed is worth an atom of its own: `redkiln validate --kb`'s
immutability check compares the working tree against `HEAD`."* `CLAUDE.md:216` records the same
finding with its transcript — *"accepted decision 'kb-decision-0042' was edited in place"* on an
uncommitted change, silence once committed — and adds the consequence the file does not: a CI
checkout's working tree **is** `HEAD`, so the disabled `backlog` job could never have fired on
this half and would not if switched on tomorrow. No `.kb` atom carries either.

**Why an open question and not governance.** The *rule* (supersede, never edit) is governance
and already an atom. The *instrument's reach* is a fact, and *what to do about it* is undecided:
the file names two remedies — hash the accepted body into the atom's own frontmatter so history
is what is compared, or a documented carve-out for referent-only edits — and `CLAUDE.md` names a
third, a diff of every `status: accepted` body against the merge base. Three candidates and no
choice is `open-questions/README.md`'s *"a known gap the work uncovered and did not close."*

**What is true today** (at `86a410c`): the check is a dirty-tree guard; it refuses an
uncommitted edit to an accepted body and passes the moment the edit is committed; it cannot tell
a referent repair (`4e13ee2`, three line ranges) from a reversal; the `backlog` CI job that would
run it is `if: false` (`.github/workflows/ci.yml:243`) and, enabled, would run it against a tree
equal to `HEAD`. Its honest reach is *pre-commit*, not *pre-merge*. **What is not decided:** which
of the three remedies, and whether a referent-only carve-out can be stated mechanically or only
as the governance atom's judgement test. **What forces it:** the next referent repair to an
accepted atom (the guard will refuse it locally and then not notice it), or the next attempt to
restore the `backlog` job (which would restore nothing on this half). **Sub-questions:** does the
corpus want the check to catch a committed reversal at all, given that the governance atom's
test is a judgement and not a byte comparison; if yes, is the comparison base the merge base or a
hash carried in frontmatter; and does a carve-out need the lint to know what a citation is.

```yaml
id: kb-open-question-immutability-check-pre-commit-001
title: The accepted-atom immutability check is a dirty-tree guard, and it cannot tell a referent repair from a reversal
kind: open_question
status: accepted
authority_tier: note
depends_on: []
related:
  - kb-governance-referent-not-reasoning-001
  - kb-open-question-references-adr-correction-policy-001
  - kb-open-question-docs-citation-anchor-contradiction-001
  - kb-decision-0058
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/2026-09-08-intake-is-outside-the-citation-scan.md
  - CLAUDE.md
  - .github/workflows/ci.yml
  - .kb/decisions/README.md
last_reviewed: 2026-09-11
```

**The governance merge is the fifth worked instance, and the first in which the rule met its
own enforcement.** `kb-governance-referent-not-reasoning-001` has four instances: rewritten in
place (crate names), left verbatim (ADR-0002), metadata flip (ADR-0006), reason expires
(ADR-0036/0060). The fifth is the smallest and the sharpest: `kb-decision-0058:33` cited
`event_store.rs:1229-1296`, line 1229 was blank, and at `4e13ee2` the three ranges were repointed
to `:1396-1431`, `:1447-1462`, `:1474-1490` with not one word of reasoning moved. A line number
is a referent in the atom's own sense — the edit does not change what the document asserts — and
the `adr-0004` file in this batch draws the same line from the other side (*"unlike a drifted line
number, it genuinely is the case the immutability rule is for"*). What the instance adds: the
repair was refused by the check that enforces the rule, on the uncommitted tree, and passed once
committed — so the discrimination this atom states is one the instrument cannot make, and the
open question above is where that goes. And one rejected alternative the atom does not yet
name: superseding 0058 to fix the citation would mark a correct, unreversed decision `superseded`
**and would not fix the gate**, because `citation_ranges_resolve` reads every atom under `.kb/`
regardless of status. The broken one would stay red beside its replacement.

```yaml
id: kb-governance-referent-not-reasoning-001
title: Rewrite the referent, never the reasoning
kind: governance
status: accepted
authority_tier: guideline
related:
  - …(existing sixteen entries carried forward)…
  - kb-decision-0058                                      # added
  - kb-open-question-immutability-check-pre-commit-001    # added
source_paths:
  - …(existing eight entries carried forward)…
  - .kb/_intake/2026-09-08-intake-is-outside-the-citation-scan.md   # added
last_reviewed: 2026-09-11
```

**`mapsImpact`:** op 7 `openQuestionIndex`; op 13 none.

### Adjudication 8 — a file that asks not to be ingested, and the one sentence it leaves behind

**Op 15** · `merge_existing` · `.kb/open-questions/stale-0-0-0-name-reservations.md` · source `adr-0004`

The file's first line is a refusal, and its second paragraph is the argument: `kb-decision-0037`
*"records the identical proposition — and records it with the correct wiring, which this brief
gets wrong."* The extraction digest verified that independently and found it holds: 0037 is
accepted, phase 12, an amendment lineage with `supersedes: null`, and its body carries every
clause of the file's *"thing worth carrying into that atom"* section. Five of the digest's five
claims scored 85–100 against 0037 with `merge_existing` proposed — and a merge into an accepted
decision's body is the one edit this corpus does not have. **No decision-layer operation.**

**One sentence is not in the corpus, and it is the one the header says the file is kept for.**
*"Measured: `max_stable_version` is `0.0.0` for all seven crates, nothing yanked"* — the
2026-09-08 registry read that `CLAUDE.md`'s MSRV constraint was corrected against.
`kb-open-question-stale-0-0-0-name-reservations-001` owns registry state, its *"What is true
today"* was read on 2026-09-03 and 2026-09-04, and it has **five** crates at `0.0.0`
(`happenstance-postgres` and `happenstance-neon` were reserved after it was written). The merge
adds one dated paragraph: as of 2026-09-08, seven at `0.0.0`, three additionally at
`0.2.0-alpha.1`, nothing yanked, `max_stable_version` reading `0.0.0` everywhere — so no consumer
is pinned to anything, `0.2.0` is a first real release for all seven, and the registry semver
baseline cannot run until the tag lands. The question stays open; its forcing event (phase 12) is
unchanged.

```yaml
id: kb-open-question-stale-0-0-0-name-reservations-001
title: Whether the live 0.0.0 name-reservation releases should be yanked before a real 0.2.0 ships
kind: open_question
status: accepted
authority_tier: note
related:
  - kb-open-question-adapter-version-lockstep-001
  - kb-decision-0037                                      # added
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - .kb/_intake/remediation-2026-09-04-briefs/repository-url-and-security-channel.md
  - .kb/_intake/2026-09-08-adr-0004-msrv-becomes-a-promise-at-publication.md   # added
  - CLAUDE.md                                                                   # added
last_reviewed: 2026-09-11
```

**After this merge the file holds nothing the corpus does not.** Its header says *"retire this
file once the tag lands"*, but what it wanted kept until then is now in the open question, and
what it proposed is already 0037. The wave recommends the clearing step take it with the other
four; the call is the orchestrator's and is in `unresolved[]`.

**`mapsImpact`:** `openQuestionIndex`.

### Adjudication 9 — the smaller merges, each in one paragraph

**Op 9** · `open-questions/es-7-and-vt-9-provisional-markers.md` · `adr-0062`, `adr-0063`. Last
wave added *"PS-2's live-transaction end is forbidden by `begin`'s total, synchronous, infallible
signature … while thirteen `[PROVISIONAL]` clauses wait on PS-2 alone."* Both halves have moved.
ADR-0062 met the bar by moving the signature rather than the marker — a falsifier that *"never
could fire"* was made fireable, which is a third arm the atom's observation did not have — and
PS-6's own falsifier fired and was acted on. ADR-0063 records that the *thirteen on PS-2 alone*
was the RUNBOOK ledger's simplification and not the clauses': PS-4, PS-5 and PS-12 stood on PS-2's
axis and freeze with it; PS-9, PS-11, PS-15 and the rebuild cluster stand on their own
falsifiers and do not. One dated section; `status` unchanged (ES-7 and VT-9 are untouched);
`related` gains 0062 and 0063. `mapsImpact`: `openQuestionIndex`.

**Op 10** · `open-questions/testkit-projection-module-unstable-projection-exemption-scope.md` ·
`adr-0063`. The question is scoped to *ADR-0036's exemption* and asks whether the testkit should be
inside it. ADR-0063 removes the exemption from the port: there is nothing to be inside. What the
brief says about the testkit — a third `xtask` test holds that *no in-tree crate still forwards
the retired feature*, so the manifest line the atom's *"What is true today"* rests on is gone on
the lane — is the brief's and is unverifiable here (`crates/happenstance-testkit/Cargo.toml` still
forwards at `86a410c`). So: annotate, do not resolve. One section stating that the question's
premise dissolved with the exemption, that `ProjectionFixture` is now a committed surface on a
*frozen* trait rather than on a moving one, and that closing the atom waits on the manifest
being readable. `related` gains 0063. `mapsImpact`: `openQuestionIndex`.

**Op 11** · `open-questions/cf-33-cf-34-scope-outside-the-testkit.md` · `host`. The atom collects
clock-adjacent assertions outside the testkit and has two. `preflight.sh` is a third, and the first
that is built to be unreachable from the gate: every value it reads exists before the first
sample, nothing in the step table, the `verify:` block or CI invokes it, and `ops/` is on `INERT`
(`xtask/src/affected.rs:448`, held by `:999`). It is also the first with a *named* path to becoming
a violation — a self-hosted runner — and the brief's sentence that this would be *a decision, not
a configuration change* is what the atom carries. CF-34 re-anchored to `:9021`. `related` gains
0064 and the reference. `mapsImpact`: `openQuestionIndex`.

**Op 14** · `playbooks/require-the-property-not-the-mechanism.md` · `adr-0062`. The playbook's one
instance is an `xtask` guard that required a literal glob by name rather than the property it
stood for. PS-6 is the same shape from the specification's side: *"`begin` MUST be neither `async`
nor fallible"* named a mechanism — a signature — standing proxy for a property: a buffering
adapter's `begin` makes no round trip, resolves at its first poll, and never fails. When the
mechanism had to move (ADR-0062), the property was held by two tests the brief names —
`begin_makes_no_round_trip` over a transport that fails every request, and
`begin_resolves_at_its_first_poll_without_a_runtime`, polled once with no executor — and the MUST
was rewritten to say the property. One section, *"A second instance, in a clause rather than a
guard"*, attributed to the brief and dated. `related` gains 0062. `mapsImpact`: none.

---

## Claims deliberately given no operation

`01` §F has the table. The two worth repeating here because they are the ones a hurried
integrator would add back:

- **Nothing merges into `kb-decision-0037`.** Five digests proposed it. It is accepted, and the
  file says it is already there.
- **Nothing cites a line into `references/adr/0062-…` or `0063-…`, or into the moved code.** The
  files are absent and the code has not moved here. An atom that cited them would pass the
  citation lint — a missing target returns `None` — and be wrong, which is the failure the lint's
  own doc comment names as *"a documented gap rather than a pass."*
