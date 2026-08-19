---
item: "HS-S0052"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — The ES-6 artefact: constraint violation versus transport fault, recovered by a caller

**All seven ACs are satisfied.** ES-6's information half now has an artefact in the same
file as its auto-trait half, and both answer to one command. Nothing is blocked, nothing is
deferred, no conformance rule was added, no file under `.kb/` was written, and
`spec/SPECIFICATION.md` is not in the diff — ES-6 is discharged with evidence rather than
amended.

**Three findings, each of which could have gone the other way.**

- **The fact was already reachable, so no accessor was added.** EC-003 provided for the
  minimal `pub` accessor if the real `worker` bindings left the distinction unreachable from
  outside the crate. They did not: `AppendError`'s arms, `is_condition_violated`,
  `CloudflareEventStoreError`'s variants, `SqlError::Thrown`, and `Display` plus the
  `core::error::Error::source` chain are all a downstream consumer needs. **This diff adds no
  public item at all**, which is the strongest available answer to D7 — the design records no
  surface, and that is permission for nothing.
- **The tier question resolved to the EC-004 branch, and for a stronger reason than a link
  error.** `worker`'s bindings resolve to panicking stubs off the target, so a
  `CloudflareEventStore` cannot be *driven* on the host at all. The reconstruction is
  therefore `wasm32`, on the same tier the two slice-mates already put their targeted tests.
  What did **not** happen is the forbidden move: no probe was deleted or relocated. The four
  `!Send` assertions are written once and wrapped twice, so a contributor with no wasm
  toolchain still runs `the_probe_is_not_vacuous` under a plain `cargo test`.
- **ADR-0009's decision holds, and this adapter is the evidence for it rather than the
  exception to it.** A caller recovers conflict-versus-transport from what `append` hands
  back without `Error` carrying `Send + Sync`. EC-006's refutation branch did not fire. The
  verdict is recorded in the crate documentation; the atom is
  `adr-0023-and-atom-resolutions`'.

## TDD Evidence

Every test executes on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner`. **74
tests, 0 failures** — 69 inherited from the two slice-mates, 5 new.

**The Red step here had to be a mutation sweep, and that is a property of the story rather
than a shortcut.** The behaviour under test — the classifier — landed with
`durable-object-write-path`, which this story `depends_on` and merges behind; EC-005 forbids
stubbing one or asserting against a `todo!()` body. So the question a Red step has to answer
is not "does the body exist" but **"can this artefact fail?"** — which is AC-004's whole
subject and the reason the spec asks for a named wrong shape. Two wrong implementations were
compiled into the *real* classifier (`event_store::classify_write`) one at a time and the
suite re-run.

| Mutant | Rejected by |
| --- | --- |
| The evidence-discarding classifier (D3) — the right `AppendError` arm, the thrown value replaced by a contentless error. The subtle one a careful implementer reaches honestly, and it passes every other check in this repository | `transport_fault_reaches_the_caller_distinguishably`, `an_evidence_discarding_classifier_is_rejected` (2 failed / 72 passed) |
| The flattening classifier — every throw becomes `AppendError::Store`, so a conflict reaches the caller as a transport fault and their retry loop never terminates | `constraint_violation_reaches_the_caller_as_condition_violated`, `the_distinction_is_reachable_from_outside_the_crate`, and the slice-mate's `write_path_tests::a_unique_constraint_message_classifies_as_condition_violated` (3 failed / 71 passed) |

Both were reverted and the suite is green: 74 passed, 0 failed.

| AC | Test | Red → Green |
| --- | --- | --- |
| AC-001 | `es6_reconstruction::constraint_violation_reaches_the_caller_as_condition_violated` | RED via the flattening mutant |
| AC-002 | `::transport_fault_reaches_the_caller_distinguishably` | RED via the evidence-discarding mutant |
| AC-003 | `::the_distinction_is_reachable_from_outside_the_crate` | Compile-time: `classify_like_a_consumer<S: EventStore>` binds the bare flavour and reads only `pub` items. RED at runtime via the flattening mutant too |
| AC-004 | `::an_evidence_discarding_classifier_is_rejected` | The control itself. It asserts the shared predicate returns **false** against the wrong shape, and the same predicate returns **true** against the real error — so the pairing is what carries it |
| AC-005 | documentation, checked by `cargo doc` under `-D warnings` and by `git status -- .kb/` being empty | — |
| AC-006 | the four `!Send` probes on the host and `wasm_tests::*` plus the five new tests on `wasm32` | — |
| AC-007 | the four probes, `send_shape`'s two doctests, `spec-trace`, `redkiln validate --kb` | — |

One more test rides with them, covering **EC-001** rather than an AC:
`an_unclassifiable_throw_still_says_what_happened`. When the classification probe cannot
reach a live JS value, the caller must not be handed a fabricated "definitely not a
conflict" — the failure still arrives on the `Store` channel carrying what the store said.
`unwrap_or(false)` in `classify_write` is correct precisely because it preserves the thrown
value on that channel rather than discarding it; the test pins that reading so a future
refactor cannot quietly turn it into the other one.

## Commits

- `caller-visible-error-verdict` — see the story checkpoint commit carrying
  `Story: cloudflare-durable-object-store/caller-visible-error-verdict`.

## Changes

| Path | Shape of the change |
| --- | --- |
| `crates/happenstance-cloudflare/src/lib.rs` | `mod es6_reconstruction` — five `#[wasm_bindgen_test]`s, one shared read-only predicate (`what_the_store_said` / `names_the_underlying_failure`), and a module doc that states what a caller is deciding and why this cannot be a conformance rule. Findings 2 and 3 gain an observational paragraph each; a new `# The ES-6 verdict` section after finding 4 records the outcome and names where the atom is minted |
| `crates/happenstance-cloudflare/src/test_object.rs` | `arm_throw` and the shim's one-shot `armThrow` hook: a constructed `new Error(message)` delivered down the production path rather than a mocked classifier |
| `standards/rust/50-dependency-hygiene.md`, `52-wasm32-and-target-cfg.md`, `61-compile-time-assertions.md` | Seven `file:line` citations into `src/lib.rs` re-pointed after the documentation edits moved them |
| `.bklg/.../caller-visible-error-verdict/_ledger.md` | Seven rows flipped with cited evidence |

