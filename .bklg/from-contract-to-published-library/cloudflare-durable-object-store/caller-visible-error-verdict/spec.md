---
item: HS-S0052
stage: spec
created: 2026-08-12T13:46:50.413Z
updated: 2026-08-12T13:46:50.413Z
template_sig: 87bbf1d0
rendered_sig: "21816702"
---

# Spec — The ES-6 artefact: constraint violation versus transport fault, recovered by a caller

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/caller-visible-error-verdict/spec.md` |
| Story discovery (answers five questions this spec does not re-ask) | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/caller-visible-error-verdict/discover.md` |
| Key briefs | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` — Architecture §"Error data flow, and the one classification that must happen early"; Architecture Notes §6 (thrown value kept live vs stringified) and §7 (standing detectors); Testing §"Targeted runtime tests, outside the conformance macro", Notes §2 (which target hosts the unit tier) and Notes §3 (the mocked thrown value) |
| Signed-off design | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` — **N/A, and that is the sign-off**: this project renders no surface, approved 2026-08-12. It therefore constrains nothing here except that no screen is owed; it also does not license a broad new public API, see the Context pack's D7 |
| Story map row | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md`, `real-worker-bindings` milestone, `caller-visible-error-verdict` |
| Roadmap pointer | `RUNBOOK.md:4276-4292` (phase 9's proof artefact, second half) and `RUNBOOK.md:4294-4299` (exit criterion "ES-6 is decided") |
| Traces to | project `AC-005` (this story owns the committed reconstruction test half) |
| Depends on | `durable-object-write-path` (HS-S0050) — it supplies the classification site |
| Blocks | `adr-0023-and-atom-resolutions` (HS-S0058) — it records what this artefact found |

## One-line PR slice

Commit the ES-6 artefact: a test that reconstructs, from a *caller-visible*
`CloudflareEventStoreError` carrying a real `worker::Error`, the one fact a caller
branches on — constraint violation versus transport fault — and fails if the error
type stops carrying it.

## Executive summary

`durable-object-write-path` lands the classifier: a thrown `worker::Error` becomes
either `AppendError::ConditionViolated`, `AppendError::ExceedsStoreLimit` or
`AppendError::Store(CloudflareEventStoreError::…)` before `Self::Error` exists
(`_decomposition.md`, Architecture, *Error data flow*). This PR lands the half that
nothing else in the tree can: proof that the distinction survives all the way out to
a caller, read through the public surface a caller actually holds.

The delta is small and the reason it exists is structural. Every event-store
conformance rule asserts on the success path or on a store-produced `AppendError`,
and none of them reads an adapter error's *contents* (`project.md`, DR-4) — so a
fully green `workerd` run is silent on whether the error a caller receives says
anything. `RUNBOOK.md:4276-4292` refuses the green suite as the artefact for exactly
that reason and demands this second half, or "ES-6 is decided" is discharged by
assertion. What ships: one negative-control-backed test in this crate's own tree
beside `not_send_probe`, whatever minimal public accessor the swap to real `worker`
bindings turns out to require for the fact to be reachable without a private field,
and the crate documentation's ES-6 findings section updated from *prediction* to
*observation*. What does not ship: any change to the bound, any `.kb/` atom, any new
`AppendError` arm.

## Context pack

The load-bearing decisions, stated as decisions. Everything deeper is a signposted
anchor.

**D1 — ES-6 is settled, and this story does not reopen it.** ADR-0009 is
**accepted** and immutable: `Error` keeps exactly `core::error::Error + 'static` on
both ports and both flavours, and the stronger property lives downstream in a
blanket-implemented marker, `ThreadSafeEventStore: SendEventStore<Error: Send + Sync>`
(`.kb/decisions/0009-error-send-sync.md`, *Decision*). `spec/SPECIFICATION.md:2629`
is `[FROZEN]` on the same terms. Project AC-005's wording — "bound added, or
ADR-0009's deferral confirmed" — predates the atom's acceptance and is stale; the
flag is already recorded twice (`_grounding.md:38-43`; `_decomposition.md`,
Architecture, *Tension (flagged)* at `:132-137`) and this story obeys the atom, not
the wording. **Adding `+ Send + Sync` to `Error`, or editing
`.kb/decisions/0009-error-send-sync.md`, is out of bounds** — `redkiln validate --kb`
checks accepted atoms against `HEAD` and would catch the second.

**D2 — the fact under test is not "is it a conflict".** The DCB conflict signal
never travels in `Self::Error` on *any* adapter: `happenstance-core` lifts it into
`AppendError::ConditionViolated` before the adapter's error is constructed, which is
why `CloudflareEventStoreError` deliberately carries **no** `ConditionViolated`
variant (`crates/happenstance-cloudflare/src/lib.rs:75-85`;
`crates/happenstance-cloudflare/src/event_store.rs:100-104`). So the reconstruction
is two-sided and both sides are the test: a constraint violation must arrive on the
`ConditionViolated` *channel*, and a transport fault must arrive on the `Store`
channel **still carrying enough for a caller to tell it from a conflict, a capacity
refusal, or a binding that was never wired up**. Adding a `ConditionViolated`
variant to the adapter's error would look like a fix and be the defect: that absence
is a finding made structural.

**D3 — the named wrong implementation this PR must reject.** Not the blunt one (an
error whose `Display` renders `"a SQL error occurred"`) but the subtle one a careful
implementer reaches honestly: a classifier that distinguishes correctly *inside*
`append`, uses the answer to pick the `AppendError` arm, and then **discards the
evidence**, so the `Store` arm a caller receives is indistinguishable from a network
fault or a storage cap (`discover.md`, *The wrong implementation*). Every existing
check passes against it, and there is a structural reason none can fail it (D2 +
DR-4). The artefact is therefore owed a positive control in the same shape as
`the_probe_is_not_vacuous` (`crates/happenstance-cloudflare/src/lib.rs:210-220`):
a named wrong error shape in this crate's own test tree that the same assertion
rejects. Without it, this is a test that cannot fail — the decorative-rule failure
mode `CLAUDE.md` names.

**D4 — read through the public surface, never a private field.** The test's whole
value is the word *caller-visible* (`_storymap.md`, `real-worker-bindings` row). A
test that reaches into a private field, or into a `pub(crate)` helper, keeps passing
after the information stops being recoverable — which is precisely the regression it
exists to catch. Reach the store through the one constructor,
`CloudflareEventStore::new(sql)` (`event_store.rs:70-87`), call `append` through the
`EventStore` port (`event_store.rs:171-177`), and read only what a downstream crate
could read: the `AppendError` arm, the public error variants, `Display`, and the
`core::error::Error::source` chain.

