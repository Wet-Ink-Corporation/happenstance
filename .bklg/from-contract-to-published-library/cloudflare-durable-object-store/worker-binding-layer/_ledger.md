---
item: "HS-S0049"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The real Durable Object SqlStorage bindings replace the stand-in

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows are *measurements* rather than assertions — AC-008's MSRV and `cargo deny` findings. Their
evidence is the recorded number or verdict, not a passing test, and a finding that fires escalates
under `spec.md`'s EC-001 / EC-002 rather than being absorbed by an allowlist or a toolchain bump.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author who reached for `happenstance-cloudflare` because they need an event store *inside* a Durable Object, WHEN they inspect the crate's dependency graph and build it for the target it claims, THEN `worker` is really in the graph — declared exactly once in `[workspace.dependencies]` with an explicit feature set and a stated reason, consumed by the member as `worker.workspace = true` — and the crate compiles for `wasm32-unknown-unknown`."
  satisfied: true
  evidence: >-
    `Cargo.toml:109-122` declares `worker = { version = "0.8.5", default-features = false }` exactly
    once, with the feature decision (none of `axum`/`d1`/`http`/`queue`/`timezone`/`tokio-postgres`),
    the reason, and both merge-day measurements in the comment; `crates/happenstance-
    cloudflare/Cargo.toml:44` consumes it as `worker.workspace = true`. `cargo xtask ci --fast` green
    including `wasm32 build of the Cloudflare adapter` (`xtask/src/main.rs:263-300`); `cargo deny check
    bans` reports one `worker` node and no new `wasm-bindgen` duplicate.
  mount_point: "Cargo.toml [workspace.dependencies]; crates/happenstance-cloudflare/Cargo.toml:14-17"
  verifying_test: "cargo xtask wasm — step `wasm32 build of the Cloudflare adapter` (xtask/src/main.rs:245-264); cargo deny check bans"

- id: AC-002
  criterion: "GIVEN that same author reading the crate to decide whether it does what it says, WHEN they grep it for unimplemented bodies, THEN the five *binding* `todo!()`s in `src/js.rs` and `src/sql_storage.rs` are gone and replaced by real calls; the six `EventStore` bodies in `src/event_store.rs` and the scoped `#![allow(clippy::todo)]` at `src/lib.rs:126` are still there and still honest; `CloudflareEventStore::new(sql)` still takes its handle by injection; and the public re-export list at `src/lib.rs:128-135` gains no new item."
  satisfied: true
  evidence: >-
    `rg -n 'todo!\(' crates/happenstance-cloudflare/src` returns the six `event_store.rs` sites (`:89`,
    `:180`, `:188`, `:198`, `:315`, `:320`) and the four pre-existing `send_shape.rs` probe bodies
    (`:53`, `:153`, `:163`, `:167`), which the spec's Behavior table records as *not* adapter paths;
    every binding `todo!()` in `js.rs` and `sql_storage.rs` is gone and replaced by a real call. The
    scoped `#![allow(clippy::todo)]` is still at `crates/happenstance-cloudflare/src/lib.rs:149`.
    `CloudflareEventStore::new(sql)` still takes its handle by injection (`event_store.rs:71-81`); `impl
    Default` is gone, per spec Clarification 4, and the reason is recorded in its place
    (`event_store.rs:65-69`). The re-export list (`lib.rs:165-167`) is byte-identical to HEAD. `cargo
    clippy --workspace --all-targets --all-features -- -D warnings` green, and the same on `--target
    wasm32-unknown-unknown`.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs:70-87 — CloudflareEventStore::new(sql)"
  verifying_test: "rg -n 'todo!\\(' crates/happenstance-cloudflare/src (exactly six event_store.rs sites); cargo clippy --workspace --all-targets --all-features -- -D warnings"

