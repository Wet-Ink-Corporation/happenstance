# Wave `2026-09-21-intake` — placement and adjudication

**Five operations**, ordered so that every atom a later operation links to exists before the link
is written. One staged file, four claims, and no cross-file cluster to collapse — the
de-duplication this wave owes is *cross-layer* rather than cross-file, and `01`'s closing section
states it.

**What this wave does to the accepted decision corpus.** One decision atom is added, and it
**supersedes `kb-decision-0022` in part, at §11 only**. **No accepted body is edited and no
`status` is flipped on any decision** — seventh wave running, and this is the first wave in which
that sentence is a *choice* rather than a description, because a staged file asked in writing for
the frontmatter spelling that would have required the flip. Adjudication 1 is that argument.

**The wave's second call, and the one most likely to be contested:** a `create_new` against a
`reference` sibling scoring 66, where `merge_existing` is the documented default. Adjudication 2.

## Ordering

```
Op 1   create_new      reference      — the evidence, before the decision that rests on it
Op 2   create_new      decision       — ADR-0065; links op 1; supersedes kb-decision-0022 IN PART
Op 3   merge_existing  open_question  — the falsifiers-fired atom; links op 2
Op 4   merge_existing  open_question  — the contention-tolerance atom; links ops 1 and 2
Op 5   merge_existing  open_question  — the status-vocabulary atom; links op 2 (discretionary)
        ── the Maps phase runs after all five ──
```

Op 1 precedes op 2 because `decisions/README.md` puts the evidence outside the decision and the
decision cites it by id; a decision created first would carry a forward reference to an atom that
does not exist. Ops 3–5 run last because each writes a link to an atom created earlier in the
same wave. The **reciprocal backlink** — op 1's `related` gaining `kb-decision-0065`, and the two
named open questions appearing in the new atoms' `related` — is the Maps phase's, so no `create`
in this wave writes a forward reference.

---

## The operation table

| # | Op | Kind | Destination | Sources | Class |
| --- | --- | --- | --- | --- | --- |
| 1 | `create_new` | reference | `reference/busy-timeout-adapter-cap-sweep-2026-09.md` | `§11` | extends |
| 2 | `create_new` | decision | `decisions/0065-adr-0022-s11-busy-timeout-is-fifteen-seconds.md` | `§11` | requires-new-decision (conflicts with `kb-decision-0022` §11) |
| 3 | `merge_existing` | open_question | `open-questions/adr-0022-falsifiers-have-fired.md` | `§11` | extends |
| 4 | `merge_existing` | open_question | `open-questions/no-fixture-tolerance-for-transient-contention.md` | `§11` | extends |
| 5 | `merge_existing` | open_question | `open-questions/adr-status-vocabulary-exceeds-the-schema.md` | `§11` (by consequence, not by citation) | extends |

`§11` = `.kb/_intake/2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md`

---

## Op 1 — the adapter cap sweep

`create_new` · `.kb/reference/busy-timeout-adapter-cap-sweep-2026-09.md` · sources `§11` ·
claim D1–D5 · **mapsImpact: `domainMap`**

