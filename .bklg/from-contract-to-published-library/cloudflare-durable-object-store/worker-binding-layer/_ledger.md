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
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml [workspace.dependencies]; crates/happenstance-cloudflare/Cargo.toml:14-17"
  verifying_test: "cargo xtask wasm — step `wasm32 build of the Cloudflare adapter` (xtask/src/main.rs:245-264); cargo deny check bans"

- id: AC-002
  criterion: "GIVEN that same author reading the crate to decide whether it does what it says, WHEN they grep it for unimplemented bodies, THEN the five *binding* `todo!()`s in `src/js.rs` and `src/sql_storage.rs` are gone and replaced by real calls; the six `EventStore` bodies in `src/event_store.rs` and the scoped `#![allow(clippy::todo)]` at `src/lib.rs:126` are still there and still honest; `CloudflareEventStore::new(sql)` still takes its handle by injection; and the public re-export list at `src/lib.rs:128-135` gains no new item."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs:70-87 — CloudflareEventStore::new(sql)"
  verifying_test: "rg -n 'todo!\\(' crates/happenstance-cloudflare/src (exactly six event_store.rs sites); cargo clippy --workspace --all-targets --all-features -- -D warnings"

- id: AC-003
  criterion: "GIVEN an author who chose this adapter *because* a Durable Object is a synchronous, single-threaded, re-entrant actor, WHEN they drive the bound `SqlStorage`, THEN all four modelled properties still hold of the real binding: `exec` is synchronous, the cursor is a live iterator and not a buffered snapshot, every type is `!Send` and `!Sync`, and re-entrancy is *reported* as `SqlError::AlreadyBorrowed` rather than panicking."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/sql_storage.rs — SqlStorage::exec (:165-178) and SqlCursor (:193-269), reached through CloudflareEventStore::new(sql)"
  verifying_test: "crates/happenstance-cloudflare/src/sql_storage.rs — sql_storage::tests::reentrant_borrow_is_reported_not_panicked; cargo check -p happenstance-cloudflare (non-async exec keeps non-async read type-checking)"

- id: AC-004
  criterion: "GIVEN a contributor mid-loop with no wasm toolchain installed at all, WHEN they run `cargo test -p happenstance-cloudflare`, THEN the four `!Send` assertions still run and pass on the host — `the_probe_is_not_vacuous` included — over the **same** error type the target compiles, with no `cfg` handing the host a `Send`-safe stand-in and the target a real one."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs:137-259 — the not_send_probe module and its tests"
  verifying_test: "cargo test -p happenstance-cloudflare — tests::the_probe_is_not_vacuous, tests::the_js_boundary_types_are_not_send, tests::the_error_type_is_not_send, tests::the_send_flavour_does_not_imply_a_send_error (crates/happenstance-cloudflare/src/lib.rs:180-259)"

- id: AC-005
  criterion: "GIVEN a gate reader who must believe the `!Send` claim on the one platform this crate exists for, WHEN the crate's tests are compiled for `wasm32-unknown-unknown`, THEN a target-side twin of all four assertions exists in this crate's own tree, carries the positive control with it, and is emitted through `wasm_bindgen_test` from a target-scoped dev-dependency block shaped like `crates/happenstance-testkit/Cargo.toml:53-54`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/lib.rs — the wasm32 twin of not_send_probe; crates/happenstance-cloudflare/Cargo.toml — [target.'cfg(target_arch = \"wasm32\")'.dev-dependencies]"
  verifying_test: "cargo check --locked -p happenstance-cloudflare --tests --target wasm32-unknown-unknown (the AC-006 step); one local wasm-bindgen-test run recorded as evidence where a runner resolves"

- id: AC-006
  criterion: "GIVEN a gate reader on a clean checkout, WHEN they run `cargo xtask ci --fast`, THEN the `wasm32 build of the Cloudflare adapter` step compiles the crate's **test** targets (`--tests`, not `--all-targets`), keeps its name so `wasm_steps()` still selects it by name rather than index, and carries a comment saying why — so the twin added by AC-005 is built by the gate on the day it merges instead of a fortnight later."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:245-264 — the `wasm32 build of the Cloudflare adapter` Step, selected by name in wasm_steps() (:784-791)"
  verifying_test: "cargo xtask ci --fast; cargo xtask wasm (a rename panics in steps_named)"

- id: AC-007
  criterion: "GIVEN a caller who must branch on constraint violation versus transport fault, WHEN they hold a `CloudflareEventStoreError` built from a real thrown value, THEN the thrown value is still **live** — `JsThrow`, not stringified — `JsThrow::is_constraint_violation` is real rather than `todo!()`, `StringifiedThrow` remains in the tree as the recorded alternative *and* as the probe's positive control, and the error type still has no `ConditionViolated` variant."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs:89-143 — CloudflareEventStoreError, reached through CloudflareEventStore::new(sql)"
  verifying_test: "cargo test -p happenstance-cloudflare — tests::the_error_type_is_not_send and tests::the_probe_is_not_vacuous; a new #[test] over JsThrow::is_constraint_violation in crates/happenstance-cloudflare/src/js.rs against `UNIQUE constraint failed: event.position`"

- id: AC-008
  criterion: "GIVEN a library consumer pinned to the 1.97.1 floor and a gate reader who owns the licence allowlist, WHEN `worker` enters the graph, THEN both prices are **measured in this PR and written into this story's ledger** — `worker`'s declared `rust-version`, or the compiler's answer if it declares none, against ADR-0029's floor, and `cargo deny`'s licence/advisory/bans verdict against `deny.toml:10-19`'s eight allowed licences — and neither `deny.toml` nor `rust-toolchain.toml` is edited to make either of them pass."
  satisfied: false
  evidence: ""
  mount_point: "Cargo.toml [workspace.dependencies] — the single declaration of `worker`; deny.toml and rust-toolchain.toml deliberately outside the PR boundary"
  verifying_test: "cargo hack check -p happenstance-cloudflare --no-dev-deps --rust-version; cargo +1.97.1 test -p happenstance-cloudflare; cargo deny check licenses advisories bans; git diff --stat showing deny.toml and rust-toolchain.toml untouched"

- id: AC-009
  criterion: "GIVEN a reviewer checking that the crate has stopped describing itself falsely, WHEN they read the manifest note, the crate documentation and ES-6's citations, THEN the deliberate no-`worker` note at `crates/happenstance-cloudflare/Cargo.toml:19-25` has been **replaced by the record of its reversal** rather than deleted, `src/lib.rs:1-30` and `src/event_store.rs:1-6` say precisely what is bound and what is still `todo!()`, and `spec/SPECIFICATION.md`'s ES-6 citations into `src/js.rs` still land on the live `Rc`-shaped payload."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/Cargo.toml:19-25 and crates/happenstance-cloudflare/src/lib.rs:1-30 — the crate's own record of what it is bound to"
  verifying_test: "cargo xtask spec-trace; cargo doc --workspace --all-features (workspace missing_docs is denied)"
```
