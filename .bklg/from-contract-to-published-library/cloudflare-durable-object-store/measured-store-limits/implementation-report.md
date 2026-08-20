---
item: "HS-S0055"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — The fixture's numeric limits are this adapter's declared refusal policy

> **Amended 2026-08-19, slice repair pass.** Two claims were withdrawn on review
> and are corrected here rather than footnoted. **The title said
> *measurements, not guesses*, and the three ceilings are neither**: they are
> this adapter's own declared refusal policy, seeded from Cloudflare's documented
> 2 MiB row cap, because no physical wall was observable on the executing host —
> which the report already said in *The one thing a reviewer must read before the
> numbers* and the changelog then contradicted. And **CF-39's `MID_BATCH_FAULT`
> was claimed on a mechanism belonging to the JavaScript host this crate ships
> for its own tests**, not to the store; the repair pass replaced it with a real
> SQLite trigger on the `event` table, which is what CF-39 names as its exemplar
> and what survives a swap of the runtime. **Ten of eleven ACs are satisfied as
> written; AC-001's *never read off a platform page* is not — 1 MiB is the
> documented 2 MiB row cap halved and then confirmed accepted, because there is
> no wall on this host to locate — and AC-002's and AC-003's positioning carries
> the same seed.** The blocking finding in
> `../every-rule-under-workerd/implementation-report.md` is the shared cause: no
> `workerd`-class runner exists inside `cargo xtask ci`, so the platform's real
> caps are unobservable from here.

**The observable this story is judged by is an
absence:** the Cloudflare conformance row now prints **no `SKIP` line at all**.
Before this story it read 89 passed with three skips — `NO_STORE_LIMITS` on
`append_reports_exceeded_store_limits`, and `MID_BATCH_FAULT` on both
atomicity-under-fault rules. It now reads:

```text
happenstance-cloudflare/durable_object_conformance: 89 rules enumerated, 10 named, executing on wasm32-unknown-unknown
test result: ok. 89 passed; 0 failed; 0 ignored; 0 filtered out; finished in 0.31s
```

Three rules that skipped in every fixture in the workspace now run somewhere, and
this adapter is where. `MID_BATCH_FAULT` is claimed for the **first time
anywhere** — new coverage for the conformance *suite*, not only for this adapter.

That absence has a cost the repair pass paid: with nothing declined, the gate
emitted **zero** `SKIP` lines, so CF-18's reporting path — project AC-003's
*emits* half — had no live instance left anywhere. It is standing again, in
`tests/fixture_contract.rs`, over a `DecliningFixture` handed to two shipped
capability-gated rules; see the sibling report's *The skip lines, verbatim*.

## The one thing a reviewer must read before the numbers

**The probe did not find a wall, and this report is not going to pretend it did.**

Phase A of the measurement drives statements through `SqlStorage::exec` against
the adapter's own schema with the adapter's ceiling check deliberately bypassed.
It found payloads accepted and read back byte-for-byte at 8 MiB, 16,384
`event_tag` rows for one event, and 8,192 consecutive inserts in one turn — no
refusal anywhere inside four times the documented row cap. The executing runtime
is real SQLite reached through `worker`'s real bindings, but the host is a Node
process rather than `workerd`, and it does not enforce the platform's documented
caps.

So the three declared numbers are **not search results**. They are the adapter's
own **refusal policy** — derived as *documented platform cap minus this adapter's
measured overhead*, and confirmed accepted on the executing runtime — and the
fixture's doc comments, the crate documentation, the experiment's README and the
evidence package all say exactly that. CF-40 requires a declared value to be
accepted and one more refused; it does not require the declaration to be the
physical maximum, and the spec's own clarification 5 blesses a conservative
stated ceiling over an unstable exact one.

What *was* measured, and what the declarations rest on:

| | Declared | Measured |
| --- | --- | --- |
| `MAX_EVENT_DATA_LEN` | 1,048,576 (1 MiB) | accepted and read back as exactly 1,048,576; 1,048,577 refused as `ExceedsStoreLimit(EventDataLen)` |
| `MAX_TAGS_PER_EVENT` | 1,024 | 1,024 tags → exactly 1,024 `event_tag` rows, 139,264 bytes of storage; 1,025 refused as `ExceedsStoreLimit(TagsPerEvent)` |
| `MAX_EVENTS_PER_BATCH` | 1,024 | a 1,024-event batch issues **2,055** statements and completes; 1,025 refused as `ExceedsStoreLimit(EventsPerBatch)` |

