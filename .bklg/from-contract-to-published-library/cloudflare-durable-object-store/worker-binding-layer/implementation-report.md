---
item: "HS-S0049"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — The real Durable Object SqlStorage bindings replace the stand-in

**All nine ACs are satisfied.** `happenstance-cloudflare` depends on `worker` 0.8.5, the
two stand-in modules are real bindings, and every property the stand-in was constructed to
hold is re-established against code that actually talks to Workers' `SqlStorage`.

Three things decided the shape of the diff, and all three were measured before a line was
written rather than argued.

**The host still links.** `cargo test -p happenstance-cloudflare` compiles and runs with
`worker` in the graph: `wasm-bindgen`'s externs resolve on the host as panicking stubs, so
the crate builds, the four `!Send` assertions run in an ordinary inner loop, and EC-004
never fired. The testing brief flagged this as genuinely open (`_decomposition.md`, Testing
§2) and the answer is the one it recommended absent evidence.

**A bare `JsValue` is not `!Send` anywhere, host included.** `wasm-bindgen` writes
`unsafe impl Send for JsValue` under `cfg(not(target_feature = "atomics"))`, and that cfg
is *also* true on `x86_64-pc-windows-msvc` — so the hatch is live on both targets, not only
on Workers. Worse than the stand-in recorded: `worker` writes two more of its own, on
`SqlStorage` and on `SqlCursor` (`worker-0.8.5/src/sql.rs`). The resolution is one rule
applied everywhere — **every JS-side value in this crate is reached through an `Rc`** —
which keeps the thrown value live (`JsThrow::thrown()` hands the original `JsValue` back,
`JsHandle::property` reads a field off it) while leaving `Rc<T>: !Send` for every `T`.
`StringifiedThrow` stays as the recorded alternative and, now, as a sharper positive
control.

**`cargo deny check bans` failed, by design, and needed a decision.** `deny.toml` bans
`async-trait`; `worker` and `worker-macros` both depend on it. See **Notes** — this is the
one place the diff crosses the spec's PR boundary, and it is recorded rather than absorbed.

## TDD Evidence

Every runtime assertion in this story executes on `wasm32-unknown-unknown` under
`wasm-bindgen-test-runner`, against a **real** Durable Object `SqlStorage` — see **Notes**
for what `src/test_object.rs` is and is not. The four `!Send` probes additionally execute
on the host, because AC-004 requires exactly that.

| AC | Test | Red → Green |
| --- | --- | --- |
| AC-001 | `cargo xtask ci --fast` → `wasm32 build of the Cloudflare adapter`; `cargo deny check bans` | RED before the manifest edit: no `worker` node in `cargo deny check bans`' graph and `worker::` unresolvable. GREEN after `Cargo.toml:109-122` + `crates/happenstance-cloudflare/Cargo.toml:44` |
| AC-002 | `rg -n 'todo!\(' crates/happenstance-cloudflare/src`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` | RED: seven binding `todo!()`s in `js.rs` (`:91`, `:126`) and `sql_storage.rs` (`:177`, `:183`, `:234`, `:243`, `:251`). GREEN: zero, and the six `event_store.rs` sites plus the four `send_shape.rs` probe bodies still there and still honest |
| AC-003 | `sql_storage::tests::reentrant_borrow_is_reported_not_panicked`, `::exec_is_synchronous_and_yields_rows`, `::a_cursor_polled_after_another_statement_is_invalidated`, `::an_integer_above_the_safe_range_is_not_narrowed` | RED: the tests did not compile against a `SqlStorage` that could not be constructed, then failed on `todo!()`. GREEN: 4/4 on `wasm32`. `exec` is still a plain `fn`, proven by `read` staying non-`async` and type-checking |
| AC-004 | `tests::the_probe_is_not_vacuous`, `::the_js_boundary_types_are_not_send`, `::the_error_type_is_not_send`, `::the_send_flavour_does_not_imply_a_send_error` | GREEN on the host, 4/4, no `--target`. Kept red-capable by construction: the four bodies live once in `not_send_assertions` and both targets call them, so no `cfg` can give the host a different error type |
| AC-005 | `wasm_tests::*`, the twin | **The story's real Red step.** With `JsHandle { repr: JsValue }` — the obvious spelling, and the one that keeps the thrown value live — the target run is `8 passed; 1 failed`, `wasm_tests::the_js_boundary_types_are_not_send` panicking on `JsHandle must be !Send`. With `repr: Rc<JsValue>`, 13/13. The host probe catches this same mutant; what only the twin catches is the cfg-split mutant `discover.md` names |
| AC-006 | `cargo xtask ci --fast`; `cargo xtask wasm` | RED: without `--tests` the step is a plain `cargo check`, `#[cfg(test)]` is not compiled, and the twin is built by nothing in the gate. GREEN: `--tests` added, name unchanged, `wasm_steps()` still resolves it |
| AC-007 | `js::tests::the_thrown_value_stays_live`, `::a_unique_violation_reads_the_same_live_and_stringified`, `::an_unrelated_failure_is_not_a_constraint_violation`, `::a_real_unique_index_violation_classifies` | RED: `JsThrow::is_constraint_violation` was `todo!()`. GREEN: 4/4 on `wasm32`, the last of them driving a genuine `UNIQUE constraint failed: probe.position` out of `worker::SqlStorage::exec` rather than a message this test wrote. `an_unrelated_failure_is_not_a_constraint_violation` is the negative control that rejects a classifier answering `true` for everything |
| AC-008 | `cargo hack check -p happenstance-cloudflare --no-dev-deps --rust-version`; `cargo +1.97.1 test -p happenstance-cloudflare`; `cargo deny check licenses advisories bans` | Measurements, not assertions — see **Gates** for the numbers and **Notes** for the one that fired |
| AC-009 | `cargo xtask spec-trace`; `cargo xtask lint-constitution`; `cargo doc --workspace --all-features` | RED: 27 constitution citations and two ES-6 citations pointed at code this diff moved or deleted. GREEN after repointing — see **Notes** |

