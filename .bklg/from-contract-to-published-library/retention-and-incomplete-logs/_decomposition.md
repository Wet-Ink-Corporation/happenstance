---
item: HS-P0018
stage: briefs
---

# Briefs — What a store may forget, and how a reader finds out

Companion to [`project.md`](project.md) and [`_grounding.md`](_grounding.md). Not a
redkiln item: no system frontmatter beyond the two keys above. This artifact holds
**all** of this project's briefs, one `##` section each. Per
[`../_decomposition.md`](../_decomposition.md) *Warranted briefs*, this project
carries **two** — `architecture` and `testing`. No `ux` or `deployment` section
belongs here: there is no screen, and this project deploys nothing.

The AC spine every section answers to is `project.md`'s *Acceptance criteria*
(AC-001 – AC-016); the verified anchors are `_grounding.md`'s.

---

## Architecture brief

### Intent

Give the implementer a technical approach for the one deliverable this project
owns: **an instrument that forgets, two rules written against it, two readers run
through it, and ADR-0028 — with no change to any surface published at `0.2.0`.**

Everything below is contract and decision. No production code is prescribed. Where
a choice is genuinely open, the brief names the discriminator rather than the
answer, because the point of CF-27's experiment
(`spec/SPECIFICATION.md:8034-8060`) is that the *result* is evidence and a
pre-decided result is not.

Three things this brief settles that would otherwise be settled by accident:

1. **What shape the instrument has** — a retained-*set* decorator, not a
   suffix-only one, and forgetting by hiding at the port rather than by rebuilding
   the inner store (DA-1, DA-2). The second half is not taste: the obvious
   construction is provably the ES-38 mutant.
2. **Where each capability mounts** — five real composition roots, all of which
   exist in the tree today except the projection runner, which arrives from
   HS-P0011 (*Composition roots*).
3. **Which of AC-003's, AC-007's and AC-010's answers are reachable inside AC-011
   and which are AC-012 escalations** — decided against the code, before anything
   is built, because the intake brief flags this as the question most likely to
   collide with the constraint and the one that must not be discovered late
   (`_intake-brief.md` *Open Questions*).

### The seam map — what is touched, and what deliberately is not

Per `CLAUDE.md`'s package layout, and its dependency rule (everything depends on
`happenstance-core`; `happenstance-core` depends on nothing in the workspace).

| Crate / path | Change | Surface class under AC-011 |
|---|---|---|
| `crates/happenstance-testkit/tests/` **(new target)** | the completeness instrument, its `Fixture` impl, and the `event_store_conformance!` invocation | **none** — `tests/` is not public API |
| `crates/happenstance-testkit/src/suite.rs` | two (possibly three) new rule bodies | **additive** `pub fn` via `pub use suite::rules` (`crates/happenstance-testkit/src/lib.rs:189`) |
| `crates/happenstance-testkit/src/registry.rs:94` | the same names inside `for_each_event_store_rule!` | **additive**, and see the *semver-MINOR* note below |
| `crates/happenstance-testkit/src/contract.rs` | *only if a rule needs it*: a **defaulted** associated const + a **defaulted** method | **additive** — see DA-4; a required item here is breaking |
| `crates/happenstance-testkit/tests/mutation_coverage/` | two new `Defect` rows in `REGISTRY` (`crates/happenstance-testkit/tests/mutation_coverage.rs:324`) | **none** |
| `crates/happenstance-core/src/append.rs`, `.../store.rs` | ES-40's documentation sentence only (`spec/SPECIFICATION.md:4351-4356` — *"the port's documentation MUST state…"*) | **none** — rustdoc prose is not a semver surface |
| `crates/happenstance-sync/src/ingest.rs` | the ingest reader of AC-006 | **none** — `crates/happenstance-sync/Cargo.toml:12` is `publish = false` |
| `crates/happenstance/` | consumes the projection runner; **builds none** | **none** |
| `spec/SPECIFICATION.md` | ES-39, ES-40 and CF-27's markers; §1.3's hand census | governed by `cargo xtask spec-trace` — see *Gate mechanics* |
| `.kb/_intake/` → `.kb/decisions/`, `.kb/open-questions/`, `.kb/maps/` | ADR-0028 and the open-question resolution | authored through `/redkiln:kb-ingest`, never by hand (`CLAUDE.md`; the revert at `0269720`) |

**Not touched, and each has an owner elsewhere.** `EventStore`'s method set
(`crates/happenstance-core/src/store.rs:119`, `:213`, `:248`, `:268`) — ES-37 is
`[FROZEN]` (`spec/SPECIFICATION.md:4274-4297`) and amending it is a new decision
atom plus a re-plan. `Fixture`'s two **required** capability consts
(`crates/happenstance-testkit/src/contract.rs:161`, `:173`) — a third required one
breaks every downstream `Fixture` impl. The replication merge rule and SY-32's
answer — HS-P0017's, consumed here by id (AC-016). The `read_from_a_gap_position`
ownership thread — fenced off by `project.md` *Out of scope*, and DA-6 says how to
resolve the atom without closing it by accident.

**One AC-011 nuance that must be recorded rather than discovered.**
`crates/happenstance-testkit/Cargo.toml:4-13` states it in the crate's own words:
*"Adding a conformance rule is a semver-MINOR change that can turn a passing
adapter's CI red"*, which is why that crate carries its own version (`0.2.0`)
rather than the workspace's. So the surface diff HS-P0016's instrument produces
will correctly report **additive**, and the *effect* of these two rules on a
downstream adapter is still a red build. That is the mechanism `CLAUDE.md`'s "the
rule that matters" exists to produce, not a violation of AC-011 — but AC-011's
recorded finding should say both sentences, not just the first.

### Composition roots — where each capability mounts

A rule body, a fixture or a reader that is not wired into one of these runs
nowhere and proves nothing. Each is a real file in the tree today unless marked.

1. **`happenstance_testkit::event_store_conformance!` in a new
   `crates/happenstance-testkit/tests/` target** — the mount for the instrument.
   Copy the shape at `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205`
   verbatim (`mod_name = …, fixture = …`). Until this invocation exists the
   instrument "does not exist" under `CLAUDE.md`'s rule that matters, and AC-001
   is not met by a type that merely compiles.
