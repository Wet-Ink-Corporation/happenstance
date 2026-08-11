# Six gaps owed a decision — six independent questions

## Ingest instruction: this file must yield six atoms, not one

Each numbered section below is a **separate question with its own evidence, its own settling
condition, and its own owner.** They share only a provenance — all six were found by the
phase 4/5 specification reconciliation (`3c704d3`…`84dcc67` on `redkiln-adoption`) and none was
closed by it, because closing any of them requires an ADR and that pass deliberately wrote
none.

**Do not merge them.** They are owed to different phases, they would be settled by different
evidence, and answering one tells you nothing about the others. Collapsing them into a single
"open gaps from the reconciliation" atom would produce exactly the artifact this repository's
own conventions warn against: a finding filed where the person who needs it will not look.

**Layer for all six: `open-questions/`. Kind: `open_question`. `authority_tier: note`.**
Suggested atom slugs are given per section. Each should carry the specification path, the code
path it is grounded in, and this intake file in `source_paths`. Where a question names a phase,
that phase should appear on the atom so the runbook and the KB can be reconciled.

Two of the six — gaps 1 and 2 — are additionally held in `UNCLAIMED_PENDING_ADR`
(`xtask/src/spec_trace.rs:1968-1997`), which prints them on every green `cargo xtask spec-trace`
run. Those two have a mechanism; the other four rely on being read.

---

## Gap 1 — the central DCB independence proposition is enforced by a rule and stated by no clause

*Suggested slug: `disjoint-boundaries-have-no-clause`.*

**What is true today.** `k_disjoint_boundaries_admit_exactly_k_commits`
(`crates/happenstance-testkit/src/concurrency.rs:436`) is a live conformance rule. It enforces
the proposition that commands sharing no consistency boundary do not conflict — the
independence property that Dynamic Consistency Boundary exists for. **No clause in
`spec/SPECIFICATION.md` states it.** The word "disjoint" occurs zero times in the
document; verified by grep against the working tree on 2026-08-10.

**Why it was not simply attached to a nearby clause.** ES-25 is the obvious candidate and it is
the wrong one. ES-25's *only if* half forbids the false-positive direction — it says a
non-overlapping append must not be rejected for the wrong reason — and does not state the
positive proposition that *k* disjoint commands all commit. Attaching the rule to ES-25 would
assert that a `[FROZEN]` clause contains a proposition it does not contain, which is a defect
strictly worse than the missing attribution because no check could ever see it.

**What would settle it.** An ADR that either widens ES-25 to state the independence
proposition, or mints a new clause for it. Widening a `[FROZEN]` clause is itself an ADR-scale
act, so the two options differ in cost less than they appear to.

**Owner.** Unassigned. This is the sharpest of the six: it is the library's central claim,
tested and unstated.

---

## Gap 2 — the model family's rule enforces the composition of seven clauses and belongs to none

*Suggested slug: `model-family-rule-has-no-clause`.*

**What is true today.** `ops_agree_with_the_model`
(`crates/happenstance-testkit/src/model.rs:598`) is the model family's single rule. It replays
a generated sequence of appends, conditional appends and reads against a model and compares
every answer. What it checks is therefore the **composition** of ES-8, ES-9, ES-11, ES-14,
ES-15, ES-18 and ES-25 over inputs no clause enumerates.

§6.4 does name the rule, but only as CF-22's illustration of a per-family enumeration, and
CF-22's MUST is where the rule *list* lives rather than what any rule asserts. Claiming it
under any one of the seven clauses it exercises would say that clause is what it checks.

**Why it differs in shape from gap 1.** Gap 1 is a rule stating a proposition no clause states.
This is a rule stating *several* propositions, and belonging to no single clause for that
reason. The two cannot be closed by the same move, which is why they are two entries rather
than one.

**What would settle it.** An ADR that either mints the clause the model family has never had —
*a store agrees with the contract over arbitrary operation sequences, not only over the
examples the suite enumerates* — or decides that check 6's bar is per-clause and a cross-clause
rule is disposed of some other way. The second option is not obviously worse and it is not
obviously safe: whatever "some other way" is, it becomes the precedent for every future
property-based rule.

**Owner.** Unassigned.

---

## Gap 3 — PS-1's MUST is a coupling, not a progress obligation

*Suggested slug: `ps-1-states-no-progress-obligation`.*

**What is true today.** PS-1 (`spec/SPECIFICATION.md:4733`) is `[FROZEN]`:

> The read-model write and the checkpoint write MUST become durable together or not at all.

