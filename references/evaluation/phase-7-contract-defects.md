# Phase 7 — what using the frozen contract revealed

- **Date:** 2026-08-16
- **Pinned to:** `78a2170c1d06bad5eec34915b0b3682f524ec91f`
- **Produced by:** HS-P0011 *Typed layer and alpha release*, story
  `defect-log-and-macros-verdict` (HS-S0032), discharging initiative **BR-01**
  and project **AC-012**.
- **Lifecycle:** immutable evidence. Superseded rather than edited — a later
  phase that disagrees with an entry writes a superseding document. The one
  permitted in-place change is repointing a `file:line` citation at the text it
  already named (`README.md:268-270`).

This is the record BR-01 exists to produce: every defect that **using** the
frozen `EventStore` / `Query` / `ProjectionStore` contract revealed while the
typed layer was built on top of it. Phase 7 is the first time anything in this
repository consumed that contract in anger, and the whole premise is that
defects of this kind are found by use rather than by review.

**Nothing here is fixed.** Every entry names a clause and a route; not one is
repaired by a line edit. That is the discipline the phase order exists to
enforce: phase 7 precedes phase 8 so that the first consumer's findings are
recorded before six adapter authors pin the crate (`RUNBOOK.md:256-258`). The
rightness of a fix is not the discriminator — the discriminator is who takes the
decision and whether it leaves a record for the people who will have built on
it.

## How to read an entry

Every entry carries the same six fields, under their own labels, in this order.
An entry missing the clause ID or the routing is not an entry.

| Field | What it is for |
| --- | --- |
| **id** | The handle other documents cite. Stable once written |
| **Clause** | The clause ID it bears on **and that clause's maturity marker**. Without it nobody can tell which promise is at issue |
| **Attempted** | What was being written, concretely, with the call site as `path:line` |
| **Contract** | What the contract did instead |
| **Why a defect** | The discriminator between a contract defect and a misuse, stated rather than left for the reader to guess |
| **Routing** | Where the finding goes next. Never *"fix it"* |

Where an entry has a proposed fix, it is recorded **inside** the entry so the
eventual decision record starts from it — and it is deliberately subordinate to
the clause and the routing. The judgement about whether a fix is right is the
most recessive thing in an entry, not its conclusion.

---

## The entries

### D-1 — no infallible `QueryItem` constructor for pre-validated inputs

**Clause:** **VT-18** — *Constructors accept values the caller already holds,
and their errors compose* (`spec/SPECIFICATION.md:1374-1386`). **`[FROZEN]`**
(`spec/SPECIFICATION.md:8858`).

**Attempted.** `Boundary::query` builds a `QueryItem` from values that are
*already* validated — `EVENT_TYPES` is `const`-constructed and a `Tags` in hand
has already been through `Tag::key_value` — so the derivation is total by
construction. `crates/happenstance/src/boundary.rs`, over scopes built at
`examples/course-subscriptions/src/main.rs:312-317`.

**Contract.** `QueryItem::new` returns `Result<Self, InvalidQuery>` regardless
(`crates/happenstance-core/src/query.rs:48-62`, whose own `# Errors` section
explains the fallible-conversion bound that forces it). Every derived query
therefore carries a `Result` that is unreachable for a well-formed model.