2. **`for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`)**
   — the mount for every new rule, and the *only* one. It feeds all four emitters
   (`__emit_tokio`, `__emit_blocking`, `__emit_wasm`, `__emit_rule_names`), so one
   registry line is what makes a new rule run in `memory_conformance.rs`,
   `local_conformance.rs`, `memory_conformance_blocking.rs`,
   `memory_conformance_wasm.rs`, `fixture_instruments.rs` and the new instrument
   target at once — and, through `for_each_mutant!`, against **every** registered
   mutant. A rule body added to `suite.rs` and not to the registry is invisible to
   all of them.
3. **`REGISTRY` (`crates/happenstance-testkit/tests/mutation_coverage.rs:324`) plus
   a `Defect` impl in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`**
   — the mount for AC-004's and AC-005's "registered wrong implementation". This is
   the existing mechanism and no parallel harness may be invented:
   `mutant_registry_is_exhaustive` (`:2754`) and
   `mutants_fail_exactly_their_declared_rules` (`:2889`) are what make a row
   load-bearing in both directions, and `mutant_registry_is_exhaustive` rejects a
   `Kind::Mutant` row with an empty `fails` list.
4. **`happenstance_core::read_decision_model`
   (`crates/happenstance-core/src/store.rs:321-331`)** — the real DCB read half,
   and the cheapest honest demonstration of ES-40's hazard. It returns
   `(events, last_seen_position)` and that position is what feeds
   `AppendCondition::after_opt`. Over a store that has forgotten matching history,
   `last` is the last *retained* match, so the follow-up conditional append is
   admitted where the destroyed history would have rejected it — silently, in
   existing code, with no new type. This is the reader for the "confidently wrong
   answer" half of AC-006, and it needs nothing built.
5. **The projection runner in `happenstance`, over the checkpoint pump in
   `crates/happenstance-core/src/projection.rs`** — the mount for AC-006's
   projection half. *Arrives from HS-P0011 `typed-layer-and-alpha-release`*
   (rank 1; this project is rank 5) per
   [ADR-0007](../../../.kb/decisions/0007-projection-runner-decodes.md); today
   `projection.rs` holds `ProjectionId` and the `ProjectionStore` trait and no
   runner, and `crates/happenstance/src/lib.rs` is still the facade. **This project
   consumes that runner; it does not build one.** If, when the work starts, no
   runner exists, that is a dependency failure to halt on and report — not a
   licence to write a bespoke one, which would demonstrate a hazard against a
   reader nobody ships.
6. **`IngestStore` (`crates/happenstance-sync/src/ingest.rs`)** — the mount for
   AC-006's ingest half, and the seam ES-40's `Rejects:` calls *the normal path*
   under the unconditional-ingest decision. Read that module's own doc comment in
   full before writing anything: the trait seam is discharged and the **write
   path** is not — `MemoryEventStore::append` mints its own `EventId` and
   `restore` builds a whole new store, so the bodies are `todo!()` today. HS-P0017
   owns closing that. `holds()` is the one method the write path does not block,
   and DA-8 is why it is the interesting one anyway.
7. **`spec/SPECIFICATION.md` + `cargo xtask spec-trace`** — the mount for AC-014.
   See *Gate mechanics*: the markers, §1.3's hand census and §7.1/§7.2's generated
   tables are three separate obligations and only one of them is regenerable.
8. **`.kb/_intake/` + `/redkiln:kb-ingest`** — the mount for ADR-0028 (AC-008) and
   for the open-question resolution (AC-015). Atoms are never hand-written into
   `.kb/decisions/`.

### DA-1 — The instrument holds an arbitrary retained **set**, not only a suffix

CF-27 says *"holds only a suffix of its own log"*
(`spec/SPECIFICATION.md:8034-8036`). ES-38 and ES-39 both say something stronger:
*"a testkit-adjacent store that deliberately holds only a **scattered subset** of
its own log"* (`:4309-4311`, `:4326-4328`). The difference is the whole of ES-39's
argument: `earliest_position()` *"is exactly correct for a device pruning old
history and useless for a regulated purge, because that purge is **scattered, not
a prefix**"* (`:4336-4341`).

So a suffix-only instrument **structurally cannot falsify the floor**. Run the
experiment against a prefix-truncated store and `earliest_position()` looks
correct — which is precisely the outcome ES-39's `Rejects:` names as the path of
least resistance. Build the instrument parameterised by a retained predicate over
positions, and run CF-27's experiment in at least two configurations: a suffix
(the prune case) and a scattered hole with survivors below it (the regulated-purge
case, E2E-46/E2E-47).

This is a deliberate widening of CF-27's own wording. It is available to take
because CF-27 is `[DEFERRED — owned by the pass that settles retention and
deletion]` and this project is that pass; record it in ADR-0028 as a widening with
its reason, so the clause text and the instrument agree afterwards.

### DA-2 — Forget by **hiding at the port**, never by rebuilding the inner store

Two constructions suggest themselves, and one of them is a mutant.

**(a) Rebuild the inner store from a filtered snapshot** —
`MemoryEventStore::restore(store_id, retained)`
(`crates/happenstance-core/src/memory.rs:163`), the way `DurableFixture` rebuilds
on reopen (`crates/happenstance-testkit/tests/fixture_instruments.rs:190-193`).
**This is wrong, and it is the ES-38 mutant.** `MemoryEventStore::append` assigns
`position_at(first_index + offset)` where `first_index = stored.len()`
(`crates/happenstance-core/src/memory.rs:386-389`, and `position_at` at `:277-282`):
positions come from the **Vec index**, not from the last position assigned. A store
restored from a suffix of three events therefore assigns `1, 2, 3…` on its next
append and **reuses positions it has already assigned** — which is ES-38's
`Rejects:` verbatim: *"an adapter that renumbers on compaction"*
(`spec/SPECIFICATION.md:4321-4323`).

That is not a defect in `MemoryEventStore` — its contract is a dense log from 1,
asserted at `crates/happenstance-core/src/memory.rs:457-470`. It is a free,
faithful mutant: **construction (a) is `positions_are_not_reused_after_removal`'s
registered wrong implementation** (AC-004), and it models the real device
compaction that motivates it.

**(b) Decorate a live inner store and filter what the port reports.** The inner
store keeps everything (memory is free in a test); the decorator's `read`, `head`
and `contains_event_id` answer as though the forgotten events are gone, and
`append` continues to allocate from the inner store, so positions stay unique and
strictly monotonic across the hole. This is the conformant instrument, it is
CF-27's own stated experiment shape (*"a decorator over any `EventStore`"*), and it
is the same one axis over from `DurableFixture`, which is the precedent to copy for
fixture ergonomics.

**Why hiding is not cheating.** The conformance suite observes only through the
port, and `EventStore` has two methods and neither deletes
(`crates/happenstance-core/src/store.rs:119`, `:213` — CF-27's own `Rejects:` says
so). At that seam a store that hides and a store that destroyed are the same value,
which is §3.7's thesis (`spec/SPECIFICATION.md:4260-4272`) restated from the
instrument's side. State the limitation explicitly in ADR-0028 and against
CF-25/CF-26 (`:8003-8021`): this discharges **falsifiability** and not
**implementability**, and the residual exposure on the completeness axis — the
*"device adapter second"* of `:8090-8098` — is recorded open, not implied closed.

**Two mechanical traps in construction (b)**, both from binding constraints:

- **`read` must stay non-`async`, returning the stream at the top level**
  (`CLAUDE.md` constraint 3; [ADR-0001](../../../.kb/decisions/0001-async-port-flavours.md),
  ADR-0008). The filtering must happen *inside* a `Stream`, not by collecting in an
  `async fn`. `DurableHandle::read` merely forwards
  (`crates/happenstance-testkit/tests/fixture_instruments.rs:126-133`) and is
  therefore not a precedent for filtering. `happenstance-testkit` depends on
  `futures-core` only (`crates/happenstance-testkit/Cargo.toml:33`), so the
  filtering adapter is hand-written — the same reason `happenstance_core::collect`
  is hand-written (`crates/happenstance-core/src/store.rs:285`).
- **Filtering changes what `ReadOptions` mean.** `limit` applies *after* filtering
  (`read_limit_applies_after_filtering` in the registry), and `from` names a
  position, not an index (`crates/happenstance-testkit/src/suite.rs:1490-1531`). A
  decorator that drops rows after the inner store applied `limit` returns short
  pages and fails several rules for a decorator bug rather than for forgetting.
  Prefer filtering below the option application — i.e. the decorator owns the whole
  read semantics over its retained set — or accept and record the distortion.

### DA-3 — AC-003 is the project's sharpest question; do not buy it with pre-seeding

AC-003 wants *at least two conformance rules to fail against the instrument, by
name*. §3.7 and CF-27 both predict the opposite: a store that has been pruned
*"passes every rule unchanged"* (`spec/SPECIFICATION.md:8050-8055`). Both cannot be
true, and how that is resolved is the project's main finding.

**The dishonest resolutions, named so they are recognisable:**

- **Pre-seeding.** A fixture whose `connect()` hands back a store that already
  holds history fails `reading_an_empty_store_yields_nothing`,
  `head_of_an_empty_store_is_none`,
  `condition_against_an_empty_store_admits_the_append` and
  `two_fixture_instances_observe_none_of_each_others_appends` — all for reasons
  about *seeding*, none about *forgetting*. Two of those satisfy AC-003 by the
  letter and destroy AC-002 entirely, because the pass list is evidence only if
  every failure excluded from it is about completeness.
- **An eager sliding window.** A store that forgets continuously fails dozens of
  rules for the boring reason that a rule writes three events and reads them back.
  Same defect, more noise.

**The honest shape.** The fixture presents an *empty* store at `connect()` like
every other fixture, and forgetting is triggered by something the rule itself does
through the fixture's own declared seam (DA-4). Then run the experiment and take
the answer:

1. Record the pass list (AC-002). If it is "all of them", that is CF-27's predicted
   result and the strongest possible evidence for ES-39 — not a failure.
2. AC-003's two named failures then come from the two **registered wrong
   implementations** AC-004 and AC-005 already require: the renumbering store of
   DA-2(a) and a store that rejects instead of passing vacuously. `project.md`'s
   DR-3 supports that reading — *"a store that **lies** about its own completeness
   is the one wrong implementation nothing in the workspace can currently
   detect"* — the failures are owed by the lying store, not by the honest one.
3. If the project concludes AC-003 requires a rule that can only exist with an
   ES-39 primitive on the port, that is the AC-012 escalation, and it is the
   expected one. See DA-7.

Whichever of the three holds, it is recorded as a decision in ADR-0028 with the
other two named. Silently re-tuning the instrument until two rules go red is the
failure mode this decision exists to prevent.

### DA-4 — The fixture seam for removal: defaulted const **and** defaulted method

`positions_are_not_reused_after_removal` needs the store to lose something *during*
the rule; nothing on `Fixture` can ask for that today. The precedent that makes
this non-breaking already exists and is exact — **`MID_BATCH_FAULT`**:

- a **defaulted** associated `Capability` const whose default is
  `Capability::declined(<reason>)`
  (`crates/happenstance-testkit/src/contract.rs:207-211`), so every existing
  `Fixture` impl in and out of the workspace keeps compiling; and
- a **defaulted method** whose body panics with a message naming both ways of
  reaching it (`arm_mid_batch_fault`, `:297-307`), so a fixture that declares the
  capability and forgets the override gets a legible error rather than a silent
  pass; and
- rules gated with `require!(F: CAPABILITY)`, which return
  `RuleOutcome::Skipped` and are **still emitted as tests** — `#[cfg]`-ing them out
  is the arrangement `contract.rs`'s module docs reject by name (`:31-42`).