§4.11's table assigns it three rules: `commit_is_atomic_with_the_read_model`,
`failed_commit_leaves_both_unchanged`, and `commit_advances_the_checkpoint`. **The third does
not follow from the sentence.** A `commit` that returns `Ok` and makes *neither* the read-model
row nor the checkpoint durable satisfies the MUST through its "or not at all" arm, passes
`commit_is_atomic_with_the_read_model` — both-absent is one of the two states that rule permits
— and fails `commit_advances_the_checkpoint`.

That a successful commit *advances* anything is stated by no clause's MUST in the document.
PS-22 presupposes it; §4.1a's prose asserts it non-normatively. The clause body now records
this.

**What would settle it.** An ADR adding either a sentence to PS-1 or a clause of its own. It is
an ADR's rather than an edit's because PS-1 is `[FROZEN]` — adding a progress obligation would
change the set of implementations the clause admits, which is the repair-versus-gap test's
dividing line.

**Owner. Phase 6** ("Freeze `ProjectionStore`", `RUNBOOK.md:3846`), which discharges
PS-1 – PS-37 and settles ADR-0017, ADR-0018 and ADR-0019.

---

## Gap 4 — PS-19's MUST is scoped after a reset; its second rule asks about an unseen id

*Suggested slug: `ps-19-scope-narrower-than-its-rule`.*

**What is true today.** PS-19 (`spec/SPECIFICATION.md:5218`) is `[FROZEN]`:

> After a successful `reset`, `checkpoint(id)` MUST return `Checkpoint::NeverRun`, and this
> MUST be distinguishable from `commit(empty_batch, id, SequencePosition::FIRST, Live)`.

§4.11 assigns it `fresh_projection_has_no_checkpoint` in addition to
`reset_is_not_commit_at_first`. That second rule asks about an id that has **never been seen**,
while the MUST is scoped *after a successful `reset`*.

**The implementation that exposes it is the natural one, not a contrivance.** A store whose
`reset` writes an explicit `NeverRun` sentinel row, and whose `checkpoint(id)` resolves a
missing row with `.unwrap_or(Checkpoint::Live { through: FIRST })`, answers `NeverRun`
post-reset — satisfying the MUST verbatim and distinguishable from a commit at `FIRST` — and
answers `Live` for an id it has never seen, failing the rule. No clause's MUST obliges an
unseen id to read as `NeverRun`.

**What would settle it.** An ADR deciding whether PS-19 widens to cover the never-seen id, or a
new clause states it. Note the interaction with gap 3: both are PS-layer clauses whose MUST is
narrower than the rule table assigns them, which may indicate a systematic problem with how
§4.11's table was populated rather than two isolated defects. **Whoever takes either should
check the other 35 PS clauses for the same shape** before deciding the scope of the ADR.

**Owner. Phase 6.**

---

## Gap 5 — ES-6 is `[FROZEN]` naming a rule that cannot be written

*Suggested slug: `es-6-names-an-unwritable-rule`.*

**What is true today.** ES-6 (`spec/SPECIFICATION.md:2629`) is `[FROZEN]` and its
`Rule:` field names `store_error_crosses_a_join_handle` **(new)**
(`SPECIFICATION.md:2670`). §7.2's generated table renders it with `†`
(`SPECIFICATION.md:8588`), where the legend defines `†` as "does not exist yet".