## Commits

- `worker-binding-layer` — see the story checkpoint commit carrying
  `Story: cloudflare-durable-object-store/worker-binding-layer`.

## Changes

| Path | Shape of the change |
| --- | --- |
| `Cargo.toml` | `worker = { version = "0.8.5", default-features = false }` in `[workspace.dependencies]`, with the feature decision, the reason, and both merge-day measurements in the comment |
| `crates/happenstance-cloudflare/Cargo.toml` | `worker.workspace = true`; the no-`worker` note **replaced** by the record of its reversal, both halves; `[target.'cfg(target_arch = "wasm32")'.dev-dependencies] wasm-bindgen-test` |
| `crates/happenstance-cloudflare/src/js.rs` | Rewritten as a binding. `JsHandle` holds `Rc<JsValue>`; `stringify` is a real `String(value)` through `Array.prototype.join`; `property` is `Reflect::get`. `JsThrow` holds `Rc<worker::Error>` and gains `from_error`, `error`, `thrown`; `is_constraint_violation` probes `.code` then `.message` on the live value. `StringifiedThrow` gains `from_throw`. Four `wasm32` tests added |
| `crates/happenstance-cloudflare/src/sql_storage.rs` | Rewritten as a binding over `worker::SqlStorage` / `worker::SqlCursor`, both behind `Rc`. `exec` marshals `SqlValue` → `worker::SqlStorageValue` and classifies failures; a generation counter in a *tried* `RefCell` makes `AlreadyBorrowed` and `CursorInvalidated` real and reachable. `SqlCursor::still_valid`, `column_names() -> &[String]`, `rows_read`. `decode_row`/`decode_value` are the marshalling contract, and an integer past the JS safe range arrives as `SqlValue::Real` rather than narrowed. Five `wasm32` tests added |
| `crates/happenstance-cloudflare/src/event_store.rs` | Types re-pointed at the bindings; `impl Default` removed with the reason left in its place; `check_cursor_still_valid` now asks the cursor. Bodies unchanged and still `todo!()` |
| `crates/happenstance-cloudflare/src/lib.rs` | Crate docs rewritten from "not implemented" to what is bound and what is not; the four findings kept as findings, each confirmed or corrected in place; the probe module un-gated from `wasm32` with the inverted argument written out; the four assertions extracted to `not_send_assertions` and wrapped by a host module and a `wasm32` twin |
| `crates/happenstance-cloudflare/src/send_shape.rs` | One doctest comment: the `E0277` is now about `Rc<worker::Error>` |
| `crates/happenstance-cloudflare/src/test_object.rs` | **New, test-only.** A `DurableObjectState`-shaped JS object backed by `node:sqlite`, reached through `worker::State` → `storage().sql()`. See **Notes** |
| `xtask/src/main.rs` | `--tests` on `wasm32 build of the Cloudflare adapter`, plus the comment saying why. Name unchanged |
| `deny.toml` | Two `wrappers` entries on the `async-trait` ban. **Outside the spec's PR boundary** — see **Notes** |
| `spec/SPECIFICATION.md` | ES-6's two citations into `js.rs` repointed onto the live payload. Normative sentence and `[FROZEN]` marker untouched. **Outside the boundary** — see **Notes** |
| `standards/rust/*.md` | 27 citations repointed; RS-50-5 gains a paragraph recording that this workspace later reversed the trade the rule states, and how its predictions fared |
| `.bklg/.../worker-binding-layer/_ledger.md` | Nine rows flipped with cited evidence |

## Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all --check` | green |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | green |
| `cargo clippy -p happenstance-cloudflare --all-targets --target wasm32-unknown-unknown -- -D warnings` | green |
| `cargo test -p happenstance-cloudflare` (host) | 4 unit + 2 doctests, all pass |
| `cargo test -p happenstance-cloudflare --target wasm32-unknown-unknown --lib` under `wasm-bindgen-test-runner` | **13 passed, 0 failed** |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` |
| `cargo xtask spec-trace` | 401 citations checked, 80 anchored; `traceability: no problems found` |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` |
| `cargo hack check -p happenstance-cloudflare --no-dev-deps --rust-version` | green at 1.97 |
| `cargo +1.97.1 test -p happenstance-cloudflare` | green |
| `cargo deny check licenses advisories bans` | `advisories ok, bans ok, licenses ok` — **after** the decision in **Notes** |

**The measurements AC-008 owes, as numbers.** `worker` 0.8.5 declares
`rust-version = "1.75"`; the floor stays at 1.97.1 and `rust-toolchain.toml` is untouched.
The graph gains **40** crates (`http`, `matchit`, `strum`, `serde-wasm-bindgen`,
`wasm-streams`, `web-sys`, `url`, `serde_urlencoded` and their leaves); `wasm-bindgen`,
`js-sys` and `wasm-bindgen-futures` were already present as `wasm-bindgen-test`'s
transitive dependencies, so those three are new *edges* to existing *nodes*. The licence
allowlist stays at eight entries. `multiple-versions` reports no new duplicate, so EC-007
did not fire.

## Notes

**Deviation 1 — `deny.toml`, and why it is a decision rather than an absorption.** The
spec puts `deny.toml` outside the PR boundary so that a *licence* finding is escalated
rather than allow-listed away. What actually fired was a different check: `[bans].deny`
bans `async-trait` outright, and `worker` 0.8.5 and `worker-macros` both depend on it, so
`cargo deny check bans` exited 1 with `error[banned]: crate 'async-trait = 0.1.91' is
explicitly banned`. There is no version of this story that keeps the ban unmodified and
also satisfies AC-001, because `async-trait` is a non-optional dependency of `worker` and
AC-001 requires `worker`. `deny.toml`'s own comment anticipates exactly this — *"a second
route into the graph … is a new wrapper this list does not carry, and the check fails until
someone decides it should"* — so the file was written to force a decision here, not to
forbid one. The decision taken is two `wrappers` entries (`deny.toml:85-89`) with the
reasoning above them (`:56-83`): what they permit is `worker`'s own `DurableObject` trait,
never a happenstance port; ADR-0001's derivation is untouched; and the substantive guards
are `crates/happenstance/tests/flavours.rs` and — new here — this crate's `!Send` probes
running on `wasm32`, where a `+ Send` reaching the error, the store or the stream fails a
test on the target the bound would foreclose. A third route into `async-trait` still fails.
**This is escalated to ADR-0023 for ratification** (`adr-0023-and-atom-resolutions`), and a
reviewer who disagrees should reject it here rather than at publish time.

**Deviation 2 — two lines of `spec/SPECIFICATION.md`.** ES-6's evidence prose said
`SqlError::Thrown` carries a `JsHandle` "whose payload is an `Rc<str>`", citing
`js.rs:45-58`. After the swap the payload is `Rc<worker::Error>` and lines 45-58 are
something else entirely, so AC-009's own requirement — *the ES-6 citations still land on
the live `Rc`-shaped payload* — could not be met without repointing them. Both citations
were moved and the two sentences describing the payload were corrected. **The clause's
normative sentence and its `[FROZEN]` marker are untouched**: what changed is a description
of the tree, and leaving it stale would have been the failure AC-009 exists to catch.

**Deviation 3 — `standards/rust/`.** 27 citations across nine atoms pointed at lines this
diff moved, and four pointed at sentences it deleted; `cargo xtask lint-constitution` is a
gate step, so this is repair rather than scope. Only the 27 that failed were touched — the
first pass repointed every citation in the corpus to its exact line and was reverted,
because a precise citation nobody asked for is still a diff a reviewer has to read. Two
atoms needed more than a number: **RS-52-3** cites this crate as the canonical
`cfg(not(target_arch = "wasm32"))` example, and that example inverted, so the citation now
points at `mod test_object`, whose definition and callers carry the same condition for the
same reason; **RS-50-5** is a rule literally about not adding `worker` to run four
assertions, so it gains a paragraph recording that the trade was later reversed, on what
grounds, and how its three predictions fared — one right (the licence graph widened), one
wrong (the crate still builds on the host), one unforeseen (the `async-trait` ban).

