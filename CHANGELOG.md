# Changelog

Notable changes to `happenstance`, `happenstance-core` and `happenstance-testkit`.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and
the project follows [semantic versioning](https://semver.org/spec/v2.0.0.html)
from `0.1.0` onward.

Started at the beginning rather than reconstructed at release time. A changelog
assembled from twelve phases of `git log` records what the commits said, which is
not the same as what a user needed to be told.

**Two things this project versions unusually, stated once here:**

- **`happenstance-testkit` has its own version**, independent of the other
  crates. Adding a conformance rule is a semver-*minor* change that can turn a
  passing adapter's CI red, so treat a minor bump there as breaking and pin it
  exactly.
- **`ProjectionStore` ships behind an off-by-default `unstable-projection`
  feature** and is exempt from semver until two adapters at opposite ends of the
  batch-shape axis have passed its conformance suite. See
  [`docs/architecture/SPECIFICATION.md`](docs/architecture/SPECIFICATION.md) §4.

## [Unreleased]

### Added

- **The conformance suite no longer requires `tokio`.** The rule set is
  enumerated in exactly one place, `for_each_event_store_rule!`, and the per-test
  wrapper is a parameter rather than something the testkit chooses. Three
  emitters ship — `__emit_tokio` (still the default, so existing callers are
  unaffected), `__emit_blocking` and `__emit_wasm` — and a runtime none of them
  covers needs no release of `happenstance-testkit`: write a `macro_rules!` that
  takes a list of identifiers and hand it to the registry yourself.
- `happenstance_testkit::block_on`, a twenty-line single-threaded executor with
  no `Send` bound and no dependencies. It is what makes the runtime-free harness
  runtime-free.
- A `no_orphan_rules` meta-test. A rule added to the suite without being
  registered used to produce no signal of any kind — it compiled, `cargo test`
  reported green, and an adapter was certified against twenty-six rules while
  its author believed it was twenty-seven.
- **The workspace's first `!Send` implementer of either port.**
  `LocalMemoryEventStore` — `Rc<RefCell<Vec<SequencedEvent>>>`, implementing the
  bare `EventStore` and nothing else — passes all twenty-seven rules under four
  harnesses, including `wasm-bindgen-test` on `wasm32-unknown-unknown` in CI.
  Until now the entire evidence base for the two-flavour port design was a
  `cargo check`: a compile of the trait, with nothing implementing the flavour
  that design exists for.
- A `wasm-conformance` CI job that **runs** the suite on
  `wasm32-unknown-unknown`, and a gate step that type-checks the harnesses for
  that target on every run. The two are different claims: `#[tokio::test]`
  type-checks for wasm32 and then cannot run there.
- [ADR-0008](docs/adr/0008-one-derivation-for-both-ports.md), which puts
  `ProjectionStore` under the same derivation scheme as `EventStore` — it had
  carried the identical construction since it was written and appeared in no ADR
  at all — and states what a provided body owes both flavours.
- A CONTRIBUTING section on adding a method to a port. The specification already
  claimed this was documented there; it was not.
- `cargo xtask reserve <name>`, which generates the `0.0.0` placeholder used to
  claim a crates.io name. Names are claimed one per phase, as the crate that
  justifies each becomes real.
- `happenstance`, `happenstance-core` and `happenstance-testkit` reserved on
  crates.io at `0.0.0`. Placeholders with no functionality; the first functional
  release will be `0.1.0-alpha.1`.
- A CI job running the full test suite at the 1.85 MSRV, dev-dependencies
  included. The existing job used `--no-dev-deps`, which is exactly the flag that
  hid `proptest` and `tokio` — both of which declare `rust-version = "1.85"` with
  no headroom, so the claim had never been checked.
- A gate step building the contract crate's documentation with
  `--no-default-features`.
- An inbound-equals-outbound licensing statement in `CONTRIBUTING.md`.
- `cargo xtask spec-trace`, which checks the architectural specification against
  the conformance suite and the case catalogue — maturity markers, falsifiers,
  rule names, case numbers and `file:line` citations — and **generates** §7.1 and
  §7.2 of the specification, failing the gate when the committed copy and the
  computed one disagree.
- A gate step asserting that each publishable crate's packaged artifact really
  contains `LICENSE-MIT`, `LICENSE-APACHE` and `README.md`. It parses the file
  list rather than trusting the exit status, and derives the set of publishable
  crates from `cargo metadata`, so promoting a stub is caught rather than
  remembered.
- Gate steps for the wasm32 feature powerset and, on nightly, the `docsrs`
  documentation configuration. The latter is the only thing that compiles
  `#![cfg_attr(docsrs, feature(doc_cfg))]` before docs.rs does — that is to say,
  before publication, which is the last moment it can be fixed.
- A weekly `cargo deny check advisories` job. A new advisory against an unchanged
  dependency is the one failure that arrives with no commit to trigger CI.

### Changed

- **The contract crate is now `happenstance-core`; `happenstance` is the typed
  layer** ([ADR-0006](docs/adr/0006-bare-name-to-the-typed-layer.md)). Today
  `happenstance` re-exports the contract unchanged, so it is published from the
  start and `cargo add happenstance` is true throughout. Adapter authors should
  depend on `happenstance-core`.
- `clippy::todo` and `clippy::unwrap_used` are denied workspace-wide rather than
  allowed and warned. `happenstance-sqlite` carries a scoped `#![allow]` for the
  two `todo!()`s it still has.

### Fixed

- **The one test guarding the two-flavour design could not fail.**
  `read_stream_is_send` asserted `Send` on a *concrete* stream, where auto-trait
  leakage satisfies it whatever the trait promises — it passed unchanged with
  `Send` struck from the derivation attribute entirely. It is replaced by two
  generic tests, and it takes two: writing the bound at the definition fixes the
  leakage, but under the `async fn read(..) -> Result<impl Stream, E>` refactor
  the outermost item is the *future*, so an assertion on the call's result is
  discharged against the future while the stream stays `!Send`. Only
  `spawns_from_generic`, which holds a read across an await inside a real
  `tokio::spawn`, rejects that.
- **`ADR-0001`'s provisional marker is lifted**, against the condition its own
  banner named. The full proof — a real `!Send` adapter — is still the Cloudflare
  work, and the banner now says which half is settled.

- **The published `.crate` contained no licence text and no README** (D11). Both
  licence files lived at the repository root, and Cargo packages only what is
  inside the crate directory — so every crate would have shipped
  `MIT OR Apache-2.0` in its metadata and nothing in the tarball.
- **The README never compiled** (D10). Its Quick start used `?` and `.await` at
  the top level, and the command-loop example referenced a `store` that was never
  defined. Both are now compiled as doctests.
- **`cargo doc --no-default-features` failed** (D13). Three intra-doc links named
  `MemoryEventStore`, which does not exist without the `memory` feature, and
  rustdoc treats a broken intra-doc link as a hard error rather than a warning.
- **The `serde` feature depended on `serde/alloc` arriving transitively** (D12)
  through `bytes/serde`, which is free to stop providing it in a patch release.
- **The specification's §7.1 summary had been wrong since the document was
  assembled.** It totalled 137 `[FROZEN]` / 41 `[PROVISIONAL]` / 14 `[DEFERRED]`
  / 1 `[NON-NORMATIVE]`, while §7.2's own rows aggregate to 132 / 46 / 13 / 2 —
  so it disagreed with the table twenty lines below it and with §1.3 six thousand
  lines above it. Both miscounts inflated `[FROZEN]`. Regenerating it also removed
  thirteen phantom conformance-rule names that were ordinary words in code
  formatting, and surfaced six pairs of rules named differently by §6 and §3.
- **The gate denied no rustdoc lint.** `RUSTFLAGS: -D warnings` reaches every
  `rustc` invocation and no `rustdoc` one, so the documentation step had been
  reporting warnings and exiting 0. Both documentation steps now set
  `RUSTDOCFLAGS`, and the three warnings this exposed are fixed.
- **Nothing in CI installed a nightly toolchain**, so the `docsrs` step's probe
  failed and it printed `skipped` on every runner while two comments and the
  runbook asserted it was running.
- Documents that the `happenstance-core` rename had inverted rather than merely
  dated — including a module that restated the "no `serde` in the contract crate"
  constraint against the crate whose entire purpose is encoding, and an accepted
  ADR linking to a source path that no longer exists.

[Unreleased]: https://github.com/Wet-Ink-Corporation/happenstance/commits/main