**D5 — the thrown value is constructed, not round-tripped.** Mock a thrown
`worker::Error` carrying the exact message text a Durable Object's SQLite surfaces —
`UNIQUE constraint failed: event.position`, already named at
`crates/happenstance-cloudflare/src/lib.rs:63-73` — rather than driving a live
Durable Object (`_decomposition.md`, Testing Notes §3). That text is the whole
mechanism: Workers surfaces SQLite's own message through the thrown `Error`'s
`message` and exposes **no numeric code**, so classification is a text test on both
the live-handle and the stringified shapes (`crates/happenstance-cloudflare/src/js.rs:113-127`,
`:152-162`). Constructing it keeps this artefact cheap and — deliberately —
independent of whether `every-rule-under-workerd`'s runner exists yet. DoD 4 is the
one exit criterion the runbook says has no artefact behind it; it must not be the
one waiting on the project's riskiest infrastructure.

**D6 — keep the thrown value live unless you argue otherwise.** Stringifying at the
boundary (`StringifiedThrow`) loses *forward* compatibility — a caller holding the
live value can read a field nobody has thought of yet — and, decisively, makes the
whole error `Send + Sync`, destroying the workspace's only ES-6 instrument
(`crates/happenstance-cloudflare/src/sql_storage.rs:83-92`; `_decomposition.md`,
Architecture Notes §6). The default is live. This story does not own the `!Send`
half of AC-005 (`worker-binding-layer` does, per `_storymap.md`'s split), but it is
the story most able to break it by convenience, so the four probe tests at
`crates/happenstance-cloudflare/src/lib.rs:180-259` are a standing detector here:
they must still pass, `the_probe_is_not_vacuous` included.

**D7 — the design records no surface, which is permission for nothing, not for
anything.** `_design.md` is signed off as N/A — no screen, no `## Signatures` block
to match. It therefore states no signature this story must render, and equally
licenses no new public API. If the swap to real `worker` bindings leaves the fact
unreachable from outside the crate, the minimal accessor that restores it is
in-scope and must be argued in this spec's own behavior table, documented with an
`# Errors`/intent line per `standards/rust/70-rustdoc-obligations.md`, and kept
`#[non_exhaustive]`-compatible. Anything broader is a semver promise nobody made.

**D8 — the verdict is recorded here, and minted elsewhere.** This story writes the
observation into the crate's own documentation (the ES-6 findings section at
`crates/happenstance-cloudflare/src/lib.rs:12-21` and `:63-95`, which today reads as
prediction against a stand-in). The **atom** — confirming or refuting ADR-0009's
asymmetry prediction, naming the alternatives that lost — belongs to
`adr-0023-and-atom-resolutions` (HS-S0058), authored through `/redkiln:kb-ingest`,
never hand-written into `.kb/decisions/` (`project.md`, DR-7; `CLAUDE.md`, *Where
the work lives*). **Write no file under `.kb/` in this PR.**

**D9 — the adapter-observable boundary, stated so it is not confused with this.**
Two neighbouring artefacts exist and neither is this one. `ViolationAsStoreErrorStore`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:962`) already covers the
*portable* half — a violation reported through the wrong `AppendError` arm — for
every adapter. And `store_error_crosses_a_join_handle`, ES-6's own named rule, is
still unwritten and unowned (`.kb/open-questions/es-6-names-an-unwritable-rule.md`);
writing it is a testkit change against ADR-0009's marker and is **not** this story.
What remains for this story is a property of *this* error type's contents, which no
portable rule can state without asserting on an adapter's internals — and that
asymmetry is itself the reason the workspace's sole `!Send` adapter is the sole
instrument for the clause.

**Persona slice.** The observer here is the **library consumer** from the project's
backbone (`_storymap.md`, *Backbone*, row E) — someone holding an `AppendError`
returned from `append` at the edge, deciding whether to retry (transport) or to
re-read and rebuild their decision model (conflict). The artefact is written from
their seat, not from the adapter's.

## Integration contract

- **Archetype**: `capability`.
- **Slice / milestone**: `real-worker-bindings`. Slice-mates implemented in one
  context and mounted as one integrated surface: `worker-binding-layer`,
  `durable-object-write-path`, `durable-object-read-path`, and this story. Within
  the slice this story merges **last** (`_storymap.md`, *Merge order* §2).
- **Mount point**: `crates/happenstance-cloudflare/src/lib.rs` — the crate root.
  It is the composition root of this crate's public surface: it declares the four
  modules (`:128-131`), re-exports the caller-visible error surface
  (`pub use event_store::{CloudflareEventStore, CloudflareEventStoreError, SqlRowStream}`,
  `:133-135`), carries the ES-6 findings documentation D8 updates (`:12-21`,
  `:63-95`), and already hosts ES-6's *other* half — `not_send_probe` and the four
  probe tests (`:137-259`). This story mounts the information half beside the
  auto-trait half, in the same file, reachable by the same ordinary `cargo test`.
  Placement of the test body itself follows the tier question D-below resolves; the
  mount — the public surface it reads and the documentation it corrects — is
  `lib.rs` either way.
- **Wires into**:
  - `crates/happenstance-cloudflare/src/event_store.rs` — the one constructor
    `CloudflareEventStore::new(sql)` (`:70-87`), the `impl EventStore` block
    (`:145-196`), and `CloudflareEventStoreError`'s variants (`:105-143`). Every
    tier goes through that constructor and no test may build a store another way
    (`_decomposition.md`, Testing Notes §3).
  - `crates/happenstance-cloudflare/src/sql_storage.rs` — `SqlError::Thrown`
    (`:93-100`) and `SqlStorage::exec` (`:176-178`), the seam a constructed thrown
    value is injected through, after `worker-binding-layer` replaces them.
  - `crates/happenstance-cloudflare/src/js.rs` — `JsThrow` / `StringifiedThrow` and
    their two `is_constraint_violation` shapes (`:113-127`, `:152-162`), the
    comparison ADR-0009 asked for.
  - `happenstance_core::{EventStore, AppendError}` — the port and the arm the
    conflict signal actually travels on (`crates/happenstance-core/src/store.rs`,
    `crates/happenstance-core/src/error.rs`). Bind `EventStore`, the weaker flavour
    (`CLAUDE.md`, constraint 4).
- **Renders surfaces**: **none.** `_design.md` records no user-facing surface for
  this project and is signed off on that basis; its `## Items` and `## Signatures`
  blocks are `N/A`, so this story claims no surface id. Any public item added under
  D7 is recorded in this spec's behavior table instead, because there is no
  signature block to add it to.