**Why a defect and not a misuse.** VT-18's sentence is *"constructors accept
values the caller already holds"*, and here the caller holds **every** value and
still pays. `Event::new` does accept a held `EventType`, so the clause is not
contradicted — the principle is half-implemented across the `Query` family.
**And it is worse than first recorded:** `Boundary` is **sealed**, so no
downstream type can be a failing `Boundary` and the error arm this creates is
untestable from outside the crate (`decision-model-composition`'s report).

**Proposed fix, subordinate to the routing.** A `QueryItem::from_validated`
taking `Vec<EventType>` and `Tags`. **It may well be the right answer.** The
objection is to a `[FROZEN]` clause's surface being amended by whoever was
mid-implementation, with no record of the alternatives.

**Routing.** A decision record, through `/redkiln:kb-ingest`, staged at
`.kb/_intake/contract-defect-log-phase-7.md`. **Never a line edit** — AC-A02,
and `_storymap.md:152-154`: no story in this project is licensed to amend a
`[FROZEN]` clause.

**Provenance.** `_design.md:652-672`, written before implementation began;
carried into `references/adr/0020-fold-query-agreement.md:268-294`.

---

### D-2 — `DomainEvent::tags` is infallible over a fallible `Tags`

**Clause:** **VT-18** (`spec/SPECIFICATION.md:1374-1386`), **`[FROZEN]`**. The
same clause as D-1 and a different face of it: D-1 is *the caller holds
everything and still pays*; D-2 is *the caller cannot pay at all, because the
signature is total*.

**Attempted.** Implementing `DomainEvent for Enrolment` in the worked example,
at `examples/course-subscriptions/src/main.rs:224-234`, where the tag values are
runtime strings — a course id and a student id supplied by the caller. `fn
tags(&self) -> Tags` is infallible.

**Contract.** `Tag::key_value` and `Tags::from_pairs` both return `Result`, so
there is **no total path** from a runtime string to a `Tags` inside `tags()`.
The implementor's routes are an `unwrap`, a panic, or a validated newtype that
holds the `Tag` and hands back a clone.

**Why a defect and not a misuse.** The third route is what the example took, at
a cost of 81 lines of `CourseId`/`StudentId`
(`examples/course-subscriptions/src/main.rs:102-182`), whose doc says so in
terms at `:104-109`. It is the right answer for an application and it is not
something the contract asks for anywhere: a reader meeting `fn tags(&self) ->
Tags` has no way to learn that a validated newtype is a prerequisite until they
write the `unwrap` the house style forbids.

**Proposed fix, subordinate to the routing.** An infallible `Tags` path over
already-validated `Tag` values — half of it exists, since `Tags:
FromIterator<Tag>` is what the example uses at `:226-232`; the gap is that
getting the *first* `Tag` is fallible. The alternative, `tags()` becoming
fallible, is a breaking change to a frozen surface.

**Routing.** A decision record, staged with D-1. **Not fixed here:** no edit was
made to `crates/happenstance-core/**` or `crates/happenstance/**`
(`worked-example-on-typed-layer`'s NF-008, AC-A02).

**Provenance.** `worked-example-on-typed-layer`, report.md:57-62 — the first
implementor of `DomainEvent` outside the library's own tests.

---

### D-3 — CF-36 names a cross-reference nothing performs

**Clause:** **CF-36** — *A clause backed only by integration- or scenario-level
cases MUST NOT name a conformance rule* (`spec/SPECIFICATION.md:8611-8622`).
**`[FROZEN]`** (`spec/SPECIFICATION.md:9062`).

**Attempted.** `projection-clause-verdicts` relied on CF-36 to decide **where
its tests live**. The check it relied on is named at
`spec/SPECIFICATION.md:8613-8614` and would live in `xtask/src/spec_trace.rs`.

**Contract.** CF-36's `Rule:` line reads *"`cargo xtask spec-trace` (CF-38),
cross-referencing each case's level marker (`E2E-CASES.md:19-28`)"*. The checker
reads no level marker at all: `grep -c "Level" xtask/src/spec_trace.rs` returns
**0**. So a frozen clause cites a cross-reference nothing performs, and the
constraint it exists to protect is enforced by review alone.

**Why a defect and not a misuse.** A clause's `Rule:` line is a claim that
something checks it, and `spec-trace` is a gate step precisely so such claims
cannot rot into decoration. A green `spec-trace` currently reads as evidence for
CF-36 and is not — the same failure class the traceability step exists to
prevent, one level up: nothing checks that a checker performs the check it is
cited for.

**Proposed fix, subordinate to the routing.** Implement the cross-reference in
`xtask/src/spec_trace.rs`, or supersede CF-36 with a clause whose `Rule:` line
is true. Both are decisions rather than patches.

**Routing.** A decision record, staged with D-1. **Neither
`xtask/src/spec_trace.rs` nor CF-36's body was edited to make the finding go
away** — this document's PR touches no path under `xtask/` or `spec/`, and that
is checkable on the diff.

**Provenance.** `projection-clause-verdicts`, implementation-report.md:117-126;
routed here by name at that story's `spec.md:405`, `:452`.

---

### D-4 — no `PS` rule name is resolved, and `†` states a fact twice

**Clause:** **CF-38** — the traceability step itself — with **PS-27** and
**PS-30** as the visible symptom. CF-38 is `[FROZEN]`.

**Attempted.** Reading §7.2's generated table (`spec/SPECIFICATION.md:9062` and
the `PS` rows above it) to find out whether a `PS` clause's named rule exists —
which is the question the table is generated to answer.

**Contract.** `xtask/src/spec_trace.rs`'s check 4 short-circuits on
`!has_suite(&c.id)` (`:695-697`), so **no `PS` clause's rule name is resolved at
all** today. A green `spec-trace` is not evidence that a `PS` rule exists.
Separately and visibly, PS-27's row renders `` `skip_and_record_is_atomic` † ``
and PS-30's `` `panicking_apply_rolls_back` † `` — *must be written* — against
clause bodies that already say, in terms, that those rules deliberately do not
exist yet and name HS-P0010 as the owner.

**Why a defect and not a misuse.** The dagger is not *wrong*; it is **redundant
with the marker and carries no owner**, so a reader who trusts the generated
table learns strictly less than one who reads the clause — which inverts the
reason the table is generated. The short-circuit is the load-bearing half: it
means the reassurance a green `spec-trace` gives about the `PS` family is
unearned.

**Proposed fix, subordinate to the routing.** Resolve `PS` rule names against
the projection rules file rather than short-circuiting, and then decide whether
the `†` convention survives at all now that maturity markers carry owners.

**Routing.** A decision record, staged with D-1. Not fixed here; `xtask/` is
outside this PR.

**Provenance.** `projection-clause-verdicts`, implementation-report.md:128-136.

---

### D-5 — `&mut P` makes N projections cost N reads, at the API level

**Clause:** **PS-31** and the projection port family, `[PROVISIONAL]` — the port
has no conformance suite yet, and CLAUDE.md's *Open questions* says a port
without one is a guess.

**Attempted.** `polling-cost-measurement` is the first consumer of
`run_projection` (`crates/happenstance/src/runner.rs:401-411`) from outside the
workspace, driving several views over one log.

**Contract.** `run_projection(events, models, &mut P, codec, chunk)` takes the
projection by exclusive reference and drives exactly one. N read models
therefore cost N independent reads of the same event stream. The runner's own
documentation states this and defends it — the write set is **owned** and cannot
be shared between tasks (`crates/happenstance/src/runner.rs`, *One projection
per call*).

**Why a defect and not a misuse.** It is recorded as a **shape**, not a wrong
answer: the defence is sound for the alpha, and a fan-out runner would force one
failure policy onto every projection an application runs, which is the argument
the runner's own page makes. What makes it belong here is that the tail-seam
decision will meet this shape and should meet it with a measurement already on
the record rather than as a discovery. The measurement is `experiments/` and the
story's own README §5.

**Proposed fix, subordinate to the routing.** None proposed. This entry exists
to be *cited by* the decision that eventually settles fan-out, not to pre-empt
it.

**Routing.** A decision record, staged with D-1, as evidence toward the
tail-seam decision. Explicitly **not** ES-32's disposition, which is HS-P0016's
and `projection-clause-verdicts`'.

**Provenance.** `polling-cost-measurement`, report.md:76-79.

---

## Findings that are **not** entries, so their absence is not read as a gap

Two findings were surfaced by the same use and are **not** entered above,
because an entry that cites a clause which was never at issue routes a decision
at a promise nobody broke — the failure EC-001 forbids by name. Recording the
reasoning is what stops the next reader assuming they were dropped.

### N-1 — `run_projection` needs a caller-side bound. **Not a defect**

`edge-flavour-and-wasm-claim` bound a generic function `S: SendEventStore + Send
+ Sync + 'static` over `run_projection` and it did not compile:

```
error[E0277]: `<S as SendEventStore>::Error` cannot be sent between threads safely
note: required because it appears within the type
      `happenstance::runner::Stopped<<S as SendEventStore>::Error, …>`
```

On a failure the runner holds the stop — carrying `S::Error` — across the port's
`rollback` await (`crates/happenstance/src/runner.rs:480`), because one
`ProjectionError` needs the error **and** what the rollback said. RS-25-4's
*collapse before the next await* is unavailable there: the value that must
survive the await **is** the error.

**Why this is not an entry.** **ES-6** is `[FROZEN]` and leaves `Error`
unbounded deliberately (`happenstance-cloudflare`'s error holds an `Rc<str>`),
and **ADR-0009** settled where the obligation is paid, in terms: *"a caller who
needs the error itself across an await adds that bound to its own signature and
pays for it there; the port never grows one."* ADR-0009 also supplies the shape
— a marker trait with a blanket impl, declared by the consumer, and explicitly
**not** shipped in `happenstance-core`. The promise was kept. What was missing
was that nobody had written it down where a caller meets it, and that is now
`crates/happenstance/src/runner.rs:297-310`.

### N-2 — dead code in eight `wasm32` combinations. **`support`: no clause ID**

`cargo hack check -p happenstance-core … --target wasm32-unknown-unknown
--feature-powerset --no-dev-deps` emits `warning: method read_through is never
used` (`crates/happenstance-core/src/projection_memory.rs:233`) in eight
combinations. CI sets `RUSTFLAGS: -D warnings` ambiently
(`.github/workflows/ci.yml:18`), so there it is a failure rather than a warning.
It **pre-dates phase 7** — the powerset step covered `happenstance-core` before
the typed layer joined it.

**Classification: support, made at the moment of the finding.** It bears on no
clause; it is an incidental defect in a fixture's private method.
`.redkiln/config.yaml:5` declares `support_initiative: support` and
`project.md:270` routes incidental bugs there rather than expanding this
project. **No clause ID was invented to promote it into an entry.** Filing the
item is a `redkiln new` handoff and is not done by this PR: `.bklg/support/**`
is outside the boundary, and this line is the record that the hand-off is owed.

---

## Reconciliation — every story that declared a routing into this log

A story's silence is indistinguishable from a finding that was dropped, so
absence is written down rather than inferred. Sweep re-run at the pinned commit:
`rg -l "defect log|AC-012"
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/*/spec.md`.

| Story | Declared routing | Disposition |
| --- | --- | --- |
| `adr-0020-fold-query-agreement` | D-1, *"logged and routed, not repaired"* (report.md:31) | **D-1** |
| `domain-event-and-decision-model` | EC-008 (spec.md:459); its report raises the unreachable `<= 35`-line fence budget | **Found none of this kind.** The budget finding is a *design* budget, not a contract clause; it is AC-013's measurement subject and is carried in the macros verdict, not here |
| `command-loop` | spec.md:419 | **Found none.** report.md:75-78 — *"nothing under `crates/happenstance-core/src/**` moved, and no new defect was found"*; the same shape as D-1, already recorded |
| `codec-and-feature-forwarding` | AC-012's log | **Found none.** report.md:61 — *"no defect candidate was added; D-1 is inherited from M2 unchanged"* |
| `decision-model-composition` | spec.md:483, `:572` | **Strengthened D-1** rather than adding an entry: the seal makes D-1's error arm untestable from outside the crate. Folded into D-1 above |
| `given-when-then-dsl` | spec.md:430 | **Found none in the contract.** Two defects it did find were in slice-mates' own files and landed as their own commits under that story's trailer — neither reaches the frozen contract |
| `misbehaving-testkit-stores` | spec.md:427 | **Found none**, stated in terms at implementation-report.md:162-163 — *"no defect in the frozen contract was found, so nothing is routed to `defect-log-and-macros-verdict` from here"* |
| `projection-trait-and-runner` | EC-010 (spec.md:526) | **Found none.** EC-009 did not fire; HS-P0010's `MemoryProjectionStore` had landed (report.md:36) |
| `projection-clause-verdicts` | AC-010, three findings by name | **D-3** and **D-4**; the third (a private module shadowing a glob re-export) is carried below as a standards candidate rather than a contract defect |
| `polling-cost-measurement` | spec.md:343, AC-008 | **D-5** |
| `worked-example-on-typed-layer` | spec.md:531 | **D-2** |
| `edge-flavour-and-wasm-claim` | spec.md:191 — ES-2/ES-3/ES-6 route here if contradicted | **Found none.** ES-6 is honoured; see **N-1** for why the near-miss is not an entry, and **N-2** for the support hand-off |
| `adr-0021-payload-evolution-and-codec-tag` | AC-005's *"yes"* route (report.md:60) | **Not taken.** The hook answer came back *"no hook"*, so the AC-012 route it reserved was never exercised |
| `compile-fail-proof-artefact` | — | Declares no routing. Listed so the sweep's own coverage is checkable |

**One finding that is neither a contract defect nor a support item.** From
`projection-clause-verdicts`: *a private module can shadow a glob-re-exported
one, and nothing in the tree said so.* `mod projection;` in `happenstance`
silently shadowed `happenstance_core::projection` reached through `pub use
happenstance_core::*;`, producing only a `hidden_glob_reexports` warning.
`_design.md`'s anti-pattern 14 is written about *type* names; this arrived
through a *module* name. It bears on no clause and it is not a bug — it is a
**missing rule**, and its destination is a candidate atom under
`standards/rust/`. Staged with the rest, for the ingest wave to adjudicate.

---

## What this document does not claim

It does not claim to be complete. Which defects exist beyond those above is
unknowable by construction: the log's premise is that they are found by *use*,
and the use so far is M2 through M6 of one project. What it does claim is that
every routing declared by those stories has a disposition, and that no finding
was absorbed by a convenience edit — which is checkable on this PR's diff,
because it touches no path under `crates/**` or `spec/**`.