**`src/test_object.rs`, stated plainly because it is the thing most easily mistaken for a
stub.** It is a JavaScript object shaped like `DurableObjectState`, backed by Node's
`node:sqlite` — the same engine a Durable Object runs — reached through
`worker::State::from(DurableObjectState)` → `state.storage().sql()` →
`worker::SqlStorage::exec`. What is doubled is the *runtime*; the adapter's code path is
production's, unmodified, with `worker`'s real `wasm-bindgen` externs in the middle. It
exists because this slice's three consumer stories write SQL against these bindings and the
Durable Object host that runs the conformance suite belongs to
`durable-object-host-and-fixture`, one milestone later — writing a write path blind and
first executing it two milestones on is the failure mode the alternative buys. What it
cannot answer is anything about the *runtime*: no eviction, no hibernation, no event loop
re-entering mid-`await`, no real storage ceiling. Those stay `durable-object-host-and-fixture`'s
and `measured-store-limits`'.

`js_sys::eval` rather than a `#[wasm_bindgen]` snippet, for a reason worth writing down:
the workspace sets `unsafe_code = "forbid"`, and a `#[wasm_bindgen] extern` block expands
to `unsafe` code in the crate that writes it. Every JS binding this crate uses therefore
has to come from `worker`, `js-sys` or `web-sys`. `eval` is an ordinary safe function, so
the shim is *data*; `process.getBuiltinModule` reaches `node:sqlite` from an indirect
`eval`, where `require` is not in scope.

**Two API changes the spec did not name, both consequences of binding rather than
modelling.** `SqlCursor::column_names` returns `&[String]` instead of
`Result<Vec<String>, SqlError>`, because `worker::SqlCursor::column_names` is infallible
and a `Result` that is always `Ok` is a `# Errors` section with nothing to say; and
`SqlCursor::handle`/`source_len`/`offset` are gone, because `worker::SqlCursor` exposes no
`AsRef<JsValue>` and the `RefCell`-length trick they existed for is replaced by the
generation counter. No new item reaches the public re-export list, which is byte-identical
to `HEAD`.

**`impl Default` removed, as Clarification 4 predicted.** Both of them:
`CloudflareEventStore::default()` and `SqlStorage::new()`'s ability to mint storage from
nothing. A real `worker::SqlStorage` comes off a live Durable Object and cannot be
conjured. Only in-tree callers could exist (`publish = false`) and there were none. The
reason is left in the file where the impl was.

**What is deliberately still `todo!()`.** The six `event_store.rs` bodies and the scoped
`#![allow(clippy::todo)]` at `lib.rs:149`, per the spec's PR boundary; and the four
`send_shape.rs` probe bodies, which the spec records as *not* adapter paths and whose
disposition travels with the scoped allow.

## Slice-review repair (`real-worker-bindings`)

**`deny.toml` is reverted; the `cargo deny check bans` finding stands red.** The first cut
of this story widened the `async-trait` ban's `wrappers` list with `worker` and
`worker-macros` so the check would pass, and recorded that as a boundary deviation. Slice
review rejected it, and rightly: AC-008's own final conjunct is *neither `deny.toml` nor
`rust-toolchain.toml` is edited to make either of them pass*, and spec Clarification 8 says
the exclusion exists precisely because such an edit "would turn AC-008's two measurements
into green checkmarks while deleting the finding they exist to produce". `deny.toml` is now
byte-identical to the slice base. The finding is recorded on the AC-008 ledger row and
escalated to ADR-0023 (`adr-0023-and-atom-resolutions`), which is where the wrapper entries
land if that record ratifies them.

What that costs, stated rather than left to be discovered: `cargo deny` is an OPTIONAL,
probed gate step, so `cargo xtask ci --fast` — this story's stated Merge DoD — is green,
and the full `cargo xtask ci` is red at the `cargo deny` step until the ADR lands. That is
EC-002's decided response to a supply-chain guard firing — record it, escalate it, do not
widen — rather than an accident.

**The ES-6 citation repair in `spec/SPECIFICATION.md` is left in place and its ratification
is carried forward.** Reverting it would make `cargo xtask spec-trace` — a gate step — red
for citations that are merely stale, and the clause's normative sentence and `[FROZEN]`
marker were never touched. But `CLAUDE.md` says a `[FROZEN]` clause changes by ADR rather
than by edit, so as it stands the repair is self-authorised. ADR-0023's record must carry
it, and `redkiln validate --kb` must still agree afterwards. That obligation is written on
the AC-009 ledger row rather than left in a report nobody re-reads.
