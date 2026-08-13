---
item: HS-S0049
stage: spec
created: 2026-08-12T13:46:47.456Z
updated: 2026-08-12T13:46:47.456Z
template_sig: 87bbf1d0
rendered_sig: 2919f5e2
---

# Spec — The real Durable Object SqlStorage bindings replace the stand-in

## Scope lock

| Anchor | Path |
| --- | --- |
| Initiative | `.bklg/from-contract-to-published-library/initiative.md` (Goals; **Definition of Done** 4) |
| Project | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` (AC-001, AC-005) |
| This spec | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/worker-binding-layer/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` — Architecture §1 (the seam table), §2 (the constraining atoms), §3 (the stand-in → real binding map), §6 (the implementer's latitude), §7 (standing detectors); Testing §1–§2 (the tier question after the swap); Deployment §4 (dependency-surface consequences of `worker`) |
| Story map | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` — the `worker-binding-layer` row and **Merge order** §2 |
| Design | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` — **N/A, no user-facing surface**, signed off 2026-08-12. Nothing here renders |
| Story discovery | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/worker-binding-layer/discover.md` — the signal ledger, the answered questions, and the named wrong implementation |
| Roadmap | `RUNBOOK.md:160` (phase 9 row), `RUNBOOK.md:4241-4307` (phase 9's goal, work and exit criteria) |

## One-line PR slice

Replace the `worker`-free stand-in (`crates/happenstance-cloudflare/src/js.rs`,
`src/sql_storage.rs`) with real Durable Object `SqlStorage` bindings, keeping all four
modelled properties true and every type `Rc`-shaped so the `!Send` probes at
`crates/happenstance-cloudflare/src/lib.rs:137-259` still hold — and price `worker`
against ADR-0029's MSRV floor and `cargo deny` on the way in.

## Executive summary

This PR lands the **dependency swap and nothing else**. `happenstance-cloudflare` stops
modelling a Durable Object and starts binding to one: `worker` enters the graph, the two
stand-in modules become real bindings, and every property the stand-in was constructed to
hold is re-established against code that actually talks to the runtime.

The delta against the tree as it stands:

- `crates/happenstance-cloudflare/Cargo.toml:19-25` currently carries a note saying `worker`
  is *deliberately* absent. That trade is reversed here, and the note is **replaced by the
  record of the reversal**, not deleted — it is the reasoning a reviewer checks the swap
  against.
- `src/js.rs` and `src/sql_storage.rs` lose their five `todo!()`s (`js.rs:91`, `:126`;
  `sql_storage.rs:177`, `:183`, `:234`, `:243`, `:251`) because those are *binding* bodies.
  `src/event_store.rs`'s six stay: `migrate`, `append`, `head`, `contains_event_id`,
  `render_read`, `decode_row` are the next two stories'. The scoped
  `#![allow(clippy::todo)]` (`src/lib.rs:121-126`) therefore **stays** in this PR and is
  removed by whichever body lands last (`_storymap.md`, **Coverage**, AC-001 row).
- The `!Send` probe module (`src/lib.rs:137-259`) gains a `wasm32` twin. Today all four
  assertions are `#[cfg(all(test, not(target_arch = "wasm32")))]` — they run only on a
  platform that has no Durable Objects. After the swap the auto-trait leak they exist to
  catch (`unsafe impl Send for JsValue` under `cfg(not(target_feature = "atomics"))`,
  quoted at `src/lib.rs:33-61`) can only appear **on the target**, so a host-only probe is a
  detector pointing away from the thing it detects.
- `xtask/src/main.rs:245-264`'s `wasm32 build of the Cloudflare adapter` step gains
  `--tests`. Without it the step is a plain `cargo check`, `#[cfg(test)]` code is not
  compiled, and the wasm32 twin above would not be built by anything in the gate on the day
  it merges — a decoration rather than a detector.

What this PR does **not** land: any `EventStore` body, the fixture, the host, the
conformance target, the gate's execution step, or a single line of `.kb/`. Those are the
four stories that consume this one.

## Context pack

Read this section and you can start. Everything deeper is behind a signposted anchor.

**1. This is substrate, and its consumers are in the same slice.** `durable-object-write-path`,
`durable-object-read-path` and `caller-visible-error-verdict` all `depends_on` this story and
merge behind it in the same milestone (`_storymap.md`, **Merge order** §2). It exists as its own
story for one reason: it carries three risks — `worker`'s MSRV, `cargo deny`'s widened graph, and
`Send`-ness restored by accident — that must be priced in one diff instead of smeared across
three. It is `archetype: foundation`, but "foundation" here still means *real substrate, mounted*:
the crate must compile, its probes must run, and the gate must be green on merge day.

**2. The bound is not reopened. The instrument is.** ES-6 is `[FROZEN]`
(`spec/SPECIFICATION.md:2629-2686`, ledger row `:8588`) and ADR-0009 settled it: `Error` keeps
`core::error::Error + 'static` on both ports and both flavours, and the stronger property lives in
a downstream marker (`.kb/decisions/0009-error-send-sync.md`). The clause names *this crate's*
`Rc<str>`-backed error as what makes the decision falsifiable at all. So this story's obligation is
not to re-litigate the bound — it is to keep the clause's own instrument genuinely `!Send` once it
carries a real `worker::Error`. If that turned out to be impossible, the outcome is a new decision
atom and a re-plan, **never** an edit to an accepted atom, which `redkiln validate --kb` checks
against `HEAD`.

**3. The named wrong implementation, which the diff must not be.** `worker` links for `wasm32`;
the natural way to keep `cargo test -p happenstance-cloudflare` building is a `cfg` — the real
`worker::Error`-carrying variant behind `#[cfg(target_arch = "wasm32")]`, the `Rc<str>` stand-in
kept behind `#[cfg(not(...))]`. Count what each check then sees: `cargo test` compiles the
stand-in, so all four probes pass exactly as today including `the_probe_is_not_vacuous`; clippy is
green; and the gate's `wasm32` step type-checks the real variant and never asks whether it is
`Send`, because `Send`-ness is not a compile error — it is the *absence* of an obligation, and only
an autoref probe can observe an absent auto trait. The workspace ends up with an ES-6 instrument
for the host, on a platform with no Durable Objects, while the target silently inherits
`wasm-bindgen`'s `unsafe impl`. **The detector must live where the claim is made**: a
`#[cfg(all(test, target_arch = "wasm32"))]` twin of all four assertions, `the_probe_is_not_vacuous`
included — a positive control on the host proves nothing about a probe compiled for another target
(`discover.md`, **The wrong implementation**).

**4. The dev-dependency argument inverts, and that is the licence for the twin.**
`src/lib.rs:149-155` gates the probes *off* `wasm32` on the grounds that "a dev-dependency that
only exists to run four assertions is a dev-dependency `cargo deny` has to clear on every run".
That argument stops holding the moment `wasm-bindgen-test` is present anyway for the conformance
target this slice's consumers need. Mirror `crates/happenstance-testkit/Cargo.toml:53-54`'s
`[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` block — the attribute resolves in the
caller's crate, never in the testkit.

**5. Four modelled properties are the acceptance test for "is this the same object".**
`src/sql_storage.rs:1-24` names them with a citation each and every one must stay true of the
replacement: (i) `exec` is **synchronous** — no future, no connection acquisition, which is the
whole reason `EventStore::read` being non-`async` is free rather than awkward here; (ii) the cursor
is **not a snapshot** across an `await`; (iii) everything is `!Send` and `!Sync`; (iv) the object is
single-threaded but **re-entrant**, which is why `SqlError::AlreadyBorrowed` is a reported error
rather than a `borrow_mut` panic. Losing any of them silently converts this crate back into an
instrument for a runtime nobody ships.

**6. Keep the thrown value live.** `JsThrow` (a handle) versus `StringifiedThrow` (a `String`) is
recorded as latitude (`_decomposition.md`, Architecture §6) with a default that is not neutral:
stringifying makes `SqlError`, `CloudflareEventStoreError` and every future over them
`Send + Sync` and destroys the only type in the workspace that can fail a `Send + Sync` bound on
`EventStore::Error` (`src/sql_storage.rs:83-92`). What stringifying actually costs a caller is
*forward* compatibility, not the conflict signal — the conflict signal never travels in
`Self::Error` on any adapter, because `happenstance-core` lifts it into
`AppendError::ConditionViolated` first (`src/lib.rs:63-85`). Keep `JsThrow` live; keep
`StringifiedThrow` in the tree as the recorded alternative and as the probes' positive control. A
deviation is argued in ADR-0023, not in a commit message.

**7. Price the dependency at merge, not on the Tuesday cron.** Two measurements are deliverables
of this story, each recorded rather than discovered later: `worker`'s own declared `rust-version`
against the 1.97.1 floor — ADR-0029's whole finding is that a dependency's build script moved the
floor and that `cargo hack --rust-version` could not see it coming because five crates declare no
`rust-version` at all, so **only running the compiler finds it** — and `cargo deny`'s widened
licence/advisory graph, which `Cargo.toml:19-25` pre-registered as the cost of this exact swap
(`_decomposition.md`, Deployment §4). If the floor has to move, that is an ADR amending ADR-0029,
never a silent bump (`CLAUDE.md`, binding constraint 5). If a licence outside `deny.toml:10-19`'s
allowlist appears, that is a finding to escalate — widening the allowlist is a decision and is
outside this PR's boundary.

**8. Three standing detectors must not go quiet** (`_decomposition.md`, Architecture §7;
`_storymap.md`, **Standing detectors**). The four `!Send` probes including the positive control
(`src/lib.rs:180-259`); `send_shape::send_flavour::SendStoreWithLocalError` still compiling
(`src/lib.rs:87-94`), which ES-6's own `Rejects:` line names as the implementation the rule must
reject; and — untouched by this story but broken by an `async fn read` refactor — the two
`read`-shape tests in `crates/happenstance-core/src/memory.rs` (`CLAUDE.md`, constraint 3). The two
doctests in `src/send_shape.rs:62-67` and `:75-80` are part of that set: one compiles, one is
`compile_fail,E0277`, and `cargo test --doc` re-proves both on every run.

**9. The persona slice.** There is no screen; the person served here is the **adapter author**
mid-loop (`_storymap.md`, **Backbone**, activity B's prerequisite). What they get from this PR is a
crate that no longer lies about what it is bound to, a `cargo test -p happenstance-cloudflare` that
still answers the `!Send` question in their inner loop, and a `cargo xtask ci` that is green with
`worker` in the graph. What they must not get is a green gate over a crate whose target-side
`Send`-ness nothing checked.

## Integration contract

- **Slice / milestone**: `real-worker-bindings`. Slice-mates, implemented in one context and
  mounted as one integrated surface: `durable-object-write-path`, `durable-object-read-path`,
  `caller-visible-error-verdict`. This story merges first within the slice; all three
  `depends_on` it.
- **Archetype**: `foundation` — real in-tree substrate consumed by the three capability stories
  above, never a double and never a fixme.
- **Mount point**: `crates/happenstance-cloudflare/src/event_store.rs` —
  `CloudflareEventStore::new(sql)` at `:70-87`, the crate's single construction root
  (`_decomposition.md`, Architecture §4a). The store takes its storage handle by **injection** and
  must never construct one itself, because in production the handle comes off
  `State::storage().sql()` inside a Durable Object class and in the fixture it comes off whatever
  the harness has. This story changes the *type* that flows through that constructor from the
  stand-in to the real binding and leaves its shape alone, so the fixture, the host and every
  future test reach the store through one path rather than two.
- **Wires into**:
  - `crates/happenstance-cloudflare/src/lib.rs` — the crate root: module declarations, the public
    re-exports at `:128-135`, the four findings, and the `not_send_probe` module at `:137-259`.
  - `crates/happenstance-cloudflare/src/send_shape.rs` — the ES-6 compile probes and their two
    doctests, which must keep compiling against the re-typed `CloudflareEventStoreError`.
  - `crates/happenstance-core` — the ports and value types the adapter binds:
    `EventStore` (bare flavour, never `SendEventStore`), `AppendError`, `SequencePosition`,
    `SequencedEvent`, `Query`, `ReadOptions`, `EventId`.
  - `Cargo.toml` `[workspace.dependencies]` (`:18-101`) — the single source of truth for every
    third-party version. `worker` is declared there and the member writes `worker.workspace = true`.
  - `xtask/src/main.rs:245-264` — the `wasm32 build of the Cloudflare adapter` step, selected by
    **name** in `wasm_steps()` (`:783-791`). Its name is stable; only its args and comment change.
  - `crates/happenstance-testkit/Cargo.toml:53-54` — the target-scoped dev-dependency block whose
    shape this crate's new one mirrors.
- **Renders surfaces**: **none.** The project's `_design.md` records `N/A — no user-facing
  surface`, signed off 2026-08-12 with the no-surface determination itself as what was approved.
  This story adds no `## Items` row because there is no `## Items` block to add one to.
- **Conformance rule(s)**: none added, none changed. This story is not adapter-observable through
  the suite — no `EventStore` body it could exercise exists yet, and "`!Send` on one target only"
  is not a property the DCB specification states, which is exactly why the assertion lives in this
  crate's own tree rather than in `crates/happenstance-testkit/tests/mutation_coverage.rs`
  (`discover.md`, **Where the detector must live**). The suite arrives with
  `every-rule-under-workerd`.
- **Clause(s)**: **ES-6 `[FROZEN]`** — confirmed against a real `worker::Error`, not amended
  (`spec/SPECIFICATION.md:2629-2686`). Its prose cites `crates/happenstance-cloudflare/src/js.rs:29-39`
  and `:45-58` by line; those citations must still point at the live `Rc`-shaped payload after the
  rewrite, or the clause reads as backed by code that moved. No other clause is discharged or
  amended here.
- **Advances DoD scenario**: initiative **DoD 4** — *"The constrained-runtime store passes the
  suite on its own target… and its error type is shown either to carry what the caller needs or
  demonstrably not to"* (`initiative.md`, **Definition of Done** 4). This story moves it from
  *unreachable* to *reachable*: DoD 4 cannot be attempted against a crate that does not depend on
  `worker`, and its second clause is a claim about a real `worker::Error` that no stand-in can
  make.

## PR boundary

```
crates/happenstance-cloudflare/**
xtask/src/main.rs
Cargo.toml
Cargo.lock
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/worker-binding-layer/**
```

**In this PR.** The `worker` dependency and its feature set, declared once in
`[workspace.dependencies]`; the real bindings in `src/js.rs` and `src/sql_storage.rs`; the
re-typing that flows from them through `src/event_store.rs`'s type definitions and
`src/send_shape.rs`'s probes; the `wasm32` twin of the `!Send` probe module and the
`wasm-bindgen-test` target-scoped dev-dependency it needs; `--tests` on the existing gate step so
the twin is compiled by the gate on merge day; the replacement manifest note; the crate-level
documentation update; and the two recorded measurements (MSRV, `cargo deny`).

**Explicitly not in this PR.**

- Any `EventStore` body — `migrate`, `append`, `head`, `contains_event_id`, `render_read`,
  `decode_row` stay `todo!()` and the scoped `#![allow(clippy::todo)]` at `src/lib.rs:126` stays
  with them. → `durable-object-write-path`, `durable-object-read-path`.
- ADR-0011's ceiling-and-page read mechanism, and the fate of `check_cursor_still_valid`
  (`src/event_store.rs:292-306`). → `durable-object-read-path`.
- The identity columns `origin_store` / `origin_position` and the schema doc comment at
  `src/event_store.rs:8-28`. → `durable-object-write-path`.
- The caller-visible error reconstruction test. → `caller-visible-error-verdict`.
- The Durable Object host, `impl Fixture for CloudflareFixture`, the conformance target, the
  `workerd` execution `Step` and the fourth `xtask/src/proof.rs` `Artefact` row. →
  `durable-object-host-and-fixture`, `every-rule-under-workerd`, `wasm-execution-gate-step`.
- The fixture's measured ceilings and `REOPEN` / `MID_BATCH_FAULT`. → `measured-store-limits`.
- Any write under `.kb/` — ADR-0023, CF-40's ownership, WF-11's atom. →
  `adr-0023-and-atom-resolutions`, via `/redkiln:kb-ingest`, never hand-written.
- `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`, and removing `publish = false`. →
  `publish-ready-crate`.
- `deny.toml` and `rust-toolchain.toml` are **deliberately outside the boundary**. A new licence
  in the graph or a floor that has to move is a finding recorded in the ledger and escalated, not
  absorbed by an allowlist edit or a silent bump.
- `.github/workflows/ci.yml` — the standalone `wasm-conformance` job's fate is
  `wasm-execution-gate-step`'s.

The implementer **may** touch the wiring files named in the Integration contract — `Cargo.toml`,
`Cargo.lock` and `xtask/src/main.rs:245-264` — to mount this slice. That is the mount, not scope
drift.

**Merge DoD.** `cargo xtask ci --fast` is green on a tree where `happenstance-cloudflare` depends
on `worker`; `cargo test -p happenstance-cloudflare` still reports four passing `!Send` assertions
including the positive control; the `wasm32` step compiles their target-side twin; and the MSRV and
`cargo deny` findings are written down in this story's ledger rather than remembered.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `worker` enters the graph exactly once | Declared in `[workspace.dependencies]` with an explicit feature set and a comment giving the reason, the way every other third-party version in this workspace is; the member writes `worker.workspace = true`. Two versions of one crate is the foot-gun the block exists to prevent. | `Cargo.toml:16-24`, `:18-101`; `crates/happenstance-cloudflare/Cargo.toml:14-17` |
| The host/target split is decided by **measurement**, and never by splitting the type | Compile it and find out: `worker`'s bindings are `wasm-bindgen` externs whose *linking*, not type-checking, target-gates in practice. If `cargo test -p happenstance-cloudflare` still links, the host probe module stays where it is (the recommended default). If it does not, the module moves behind `#[cfg(all(test, target_arch = "wasm32"))]` and re-emits through `wasm_bindgen_test`. What is forbidden either way is a `cfg` that gives the host a `Send`-safe stand-in error and the target a real one — that is the named mutant, and it passes every check in the gate. | `_decomposition.md`, Testing §2; `discover.md`, **The wrong implementation**; `crates/happenstance-cloudflare/src/lib.rs:149-163` |
| The four assertions keep passing somewhere an ordinary `cargo test` reaches | Non-negotiable regardless of how the split falls. A probe only a `workerd` run exercises is a probe no contributor's inner loop ever runs. | `_decomposition.md`, Architecture §7.1, Testing §2 |
| A `wasm32` twin of all four assertions is added | `#[cfg(all(test, target_arch = "wasm32"))]`, emitted through `#[wasm_bindgen_test]`, carrying `the_probe_is_not_vacuous` with it. The autoref-specialisation `Probe<T>` mechanism is reused verbatim — it is the only way to observe the *absence* of an auto trait on stable. | `crates/happenstance-cloudflare/src/lib.rs:137-178`, `:180-259`; `crates/happenstance-testkit/Cargo.toml:53-54` |
| The gate compiles the twin | `xtask/src/main.rs:245-264`'s step gains `--tests` (and `--tests`, not `--all-targets`, for the reason the sibling step at `:228-231` already gives). The step's **name** is unchanged because `wasm_steps()` selects by name and an index-selected step once pointed at clippy while printing green. Compiling is not executing; execution arrives with `wasm-execution-gate-step`. | `xtask/src/main.rs:219-244`, `:245-264`, `:769-791` |
| `SqlStorage::exec` stays synchronous | No future, no connection acquisition. This is the property that makes non-`async` `read` free here rather than awkward, and it is what lets an append evaluate its condition and insert atomically with nothing awaited in between. | `crates/happenstance-cloudflare/src/sql_storage.rs:1-12`, `:165-178`; `_decomposition.md`, Architecture §3 |
| `SqlCursor` binds to `SqlStorageCursor` and stays not-a-snapshot | The property is preserved, not fixed. The fix — ADR-0011's ceiling-and-page — and the fate of `check_cursor_still_valid` belong to `durable-object-read-path`. This story must not quietly buffer the result set to make the problem go away. | `crates/happenstance-cloudflare/src/sql_storage.rs:13-17`, `:193-275`; `.kb/decisions/0011-read-laziness-and-isolation.md` |
| `SqlError::AlreadyBorrowed` remains a reported error | A Durable Object is single-threaded but re-entrant: two `append` futures from one handle, polled alternately, both hold `&self`. Reporting beats the panic `borrow_mut` would give. | `crates/happenstance-cloudflare/src/sql_storage.rs:18-23`, `:101-109` |
| `SqlError::StorageLimitExceeded` remains, bound to the object's real cap | It is the input to the fixture's `MAX_*` constants; measuring those is `measured-store-limits`'. | `crates/happenstance-cloudflare/src/sql_storage.rs:119-121`; `_decomposition.md`, Architecture §3 |
| `SqlValue`'s variant set is the marshalling contract | `Null`, `Integer(i64)`, `Real`, `Text`, `Blob` — exactly what Workers SQL accepts and returns; no boolean, no date. The `Integer` doc comment's 2^53 note stays: Workers SQL widens integers through a JS number. Payloads stay opaque `Bytes` → `Blob`; this crate never inspects one (ADR-0003). | `crates/happenstance-cloudflare/src/sql_storage.rs:30-49`; `.kb/decisions/0003-opaque-payloads.md` |
| `JsHandle` keeps an `Rc`-shaped payload | This is the entire ES-6 instrument. A real `JsValue` is `Send + Sync` on non-`atomics` `wasm32`, and `unsafe_code = "forbid"` means an adapter can only ever *inherit* that hatch by holding one — so the payload the error reaches through must be `Rc`-backed, unconditionally. | `crates/happenstance-cloudflare/src/js.rs:13-39`, `:43-58`; `Cargo.toml:101-102`; `spec/SPECIFICATION.md:2647-2658` |
| `JsHandle::property` and `JsThrow::is_constraint_violation` become real | The two `todo!()`s this story is responsible for on the JS side: `Reflect::get` through `js-sys`, and a probe of `.message` / `.code` on the live value. The finding they exist to keep true — that a Durable Object surfaces SQLite's own `UNIQUE constraint failed: …` text through the thrown `Error`'s `message` and exposes no numeric code — is now checkable against the real API rather than asserted. | `crates/happenstance-cloudflare/src/js.rs:79-92`, `:113-127`, `:152-162` |
| `JsThrow` is kept live; `StringifiedThrow` is kept as the recorded alternative | Stringifying would make the error `Send + Sync` and delete the workspace's only instrument that can fail the bound. `StringifiedThrow` also earns its keep as `the_probe_is_not_vacuous`'s positive control. | `crates/happenstance-cloudflare/src/sql_storage.rs:83-92`; `crates/happenstance-cloudflare/src/lib.rs:63-73`, `:210-220` |
| `CloudflareEventStoreError` keeps its shape, and its absence | Still `!Send` and `!Sync` transitively; still **no** `ConditionViolated` variant, because the DCB conflict signal is lifted into `AppendError::ConditionViolated` by the contract before `Self::Error` is constructed. The absence is structural, not an oversight, and re-typing the `Sql` variant must not restore it. | `crates/happenstance-cloudflare/src/event_store.rs:89-143`; `_decomposition.md`, Architecture ARCH-AC-07 |
| `CloudflareEventStore::new(sql)` keeps its shape | Injection, not construction. `Default` (`:64-68`) is re-examined here rather than assumed: a store that can mint its own storage from nothing is a second path the fixture's "one instance, one object" invariant does not cover. Whether it survives the swap, and on what terms, is decided in this PR. | `crates/happenstance-cloudflare/src/event_store.rs:64-87`; `_decomposition.md`, Architecture §4a, Testing §3 |
| `send_shape` stays structurally untouched and keeps compiling | `SendStoreWithLocalError` implements `SendEventStore` — `Send` store, `Send` stream, `Send` future — over a `!Send` `Error`, and ES-6's `Rejects:` line names it. Its two doctests (one passing, one `compile_fail,E0277`) are re-proved by `cargo test --doc` on every run and must still hold against the re-typed error. Its `todo!()` probe bodies are *not* adapter paths; their disposition travels with the scoped allow, in whichever story removes it. | `crates/happenstance-cloudflare/src/send_shape.rs:46-86`, `:88-170`; `spec/SPECIFICATION.md:2680-2686` |
| `worker`'s MSRV is measured against the 1.97.1 floor, at merge | Read its declared `rust-version`; if it declares none, the compiler is the only instrument that can answer, which is ADR-0029's whole finding. A floor that has to move is a new atom amending ADR-0029, authored through the runbook's ADR queue. | `.kb/decisions/0029-msrv-raised-to-1-97-1.md:44-46`, `:63-66`; `CLAUDE.md`, binding constraint 5 |
| `cargo deny` is run against the widened graph, at merge | Licences against `deny.toml:8-20`'s allowlist and advisories against the new `wasm-bindgen` / `js-sys` / `web-sys` / `worker-sys` surface. `wasm-bindgen` and `js-sys` are already in `Cargo.lock` as `wasm-bindgen-test`'s transitive deps, so the delta is smaller than the manifest note feared — measure it rather than assume either way. | `deny.toml:1-28`; `Cargo.lock`; `_decomposition.md`, Deployment §4 |
| The manifest's no-`worker` note is replaced, not deleted | It records why the stand-in existed and what it bought; the replacement records what the swap cost and what it bought instead. Deleting it deletes the reasoning a reviewer checks the swap against. | `crates/happenstance-cloudflare/Cargo.toml:19-25`; `_decomposition.md`, Architecture §1 |
| The crate's own documentation stops being false | `src/lib.rs:1-10`'s "Status: not implemented / every body is `todo!()` / phase 9 swaps [`sql_storage`] for the real `worker` bindings" and `src/event_store.rs:3-6`'s echo are half-false the moment this merges. They are rewritten to say precisely what is bound and what is still `todo!()`, and the four findings keep pointing at live code — including the `js.rs` line ranges `spec/SPECIFICATION.md`'s ES-6 clause cites by number. | `crates/happenstance-cloudflare/src/lib.rs:1-30`; `crates/happenstance-cloudflare/src/event_store.rs:1-28`; `spec/SPECIFICATION.md:2645`, `:2652` |

## Data and migrations

**N/A — no schema is created, altered or read by this story.** `migrate()` stays `todo!()`
(`crates/happenstance-cloudflare/src/event_store.rs:79-86`), no statement is executed, and no
persisted data exists to migrate: the crate has never talked to a Durable Object, so there is no
deployed shape to be compatible with.

Two data-shaped facts are nonetheless **bound** here rather than deferred, because they are
properties of the marshalling layer this story writes:

1. **The value domain.** `SqlValue`'s five variants are the whole contract between the adapter and
   Workers SQL — no boolean, no date — which is why the intended schema stores positions as
   integers and payloads as blobs (`crates/happenstance-cloudflare/src/sql_storage.rs:30-49`,
   `src/event_store.rs:8-28`). Binding to the real API must not widen or narrow that set silently.
2. **The 2^53 integer ceiling.** Workers SQL widens integers through a JS number on the way out, so
   a `SequencePosition` above `Number.MAX_SAFE_INTEGER` is not round-trippable even though
   `NonZeroU64` permits it. This story preserves the fact and the variant that will report it
   (`CloudflareEventStoreError::StoredPosition`,
   `crates/happenstance-cloudflare/src/event_store.rs:135-142`); *surfacing* it — as a declared
   store limit rather than by truncation — is `durable-object-write-path`'s under project AC-007(b).

The schema itself, including ADR-0014's `origin_store` / `origin_position` identity columns that
the doc comment at `src/event_store.rs:8-28` does not yet carry, belongs to
`durable-object-write-path` (`_decomposition.md`, Architecture §2, ADR-0014).

## Acceptance criteria

There is no screen here, so "persona goal crossing the full stack" is stated against the three
people `_storymap.md`'s **Backbone** names as the ones who actually observe this project's output:
the **adapter author** mid-loop, the **gate reader** who must be able to believe what ran, and the
**library consumer** who will one day depend on the crate. The stack they cross is manifest →
bindings → error type → probe → gate step.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN an adapter author who reached for `happenstance-cloudflare` because they need an event store *inside* a Durable Object, WHEN they inspect the crate's dependency graph and build it for the target it claims, THEN `worker` is really in the graph — declared exactly once in `[workspace.dependencies]` with an explicit feature set and a stated reason, consumed by the member as `worker.workspace = true` — and the crate compiles for `wasm32-unknown-unknown`. | `cargo xtask wasm` → step `wasm32 build of the Cloudflare adapter` (`xtask/src/main.rs:245-264`); `cargo deny check bans` (one `worker`, one `wasm-bindgen`); manifests read at `Cargo.toml` `[workspace.dependencies]` and `crates/happenstance-cloudflare/Cargo.toml:14-17` |
| AC-002 | GIVEN that same author reading the crate to decide whether it does what it says, WHEN they grep it for unimplemented bodies, THEN the five *binding* `todo!()`s in `src/js.rs` and `src/sql_storage.rs` are gone and replaced by real calls; the six `EventStore` bodies in `src/event_store.rs` and the scoped `#![allow(clippy::todo)]` at `src/lib.rs:126` are still there and still honest; `CloudflareEventStore::new(sql)` still takes its handle by injection; and the public re-export list at `src/lib.rs:128-135` gains no new item. | `rg -n "todo!\(" crates/happenstance-cloudflare/src` returns exactly the six `event_store.rs` sites; `cargo clippy --workspace --all-targets --all-features -- -D warnings`; diff review of `src/lib.rs:128-135` against `HEAD` |
| AC-003 | GIVEN an author who chose this adapter *because* a Durable Object is a synchronous, single-threaded, re-entrant actor, WHEN they drive the bound `SqlStorage`, THEN all four modelled properties still hold of the real binding: `exec` is synchronous, the cursor is a live iterator and not a buffered snapshot, every type is `!Send` and `!Sync`, and re-entrancy is *reported* as `SqlError::AlreadyBorrowed` rather than panicking. | `exec` keeps a non-`async` signature returning `Result<SqlCursor, SqlError>`, proven by `src/event_store.rs`'s non-`async` `read` continuing to type-check (`cargo check -p happenstance-cloudflare`); a new `#[test]` `sql_storage::tests::reentrant_borrow_is_reported_not_panicked`; `!Send` by AC-004 and AC-005 |
| AC-004 | GIVEN a contributor mid-loop with no wasm toolchain installed at all, WHEN they run `cargo test -p happenstance-cloudflare`, THEN the four `!Send` assertions still run and pass on the host — `the_probe_is_not_vacuous` included — over the **same** error type the target compiles, with no `cfg` handing the host a `Send`-safe stand-in and the target a real one. | `cargo test -p happenstance-cloudflare` → `tests::the_probe_is_not_vacuous`, `tests::the_js_boundary_types_are_not_send`, `tests::the_error_type_is_not_send`, `tests::the_send_flavour_does_not_imply_a_send_error` (`src/lib.rs:180-259`); `rg -n "cfg\(.*target_arch" crates/happenstance-cloudflare/src` reviewed against the named mutant |
| AC-005 | GIVEN a gate reader who must believe the `!Send` claim on the one platform this crate exists for, WHEN the crate's tests are compiled for `wasm32-unknown-unknown`, THEN a target-side twin of all four assertions exists in this crate's own tree, carries the positive control with it, and is emitted through `wasm_bindgen_test` from a target-scoped dev-dependency block shaped like `crates/happenstance-testkit/Cargo.toml:53-54`. | `cargo check --locked -p happenstance-cloudflare --tests --target wasm32-unknown-unknown` compiles it (AC-006's amended step); where a `wasm-bindgen-test-runner` resolves locally, it is *executed* once and the result recorded in this story's ledger; standing execution inside the gate is `wasm-execution-gate-step`'s |
| AC-006 | GIVEN a gate reader on a clean checkout, WHEN they run `cargo xtask ci --fast`, THEN the `wasm32 build of the Cloudflare adapter` step compiles the crate's **test** targets (`--tests`, not `--all-targets`), keeps its name so `wasm_steps()` still selects it by name rather than index, and carries a comment saying why — so the twin added by AC-005 is built by the gate on the day it merges instead of a fortnight later. | `cargo xtask ci --fast`; `cargo xtask wasm`; a rename would panic in `steps_named` (`xtask/src/main.rs:769-791`), which is the intended failure |
| AC-007 | GIVEN a caller who must branch on constraint violation versus transport fault, WHEN they hold a `CloudflareEventStoreError` built from a real thrown value, THEN the thrown value is still **live** — `JsThrow`, not stringified — `JsThrow::is_constraint_violation` is real rather than `todo!()`, `StringifiedThrow` remains in the tree as the recorded alternative *and* as the probe's positive control, and the error type still has no `ConditionViolated` variant. | `cargo test -p happenstance-cloudflare` → `tests::the_error_type_is_not_send` and `tests::the_probe_is_not_vacuous` (which asserts `StringifiedThrow: Send`); a new `#[test]` over `JsThrow::is_constraint_violation` against a thrown value carrying `UNIQUE constraint failed: event.position`; the caller-visible reconstruction artefact itself is `caller-visible-error-verdict`'s |
| AC-008 | GIVEN a library consumer pinned to the 1.97.1 floor and a gate reader who owns the licence allowlist, WHEN `worker` enters the graph, THEN both prices are **measured in this PR and written into this story's ledger** — `worker`'s declared `rust-version`, or the compiler's answer if it declares none, against ADR-0029's floor, and `cargo deny`'s licence/advisory/bans verdict against `deny.toml:10-19`'s eight allowed licences — and neither `deny.toml` nor `rust-toolchain.toml` is edited to make either of them pass. | `cargo hack check -p happenstance-cloudflare --no-dev-deps --rust-version`; `cargo +1.97.1 test -p happenstance-cloudflare`; `cargo deny check licenses advisories bans`; `git diff --stat` shows `deny.toml` and `rust-toolchain.toml` untouched; ledger evidence rows on this AC |
| AC-009 | GIVEN a reviewer checking that the crate has stopped describing itself falsely, WHEN they read the manifest note, the crate documentation and ES-6's citations, THEN the deliberate no-`worker` note at `crates/happenstance-cloudflare/Cargo.toml:19-25` has been **replaced by the record of its reversal** rather than deleted, `src/lib.rs:1-30` and `src/event_store.rs:1-6` say precisely what is bound and what is still `todo!()`, and `spec/SPECIFICATION.md`'s ES-6 citations into `src/js.rs` still land on the live `Rc`-shaped payload. | `cargo xtask spec-trace`; `cargo doc --workspace --all-features` under the workspace's denied `missing_docs`; diff review that the manifest note is replaced, not removed |

**Coverage of the traced project ACs.** Project **AC-001** (*the adapter is real*) is served by
AC-001, AC-002, AC-003, AC-006 and AC-009 — this story owns the *bindings* half; the bodies and the
removal of the scoped allow are `durable-object-write-path`'s and `durable-object-read-path`'s
(`_storymap.md`, **Coverage**, AC-001 row). Project **AC-005** (*ES-6 decided with an artefact*) is
served by AC-004, AC-005 and AC-007 — this story owns *the error type stays genuinely `!Send`*; the
committed reconstruction test is `caller-visible-error-verdict`'s and the atom is
`adr-0023-and-atom-resolutions`'. AC-008 serves neither project AC directly and is not decoration:
it is the merge-day price of the dependency both of them require, and the failure mode it guards is
the one ADR-0029 was written about.

## Interaction quality

This story renders **no surface**. The project's signed-off `_design.md` records
`N/A — no user-facing surface`, and what was approved *is* that determination — so there is no
composition, transience policy or hierarchy to implement, and inventing one here would contradict a
human sign-off rather than honour it. Accordingly the COMPOSITION family below is taken from the
repository's own substitution for it, which is not a softening: `.redkiln/templates/spec.md`'s
**Surface quality** block replaces the bundled screen-shaped list with API-surface invariants,
under the standing instruction to **strike a bullet that cannot fail here rather than tick it**.

Every invariant that applies is carried by an `AC-###` row in the table above. Nothing below is a
new obligation; this section says which id carries which invariant and how it is observed.

**STATE family, translated to this story's medium** — the adapter author's inner loop and the
reviewer's diff are the "state" that must survive the change.

| Invariant | Carried by | How it is observed |
| --- | --- | --- |
| **In place, not a context jump** — the `!Send` question keeps being answerable where it is asked today, `cargo test -p happenstance-cloudflare`; it does not relocate into a `workerd`-only run a contributor cannot reach. | AC-004 | The four named tests run under a plain `cargo test`, with no `--target` and no runner |
| **Non-occlusion** — the swap must not hide what is still unbuilt behind a crate that now compiles against a real API. The six `EventStore` `todo!()`s and the scoped allow stay visible; the documentation stops overstating in the other direction too. | AC-002, AC-009 | `rg -n "todo!\("` returns exactly six sites; `cargo xtask spec-trace`; doc review |
| **Preserved selection** — every existing path into the store survives: one constructor by injection, one public re-export list, one error shape including its deliberate absence. | AC-002, AC-007 | Diff review against `HEAD` of `src/lib.rs:128-135` and `src/event_store.rs:89-143`; `send_shape`'s two doctests still compile |
| **Reversibility** — the trade this PR reverses stays re-litigable: the reasoning for the previous state is replaced by the reasoning for the new one, never deleted. | AC-009 | The manifest note at `Cargo.toml:19-25` is present, rewritten |
| ~~Keyboard reachability, focus and scroll~~ | — | **Struck.** No surface exists that could fail them; ticking them would be the decorative-check shape `xtask/src/main.rs`'s own module documentation warns about |

**COMPOSITION family, as the repository substitutes it** — is the thing a caller meets the thing
that was designed?

| Invariant | Carried by | How it is observed |
| --- | --- | --- |
| **Presentation exists at all** — every public item the swap re-types carries real rustdoc, not a bare signature; the crate's four findings still point at live code. | AC-009 | `missing_docs` is denied workspace-wide, so `cargo doc --workspace --all-features` is the check; `cargo xtask spec-trace` for the citations |
| **Both flavours** — anything touching a port type-checks on the bare *and* the `Send` flavour (ADR-0001). `send_shape::send_flavour::SendStoreWithLocalError` is the compiled proof and ES-6's `Rejects:` line names it. | AC-004 | `cargo check -p happenstance-cloudflare`; the crate stops compiling if it breaks, which is the point |
| **The example compiles** — `src/send_shape.rs:62-67` and `:75-80`, one passing and one `compile_fail,E0277`, are re-proved against the re-typed error. | AC-004 | `cargo test -p happenstance-cloudflare --doc` |
| **Errors are documented** — every re-typed fallible public function keeps an `# Errors` section naming the *conditions*, not the error type. | AC-003, AC-007 | `cargo doc` review of `SqlStorage::exec`, `SqlCursor::next_row`, `JsHandle::property`, `JsThrow::is_constraint_violation` |
| **Visibility is as signed off** — `_design.md` declares no `## Items` block, so the honest reading is that this story promises **no new public item**; anything that became `pub` during the swap is a semver promise nobody made. | AC-002 | Diff review of `src/lib.rs:128-135` and of every `pub` added under `crates/happenstance-cloudflare/src/` |
| **Named anti-patterns** — the cfg-split error type (`discover.md`, **The wrong implementation**); stringifying the thrown value by default; `#[async_trait]` anywhere (ADR-0001); an `async fn read` (CLAUDE.md constraint 3); binding `SendEventStore` in generic code (constraint 4). | AC-004, AC-005, AC-007 | The `wasm32` twin is what makes the first one fail; the rest are compile-time or review checks |
| ~~Suite invariants (rule can fail, no literal positions, no clock, changelog entry)~~ | — | **Struck.** This story adds and changes no conformance rule (**Integration contract**, *Conformance rule(s)*). They bind `every-rule-under-workerd` and `measured-store-limits` |

**Density budget, in the real numbers this story moves.** Five `todo!()`s removed and six
deliberately left; four `!Send` assertions become eight (four host, four target); one new
target-scoped dev-dependency (`wasm-bindgen-test`, already in `Cargo.lock` for the testkit); one new
direct dependency (`worker`) declared in one place; the licence allowlist stays at
`deny.toml:10-19`'s **eight** entries; the MSRV floor stays at **1.97.1**; the gate gains **zero**
new steps and exactly one new flag. Any number in that list moving in the diff without a line in
the ledger explaining it is the signal that this PR grew past its boundary.

## Error conditions

These are conditions on the *work*, not runtime error variants — each is a way this story can
discover that its premise is wrong, and each has a decided response so the response is not improvised
at 5pm.

- **EC-001 — `worker` forces the MSRV floor above 1.97.1.** Detected by
  `cargo hack check -p happenstance-cloudflare --no-dev-deps --rust-version` or, if it declares no
  `rust-version` at all, by `cargo +1.97.1 test -p happenstance-cloudflare` — which is ADR-0029's
  entire finding, that only running the compiler finds this. **Response:** record the measurement in
  the ledger and **stop**. Raising the floor is a new decision atom amending ADR-0029, authored
  through the runbook's ADR queue; `rust-toolchain.toml` is outside this PR's boundary and a silent
  bump is forbidden (`CLAUDE.md`, binding constraint 5).
- **EC-002 — a licence outside `deny.toml:10-19`'s allowlist appears in the widened graph.**
  **Response:** record it, escalate it, do not widen the allowlist. Widening is a decision with a
  blast radius past this crate, and `deny.toml` is deliberately outside the boundary (**PR
  boundary**).
- **EC-003 — an advisory or a yanked crate lands in the new surface.** `yanked = "deny"`
  (`deny.toml:6`). **Response:** prefer a version bump inside `[workspace.dependencies]`; if none
  exists, record and escalate rather than adding an ignore.
- **EC-004 — `cargo test -p happenstance-cloudflare` stops linking once `worker` lands.** The
  testing brief flags this as genuinely open (`_decomposition.md`, Testing §2). **Response:** move
  the probe module and its four assertions behind `#[cfg(all(test, target_arch = "wasm32"))]` and
  emit them through `wasm_bindgen_test`, with the module's own doc comment rewritten to say the old
  argument inverted and why. **Never** the cfg-split error type. If that move happens, AC-004's
  "reachable by a plain `cargo test`" is not silently dropped — it is escalated, because
  Architecture §7.1 states it as a requirement that does not move.
- **EC-005 — holding a real `worker::Error` makes `CloudflareEventStoreError` `Send`.** Detected by
  the target-side twin, which is exactly why AC-005 exists. **Response:** keep the payload
  `Rc`-shaped so the leak cannot arrive through a field type; if that turns out to be impossible,
  the outcome is a **new decision atom and a re-plan**, never an edit to
  `.kb/decisions/0009-error-send-sync.md` and never a quiet stringification.
- **EC-006 — the real `SqlStorage::exec` is not synchronous, or the real cursor cannot be advanced
  without buffering.** Either would falsify a modelled property this crate's whole design rests on.
  **Response:** stop and escalate. Do not buffer the result set to make the problem disappear —
  that converts a read-path decision (ADR-0011, `durable-object-read-path`'s) into an accident of
  this PR.
- **EC-007 — `cargo deny check bans` reports two versions of `wasm-bindgen`.** `multiple-versions`
  is `warn`, so this will not fail the gate on its own. **Response:** record the duplicate in the
  ledger anyway. A silent duplicate is how the `[workspace.dependencies]` block stops being a single
  source of truth.

## Non-functional

- **NF-001 — the dependency is taken at minimum width.** `worker` is declared with an explicit
  feature list and default features considered rather than inherited, because every feature is
  licence surface, advisory surface and compile time for a crate three other stories are waiting on.
  The reason goes in the manifest comment (AC-009), not in the commit message.
- **NF-002 — no `unsafe`, still.** The workspace sets `unsafe_code = "forbid"`. That is the whole
  reason the `JsValue` auto-trait hatch can only be *inherited* rather than written, and this story
  must not become the first exception.
- **NF-003 — the gate does not get slower than the fact it buys.** `--tests` on one `cargo check`
  for one small crate is the cheapest possible way to compile the twin; if it measurably moves
  `cargo xtask ci --fast`'s wall clock, record the number rather than discovering it as a complaint.
- **NF-004 — the inner loop stays runnable without a wasm toolchain.** A contributor with a stock
  stable toolchain must still be able to run `cargo test -p happenstance-cloudflare` and
  `cargo clippy` to completion. Whatever the gate gains, it must not become a precondition for
  touching this crate at all — `wasm-bindgen-test` is target-scoped precisely so it is not resolved
  on a host build.
- **NF-005 — determinism.** Nothing added here reads a clock, a network or an environment variable.
  The probes are compile-time facts reported as `bool`s, and the two measurements are recorded
  numbers rather than assertions that re-run.

## Implementation notes (non-prescriptive)

These are latitude, not instruction. Where they conflict with an AC, the AC wins.

- **Measure before you split.** The first thing worth doing is adding `worker` and running
  `cargo test -p happenstance-cloudflare` to see whether it links. Everything about the probe
  placement follows from that one observation (EC-004), and guessing it costs a rewrite.
- **The mapping is item-for-item.** `_decomposition.md`, Architecture §3 already lays out the
  stand-in → real binding map; the shapes to bind are `state.storage().sql()` for the handle,
  `exec` / `databaseSize` on the storage, and `next` / `columnNames` / `rowsRead` on the cursor.
  `js-sys`'s `Reflect::get` is what `JsHandle::property` becomes.
- **`Default` is a decision, not an oversight.** `SqlStorage::new()` at `src/sql_storage.rs:155-163`
  mints empty storage out of nothing, and `impl Default` at `:149-153` and
  `src/event_store.rs:64-68` follow from it. A real binding cannot be conjured without a live
  Durable Object, so the expected outcome is that both disappear — see **Clarifications**. If either
  survives, the ledger says on what terms.
- **`SqlCursor::source_len` (`src/sql_storage.rs:264-269`) exists so the stand-in's `state` field is
  read somewhere.** When `state` goes, it goes with it — but the `try_borrow` shape it demonstrates
  is exactly what AC-003's re-entrancy test needs to keep observing on the real cursor.
- **Keep the ES-6 findings in the crate documentation as *findings*.** They were made against a
  stand-in; each one is now either confirmed against the real API or corrected in place with the
  correction stated. A finding quietly deleted is a finding that will be re-discovered.
- **The twin is a twin, not a variation.** Reuse the `Probe<T>` / `NotSend` autoref mechanism
  verbatim (`src/lib.rs:156-178`) and the same two macros; the value of a twin is that it fails for
  the same reasons.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`, Testing §1 (the tier mix) and §4 (the merge-gate commands), and in
`CLAUDE.md`, **Commands**.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static — format and lint | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` | AC-002 (the scoped allow still covers exactly the bodies that remain, and nothing else went red) |
| Static — grep | `rg -n "todo!\(" crates/happenstance-cloudflare/src` | AC-002 — exactly six sites, all in `event_store.rs`. This is project AC-001's own instrument (`_storymap.md`, DoD 3) |
| Static — target compile | `cargo check --locked -p happenstance-cloudflare --tests --target wasm32-unknown-unknown` (the amended step, `xtask/src/main.rs:245-264`) | AC-001, AC-005, AC-006 — the crate and its test targets compile for the platform it claims |
| Static — supply chain | `cargo deny check licenses advisories bans` (`deny.toml`) | AC-008, EC-002, EC-003, EC-007 |
| Static — MSRV | `cargo hack check -p happenstance-cloudflare --no-dev-deps --rust-version`; then `cargo +1.97.1 test -p happenstance-cloudflare`, because `--no-dev-deps` is the flag that hides exactly the dev-dependencies this story adds | AC-008, EC-001 |
| Static — citations | `cargo xtask spec-trace` | AC-009 — ES-6's line citations into `src/js.rs` still land on live code |
| Unit, host-native | `cargo test -p happenstance-cloudflare` → `tests::the_probe_is_not_vacuous`, `tests::the_js_boundary_types_are_not_send`, `tests::the_error_type_is_not_send`, `tests::the_send_flavour_does_not_imply_a_send_error` (`src/lib.rs:180-259`) | AC-004, AC-007 — and the positive control is what stops the whole module passing vacuously |
| Unit, host-native (new) | `cargo test -p happenstance-cloudflare` → `sql_storage::tests::reentrant_borrow_is_reported_not_panicked`; a `#[test]` over `JsThrow::is_constraint_violation` against `UNIQUE constraint failed: event.position` | AC-003, AC-007 |
| Unit, target-side (new) | the `#[cfg(all(test, target_arch = "wasm32"))]` twin in `crates/happenstance-cloudflare/src/lib.rs`, emitted through `#[wasm_bindgen_test]` | AC-005 — compiled by the gate here; *executed* in the gate by `wasm-execution-gate-step` + `every-rule-under-workerd`. Where a runner resolves locally it is run once and the result recorded in the ledger |
| Compile-time proof | `cargo test -p happenstance-cloudflare --doc` → the two doctests at `src/send_shape.rs:62-67` and `:75-80`, one of them `compile_fail,E0277` | AC-004 — a `Send` flavour still does not imply a `Send` error |
| Documentation | `cargo doc --workspace --all-features` (workspace `missing_docs` is denied) | AC-009 |
| Story grain | `cargo xtask affected --base main` | The `.redkiln/config.yaml` `verify:` wiring's story-grain bar — only what this diff could break |
| Project grain (merge gate) | `cargo xtask ci --fast` | The bar this non-terminal project is held to (`_decomposition.md`, Testing §4). The full `cargo xtask ci` belongs to the terminal project's `verify.e2e` grain |
| Conformance | **none.** No rule added, changed or newly exercised, so no `mutation_coverage.rs` row and no CF-29 changelog entry is owed | Stated so its absence is a decision rather than an omission (**Integration contract**) |

## Risks and coupling (PR-scoped)

| Risk | Blast radius | Early signal | Mitigation in this PR |
| --- | --- | --- | --- |
| **`worker` moves the MSRV floor** | The whole workspace and every future consumer — the floor is a promise from phase 12 onward | `cargo +1.97.1 test -p happenstance-cloudflare` fails on a dependency's build script, exactly as ADR-0029 records | Measured at merge (AC-008), escalated as an ADR (EC-001), never absorbed |
| **`Send`-ness restored by accident** | ES-6 loses its only instrument, and a `[FROZEN]` clause ends up citing code the target never runs | The target-side twin fails to compile or fails at runtime | AC-005's twin plus the unconditional `Rc`-shaped payload (AC-004, AC-007) |
| **The cfg-split mutant lands anyway**, because it is the path of least resistance under a link error | Silent: every existing check stays green (`discover.md`, **The wrong implementation**) | Nothing in today's gate signals it — which is the point | The twin (AC-005) and the review check in AC-004's verification |
| **`cargo deny`'s widened graph** | The gate goes red for a reason unrelated to this crate's design | `cargo deny check licenses` on the new lockfile | Run at merge (AC-008); `deny.toml` deliberately out of boundary |
| **Three stories are queued behind this one** | `durable-object-write-path`, `durable-object-read-path` and, transitively, `caller-visible-error-verdict` all block on it; `publish-ready-crate` does too | Slice-mates cannot start | Boundary kept deliberately narrow — no `EventStore` body, no fixture, no host — so this diff is small and reviewable |
| **The real API disagrees with a modelled property** | The crate's design premise, and ADR-0011's read-path plan behind it | `exec` needs an `await`, or the cursor needs buffering | EC-006: stop and escalate rather than absorbing a read-path decision into a binding PR |
| **`Default` removal ripples** | Any in-tree caller of `CloudflareEventStore::default()` | `cargo check` | Only in-tree callers exist (`publish = false`), so the ripple is bounded to this crate |
| **Coupling to `xtask`** | One shared file, `xtask/src/main.rs`, also touched by `wasm-execution-gate-step` in another milestone | A merge conflict on the `REQUIRED` array | This story changes only *args and comment* of an existing step and adds none, so the two diffs touch different regions |

## Dependencies

- **Blocks on:** *(none)* — `depends_on: []`. This story has no upstream inside the backlog; it
  depends only on the tree as it stands (`_storymap.md`, **Slices**, `worker-binding-layer` row).
- **Unlocks:**
  - `durable-object-write-path` — needs the real bindings to write `migrate`, `append`, `head`,
    `contains_event_id` against.
  - `durable-object-read-path` — same bindings, for ADR-0011's ceiling-and-page `read`.
  - `caller-visible-error-verdict` — needs `CloudflareEventStoreError` to carry a real
    `worker::Error` before it can reconstruct anything from one (it depends on
    `durable-object-write-path`, so the unlock is transitive).
  - `publish-ready-crate` — declares `worker-binding-layer` among its `depends_on`; a crate cannot
    be published-ready while its dependency graph is a stand-in.
- **Independent of** `wasm-execution-gate-step` and the whole `wasm-execution-seam` milestone: no
  edge is declared between them, and inventing one would be dishonest (`_storymap.md`, **Merge
  order** §2). This story *compiles* the target-side twin; nothing here needs the twin to be
  *executed*, which is what that milestone provides.

## Anchors (progressive disclosure)

Every path below exists in this worktree, checked before citation. Link, do not paste.

| Anchor (real path) | Why it is load-bearing | When to open | Serves AC |
| --- | --- | --- | --- |
| `crates/happenstance-cloudflare/src/lib.rs` | The crate root carries the four findings, the `wasm-bindgen` `unsafe impl` quoted verbatim at `:33-61`, the scoped allow at `:121-126`, the re-export list at `:128-135` and the whole probe module at `:137-259`. It is the file this story is mostly about, and the twin is written by reading `:156-178` and `:180-259` side by side | First, before touching anything else | AC-002, AC-004, AC-005, AC-009 |
| `crates/happenstance-cloudflare/src/sql_storage.rs` | `:1-24` names the four modelled properties with a citation each — the acceptance test for *is this the same object*; `:150-191` and `:193-269` are the bodies being replaced, including the `try_borrow` shape the re-entrancy test needs | Before implementing AC-003, and again before AC-002's `todo!()` sweep | AC-002, AC-003 |
| `crates/happenstance-cloudflare/src/js.rs` | The `Rc<str>` payload and *why it is stricter than the original* (`:13-39`); the two `todo!()`s this story owns (`:90-92`, `:125-127`); `StringifiedThrow`'s substring test at `:152-163` that `JsThrow` must now match through a property lookup. `spec/SPECIFICATION.md` cites this file by line | Before implementing AC-007; re-read at AC-009 to confirm the cited ranges still hold | AC-007, AC-009 |
| `crates/happenstance-cloudflare/src/event_store.rs` | The mount: `CloudflareEventStore::new(sql)` at `:70-87` and the `Default` at `:64-68` this story must decide about; the error enum at `:89-143` whose shape *and deliberate absence* must survive re-typing | Before AC-002, and before writing a single line that changes a type flowing through the constructor | AC-002, AC-007 |
| `crates/happenstance-cloudflare/src/send_shape.rs` | `SendStoreWithLocalError` is the standing negative control ES-6's `Rejects:` line names, and the two doctests at `:62-67` and `:75-80` (one `compile_fail,E0277`) are re-proved on every `cargo test --doc` | Whenever the error type's shape changes — if this file stops compiling, the swap is wrong even with a green gate | AC-004 |
| `crates/happenstance-cloudflare/Cargo.toml` | Lines `:19-25` are the note being *replaced rather than deleted*, and they state the exact cost this PR is now paying | At the start (to read the trade being reversed) and at the end (to write its replacement) | AC-001, AC-009 |
| `crates/happenstance-testkit/Cargo.toml` | `:50-54` is the target-scoped dev-dependency split to mirror, with the reason written out: the runtime attribute resolves in the caller's crate, never in the testkit | Before adding `wasm-bindgen-test` for AC-005 | AC-005 |
| `xtask/src/main.rs` | `:219-244` already explains why `--tests` and not `--all-targets`; `:245-264` is the step to amend; `:769-791` is why the name must not change (an index once pointed the wasm command at clippy while printing green) | Before amending the gate step | AC-006 |
| `deny.toml` | The eight allowed licences at `:10-19`, `yanked = "deny"` at `:6` and `multiple-versions = "warn"` at `:22-23` are the exact bars the widened graph is measured against — and the file is deliberately outside this PR's boundary | When running AC-008's `cargo deny`, and again if EC-002 or EC-007 fires | AC-008 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The floor, and the finding that a *dependency's build script* moved it while `cargo hack --rust-version` could not see it coming — which is why AC-008 runs the compiler and not only the manifest check | Before running AC-008's MSRV measurement; mandatory reading if EC-001 fires | AC-008 |
| `.kb/decisions/0009-error-send-sync.md` | The accepted, immutable decision that `Error` keeps `core::error::Error + 'static` on both ports and both flavours, with the strength in a downstream marker. It is what makes this story *confirm* ES-6 rather than reopen it | Before touching the error type, and before any thought of stringifying | AC-004, AC-007 |
| `spec/SPECIFICATION.md` | ES-6 at `:2629-2686` is `[FROZEN]`, names this crate's `Rc<str>`-backed error as what makes the decision falsifiable, and cites `js.rs` by line number; the ledger row is at `:8588` | Before AC-009's citation check; before any change to the error's payload type | AC-004, AC-007, AC-009 |
| `.kb/decisions/0001-async-port-flavours.md` | Why no `#[async_trait]` and why the two-flavour derivation exists at all — the reason a `Send` bound cannot simply be added to make this easier | If the twin's `Send` result is surprising, or before touching anything with a `Send` bound | AC-004, AC-005 |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | The ceiling-and-page mechanism the non-snapshot cursor is *destined* for. Opened here to know what **not** to do: buffering the cursor in this PR pre-empts a decision that belongs to another story | Before AC-003's cursor work, if buffering starts to look tempting (EC-006) | AC-003 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` | Architecture §1 (the seam table), §3 (the stand-in → real binding map), §6 (latitude), §7 (standing detectors); Testing §1–§2 (the tier question); Deployment §4 (the dependency-surface cost) | §3 before writing bindings; §7 before declaring done; Testing §2 the moment `cargo test` link behaviour is known | AC-001, AC-003, AC-004, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/worker-binding-layer/discover.md` | The signal ledger and, more importantly, **The wrong implementation** at `:92-130` — the cfg-split mutant stated in full with a count of what each check would see | Before deciding where the probes live. Re-read at review time as the mutant this diff must not be | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` | The **Standing detectors these stories must not silence** list and the **Merge order** §2 that puts this story first in its slice with three consumers behind it | At the start, for sequencing; at the end, to check no detector went quiet | AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` | The signed-off determination that this project has **no user-facing surface** — the authority for why the composition family above is substituted rather than invented | Only if a surface obligation seems to be implied; it is not | — |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half enumerated.** None added, none dropped. AC-001…
   AC-003, AC-006 and AC-009 carry project AC-001's bindings half; AC-004, AC-005 and AC-007 carry
   project AC-005's `!Send` half; AC-008 carries the merge-day price of the dependency both require.
2. **Which target hosts the unit tier is decided by measurement, and the twin is added either way.**
   The testing brief left this open (`_decomposition.md`, Testing §2) and `discover.md` set the
   default: host-native if `cargo test -p happenstance-cloudflare` still links. That default is kept
   (AC-004) *and* AC-005 adds the `wasm32` twin unconditionally, because the mutant this story must
   not be is precisely one that satisfies a host-only probe. EC-004 states the fallback.
3. **`JsThrow` stays live; `StringifiedThrow` stays in the tree.** Not re-litigated — recorded as
   latitude with a non-neutral default (`_decomposition.md`, Architecture §6), and it earns a second
   keep as the positive control at `src/lib.rs:216-219`. A deviation is argued in ADR-0023, not in a
   commit message (AC-007).
4. **`Default` for `CloudflareEventStore` and `SqlStorage` is expected to be removed.** The front
   half left it as "decided in this PR"; this is the decision. A real `SqlStorage` cannot be minted
   from nothing — it comes off `State::storage().sql()` inside a live Durable Object — so a `Default`
   that returns one would be a second construction path the fixture's *one instance, one object*
   invariant does not cover. Removing it is inside the boundary (it is `crates/happenstance-cloudflare/**`),
   only in-tree callers can exist because the crate is `publish = false`, and if it survives for a
   reason discovered at implementation time, the ledger records that reason under AC-002.
5. **Compiling the twin is not executing it, and this story only promises the first.** The gate step
   amended by AC-006 is a `cargo check`. Standing execution needs the `wasm-execution-seam`
   milestone, which carries no dependency edge to this story. Where a runner happens to resolve on
   the implementer's machine, AC-005 asks for one run and a recorded result — evidence, not a gate.
6. **No conformance rule is added, changed or newly exercised**, so no `mutation_coverage.rs` row
   and no CF-29 changelog entry is owed here. Stated because an absent changelog entry usually *is*
   a defect in this repository; here it is the correct state, and the suite arrives with
   `every-rule-under-workerd`.
7. **Nothing `[FROZEN]` is amended.** ES-6 is confirmed against a real `worker::Error`, which is what
   ADR-0009 already decided would happen. If it cannot be — EC-005 — the answer is a new atom and a
   re-plan, never an edit to an accepted decision body, which `redkiln validate --kb` checks against
   `HEAD`.
8. **`deny.toml` and `rust-toolchain.toml` stay outside the boundary, and that is a load-bearing
   choice rather than tidiness.** Both are one-line edits that would turn AC-008's two measurements
   into green checkmarks while deleting the finding they exist to produce.