```yaml
id: kb-reference-busy-timeout-adapter-cap-sweep-001
title: The cap sweep on the adapter that ships — 5,000 ms goes red 7 launches in 8, 15,000 ms 0 in 16
kind: reference
status: accepted
authority_tier: note
summary: >-
  The measurement taken 2026-09-21 from
  experiments/busy-timeout-margin/results/adapter-cap-sweep.md, of what red rate
  happenstance-sqlite actually has at three candidate values of BUSY_TIMEOUT_MS. It is the
  sibling of kb-reference-busy-timeout-margin-001 and not a re-run of it: that page measured how
  close per-contender waits run to the cap, on experiments/append-condition's measurement
  candidate, with a counting busy handler; this one measures how often the suite therefore goes
  red, on crates/happenstance-sqlite's own concurrency target, unmodified, and reports one bit
  per launch - the bit an adapter author experiences. Running the real adapter is what closes
  caveat 1 of experiments/busy-timeout-margin/README.md, the caveat the sibling atom correctly
  still carries. At --test-threads=1, which serialises the rules so that all 64 contenders get
  the whole machine and is the worst case this host can produce: 5,000 ms went red in 7 launches
  of 8; 15,000 ms in 0 of 16; 30,000 ms in 0 of 8. The 5,000 ms cell was run a second time in a
  parallelism control and went red 8 of 8, so 15 of 16 between the two runs, and both are
  reported rather than averaged. The control is the other half of that table: at libtest's
  default parallelism - what cargo xtask ci actually runs - 5,000 ms went red 1 of 8, the same
  direction as the sibling's finding that fewer simultaneously-runnable contenders measure
  safer. Raising the cap costs nothing when nothing is contended: passing-run target durations
  were 4.91-5.65 s at 5,000 ms and 4.99-5.40 s at 15,000 ms, indistinguishable on a shared host,
  because the busy handler returns the instant the lock is acquired and the cap bounds only the
  tail. Every failure was a liveness failure - committed was correct in every row, and the two
  rules that went red are the two that require every contender to commit. Conditions, because
  none of it is portable - the same Windows i9-13905H host as every other page in that
  experiment, not the declared-conditions Linux host of kb-decision-0064, debug build as the
  gate runs it, CONTENDERS at the shipped 64, happenstance-sqlite at 0.3.2, host shared. What
  this cannot show: one bit per launch discards the margin, a launch clearing the cap by 1 ms
  and one clearing it by 14 s being the same row; one host; and it does not show the
  contended-versus-broken conflation is fixed, because it is not.
depends_on: []
related:
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
  - kb-reference-one-connection-latency-001
  - kb-decision-0022
  - kb-decision-0064
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-testkit-contention-tolerance-001
source_paths:
  - .kb/_intake/2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md
  - experiments/busy-timeout-margin/results/adapter-cap-sweep.md
  - experiments/busy-timeout-margin/README.md
  - experiments/busy-timeout-margin/results/busy-timeout-margin.md
  - crates/happenstance-sqlite/src/connection.rs
  - crates/happenstance-sqlite/tests/concurrency.rs
last_reviewed: 2026-09-21
```

Body sections, mirroring `kb-reference-busy-timeout-margin-001`'s shape so the two read as a
pair: *"What this is a pointer to"* (and how it differs from its sibling) · *"The sweep"* ·
*"The control, and why `--test-threads=1` is the worst case"* · *"What the raise costs when
nothing is contended"* · *"Conditions"* · *"What this does not show"*.

**The conclusion is not in this atom.** `reference/README.md`: *"A conclusion drawn from the
measurement … the number goes here; what the project decided because of it is a `decision` atom
that cites this one."* The atom states the red counts and stops; ADR-0065 is where fifteen
seconds is chosen.

---

## Op 2 — ADR-0065

`create_new` · `.kb/decisions/0065-adr-0022-s11-busy-timeout-is-fifteen-seconds.md` · sources
`§11` · claims A1–A10 · **mapsImpact: `decisionMap`, `domainMap`**

