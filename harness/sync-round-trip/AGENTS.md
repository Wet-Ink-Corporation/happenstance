# happenstance-sync-round-trip-harness

## Purpose

An instrument, never published. It replicates a native SQLite store through
the two SQL-backed sync peers, `NeonSyncPeer` and `StoreSyncPeer` over
`PostgresEventStore`, and checks that payload bytes come back identical both
ways and that a replayed batch changes nothing (#131).

## Layer

Test instrument, beside `harness/workerd`. Not an adapter, so the rule that no
adapter depends on another (SPECIFICATION §1.5) does not reach it; that rule is
why the round trip lives here and not in either adapter.

## Allowed dependencies

Dev-dependencies only, host-only, path-only for workspace crates:
`happenstance-core`, `happenstance-sync`, `happenstance-sqlite` (`sync`),
`happenstance-neon` (`sync`), `happenstance-postgres` (`sync`), and the crates
the adapters' own live tests already use (`tokio`, `sqlx`, `testcontainers`,
the `hyper`/`rustls` stack, `thiserror`). Nothing may add a node to
`Cargo.lock`.

## Public surface

None. `src/lib.rs` is crate documentation only; the work is the test binary
`tests/round_trip/`.

## Invariants

- Every live case is `#[ignore]`d with its reason, and fails (never passes)
  when reached with `--ignored` and no server.
- The helpers in `tests/round_trip/main.rs` name only the ports, so a far end
  is two lines.
- Never assert a literal local position.

## Error types

None of its own beyond the restated Neon transport's error.

## Test strategy

`neon::` cases run in CI's `live-neon` job and `postgres::` cases in
`live-postgres`, each as a strict list/run/assert trio. Locally:
`HAPPENSTANCE_POSTGRES_URL=... cargo test -p happenstance-sync-round-trip-harness
--test round_trip -- --ignored --test-threads=1 postgres::`.

## Do not

- Publish it, or give it a normal dependency.
- Depend on it from any crate.
- Weaken a case to pass without a server.

## Decisions log

- 2026-11-08: placed here rather than as a dev-dependency between adapters,
  decided by the coordinator under the owner's standing authority, after
  #130's `harness/workerd` precedent.
