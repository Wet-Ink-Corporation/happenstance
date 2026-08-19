---
item: "HS-S0053"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — A Durable Object host and the CloudflareFixture mounted on it

**All seven ACs are satisfied.** There is now something to hang the store off that a
*second compilation unit* can name, and a `Fixture` the shipped macro can be handed
unchanged. Three decisions shaped the diff and each was forced by something the compiler
or the gate said rather than argued for.

**The host is a promotion, not a new file.** `src/test_object.rs` already stood up a real
Durable Object `state` backed by Node's `node:sqlite`, reached through
`worker::State::from(DurableObjectState)` → `state.storage().sql()` — the production path
with `worker`'s real `wasm-bindgen` externs in the middle. What it was not was *reachable*:
`#[cfg(all(test, target_arch = "wasm32"))] mod test_object` is invisible from `tests/`,
which is EC-005 word for word. So it became `pub mod host` — renamed because "test_object"
is not what a public item that hosts a store is called, and unconditional because every
`worker` binding links off-target as a panicking stub, so the module compiles everywhere
and only *runs* where a JavaScript heap is. That is the honest shape: a fixture that cannot
connect is a broken test environment and says so by panicking, not by not existing.

**The fixture is in `tests/`, and the reason is the dependency graph rather than taste.**
`happenstance-testkit` is a dev-dependency; an `impl Fixture` in `src/` would put the
conformance suite in the runtime graph of a crate this initiative intends to publish. A
`tests/support/mod.rs` declared by both integration targets buys the same reachability with
none of that, and the spec anticipated it: the *host* is what has to cross the compilation
unit boundary, and it does.

**`REOPEN` came out `SUPPORTED`, with a real override.** The spec expected this verdict to
be the one most likely to be overturned. It was not, and the reason is the host type:
holding the object's `state` and re-deriving a binding from it on demand is exactly what
the trait means by "discard every outstanding handle's process-level state" — a fresh
`SqlStorage` with a fresh cursor generation and a fresh (empty) store-id cache, over rows
nothing touched. `MID_BATCH_FAULT` is the one that stayed declined here, and deliberately:
the *mechanism* exists (the host can throw on a chosen statement), but CF-39 requires a
fixture claiming it to name which statement is the k-th row's, and that verdict is owed to
the executed run.

## TDD Evidence

Every row was RED first, and the RED was an assertion about missing behaviour rather than a
compile accident — except AC-001, where the compile error **is** the behaviour under test.
The RED shape was a deliberately wrong fixture committed to the working tree long enough to
run: instances wired to one `thread_local` host, `migrate()` inside `connect()`,
`SECOND_HANDLE` declined, a generic decline reason, and `MID_BATCH_FAULT` plus all three
ceilings inherited.

| AC | Test | Red | Green |
| --- | --- | --- | --- |
| AC-001 | `tests/fixture_contract.rs::on_the_object::the_host_is_reachable_from_an_integration_test` | `error[E0432]: unresolved import happenstance_cloudflare::host — could not find host in happenstance_cloudflare`, with the module still `#[cfg(all(test, target_arch = "wasm32"))]`. This is EC-005's failure mode, observed from the boundary the slice-mate sits on | `pub mod host` (`src/lib.rs:281`); the test drives one append through `CloudflareEventStore::new(host.storage())` and reads it back |
| AC-002 | `tests/fixture_contract.rs::the_fixture_store_is_not_send` | — passed from the first run, and that is the correct outcome for a standing detector: this criterion is *non-regression*, and its own positive control (`SequencePosition` is `Send`) is what makes the negative half mean anything | unchanged and green, alongside the four `src/lib.rs` probes on both targets |
| AC-003 | `::on_the_object::two_instances_alive_at_once_observe_none_of_each_others_appends` | `assertion left == right failed` — both instances read both appends, because every instance was handed one shared `DurableObjectHost` | `CloudflareFixture::new` stands up its own object (`tests/support/mod.rs:109`) |
| AC-004 | `::on_the_object::two_handles_from_one_instance_observe_each_others_appends`, `::migrate_runs_once_per_instance_not_per_connect`, `::a_second_handle_does_not_re_mint_the_store_id` | the migration count came back `3` against a required `1`; the two-handle case failed on events leaking in from the shared object; `SECOND_HANDLE` declined additionally failed `the_fixture_expression_satisfies_the_macro_arm` | `migrate()` moved into `new()`, `SECOND_HANDLE = SUPPORTED`, `connect()` a `SqlStorage` clone |
| AC-005 | `::every_declined_capability_names_this_runtime` | `SECOND_HANDLE is declined with 'not decided yet', which says *that* this fixture cannot rather than *why this runtime* cannot` | one declined capability, `MID_BATCH_FAULT`, with a reason about this host's throwing statement and CF-39's obligation |
| AC-006 | `::mid_batch_fault_is_restated_not_inherited`, `::on_the_object::a_supported_capability_has_its_method_overridden` | `assertion left != right failed`, both sides printing the trait's default sentence verbatim | `MID_BATCH_FAULT` restated; `REOPEN = SUPPORTED` with a `reopen()` override the supported-capability case then exercises |
| AC-007 | `::the_fixture_expression_satisfies_the_macro_arm`, `::the_three_store_limits_are_stated_here` | ``MAX_EVENT_DATA_LEN is not written in tests/support/mod.rs, so this fixture inherited it from the trait`` | three `const … : Option<usize> = None` written out, each naming `measured-store-limits` as the owner of the value |

