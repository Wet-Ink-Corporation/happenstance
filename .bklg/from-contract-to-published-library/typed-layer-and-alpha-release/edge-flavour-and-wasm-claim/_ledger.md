---
item: HS-S0031
stage: implement
created: 2026-08-12T13:46:28.148Z
updated: 2026-08-12T13:46:28.148Z
---

# Acceptance ledger — The typed layer's wasm32 claim, stated either way

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows carry a **hand-run negative control** whose observation is part of the evidence, not a
permanent assertion (spec, *Clarifications* item 5): AC-002 (delete the `REQUIRED` entry, confirm
`steps_named` panics) and AC-004 (break the probe, confirm the positive control fails). A row flipped
without its control observed has not been proven.

```yaml
- id: AC-001
  criterion: "GIVEN P3, who has just run `cargo add happenstance` — the crate ADR-0006 gave the bare name to — and whose target is `wasm32-unknown-unknown`, WHEN they run `cargo build --target wasm32-unknown-unknown` on their Workers project, THEN it compiles, because this repository's own mandatory gate compiled that same crate for that same target first: a fifth `Step` sits in `xtask/src/main.rs::REQUIRED` (`:105`, composition root 5) with `program: \"cargo\"`, args `check --locked -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json` exactly as `_design.md:648` fixed them, `env: &[]` and `probe: None` — persistent chrome, never opened on demand, because a probed step means *skip when the tool is absent* and this guard may never be skippable (RS-80-2); it carries `--locked` (RS-80-4); its name is a grammatical peer of the four it joins so the `=== name ===` transcript still scans as one family; and it carries a real comment stating both what it buys (the crate a Workers application installs is compiled for the target it claims) and what it does not (`Send` exists on `wasm32`, so this proves nothing about which flavour is bound) — a step whose comment only restates its arguments is the decorative shape `xtask/src/main.rs:192-202` warns about"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs::REQUIRED (`:105`) — composition root 5; inherited by run_ci (`:828-833`) and run_fast (`:853-860`)"
  verifying_test: "xtask/src/main.rs::tests::typed_layer_wasm_step_carries_the_designed_arguments (new); `cargo xtask ci --fast` printing five `=== wasm32 … ===` sections"

- id: AC-002
  criterion: "GIVEN a maintainer six months from now refactoring `xtask/src/main.rs` — reordering steps, renaming one, or deleting what looks redundant — WHEN they remove or rename the typed layer's step, THEN the gate fails loudly and by name rather than narrowing in silence: the step's name is listed in `wasm_steps()` (`:784-791`), which resolves through `steps_named` (`:816-826`) and panics with ``REQUIRED must contain the `{name}` step`` — the replacement for an index selection that once pointed `cargo xtask wasm` at clippy while printing green (`:769-782`); and the reader's position is preserved: the four existing steps keep their names, their order and their argument lists byte-for-byte, the new one is appended after them, and `cargo xtask wasm` therefore prints the same four sections in the same order followed by one more"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs::wasm_steps() (`:784-791`) reaching REQUIRED (`:105`) by name via steps_named (`:816-826`)"
  verifying_test: "xtask/src/main.rs::tests::wasm_steps_resolve_and_number_five (new); plus the hand-run negative control — delete the REQUIRED entry, confirm `cargo xtask wasm` panics"