And the one falsifier that was tested rather than assumed: **CF-40's own
`[PROVISIONAL]` falsifier did not fire.** A 2 MiB row is accepted both on a fresh
object and on one already holding a megabyte, so the boundary is a constant and
the cumulative `SqlError::StorageLimitExceeded` cap was not mistaken for the
per-value one.

## TDD Evidence

| AC | Test | Red | Green |
| --- | --- | --- | --- |
| AC-004 | `tests/fixture_contract.rs::the_three_store_limits_are_measured_not_defaulted` | `MAX_EVENT_DATA_LEN is None, which says this store has no ceiling on that value. It is not true of a store whose backing API carries SqlError::StorageLimitExceeded …` | all three `Some`; the `NO_STORE_LIMITS` skip line is gone from the run |
| AC-001–003 | `dcb_conformance_wasm::append_reports_exceeded_store_limits` | **skipped**, which is the failure — a rule that certifies nothing while printing an honest-looking line | Ran and passed, all three arms |
| AC-005 | `::no_declared_ceiling_is_below_its_floor` | vacuous while every constant was `None` (its `let Some(…) else { continue }` is what made it so) and load-bearing the moment numbers landed | 16×, 16×, 8× above the floors; the three guaranteed-minimum rules green in the same run |
| AC-008 | `dcb_conformance_wasm::arming_a_mid_batch_fault_makes_the_append_fail` | **the negative control**, run deliberately: arm removed, override left in place — the registered `NoopFaultFixture` shape — and the rule went red with `… armed a fault at row 2 of a three-event batch and the append succeeded … Got Ok(SequencePosition(6))` | arm restored; both fault rules Ran and passed |
| AC-009 | `::acknowledged_writes_survive_a_reopen`, `::reopened_store_does_not_reissue_an_event_id`, `::recorded_time_survives_a_reopen` | — a *confirmation* rather than a change; the verdict was declared by HS-S0053 and this story's job was to find out whether it survived | all three Ran and passed, the first time any of them has executed anywhere |
| AC-006, AC-007 | `experiments/durable-object-limits/` | the first cut of the probe had only phase B, and it **stopped being a measurement** the moment the adapter declared its ceilings — it reported 1 MiB / 1,024 / 1,024, which is the declaration read back to itself | split into phase A (runtime, adapter policy bypassed) and phase B (declaration, both directions); two runs byte-identical |
| AC-010 | `cargo xtask lint-changelog`, `cargo doc` | — | CF-29-shaped entry; the limits table in the crate docs |
| AC-011 | `git status`, `spec-trace` | — | `_evidence.md` §1–§8; no `.kb/**` path |

## Commits

One checkpoint, on `initiative/from-contract-to-published-library`, not pushed:

- `feat(cloudflare-durable-object-store): The store limits, measured rather than guessed`,
  carrying the trailer `Story: cloudflare-durable-object-store/measured-store-limits`
  and this report's own body.

The SHA is not transcribed here, for the reason the two sibling reports give: this
file is inside the commit it would name. `git log --grep "Story:
cloudflare-durable-object-store/measured-store-limits"` is the lookup that cannot
go stale.

## Changes

| Path | Shape of the change |
| --- | --- |
| `crates/happenstance-cloudflare/src/event_store.rs` | `Ceilings::MEASURED` replaces `Ceilings::UNMEASURED` as what `new()` carries, with the derivation of each number on the constant. `UNMEASURED` stays, now `#[cfg(all(test, target_arch = "wasm32"))]`, because it is the base every `with_ceilings` test builds from and inheriting production numbers there would make each of those tests depend on a value it is not about |
| `crates/happenstance-cloudflare/tests/support/mod.rs` | The three `Option<usize>` constants become `Some(…)`, each with its derivation and the *stated ceiling* caveat; `MID_BATCH_FAULT` flips to `SUPPORTED` with the mechanism stated and an `arm_mid_batch_fault` override |
| `crates/happenstance-cloudflare/tests/fixture_contract.rs` | Two new host-native cases: `the_three_store_limits_are_measured_not_defaulted` and `no_declared_ceiling_is_below_its_floor` |
| `crates/happenstance-cloudflare/src/host.rs` | Nothing new at the time — `arm_throw_after` and the shim's `skip` parameter landed in `durable-object-host-and-fixture`, precisely so this story would inherit a working seam and a control test for it. **The 2026-08-19 slice repair pass removed both** and rewrote the module docs: CF-39's fault is now a real SQLite trigger armed by the fixture, and the docs state what is still owed to `workerd` and who owns it |
| `crates/happenstance-cloudflare/src/lib.rs` | VT-21's obligation: a table of the three declared limits, what each is refused as, and the *stated ceiling* finding |
| `experiments/durable-object-limits/**` | **New.** `Cargo.toml` (outside the workspace, bare `[workspace]`), `tests/boundaries.rs` (M1–M5, two phases), `README.md`, `results/run-1.txt`, `results/run-2.txt` |
| `CHANGELOG.md` | A CF-29-shaped entry naming the defect each newly-reachable rule detects |
| `.bklg/…/measured-store-limits/_evidence.md` | **New.** The hand-off package for `adr-0023-and-atom-resolutions` |
| `standards/rust/25-…, 50-…, 52-…, 61-…` | Seven `file:line` citations repointed — see **Notes** |

