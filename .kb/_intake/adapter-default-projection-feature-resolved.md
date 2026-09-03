# Resolved: the projection port leaves `happenstance-neon`'s and `happenstance-postgres`'s defaults

**Decided 2026-09-03.** Staged for `/redkiln:kb-ingest`. This is a **resolution of an existing open
question**, not a new decision atom: the ingest wave should amend
`kb-open-question-adapter-default-projection-feature-001` to record that it is answered and how,
rather than mint a sibling atom that restates it.

## The answer

**No — they should not stay.** Both crates now read:

```toml
happenstance-core = { workspace = true, features = ["std"] }   # was […, "unstable-projection"]
default          = ["event-store"]                              # was ["event-store", "projection-store"]
projection-store = ["happenstance-core/unstable-projection"]    # was []
```

This is the same shape `happenstance-sqlite` took on 2026-09-02, and it closes the same hole:
`ADR-0036` ships `ProjectionStore` behind an off-by-default gate, and an adapter that turns that
gate on for everyone makes the decision true of the contract crates and false of the crate a
consumer installs.

## The open question was right that this was not the SQLite fix

Its central observation is the reason this took two edits per crate rather than one, and it holds
up exactly as written: *"changing `default` closes nothing on its own — `--no-default-features`
still enables the core gate in that build."*

`happenstance-sqlite` already forwarded the flag **through** its feature, so removing
`projection-store` from `default` was sufficient there. These two named
`happenstance-core = { features = ["std", "unstable-projection"] }` in the **dependency table**,
outside any feature, while `projection-store = []` forwarded nothing at all. So the feature named
the capability while doing none of the gating, and turning it off changed nothing a consumer could
observe. Trimming `default` alone would have produced a crate that *looks* gated and is not — a
worse state than the one it started in, because it would read as fixed.

Both halves moved together: the flag left the unconditional dependency and became the feature's
forward. That is the only arrangement in which `--no-default-features` actually yields a build
without the unfrozen port.

## Verified

All four feature combinations compile for **each** crate — `--no-default-features`, default,
`--features projection-store`, `--all-features` — eight checks in total. Both crates' projection
modules were already gated on `feature = "projection-store"` in `src/lib.rs`, which is why the
forward could move without touching any Rust.

## What this does not change

- **Neither crate is published**, and both remain `publish = false` skeletons whose projection
  bodies are `todo!()`. The exposure this closes was latent, and closing it now is cheap precisely
  because nothing depends on the old shape.
- **`spec/SPECIFICATION.md`'s impl census is untouched** and is not restated here, as the open
  question itself declines to restate it.
- **PS-2's bar is unaffected.** Nothing here changes what has run the projection suite, so ADR-0036
  stands exactly as written and its `[PROVISIONAL]` marker does not move.

## Why it was taken here rather than by the owning project

The open question named `postgres-and-neon-stores` as owner and *"before either crate is published"*
as the deadline. That project has not started; its stories are all at `plan`. The change is three
manifest lines per crate with no Rust touched and a mechanical verification, and leaving a known
walk-around in place until a project starts is how a default nobody re-read becomes a consumer's
problem. The owning project inherits it as done rather than as owed — its deskeleton story already
contemplated shipping `happenstance-neon` with `projection-store` off by default, and now finds it
that way.

## Evidence

- `crates/happenstance-neon/Cargo.toml`, `crates/happenstance-postgres/Cargo.toml` — the three
  changed lines in each, with the reasoning inline
- `crates/happenstance-sqlite/Cargo.toml` — the shape both now match
- `.kb/decisions/0036-the-projection-port-ships-gated.md` — the decision this enforces
- `.kb/open-questions/projection-store-in-adapter-default-features.md` — the question being resolved