- **Conformance rule(s)**: **none, and that is the point.** This behaviour is not
  adapter-observable by the shared suite: every event-store rule asserts on the
  success path or on a store-produced `AppendError` and none reads an adapter
  error's contents (`project.md`, DR-4), and the portable neighbour that *is*
  observable — a violation on the wrong `AppendError` arm — is already covered by
  `ViolationAsStoreErrorStore` (`crates/happenstance-testkit/tests/mutation_coverage.rs:962`).
  No rule is added to `happenstance-testkit` here, so the suite invariants (CF-6's
  literal positions, CF-33's clock, CF-29's changelog) are not engaged.
- **Clause(s)**: `spec/SPECIFICATION.md:2629` **ES-6 `[FROZEN]`** — *discharged
  with evidence, not amended*. This story supplies the real-error-type evidence the
  clause's own reasoning was written against a stand-in for. ES-6's `Rule:` line
  stays `(new)`/`†` and its scheduling gap stays open
  (`.kb/open-questions/es-6-names-an-unwritable-rule.md`); closing that is a
  separate, unowned testkit change. `cargo xtask spec-trace` must stay green.
- **Advances DoD scenario**: the project's **DoD 4** — "a test in the adapter's own
  tree recovers the constraint-violation-versus-transport distinction from a
  caller-visible error, and fails if the error type stops carrying it"
  (`project.md`, *Definition of done*, item 4), which is the boundary restatement of
  the initiative's DoD row for the constrained-runtime store. It also moves phase 9's
  second exit criterion, "ES-6 is decided", from assertion to artefact
  (`RUNBOOK.md:4294-4299`).

## PR boundary

**In this PR**

- One committed test that constructs a thrown value carrying
  `UNIQUE constraint failed: event.position`, drives it through the same
  classification path `append` uses, and asserts the caller receives
  `AppendError::ConditionViolated`.
- Its mirror: a thrown value that is *not* a constraint violation arrives as
  `AppendError::Store(..)` and a caller can still recover "transport fault" from the
  public surface alone.
- A named wrong error shape in this crate's own test tree — the evidence-discarding
  classifier of D3 — that the same assertion rejects, so the artefact is not
  decorative.
- The minimal public accessor, if and only if the real `worker` bindings leave the
  fact unreachable from outside the crate (D7), with its `# Errors`/intent
  documentation.
- The crate documentation's ES-6 findings section rewritten from prediction against a
  stand-in to observation against a real `worker::Error`, plus the one sentence
  handing the verdict to ADR-0023.
- The story's own backlog folder (`_ledger.md`, the implementation report).

**Explicitly not in this PR**

- Any change to `Error`'s bound, and any edit to `.kb/decisions/0009-error-send-sync.md`
  or to any accepted atom (D1).
- Any file under `.kb/` at all — the atom is `adr-0023-and-atom-resolutions`'s (D8).
- `store_error_crosses_a_join_handle` and any change to `happenstance-testkit`,
  including its registry, its rule enumeration and `mutation_coverage.rs` (D9).
- A `ConditionViolated` variant on `CloudflareEventStoreError`, or a new
  `AppendError` arm (D2).
- The classifier itself, and `append`/`head`/`contains_event_id`/`migrate` bodies —
  `durable-object-write-path`'s.
- The `worker` dependency swap, the `!Send`-ness of the error type, and the MSRV /
  `cargo deny` pricing of `worker` — `worker-binding-layer`'s.
- The `workerd` runner, the conformance target, the fixture and its limits — the
  `durable-object-conformance-run` milestone's.
- `spec/SPECIFICATION.md` edits of any kind.

**Merge DoD (one line).** Someone who did not do the work can run one `cargo test`,
see a test that recovers constraint-violation-versus-transport from a caller-visible
error, delete the information from the error type, and watch that same test fail.

The implementer may additionally touch the wiring files named under **Mount point**
and **Wires into** to mount this slice; that is not scope drift.

```
crates/happenstance-cloudflare/src/**
crates/happenstance-cloudflare/tests/**
crates/happenstance-cloudflare/Cargo.toml
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/caller-visible-error-verdict/**
```