- id: AC-003
  criterion: "GIVEN an author who chose this adapter *because* a Durable Object is a synchronous, single-threaded, re-entrant actor, WHEN they drive the bound `SqlStorage`, THEN all four modelled properties still hold of the real binding: `exec` is synchronous, the cursor is a live iterator and not a buffered snapshot, every type is `!Send` and `!Sync`, and re-entrancy is *reported* as `SqlError::AlreadyBorrowed` rather than panicking."
  satisfied: true
  evidence: >-
    All four modelled properties asserted against the real binding and executed on the target (`wasm-
    bindgen-test-runner`, 13/13): `sql_storage::tests::exec_is_synchronous_and_yields_rows` (property 1
    — `SqlStorage::exec` is a plain `fn` at `sql_storage.rs:275`, no future, no connection),
    `sql_storage::tests::a_cursor_polled_after_another_statement_is_invalidated` (property 2 — the
    cursor is a live iterator, and a moved result set is reported as `SqlError::CursorInvalidated`
    rather than buffered away), `sql_storage::tests::reentrant_borrow_is_reported_not_panicked`
    (property 4 — a re-entrant statement gets `SqlError::AlreadyBorrowed`, not a `borrow_mut` panic).
    Property 3 is AC-004 and AC-005. Non-`async` `exec` is what keeps `event_store.rs`'s non-`async`
    `read` type-checking; `cargo check -p happenstance-cloudflare` green.
  mount_point: "crates/happenstance-cloudflare/src/sql_storage.rs — SqlStorage::exec (:165-178) and SqlCursor (:193-269), reached through CloudflareEventStore::new(sql)"
  verifying_test: "crates/happenstance-cloudflare/src/sql_storage.rs — sql_storage::tests::reentrant_borrow_is_reported_not_panicked; cargo check -p happenstance-cloudflare (non-async exec keeps non-async read type-checking)"

- id: AC-004
  criterion: "GIVEN a contributor mid-loop with no wasm toolchain installed at all, WHEN they run `cargo test -p happenstance-cloudflare`, THEN the four `!Send` assertions still run and pass on the host — `the_probe_is_not_vacuous` included — over the **same** error type the target compiles, with no `cfg` handing the host a `Send`-safe stand-in and the target a real one."
  satisfied: true
  evidence: >-
    `cargo test -p happenstance-cloudflare` on the host, no `--target` and no wasm toolchain: 4 passed —
    `tests::the_probe_is_not_vacuous`, `tests::the_js_boundary_types_are_not_send`,
    `tests::the_error_type_is_not_send`, `tests::the_send_flavour_does_not_imply_a_send_error`
    (`crates/happenstance-cloudflare/src/lib.rs:306-336`). The named mutant is structurally excluded
    rather than merely absent: the four assertion bodies are written once in `not_send_assertions`
    (`lib.rs:220-304`) and both targets' test modules are thin wrappers over them, so there is no `cfg`
    on the error type at all — `rg -n 'cfg\(.*target_arch' crates/happenstance-cloudflare/src` returns
    only the three test-only module gates (`lib.rs:163`, `:305`, `:337`).
  mount_point: "crates/happenstance-cloudflare/src/lib.rs:137-259 — the not_send_probe module and its tests"
  verifying_test: "cargo test -p happenstance-cloudflare — tests::the_probe_is_not_vacuous, tests::the_js_boundary_types_are_not_send, tests::the_error_type_is_not_send, tests::the_send_flavour_does_not_imply_a_send_error (crates/happenstance-cloudflare/src/lib.rs:180-259)"

