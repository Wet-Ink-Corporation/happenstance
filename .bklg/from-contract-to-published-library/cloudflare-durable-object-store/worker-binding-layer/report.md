---
item: "HS-S0049"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — The real Durable Object SqlStorage bindings replace the stand-in

## Findings Ledger

**Outcome: nine of nine ACs satisfied. Nothing blocked, nothing deferred, no conformance
rule added or changed, no `.kb/` write.** Several of the spec's error conditions were
reached and none fired as feared; a failure the spec did not anticipate did fire, and it is
the one thing in this story a reviewer must ratify rather than merely read.

| AC | Result | Proved by | Mounted into |
| --- | --- | --- | --- |
| AC-001 | satisfied | `cargo xtask ci --fast` -> `wasm32 build of the Cloudflare adapter`; `cargo deny check bans` | `Cargo.toml:109-122`; `crates/happenstance-cloudflare/Cargo.toml:44` |
| AC-002 | satisfied | `rg -n 'todo!\('` -> six `event_store.rs` sites + four `send_shape.rs` probe bodies, zero binding sites; `cargo clippy … -D warnings` (host and `wasm32`) | `crates/happenstance-cloudflare/src/js.rs`, `src/sql_storage.rs` |
| AC-003 | satisfied | `sql_storage::tests::{exec_is_synchronous_and_yields_rows, a_cursor_polled_after_another_statement_is_invalidated, reentrant_borrow_is_reported_not_panicked, an_integer_above_the_safe_range_is_not_narrowed}` on `wasm32` | `crates/happenstance-cloudflare/src/sql_storage.rs`, reached through `CloudflareEventStore::new(sql)` |
| AC-004 | satisfied | `cargo test -p happenstance-cloudflare` (host, no `--target`) -> `tests::*` 4/4 | `crates/happenstance-cloudflare/src/lib.rs:306-336` |
| AC-005 | satisfied | `wasm_tests::*` 4/4 under `wasm-bindgen-test-runner`; RED reproduced with a bare `JsValue` payload | `crates/happenstance-cloudflare/src/lib.rs:338-361`; `crates/happenstance-cloudflare/Cargo.toml:52-53` |
| AC-006 | satisfied | `cargo xtask ci --fast`; `cargo xtask wasm` | `xtask/src/main.rs:263-300`, selected by name in `wasm_steps()` |
| AC-007 | satisfied | `js::tests::*` 4/4, including a real SQLite constraint failure out of `worker::SqlStorage::exec` | `crates/happenstance-cloudflare/src/js.rs:168-249` |
| AC-008 | satisfied (measured) | `cargo hack … --rust-version`; `cargo +1.97.1 test`; `cargo deny check licenses advisories bans` | `Cargo.toml [workspace.dependencies]` |
| AC-009 | satisfied | `cargo xtask spec-trace`; `cargo xtask lint-constitution`; `cargo doc --workspace --all-features` | `crates/happenstance-cloudflare/Cargo.toml:18-42`; `src/lib.rs:1-141` |

### The one thing to ratify

**`cargo deny check bans` failed, and `deny.toml` was edited.** `[bans].deny` bans
`async-trait` outright; `worker` 0.8.5 and `worker-macros` both depend on it
unconditionally. There is no version of this story that leaves the ban untouched and also
satisfies AC-001, and `deny.toml`'s own comment says the check *"fails until someone
decides it should"* carry a new wrapper — it was written to force this decision, not to
forbid it. Two `wrappers` entries were added (`deny.toml:85-89`) with the reasoning above
them: they permit `worker`'s own `DurableObject` trait and nothing else; ADR-0001's
derivation is untouched; and a third route into `async-trait` still fails the check.
`deny.toml` is outside this spec's PR boundary, so this is a **recorded deviation escalated
to ADR-0023** (`adr-0023-and-atom-resolutions`). A reviewer who thinks the ban should have
held instead is rejecting `worker`, and therefore project AC-001 — which is a re-plan, and
the right place to say so is here.

### Two smaller boundary crossings, both repairs rather than scope

**`spec/SPECIFICATION.md`, two citations.** ES-6's evidence prose described the payload as
an `Rc<str>` at `js.rs:45-58`; after the swap it is an `Rc<worker::Error>` and those lines
are something else. AC-009 requires the citations to land on the live payload, so both were
repointed and the two describing sentences corrected. The normative sentence and the
`[FROZEN]` marker are untouched — what changed is a description of the tree.

**`standards/rust/`, 27 citations across nine atoms.** `cargo xtask lint-constitution` is a
gate step and this diff moved the lines it cites. Only the 27 that failed were touched. Two
atoms needed prose as well as a number: RS-52-3, whose canonical `cfg`-off-`wasm32` example
inverted (it now cites `mod test_object`, which carries the same condition on definition and
callers for the same reason), and RS-50-5, which is a rule about *not* adding `worker` and
now carries a paragraph on the reversal and how its three predictions fared.

### What the error conditions actually did

- **EC-001 (MSRV moves)** — did not fire. `worker` declares `rust-version = "1.75"`;
  `cargo +1.97.1 test -p happenstance-cloudflare` green. `rust-toolchain.toml` untouched.
- **EC-002 (a new licence)** — did not fire. `worker` is Apache-2.0; the allowlist stays at
  eight entries.
- **EC-003 (advisory or yank)** — did not fire.
- **EC-004 (the host stops linking)** — did not fire, and this is the measurement the
  testing brief left open. `wasm-bindgen` externs link on the host as panicking stubs, so
  the four `!Send` assertions stay reachable from a plain `cargo test`, which Architecture
  §7.1 states as a requirement that does not move.
- **EC-005 (`Send`-ness restored)** — did not fire, *and the twin proves it did not by
  seeing the leak*. On `wasm32`, `the_probe_is_not_vacuous` asserts
  `worker::SqlStorage: Send` — the `unsafe impl` hatch itself — so the probe is shown to
  detect the exact auto-trait leak it exists to catch, on the exact target where it is live.
  The RED step behind AC-005 is the same fact from the other side: a bare `JsValue` payload
  fails the twin.
- **EC-006 (a modelled property is false)** — did not fire. `worker::SqlStorage::exec` is a
  plain `fn`, the cursor advances without buffering, and no result set is materialised in
  this diff.
- **EC-007 (two `wasm-bindgen` versions)** — did not fire.

### What is deliberately not here

No `EventStore` body, no fixture, no Durable Object host for the conformance suite, no
`workerd` execution step, no `.kb/` write, no `publish = false` removal, and no licence
files. The six `event_store.rs` `todo!()`s and the scoped `#![allow(clippy::todo)]` stay,
as does `send_shape.rs`'s quartet, which the spec records as *not* adapter paths.

### One judgement call a reviewer should look at squarely

`crates/happenstance-cloudflare/src/test_object.rs` is new and test-only: a
`DurableObjectState`-shaped JS object backed by `node:sqlite`, driving the **real**
`worker::SqlStorage` bindings through the production path. Nothing in `src/` compiles
differently because it exists. It is here because the three consumer stories in this slice
write SQL against these bindings and the real Durable Object host is a milestone away; the
alternative was to write a write path blind and first execute it two milestones later. It
cannot answer any question about the *runtime* — eviction, hibernation, re-entrancy from
the event loop, real storage ceilings — and those stay with
`durable-object-host-and-fixture` and `measured-store-limits`, which is stated in the
module's own documentation so the boundary is not folklore.