`Cargo.toml` is inside the boundary for one reason only: the tier question below may
require `wasm-bindgen-test` in this crate's wasm32 dev-dependencies. If the host tier
holds, the manifest is not touched.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A constraint violation reaches the caller on the `ConditionViolated` channel | A `SqlStorage` whose `exec` throws a value carrying `UNIQUE constraint failed: event.position` is handed to `CloudflareEventStore::new(sql)`; `append` with a condition returns `Err(AppendError::ConditionViolated)`, never `AppendError::Store(..)`. The conflict signal is lifted out of `Self::Error` by the contract, so this asserts the *channel*, not a variant | `crates/happenstance-cloudflare/src/lib.rs:75-85`; `crates/happenstance-cloudflare/src/event_store.rs:100-104`, `:171-177`; `_decomposition.md`, Architecture, *Error data flow* (AC-001) |
| A transport fault reaches the caller distinguishably | A thrown value with non-constraint text arrives as `AppendError::Store(CloudflareEventStoreError::Sql(SqlError::Thrown(..)))` and a caller can recover "this was a transport fault" from the public surface — the variant, `Display`, or the `source` chain — without knowing which SQL statement ran | `crates/happenstance-cloudflare/src/sql_storage.rs:93-122`; `crates/happenstance-cloudflare/src/event_store.rs:105-143` (AC-002) |
| The reconstruction reads only what a downstream crate can read | No private field, no `pub(crate)` helper, no `#[cfg(test)]` back door. Store built through `CloudflareEventStore::new(sql)`; error read through `EventStore::append`'s return type. Bind `EventStore`, not `SendEventStore` | `crates/happenstance-cloudflare/src/event_store.rs:70-87`; `_decomposition.md`, Testing Notes §3; `CLAUDE.md`, constraint 4 (AC-003) |
| The artefact is not decorative | A named wrong error shape lives in this crate's test tree — the classifier that picks the right arm and discards the evidence — and the same assertion rejects it. Modelled on `the_probe_is_not_vacuous`, which exists because the `!Send` module would otherwise pass with a broken probe | `crates/happenstance-cloudflare/src/lib.rs:210-220`; `discover.md`, *The wrong implementation*; `CLAUDE.md`, *a rule that no adapter can fail is decorative* (AC-004) |
| The thrown value is constructed, not round-tripped | The exact Durable Object SQLite text is used as the fixture input, so the artefact does not depend on the `workerd` runner existing. Workers exposes no numeric code, so classification is a text test on both the live-handle and stringified shapes | `crates/happenstance-cloudflare/src/lib.rs:63-73`; `crates/happenstance-cloudflare/src/js.rs:113-127`, `:152-162`; `_decomposition.md`, Testing Notes §3 (AC-001, AC-002) |
| Any new public item is minimal and argued | `_design.md` declares no surface, so it neither supplies a signature to match nor licenses an API. If the real bindings leave the fact unreachable, add the smallest accessor that restores it, document it, keep the enum `#[non_exhaustive]`, and record it in this table's row on merge. Adding a `ConditionViolated` variant is forbidden outright | `crates/happenstance-cloudflare/src/event_store.rs:100-143`; `_design.md`, *Items* / *Signatures* (N/A); `standards/rust/70-rustdoc-obligations.md` (AC-003, AC-007) |
| The verdict is recorded where a reader meets the crate | The crate's ES-6 findings section moves from prediction-against-a-stand-in to observation-against-a-real-`worker::Error`, and names ADR-0023 as where the atom lands. No `.kb/` file is written | `crates/happenstance-cloudflare/src/lib.rs:12-21`, `:63-95`; `project.md`, DR-7; `RUNBOOK.md:4276-4292` (AC-005) |
| Reachable by an ordinary `cargo test` | Open question inherited from `worker-binding-layer` and settled at implementation, not before: if `cargo test -p happenstance-cloudflare` still links once `worker` is real, this lives host-native beside the probes (the brief's recommended default); if it does not, it moves behind `#[cfg(all(test, target_arch = "wasm32"))]` and re-emits through `wasm_bindgen_test`, alongside the four existing probe assertions, which must keep passing *somewhere an ordinary contributor's inner loop reaches* | `crates/happenstance-cloudflare/src/lib.rs:151-163`; `_decomposition.md`, Testing Notes §2; Architecture Notes §7 item 1 (AC-006) |
| Standing detectors stay loud | The four `!Send` probes including `the_probe_is_not_vacuous` still pass; `send_shape::send_flavour::SendStoreWithLocalError` still compiles; the two `read`-shape tests in `crates/happenstance-core/src/memory.rs` are untouched; `cargo xtask spec-trace` stays green with ES-6's markers intact | `crates/happenstance-cloudflare/src/lib.rs:180-259`, `:87-94`; `crates/happenstance-core/src/memory.rs`; `_decomposition.md`, Architecture Notes §7 (AC-007) |
| Story-grain verification | `cargo xtask affected --base main` green; the slice's `cargo xtask ci --fast` green at slice close. This story adds no gate step | `CLAUDE.md`, *Commands*; `_decomposition.md`, Testing Notes §4 (AC-006) |

## Data and migrations

**N/A.** This story adds no table, no column and no migration. The schema and its
identity columns (`origin_store`, `origin_position`) belong to
`durable-object-write-path` (`crates/happenstance-cloudflare/src/event_store.rs:8-24`,
`:187-195`), and the wire format is explicitly out of this project's scope
(`project.md`, *Out of scope* — `replication-identity-and-ingest` owns it). The only
persisted artefact this story touches is text in a doc comment. The one datum it
*consumes* is not stored at all: the message string
`UNIQUE constraint failed: event.position`, constructed in-test and never read from
storage (`crates/happenstance-cloudflare/src/lib.rs:63-73`).

## Acceptance criteria

Seven criteria, each written from the seat of the person who observes it. The persona
is the project backbone's **library consumer** (`_storymap.md`, *Backbone*, row E) —
someone holding an `AppendError` at the edge of their own handler — except where the
observer is the **adapter author** (row B) or the **gate reader** (row D), named in
the criterion. Every one crosses the full stack this story has: constructed thrown
value → `SqlStorage::exec` → the classifier `durable-object-write-path` landed →
`EventStore::append` → what a caller can read. All seven together are project
`AC-005`'s committed-reconstruction half and the project's DoD 4 (`project.md`,
*Definition of done*, item 4).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** a library consumer appending under an `AppendCondition` against a Durable Object whose SQLite refuses the write with the real text `UNIQUE constraint failed: event.position`, **WHEN** they call `append` on a store built the only way there is — `CloudflareEventStore::new(sql)` — **THEN** they receive `Err(AppendError::ConditionViolated)` and nothing else, so their next move is "re-read and rebuild the decision model" and never "retry the transport". The thrown value is *constructed in-test* with that exact text, not round-tripped through a live Durable Object, so the artefact stands whether or not the `workerd` runner exists yet | `#[test] constraint_violation_reaches_the_caller_as_condition_violated` in `crates/happenstance-cloudflare/src/lib.rs` `mod tests` (tier fixed by AC-006), driving `<CloudflareEventStore as EventStore>::append`; asserts the `ConditionViolated` **channel**, not a variant on `CloudflareEventStoreError` — that variant must not exist (`crates/happenstance-cloudflare/src/event_store.rs:100-104`). Run by `cargo test -p happenstance-cloudflare` |
| AC-002 | **GIVEN** the same consumer, and a Durable Object that fails for a reason that is *not* a constraint violation, **WHEN** `append` returns, **THEN** they receive `Err(AppendError::Store(CloudflareEventStoreError::Sql(SqlError::Thrown(..))))` and can recover, from the public surface alone, that this was a **transport fault and not a conflict** — enough to retry rather than rebuild. "Enough" is asserted on the variant plus the thrown text reachable through `Display` or `core::error::Error::source`, never on which SQL statement ran | `#[test] transport_fault_reaches_the_caller_distinguishably`, same target; asserts the `Store` arm, the `Sql(Thrown(..))` shape, and that the recovered text is not the constraint text. Paired with AC-001 in one file so the two-sidedness is visible. Run by `cargo test -p happenstance-cloudflare` |
| AC-003 | **GIVEN** a *downstream* consumer — someone who has only `happenstance-cloudflare` as a dependency and cannot see its internals — **WHEN** they attempt the reconstruction AC-001 and AC-002 assert, **THEN** they can do it: the tests reach the store through `CloudflareEventStore::new(sql)`, bind `EventStore` (not `SendEventStore`, `CLAUDE.md` constraint 4), and read only `pub` items — no private field, no `pub(crate)` helper, no `#[cfg(test)]` back door. If the real `worker` bindings leave the fact unreachable, the **minimal** accessor that restores it is added, documented per `standards/rust/70-rustdoc-obligations.md`, keeps `CloudflareEventStoreError` `#[non_exhaustive]`, and is recorded in this spec's behavior table on merge | Review of the test bodies against the crate's public surface, plus the mechanical check that they reach every item by public path (no `super::` reach into private state); `cargo doc -p happenstance-cloudflare` shows every item AC-001/AC-002 depend on; `cargo clippy … -D warnings` holds the rustdoc obligations on anything new |
| AC-004 | **GIVEN** the **adapter author** who wants to trust this artefact rather than admire it, **WHEN** they read the diff, **THEN** they find a *named wrong error shape* in this crate's own test tree — D3's classifier that picks the right `AppendError` arm and then discards the evidence — and the same assertion AC-002 uses rejects it. Modelled on `the_probe_is_not_vacuous`, which exists because the `!Send` module would otherwise pass with a broken probe; without it this is a rule no adapter can fail (`CLAUDE.md`, *a rule that no adapter can fail is decorative*) | `#[test] an_evidence_discarding_classifier_is_rejected` (or equivalently named), asserting the shared AC-002 predicate returns **false** against the wrong shape. Pattern: `crates/happenstance-cloudflare/src/lib.rs:210-220`. Run by `cargo test -p happenstance-cloudflare` |
| AC-005 | **GIVEN** the **gate reader** who opens the crate documentation to find out what ES-6 resolved to, **WHEN** they read the Findings section, **THEN** it reads as *observation against a real `worker::Error`* rather than prediction against a stand-in — findings 2 and 3 in particular — and names `adr-0023-and-atom-resolutions` as where the atom lands. **AND** the diff writes no file under `.kb/`: the verdict is recorded here and minted there, through `/redkiln:kb-ingest`, never hand-written (`project.md`, DR-7) | Review of `crates/happenstance-cloudflare/src/lib.rs:12-21`, `:63-95` in the diff; mechanically, `git diff --name-only` against the merge base contains no path under `.kb/`, and `redkiln validate --kb` stays clean (an edited accepted atom fails it against `HEAD`). Docs build via `cargo xtask ci`'s docs step |
| AC-006 | **GIVEN** a contributor with no `workerd` toolchain — an ordinary inner loop — **WHEN** they run one `cargo test`, **THEN** they reach AC-001 through AC-004 *and* the four `!Send` probes: ES-6's information half and its auto-trait half answer to the same command. Which target hosts them is settled at implementation, not before — host-native beside the probes if `cargo test -p happenstance-cloudflare` still links once `worker` is real (the testing brief's recommended default), otherwise behind `#[cfg(all(test, target_arch = "wasm32"))]` re-emitting through `wasm_bindgen_test`, with the probes moved too and never deleted | `cargo test -p happenstance-cloudflare` (or, in the wasm32 branch, the crate's `wasm-bindgen-test` invocation) lists all four new tests **and** `the_probe_is_not_vacuous`; `cargo xtask affected --base main` green at story close and `cargo xtask ci --fast` green at slice close |
| AC-007 | **GIVEN** the gate reader again, **WHEN** the slice merges, **THEN** nothing this story did weakened a standing detector: `the_error_type_is_not_send`, `the_js_boundary_types_are_not_send`, `the_send_flavour_does_not_imply_a_send_error` and `the_probe_is_not_vacuous` all still pass; `send_shape::send_flavour::SendStoreWithLocalError` still compiles; `Error`'s bound is untouched and `.kb/decisions/0009-error-send-sync.md` unedited; ES-6's `[FROZEN]` clause is discharged with evidence, not amended | `cargo test -p happenstance-cloudflare` over `crates/happenstance-cloudflare/src/lib.rs:180-259`; `cargo xtask spec-trace` green with `spec/SPECIFICATION.md:2629`'s markers and `Rule:` line unchanged; `redkiln validate --kb` clean; the diff shows no change to `happenstance-core`'s port definitions |

Coverage of project `AC-005`'s committed-test half: AC-001 + AC-002 are the
distinction, AC-003 + AC-004 make it *caller-visible* and *failable*, AC-005 records
it, AC-006 keeps it reachable, AC-007 proves nothing else moved. The other two halves
stay where `_storymap.md`'s *Coverage* table puts them — `worker-binding-layer` keeps
the error type genuinely `!Send`, `adr-0023-and-atom-resolutions` mints the record.

## Interaction quality

**This story renders no surface, and that is signed off.** `_design.md` records
`N/A — no user-facing surface` in every block including `## Items`, `## Signatures`
and `## Anti-patterns`, approved 2026-08-12 at the `/redkiln:plan` design sign-off
gate. There is therefore no composition, transience, density budget or hierarchy to
honour, and `design.capture` remains a **declared** skip (`CLAUDE.md`, *Where the work
lives*). The families below are recorded as the API-surface analogues the same
invariants take in a library, so this is a mapping rather than a silence — and every
invariant that applies is **carried by an AC row in the table above**, never by a
bullet here.

**STATE family — the analogues that do apply.**

| invariant | its form here | carried by | verified by |
| --- | --- | --- | --- |
| Non-occlusion | The fact a caller branches on is not hidden behind crate privacy: it is readable from outside the crate, or a minimal accessor is added | AC-003 | tests reach only `pub` items; `cargo doc -p happenstance-cloudflare` |
| Keyboard reachability | The library analogue is *inner-loop* reachability: one ordinary `cargo test`, no `workerd` toolchain, exercises both halves of ES-6 | AC-006 | `cargo test -p happenstance-cloudflare` lists the four tests and the probes |
| Reversibility | The caller's recovery path is unambiguous — conflict means re-read and rebuild, transport means retry — because the two arrive on different channels and stay distinguishable | AC-001, AC-002 | the paired tests assert both directions |
| Preserved state | Standing detectors are preserved across the change rather than relocated away: the four probes and `SendStoreWithLocalError` still hold, moved with the tier if the tier moves, never deleted | AC-006, AC-007 | `crates/happenstance-cloudflare/src/lib.rs:180-259` still green |
| In-place, not context-jump | ES-6's two halves are mounted in the same file and answer to the same command, rather than split across a runtime a contributor must go and install | AC-006 | mount point `crates/happenstance-cloudflare/src/lib.rs` |

**COMPOSITION family — `N/A` by sign-off, with one exception that is not optional.**
"Presentation exists at all" has a real analogue for an error type, and it is the
whole story: an error whose only presentation is `"a SQL error occurred"` is the
bare-markup failure mode, and an unstyled render that satisfies every assertion is
exactly D3's evidence-discarding classifier. That invariant is carried by **AC-002**
(the variant plus the text recovered through `Display`/`source`) and made non-vacuous
by **AC-004** (the named wrong shape the same assertion rejects). Placement,
transience, density budget and hierarchy have no analogue in a two-direction
reconstruction and are not invented here. The design's named anti-patterns are
`N/A — no user-facing surface`; this story's operative anti-patterns are D2 (adding a
`ConditionViolated` variant), D3 (discarding the evidence) and D4 (reading a private
field), all three carried as AC rows rather than as prose.

