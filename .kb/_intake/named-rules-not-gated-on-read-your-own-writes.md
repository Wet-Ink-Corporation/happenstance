# The named event-store rules are not gated on `READ_YOUR_OWN_WRITES`

Found while planning #79 (IMPL-7), 2026-10-19. Raw material, not an atom. A sibling
of #79 and deliberately outside its scope.

## What is true now

`Fixture::READ_YOUR_OWN_WRITES` is read by the model family alone. No rule in
`crates/happenstance-testkit/src/suite.rs` carries `require!(F: READ_YOUR_OWN_WRITES)`,
yet many of them append and then read the append back. `happenstance-postgres`
and `happenstance-neon` decline the capability and pass those rules only because
the server is quiet under `--test-threads=1`: nothing else holds
`pg_snapshot_xmin` back while they run. `crates/happenstance-postgres/tests/runtime_seam.rs`
already carries one test that had to tolerate the lag by hand (#272).

## Why it matters

A rule that passes only on a quiet server is the flake `[FROZEN]` CF-33 names as
worse than none. #79 fixed this for the model family by running a trailing model
instead of skipping. The named rules have no equivalent yet.

## Options, for whoever takes it

1. Gate each read-back rule on the capability, so a declining fixture reports a
   skip. Honest, and it removes coverage from the two adapters that most need it.
2. Give the read-back rules a trailing form, as the model family now has. More
   work, and it keeps the coverage.
3. Leave it, and record in each adapter's README that its CI must run serially on
   a quiet server. That is what is true today, unrecorded.