```yaml
id: kb-decision-0065
title: ADR-0022 §11 is superseded in part — the busy timeout is fifteen seconds
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0065
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  Partly supersedes ADR-0022 at section 11 only, leaving sections 4, 6, 7, 9, 10, 12 and 15
  untouched and standing - the shape kb-decision-0007 used against ADR-0006 and kb-decision-0031
  against ADR-0007, which is why supersedes stays null and kb-decision-0022 stays accepted.
  Section 11 set busy_timeout at 5,000 ms and rested the value on a premise - "busy = 0 in every
  row of the 64-contender table, so the timeout did real work and never ran out" - and set its
  own re-open condition on that premise: re-open if any run ever reports busy > 0. The premise is
  false. kb-reference-busy-timeout-margin-001 recorded busy > 0 at the shipped CONTENDERS = 64,
  one launch in seven, three attempts in 6,720, the first nonzero busy count in this tree, with a
  margin of 1.31x-1.38x on the plateau and every wait figure a lower bound. The decision:
  crates/happenstance-sqlite/src/connection.rs's BUSY_TIMEOUT_MS is 15_000. Measured rather than
  reasoned, and measured on this adapter rather than on the experiment's candidate, which closes
  that experiment's own first caveat - at --test-threads=1, the worst case the host can produce,
  5,000 ms went red in 7 launches of 8, 15,000 ms in 0 of 16 and 30,000 ms in 0 of 8
  (kb-reference-busy-timeout-adapter-cap-sweep-001). The argument that decided it over leaving
  the cap alone is that raising it is free on the healthy path: the handler returns the instant
  the lock is acquired, so the cap bounds the pathological tail rather than delaying every
  append, and passing runs took 4.91-5.65 s at five seconds against 4.99-5.40 s at fifteen. What
  it is paid for by is named: a genuinely stuck writer now takes 15 s rather than 5 s to report,
  a rare path that ends in a red rule rather than a hang, because the cap stays finite. Three
  alternatives are rejected in terms. Ratifying section 11 as still correct - defensible, since
  what fired was liveness and not semantics, committed being correct in every row - loses because
  the cost is paid by the wrong person: a roughly 1-in-7 red at the shipped contender count tells
  an adapter author their store is unsound when it is not. 30,000 ms is equally clean here, buys
  nothing measurable over 15,000, and triples the pathological wait, and the core sweep's
  fewer-cores-is-better result (about 450x from twenty cores to one) is what makes fifteen
  defensible off this host rather than lucky. An unbounded handler stays rejected for ADR-0022's
  original reason, restated rather than assumed: CF-33 is [FROZEN] and forbids the conformance
  suite a watchdog, so this constant is the only liveness bound in the system and an unbounded
  wait converts a livelock into a hung job naming no rule. CONTENDERS is untouched - section 12
  says the contender count is not that record's to re-open, and it is not this one's either. The
  replacement falsifier names the instrument the old one lacked, which is why the old one was
  unfalsifiable for as long as it existed: re-open this cap if the adapter's own concurrency
  target goes red on a SQLITE_BUSY at 15_000 on any host in any configuration, reproducing with
  the cap sweep's loop; re-open it downward if the pathological 15 s wait is ever what makes a
  real failure undiagnosable. Three things are deliberately not settled: section 9's firing,
  which still owes the twenty-line reproduction its own wording asks for; sections 8 and 16,
  whose falsifier cannot fire as written; and the conflation of a contended store with a broken
  one, which raising the cap makes rarer and cannot remove, since CF-33 denies a rule the clock
  that would tell them apart - owned by kb-open-question-testkit-contention-tolerance-001, whose
  blocking instrument now exists.
depends_on:
  - kb-decision-0022
related:
  - kb-reference-busy-timeout-adapter-cap-sweep-001
  - kb-reference-busy-timeout-margin-001
  - kb-decision-0064
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-testkit-contention-tolerance-001
  - kb-open-question-cf-33-cf-34-scope-001
  - kb-open-question-adr-status-vocabulary-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/busy-timeout-margin/results/adapter-cap-sweep.md
  - experiments/busy-timeout-margin/results/busy-timeout-margin.md
  - experiments/busy-timeout-margin/README.md
  - crates/happenstance-sqlite/src/connection.rs
  - crates/happenstance-sqlite/README.md
  - crates/happenstance-testkit/src/faulty.rs
  - crates/happenstance-testkit/tests/contended_store_instruments.rs
last_reviewed: 2026-09-21
```