- id: AC-003
  criterion: "GIVEN P3 on a Cloudflare Durable Object, whose store is `Rc`-backed and therefore genuinely `!Send`, WHEN they call any entry point this crate exposes — `commit`, `commit_with`, `run_projection`, the `testing` DSL's seams — THEN every one of them accepts their store, because this crate's generic code binds `EventStore` and never `SendEventStore` (the weaker requirement, which `trait_variant`'s blanket impl makes accept both — RS-20-2), each module imports exactly one flavour name (RS-20-3; CLAUDE.md constraint 4), and no signature says `dyn EventStore` (RS-20-5); the proof is an instantiation, not a bound read off the source, because a generic body type-checks against its declared bounds whether or not anything instantiates it"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/** entry points, instantiated from crates/happenstance/tests/flavours.rs (created by command-loop, extended here)"
  verifying_test: "crates/happenstance/tests/flavours.rs::every_entry_point_binds_the_weak_flavour (new), run under `cargo test -p happenstance --all-features --test flavours`"

- id: AC-004
  criterion: "GIVEN the same maintainer, who has to be able to believe AC-003 rather than take it on trust, WHEN the `!Send` instrument runs, THEN it cannot pass vacuously: the autoref-specialisation probe that observes the absence of an auto trait carries a positive control asserting `MemoryEventStore` is `Send` in the same test — without it the assertion passes just as happily when the probe is simply broken and always answers `false`, which is the class of decorative check this repository has removed twice (RS-61-3, `standards/rust/61-compile-time-assertions.md:171`, which forbids the probe without a control by name)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/tests/flavours.rs — the probe module, gated `cfg(not(target_arch = \"wasm32\"))`"
  verifying_test: "crates/happenstance/tests/flavours.rs::the_local_store_is_not_send (new); plus the hand-run discriminator — delete the `impl<T: Send> Probe<T>` inherent block and confirm the positive control fails"

- id: AC-005
  criterion: "GIVEN P1 on a multi-threaded Tokio service who moves a projection replay onto its own task, WHEN they `tokio::spawn` a typed-layer read, THEN the `Send` flavour still composes: a generic function bounded `S: SendEventStore + Send + Sync + 'static` holds the stream across an await inside a real spawn, `Sync` and `'static` load-bearing and `Send` redundant-but-stated exactly as `crates/happenstance-core/src/memory.rs:651-680` records; the query is bound to a local rather than inlined (edition 2024 RPITIT lifetime capture, E0716); and every `Result<_, S::Error>` is collapsed before the next await, because `Error` carries no `Send` bound and ES-6 is [FROZEN] (`spec/SPECIFICATION.md:2629`; ADR-0009; RS-25-4). This is the assertion that survives an `async fn read` refactor, which a \"this concrete stream is `Send`\" assertion does not (ADR-0008; ES-2, `:2492`)"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/** — the projection runner entry point (`run_projection`), the one site in this crate where `SendEventStore` may be named, reached by full path"
  verifying_test: "crates/happenstance/tests/flavours.rs::run_projection_spawns_from_generic (new), shaped after crates/happenstance-core/src/memory.rs:643-699"

- id: AC-006
  criterion: "GIVEN P3 who does not take the defaults — they want `postcard` for payload size, or `cbor`, or the `unstable-projection` runner on the edge — WHEN they select any combination the alpha ships, THEN it compiles for `wasm32-unknown-unknown`, because `-p happenstance` is added to the existing `wasm32 feature powerset` step (`xtask/src/main.rs:558-594`) and every combination is compiled on that target rather than only the mandatory step's single `std,json` point. A feature is not target-scoped (RS-52-2) — which is precisely how the testkit's `proptest` feature came to need a second `cfg` condition (`xtask/src/main.rs:576-586`). This is a widening above the mandatory guard and never a replacement for it: the powerset step is OPTIONAL and probed on `cargo hack --version`, and \"a constraint whose only check is skippable is unguarded on every machine that lacks one tool\" (`:198-202`)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the `wasm32 feature powerset` Step in OPTIONAL (`:558-594`), probe retained"
  verifying_test: "xtask/src/main.rs::tests::the_wasm32_powerset_covers_the_typed_layer (new); plus `cargo xtask ci` (not --fast) running the step green"

- id: AC-007
  criterion: "GIVEN P3, whose stated fear is \"being on the less-supported path and being orphaned\" (`initiative.md:216-220`), and a future contributor who reaches for `#[async_trait]` to make `Projection` object-safe, WHEN that contributor adds `async-trait` to any manifest in the workspace, THEN it is refused rather than merged: `async-trait` is listed in `deny.toml`'s `[bans]` deny list (`:22-24`) with a comment naming ADR-0001 and the reason — the attribute injects `+ Send`, which makes the `wasm32` target impossible (`.kb/decisions/0001-async-port-flavours.md`; CLAUDE.md constraint 1; `_design.md:1008-1010` keeps it standing and not re-litigated). And the promise is legible where the reader meets it, in place: each typed-layer entry point's own rustdoc states which flavour it binds and why, on the item itself rather than in a separate \"edge notes\" section a reader must jump to (`standards/rust/70-rustdoc-obligations.md`), holding the design's density budget — first sentence ≤ 80 characters and a complete claim, doc prose ≤ 80 columns. The ban's strength is stated honestly in the same breath: `cargo deny` is OPTIONAL and probed (`xtask/src/main.rs:595-601`), so this is the legible guard and AC-003's instantiation is the unskippable one"
  satisfied: false
  evidence: ""
  mount_point: "deny.toml `[bans]` (`:22-24`), reached by `cargo deny check` (xtask/src/main.rs:595-601); and the rustdoc on each entry point in crates/happenstance/src/**"
  verifying_test: "crates/happenstance/tests/manifest_contract.rs::async_trait_is_banned (extends M3's file); crates/happenstance/tests/doc_surface.rs::entry_points_state_their_flavour (extends M5's file); `cargo deny check bans`"
```
