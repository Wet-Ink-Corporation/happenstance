# ADR-0029: The MSRV is 1.97.1

- **Status:** accepted
- **Date:** 2026-08-07
- **Amends:** [ADR-0004](0004-edition-and-msrv.md), which set the floor at 1.85
  and carries a `provisional` marker for exactly this possibility. ADR-0004's
  reasoning is untouched and its body stays verbatim; what changes is the number.

## Context

Phase 2 landed six adapter skeletons. Five of them build at 1.85. The sixth does
not, and the way it fails is the argument for this decision.

`happenstance-sqlite` takes `rusqlite 0.40`, which takes `libsqlite3-sys 0.38.1`,
whose **build script** uses `cfg_select!`:

```text
error: cannot find macro `cfg_select` in this scope
   --> libsqlite3-sys-0.38.1/build.rs:110
error: could not compile `libsqlite3-sys` (build script)
error: process didn't exit successfully:
       `rustup run 1.85 cargo check --manifest-path crates\happenstance-sqlite\Cargo.toml`
```

**`libsqlite3-sys` declares no `rust-version` at all.** Neither does `rusqlite`,
`sqlx`, `sqlx-core` or `sqlx-postgres`. So `cargo hack check --no-dev-deps
--rust-version` — the whole mechanism this workspace relies on to protect the
floor — cannot see any of them coming, and `resolver = "3"`'s MSRV-aware
resolution cannot either, because both need the metadata that is absent. The
break was found by running the compiler, which is the only instrument that
works here.

Two facts bound the decision:

- **The break is not in our code.** Nothing this project wrote wants a newer
  compiler. It is a C-bindings build script in a dependency of a `publish = false`
  skeleton whose every body is `todo!()`.
- **The break is recoverable.** `rusqlite 0.37` / `libsqlite3-sys 0.35` was
  verified building at *both* 1.85 and 1.97.1, so staying at 1.85 was available
  and costed: an older SQLite binding in a crate that issues no SQL yet.

## Decision

**The MSRV is 1.97.1**, raised from 1.85, and `rusqlite` stays at 0.40.

The costed alternative was taken deliberately rather than by default. ADR-0004
carries `provisional` precisely so that this is a decision and not a violation,
and CLAUDE.md's fifth constraint says the quiet part out loud: *weigh it, do not
obey it — until first publish the MSRV is a preference, not a promise.* Nothing
is published. No downstream consumer is pinned to anything. The floor costs
nobody anything today, and phase 12 is where it turns into a promise.

`rust-version` and `rust-toolchain.toml` now carry the same number. **They are
still two different facts** and are deliberately not collapsed into one: the pin
is what contributors build with and moves whenever someone wants a newer
compiler; the floor is what consumers may build with and moves only by ADR.

## Consequences

**Good.** Adapters may take current dependency versions. This will not be the
last crate in this workspace to bind a C library or a database driver, and the
ecosystem's practice — as measured here, on five packages out of five — is not to
declare `rust-version` at all. A floor twelve releases below the pin was going to
be paid for repeatedly, in exactly this currency, by every adapter phase.

**Good, and immediate: let-chains are now available.** They stabilised in 1.88.
CLAUDE.md's constraint 5 forbade them for the MSRV's sake and is rewritten in this
change. `happenstance-core` has at least one site that was written around the
absence (`append.rs:101-102`); nothing is rewritten here, because a decision and
a refactor should not land in one commit.

**Bad, and the real cost of this ADR.** The `msrv` CI job now runs the same
compiler the gate runs, so it proves nothing until the two numbers diverge again.
At 1.85 it was checking a twelve-release gap with genuinely no headroom — both
`proptest` and `tokio` declared exactly 1.85 — and that check is gone. The job is
kept, with a comment saying why it is currently vacuous, rather than deleted:
deleting it would mean re-deriving it the first time someone bumps
`rust-toolchain.toml`, which is the sort of thing that gets re-derived wrongly.

**Bad.** 1.97.1 is recent, and anyone on a distribution-packaged toolchain is
excluded. This is the trade, and it is only defensible while nothing is published.
**Phase 12 must revisit it**: at first publish the number stops being a preference,
and "the newest compiler that existed when we shipped" is not obviously the right
promise to make to a consumer who is not building `happenstance-sqlite` at all.

**Neutral.** Nothing in the three publishable crates needed this. All three build
at 1.85 today and would keep doing so; the floor is raised for workspace
uniformity, not out of necessity. That is worth stating because it is the thing a
future reader will most want to know before lowering it again.

## Alternatives rejected

- **Pin `rusqlite` back to 0.37.** Verified working at both toolchains, and the
  narrowest possible fix. Rejected because it buys the floor by freezing a
  dependency for reasons unrelated to what the dependency does — and phase 8, which
  owns the driver decision through ADR-0022, would inherit a version choice made
  by an MSRV constraint rather than by anything about SQLite.

- **Give `happenstance-sqlite` its own higher `rust-version`.** Honest
  per-package metadata: the crate is `publish = false`, so the three published
  crates could have kept 1.85 truthfully. Rejected on moving parts — CI would need
  a second toolchain installed, and `cargo test --workspace` at 1.85 would still
  fail, so that job needs an exclusion list that then has to be maintained. It also
  makes "the MSRV" ambiguous in conversation, which is worse than making it high.

- **Raise it only as far as `cfg_select!` requires.** A smaller number, and it
  would have preserved some headroom. Rejected because the exact floor is not
  discoverable from any metadata — it would have to be found by bisecting
  toolchains against a dependency's build script, and it would be invalidated by
  the next dependency that declares nothing. A number tied to the pin is at least
  a number with a reason.