Two additional guards were written that no AC demanded and that a reviewer should read as
load-bearing rather than as padding:

- `::on_the_object::the_hosts_arming_hook_fires_once_and_disarms` is the control for the
  host's new `arm_throw_after` seam. A fault seam nobody has watched *fail* is
  indistinguishable from a no-op, and this one is what `measured-store-limits` will build
  CF-39's verdict on.
- `xtask/src/proof.rs::tests::each_unit_row_is_gated_by_the_spelling_its_own_tree_uses`,
  including its negative half. Without it, `WASM32_TARGET_GATE` could silently become a
  copy of `WASM32_TEST_GATE` and both rows' `source_dir` guards would keep passing on the
  strength of the *other* row's tree.

## Commits

One checkpoint, on `initiative/from-contract-to-published-library`, not pushed:

- `feat(cloudflare-durable-object-store): The Durable Object host and the conformance fixture`,
  carrying the trailer `Story: cloudflare-durable-object-store/durable-object-host-and-fixture`
  and this report's own body.

**The SHA is deliberately not transcribed into this paragraph, and that is not an
omission.** This report is *inside* the commit it would be naming, so any number written
here is the number of a commit that no longer exists the moment the write lands — a
self-reference that is wrong by exactly one revision, every time. The two lookups that
cannot go stale are the trailer and the item card:

```console
git log --grep "Story: cloudflare-durable-object-store/durable-object-host-and-fixture"
```

and `story.md`'s `links.commits`, which `redkiln record-links` fills once the checkpoint has
settled. The slice digest returned by this run carries the settled SHA as well.

## Changes

