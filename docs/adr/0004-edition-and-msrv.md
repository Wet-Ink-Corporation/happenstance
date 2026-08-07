# ADR-0004: Rust 2024 edition, MSRV 1.85

- **Status:** accepted — **provisional**
- **Date:** 2026-08-05

> **Provisional.** Authored on 2026-08-05 alongside the initial scaffold, before
> any of the code this decision constrains existed.
>
> The Policy section below describes an obligation to downstream users. **There
> are none** — nothing has been published, so `rust-version = "1.85"` is a
> self-imposed constraint that can be raised at any time at zero cost to anyone.
> Do not treat it as a hard limit when weighing a dependency: it is a preference
> until first publish, and a promise afterwards. It has also never been checked
> against a local 1.85 toolchain (see Consequences).
>
> **Lifts on first publish**, at which point the Policy section becomes real.

## Context

A library crate has to state which compilers it supports, and the choice
interacts with the language features the design depends on.

What happenstance actually needs:

| Feature | Stabilised |
|---|---|
| `async fn` / RPIT in traits (`ADR-0001`) | 1.75 |
| `core::error::Error` (for `no_std`) | 1.81 |
| `Option::is_none_or` | 1.82 |
| Edition 2024 | 1.85 |

## Decision

Edition 2024, `rust-version = "1.85"`, toolchain pinned to 1.97.1 in
`rust-toolchain.toml`.

Edition 2024 is not incidental: its rule that return-position `impl Trait`
captures all in-scope lifetimes by default is what lets `EventStore::read`
return a stream borrowing `&self` without an explicit `+ '_`.

**Let-chains are avoided**, despite being stable in edition 2024, because they
only landed in 1.88. Where one would read naturally there is a `match` and a
comment saying why.

## Consequences

**Good.** 1.85 is old enough to be undemanding and new enough for everything the
design needs. Edition 2024 gets the lifetime-capture rules the ports rely on.

**Bad.** The MSRV is verified **only in CI**, by a dedicated job running
`cargo hack check --workspace --no-dev-deps --rust-version` on a pinned 1.85
toolchain. It has not been checked against a local 1.85 install. If that job
fails, raise the MSRV rather than working around it.

**Neutral.** `happenstance-core` is `no_std` + `alloc` under
`--no-default-features`, verified in the feature matrix. Nothing needs that
today; it costs a handful of `alloc::` imports and keeps the option open.

## Policy

An MSRV bump is a minor version bump, and is called out in the changelog.