**Not touched, and each absence is a decision:** `crates/happenstance-cloudflare/Cargo.toml`
(the wasm32 dev-dependency `worker-binding-layer` added already covers this tier, so NF-002's
"no new dependency" holds without qualification); `crates/happenstance-core/` in its entirety;
`.kb/` in its entirety; `spec/SPECIFICATION.md`; `crates/happenstance-testkit/`;
`xtask/src/main.rs`'s step arrays.

## Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all -- --check` | green |
| `cargo clippy -p happenstance-cloudflare --all-targets --all-features -- -D warnings` | green |
| `cargo clippy -p happenstance-cloudflare --all-targets --target wasm32-unknown-unknown -- -D warnings` | green |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-cloudflare --no-deps --document-private-items` | green |
| `cargo test -p happenstance-cloudflare --target wasm32-unknown-unknown --lib` | **74 passed, 0 failed** |
| `cargo test -p happenstance-cloudflare` (host) | 4 unit + 2 doctests, all pass |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` |
| `cargo xtask spec-trace` | `traceability: no problems found` — 201 clauses, 401 citations |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` |
| `redkiln validate --kb` | `validate passed` |

## Notes

**Why the thrown value is constructed but not the path it travels.** D5 asked for a
constructed thrown value rather than a live Durable Object race, so that DoD 4 — the one exit
criterion the runbook says has no artefact behind it — does not end up waiting on this
project's riskiest infrastructure. The shim's one-shot `armThrow` gives exactly that and
gives it more cheaply than a mocked `SqlStorage` would: the value is `new Error(message)` on
the JS side, and everything after it is production code — `worker`'s real `wasm-bindgen`
externs, `SqlError::from_worker`, `JsThrow::is_constraint_violation`, `classify_write`,
`EventStore::append`. Nothing between the throw and the caller is doubled, and no `workerd`
runner is involved.

**Why the arming disarms itself.** One arming is one throw, so the same store can be shown
failing and then succeeding in one test — which is what
`the_distinction_is_reachable_from_outside_the_crate` needs to return all three of "rebuild",
"retry" and "accepted" against one object. A latched throw would have made every test build a
fresh store and would have hidden the fact that recovery is possible at all.

**One shared predicate, and the reason it is a named function.** Both directions read the
error through `what_the_store_said`, which walks `Display` plus the public `source` chain and
nothing else. Inlining it into each test would have let one of them drift toward a private
field without the other noticing — and a test that reaches inside keeps passing after the
information stops being recoverable, which is the exact regression this artefact exists to
catch. Factoring it out is also what makes AC-004 mean anything: the control asserts the
*same* function returns `false`.

**`is_condition_violated()` rather than a `matches!` on the arm, in AC-001.** It is what a
caller actually writes, and asserting on it keeps the test in the register the spec asks for
— written from the consumer's seat, not the adapter's. AC-002 does assert the `Store` arm's
shape, because there the shape *is* the claim: the thrown value is still in there.

**The constitution citations.** Rewriting the crate documentation moved seven `file:line`
citations in `standards/rust/`, and `cargo xtask lint-constitution` fails on a citation that
has drifted more than ten lines. Re-pointing them is forced by this diff rather than scope
drift; leaving them would have been a silent rot of exactly the kind that file exists to
prevent.

**What this story deliberately did not do.** `store_error_crosses_a_join_handle` — ES-6's own
named rule — is still unwritten and unowned, and writing it is a testkit change against
ADR-0009's marker rather than this story's work
(`.kb/open-questions/es-6-names-an-unwritable-rule.md`). The portable half of the
distinction is already covered by `ViolationAsStoreErrorStore` in the testkit's mutation
coverage. What was left is unportable by construction — a property of *this* error type's
contents — and that asymmetry is itself why the workspace's only `!Send` adapter is the only
instrument for the clause.
