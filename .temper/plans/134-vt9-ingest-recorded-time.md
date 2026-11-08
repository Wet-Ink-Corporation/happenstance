# #134 · VT-9: ingest preserves `RecordedAt`, with mutants

## Goal

A conformance rule of VT-9's own in `happenstance-sync-testkit`'s ingest family:
an event accepted through ingest is held with the `RecordedAt` its origin
carried, unchanged. Named wrong implementations in
`tests/wrong_ingest_stores.rs`, each failing exactly this rule.

## Invariants and constraints

- The rule compares each held event's `RecordedAt` with the one the origin
  sent, and nothing else: no two held times against each other, no time
  against a position, no plausibility check against the harness's clock
  (VT-9's own prose, CF-33).
- Events are found by their distinct opaque bytes, never by identity, so a
  re-minting store fails VT-5's rule alone (the family's stated convention).
- Times stay inside Cloudflare's stated JS-safe range (±2^53 − 1): a time it
  refuses is its declared limit, not this clause.
- No literal local position or store-chosen time is asserted.

## Design

- `ingest_rules::ingest_preserves_the_recorded_time` (clause `VT-9`), origin
  `StoreId([0x76; 16])`. One local append first (the store stamps it), then one
  call of two groups: origin positions 2, 3, 5 in the first, recorded at
  −1,234 ms (before 1970, not a whole second), 1,700,000,000,123 and
  1,600,000,000,456 (earlier than its predecessor); position 8 in the second,
  at 1,650,000,000,789. Each sent event is found by bytes and its held time
  compared with the sent one.
- VT-5's `ingest_preserves_origin_identity` stops checking the recorded time:
  the check moves to the new rule, so the restamping mutant fails exactly one
  rule. The family table in `lib.rs` splits the row.
- Registered in `for_each_ingest_store_rule!`, so every mount (memory, local,
  wasm, sqlite spike, postgres, neon, Cloudflare `--lib`, workerd dispatch)
  runs it with no edit. `xtask/src/proof.rs`'s transcribed Cloudflare `--lib`
  names gain the rule.

## Mutants (tests/wrong_ingest_stores.rs)

| Mistake | What it does | Caught by |
|---|---|---|
| `RestampsRecordedAt` (moved from VT-5's rule) | stamps arrival time | every event |
| `TruncatesRecordedAtToSeconds` | stores whole seconds | sub-second times |
| `ClampsRecordedAtAtTheEpoch` | unsigned storage: negative becomes 0 | the pre-1970 event |
| `KeepsRecordedAtMonotonic` | max(origin time, previous row's time) | position 5's earlier time |
| `StampsEachGroupWithItsHeadsTime` | one bound time per group | positions 3 and 5 |

## Test list

- [x] each mutant test names the new rule and fails (red: not in enumeration)
- [x] the rule exists and is registered; every mutant fails exactly it
- [x] faithful store passes every rule; identity-unreadable lists VT-9
- [x] memory / local / sqlite mounts pass
- [x] postgres live mount passes (PG 17.10 container); neon compiled and listed, not run (no endpoint)
- [x] Cloudflare `--lib` on wasm32 under the Node 24 shim passes; workerd not run here

## Decisions (H-16)

- Moving the recorded-time assertion out of VT-5's rule and retargeting
  `..._fails_a_store_that_restamps_recorded_at` at the new rule: required by
  the request's "failing exactly this rule"; reported for the owner.
- VT-9's marker is not moved (freezes held for the owner).

## Progress


- Red 1: `cargo test -p happenstance-sync-testkit --test wrong_ingest_stores recorded`
  failed 5 of 5 at `wrong_ingest_stores.rs:806`: "`ingest_preserves_the_recorded_time`
  is not in the enumeration" (the restamp mutant then failed VT-5's rule instead).
- Green: rule added and registered, VT-5's time check moved; 29/29 in
  `wrong_ingest_stores`, each of the five mutants `Err("VT-9")` and every other
  rule `Ran`; the faithful store `Ran` on all eight.
- Mounts: memory, local, memory wasm32, SQLite spike, Postgres live, Cloudflare
  `--lib` on wasm32 (Node 24) all pass the new rule.
