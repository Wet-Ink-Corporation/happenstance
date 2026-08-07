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

[Unreleased]: https://github.com/Wet-Ink-Corporation/happenstance/commits/main