Two live rules already ride that pattern
(`append_is_atomic_under_a_mid_batch_fault`,
`arming_a_mid_batch_fault_makes_the_append_fail`), so the shape is proven, not
proposed.

**Capability or fact?** `crates/happenstance-testkit/src/contract.rs:44-54` draws the line: a `Capability` is a
**trade** (the fixture could have co-operated and owes a reason), an
`Option<usize>` limit is a **fact** (nothing to justify). *"Can you be made to
forget?"* is a trade — every store could co-operate in a test — so `Capability` is
the right family. What a store *does not hold* is a fact, and it has no home on
`Fixture` at all: that is ES-39's port primitive, and putting it on the fixture
instead would be answering ES-39 in a place no application can read.

Add **nothing** if the rules can be written without it. A defaulted const nobody
consults is one more line every adapter author reads and no information.

### DA-5 — ES-40's rule asserts the specified outcome, and its home is prose

`Guard::is_violated_by` (`crates/happenstance-core/src/append.rs:239-253`) is a
pure predicate over events that still exist and has **no third arm** — the two-arm
`match` is right there, and `AppendCondition::is_violated_by` at `:225-234` fans
out to it. Over a store with the matching history removed the condition therefore
passes **vacuously**, and `condition_over_removed_history_does_not_reject` asserts
that as *specified behaviour* rather than desired behaviour (AC-005). Its registered
wrong implementation is a store that rejects (or errors) instead — the plausible
adapter that "helpfully" fails closed when it notices its own gap.

