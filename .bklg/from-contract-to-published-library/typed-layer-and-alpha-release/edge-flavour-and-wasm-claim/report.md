---
item: "HS-S0031"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — The typed layer's wasm32 claim, stated either way

## Findings Ledger

**Outcome: seven of seven ACs satisfied. Nothing deferred, nothing blocked.** Both
hand-run negative controls were executed and observed. One real defect was found by
AC-005's instrument and resolved without touching a frozen clause; two findings were routed
to the slice-mate's defect log rather than absorbed.

| AC | Result | Proven by | Notes |
| --- | --- | --- | --- |
| **AC-001** | satisfied | `xtask/src/main.rs:1257::tests::typed_layer_wasm_step_carries_the_designed_arguments`; `cargo xtask ci --fast` printing five `=== wasm32 … ===` sections | RED: *no step named `wasm32 build of the typed layer`*. The test reads the step's `//` comment block out of the source and requires it to name Workers, `Send`/flavour **and** `flavours.rs` — a comment that restates the arguments fails |
| **AC-002** | satisfied | `xtask/src/main.rs:1318::tests::wasm_steps_resolve_and_number_five` + the hand-run control | RED: `left: 4 right: 5`. Control observed: deleting the `REQUIRED` entry made `cargo xtask wasm` panic at `xtask/src/main.rs:907` with ``REQUIRED must contain the `wasm32 build of the typed layer` step`` rather than printing four green sections |
| **AC-003** | satisfied | `crates/happenstance/tests/flavours.rs:333::every_entry_point_binds_the_weak_flavour`; `:403::each_module_imports_one_flavour_name` | `commit`, `commit_with` and `run_projection` each instantiated against the `Rc`-backed `LocalStore`. No module under `crates/happenstance/src/` imports `SendEventStore` and none writes `dyn EventStore` |
| **AC-004** | satisfied | `crates/happenstance/tests/flavours.rs:287::the_local_store_is_not_send` + the hand-run discriminator | **Two** positive controls, not one: `MemoryEventStore` is `Send`, and so is `RefCell<Vec<SequencedEvent>>` — which turns *"`Rc`, not `RefCell`"* into an assertion. Discriminator observed: deleting the probe's inherent block failed **both** positive controls (2 failed / 5 passed) while both negatives kept passing |
| **AC-005** | satisfied | `crates/happenstance/tests/flavours.rs:545::run_projection_spawns_from_generic` over `:519::spawns_the_projection_runner` | **The one row that was genuinely red for a behavioural reason.** See *Finding 1* |
| **AC-006** | satisfied | `xtask/src/main.rs:1356::tests::the_wasm32_powerset_covers_the_typed_layer`; `cargo hack check -p happenstance --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` 144/144 green | RED listed the three packages that were there. The test also asserts the probe is **retained** — a mandatory powerset breaks every machine without `cargo hack` |
| **AC-007** | satisfied | `crates/happenstance/tests/manifest_contract.rs:235::async_trait_is_banned`; `crates/happenstance/tests/doc_surface.rs:492::entry_points_state_their_flavour`; `:554::no_page_claims_an_executed_edge_test`; `cargo deny check bans` exit 0 | Both halves RED first. The density check then failed a second time on real prose (81 columns) and the sentence yielded |

### Mount point

`xtask/src/main.rs::REQUIRED` — composition root 5 — at `:296-333`, reached by name from
`wasm_steps()` (`:905-912`) through `steps_named`, and inherited by `run_ci` and `run_fast`
with no further wiring. The `OPTIONAL` co-mount is the `wasm32 feature powerset` step at
`:690-698`. The second mount is `deny.toml`'s `[bans]` (`:38-60`), reached by
`cargo deny check`. Both are live: `cargo xtask ci --fast` runs the first and prints five
`wasm32` sections; `cargo deny check` runs the second and exits 0.