## Error conditions

| id | condition | required behaviour | evidence |
| --- | --- | --- | --- |
| EC-001 | The classification probe itself fails — `JsThrow::is_constraint_violation` returns `Err(Self)` because the property lookup throws | The caller must **not** receive a silent "not a violation". An unclassifiable throw arrives on the `Store` channel still carrying the thrown value, so a caller can tell "the store failed and could not say why" from "the store says this was transport". Never collapse `Err` to `false` — that turns a conflict into a retry loop against a condition that will never pass | `crates/happenstance-cloudflare/src/js.rs:113-127` — the `Result<bool, Self>` signature is deliberate |
| EC-002 | A non-constraint failure whose text happens to contain `constraint failed`, or a violation of a *different* constraint than the position uniqueness the condition guards | The substring test is the documented mechanism and Workers exposes no numeric code, so a false positive is possible by construction. This story does not fix the classifier — that is `durable-object-write-path`'s — but the fixture must use the *exact* documented text so it never certifies a looser match than the classifier makes; a mismatch between the two is a finding for `adr-0023-and-atom-resolutions`, not a quiet widening | `crates/happenstance-cloudflare/src/lib.rs:63-73`; `crates/happenstance-cloudflare/src/js.rs:152-162` |
| EC-003 | After the real `worker` swap the fact is **not** reachable from outside the crate | Do not reach into a private field to make the test pass — that is precisely the regression the test exists to catch (D4). Add the minimal `pub` accessor under D7 with its `# Errors`/intent documentation, keep the enum `#[non_exhaustive]`, and record it in the behavior table. If no minimal accessor is defensible, halt and raise it rather than shipping a test that reads internals | `crates/happenstance-cloudflare/src/event_store.rs:100-143`; `standards/rust/70-rustdoc-obligations.md` |
| EC-004 | `cargo test -p happenstance-cloudflare` stops linking once `worker` is a real dependency | Move this story's tests **and** the four probes behind `#[cfg(all(test, target_arch = "wasm32"))]`, re-emit through `wasm_bindgen_test`, and add the dev-dependency to `Cargo.toml`'s wasm32 section — the only reason `Cargo.toml` is inside the PR boundary. Deleting a probe to make the tier question go away is the forbidden move | `_decomposition.md`, Testing Notes §2 (`:587-616`); `crates/happenstance-cloudflare/src/lib.rs:149-155` |
| EC-005 | `durable-object-write-path` has not landed the classifier when this story is picked up | Halt loudly. This story does not stub a classifier, does not implement `append`, and does not assert against a `todo!()` body. Within the slice it merges **last** precisely so the classification site exists | `crates/happenstance-cloudflare/src/event_store.rs:171-177`; `_storymap.md`, *Slices* |
| EC-006 | The artefact **refutes** ADR-0009 — a real `worker::Error` turns out to carry nothing a caller can act on | A finding, not a failure of this story, and it is recorded rather than acted on: the Findings section states it (AC-005) and `adr-0023-and-atom-resolutions` mints the atom naming the alternatives that lost. Editing `.kb/decisions/0009-error-send-sync.md` or `spec/SPECIFICATION.md:2629` is out of bounds in either direction | `.kb/decisions/0009-error-send-sync.md`; `project.md`, DR-7; `discover.md`, *Gate: Discover*, Box 7 |