ES-40 is a **documentation** clause — *"The port's documentation MUST state…"*
(`spec/SPECIFICATION.md:4351-4356`) — so discharging it is rustdoc on
`AppendCondition` (`append.rs`, beside `is_violated_by`) and/or
`EventStore::append` (`store.rs:213`). Rustdoc prose is not a semver surface. **ES-40
is fully dischargeable inside AC-011**, and this is the mitigating fact `project.md`
already carries; the collision, if it comes, comes from ES-39.

### DA-6 — Resolving the open-question atom without closing the half this project does not own

`_grounding.md` §3 flagged the mechanism as unverified. It is written down:
[`.kb/open-questions/README.md`](../../../.kb/open-questions/README.md), *"Resolving
one"* — the answer is a **new atom**, the question's record **stays**, the two are
linked via `related`, the question's `status` moves to `withdrawn` or `superseded`,
and *"do not rewrite a question into its own answer"*. The status enum is
`.kb/README.md:21` (`draft · proposed · accepted · superseded · withdrawn`), and
`.kb/maps/open-questions-index.md`'s own summary says a withdrawn or superseded
question stays listed and annotated rather than removed.

The complication is that
[`es-38-and-gap-read-rules-are-unowned.md`](../../../.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md)
bundles **two** threads and this project owns one. Flipping the whole atom to
`withdrawn` would silently close `read_from_a_gap_position`'s ownership — exactly
the "settled in passing" defect `project.md` *Out of scope* fences against. So the
shape that satisfies AC-015 honestly is a three-artifact wave staged in
`.kb/_intake/` and landed by `/redkiln:kb-ingest`:

1. **ADR-0028** — the decision atom (or the reasoned refusal), answering sub-question
   3 on the record.
2. **A narrowed successor `open_question` atom** carrying only the
   `read_from_a_gap_position` ownership thread, still `accepted`, with its own
   `related` edge back to the original.
3. **The original atom** moved to `superseded`, `related` naming both, plus the
   bullet update on `.kb/maps/open-questions-index.md` the map's own summary
   requires.

If the ingest wave cannot express (3) as a mutation of an existing `open_question`
atom, that is a finding to report — not grounds for hand-editing `.kb/`, which is
what the revert at `0269720` exists to prevent. Operational note for whoever runs
the wave: the intake glob picks up the intake directory's own README, so drop it at
the approval gate, and suffix the wave id so it does not overwrite an earlier wave's
audit trail.

### DA-7 — The AC-012 escalation, pre-computed

Everything ES-39 can be answered *with* is a port-surface change, so if ADR-0028
decides rather than refuses, it escalates. Naming the option set now is what makes
the escalation a decision instead of a discovery:

| Option | Surface | Cost |
|---|---|---|
| `earliest_position()` (a floor) | new `EventStore` method | ES-39 argues against it by name: scattered purges are not prefixes, and it *"ships looking correct until a claim runs long"* (`spec/SPECIFICATION.md:4336-4341`) |
| a set of retained ranges | new method + a new public type | honest for scattered purge; every adapter must be able to compute it |
| a third outcome on condition evaluation | new variant on an `AppendError`/result type (`crates/happenstance-core/src/error.rs:214-248`) | reaches every caller's `match`, including the retry loop `CLAUDE.md` describes |
| **a tri-state `contains_event_id`** — see DA-8 | `Result<bool, _>` → a three-valued answer at `crates/happenstance-core/src/store.rs:268` | a *fourth* candidate ES-39 does not list; smallest signature delta, same semver class |

All four are minor bumps under 0.x, i.e. a `0.3.0` this initiative's exit criteria
do not contemplate (`../_decomposition.md` *Decisions taken at the gate*, item 4).
The escalation names the surface, the version consequence and the options, and stops
(AC-012). It does not make the change.

### DA-8 — `contains_event_id` is the existing vocabulary's only "I don't hold it", and it lies

`EventStore::contains_event_id` (`crates/happenstance-core/src/store.rs:268`) is the
membership question *"a replicating peer must be able to ask"*, and its own doc
explains why every store can answer cheaply: *"if the identifier's store half is not
its own incarnation the answer is `false`, and if it is, the question reduces to
whether that position exists"* (`:256-263`). Under forgetting that reduction is
exactly wrong — the position existed, this store minted it and acknowledged it, and
`false` now means *"never had it"* to every caller. A peer that re-sends on `false`
re-sends forever; `IngestStore::holds`
(`crates/happenstance-sync/src/ingest.rs:164`) is the same question one crate up
and inherits the same lie.

This is ES-39's question visible on a surface that **already exists**, and it is the
cheapest concrete demonstration for AC-006's ingest half — no new type, no surface
change, an observed wrong answer rather than an asserted one. Demonstrating it costs
nothing; *fixing* it is DA-7's fourth row.

### Key contracts and data flow

```
                 retained-set predicate (suffix | scattered)
                                  |
  rule --open()--> Fixture --connect()--> ForgettingHandle --+--> inner EventStore
                     ^                      | read   (filters the stream)   (whole log,
                     |                      | head   (highest RETAINED)      positions
              defaulted Capability          | append (delegates — positions   allocated
              + defaulted method            |         stay unique & monotonic  here)
              (DA-4), declined by           |         across the hole)
              every other fixture           | contains_event_id (false for forgotten — DA-8)
```

Readers, both mounted on the same instrument (AC-006):

```
  read_decision_model(store, query)  -> (retained events, last RETAINED position)
        -> AppendCondition::after_opt(last) -> append ADMITTED where destroyed
           history would have rejected           = ES-40's vacuous pass, silent

  projection runner (happenstance, HS-P0011) --checkpoint--> read(from = checkpoint)
        -> resumes across the hole; `from` names a position, not an index
           (crates/happenstance-testkit/src/suite.rs:1490-1531 already states the
            hazard: "a projection resuming across a gap then stalls forever with no
            error anywhere")

  IngestStore::holds(id) -> false for an event this store minted (DA-8)
        -> peer re-sends; ingest is unconditional, so nothing rejects and nothing says so
```

The three arrows above are the "reader fails loudly" experiment. AC-006 is met by
**observing** what each does — recorded as output, per `RUNBOOK.md:4658-4668` — not
by asserting in prose that it would. Where a reader cannot be made to fail loudly
without a surface the port does not have, that is recorded against AC-011 and fed to
DA-7, which is what AC-006's own second sentence asks for.

