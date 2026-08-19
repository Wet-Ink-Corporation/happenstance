---
item: "HS-S0053"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — A Durable Object host and the CloudflareFixture mounted on it

## Findings Ledger

**Outcome: seven of seven ACs satisfied. Nothing blocked, nothing deferred, no conformance
rule added or changed, no clause amended, no `.kb/` write.** One repair outside the story's
own files was compelled by a gate step and is flagged below for ratification; one deliberate
deviation from the spec's tier table is flagged for the same reason.

| AC | Result | Proved by | Mounted into |
| --- | --- | --- | --- |
| AC-001 | satisfied | `tests/fixture_contract.rs::on_the_object::the_host_is_reachable_from_an_integration_test`, executed by `cargo run -p xtask -- wasm-conformance`. Red beat: `E0432 could not find host in happenstance_cloudflare` | `crates/happenstance-cloudflare/src/lib.rs:281` (`pub mod host`); `xtask/src/proof.rs` `WASM_UNIT_TARGETS` row 2 |
| AC-002 | satisfied | `tests/fixture_contract.rs::the_fixture_store_is_not_send` (with positive control) + the four standing probes in `src/lib.rs`, host and `wasm32` | `crates/happenstance-cloudflare/tests/support/mod.rs:145-148` |
| AC-003 | satisfied | `::on_the_object::two_instances_alive_at_once_observe_none_of_each_others_appends` — both instances constructed before either appends. Red beat: a shared `thread_local` host failed it | `tests/support/mod.rs:109`; `src/host.rs:236` |
| AC-004 | satisfied | `::two_handles_from_one_instance_observe_each_others_appends`, `::migrate_runs_once_per_instance_not_per_connect` (host statement log, count == 1), `::a_second_handle_does_not_re_mint_the_store_id`. Red beat: migration count 3 | `tests/support/mod.rs:157`, `:215`, `:111`; `src/event_store.rs:227`, `:284` |
| AC-005 | satisfied | `::every_declined_capability_names_this_runtime`. Red beat: `declined("not decided yet")` rejected by name | `tests/support/mod.rs:184` |
| AC-006 | satisfied | `::on_the_object::a_supported_capability_has_its_method_overridden` (reaches the provided body's panic path for every supported capability), `::mid_batch_fault_is_restated_not_inherited`. Red beat: the inherited default printed identical on both sides | `tests/support/mod.rs:169`, `:224`, `:184` |
| AC-007 | satisfied | `::the_fixture_expression_satisfies_the_macro_arm` (the macro arm's own `impl AsyncFn() -> F` bound), `::the_three_store_limits_are_stated_here` (reads the impl's source). Red beat: absent constants rejected by name | `tests/support/mod.rs:205`/`:209`/`:213`; consumed at `crates/happenstance-testkit/src/lib.rs:494-521` |

### The two capability verdicts this story states, and what would overturn each

`REOPEN` is **`SUPPORTED`**, and it is a stronger answer than the spec expected. The host
type is why: a Durable Object's `state` outlives any binding taken off it, so
`DurableObjectHost::storage()` called twice is two bindings onto one object, and replacing
the fixture's binding is precisely "discard every outstanding handle's process-level state"
with the rows untouched. Three suite rules — `acknowledged_writes_survive_a_reopen`,
`reopened_store_does_not_reissue_an_event_id`, `recorded_time_survives_a_reopen` — will
therefore **run** rather than skip. If the executed suite disagrees, the correction is a
one-line constant change plus deleting one override, by design.

`MID_BATCH_FAULT` is **declined**, with the mechanism named rather than denied. The host
*can* throw on a chosen statement, and `arm_throw_after` was added in this story precisely so
that a fault can fire on the k-th matching statement rather than the first. What is not
settled here is CF-39's actual requirement — naming *which* statement of this adapter's write
path is the k-th row's — and that is a verdict owed to an executed run.
`measured-store-limits` inherits a working seam, its control test
(`::the_hosts_arming_hook_fires_once_and_disarms`) and a one-line constant.

### For the reviewer to ratify

1. **One `standards/rust/` citation was repaired.**
   `52-wasm32-and-target-cfg.md`'s RS-52-3 evidence row cited a `src/lib.rs` comment that
   existed *because* the host module was `#[cfg]`-gated, and the promotion deletes it.
   `lint-constitution` is a gate step, so the citation had to move; it now points at
   `crates/happenstance-cloudflare/tests/support/mod.rs:106`, a live instance of the same
   rule in the same crate. No normative text changed. This is the fence shape the
   `real-worker-bindings` amendments established, and the fence for this story should be
   widened to `standards/rust/**` and `xtask/src/proof.rs` on the same reasoning.
2. **`happenstance-testkit` is a *host* dev-dependency, not a target-scoped one.** The spec
   and the slice-mate's spec both contemplated target-scoping it. Host-scoping is what lets
   the five declaration-level criteria run in an ordinary `cargo test` with no wasm
   toolchain, and it is what `measured-store-limits`' own "Unit (host-native)" tier asks
   for. `[dependencies]` is untouched and nothing reaches a consumer.
3. **The host is `pub` on both targets.** It is a test-and-example surface, said so in its
   own module documentation and in the crate's. `publish = false` still stands and
   `publish-ready-crate` owns whether it travels.

### Not claimed

The suite has **not** been pointed at this fixture in this story — that is
`every-rule-under-workerd`'s, and until it has run, this adapter is an implementation rather
than a conformant one. The three ceilings are `None`, stated deliberately, and that is
**not** a discharge of CF-40. No rule of the event-store family was executed against
`CloudflareFixture` here.