### Finding 1 — `run_projection` cannot be spawned without a caller-side bound

AC-005's test, written with the bound the spec fixed, did not compile:

> `error[E0277]: <S as SendEventStore>::Error cannot be sent between threads safely` …
> *required because it appears within the type*
> `happenstance::runner::Stopped<<S as SendEventStore>::Error, MemoryProjectionStoreError>`

On a failure the runner holds the stop — carrying `S::Error` — across the port's `rollback`
await (`crates/happenstance/src/runner.rs:480`), because one `ProjectionError` needs the
error *and* what the rollback said. RS-25-4's collapse is unavailable there: the value that
must survive the await **is** the error. The command loop escapes it only because it returns
its error with no await after it.

**Disposition: not a defect, and not routed as one.** ES-6 is `[FROZEN]` and leaves `Error`
unbounded deliberately; ADR-0009 assigns the obligation to the caller in terms and supplies
the shape — a marker trait with a blanket impl, declared by the consumer and explicitly not
shipped in `happenstance-core`. So `crates/happenstance/tests/flavours.rs:503` declares
`ThreadSafeEventStore` locally and the bound reads `S: ThreadSafeEventStore + Send + Sync +
'static`. The promise was kept; what was missing was that anyone had said so where a caller
would meet it. That is now `crates/happenstance/src/runner.rs:297-310`.

Inventing a defect entry here would have cited a clause that was never at issue, which
`defect-log-and-macros-verdict`'s EC-001 forbids by name. It is carried into that story's
log as an **accounted finding with the disposition "found none"**, with this reasoning.

### Finding 2 — routed to `support`, no clause ID

`cargo hack`'s wasm32 powerset emits `warning: method read_through is never used`
(`crates/happenstance-core/src/projection_memory.rs:233`) in eight feature combinations. It
**pre-dates this story** — the existing three-package powerset emits it too — and CI's
ambient `RUSTFLAGS: -D warnings` turns it into a failure there. It bears on no clause, and
`crates/happenstance-core/src/**` is inadmissible to this project, so it is classified
**support** at the moment of finding, per EC-001, and named in the slice-mate's log.

### Two boundary widenings, declared

1. **`deny.toml` gained `allow-wildcard-paths = true`.** `cargo deny check bans` was already
   red on this tree before any edit — confirmed by stashing the change and re-running —
   because `happenstance` dev-depends on `happenstance-testkit` by versionless path, which is
   deliberate and documented (it keeps CF-32's release order free). It is inside this story's
   fence and had to be green before `publish-0-2-0-alpha-1` could run the full gate.
2. **`standards/rust/` re-anchoring**, in a **separate commit** following this branch's own
   precedent (`c4e36c4`). Inserting the fifth step moved twenty `xtask/src/main.rs:NNN`
   citations; `cargo xtask lint-constitution` now reports *27 atoms, all consistent*.

**Amended 2026-08-16 — declared is not the same as admitted.** Both items above were
written up here and in the implementation report, and neither reached the artefact that
decides: item 2's paths were outside `spec.md`'s fenced block, so `redkiln verify --grain
story` would still fail on them, and item 1's relaxation of `wildcards = "deny"` appeared
in no *In this PR* list, ledger row or AC table. Both are now on the record where a
reader and the verifier look — `standards/rust/**` added to the fence on `34d5311`'s
**CITATION RE-ANCHORING ONLY** terms, and `allow-wildcard-paths` named in *In this PR*
and cited in AC-007's ledger evidence beside the `async-trait` ban that motivated the
visit.

### Nothing deferred

No public item added (NF-005). No `[FROZEN]` clause edited and no file under
`crates/happenstance-core/src/**` or `spec/` touched (NF-006). No dependency-graph change
(NF-002, NF-003). The four existing `wasm32` steps are byte-identical in name, order and
arguments (NF-004). `cargo xtask ci --fast`'s packaging step is green (NF-008).