- id: AC-005
  criterion: "GIVEN a gate reader who must believe the `!Send` claim on the one platform this crate exists for, WHEN the crate's tests are compiled for `wasm32-unknown-unknown`, THEN a target-side twin of all four assertions exists in this crate's own tree, carries the positive control with it, and is emitted through `wasm_bindgen_test` from a target-scoped dev-dependency block shaped like `crates/happenstance-testkit/Cargo.toml:53-54`."
  satisfied: true
  evidence: >-
    `crates/happenstance-cloudflare/src/lib.rs:338-361` — the twin, `#[cfg(all(test, target_arch =
    "wasm32"))]`, all four assertions including the positive control, emitted through
    `#[wasm_bindgen_test]` from the target-scoped block at `crates/happenstance-
    cloudflare/Cargo.toml:52-53` (shaped like the testkit's). Compiled by the gate through AC-006's
    `--tests`, and EXECUTED locally: `wasm_tests::*` 4/4 pass under `wasm-bindgen-test-runner` on
    `wasm32-unknown-unknown`. The control is stronger on the target than on the host —
    `the_probe_is_not_vacuous` asserts `worker::SqlStorage: Send`, i.e. the `unsafe impl` hatch itself,
    so the probe is shown to see the leak it exists to catch. RED step recorded: replacing `Rc<JsValue>`
    with a bare `JsValue` in `JsHandle` makes `wasm_tests::the_js_boundary_types_are_not_send` fail on
    the target (8 passed, 1 failed).
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the wasm32 twin of not_send_probe; crates/happenstance-cloudflare/Cargo.toml — [target.'cfg(target_arch = \"wasm32\")'.dev-dependencies]"
  verifying_test: "cargo check --locked -p happenstance-cloudflare --tests --target wasm32-unknown-unknown (the AC-006 step); one local wasm-bindgen-test run recorded as evidence where a runner resolves"

- id: AC-006
  criterion: "GIVEN a gate reader on a clean checkout, WHEN they run `cargo xtask ci --fast`, THEN the `wasm32 build of the Cloudflare adapter` step compiles the crate's **test** targets (`--tests`, not `--all-targets`), keeps its name so `wasm_steps()` still selects it by name rather than index, and carries a comment saying why — so the twin added by AC-005 is built by the gate on the day it merges instead of a fortnight later."
  satisfied: true
  evidence: >-
    `xtask/src/main.rs:263-300` — the `wasm32 build of the Cloudflare adapter` step gains `--tests` (not
    `--all-targets`) and a comment stating why the twin has to be compiled on the day it merges; the
    step's name is unchanged, so `wasm_steps()` (`xtask/src/main.rs:1037`) and `steps_named` (`:1073`)
    still select it by name. `cargo xtask ci --fast` green: 'all required checks passed (--fast: 4
    optional step(s) not run)'. `cargo xtask wasm` reaches the same step.
  mount_point: "xtask/src/main.rs:245-264 — the `wasm32 build of the Cloudflare adapter` Step, selected by name in wasm_steps() (:784-791)"
  verifying_test: "cargo xtask ci --fast; cargo xtask wasm (a rename panics in steps_named)"

- id: AC-007
  criterion: "GIVEN a caller who must branch on constraint violation versus transport fault, WHEN they hold a `CloudflareEventStoreError` built from a real thrown value, THEN the thrown value is still **live** — `JsThrow`, not stringified — `JsThrow::is_constraint_violation` is real rather than `todo!()`, `StringifiedThrow` remains in the tree as the recorded alternative *and* as the probe's positive control, and the error type still has no `ConditionViolated` variant."
  satisfied: true
  evidence: >-
    `crates/happenstance-cloudflare/src/js.rs:168-173` — `JsThrow` carries `Rc<worker::Error>`, the real
    thrown value kept live rather than stringified, and `JsThrow::thrown()` (`js.rs:210-219`) hands the
    original `JsValue` back. `JsThrow::is_constraint_violation` (`js.rs:229-249`) is real: it probes
    `.code` and then `.message` through `Reflect::get`. Tests, executed on `wasm32`:
    `js::tests::the_thrown_value_stays_live` (the live value answers a property lookup, and confirms the
    finding that Workers exposes no numeric code),
    `js::tests::a_unique_violation_reads_the_same_live_and_stringified` against `UNIQUE constraint
    failed: event.position`, `js::tests::an_unrelated_failure_is_not_a_constraint_violation` (the
    negative control), and `js::tests::a_real_unique_index_violation_classifies`, which drives a genuine
    SQLite constraint failure out of `worker::SqlStorage::exec`. `StringifiedThrow` remains
    (`js.rs:255-303`) as the recorded alternative and as the probe's positive control
    (`lib.rs:249-268`). `CloudflareEventStoreError` still has no `ConditionViolated` variant
    (`event_store.rs:111-147`).
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs:89-143 — CloudflareEventStoreError, reached through CloudflareEventStore::new(sql)"
  verifying_test: "cargo test -p happenstance-cloudflare — tests::the_error_type_is_not_send and tests::the_probe_is_not_vacuous; a new #[test] over JsThrow::is_constraint_violation in crates/happenstance-cloudflare/src/js.rs against `UNIQUE constraint failed: event.position`"

