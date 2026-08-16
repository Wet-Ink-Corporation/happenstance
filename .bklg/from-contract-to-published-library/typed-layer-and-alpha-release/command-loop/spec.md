---
item: HS-S0023
stage: spec
created: 2026-08-12T13:46:19.582Z
updated: 2026-08-12T13:46:19.582Z
template_sig: 87bbf1d0
rendered_sig: 58f63e5b
---

# Spec — The command loop — read, decide, append, retry on ConditionViolated

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/command-loop/spec.md` |
| Key brief — architecture | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md`, *Composition roots*, *The contracts this layer consumes unchanged — and five ways to get them wrong* |
| Key brief — ux | same file, *Interaction-quality invariants* (AC-U05, AC-U06, AC-U09, AC-U10, AC-U12, AC-U13, AC-U15) |
| Key brief — testing | same file, AC-005 / AC-009 rows of the test-level matrix |
| **Signed-off design (BINDING)** | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` — approved 2026-08-12, *Sign-off* |
| Story map (this story's row) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md`, M3 `codec-and-command-loop` |
| Roadmap pointer | `RUNBOOK.md:4015-4020` (phase 7's command-loop bullet and its retry-policy obligation) |

Traces to project **AC-005** (`project.md`, *Acceptance criteria*). Depends on
`domain-event-and-decision-model` and `codec-and-feature-forwarding`.

## One-line PR slice

Land read → decide → append → retry-on-`ConditionViolated` as `happenstance::commit` /
`commit_with`, taking its `after` anchor from the read (`read_decision_model`,
`crates/happenstance-core/src/store.rs:321`) and never from `append`'s return
(`:131-145`), treating `conflicting_position` as the hint it is
(`crates/happenstance-core/src/error.rs:139-147`), with a bounded, caller-visible retry,
a `#[must_use]` outcome value and the policy stated in the loop's own rustdoc beside the
*verbatim resubmission* distinction (`RUNBOOK.md:4015-4020`).

## Executive summary

**What this PR lands.** Four public items at `crates/happenstance`'s crate root —
`commit`, `commit_with`, `Retry`, `Committed`, plus the `CommandError` they return — and
the module-doc bullet that today reads *"**The command loop** — read, decide, append,
retry on `ConditionViolated`"* under the heading **Planned**
(`crates/happenstance/src/lib.rs:47-48`) becomes an intra-doc link to the real function,
in place.

**Delta against what exists.** Today the loop exists three times as prose and once as a
half-implementation. `crates/happenstance-core/src/store.rs:315-320` documents the read
half (*"build a decision model from what you can see, then append conditioned on nothing
new having appeared"*) and ships `read_decision_model`. The example's private `commit`
(`examples/course-subscriptions/src/main.rs:207`) does the append half and its own doc
comment names its destination — *"this is the second half of every DCB command handler,
and it is identical every time — which is exactly why it belongs in the typed layer"* —
and then **bails instead of retrying** (`:217-219`, *"Under real contention this is where
a retry loop would go"*). This PR is the retry loop that comment points at, written once,
in the crate a caller installs.

**Not a new mechanism — a refusal to let four mistakes be writable.** The contract is
`[FROZEN]`; everything this story adds is the composition of frozen parts. Its whole value
is that the four documented ways to get that composition wrong (`_decomposition.md`, *five
ways to get them wrong*, items 1, 2 and 5) stop being reachable from the surface a caller
meets.

## Context pack

Ten decisions. Each is settled — this story implements them, it does not re-open them.

**1. The `after` anchor comes from the read, and only from the read.** `append` returns
the position of the last appended event and its own doc says in terms that it *"is not a
sound `after` for a follow-up condition"* unless the caller has already read up to it,
because positions may have gaps and another writer may hold one *below* that value which
this caller never saw (`crates/happenstance-core/src/store.rs:131-145`). The sound anchor
is `read_decision_model`'s second return value — the last position **actually observed** —
which is exactly what `AppendCondition::after_opt` expects (`:315-330`). A loop that
threads `append`'s return into the next condition silently excludes the events a condition
exists to catch. This is the defect the loop exists to make unwritable (`_design.md`,
*Shape decision*, row *The `after` anchor*).

**2. The retry never branches on `conflicting_position`.** It is
`Option<SequencePosition>` and `None` is **legitimate**: a store reached over one-shot
HTTP has no interactive transaction, so it can only express a conditional
`INSERT … SELECT … WHERE NOT EXISTS`, which yields a boolean and no row
(`crates/happenstance-core/src/error.rs:139-147`). A retry loop that branches on `Some`
passes every test this project can write and fails against `happenstance-neon`. The retry
decision is taken through `AppendError::is_condition_violated()` (`error.rs:253`) and
nothing else; the field may be *carried* into the error for reporting, never *consulted*
for control flow.

**3. Retry means re-read and re-decide from a pristine model.** After a violation the
loop goes back to step one: derive the query again, read again, fold a **fresh** model,
call the caller's `decide` again. It never reuses a stale fold, never keeps the events
from the previous attempt, and never mutates the caller's command input. This is why
`DecisionModel`'s supertrait is `Clone` and not `Default` — the loop re-folds from the
pristine model the caller handed it (`_design.md`, *Shape decision*, row *`DecisionModel`
supertrait*; AC-U13). Distinguish this from the retry-safety property of a **verbatim
resubmission**: re-sending the *same* events under the *same* condition is a different
guarantee, and collapsing the two is a lost update (`RUNBOOK.md:4015-4020`). Both
statements belong in `commit`'s own rustdoc.

**4. The bound is a required argument, not a hidden default.** `Retry` is passed in,
carries a `NonZeroU32` and has no `Default`. *A loop whose only exit is success is a hang
with better manners* (`_decomposition.md`, AC-U13). `attempts(0)` has no honest meaning,
which is what `NonZeroU32` buys; `Retry::once()` is the no-retry spelling. Running out is
its own outcome — `CommandError::Exhausted { attempts, source }` — distinct from a store
failure and from a refusal.

**5. The error vocabulary is the contract's, extended by type parameters, never
duplicated.** `CommandError<E, D>` carries the store's error as `E` and the caller's own
refusal as `D`, both with `#[source]` (RS-30-2). No `String` payloads, no
`happenstance`-local re-spelling of `ConditionViolated` (AC-U05). The exact variant set is
fixed by `_design.md`, *Signatures* — `Boundary`, `Read`, `Append`, `Decode`, `Encode`,
`Refused`, `Exhausted` — and the enum is `#[non_exhaustive]`.

**6. What comes back is a named struct.** `Committed { position, attempts }`,
`#[non_exhaustive]`, public fields, readable but not fabricable (AC-U06; RS-13-4/RS-40-3).
`attempts` is the field a later observability pass grows into and is the observable proof
that the retry ran; a tuple would freeze the arity on the one surface the alpha exists to
keep movable.

**7. The `Send` flavour is bought with bounds at the assertion, not assumed.** Generic
code here binds `EventStore` — the weaker requirement that accepts both flavours
(`CLAUDE.md`, binding constraint 4; RS-20-2). No `#[async_trait]`, ever (ADR-0001,
`.kb/decisions/0001-async-port-flavours.md`). And a specific trap this loop walks into:
`S::Error` carries **no `Send` bound** (ADR-0009 settled it that way), so a
`Result<_, S::Error>` — or an `AppendError<S::Error>` — held across the *next* read's
await makes the whole future `!Send`. `crates/happenstance-core/src/memory.rs:643-700`
records the same discovery and its fix: collapse the error to a decision value **before**
the next suspension point, exactly as that test collapses a read to a `usize`. The retry
loop must inspect `is_condition_violated()` and drop the error (or return it) in the same
statement; it must not hold it while it re-reads.

**8. Nothing before the append is irreversible.** Deriving the query, reading, decoding,
folding, deciding and encoding perform no write. The append is the single irreversible act
and nothing performs one as a side effect of construction (AC-U12). A `decide` that
refuses must leave the store byte-identical — no partial append, no probe write.

**9. `commit` is the JSON convenience; `commit_with` takes any codec.** Two entry points,
because one entry point taking `&C` always would mean every first program names a codec
before it names a domain (`_design.md`, *Shape decision*, row *Codec selection*; DT-2).
`commit` is `#[cfg(feature = "json")]`; `commit_with` is ungated. The `json` feature and
the `Codec` trait itself arrive from the slice-mate `codec-and-feature-forwarding`; this
story consumes them and adds no feature of its own.

**10. The persona slice this realizes.** Beat 3 of *"Choose a contract before a
database"* — **run a command against a store** (`_storymap.md`, activity **A3**). P1's
whole experience of DCB concurrency is this function: they write a closure that returns
events, and contention is handled for them, boundedly, with the bound visible. The
measurement this is judged against is the first program in `_design.md`, *The doctest* —
`commit(&store, seats, Retry::attempts(3.try_into()?), |seats| …)` is the one call it
makes, and its ceremony budget is DT-2's resolved answer, already approved.

**One instrument this story does not have, and must not invent a public substitute for.**
`FaultyStore<S>` — the store that violates on demand — is M4's
(`misbehaving-testkit-stores`) and lands in `happenstance-testkit`, not here
(`_design.md`, *Placement and re-export*, second exception; CF-32). The retry path is
nonetheless tested in this PR, with a `#[cfg(test)]`-only violating wrapper **private to
`crates/happenstance`**. Do not promote it to a public item, and do not add a
`happenstance-testkit` dependency to `happenstance`'s non-dev graph to get at M4's version
early: an application must not carry a test double in its dependency graph.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice from the caller's closure through to a committed append |
| **Slice / milestone** | **M3 `codec-and-command-loop`**; slice-mate: `codec-and-feature-forwarding`. Implemented in one context and mounted as one integrated surface |
| **Mount point** | **`crates/happenstance/src/lib.rs`** — the crate root is the only render path this library has (`_decomposition.md`, *Composition roots*, item 1). `commit`, `commit_with`, `Retry`, `Committed` and `CommandError` are `pub use`d there **beside** the surviving `pub use happenstance_core::*;` (`:75`), and the module doc's `**The command loop**` bullet (`:47-48`) becomes an intra-doc link to `commit` in place, out from under the word *Planned* |
| **Wires into** | `happenstance_core::read_decision_model` (`crates/happenstance-core/src/store.rs:321`) · `EventStore::append` + `AppendCondition::after_opt` (`store.rs:213`) · `AppendError::is_condition_violated` (`crates/happenstance-core/src/error.rs:253`) · `ConditionViolated::conflicting_position` (`error.rs:147`, carried not consulted) · `MemoryEventStore` (`crates/happenstance-core/src/memory.rs`) as the only store this project needs · **`Boundary::query` / `Boundary::absorb`** from slice-predecessor `domain-event-and-decision-model` · **`Codec` / `Json` / `CodecError` and the `json` feature** from slice-mate `codec-and-feature-forwarding` |
| **Renders surfaces** | `crate-root-rustdoc` (regions 2 and 4 — the first program calls `commit`; the vocabulary bullet becomes a link) and `first-program-doctest`, both from `_design.md`, *Surfaces*. The retry *policy* prose is persistent chrome on `commit`'s own item page (`_design.md`, *Transience policy*, last-but-two row). No new surface id is created |
| **Public items** (`_design.md`, *Items*) | `happenstance::commit`, `happenstance::commit_with`, `happenstance::Retry`, `happenstance::Committed`, `happenstance::CommandError` |
| **Conformance rule(s)** | **None, and that is correct.** This is a caller-side composition of frozen port methods; it is not adapter-observable and `happenstance-testkit`'s suite observes adapters, not consumers. The port is unchanged, so there is no rule to add — the instruments that make this story falsifiable are its own tests plus M4's `FaultyStore`/`GappyMemoryStore` |
| **Clause(s)** | Discharges none and amends none. `_design.md` records `ES-10` (the position-visibility invariant the after-anchor rests on) as the clause `commit` *depends* on; the typed layer has no clause namespace in `spec/SPECIFICATION.md` (`_design.md`, header). If using the frozen contract here reveals a defect, it is logged with its clause ID for AC-012 and routed to a decision record — never fixed by a line edit (AC-A02) |
| **Advances DoD scenario** | Initiative **DoD 1** (*@smoke — the worked example runs end to end*): the example's private `commit` is deleted *into* this function by M6, and until this function exists that story cannot start. Also unblocks **DoD 9**'s smallest write-then-read cycle, which is what a stranger installs `happenstance` to write |

## PR boundary

```
crates/happenstance/src/**
crates/happenstance/tests/**
crates/happenstance/Cargo.toml
Cargo.lock
standards/rust/**
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/command-loop/**
```

**`standards/rust/**` was added on 2026-08-16, and it admits citation re-anchoring ONLY.**
The constitution cites this story's own sources by `file:line`, `cargo xtask
lint-constitution` is a gate step, and landing the command loop moves the cited lines — so
the story is forced across its boundary or into a red gate, with no third option. This
entry permits **line-number repair to existing citations and nothing else**: rule text,
evidence selection, rule retirement and new atoms all stay outside, so the widening cannot
later be cited to justify editing a rule. This story's actual use of it is one line of
`standards/rust/90-skeletons-and-todo.md`. Fourth instance of this class in the initiative;
HS-P0010 settled the first three the same way and predicted the recurrence.

**`Cargo.lock` was added to the fence on 2026-08-16, and it is a repair rather than a
widening.** The fence already authorises `crates/happenstance/Cargo.toml`, and cargo
rewrites the workspace lockfile as the mechanical consequence of any dependency change —
so the boundary permitted the cause and forbade the effect, and no implementation could
satisfy both. `redkiln verify --grain story` rejected this story's checkpoint on
`Cargo.lock` alone. Four sibling specs in this project — `codec-and-feature-forwarding`,
`projection-trait-and-runner`, `compile-fail-proof-artefact` and `publish-0-2-0-alpha-1` —
already carry the entry, so the omission was an inconsistency inside one planning pass, not
a policy. The addition admits the lockfile and nothing else: `[workspace.dependencies]` and
every other manifest stay outside.

**In this PR**

- `commit`, `commit_with`, `Retry`, `Committed`, `CommandError` in
  `crates/happenstance/src/`, in modules of the implementer's choosing, `pub use`d at the
  crate root (`_design.md`, *Placement and re-export*).
- The crate root's mount: the `pub use` line beside the glob, and the module-doc bullet at
  `crates/happenstance/src/lib.rs:47-48` rewritten as an intra-doc link to `commit`.
- `commit`'s rustdoc: the retry policy, the verbatim-resubmission distinction, the
  `# Errors` section, and the alternative that lost (RS-70-5).
- Unit and integration tests for every AC below, including the `#[cfg(test)]`-private
  violating store wrapper described in the context pack.
- `crates/happenstance/Cargo.toml` only for what this story needs — a `[dev-dependencies]`
  entry for the async test harness. The `[features]` block is the slice-mate's edit;
  coordinate, do not duplicate.

**Explicitly not in this PR**

- `FaultyStore` / `SendFaultyStore` / `GappyMemoryStore` — M4, in `happenstance-testkit`.
- `happenstance::testing`'s given/when/then DSL — M4.
- Rewriting `examples/course-subscriptions/` or deleting its private `commit` — M6.
- The `Codec` trait, the codec tag's home, and the `json`/`cbor`/`postcard` feature rows —
  the slice-mate's, and ADR-0021's.
- The README's `## Stability`, `CHANGELOG.md`, the fifth `wasm32` gate step, and any
  version number — M7.
- Any edit to `crates/happenstance-core/**` or to `spec/SPECIFICATION.md`.

The implementer **may** also touch the composition-root/wiring files named in the
Integration contract to mount this slice — that is the mount, not scope drift. A story that
does not replace its own roadmap bullet is not done (`_storymap.md`, *Slices*, preamble).

**Merge DoD.** `commit`/`commit_with` are reachable from `crates/happenstance/src/lib.rs`'s
crate root, the module doc links them rather than listing them as *Planned*, every AC below
has a passing real-path test, and `cargo xtask affected --base main` is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| Read half | Derive the query from the boundary (`Boundary::query`), read through `read_decision_model`, fold each nominated event into the model with `Boundary::absorb`. The **second** return value is retained as the `after` anchor | `crates/happenstance-core/src/store.rs:315-330`; `_design.md`, *Signatures* |
| Decide half | Call the caller's `FnMut(&B) -> Result<Vec<B::Event>, D>` against the folded boundary. `Ok(vec)` proceeds to append; `Err(d)` becomes `CommandError::Refused(d)` and **appends nothing** | `_design.md`, *Signatures*; AC-U12 |
| Append half | Encode each event through the codec, build `AppendCondition::new(query).after_opt(anchor)`, call `EventStore::append` | `examples/course-subscriptions/src/main.rs:207-216` (the shape being absorbed) |
| The `after` anchor | **From the read, never from `append`'s return.** `append`'s return is used only to populate `Committed.position` | `crates/happenstance-core/src/store.rs:131-145` |
| Retry trigger | `AppendError::is_condition_violated()` — a `const fn` on the contract's own error — and nothing else. Any other `AppendError` returns immediately as `CommandError::Append` | `crates/happenstance-core/src/error.rs:253` |
| `conflicting_position` | Carried into the returned error for reporting; **never** consulted to decide whether or how to retry. `None` is a conformant answer | `crates/happenstance-core/src/error.rs:139-147` |
| Retry body | Re-derive, re-read, re-fold from a **clone of the pristine boundary**, re-call `decide`. No stale fold, no reused event batch, no mutation of caller state | `_design.md`, *Shape decision*, `DecisionModel` supertrait row; AC-U13 |
| Bound | `Retry::attempts(NonZeroU32)` / `Retry::once()`, a required argument with no `Default`. Exhaustion → `CommandError::Exhausted { attempts, source: ConditionViolated }` | `_design.md`, *Signatures*; *Shape decision*, Retry-bound row |
| Success value | `Committed { position, attempts }`, `#[non_exhaustive]`, `#[must_use]`. `attempts` is 1 on first-try success and N when the Nth attempt committed | `_design.md`, *Signatures*; AC-U06 |
| Error type | `CommandError<E, D>`, `#[non_exhaustive]`, seven variants exactly as designed, every payload a typed `#[source]` — no `String` | `_design.md`, *Signatures*; `standards/rust/30-error-taxonomy.md` RS-30-2 |
| Entry points | `commit` (`#[cfg(feature = "json")]`, JSON) and `commit_with` (ungated, any `C: Codec`). Both `pub use`d at the crate root | `_design.md`, *Items*, *Visibility and stability* |
| Flavour discipline | Generic bound is `EventStore`, one flavour name per module, no `#[async_trait]`. The `Send`-flavour assertion names its bounds (`S: Sync + 'static`, `S::Error: Send`, `D: Send`) and the loop collapses the append error to a control decision **before** the next await | `CLAUDE.md` constraints 1 and 4; `crates/happenstance-core/src/memory.rs:643-700`; `standards/rust/20-two-flavour-ports.md` |
| Rustdoc obligation | The retry policy and the verbatim-resubmission distinction are stated **on `commit` itself**; a doc comment that says *"see the specification"* fails AC-U10. The rejected alternative is named once, where the reader is (RS-70-5) | `RUNBOOK.md:4015-4020`; `_decomposition.md`, AC-U10, AC-U15 |
| No shadowing | No new item may shadow a contract name reachable through the glob re-export; `pub use happenstance_core::*;` stays | `crates/happenstance/src/lib.rs:75`; AC-A01 |

## Data and migrations

**N/A.** This story adds no persistent schema, no stored format and no on-disk artefact.
It composes frozen port calls against whatever store the caller passes, and the only store
in this project's scope is `MemoryEventStore` (`project.md`, *Out of scope*, third bullet).
The one format-shaped question in this slice — where the codec tag is written — belongs to
ADR-0021 and to the slice-mate `codec-and-feature-forwarding`; the command loop's surface
is **invariant under that choice** (`_design.md`, *Shape decision*, closing note), so
nothing here waits on it and nothing here encodes an assumption about it.

## Acceptance criteria

Ten criteria. Each is stated from **P1's** goal — *"model their domain's consistency
boundary once, against a contract, and defer which database to a decision they can revisit
later"* (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:52-54`)
— crossing the full stack from the closure P1 writes to a committed append, and each maps
to one real-path test. Together they discharge project **AC-005**'s command-loop half
(`project.md:177-181`; `_storymap.md`, *Coverage*, AC-005 row).

Test paths below are the ones this PR creates. `crates/happenstance/tests/` does not exist
today — verified — so every integration path is new; module names inside `src/` are the
implementer's (`_design.md`, *Placement and re-export*).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **P1 is protected from the lost update `append`'s return value invites.** GIVEN P1 has written one `commit` call against a boundary another writer is also appending to, WHEN the loop builds the `AppendCondition` for any attempt — the first or the fourth — THEN its `after` is the last position that attempt's own `read_decision_model` actually observed (`crates/happenstance-core/src/store.rs:315-330`), never the position the previous attempt's `append` returned (`:131-145`), so no event sitting below that return is excluded from the condition | `crates/happenstance/tests/command_loop.rs::after_anchor_comes_from_the_read` — a test-local recording wrapper captures every `AppendCondition` the loop submits and asserts each attempt's `after` equals the anchor its own preceding read returned. The wrong implementation is written into the same file as `threads_append_return_is_rejected`, seeded so `append`'s return and the observed anchor differ |
| **AC-002** | **P1 can build, inspect and discard a decision without consequence.** GIVEN a domain rule P1 encoded as an `Err` return from their `decide` closure, WHEN the loop reaches that refusal, THEN it returns `CommandError::Refused(d)` carrying P1's own error type, and the store is byte-identical to before the call — no partial append, no probe write, and nothing before the append performed any write at all (AC-U12, `_decomposition.md:170-178`) | `crates/happenstance/tests/command_loop.rs::refusal_appends_nothing` — reads the whole log through `Query::all()` before and after, asserts equality by the positions the store actually assigned (never literal values), and asserts the recording wrapper saw zero `append` calls |
| **AC-003** | **P1 gets back a value that says what happened, not a tuple that freezes what can be said.** GIVEN an uncontended store, WHEN P1 awaits `commit`, THEN they receive `Committed { position, attempts }` where `position` is what `append` assigned and `attempts` is 1; the type is `#[non_exhaustive]` and `#[must_use]`, so a later observability field is not a breaking change and a dropped outcome does not compile quietly (AC-U06, AC-U09; `_design.md`, *Signatures*) | `crates/happenstance/src/command.rs` `#[cfg(test)] mod tests::first_try_reports_one_attempt` for the value; `crates/happenstance/tests/docs_composition.rs::committed_is_non_exhaustive_and_must_use` asserts both attributes are present on the item source |
| **AC-004** | **P1's retry re-decides against the world as it now is, and their own inputs survive it.** GIVEN a competing writer lands an event between P1's read and P1's append, WHEN the loop retries, THEN it re-derives the query, re-reads, folds a **fresh clone of the pristine boundary** and calls `decide` again with that fresh state — it never reuses the previous fold, never re-submits the previous event batch, and never mutates the boundary value P1 passed in (AC-U13, `_decomposition.md:179-185`; `_design.md`, *Shape decision*, `DecisionModel` supertrait row) | `crates/happenstance/tests/command_loop.rs::retry_refolds_from_pristine_state` — the interloper event is appended by the wrapper between read and append; the closure records the folded state it saw on each attempt and the test asserts attempt 2 saw the interloper and attempt 1 did not. Companion `retry_does_not_resubmit_the_previous_batch` asserts the events reaching `append` on attempt 2 are the ones attempt 2's `decide` returned |
| **AC-005** | **P1's command works against a remote store, not only an in-process one.** GIVEN a store that reports `ConditionViolated` with `conflicting_position: None` — which a one-shot-HTTP adapter legitimately does (`crates/happenstance-core/src/error.rs:139-147`) — WHEN the loop decides whether to retry, THEN it decides through `AppendError::is_condition_violated()` (`error.rs:253`) and nothing else, retries normally, and still surfaces the hint it declined to branch on by carrying the `ConditionViolated` value into whatever error it eventually returns rather than discarding it | `crates/happenstance/tests/command_loop.rs::retries_when_conflicting_position_is_none` plus `branching_on_some_is_rejected` — the second drives the same scenario through a wrapper that only ever reports `None` and asserts a loop that gates its retry on `Some` cannot pass. `exhausted_carries_the_violation` asserts the hint survives into `CommandError::Exhausted`'s `source` |
| **AC-006** | **P1 can see the bound before they hit it, and running out is a distinct answer.** GIVEN P1 wrote `Retry::attempts(2.try_into()?)` at the call site — a required argument with no `Default` and no hidden 3 — WHEN the store violates on every attempt, THEN exactly 2 appends are attempted and the call returns `CommandError::Exhausted { attempts: 2, source }`, distinguishable from a store failure and from a refusal; `Retry::once()` attempts exactly 1 (AC-U13; `_design.md`, *Shape decision*, Retry-bound row) | `crates/happenstance/tests/command_loop.rs::exhaustion_is_bounded_and_named` (counts the wrapper's append calls for `attempts(2)` and for `once()`); `crates/happenstance/src/command.rs` tests assert `Retry` has no `Default` impl and that `NonZeroU32` makes `attempts(0)` unwritable |
| **AC-007** | **P1 matches on the vocabulary they already learned, and every error carries the value.** GIVEN any failure a caller can act on, WHEN P1 inspects it, THEN it is a `#[non_exhaustive]` `CommandError<E, D>` variant whose payload is a typed `#[source]` — the store's own `E`, P1's own `D`, a `CodecError`, an `InvalidQuery` — with no `String` payload and no `happenstance`-local re-spelling of `ConditionViolated`, and the chain is reachable through `core::error::Error::source` (AC-U05, AC-U08; `standards/rust/30-error-taxonomy.md` RS-30-2, RS-30-4) | `crates/happenstance/src/command.rs` tests `store_read_failure_chains_to_source`, `decode_failure_names_its_position`, `encode_failure_names_its_event_type` walk `source()` and assert the concrete inner type; `crates/happenstance/tests/docs_composition.rs::no_string_payloads_in_command_error` asserts no `String` field appears in the enum's source |
| **AC-008** | **P3's edge build survives P1's command loop.** GIVEN a Workers or `wasm32` caller on the `!Send` flavour and a multi-threaded caller on the `Send` flavour, WHEN either compiles `commit`/`commit_with`, THEN both work: this crate's generic code binds `EventStore` (the weaker requirement that accepts both), imports exactly one flavour name per module, uses no `#[async_trait]`, and — because `S::Error` carries no `Send` bound (ADR-0009) — the loop collapses each `AppendError` to a retry decision **before** the next read's await, so the whole future stays `Send` inside a real `tokio::spawn` (`CLAUDE.md` constraints 1 and 4; `crates/happenstance-core/src/memory.rs:643-700`) | `crates/happenstance/tests/flavours.rs::commit_spawns_from_generic` — a generic fn bound `S: SendEventStore + Send + Sync + 'static` that awaits the loop inside `tokio::spawn`, written the way `spawns_from_generic` is, so holding the error across an await fails to compile; `commit_binds_the_weak_flavour` instantiates the same call against a `!Send` store. `cargo xtask affected --base main` runs clippy with `-D warnings` over both |
| **AC-009** | **P4 lands on the crate root and meets the loop as a working thing, not a plan.** GIVEN P4's one bounded, one-shot sitting (`_decomposition.md`, UX brief persona table), WHEN they open `docs.rs/happenstance`, THEN region 2's first program calls `commit` and compiles; region 4's `**The command loop**` bullet is an intra-doc link to that function **in place**, out from under the word *Planned* (`crates/happenstance/src/lib.rs:35,47-48`); `commit`, `commit_with`, `Retry`, `Committed` and `CommandError` are reachable from the crate root beside the surviving `pub use happenstance_core::*;` (`:75`) and none of them shadows a core name; and `commit`'s **own** item page carries the retry policy, the verbatim-resubmission distinction and the rejected alternative as persistent prose — a doc comment that says *see the specification* fails this (AC-U10, AC-U14, AC-U15; `_design.md`, *Composition* regions 2 and 4, *Transience policy* rows for the retry policy and for `Boundary`/`Committed`/`CommandError`; anti-patterns 1, 2, 14) | `crates/happenstance/tests/docs_composition.rs` — `include_str!("../src/lib.rs")` assertions: `roadmap_bullet_became_a_link`, `no_planned_heading_survives`, `no_item_shadows_a_core_name`, `retry_policy_is_on_commit_itself` (asserts `commit`'s doc block contains the policy sentence, the verbatim-resubmission sentence and a rejected-alternative sentence, and contains no *see the specification* deferral). `cargo test -p happenstance --doc` compiles region 2's program; `cargo doc -p happenstance --no-deps` with `rustdoc::broken_intra_doc_links` denied proves the link resolves |
| **AC-010** | **P4 can read the page without scrolling sideways and can tell what a feature turns on.** GIVEN the same one sitting, at 1024x768, WHEN P4 reads the rendered page, THEN the composed presentation holds its budget: doc prose at most 80 columns, code inside a doc fence at most 72 columns, the first sentence of every item this story adds at most 80 characters and a complete claim, every added identifier at most 24 characters, and the crate-root module doc at most 130 lines (`_design.md`, *Density budget*); AND the feature story is legible without opening `Cargo.toml` — `commit` is the JSON convenience gated `#[cfg(feature = "json")]` and renders a `doc_cfg` gate badge on docs.rs, `commit_with` is ungated and reachable with the gate off, and no intra-doc link on the page resolves in only some feature configurations (AC-U14; RS-70-2, RS-70-4, RS-51-5; anti-patterns 3, 5, 6) | `crates/happenstance/tests/docs_composition.rs::density_budget_holds` (per-line column counts and first-sentence character counts over `include_str!`ed sources, plus the identifier-length and module-doc-length assertions); `cargo check -p happenstance --no-default-features` and `--no-default-features --features std` prove `commit_with` stands alone and `commit` disappears cleanly; `cargo doc -p happenstance --no-deps` under default and `--no-default-features` proves no link breaks in either state. The docs.rs badge itself is asserted by `docsrs_metadata_is_present`, reading `Cargo.toml` and `lib.rs` for the `[package.metadata.docs.rs]` block and `#![cfg_attr(docsrs, feature(doc_cfg))]` |

## Interaction quality

RFC §6.7/D6. This story renders two of `_design.md`'s six surfaces —
`crate-root-rustdoc` (regions 2 and 4) and `first-program-doctest` — plus one item page,
`commit`'s. The medium is rustdoc, so *presentation* means composed doc structure that a
reader meets in a rendered page, and an unstyled render here is a doc comment that is
technically accurate and unreadable: correct signatures, no policy, no links, a bullet
still under *Planned*. **Every invariant below is carried by an AC row in the table above**
— this section only says which row carries it and how it is verified.

**STATE invariants.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (AC-U10) — the retry policy and the verbatim-resubmission distinction are readable at the call site's own item page, never by opening `spec/SPECIFICATION.md` | **AC-009** | `docs_composition.rs::retry_policy_is_on_commit_itself` asserts the three sentences are present and that no *see the specification* deferral stands in for them |
| **Non-occlusion — a loop must not hide what it declined to use** (AC-U11's principle, applied to a hint rather than a filter). `conflicting_position` is not consulted for control flow, and is also not dropped: the violation travels into the returned error | **AC-005** | `exhausted_carries_the_violation` |
| **Preserved state across retry** (AC-U13) — the analogue of preserved focus/scroll/selection. The caller's boundary value and command input survive every attempt unmutated; the *fold* is what is discarded and rebuilt, deliberately | **AC-004** | `retry_refolds_from_pristine_state`, `retry_does_not_resubmit_the_previous_batch` |
| **Reversibility** (AC-U12) — everything before the append is pure; a refusal leaves the store byte-identical; the append is the single irreversible act | **AC-002** | `refusal_appends_nothing`, compared against positions the store actually assigned |
| **Bounded exit** — a loop whose only exit is success is a hang with better manners; the bound is a required argument and exhaustion is its own named outcome | **AC-006** | `exhaustion_is_bounded_and_named`; `Retry` has no `Default` |
| **Keyboard reachability**, in this medium: no interactive prompt, no TTY requirement, no ambient global state. The loop performs no I/O but the store calls the caller passed it, and every example runs to completion with stdout piped, which is how the gate runs it (AC-U16's last clause) | **AC-010** | `cargo test -p happenstance --doc` runs the doctests non-interactively under the gate |

**COMPOSITION invariants**, taken from the signed-off `_design.md` (approved 2026-08-12,
*Sign-off*). These are the ones an unstyled render passes and a reader does not.

| Invariant | Design source | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** — every item this story adds carries real composed rustdoc: a complete-claim first sentence, an `# Errors` section, and for `commit` the policy prose. Bare signatures with `/// The command loop.` fail | *Transience policy*, retry-policy row; RS-70-1/70-5 | **AC-009**, **AC-010** | `retry_policy_is_on_commit_itself`; `density_budget_holds`'s first-sentence assertions, which fail on an absent or fragmentary summary |
| **Composition and placement** — the vocabulary bullet is rewritten **in place** in region 4, keeping its order and its discriminator prose; the first program stays region 2, alone above the fold; the adapter pointer stays last | *Composition*, `crate-root-rustdoc` table rows 2, 4, 7 | **AC-009** | `roadmap_bullet_became_a_link` asserts the bullet is at its original position in the region-4 list and that regions 2 and 7 keep their ordinal positions |
| **Transience** — persistent chrome: `commit`, `Retry` and the first program on the landing page, and the retry policy on `commit`'s item page. Revealed: `Committed`, `CommandError`, `Boundary` reached in one click from `commit`'s signature, **not** hoisted onto the landing page. Opened on demand: ADR and specification links, off-site | *Transience policy*, rows 1, 2, 4 and the ADR-links row | **AC-009** | `docs_composition.rs::landing_page_names_only_the_persistent_four` asserts `Committed`/`CommandError` do not appear as module-doc bullets; the rustdoc build proves they are reachable from `commit`'s signature |
| **Density budget, with its real numbers** — 80 prose columns; 72 columns inside a doc fence; 80-character first sentences; 24-character identifiers; ≤130 module-doc lines; the first program ≤35 visible lines with at most 2 hidden | *Density budget*, both tables | **AC-010** | `density_budget_holds`, counting columns and characters over the real source |
| **Hierarchy** — the intra-doc link *is* the emphasis: nothing in region 4 is bolded that is not also reachable, and the four persistent names carry primacy by position, not by decoration | *Hierarchy*, `crate-root-rustdoc` paragraph | **AC-009** | `roadmap_bullet_became_a_link` asserts the bolded lead-in term is itself the link target |
| **Named anti-patterns 1, 2, 3, 5, 6, 14** — no bulleted list under *Planned*; nothing but one sentence above the first fence; no horizontal scroll at 1024px; no item summary ending in an ellipsis; no gate badge missing on a gated item; no `happenstance` item shadowing a `happenstance_core` re-export | *Anti-patterns* | **AC-009** (1, 2, 14), **AC-010** (3, 5, 6) | `no_planned_heading_survives`, `no_item_shadows_a_core_name`, `density_budget_holds` (3 and 5 reduce to the 72-column and 80-character budgets), `docsrs_metadata_is_present` (6) |

**Anti-patterns 4, 7–13, 15 do not apply to this story** and are named so the silence is
not read as an oversight: 4 belongs to the first program's elision policy (M2 authors the
program; this story only makes its `commit` call real and re-checks the budget), 7–8 to the
README (M7), 9–11 to the worked-example transcript (M6), 12 to the DSL failure message
(M4), 13 to the `trybuild` fixture (M6), and 15 to the projection runner (M5).

## Error conditions

Every row is a `CommandError` variant from `_design.md`, *Signatures* — the enum is
`#[non_exhaustive]` and this is its complete authored set.

| id | condition | behaviour | evidence |
| --- | --- | --- | --- |
| **EC-001** | The boundary constrains neither event types nor tags, so the derived query is `Query::all()` | `CommandError::Boundary(InvalidQuery::UnconstrainedItem)`, before any I/O. Absorbed by `#[from]` so the caller's first program writes no extra `?` for it | `crates/happenstance-core/src/error.rs:99-102`; `_design.md`, *The residual*, item 3 |
| **EC-002** | The store fails during the read | `CommandError::Read(E)` — the adapter's own error, unwrapped and unstringified. No retry: a read failure is not the DCB concurrency signal | `_design.md`, *Signatures*; RS-30-2 |
| **EC-003** | The store fails during the append for its own reasons (`AppendError::Store`) | `CommandError::Append(AppendError<E>)`, returned immediately. Only `is_condition_violated()` routes to a retry | `crates/happenstance-core/src/error.rs:245-256` |
| **EC-004** | A nominated event in the read cannot be decoded into `B::Event` | `CommandError::Decode { position, source }` — the position names *which* event, because a fold/`EVENT_TYPES` disagreement is otherwise invisible | `_design.md`, *The states the API must express*, **Absent** |
| **EC-005** | An event the decision produced cannot be encoded | `CommandError::Encode { event_type, source }`, and **nothing is appended** — the encode failure happens before the single irreversible act | AC-U12; **AC-002** covers the store-unchanged half |
| **EC-006** | The caller's `decide` returns `Err(d)` | `CommandError::Refused(d)`, the caller's own type with `#[source]`. Not an error of this library; the loop's job is to carry it faithfully | **AC-002** |
| **EC-007** | Every attempt was violated | `CommandError::Exhausted { attempts, source: ConditionViolated }` — `attempts` is the bound that was reached, `source` the last violation, hint included | **AC-005**, **AC-006** |
| **EC-008** | The `json` feature is off and the caller wrote `commit` | A compile error naming the feature, and `commit_with` still present. A feature only ever *adds* (RS-51-1) | **AC-010** |

Two conditions are deliberately **not** errors of this loop: an empty read (`(vec![], None)`
is the well-formed *nothing existed* anchor and `after_opt(None)` is its condition — not an
error, `_design.md`, *The states the API must express*, **Empty**), and a gap in the
positions the store assigned (**Gapped** — nothing here computes `head - checkpoint` and no
test asserts a literal position, per `CLAUDE.md`, *The rule that matters*).

## Non-functional

| id | requirement | why, and how it is met |
| --- | --- | --- |
| **NF-001** | No `unwrap`, `expect`, `panic!` or indexing that can panic on any path in `crates/happenstance/src/` | `standards/rust/00-prime-directives.md`; the workspace `[lints]` table is inherited by `crates/happenstance/Cargo.toml` and `cargo xtask affected` runs clippy with `-D warnings`. The one place a panic was tempting — the fallible `QueryItem::new` on pre-validated inputs — is absorbed into `CommandError::Boundary` instead (`_design.md`, *The residual*) |
| **NF-002** | Buffering is confined to the decision path | AC-A05 permits the decision path to buffer and requires the projection runner to stream. `read_decision_model` already collects (`crates/happenstance-core/src/store.rs:326`); this loop adds no second buffer and holds no events from a previous attempt across a retry (**AC-004**) |
| **NF-003** | The loop adds no `Send` bound to the port and no dependency that would forfeit `wasm32` | `commit_with` is generic over `EventStore`, the weaker flavour; the only new manifest entry is a `[dev-dependencies]` async harness, which is not in a consumer's graph. Whether a fifth `wasm32` gate step compiling `happenstance` is added is **M7's** (`edge-flavour-and-wasm-claim`, AC-A06) — this story must not pre-empt it, and must not break it (**AC-008**) |
| **NF-004** | The MSRV floor of 1.97.1 is not moved | ADR-0029 (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). Let-chains are available and may be used; no new non-dev dependency is introduced, so no build script can raise the floor underneath us (`CLAUDE.md`, binding constraint 5) |
| **NF-005** | No test double reaches a consumer's dependency graph | The violating/recording wrapper is local to this crate's test targets. `happenstance-testkit` is **not** added to `happenstance`'s non-dev dependencies, early or otherwise (`_design.md`, *Placement and re-export*, second exception; CF-32) |
| **NF-006** | The public surface stays movable | `Committed`, `CommandError` and `Retry` are `#[non_exhaustive]`; nothing added is `#[doc(hidden)]`; no item is `pub` because nobody decided (`_design.md`, *Visibility and stability*, and its `pub`-because-nobody-decided check) |

## Implementation notes (non-prescriptive)

Shape suggestions only — the signatures in `_design.md` are binding, the arrangement below
is not.

- **One private loop, two public doors.** `commit` can be a thin
  `#[cfg(feature = "json")]` wrapper delegating to `commit_with(store, boundary, &Json,
  retry, decide)`, so there is exactly one implementation of the policy to get right and
  the JSON convenience costs a line. Both are `pub use`d at the root regardless of which
  module holds them.
- **The `Send` trap is a statement-shape problem, not a bound problem.** Write the retry
  decision as a single expression that consumes the `AppendError` — match it, or
  `if err.is_condition_violated() { … } else { return Err(CommandError::Append(err)) }`
  with nothing awaited between the inspection and the drop. Binding
  `let err = …; something.await; if err.…` is the version that makes the future `!Send`,
  and it will compile fine until `flavours.rs` tries to spawn it
  (`crates/happenstance-core/src/memory.rs:643-700` is the transcript of the same lesson).
- **Count attempts, do not compute them.** `Committed.attempts` and
  `Exhausted.attempts` should come from the same counter, incremented once per append
  submitted, so the two can never disagree about what *attempt* means.
- **Clone the boundary once per attempt, at the top.** `DecisionModel: Clone` exists for
  this. Cloning inside the fold, or re-using the previous attempt's value with a "reset",
  is how a stale field survives a retry — which is exactly what **AC-004** falsifies.
- **The test wrapper wants three knobs and no more:** violate the next *n* appends
  (reporting `conflicting_position: None`), record every `AppendCondition` submitted, and
  append an interloper event between read and append. Keep it in the test target; M4 ships
  the public instrument (`FaultyStore<S>`) and this wrapper should be deleted when
  `given-when-then-dsl` can drive the same scenarios through it.
- **The composition test is a file-reading lint, not a rustdoc parser.** `include_str!` the
  crate root and the loop's module, then assert over lines. The repository already treats
  file-reading lints as first-class (`cargo xtask lints`), but adding a sixth lint to
  `xtask` is outside this PR's boundary — keep it as a test in `crates/happenstance/tests/`.
- **Order of work that keeps the tree green:** the error enum and `Retry`/`Committed`
  first (they compile against nothing), then `commit_with` against a hand-rolled boundary
  in tests, then the crate-root mount, then the doctest — because the doctest is the only
  artefact that needs both slice halves present.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`, *Testing brief* (the AC-005 row and the four grains
`.redkiln/config.yaml`'s `verify:` block wires). This project's integration bar is
`cargo xtask ci --fast`, **not** the full gate — `project.md`'s frontmatter carries
`terminal: false`.

| tier | command / path | proves |
| --- | --- | --- |
| Unit | `cargo test -p happenstance --lib` → `crates/happenstance/src/command.rs` `#[cfg(test)] mod tests` | AC-003 (the outcome value), AC-006 (`Retry` has no `Default`; `once()` is one attempt), AC-007 (every `CommandError` variant chains to a typed `source`) |
| Integration | `cargo test -p happenstance --test command_loop` → `crates/happenstance/tests/command_loop.rs` | AC-001, AC-002, AC-004, AC-005, AC-006 — the whole read → decide → append → retry cycle against `MemoryEventStore` wrapped by the test-local violating/recording store. *"`MemoryEventStore` is all this project needs"* (`RUNBOOK.md:3977-3979`) |
| Integration (flavour) | `cargo test -p happenstance --test flavours` → `crates/happenstance/tests/flavours.rs` | AC-008 — the loop awaited inside a real `tokio::spawn` on the `Send` flavour, and instantiated against a `!Send` store, following `spawns_from_generic`'s bound pattern |
| Composition lint | `cargo test -p happenstance --test docs_composition` → `crates/happenstance/tests/docs_composition.rs` | AC-009 and AC-010 — the placement, transience, density and anti-pattern assertions, read off the real source. This is the tier that fails on a technically-correct, unreadable page |
| Doctest | `cargo test -p happenstance --doc` | AC-009's *region 2 compiles and calls `commit`*, AC-010's non-interactive/piped-stdout claim, and `commit`'s own `# Errors` example. Collected by `cargo test --workspace --all-features` inside the gate — not a separate merge-gate command (`_decomposition.md`, *Testing brief → Intent*) |
| Static (features) | `cargo check -p happenstance --no-default-features` and `cargo check -p happenstance --no-default-features --features std` | AC-010 — `commit_with` stands without `json`, `commit` disappears cleanly, and a feature only ever *adds* (RS-51-1) |
| Static (docs) | `cargo doc -p happenstance --no-deps` under default features **and** `--no-default-features`, with `rustdoc::broken_intra_doc_links` denied | AC-009's link resolution and AC-010's *no link resolves in only some feature configurations* (RS-70-2). This is the check the design's mock found failing first (`_design.md`, *Mock*, finding 2) |
| Story grain (merge gate) | `cargo xtask affected --base main` | fmt, `clippy -D warnings` and the tests for every package this diff touches plus their dependents — wired by `.redkiln/config.yaml:28-40`, so it runs whether or not anyone types it. NF-001 rides on the clippy half |
| Integration grain (merge gate) | `cargo xtask ci --fast` | this project's bar: `REQUIRED` without `OPTIONAL`, all four existing `wasm32` steps kept in (`xtask/src/main.rs:835-857`) |
| **Not this story's gate** | `cargo xtask ci` (full) — feature powerset, `cargo deny`, nightly `--cfg docsrs` | The powerset is what actually proves AC-005's feature combinations compose independently, and it is **M7's** pre-publish gate, not this story's (`_decomposition.md`, *The feature powerset is not this project's gate*). Named here so its absence is not mistaken for coverage |

**Merge DoD, restated as a command sequence.** `cargo test -p happenstance --all-targets`,
then the two `cargo check` feature states, then `cargo doc -p happenstance --no-deps` twice,
then `cargo xtask affected --base main`. Every AC row above has a passing real-path test and
its ledger row cites it.

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation inside this PR |
| --- | --- | --- |
| **The slice-mate owns `[features]`; this story needs `json` to exist** | `commit` is `#[cfg(feature = "json")]` and `commit_with` takes `C: Codec`. Both names arrive from `codec-and-feature-forwarding`, which is implemented in the *same* context and merged as one surface | Implement `commit_with` against the `Codec` trait first; touch `crates/happenstance/Cargo.toml` only for `[dev-dependencies]`. If both halves edit `[features]`, that is a merge conflict inside one context — coordinate, do not duplicate (PR boundary, *In this PR*, last bullet) |
| **`Boundary` is M2's and may still be moving** | `commit_with`'s whole signature rests on `Boundary::query` / `absorb`, and M2 lands them one milestone earlier | Bind exactly the two methods in `_design.md`, *Signatures*, and nothing else. If `Boundary` arrived with a different shape, that is a **blocking** discrepancy against a signed-off design: escalate rather than adapt silently |
| **The retry path has no public instrument until M4** | `FaultyStore<S>` is `misbehaving-testkit-stores`', in another crate | The test-local wrapper (NF-005). Do **not** promote it public and do **not** add `happenstance-testkit` to the non-dev graph to borrow M4's version early |
| **The `Send` trap compiles until it does not** | `S::Error` has no `Send` bound (ADR-0009), so a naive `let err = …` across an await silently makes the whole future `!Send` — and every single-threaded test still passes | `flavours.rs` is written **first**, before the retry body, so the constraint is a red test rather than a discovery during M7's `wasm32` work (AC-008) |
| **`[package.metadata.docs.rs]` does not exist in this crate's manifest** | Verified absent; without it every gated item renders with no badge, which is anti-pattern 6 and AC-010's failure | The block and `#![cfg_attr(docsrs, feature(doc_cfg))]` are the slice's to add, once. If the slice-mate has not added them by the time this story mounts, this story adds them — see *Clarifications* |
| **Region 2's first program is M2's artefact and this story changes what it calls** | The `_design.md` doctest calls `commit`, which does not exist until this story lands, so M2 cannot have shipped it in final form | Treat the program as jointly owned: this story completes it to the designed form and re-runs the density assertions (AC-010) rather than leaving a program that compiles but does not demonstrate the loop |
| **Scope creep into `happenstance-core`** | The residual (`QueryItem::new` is fallible for pre-validated inputs) makes a one-line edit to the frozen crate look attractive | AC-A02 forbids it. The residual is already routed: **defect candidate D-1** in `_design.md`, *The residual*, collected by `defect-log-and-macros-verdict` (AC-012). Log, do not edit |

## Dependencies

**Blocks on** (must be merged before this story starts — both are in the same or an earlier
milestone, so neither is a cross-project wait):

- `domain-event-and-decision-model` (M2) — supplies `DomainEvent`, `DecisionModel` and
  `Boundary::query` / `Boundary::absorb`, which are `commit_with`'s entire input vocabulary.
- `codec-and-feature-forwarding` (M3, slice-mate) — supplies `Codec`, `Json`, `CodecError`
  and the `json` feature row. Implemented in the same context; mounted as one surface.

**Unlocks** (their `depends_on` names this story — `_storymap.md`, *Slices*):

- `given-when-then-dsl` (M4) — its `when` clause runs the loop, and its whole demonstration
  is *a caller's retry loop driven against `FaultyStore<S>` with no database*.
- `worked-example-on-typed-layer` (M6) — deletes `examples/course-subscriptions/src/main.rs:207`'s
  private `commit` *into* this function; it cannot start until this exists. This is the
  edge that advances initiative **DoD 1**.
- `edge-flavour-and-wasm-claim` (M7) — settles AC-A06 over generic code that includes this
  loop's bounds.

**Not a dependency, deliberately:** `misbehaving-testkit-stores` (M4). The story map gives
it no dependency on this one and this one none on it; the retry path is tested here through
a test-local wrapper so the two milestones do not serialise.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Every path was confirmed present in the
worktree before it was cited. Open each at the moment named, not before.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-core/src/store.rs` | `:125-145` is `append`'s own doc refusing its return value as an `after`; `:310-331` is `read_decision_model` and the sound anchor. The two paragraphs are the whole of AC-001, in the contract's own words | Before writing the first `AppendCondition`, and again before writing the retry body | AC-001 |
| `crates/happenstance-core/src/error.rs` | `:130-150` documents why `conflicting_position: None` is conformant and names the wrong loop by shape; `:245-256` is `is_condition_violated`, the only admissible retry predicate | Before writing the retry predicate — this is the file that stops you branching on `Some` | AC-005 |
| `crates/happenstance-core/src/memory.rs` | `:640-700` is `spawns_from_generic` with per-bound reasoning: which bounds are load-bearing, why `S::Error` must not be held across an await, and the `map_or` trick that collapses the error first. It is a worked transcript of the exact trap this loop walks into | Before writing `tests/flavours.rs`, i.e. before the retry body | AC-008 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | The binding surface: *Signatures* (the exact `CommandError` variant set, `Retry`, `Committed`, both entry points), *Shape decision* (the Retry-bound, after-anchor and codec-selection rows), *Transience policy*, *Density budget*, *Anti-patterns*, and *The doctest* | *Signatures* before the first line of code; *Transience policy* + *Density budget* + *Anti-patterns* before touching the crate root; *The doctest* before finishing region 2 | AC-003, AC-006, AC-007, AC-009, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | `:488-524` is *five ways to get them wrong*, items 1, 2 and 5 — the defects this story exists to make unwritable; `:150-199` is the AC-U10/U12/U13/U14/U15 invariant text this spec's interaction-quality rows compress; `:777-795` is the testing brief's tier assignment | *Five ways* at the start, as orientation; the AC-U block before writing rustdoc; the testing brief before naming a test file | AC-001, AC-002, AC-004, AC-009 |
| `examples/course-subscriptions/src/main.rs` | `:207-219` is the half-implementation being absorbed, including the comment that names its own destination and the `// Under real contention this is where a retry loop would go` bail. It shows the append half already working against a real store, and the exact shape M6 will delete | When writing the append half, and again when writing `commit`'s rustdoc — its doc comment is the audience test | AC-002, AC-004 |
| `RUNBOOK.md` | `:4015-4020` is the roadmap obligation in the runbook's own words: state the retry policy **in the docs, not only in the code**, and distinguish it from the retry-safety property of a verbatim resubmission — *collapsing them is a lost update* | When writing `commit`'s doc comment, not before | AC-009 |
| `standards/rust/30-error-taxonomy.md` | RS-30-1/30-2/30-4/30-5 with a compiled example and a named wrong implementation each: carry the adapter's error as a type parameter with `#[source]`, never a `String`; render the value, not the category | Before declaring `CommandError` | AC-007 |
| `standards/rust/20-two-flavour-ports.md` | RS-20-2 (bind the weak flavour), RS-20-3 (one flavour name per module), RS-20-4 (why one type cannot carry both) — the rules the coherence error will otherwise teach you at compile time | With `memory.rs:640-700`, before `tests/flavours.rs` | AC-008 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 (a link must resolve in every feature configuration), RS-70-4 (`doc_cfg` badges), RS-70-5 (name the alternative that lost, once, where the reader is) | Before the crate-root mount and before `commit`'s doc comment | AC-009, AC-010 |
| `standards/rust/51-features-and-no-std.md` | RS-51-1 (*a feature adds*) and RS-51-5 (declare the docs.rs configuration in the manifest) — the rules behind EC-008 and the badge assertion | When adding `#[cfg(feature = "json")]` to `commit`, and when checking the manifest metadata | AC-010 |
| `standards/rust/13-sealing-and-exhaustiveness.md` | RS-13-3/13-4: readable-but-not-fabricable structs and why `#[non_exhaustive]` belongs on a value that will grow a field | When declaring `Committed` and `CommandError` | AC-003, AC-007 |
| `.kb/decisions/0001-async-port-flavours.md` | The accepted atom behind *never `#[async_trait]`* and the two-flavour derivation. Binding constraint 1 in `CLAUDE.md` is its summary; this is the record | Once, before writing any `async fn` in this crate | AC-008 |
| `.kb/decisions/0009-error-send-sync.md` | The accepted atom that settled `Error` **without** a `Send` bound on either port — the decision that creates the trap in AC-008, and the reason the fix belongs in the caller's statement shape rather than in a new port bound | Alongside `memory.rs:640-700` when the `!Send` future error appears | AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | `:42-113` is P1 — the goal, the fear (*being the one who discovers a contract defect in production*), and the journey beat this story realises; `:249-316` is P4's one bounded sitting, which is what AC-009 and AC-010 are written for | Before writing the acceptance-facing rustdoc, when a criterion's *why* stops being obvious | AC-009, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md` | Activity **A3** (*run a command against a store*), this story's row and its `depends_on`/`traces_to` edges, and the merge order that puts M3 after M2 | At the start, to confirm the slice boundary; and before claiming done, to confirm no unlocked story was pre-empted | AC-001, AC-009 |
| `crates/happenstance/src/lib.rs` | The mount point itself: `:35` (*Planned, and specified in…*), `:47-48` (the bullet this story rewrites in place) and `:75` (the glob re-export that must survive) | When mounting, and it is the last file this PR touches | AC-009 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated.** AC-001 … AC-010, no
   additions and no drops; the ledger carries the same ten ids and no others.
2. **Composition invariants became AC rows, not prose bullets.** `redkiln verify` extracts
   ACs from a leading `| AC-00n |` table cell, so an invariant written only as a bullet in
   *Interaction quality* would never be gated. AC-009 carries placement, transience and
   hierarchy; AC-010 carries the density budget and the feature-state/badge invariants. The
   *Interaction quality* section names which row carries what and nothing else.
3. **`crates/happenstance/tests/` does not exist yet** — verified. Every integration path in
   this spec is created by this PR, which is why the PR boundary already includes
   `crates/happenstance/tests/**`.
4. **Who adds `[package.metadata.docs.rs]`.** `_design.md`, *Placement and re-export*,
   requires it plus `#![cfg_attr(docsrs, feature(doc_cfg))]`, and confirms both are absent
   from `crates/happenstance` today (the block exists in `happenstance-core` and
   `happenstance-testkit`). It is not assigned to a story. **Resolved:** it belongs to the
   M3 slice and is added **once**, by whichever half of the slice mounts a gated item first.
   This story's AC-010 asserts it is present when the story completes; if the slice-mate
   added it, that assertion simply passes. It is not a licence to edit `[features]`.
5. **Region 2's first program is jointly owned with M2.** `_design.md`, *The doctest*, shows
   the program calling `commit`, which cannot compile until this story lands. Resolved as a
   handover rather than a conflict: M2 ships the program in whatever form compiles at M2,
   and this story completes it to the designed form and re-checks the density budget
   (AC-009, AC-010). Nothing else in region 2 moves.
6. **Anti-pattern 5 is scoped, not waived.** `_design.md`, *Mock*, finding 3 records that
   three crate-root item summaries and several identifiers exceed the budget and that
   **every one of them arrives through `pub use happenstance_core::*`**, frozen by AC-A02.
   AC-010's assertions are therefore scoped to *the items this story adds*. Widening them to
   the whole page would fail on names this project may not change.
7. **No conformance rule is added, and that is not an omission.** The Integration contract
   already states it: `happenstance-testkit`'s suite observes adapters, and this is a
   caller-side composition of unchanged port methods. The instruments that make the story
   falsifiable are its own tests plus M4's `FaultyStore`/`GappyMemoryStore`.
8. **The `spec/SPECIFICATION.md` clause namespace does not cover this layer** (`VT-`, `ES-`,
   `PS-`, `SY-`, `WF-`, `CF-` only), so no clause is discharged or amended here. `ES-10` is
   named as the clause the after-anchor *depends* on, not one this story touches — it is
   `[FROZEN]` as of phase 4 (`RUNBOOK.md:256`).
9. **Defect candidate D-1 is inherited, not re-decided.** `_design.md` already logged
   *`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs*.
   If implementing this loop reveals a second defect, log it the same way for AC-012's
   record and route it to a decision record — never a line edit to the frozen crate
   (AC-A02), and never an ADR written as a side effect of this story.