### Constraints inherited from Accepted decision atoms

| Atom | What it binds here | Tension |
|---|---|---|
| [ADR-0001](../../../.kb/decisions/0001-async-port-flavours.md) — async port flavours | The instrument implements one flavour without `#[async_trait]`; `read` returns the stream at the top level and is **not** `async` (`CLAUDE.md` constraints 1 and 3). `Fixture::Store: EventStore` (`contract.rs:125`) is the weaker bound and accepts both flavours — bind that, never `SendEventStore`, and import only one of the two names per module (constraint 4) | none — a filtering `read` is where this is easiest to break, DA-2 |
| [ADR-0013](../../../.kb/decisions/0013-position-assignment-and-visibility.md) — position assignment and visibility | ES-38 is *settled by* this ADR (`spec/SPECIFICATION.md:4299` says so); this project discharges an obligation ADR-0013 created and does **not** renegotiate ES-38's text. Its *"What this ADR leaves open"* section is the origin of the "Owner: phase 14" line | none — the obligation is inherited, not reopened |
| [ADR-0029](../../../.kb/decisions/0029-msrv-raised-to-1-97-1.md) — MSRV 1.97.1 | Let-chains are available; house style still prefers a `match` for a two-arm predicate (`crates/happenstance-core/src/append.rs:239-253`). New rule bodies follow that precedent | none |
| [ADR-0007](../../../.kb/decisions/0007-projection-runner-decodes.md) — projection runner decodes | The runner this project runs is `happenstance`'s over core's checkpoint pump; this project does not build one | latent: the runner does not exist in the tree yet (HS-P0011, rank 1). Composition root 5 |
| ES-37 `[FROZEN]` (`spec/SPECIFICATION.md:4274-4297`) | `EventStore` grows no delete / truncate / redact / compact / tombstone | **live** against AC-010 — DA-9 below |

**DA-9 — the redaction question (AC-010), answered against the code.** `Tag` is
`Tag(Cow<'static, str>)` (`crates/happenstance-core/src/tag.rs:79`) and its
`PartialEq`/`Eq`/`Ord`/`Hash` are all hand-written, each delegating to `as_str()`
(`:170-193`): tag equality **is** string equality, with no digest field a redaction
scheme could swap in. `EventParts` is `#[non_exhaustive]` and separates opaque
`data: Bytes` from queryable `tags: Tags` (`crates/happenstance-core/src/event.rs`),
which is the structural fact ES-37 cites — shredding covers `data` and cannot cover
`tags`. There is no store-side update path: `EventStore` has four methods and none
writes over an existing event, and `Event`'s fields are private with `with_tags`
constructing a new value (E2E-49, `spec/E2E-CASES.md:1288-1311`).

So a tag redaction needs **either** a new store operation (surface change → DA-7)
**or** a change to what `Tag` equality means, which silently re-keys every tag index
in every adapter — a far larger break than a signature. Inside AC-011 the reachable
answer is an **explicit deferral naming an experiment**, and the experiment must be
named because CF-38 makes an unnamed deferral a gate failure. `experiments/` is the
repository's home for measurements that are reproducible and not in the gate
(`CLAUDE.md`), which is where a tag-digest scheme measured against
`query_item_tags_are_and` and `query_item_tags_match_supersets` would live. Deciding
to redact is **not** available here: it amends a `[FROZEN]` clause, which
`project.md` *Out of scope* routes to a new decision atom and a re-plan.

### Gate mechanics AC-014 must satisfy

`cargo xtask spec-trace` (`xtask/src/spec_trace.rs`) runs nine checks and three of
them react to this project's edits in ways that are easy to trip:

- **Rule names resolve against `suite.rs` only** (check 4,
  `xtask/src/spec_trace.rs:700-711`). A rule body must live in
  `crates/happenstance-testkit/src/suite.rs`; a body in a `tests/` file is invisible
  to the checker. A clause may name a rule that does not exist yet **only** while its
  `Rule:` line carries `(new)` or `†` (`:1626-1631`) — so when ES-38's and ES-40's
  rules are written, the `(new)` markers come out in the same change.
- **Every rule must be claimed by a clause** (check 6, `:727`). Adding
  `suffix_store_is_distinguishable_from_a_young_store` to the registry without CF-27
  claiming it fails the gate; CF-27 already names it, so the refusal branch of AC-007
  is the branch where the rule is *not* written and CF-27's text explains why.
- **§7.1/§7.2 are generated; §1.3 is not** (checks 8 and 9, `:39-57`). Moving a
  marker changes the census, so `cargo xtask spec-trace --write`
  (`xtask/src/main.rs:671-674`) regenerates the two tables and **§1.3's prose
  numbers are hand-edited by a human** — deliberately, because generating them would
  let a parser regression shift the census and the table together and stay green.
- Also in the gate and separate from `spec-trace`: `cargo xtask lints`' *"no retired
  rule is still live"* step (`xtask/src/main.rs:342`) — relevant only if this
  project retires anything, which it should not.
- A `[PROVISIONAL]` clause must name a falsifier. ES-40's names *"the suffix store of
  CF-27"*; when the instrument lands, that marker is discharged or renewed against a
  named experiment, never simply left.

The project's own bar is `cargo xtask ci --fast` (`.redkiln/config.yaml:55`), the
non-terminal integration grain; the whole gate on the assembled tree is HS-P0019's.

### Acceptance Criteria

Brief-grain, testable, each tracing to `project.md`'s spine.

- **AC-A01** — The instrument is mounted through
  `happenstance_testkit::event_store_conformance!` in a `crates/happenstance-testkit/tests/`
  target, not in `src/fixtures/`, and no item of the three publishable crates' public
  API names it. *(AC-001, AC-013; `crates/happenstance-testkit/Cargo.toml:25`)*
- **AC-A02** — The instrument is parameterised by a retained set and the experiment is
  run in **both** a suffix and a scattered configuration, with a pass list recorded per
  configuration. *(AC-002; DA-1)*
- **AC-A03** — Positions across a hole remain unique and strictly monotonic, and the
  construction that renumbers is registered as a mutant rather than shipped as the
  instrument. *(AC-003, AC-004; DA-2)*
- **AC-A04** — The fixture presents an empty store at `connect()`; no rule in the
  recorded failure set fails for a seeding reason. *(AC-002, AC-003; DA-3)*
- **AC-A05** — Any new `Fixture` item is a **defaulted** const and/or a **defaulted**
  method, and every existing `Fixture` impl in the workspace compiles unchanged.
  *(AC-011; DA-4)*