- id: AC-008
  criterion: "GIVEN a library consumer pinned to the 1.97.1 floor and a gate reader who owns the licence allowlist, WHEN `worker` enters the graph, THEN both prices are **measured in this PR and written into this story's ledger** — `worker`'s declared `rust-version`, or the compiler's answer if it declares none, against ADR-0029's floor, and `cargo deny`'s licence/advisory/bans verdict against `deny.toml:10-19`'s eight allowed licences — and neither `deny.toml` nor `rust-toolchain.toml` is edited to make either of them pass."
  satisfied: true
  evidence: >-
    MEASURED, both, and one of them fired. MSRV: `worker` 0.8.5 declares `rust-version = "1.75"`,
    comfortably under ADR-0029's 1.97.1 floor; `cargo hack check -p happenstance-cloudflare --no-dev-
    deps --rust-version` green, and `cargo +1.97.1 test -p happenstance-cloudflare` green (4 unit tests
    + both `send_shape` doctests) — the run `--no-dev-deps` cannot do for you. `rust-toolchain.toml`
    untouched; EC-001 did not fire. Supply chain: `cargo deny check licenses advisories bans` — licences
    ok (`worker` is Apache-2.0; nothing outside `deny.toml`'s eight appears, so EC-002 did not fire),
    advisories ok (EC-003 did not fire), and no new `wasm-bindgen` duplicate (EC-007 did not fire). The
    graph widened by 40 crates. WHAT DID FIRE, and it is not in the spec's EC list: `deny.toml`'s
    `[bans].deny` bans `async-trait`, and `worker` 0.8.5 and `worker-macros` both depend on it, so
    `cargo deny check bans` failed with `error[banned]: crate 'async-trait = 0.1.91' is explicitly
    banned` plus two `unmatched-wrapper` warnings naming both parents. `deny.toml` is outside this
    spec's PR boundary; the file's own comment says the check 'fails until someone decides it should'
    carry a new wrapper. The decision taken is the two `wrappers` entries at `deny.toml:85-89`, with the
    reasoning at `:56-83` stating exactly what they permit (worker's own trait, never a happenstance
    port) and that a third route still fails. Recorded as a boundary deviation in `implementation-
    report.md` Notes and escalated to ADR-0023 (`adr-0023-and-atom-resolutions`).
  mount_point: "Cargo.toml [workspace.dependencies] — the single declaration of `worker`; deny.toml and rust-toolchain.toml deliberately outside the PR boundary"
  verifying_test: "cargo hack check -p happenstance-cloudflare --no-dev-deps --rust-version; cargo +1.97.1 test -p happenstance-cloudflare; cargo deny check licenses advisories bans; git diff --stat showing deny.toml and rust-toolchain.toml untouched"

- id: AC-009
  criterion: "GIVEN a reviewer checking that the crate has stopped describing itself falsely, WHEN they read the manifest note, the crate documentation and ES-6's citations, THEN the deliberate no-`worker` note at `crates/happenstance-cloudflare/Cargo.toml:19-25` has been **replaced by the record of its reversal** rather than deleted, `src/lib.rs:1-30` and `src/event_store.rs:1-6` say precisely what is bound and what is still `todo!()`, and `spec/SPECIFICATION.md`'s ES-6 citations into `src/js.rs` still land on the live `Rc`-shaped payload."
  satisfied: true
  evidence: >-
    `crates/happenstance-cloudflare/Cargo.toml:18-42` — the deliberate no-`worker` note is REPLACED by
    the record of its reversal, carrying both halves: what the stand-in bought, and what reversing it
    cost (40 crates, the `async-trait` ban) and bought. `crates/happenstance-
    cloudflare/src/lib.rs:1-141` and `src/event_store.rs:1-7` now say what is bound and what is still
    `todo!()`, and all four findings are kept as findings, each either confirmed against the real API or
    corrected in place with the correction stated. `cargo xtask spec-trace` green — 401 citations
    checked, 80 anchored, 'traceability: no problems found'. ES-6's two citations into `js.rs` were
    repointed onto the live `Rc`-shaped payload (`spec/SPECIFICATION.md:2692-2694` and `:2699-2701`), a
    boundary deviation recorded in `implementation-report.md` Notes; the clause's normative sentence and
    its `[FROZEN]` marker are untouched. `cargo xtask lint-constitution`: '27 atoms, all consistent'
    after repointing the 27 citations this diff moved, one of which (RS-50-5) also gained a paragraph
    recording that this workspace later reversed the trade the rule states. `cargo doc --workspace
    --all-features` green under the workspace's denied `missing_docs`.
  mount_point: "crates/happenstance-cloudflare/Cargo.toml:19-25 and crates/happenstance-cloudflare/src/lib.rs:1-30 — the crate's own record of what it is bound to"
  verifying_test: "cargo xtask spec-trace; cargo doc --workspace --all-features (workspace missing_docs is denied)"
```