## Non-functional

| id | requirement | why it binds here |
| --- | --- | --- |
| NF-001 | **The artefact does not depend on the `workerd` runner.** The thrown value is constructed in-test; nothing added here needs the conformance harness, the Durable Object host, or the fixture to exist | DoD 4 is the one exit criterion the runbook says has no artefact behind it (`RUNBOOK.md:4276-4292`); it must not be the one waiting on the project's riskiest infrastructure (`_decomposition.md`, Testing Notes §3) |
| NF-002 | **No new dependency unless EC-004 fires**, and then only `wasm-bindgen-test` in the wasm32 dev-dependencies. `cargo deny` stays green on the widened graph and ADR-0029's 1.97.1 floor still holds | `cargo deny` and `cargo hack` both resolve on this machine and therefore run rather than skip (`CLAUDE.md`, *Commands*); `worker`'s own MSRV pricing is `worker-binding-layer`'s and must not be re-paid here |
| NF-003 | **`-D warnings` clean; `unsafe_code = "forbid"` untouched.** Any new public item carries full rustdoc including `# Errors`; no `#[allow]` is added, and the crate's scoped `#![allow(clippy::todo)]` is not this story's to remove | `CLAUDE.md`, *House style*; `standards/rust/70-rustdoc-obligations.md`; `crates/happenstance-cloudflare/src/lib.rs:121-126` |
| NF-004 | **No new gate step and no measurable gate-time cost.** Four `#[test]`s over constructed values; the `REQUIRED`/`OPTIONAL` step arrays in `xtask/src/main.rs` are untouched | `_decomposition.md`, Testing Notes §4; the wasm32-execution step belongs to `wasm-execution-gate-step` |
| NF-005 | **The tests are legible to someone who did not do the work.** Each carries a doc comment naming what a caller *does* with the answer — retry versus rebuild — in the register of `the_probe_is_not_vacuous`'s comment | the Merge DoD is a stranger running one command and reading one file; `crates/happenstance-cloudflare/src/lib.rs:207-220` |
| NF-006 | **Determinism.** No clock, no network, no ambient state: the same thrown text yields the same verdict on every run and on either target the tier question could land on | the suite's clock and position rules are not engaged (see *Integration contract*, *Conformance rule(s)*) |

## Implementation notes (non-prescriptive)

Shape suggestions only — the AC table and the PR boundary are what bind.

- **Order of work.** Resolve AC-006's tier question *first*, by simply running
  `cargo test -p happenstance-cloudflare` after the slice's `worker` swap. The answer
  decides where the tests live and whether `Cargo.toml` is touched at all; it is a
  one-command observation, not a design exercise.
- **One file, two directions, one control.** The four tests want to be read together —
  violation, transport, positive control, and whatever the tier demands — because
  their value is the *pairing*. Beside `mod tests` in
  `crates/happenstance-cloudflare/src/lib.rs` keeps ES-6's two halves adjacent; a
  sibling `mod es6_reconstruction` in the same file is a fine alternative if `mod tests`
  is getting long.
- **The injection seam** is `SqlStorage::exec`
  (`crates/happenstance-cloudflare/src/sql_storage.rs:176`), which returns
  `Result<SqlCursor, SqlError>` and is the one place a thrown value enters. Once
  `worker-binding-layer` replaces it, prefer whatever the real binding offers for
  building a thrown `worker::Error` from a message string over any test-only trait the
  crate would then have to keep public.
- **The positive control needs no store.** D3's wrong shape is a *classifier*, not an
  adapter: a local function or type in the test module that maps a thrown value to an
  `AppendError` while discarding the evidence, run through the same predicate AC-002
  asserts with. Factoring that predicate into one named helper both tests call is what
  makes AC-004 mean anything.
- **Do not generalise.** No trait, no macro, no testkit rule.
  `ViolationAsStoreErrorStore` already owns the portable half
  (`crates/happenstance-testkit/tests/mutation_coverage.rs:962`), and the unportable
  half is the point (D9).