- **AC-A06** — Both new rules appear in `for_each_event_store_rule!` and each has a
  row in `mutation_coverage.rs`'s `REGISTRY` with a non-empty `fails` list, with
  `mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive`
  green. *(AC-004, AC-005)*
- **AC-A07** — ES-40's sentence lands as rustdoc on the append/condition surface, and
  the surface diff records **no** signature change to `happenstance-core`.
  *(AC-005, AC-011; DA-5)*
- **AC-A08** — Each of the three readers (composition roots 4, 5, 6) is run against
  the instrument and its actual output recorded; any that cannot be made to fail
  loudly is written up against AC-011 with the missing surface named. *(AC-006, AC-012)*
- **AC-A09** — The open-question resolution lands as a new atom plus a narrowed
  successor question, with the original `superseded` and the index bullet updated,
  through `/redkiln:kb-ingest`; `redkiln validate --kb` clean. *(AC-008, AC-015; DA-6)*
- **AC-A10** — AC-010's answer is a decision or a deferral naming a real experiment
  path; no unnamed deferral. *(AC-010; DA-9)*
- **AC-A11** — If a surface change is concluded necessary, the escalation states the
  surface, the version consequence and DA-7's option table, and no such change is
  made. *(AC-012)*
- **AC-A12** — `cargo xtask spec-trace` green after the marker moves, with §1.3
  hand-reconciled and §7.1/§7.2 regenerated in the same commit. *(AC-014)*

### Notes

Non-prescriptive, and each is a place the implementer should exercise judgement
rather than follow this brief.

- **Sequencing inside the project.** Build the instrument and run the suite
  *before* writing either rule. The pass list is the evidence base for ADR-0028
  (AC-002), and a rule written first will be written to the answer someone
  expected. DA-3's three-way outcome is decided by that run, not before it.
- **Naming.** `suffix` is CF-27's word and DA-1 widens the thing it names. If the
  implementer keeps the clause's word for the type, say in its own docs that the
  retained set is arbitrary, so the next reader does not infer a floor from an
  identifier.
- **The decorator's generality is a cost, not a virtue.** CF-27 says "over any
  `EventStore`", and a fully generic decorator over `S: EventStore` is more work
  and buys nothing this project needs — one inner store is enough to run the
  experiment. If genericity is dropped, record it as a deliberate narrowing of
  CF-27's text so the clause and the instrument agree.
- **Do not settle `read_from_a_gap_position`'s ownership.** It is one clause away,
  it already has a body (`crates/happenstance-testkit/src/suite.rs:1490`), and its
  narrative — a projection stalling forever across a gap — will look like this
  project's subject while it is being written. It is not (see DA-6, and
  `project.md` *Out of scope*).
- **The refusal branch is not cheaper.** DR-9 keeps the instrument and the
  illustration under refusal, so the only thing refusing saves is DA-7's
  escalation. Choosing it on schedule grounds at rank 5 is the risk `project.md`
  names, and it will be visible in ADR-0028's reasoning.
- **What to do if HS-P0017 has not landed the ingest write path.** Composition root
  6's bodies are `todo!()` today for a reason recorded in that module. If the write
  path is still missing when this project starts, run the ingest half against
  whatever seam HS-P0017 actually delivered and record the gap — do not stub a
  fake ingest to produce an observation, which would demonstrate a hazard against a
  reader nobody ships.

---

## Testing brief

### Intent

Give the implementer the test mix, the merge-gate commands and the fixtures to
build against for this project's one deliverable, grounded in `CLAUDE.md`'s
*Commands* section and `.redkiln/config.yaml`'s `verify:` block — the same
commands that fire automatically at each redkiln stage transition, not a
parallel checklist. This project ships no new crate, no adapter and no
application surface, so there is no UI e2e tier; the nearest thing this repo
has to one is the **conformance suite itself**, run in full against a real
`EventStore` implementation through `event_store_conformance!`
(`crates/happenstance-testkit/src/contract.rs`), plus the three reader
experiments (composition roots 4–6 in the architecture brief) that are
*observed*, not asserted.

Every project AC-001 – AC-016 is mapped to a tier below, following the same
table shape `publication-and-positioning`'s testing brief already used for
this initiative
(`.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md:460-479`) —
reused here for consistency, not copied blind: this project's tiers differ
because it ships tests, not release tooling. Per `CLAUDE.md`'s corollary, no
row below asserts a tier without naming the wrong implementation that tier is
built to reject — a tier that cannot fail is decorative.

### Test mix, mapped to every project AC