**That rule occurs as no `fn` anywhere in the workspace.** It occurs only in prose: twice in
`RUNBOOK.md`, once in `docs/adr/0008`, three times in `docs/adr/0009`, three times in
comments in `happenstance-cloudflare`, and in the specification itself. One of those comments
(`crates/happenstance-cloudflare/src/lib.rs:92`) states that it "is unwritable against today's
port for *every* adapter, not merely for this one", and the probe backing that claim is
`SendStoreWithLocalError` (`crates/happenstance-cloudflare/src/send_shape.rs`, whose doc
comment at `:94-98` says: "If this compiles — and it does — then the *derived* flavour does not
imply a `Send` error either").

**Why the gate does not catch it.** Check 6 was widened in `52105d2` so that every rule has a
clause. This is the inverse: a clause whose rule cannot be written. Check 4 — which looks a
clause's named rules up in `RULE_FILES` — skips any clause that declares its rule new or marks
it `†`: `if c.schedules_new || !has_suite(&c.id) { continue; }`
(`xtask/src/spec_trace.rs:685`), where `schedules_new` is set by `(new)`, `†`, a leading
`new `, or ` new \`` (`:1616-1620`). The escape hatch is deliberate and correct in general — a
clause may legitimately schedule a rule the phase has not written yet — and it has no
expiry, so a rule scheduled forever is indistinguishable from one scheduled for next week.

**A partial resolution already exists and has not been executed.** ADR-0009 settles the
underlying question: `Error` keeps `core::error::Error + 'static` on both ports and both
flavours, and the stronger property becomes a **marker trait declared downstream**. That makes
the rule writable, and the wrong implementation it must reject already exists in the tree.
So this may be a scheduling gap rather than a design gap — but the clause is `[FROZEN]` and
still names an unwritten rule, and that is the state a decision has to address.

**What would settle it.** Either the rule is written against ADR-0009's marker (in which case
ES-6's `(new)` and its `†` come off, and the escape hatch stops applying), or a decision is
taken about `†` clauses that have no scheduled phase. The second is worth taking regardless:
**a `†` with no owning phase should probably be a hard failure, and today it is silence.**

**Found twice, independently.** `references/evaluation/review-citation-drift.md` §2 reports the same
finding, written the same day from the `standards/rust/` work with no knowledge of this pass, and
records that the identifier occurs "in **three**, all of them comment" locations in the crates.

**Owner.** ADR-0009 exists and is accepted; the phase that writes the rule is unassigned.

---

## Gap 6 — two `[PROVISIONAL]` markers whose falsifiers appear to be discharged

*Suggested slug: `es-7-and-vt-9-provisional-markers`. This may reasonably split into two atoms;
if it does, keep the shared observation about falsifier hygiene on both.*

A `[PROVISIONAL]` marker in this specification carries a stated falsifier — the observation
that would demote or change the clause. Two markers name falsifiers that no longer discriminate,
and **moving a maturity marker is an ADR's**, so both are recorded rather than moved.

**6a — ES-7.** `spec/SPECIFICATION.md:2685`. The clause says a downstream crate
may implement the bare flavour directly without colliding with the blanket impl. Its marker
reads:

> `[PROVISIONAL — falsified by error[E0119]: conflicting implementations on a downstream
> impl EventStore for LocalType. The named test is the !Send reference store, which lives in
> happenstance-testkit — a genuinely downstream crate — and which is also ADR-0001's own lift
> condition.]`

The named instrument is `LocalMemoryEventStore`
(`crates/happenstance-testkit/tests/local_conformance.rs:198`), and the comment immediately
above it reads: "This is the ES-7 evidence: a direct `impl EventStore for` a local type in a
downstream crate, alongside the blanket `impl<T: SendEventStore> EventStore for T` that
`trait_variant` emits… **No `error[E0119]`.**" The instrument passes natively and on `wasm32`.
A falsifier its own named instrument cannot produce is not doing work.

**6b — VT-9.** `spec/SPECIFICATION.md:903`. The marker reads:

> `[PROVISIONAL — falsified by a target that cannot supply a wall clock at append time; a
> Cloudflare Durable Object returning a frozen clock between I/O operations is the candidate,
> and the Workers skeleton is the instrument]`

`crates/happenstance-core/src/memory.rs:263-274` says outright that such a target exists and is
in the build: on `wasm32-unknown-unknown` there is no clock, `wall_clock_millis()` returns
`None`, and every event is stamped `RecordedAt::from_millis(0)`. Its doc comment names the
clause: "VT-9's own falsifier names 'a target that cannot supply a wall clock at append time',
and this is one." **Yet VT-9's MUST is satisfied there** — the two rules assert that a recorded
time is *present* and *stable*, never that it is recent, and CF-33 independently forbids
checking a value for plausibility against the harness's clock. So the falsifier is
**satisfiable without falsifying**, which is the defect: the condition has occurred and the
clause is unharmed.

**What would settle either.** An ADR that lifts the marker to `[FROZEN]`, or restates the
falsifier as something that would actually discriminate. For VT-9 the honest restatement is
probably about *what a clockless target is obliged to do* rather than about whether one exists.

**The transferable observation, worth keeping whichever way these go.** A provisional marker is
a claim that a future observation will change the clause. **A falsifier that has already
occurred without changing anything is a marker that has quietly become decoration** — and
nothing in `cargo xtask spec-trace` can detect that, because the checker can see whether a
marker exists and not whether its condition has been met. This suggests a phase-exit reading
task rather than a gate step, which connects it to
`open-question-nothing-owns-the-post-phase-reconciliation.md`.

**Owner.** Unassigned. ES-7 is entangled with ADR-0001's own lift condition; VT-9 is entangled
with the Workers adapter's schedule.