- **The documentation edit is surgical.** Findings 2 and 3
  (`crates/happenstance-cloudflare/src/lib.rs:63-95`) already state the prediction in
  the right words; what changes is the evidential mood plus one sentence pointing at
  ADR-0023. Do not restructure the Findings section.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`'s Testing brief — the tiers are its tiers, and this
story lives entirely in the fourth ("Targeted runtime tests, outside the conformance
macro", `:572-578`) plus the static tier.

| tier | command / path | proves |
| --- | --- | --- |
| Static | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` | NF-003: no new `#[allow]`, no undocumented public item, rustdoc lints clean on any accessor added under EC-003 |
| Static | `cargo xtask spec-trace` | AC-007: `spec/SPECIFICATION.md:2629`'s ES-6 markers, `Rule:` line and citations still resolve — the clause is discharged with evidence, not amended |
| Static | `cargo doc -p happenstance-cloudflare` (the `cargo xtask ci` docs step) | AC-003, AC-005: the surface the tests read is the surface a downstream crate sees, and the rewritten Findings section builds |
| Unit (this story's deliverable) | `cargo test -p happenstance-cloudflare` → `crates/happenstance-cloudflare/src/lib.rs` `mod tests` | AC-001, AC-002: both directions of the reconstruction, through `CloudflareEventStore::new` and `EventStore::append` |
| Unit (positive control) | same target — `an_evidence_discarding_classifier_is_rejected` | AC-004: the assertion can fail, so the artefact is not decorative (`CLAUDE.md` corollary) |
| Unit (standing detectors) | same target — `crates/happenstance-cloudflare/src/lib.rs:180-259` | AC-006, AC-007: the four `!Send` probes including `the_probe_is_not_vacuous` still pass and are still reachable by an ordinary `cargo test` |
| Unit (wasm32 branch, only if EC-004 fires) | `wasm-bindgen-test` over the same module under `#[cfg(all(test, target_arch = "wasm32"))]` | AC-006: probes and reconstruction moved together, neither deleted (`_decomposition.md`, Testing Notes §2) |
| Process gate | `redkiln validate --kb && redkiln doctor` | AC-005, AC-007: no `.kb/` file written and no accepted atom edited — `0009-error-send-sync.md` is checked against `HEAD` |
| Story grain | `cargo xtask affected --base main` | the story-grain bar `.redkiln/config.yaml`'s `verify:` block wires to `redkiln advance` (`CLAUDE.md`, *Commands*) |
| Slice / project grain | `cargo xtask ci --fast` at slice close | `REQUIRED` only — the bar this **non-terminal** project is held to (`_decomposition.md`, Testing Notes §4; `xtask/src/main.rs:853`) |

No conformance rule is added, so `happenstance-testkit`'s registry, its rule
enumeration, `mutation_coverage.rs` and CF-29's changelog obligation are not engaged.

## Risks and coupling (PR-scoped)

| risk | how it bites this PR | mitigation held here |
| --- | --- | --- |
| **The test that cannot fail.** The likeliest outcome is an artefact that asserts what the classifier already guarantees and would pass against D3's evidence-discarding version | DoD 4 discharged by assertion — exactly what `RUNBOOK.md:4276-4292` refuses | AC-004 is a blocking row with its own named wrong shape; a reviewer can delete the evidence from the error type and watch AC-002 fail (the Merge DoD) |
| **The private-field shortcut.** Under pressure the fastest green is a `pub(crate)` helper or a `#[cfg(test)]` accessor | The test survives the exact regression it exists to catch, silently, forever | AC-003 is blocking and names the reachable set; EC-003 supplies the sanctioned escape (a minimal, documented accessor) so the shortcut is never the only way through |
| **Convenience restores `Send`.** Reaching for `StringifiedThrow` to make the fixture easier destroys the workspace's only ES-6 instrument | Project AC-005 collapses — there would be no `!Send` error type left to decide ES-6 against | D6; AC-007 keeps the four probes blocking, and the `!Send` half is `worker-binding-layer`'s — this story must not "help" |
| **Tier drift.** EC-004 fires, `wasm-bindgen-test` lands, and the probes get left host-side "for now" | ES-6's two halves split across two commands; an ordinary inner loop stops exercising one of them | AC-006 requires both halves reachable by the same ordinary `cargo test`, per Architecture brief Notes §7 item 1 |
| **Coupling to `durable-object-write-path`.** The classifier's shape is not this story's to choose, and a late change to it invalidates the fixture text | Rework inside one slice rather than across slices | Same slice, one context, this story merges last (`_storymap.md`, *Merge order* §2); EC-005 says halt rather than stub |
| **Scope creep into the KB.** Having produced the verdict, writing the atom is one file away | Two atoms for one decision, or a hand-written `.kb/decisions/` entry bypassing `/redkiln:kb-ingest` — the failure already reverted once (`CLAUDE.md`, *Where the work lives*) | D8 and the PR boundary forbid any `.kb/` write; AC-005 checks it mechanically with `git diff --name-only` |
| **Stale project AC-005 wording.** It still reads "bound added, or ADR-0009's deferral confirmed", which reads ES-6 as open | An implementer could add `+ Send + Sync` to `Error` believing it in scope | D1, Clarification 1 below, and AC-007's "`Error`'s bound is untouched"; the flag is already recorded twice (`_grounding.md:38-43`; `_decomposition.md:132-137`) |

## Dependencies

**Blocks on** — `durable-object-write-path` (HS-S0050). It lands the classifier: the
site where a thrown `worker::Error` becomes `AppendError::ConditionViolated`,
`AppendError::ExceedsStoreLimit` or `AppendError::Store(..)` before `Self::Error`
exists. Without it there is nothing to reconstruct *from*, and `append` is still
`todo!()` (`crates/happenstance-cloudflare/src/event_store.rs:171-177`). This matches
the story's `depends_on` exactly: `["durable-object-write-path"]`.

**Slice-mates** (`real-worker-bindings`, implemented in one context and mounted as one
integrated surface): `worker-binding-layer` → `durable-object-write-path` /
`durable-object-read-path` → **this story, last** (`_storymap.md`, *Merge order* §2).

**Unlocks** — `adr-0023-and-atom-resolutions` (HS-S0058), which names
`caller-visible-error-verdict` in its own `depends_on` and mints the atom this story's
observation feeds. It is also one of the three stories `_storymap.md`'s *Coverage*
table assigns to project AC-005; the other two own the `!Send` half and the record.

**Explicitly not a dependency** — `every-rule-under-workerd`, the fixture, the host,
and the whole `durable-object-conformance-run` milestone. NF-001 makes that
independence a requirement rather than an accident.

## Anchors (progressive disclosure)

Deferred depth, signposted and AC-bound. Link, do not paste; open each at the moment
named.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/decisions/0009-error-send-sync.md` | The **accepted, immutable** decision that settled ES-6: `Error` keeps `core::error::Error + 'static` on both ports and both flavours, with the stronger property downstream in a blanket-implemented marker. It is the boundary this story must not cross | Before writing a line — it is what puts "add `+ Send + Sync`" out of bounds; re-read if EC-006 fires | AC-007 |
| `crates/happenstance-cloudflare/src/event_store.rs` | The one constructor `new(sql)` (`:70-87`), the `impl EventStore` block (`:145-196`), and `CloudflareEventStoreError`'s variants with the deliberate **absence** of `ConditionViolated` (`:100-143`) | Opening move for AC-001/AC-002: it fixes what the test may construct and what it may read | AC-001 |
| `crates/happenstance-cloudflare/src/lib.rs` | The mount point. Carries the Findings section AC-005 rewrites (`:12-21`, `:63-95`), the exact SQLite text the fixture uses (`:63-73`), and the probe tests with `the_probe_is_not_vacuous` as AC-004's model (`:180-259`) | Before writing the tests, and again before editing the documentation | AC-005 |
| `crates/happenstance-cloudflare/src/js.rs` | The two classification shapes and their contract: `JsThrow::is_constraint_violation` returns `Result<bool, Self>` (`:113-127`); `StringifiedThrow`'s is a substring test (`:152-162`). EC-001 and EC-002 are read straight off these | When deciding what the constructed thrown value must contain, and when writing EC-001's assertion | AC-002 |
| `crates/happenstance-cloudflare/src/sql_storage.rs` | `SqlError::Thrown` (`:93-100`), `SqlStorage::exec` (`:176`) — the seam a constructed thrown value is injected through — and the doc comment on why `Thrown` carries a handle rather than a `String` (`:85`) | When wiring the fixture, after the `worker` swap has replaced these types | AC-002 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `ViolationAsStoreErrorStore` (`:962`) already covers the *portable* half for every adapter. Reading it is how you confirm this story must **not** add a testkit rule | Before any urge to generalise the artefact into the conformance suite | AC-004 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` | The Testing brief: the tier taxonomy (§1), the open target question this story inherits (§2, `:587-616`), and the constructed-thrown-value fixture rule (§3, `:638-642`); Architecture Notes §6/§7 carry the live-value trade and the standing detectors | §2 when resolving AC-006's tier; §3 before building the fixture | AC-006 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` | DR-4 (no conformance rule reads an adapter error's contents — the structural reason this story exists), DR-7 (atoms are minted through ingest), AC-005's stale wording, and DoD item 4 verbatim (`:250-265`) | Before disputing whether the conformance run could have covered this, and before touching anything under `.kb/` | AC-005 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/caller-visible-error-verdict/discover.md` | The wrong implementation named in full — both the blunt and the subtle shape (`:90-121`) — plus the signal ledger behind every Context-pack decision | Before writing AC-004's positive control; it is the spec for what that control must model | AC-004 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` | The slice, the merge order within it (this story last), and the *Coverage* table's three-way split of project AC-005 | When sequencing against slice-mates, and when tempted to own the `!Send` half or the atom | AC-006 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` | The signed-off `N/A — no surface` determination in every block including `## Signatures` and `## Anti-patterns`. It licenses no new public API — the constraint EC-003 operates under | Before adding any public item under EC-003 | AC-003 |
| `standards/rust/70-rustdoc-obligations.md` | The house rule for what a new public item owes: intent, `# Errors`, and a compiled example, held by `-D warnings` | At the moment EC-003 fires, not before | AC-003 |
| `spec/SPECIFICATION.md` | ES-6 at `:2629`, `[FROZEN]`, its `Rule:` line still `(new)`/`†`. This story supplies evidence for it and changes nothing about it; `cargo xtask spec-trace` enforces that | Before and after the documentation edit, to confirm no marker moved | AC-007 |
| `.kb/open-questions/es-6-names-an-unwritable-rule.md` | The deliberately-open scheduling gap: `store_error_crosses_a_join_handle` is unwritable against today's port for *every* adapter. It stays open, and writing it is not this story | If tempted to "finish" ES-6 by writing its named rule | AC-004 |
| `RUNBOOK.md` | Phase 9's proof artefact, second half (`:4276-4292`) and the exit criterion "ES-6 is decided" (`:4294-4299`) — the roadmap-level statement that a green suite is not the artefact | Once for orientation on why this story exists; again at close, to confirm the criterion is now backed | AC-005 |

## Clarifications resolved during spec

1. **Project AC-005's wording is stale, and this spec obeys the atom.** "bound added,
   or ADR-0009's deferral confirmed" (`project.md:214-218`) predates
   `.kb/decisions/0009-error-send-sync.md`'s acceptance and reads ES-6 as still open.
   It is not. The spec follows the accepted atom — authority order puts accepted
   decision atoms first — and the flag is already recorded twice upstream
   (`_grounding.md:38-43`; `_decomposition.md:132-137`) rather than discovered here. No
   project artifact is edited to correct the wording; that is not this story's grain.
2. **The AC set is exactly the seven the front half decided.** AC-001 … AC-007, none
   added and none dropped. The behavior table's parenthesised tags map onto them
   one-to-one, with the "any new public item is minimal and argued" row deliberately
   split across AC-003 (it must stay reachable) and AC-007 (nothing else regressed),
   because that row can bite in both directions.
3. **The tier question is deferred to implementation, deliberately.** Host-native
   versus `wasm32` is `worker-binding-layer`'s open question (`_decomposition.md`,
   Testing Notes §2) and is settled by running one command after the swap, not by
   predicting. AC-006 binds the *invariant* — one ordinary `cargo test` reaches both
   halves of ES-6 — rather than the answer. It is the only reason `Cargo.toml` sits
   inside the PR boundary.
4. **"Caller-visible" is defined operationally.** It means constructible and readable
   by a crate that depends on `happenstance-cloudflare` and nothing else: the one
   constructor, the `EventStore` port, `pub` variants, `Display`, and the
   `core::error::Error::source` chain. Anything a downstream crate cannot reach does
   not count — which is what makes AC-003 a blocking row rather than a style note.
5. **No conformance rule, and no surface-shaped interaction ACs.** The behaviour is not
   adapter-observable by the shared suite (DR-4) and `_design.md` records no surface,
   so *Interaction quality* maps the applicable invariants onto existing AC rows rather
   than inventing criteria with nothing to render. The one composition invariant with a
   real analogue — presentation exists at all — is carried by AC-002 and made
   non-vacuous by AC-004.
6. **The verdict's destination is fixed.** The observation lands in the crate's own
   documentation (AC-005); the atom lands via `adr-0023-and-atom-resolutions` through
   `/redkiln:kb-ingest`. This PR writes no file under `.kb/`, in the confirming case
   and in the refuting case alike (EC-006).