| AC | Tier(s) | Instrument | Wrong implementation it must reject |
| --- | --- | --- | --- |
| AC-001 | integration (new `tests/` target) + unit (`Fixture` impl compiles against both `EventStore` flavours) | `happenstance_testkit::event_store_conformance!` invoked on the suffix/scattered fixture, mounted exactly as `crates/happenstance-testkit/tests/fixture_instruments.rs:201-205` mounts `DurableFixture` | the instrument built but never wired into the macro — compiles, proves nothing, and is "not an adapter" under `CLAUDE.md`'s rule that matters |
| AC-002 | integration (the suite run itself, in two configurations per DA-1) + the recorded pass list as a committed artifact, not a test assertion | the same `event_store_conformance!` mount, run once with a suffix-retained predicate and once with a scattered one | reporting "it passed" instead of the enumerated rule-name list — DR-2's own text names this as the failure the AC exists to catch |
| AC-003 | integration — discharged by AC-004's and AC-005's mutants, per DA-3(2); **not** a property of the honest instrument, which DA-3 predicts passes everything | `mutation_coverage.rs`'s `REGISTRY` rows for the two new mutants, run through `mutants_fail_exactly_their_declared_rules` (`:2889`) | treating a pre-seeded or eagerly-forgetting fixture as the source of the two failures (DA-3's "dishonest resolutions") — the meta-test only certifies failures that come from a registered mutant, not from a seeding bug |
| AC-004 | unit (one `REGISTRY` row + one `Defect` impl) + integration (`positions_are_not_reused_after_removal` green against the honest instrument, red against the mutant) | a `Defect` implementing DA-2(a)'s rejected construction — `MemoryEventStore::restore` from a truncated snapshot, which reuses `position_at` indices (`crates/happenstance-core/src/memory.rs:277-282`, `:386-389`) | the renumbering-on-compaction store ES-38's own `Rejects:` names (`spec/SPECIFICATION.md:4321-4323`) — proven wrong by `mutant_registry_is_exhaustive` (`:2754`) if the row's `fails` list is empty |
| AC-005 | unit (one more `REGISTRY` row) + integration (`condition_over_removed_history_does_not_reject` green against the honest instrument, red against the mutant) | a `Defect` overriding `Guard::is_violated_by` (`crates/happenstance-core/src/append.rs:239-253`) to reject or error instead of passing vacuously | the adapter that "helpfully" fails closed on its own gap — DA-5's named wrong implementation |
| AC-006 | **observation**, the closest tier this repo has to e2e — composition roots 4 (`read_decision_model`), 5 (the projection runner, arrives from HS-P0011) and 6 (`IngestStore::holds`) each run against the instrument and their actual output recorded, per `RUNBOOK.md:4658-4668`'s "observed... not asserted in prose" | `happenstance_core::read_decision_model` (`store.rs:321-331`) directly; the projection runner over `crates/happenstance-core/src/projection.rs`'s checkpoint pump; `IngestStore::holds` (`crates/happenstance-sync/src/ingest.rs:164`) | recording a *predicted* outcome in prose instead of the actual return value / actual runner state / actual `holds()` answer — DR-6's "not a confidently wrong answer" is itself the criterion a prose-only writeup would fail |
| AC-007 | static (`cargo xtask spec-trace` check 6, `xtask/src/spec_trace.rs:727`) + integration if written | `suffix_store_is_distinguishable_from_a_young_store`, registered in `for_each_event_store_rule!` **only** if ADR-0028 decides rather than refuses (DA-3(3), DA-7) | a rule added to the registry with no clause claiming it (spec-trace check 6 fails this), or a clause left claiming a rule that was never written and never marked `(new)`/`†` |
| AC-008 | static (process) | `redkiln validate --kb`, run after `/redkiln:kb-ingest` lands ADR-0028; `KbFrontmatter` conformance plus the accepted-decision-immutability check against `HEAD` | ADR-0028 hand-written directly under `.kb/decisions/` — the revert at `0269720` `CLAUDE.md` cites is the recorded cost of exactly this |
| AC-009 | static (content review, no compiled test) | `_design.md`'s DT-7 section, read for the option that lost and why | a `_design.md` that states a winner without naming what lost — the same bar `RS-70-5`-style doc obligations and `publication-and-positioning`'s AC-011 row already set for this initiative |
| AC-010 | static (content review) + CF-38's empty-falsifier prohibition, already a `spec-trace` check | AC-010's answer in ADR-0028, or a named path under `experiments/` per DA-9 | a deferral naming no experiment — CF-38 makes an *unnamed* deferral a gate failure by construction, not by this brief's invention |
| AC-011 | static — `cargo-semver-checks` at the PR grain (`CONTRIBUTING.md:291-296`) **and** the surface-diff gate step `publication-and-positioning` builds against the `0.2.0` registry baseline (cited by id and mechanism, not content — that baseline does not exist in this worktree yet, per `_grounding.md` §5) | every `Fixture`-trait addition this project proposes (DA-4) | a **required** trait item added instead of a **defaulted** one — breaks every existing `Fixture` impl in and out of the workspace, which is exactly what the semver tooling exists to catch and what DA-4's precedent (`MID_BATCH_FAULT`) was built to avoid |
| AC-012 | static (content review) | the escalation's own text: names the surface, the version consequence and DA-7's four-row option table | an escalation that recommends a change without naming which of DA-7's rows it is, or a change that lands in code instead of stopping at the finding — checked by confirming no diff touches `crates/happenstance-core/src/store.rs`'s or `append.rs`'s public signatures |
| AC-013 | static (process, mirrors `publication-and-positioning`'s `package-check` precedent) | the instrument's location (`crates/happenstance-testkit/tests/`, never `src/fixtures/`) and, if it gets its own crate, `publish = false` in its manifest; `cargo package --list` over the three publishable crates | the instrument landing in `src/fixtures/` — `crates/happenstance-testkit/Cargo.toml`'s own comment (`:25`) already states `MemoryFixture` there is "a *published* item," which is the precedent this AC exists to not repeat |
| AC-014 | static | `cargo xtask spec-trace` (checks 4, 6, 8, 9 per the architecture brief's *Gate mechanics*), then `cargo xtask spec-trace --write` (`xtask/src/main.rs:671-674`) to regenerate §7.1/§7.2, then a **hand** reconciliation of §1.3's prose census in the same commit | committing a marker move without regenerating the two tables, or regenerating the tables without hand-editing §1.3 — the gate is green in both wrong cases until the *next* unrelated marker move, which is why `spec-trace` checks the tables' shape rather than trusting the last author |
| AC-015 | static (process) | `redkiln validate --kb`, after the three-artifact wave (DA-6) lands through `/redkiln:kb-ingest`; check the original atom's `status` is `superseded` (not deleted) and `.kb/maps/open-questions-index.md`'s bullet is updated | flipping the whole atom to `withdrawn`/`superseded` without the narrowed successor atom — silently closes `read_from_a_gap_position`'s ownership, exactly what DA-6 exists to prevent |
| AC-016 | static (citation check) | ADR-0028's text, read for a citation to SY-32 by id rather than a re-derivation of its content | ADR-0028 restating or amending the retention-gap answer instead of citing it — the check is that ADR-0026/ADR-0027 are not touched by this project's diff |

### Merge-gate commands

This project is rank 5 on the serial trunk and is **not** the initiative's
terminal project — `closeout-and-durable-audience` (HS-P0019) is the one
wired to `verify.e2e` (`.redkiln/config.yaml:60`). So the commands that fire
automatically at each redkiln stage transition are:

- **Story grain** — `cargo xtask affected --base {{base}}`
  (`.redkiln/config.yaml:40`): maps the diff to workspace packages plus
  dependents, runs fmt/clippy `-D warnings`/tests for that set, and runs the
  five file-reading lints and `spec-trace` unconditionally — so a story that
  only edits `spec/SPECIFICATION.md` (most of AC-007's and AC-014's stories)
  is still gated on something.
- **Integration grain, cheap tripwire** — `cargo xtask lints && cargo xtask
  spec-trace` (`.redkiln/config.yaml:48`).
- **Integration grain, non-terminal** — `cargo xtask ci --fast`
  (`.redkiln/config.yaml:55`), the bar `project.md`'s Definition of done
  already names. Drops the two feature powersets, `cargo deny` and the
  nightly `--cfg docsrs` build; keeps fmt, clippy, tests, the four mandatory
  `wasm32` steps, `spec-trace`, and the doc builds.
- **Not wired here, run by hand where this project's own ACs need it**:
  `redkiln validate --kb && redkiln doctor` for AC-008 and AC-015 (KB
  conformance is not part of `verify:`'s command set); `cargo-semver-checks`
  for AC-011, at the PR grain per `CONTRIBUTING.md:291-296`, which proves
  *this* diff does not break the API it branched from — not that the
  registry baseline is clean, which is `publication-and-positioning`'s
  surface-diff step once the `0.2.0` baseline exists.
- **`cargo xtask ci` (the whole gate, `.redkiln/config.yaml:60`) is
  HS-P0019's**, not this project's, per DoD's own text: "The whole gate on
  the assembled tree is `closeout-and-durable-audience`'s." Running it here
  is a useful local sanity check, never the recorded proof artefact.

### Fixtures and seams to mock

- **`crates/happenstance_testkit::fixtures::MemoryFixture`** — the reference
  `Fixture` impl (`project.md`'s own Code anchors) to model the new
  suffix/scattered fixture's `connect()` and capability declarations on;
  DA-3's "present an empty store at `connect()`" requirement is exactly
  `MemoryFixture`'s own shape.
- **`DurableFixture` (`crates/happenstance-testkit/tests/fixture_instruments.rs`)**
  — precedent for fixture *ergonomics only*: an axis-specific,
  testkit-adjacent instrument wrapping a live `MemoryEventStore`, mounted via
  `event_store_conformance!` at `:201-205`. **Not** a precedent for the
  instrument's *construction* — `DurableFixture::reopen` rebuilds via
  `MemoryEventStore::restore`, which is DA-2(a), the rejected shape and
  ES-38's own mutant. Copy the mounting pattern; do not copy the rebuild.
- **`mutation_coverage/mutants.rs`'s `REGISTRY`
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:324`)** — the seam
  for both new wrong implementations (AC-004, AC-005). No parallel harness;
  two new rows, each a `Defect` impl, per the existing eight.
- **`happenstance_core::read_decision_model`
  (`crates/happenstance-core/src/store.rs:321-331`)** — run directly against
  the instrument for AC-006's read half; nothing to mock, it is real
  production code exercised over a real (if forgetful) `EventStore`.
- **The projection runner (`happenstance`, over
  `crates/happenstance-core/src/projection.rs`'s checkpoint pump)** — arrives
  from HS-P0011, rank 1. If it does not exist when this project starts, that
  is a dependency failure to halt on and report, not a seam to stub — a
  hand-written test double would demonstrate a hazard against a reader
  nobody ships (the architecture brief's own closing note).
- **`IngestStore` (`crates/happenstance-sync/src/ingest.rs`)** — `holds()` is
  the one method with a real body today and is DA-8's cheapest concrete
  demonstration; the write path is `todo!()` pending HS-P0017, so AC-006's
  ingest half is scoped to whatever seam has actually landed by the time this
  project runs (`_grounding.md` §5) — never a fake ingest built to produce an
  observation.
- **`crates/happenstance-testkit/src/contract.rs`'s `Capability` /
  `Option<usize>` split** — if a fixture-side seam is added at all (DA-4), it
  is a **defaulted** `Capability` const plus a **defaulted** method, mirroring
  `MID_BATCH_FAULT` (`:207-211`, `:297-307`) exactly; this is what AC-011's
  test row above checks for.

### Acceptance Criteria

- **AC-T01** — every project AC-001 – AC-016 has at least one row in the *Test
  mix* table above, each naming the wrong implementation its tier rejects; no
  row is added that cannot fail (`CLAUDE.md`'s decorative-gate corollary).
- **AC-T02** — the suffix/scattered instrument is reachable **only** through
  `event_store_conformance!` in `crates/happenstance-testkit/tests/`, never
  through a hand-rolled test driver, per AC-001's row.
- **AC-T03** — ES-38's and ES-40's rules are proven by the existing
  `mutation_coverage.rs` mechanism (`REGISTRY` + `for_each_mutant!` +
  `mutants_fail_exactly_their_declared_rules` +
  `mutant_registry_is_exhaustive`) and by no other harness.
- **AC-T04** — the pass list (AC-002) and the three reader observations
  (AC-006) are committed as recorded data — an enumerated rule-name list and
  actual runner/store output — never summarised as "it worked" in prose.
- **AC-T05** — `cargo xtask ci --fast` is green before this project's stage
  advances past `integration_scoped`; `cargo xtask ci` (the full gate) is
  exercised at most as a local sanity check and is not treated as this
  project's proof artefact.
- **AC-T06** — `redkiln validate --kb` is clean after ADR-0028 and the
  open-question wave land, reporting the expected shape (a new decision atom,
  a narrowed successor `open_question` atom, the original moved to
  `superseded`) rather than an atom edited in place.

### Notes

- **Run the honest instrument before writing either new rule.** The
  architecture brief's own sequencing note applies here unchanged: the pass
  list is evidence, and a rule written before the run is written to the
  answer someone expected, which defeats AC-002's purpose.
- **The `0.2.0` baseline is a citation, not a fixture, until `publication-and-positioning`
  ships.** Do not construct a synthetic stand-in registry snapshot to run
  `cargo-semver-checks` against early — `CONTRIBUTING.md:291-296`'s own
  caveat ("the registry baseline... is not available yet") is about exactly
  this gap, and this project inherits it rather than working around it
  (`_grounding.md` §5).
- **AC-006's tier is "observation," not "assertion," on purpose.** DR-6 and
  `RUNBOOK.md:4658-4668` both ask for what a reader actually does, not a
  predicted description of what it would do — resist writing a
  `#[should_panic]`-style test here unless a reader's real failure mode
  turns out to be a real panic; if it is a silent wrong answer instead (the
  more likely case per ES-40), the correct test is one that captures and
  asserts on the *actual wrong value*, per CF-2's discipline against tests
  that record only that something failed (`crates/happenstance-testkit/tests/mutation_coverage.rs`'s
  own module doc, "Why the wrong implementations are no longer here").
- **Do not let AC-011's static check become the only check.** `cargo-semver-checks`
  proves the API surface is unchanged; it does not prove `mutants_fail_exactly_their_declared_rules`
  stays green after a `REGISTRY` row is added, or that `spec-trace` stays
  green after a marker moves. All three gate mechanisms are independent and
  this brief's table keeps them that way — collapsing them into "the gate is
  green" is the loss of information DR-2 and AC-002 are written against one
  level up.
