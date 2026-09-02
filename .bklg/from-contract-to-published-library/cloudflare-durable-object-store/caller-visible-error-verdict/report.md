---
item: "HS-S0052"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — The ES-6 artefact: constraint violation versus transport fault, recovered by a caller

## Findings Ledger

**Outcome: seven of seven ACs satisfied. Nothing blocked, nothing deferred, no conformance
rule added, no file under `.kb/` written, no accepted atom edited, and no `[FROZEN]` clause
amended.** ES-6 is discharged with evidence: `spec/SPECIFICATION.md` is not in this diff at
all, and `cargo xtask spec-trace` is green over its markers, its `Rule:` line and its
citations into `src/js.rs`.

**The verdict, since the story owes one either way: ADR-0009's decision holds, and this
adapter is the evidence for it rather than the exception to it.** A caller recovers the one
fact they must branch on — conflict versus transport fault — from what `append` hands back,
without `Error` carrying a `Send + Sync` bound. EC-006's refutation branch did not fire. The
verdict is written into the crate documentation where a reader meets it; the atom naming the
alternatives that lost is `adr-0023-and-atom-resolutions`', minted through
`/redkiln:kb-ingest`.

| AC | Result | Proved by | Mount point |
| --- | --- | --- | --- |
| AC-001 | satisfied | `es6_reconstruction::constraint_violation_reaches_the_caller_as_condition_violated` — asserts `is_condition_violated()` **and** the absence of a `Store` arm | `crates/happenstance-cloudflare/src/lib.rs`, beside the `!Send` probes |
| AC-002 | satisfied | `::transport_fault_reaches_the_caller_distinguishably` — the `Store(Sql(Thrown(..)))` shape plus the recovered text, read through `Display` and the public `source` chain | same file; the predicate `what_the_store_said` |
| AC-003 | satisfied | `::the_distinction_is_reachable_from_outside_the_crate`, whose `classify_like_a_consumer<S: EventStore>` binds the **bare** flavour and reads only `pub` items; `cargo doc` and `cargo clippy` clean | the crate's public surface, unchanged |
| AC-004 | satisfied | `::an_evidence_discarding_classifier_is_rejected` — the same predicate returns `false` against the named wrong shape and `true` against the real error | same file |
| AC-005 | satisfied | Findings 2 and 3 rewritten to observational mood; a new `# The ES-6 verdict` section naming `adr-0023-and-atom-resolutions`; `git status -- .kb/` empty; `redkiln validate --kb` passed | `crates/happenstance-cloudflare/src/lib.rs` crate documentation |
| AC-006 | satisfied | one `cargo test -p happenstance-cloudflare --target wasm32-unknown-unknown --lib` lists all five reconstruction tests **and** `wasm_tests::the_probe_is_not_vacuous` (74 passed); the host `cargo test` still runs all four probes with no wasm toolchain | both test wrappers over one set of assertions |
| AC-007 | satisfied | the four probes on both targets; `send_shape`'s two doctests including the `compile_fail,E0277` one; `happenstance-core` untouched; `spec-trace` and `redkiln validate --kb` green | — |

**Three findings a reviewer should read before the tests.**

1. **No public item was added.** EC-003 provided for a minimal accessor if the real `worker`
   bindings left the fact unreachable from outside the crate. They did not, so none was
   added — the strongest available answer to a design that records no surface. The only new
   item anywhere is `test_object::arm_throw`, which is `pub(crate)` and
   `#[cfg(all(test, target_arch = "wasm32"))]`.
2. **The tier resolved to EC-004's branch, and no probe moved to get there.** `worker`'s
   bindings resolve to panicking stubs off the target, so a store cannot be *driven* on the
   host at all — the reconstruction is `wasm32`, the same tier both slice-mates already use.
   The forbidden move was deleting or relocating a probe to make the tier question go away;
   instead the four `!Send` assertions are written once and wrapped twice, so an ordinary
   `cargo test` with no wasm toolchain still reaches `the_probe_is_not_vacuous`.
3. **The artefact can fail, and that was demonstrated rather than asserted.** Two wrong
   classifiers were compiled into the *real* `classify_write` and the suite re-run: the
   evidence-discarding shape (D3) was rejected by two tests, and the flattening shape by
   three — including a slice-mate's. This matters more than usual here, because the
   behaviour under test landed with `durable-object-write-path`; the question a Red step can
   honestly ask of this story is not "does the body exist" but "can this assertion fail",
   which is AC-004's subject.

**What is explicitly not claimed.** `store_error_crosses_a_join_handle`, ES-6's own named
rule, is still unwritten and unowned; writing it is a testkit change against ADR-0009's
marker and is not this story's
(`.kb/open-questions/es-6-names-an-unwritable-rule.md`). The portable half of the
distinction — a violation on the wrong `AppendError` arm — is already covered for every
adapter by `ViolationAsStoreErrorStore`. And this adapter still has not run the conformance
suite under `workerd`; that is the next milestone's.

**Deferred, with the owner named.** The ES-6 decision atom
(`adr-0023-and-atom-resolutions`); the `Rule:` scheduling gap
(`.kb/open-questions/es-6-names-an-unwritable-rule.md`, unowned by design); project AC-005's
stale wording, which predates ADR-0009's acceptance and was obeyed as the atom rather than
as written, exactly as this story's spec directs.

## Slice close

This story merges last in `real-worker-bindings`. At close: `cargo xtask ci --fast` reports
`all required checks passed (--fast: 4 optional step(s) not run)`, `cargo xtask affected
--base main` reports `affected gate passed`, and `cargo test -p happenstance-cloudflare
--target wasm32-unknown-unknown --lib` is 74 passed / 0 failed. The crate carries no
`todo!()` and no `#![allow(clippy::todo)]`, which is project DoD 3 as a grep and the
precondition `publish-ready-crate` verifies rather than causes.