Body sections: *"Decision"* · *"The scope of the supersession"* (Adjudication 1's argument, in
the atom, so a later reader does not have to find this wave folder) · *"Why this is forced"*
(the premise, the firing, and `kb-governance-what-may-refute-a-finding-001`'s bar) ·
*"Alternatives rejected"* (ratification, 30,000 ms, the unbounded handler, and `CONTENDERS` as
out of scope) · *"The replacement falsifier, and its instrument"* · *"What this does not
settle"*.

Two notes for the integrator:

**`phase: null`, not `phase: 12`.** The open question this resolves says *"Forced by phase
12."* Phase 12 has happened — `happenstance-sqlite` is at `0.3.2` on the registry and the sweep
records that version as its subject — so this decision is taken *after* the phase that forced it,
not within it. `kb-decision-0064` is the precedent for `null` on work no numbered phase owns.
Writing `12` would claim the decision was made inside the phase, which is the kind of small
false fact this corpus spends whole atoms undoing.

**`reversibility: medium`**, matching `kb-decision-0022`. The constant is a one-line change, which
argues `high`; but `crates/happenstance-sqlite/README.md:128` now documents it as an adapter
property of a published crate, which is what the open question said phase 12 would make of the
pragma set. `medium` is the honest reading of both halves.

---

## Op 3 — the falsifiers-fired open question is amended, not closed

`merge_existing` · `.kb/open-questions/adr-0022-falsifiers-have-fired.md` · sources `§11` ·
claims B1–B4 · **mapsImpact: `openQuestionIndex`** · `status` **unchanged**

`kb-open-question-adr-0022-falsifiers-fired-001` scores 96 and is unambiguously the owner. One
dated section, appended after *"What forces it"*, in the corpus's own convention for an amended
question:

- **§11 is decided, by the fourth move this atom itself named.** The atom's *"What is not
  decided"* offers three moves and then records a fourth, proposed for §8: *"supersede it **in
  part**, items 1 and 2 only … narrower than superseding ADR-0022 and wider than ratifying it,
  and shows that 'which of three moves' was itself too coarse a question."* ADR-0065 applies that
  move to §11. The amendment says so, and notes the irony that the sentence proposing it lists
  §11 among the sections left untouched, because it was written about §8.
- **The routing question this atom leaves open is half-answered.** It asks *"who owns the
  testkit-facing contention-tolerance question section 11's firing raises, versus who owns
  section 9's runtime-seam correction."* The first half now has an answer:
  `kb-open-question-testkit-contention-tolerance-001` owns it, ADR-0065 says so in terms, and
  op 4 records that its blocking instrument exists. The second half is untouched.
- **§9 and §8/§16 stand, and the amendment states why each stands separately** — §9 owes a
  reproduction the tree does not contain; §8/§16's repair runs through a `[FROZEN]` clause
  (ES-27's `Rejects:` prose) and is therefore a decision, not a documentation sweep.
- `related` gains `kb-decision-0065` and `kb-reference-busy-timeout-adapter-cap-sweep-001`;
  `source_paths` gains the intake file and the sweep page; `last_reviewed` → `2026-09-21`.

**The title is not changed, and that is a deliberate call rather than an oversight.** It reads
*"Two of ADR-0022's falsifiers have fired, a third cannot fire as written, and nobody has
re-opened"*, and the final clause is now false for one of the three. `open-questions/README.md`:
*"Do not rewrite a question into its own answer: the value of the record is that it shows the
state of knowledge on the day the choice was made."* The dated section sits directly under the
question it corrects, and the `openQuestionIndex` bullet carries the annotation so the map does
not advertise a stale clause. Carried to `unresolved[]` so a human can overrule it, because the
argument for a retitle — findability — is real.

---

## Op 4 — the contention instrument exists, and it has already discriminated the arms

`merge_existing` · `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md` ·
sources `§11` · claims C1–C4 · **mapsImpact: `openQuestionIndex`** · `status` **unchanged**

`kb-open-question-testkit-contention-tolerance-001` scores 93, and the atom's final substantive
paragraph is a *specification of the missing instrument*: *"a decorator over `MemoryEventStore`
whose `append` refuses the first `m` callers with a transient store error; it discriminates the
two arms as well as gating them, because a per-error channel and a per-fixture capability behave
differently against it."* `SendFaultyStore::contend_next` is that decorator, to the letter.

One dated section. Its centre is **C4, not C1**, and the difference matters enough to state:

- **The gate is open.** `faulty.rs:333`/`:429` add `contend_next(n)`, refusing the next *n*
  appends as `AppendError::Store(FaultyStoreError::Contended)` (`:556`). Landed 2026-09-21. The
  atom's *"Both live arms are gated by the same missing instrument"* stops being true on that
  date.
- **The atom's prediction has been tested and held, and the result is an asymmetry.**
  `a_retry_loop_gated_on_the_dcb_signal_never_retries_a_busy_store`
  (`tests/contended_store_instruments.rs:290`) shows the port offers a caller exactly one
  classifier, `AppendError::is_condition_violated`, and it answers `false` for a busy store
  exactly as it does for a broken one. So the **per-error arm cannot be built in the testkit at
  all** — it needs a contract change first — and the per-fixture arm can. The test file states it
  as *"a fact about the two designs, not a preference between them"*, and that is the form it
  goes into the atom in. This is the half of the merge the digest did not carry, and it is the
  half that moves the question.
- **The three-rules claim is now executed rather than reasoned.**
  `two_rules_reject_a_store_that_is_merely_contended` (`:346`) pins today's behaviour —
  including the part the file itself calls arguably wrong — and
  `the_same_fixture_passes_the_rule_when_it_is_not_contended` (`:390`) is its negative control.
  The atom's *"a tolerance is therefore a change to what those rules assert rather than one arm
  on a private enum"* now has a test behind it.
- **The question does not move.** Fixture-side, contract-side, or no tolerance is still open, and
  ADR-0065 narrows only the *rate* at which the conflation bites. CF-33 denies the clock, so no
  value of `BUSY_TIMEOUT_MS` can close this. The sweep page says the same thing independently in
  its third *"What this does not show."*
- `related` gains `kb-decision-0065` and `kb-reference-busy-timeout-adapter-cap-sweep-001`;
  `source_paths` gains the intake file, `crates/happenstance-testkit/src/faulty.rs` and
  `crates/happenstance-testkit/tests/contended_store_instruments.rs`; `last_reviewed` →
  `2026-09-21`.

**What the merge must *not* do:** soften the atom's *"Nothing at all remains an arm: a suite that
tolerates a busy store has stopped measuring conformance and started measuring luck."* ADR-0065
lowers the red rate to zero in sixteen launches on one host, and the temptation is to read that
as the question going away. It is not; it is the question becoming rarer, which is precisely the
condition under which a defect stops being noticed.

---

## Op 5 — the third instance of a convention the corpus is unsure about

`merge_existing` · `.kb/open-questions/adr-status-vocabulary-exceeds-the-schema.md` · sources
`§11` (by consequence) · **mapsImpact: `openQuestionIndex`** · `status` **unchanged** ·
**discretionary**

`kb-open-question-adr-status-vocabulary-001` is the only atom in the corpus that owns the defect
Adjudication 1 runs into. Its body: *"Two more are 'partly superseded' — ADR-0005 and ADR-0006 —
where marking the atom superseded would strip status from a half that still binds, and marking it
accepted hides that a half does not. The 2026-08-10 import applied a stated convention rather
than inventing keys … the `supersedes` pair used only for full supersession. What is not decided
is whether that is the corpus's answer or a stopgap."*

Three sentences, dated:

- ADR-0065 is the **third** partial supersession and the first at **section** grain — §11 of a
  fifteen-section record — rather than at half-a-document grain. The convention scales down
  further than the import assumed.
- It is the first instance where a **staged intake file asked in writing for the other
  spelling** (*"should carry `supersedes: [kb-decision-0022]` scoped to §11"*) and the wave
  declined it on the mechanical consequence. That is evidence for *stopgap* rather than *answer*:
  a convention that a well-informed author writes against is a convention that is not
  self-evident from the schema.
- `last_reviewed` → `2026-09-21`; `related` gains `kb-decision-0065`.

It is marked discretionary because no claim in the intake names this atom. An integrator who
drops it loses evidence, not correctness. Dropping it is preferable to enlarging it — this is a
three-sentence merge, not a re-opening of the status-vocabulary question.

---

# Adjudications

## Adjudication 1 — the intake asks for `supersedes: [kb-decision-0022]`, and the wave writes `supersedes: null`

**This is the wave.** Everything else follows from it.

The intake is explicit and it uses a modal: *"The new atom **should** carry `supersedes:
[kb-decision-0022]` **scoped to §11**, and say in as many words that §4, §6, §7, §9, §10 and §15
stand unchanged."* Both halves are honoured in substance. The second half is honoured literally.
The first is not, and here is why.

**The key carries no scope, and it carries a consequence.** `decisions/README.md` states the
mechanism in one sentence: *"To correct one, write a new atom carrying `supersedes: [<old id>]`,
**and flip the old atom's frontmatter to `status: superseded` + `superseded_by: <new id>`**."*
The two are one operation. Setting `supersedes` and declining the flip produces a corpus in which
`kb-decision-0065` claims to supersede an atom that does not know it has been superseded — a
state nothing validates and every later wave reads as fact.

**And the flip would be false.** `kb-decision-0022` is a fifteen-section record. What §11 fixed
was three pragma values, one of which moves. Untouched and load-bearing at `HEAD`: the
`max(position)` guard inside `BEGIN IMMEDIATE` (§4), the `event_tag(tag, position)`
`WITHOUT ROWID` join table (§6), `tag_cardinality` and most-selective-tag-first probing as
requirements (§7 — itself under separate challenge by the §8/§16 finding, which this wave does
**not** settle), the runtime seam (§9 — fired, unresolved), `Query::index_arms()`'s rejection
(§10), `CONTENDERS` as out of scope (§12), and the ratification of `rusqlite` without a pool
(§15). `status: superseded` on that atom would tell a reader that a store built to it today is
built to a withdrawn record. Two of the three pragma values in the very same section are also
unchanged: `journal_mode=WAL` and `synchronous=NORMAL` stand.

**The corpus has a convention for exactly this and has used it twice.**
`kb-decision-0007`: *"Partly supersedes ADR-0006's runner allocation while leaving its naming
decision untouched and standing"* — `supersedes: null`, and `kb-decision-0006` is `accepted` with
`superseded_by: null` at `HEAD`. `kb-decision-0031`: *"Partly supersedes ADR-0007's runner
allocation while leaving its discriminator and all three of its shape decisions untouched"* —
`supersedes: null`, and it states the consequence in its own summary: *"which is why
`kb-decision-0007` stays accepted and `superseded_by` stays null."* Every `supersedes` and
`superseded_by` pair actually set in this corpus — 0002→0005, 0021→0032 — is a **full**
supersession with the old atom flipped. The convention is not ambiguous; it is just not written
in the schema.

**What is lost by the prose spelling, stated rather than hidden.** A machine reading frontmatter
cannot see that §11 has moved. `redkiln validate --kb` will not flag a reader who cites
`kb-decision-0022`'s *"busy_timeout 5,000 ms"* as current, and the atom's summary will keep
saying five seconds forever, correctly, because it is immutable. The three mitigations are the
ones the corpus already uses: the new atom's **title** names the superseded section
(`ADR-0022 §11 is superseded in part`), `depends_on` names `kb-decision-0022` so the link is
machine-visible in one direction, and the `decisionMap` row carries the partial supersession in
its *Supersedes / superseded by* cell — which is the one place in this corpus where supersession
is a *rendered* fact rather than a frontmatter one.

**The alternative that lost, in full**: set `supersedes: [kb-decision-0022]` *and* flip 0022 to
`superseded`, then rely on ADR-0065's prose to tell readers which sections still stand. Rejected
because the flip is a claim about the whole atom made in a field that has no room for a
qualifier, and because it would make this the corpus's first supersession that a reader of the
`decisionMap` alone would read as total. `kb-open-question-adr-status-vocabulary-001` predicted
this exact trade in 2026-08-13 and did not resolve it. Op 5 records the third instance.

Carried to `unresolved[]` as a residual: **the corpus cannot express a scoped supersession in
frontmatter, and nothing detects a partial supersession that should have been a full one.**

## Adjudication 2 — the sweep gets its own reference atom, against a sibling scoring 66

`merge_existing` is the documented default at this score and it loses on three independent
grounds. Any one would be enough; they are listed separately because a future wave facing a
`reference` merge needs the test, not the verdict.

**One — the dating rule forbids it.** `reference/README.md`: *"Every atom here is a statement
about a moment … A reference atom is a snapshot."* `kb-reference-busy-timeout-margin-001` is the
snapshot of **2026-09-03**, and its own closing section makes the disagreement with
`kb-reference-append-condition-experiment-001` *the finding*: *"a later, contradicting
measurement supersedes nothing and simply sits alongside the earlier one."* Folding a 2026-09-21
measurement into it would be the corpus breaking its own rule in the atom that states it most
clearly.

**Two — it is a different instrument measuring a different subject.** The margin atom's harness
opens N `rusqlite` connections against one file with a **counting busy handler** and records a
per-contender wait vector; the sweep runs `crates/happenstance-sqlite`'s **own conformance
target**, unmodified, through the adapter's `Mutex`-across-transaction write path, and reports
**one bit per launch**. The sweep page says so in terms: *"What this page gives up in exchange is
the instrumentation … Read the two together."* Two instruments, two atoms, one citation between
them.

**Three — a merge would make the surviving atom internally false.** The margin atom's summary
reads *"how much of ADR-0022's fixed 5,000 ms `busy_timeout` the shipped conformance
configuration actually consumes"*, and quotes margins **against 5,000**: 3,628 of 5,000, 1.38x.
Those numbers are correct about their moment and meaningless against 15,000. A merged atom would
have to either restate them against the new cap — which nobody measured, and which its own second
caveat says it cannot supply — or carry two budgets in one snapshot.

**What the merge would have bought, and how it is bought instead.** Findability: one atom, one
place. Bought instead by each atom's `related` naming the other, by the new atom's first section
stating the difference explicitly, and by the `domainMap`'s measurement section listing them
adjacently. `kb-reference-one-connection-latency-001` is the shape precedent — an adapter-level
SQLite measurement that sits beside rather than inside the experiment-level ones.

## Adjudication 3 — two of three findings stay open, so the question stays `accepted`

The last wave resolved an open question by flipping it to `superseded`, on five prior instances
of that convention. It is not available here, and the reason is arithmetic:
`kb-open-question-adr-0022-falsifiers-fired-001` carries **three** findings and ADR-0065 answers
**one**. `open-questions/README.md` reserves the flip for *"when the question is answered"*.

The tempting error is the other direction — treating the atom as *mostly* resolved because its
loudest finding is settled. §11 is the one with a measurement behind it and the one the code
already acted on, so it reads as the atom's centre. It is not. §9 is a **correctness** gap —
`SqliteEventStoreError::NoRuntime` unreachable for a store outliving its runtime, `read` calls
that hang or yield one cancelled-task item — which outranks a liveness cap on any reading, and it
is still owed the twenty-line reproduction the falsifier's own *"a deployment shows"* demands.
§8/§16 is a shipped SQL shape measured to lose in nine of nine two-tag cells. The amendment
therefore leads with what is *still* open and records §11's resolution as one dated section
inside it, rather than the reverse.

## Adjudication 4 — the merge carries the finding the digest did not extract

The digest's Claim 3 says the instrument demonstrates *"both affected rules reject a
merely-contended store on different assertions."* True, and it is the weaker half of what the
tree carries.

The open question's own closing sentence is a **prediction**: the instrument *"discriminates the
two arms as well as gating them, because a per-error channel and a per-fixture capability behave
differently against it."* A merge that recorded only the instrument's existence would leave that
prediction standing while the file that tests it has already returned a result — and the next
reader would re-derive it, which is the failure `open-questions/README.md` says this layer exists
to prevent.

The result, from `a_retry_loop_gated_on_the_dcb_signal_never_retries_a_busy_store`: the port
offers a caller exactly one classifier, `AppendError::is_condition_violated`, and it answers
`false` for a busy store exactly as for a broken one. The per-error arm therefore cannot be
prototyped in the testkit at all — it needs `AppendError::Busy` in `happenstance-core` first —
while the per-fixture arm can be built today. That does not choose between them, and the merge
must not pretend it does: the atom's live counter-argument, *"`happenstance-sqlite` is the only
event-store adapter to have run the suite, and one implementor is not a spread"*, is untouched by
this. What changed is that the two arms now differ in **cost to try**, which is a new axis in a
question that previously had them symmetric behind one missing instrument.

## Adjudication 5 — three claims are recorded as explicitly not-settled, and none becomes a new open question

The intake names three things it declines: §9, §8/§16, and the contended-versus-broken
conflation. The reflex is `defer_open_question` for each. All three already have owners:

| Declined | Owner | Action |
| --- | --- | --- |
| §9's firing, owed a reproduction | `kb-open-question-adr-0022-falsifiers-fired-001` | already recorded there in detail; op 3's merge restates that it stands. **No new atom** |
| §8/§16's unfireable falsifier | same atom | same. **No new atom** |
| busy-versus-broken conflation | `kb-open-question-testkit-contention-tolerance-001` | op 4's merge. **No new atom** |
| `CONTENDERS` | `kb-open-question-testkit-contention-tolerance-001` (*"whoever next proposes changing `CONTENDERS`"*) | out of scope by ADR-0022 §12; recorded as scope in ADR-0065's body. **No new atom** |

Rule 3 — merge and link over creating — and rule 4's *"unresolved conflicts → an open_question
atom"* do not conflict here: rule 4 is about conflicts with **no** home, and every one of these
has one. **Zero open questions are created by this wave**, which is worth stating because a wave
that resolves a decision usually creates at least one.

## Adjudication 6 — nothing in this wave is a `[FROZEN]` clause edit, and the check was run

`grep -n "busy_timeout\|BUSY_TIMEOUT\|5,000 ms\|5000 ms" spec/SPECIFICATION.md` returns nothing.
No clause quotes the constant, so ADR-0065 moves a value the specification never fixed and
`cargo xtask spec-trace` has no stake in this wave.

This is not a formality. The sibling finding in the same open question — §8/§16 — has the
opposite answer: ES-27's `Rejects:` prose at `spec/SPECIFICATION.md:3902-3905` quotes the
aggregate's *"roughly 200x"*, so repairing it **is** a `[FROZEN]` clause edit that CLAUDE.md
routes through a new decision. The check is what tells the two apart, and running it is what lets
this wave say the difference out loud rather than assume it.

The corresponding live-truth check in the other direction also came back clean:
`crates/happenstance-sqlite/README.md:128` already documents `15,000` ms with the measurement
behind it, so the published adapter's own documentation and ADR-0065 agree on the day the atom is
written. `kb-decision-0022`'s summary still says 5,000, which is correct and immutable — it is
history, and `decisions/README.md` says *"An ADR is never updated to match the code."*