| Path | Shape of the change |
| --- | --- |
| `crates/happenstance-cloudflare/src/host.rs` | Renamed from `src/test_object.rs`. New `DurableObjectHost` — the object held so its storage can be bound *again*, which is the reopen seam — plus `arm_throw_after`, and `pub` on `durable_object`, `arm_throw`, `arm_throws`, `statements`. The JS shim's `armThrow` gains a fourth `skip` parameter so a fault can fire on the k-th matching statement rather than the first. Module docs rewritten to state what the visibility does and does not promise |
| `crates/happenstance-cloudflare/src/lib.rs` | `pub mod host;` replaces `#[cfg(all(test, target_arch = "wasm32"))] mod test_object;`, with the reasoning inline; four `use crate::test_object::…` updated; the crate-level status paragraph corrected to say the host exists and the suite has still not run |
| `crates/happenstance-cloudflare/src/sql_storage.rs` | `storage_from_durable_object_state` loses its `#[cfg(all(test, …))]`, because a non-test module now calls it |
| `crates/happenstance-cloudflare/tests/support/mod.rs` | **New.** `CloudflareFixture` and `impl Fixture` — the five constants, `connect()`, the `reopen()` override, and the documented non-invocation of the concurrency and model families |
| `crates/happenstance-cloudflare/tests/fixture_contract.rs` | **New.** Twelve cases: five plain `#[test]`s about what the fixture *declares*, seven `#[wasm_bindgen_test]`s about what it *does* |
| `crates/happenstance-cloudflare/Cargo.toml` | `happenstance-testkit`, `happenstance-core` and `futures-core` as **host** dev-dependencies (an integration target is a second compilation unit and does not inherit the library's `[dependencies]`); `wasm-bindgen-test` stays target-scoped. Nothing added to `[dependencies]` |
| `xtask/src/proof.rs` | A second `WASM_UNIT_TARGETS` row for `--test fixture_contract`, so the seven `wasm32` cases are executed by the gate rather than merely compiled. `WasmUnitTarget` gains a `gate` field and `WASM32_TARGET_GATE` is added beside `WASM32_TEST_GATE`; one new unit test with a negative half |
| `standards/rust/52-wasm32-and-target-cfg.md` | One evidence citation repointed — see **Notes** |

## Gates

Scoped to the project's affected packages throughout; no unfiltered whole-repo script was
run.

| Command | Result |
| --- | --- |
| `cargo test -p happenstance-cloudflare --test fixture_contract` | 5 passed (host-native half) |
| `CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner cargo test -p happenstance-cloudflare --test fixture_contract --target wasm32-unknown-unknown` | 7 passed |
| `cargo clippy -p happenstance-cloudflare -p happenstance-testkit -p xtask --all-targets --all-features -- -D warnings` | clean |
| `cargo clippy -p happenstance-cloudflare --all-targets --target wasm32-unknown-unknown -- -D warnings` | clean |
| `cargo test -p happenstance-cloudflare -p happenstance-testkit -p xtask --all-features` | all targets green |
| `cargo run -p xtask -- wasm-conformance` | every registered `wasm32` target executed, including the new row: `happenstance-cloudflare/--test fixture_contract: 7 tests listed, 7 named, executing on wasm32-unknown-unknown` |
| `cargo run -p xtask -- lints` | six file-reading checks green, including the constitution's 27 atoms |
| `cargo run -p xtask -- spec-trace` | 201 clauses, 401 citations, no problems (the two pre-existing unclaimed rules are unchanged and are not this story's) |
| `cargo doc -p happenstance-cloudflare --no-deps` | clean |
| `cargo fmt --all --check` | clean (run last, after every edit) |

The spec's own **Risks** section asked for one thing explicitly: run the fixture target for
the `wasm32` target locally before merge, because the `wasm32 build of the Cloudflare
adapter` step is a `cargo check`. It was run, and it is now more than a local courtesy —
the `xtask` row means the gate runs it too.

## Notes

**One gate-compelled repair outside the story's own files, and it is the shape the
`real-worker-bindings` fence amendments described.** `standards/rust/52-wasm32-and-target-cfg.md`
cited `crates/happenstance-cloudflare/src/lib.rs:256 (Left un-gated it is dead code on
wasm)` — a comment that existed *because* `mod test_object` was `#[cfg]`-gated, and that
this story's promotion necessarily deletes. `cargo xtask lint-constitution` is a gate step,
so leaving the citation stale is red and there was no third option. The evidence row is
repointed at a live instance of the **same** rule in the same crate:
`crates/happenstance-cloudflare/tests/support/mod.rs:106`, where `issued_statements` carries
`#[cfg(target_arch = "wasm32")]` with the `dead_code`-is-denied reasoning attached. RS-52-3's
normative text is untouched; nothing in `spec/SPECIFICATION.md` was edited.

**A deviation from the spec's tier table, taken deliberately and worth ratifying.** The spec
contemplated `fixture_contract.rs` being emitted wholly as `#[wasm_bindgen_test]` "where
`worker`'s linking forces it". It does not force it for the five criteria that only *read*
associated constants, so those are plain `#[test]`s and `happenstance-testkit` is a **host**
dev-dependency rather than a target-scoped one. The trade: an ordinary
`cargo test -p happenstance-cloudflare` — no wasm toolchain, no runner — now proves AC-002,
AC-005, AC-006's restatement half and AC-007. Against that, `cargo deny` and the feature
powerset see one more dev-edge to an existing workspace member, and `[dependencies]` is
untouched, so nothing reaches a consumer. This also pre-satisfies `measured-store-limits`'
"Unit (host-native)" tier, which asked for exactly this and which a target-scoped testkit
would have made impossible.

**`SqlStorage` was not touched, and `with_ceilings` was not reached for.** The fixture builds
its store through `CloudflareEventStore::new(sql)` and nothing else, which is what keeps the
"one instance, one object" invariant on a single code path. `with_ceilings` remains
`#[cfg(all(test, target_arch = "wasm32"))] pub(crate)` and is unreachable from `tests/` —
correctly, because the ceilings a *fixture* declares must be ceilings the *adapter* keeps,
and wiring them through a test-only seam would have made the promise true only under test.
That is `measured-store-limits`' problem to solve in the right place.

**No `.kb/` file was written, no clause was amended, and no conformance rule was added.**
CF-40's ownership is named as a hand-off in the fixture's own doc comments and settled
nowhere.