`crates/happenstance-testkit/**` is untouched. No conformance rule, no mutant, no
clause, no `.kb/` write, no `xtask` change.

## Gates

| Command | Result |
| --- | --- |
| `cargo run -p xtask -- wasm-conformance` | 89 + 89 + 17 + **89** + 81 + 7, all green; **zero** `SKIP` lines on the Cloudflare row |
| `cargo test -p happenstance-cloudflare --test fixture_contract` | 7 passed (host-native) |
| `cargo test -p happenstance-cloudflare -p happenstance-testkit -p xtask --all-features` | green |
| `cargo clippy -p happenstance-cloudflare -p happenstance-testkit -p xtask --all-targets --all-features -- -D warnings` | clean |
| `cargo clippy -p happenstance-cloudflare --all-targets --target wasm32-unknown-unknown -- -D warnings` | clean |
| `cargo run -p xtask -- lints` | six checks green, including `CF-29: all 112 rules in 4 file(s) have a changelog entry`; 27 constitution atoms consistent |
| `cargo run -p xtask -- spec-trace` | `traceability: no problems found` |
| `cargo doc -p happenstance-cloudflare --no-deps` | clean |
| `cargo fmt --all --check` | clean, run last |
| `experiments/durable-object-limits` — two runs | byte-identical (`diff` clean) |

**NF-002, measured.** The Cloudflare conformance row went from 0.18 s to 0.31 s.
That is the rule's own three boundary appends — a 1 MiB payload, 1,024 tags, a
1,024-event batch — and nothing else; the search never runs in CI.

## Notes

**One `#[allow(dead_code)]` was added, and it is the price of a shared
`tests/support` module rather than a shortcut.** A support module is compiled once
per *declaring target*, so `CloudflareFixture::issued_statements` is live in
`fixture_contract` and dead in `durable_object_conformance`, from one source.
`expect` is wrong for exactly that reason — it would fire in the target where the
method is used. The alternatives are a second support module duplicating the
fixture, or moving the method into its caller and losing the encapsulation of
`sql`. The reasoning is written at the attribute.

**Seven more `standards/rust/` citations were repointed**, for the third time in
this slice and for the same mechanical reason: the VT-21 limits table added to
`src/lib.rs` moves every line below it, and four constitution atoms cite that file
by `file:line`. Only line numbers changed. This is now a standing cost of editing
`crates/happenstance-cloudflare/src/lib.rs`'s header, and it is worth saying out
loud that a `file:line` citation into a heavily-documented crate root is a
maintenance edge the constitution's own anchor mechanism was not designed for.

**What was measured on a Node host and what would change under `workerd`.** The
finding in `_evidence.md` §2 is the honest limit of this measurement, and it is
the one thing a `workerd`-class runner would change: phase A would produce real
numbers, and the three declarations would either be confirmed conservative or
moved. Both events are loud rather than silent, because
`append_reports_exceeded_store_limits` fails in one direction or the other.

**Scope kept.** No new conformance rule and no new mutant: the `None`-declaration
defect is in a *fixture's declaration*, which no store can fail, so `CLAUDE.md`'s
rule forbids a suite rule for it. The mechanical guard is the adapter-local
`the_three_store_limits_are_measured_not_defaulted`; the human guards are the
CF-29 changelog entry and `_evidence.md`. CF-40's ownership is **narrowed and
handed on**, not minted — `.kb/open-questions/cf-40-fixture-limits-ownership.md` is
unchanged and `sqlite-durable-store` closed without minting an answer, so
`adr-0023-and-atom-resolutions` is free to mint it once.
