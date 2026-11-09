# #140 — SY-27, SY-28 on `round_trip_preserves_the_whole_log`; SY-29 renewal proposed

## Goal
Build the round-trip rule SY-27 and SY-28 name (it does not exist yet: grep finds
no `round_trip_preserves_the_whole_log` anywhere in `crates/`), with a filtered
mutant red and a faithful control green, so a consolidated freeze PR can flip
both markers. SY-29 keeps its marker; the renewal record is proposed in the report.

## Constraints and non-goals
- No marker changes (SY-27/28/29 stay `[PROVISIONAL]` here). No ledger/runbook marker edits.
- No new dependency, no port change. Additive rule in a published crate (a minor, as lib.rs says).
- Avoid `crates/happenstance-sync/src/runner*`, `store_peer.rs`, write-group files.
- Edits to `sync_suite.rs` and `lib.rs` tables additive and compact (PR #432 touches them).
- Never `.bklg/`, `references/`, accepted ADR bodies.

## Invariants
- The rule holds one peer (SY-8); the rule plays the other side.
- No payload decoded (SY-35); events compared whole (`ReplicatedEvent: PartialEq`).
- No position compared, no literal positions asserted; identities are `EventId`s the rule minted.

## Design
- Placement: `pub async fn round_trip_preserves_the_whole_log` in `sync_suite.rs`'s
  `rules` (xtask lints, spec-trace and `no_orphan_sync_rules` scan that file), its
  log builder in a submodule `src/sync_suite/rules/whole_log.rs` (the
  `ingest_suite/ingest_rules/batch_scope.rs` precedent) so the big file grows little.
- The rule: push a log spanning several slices — two origins, gapped origin
  positions, three event types, tagged and untagged, one with metadata, one
  two-event group — then pull from the start to an empty batch, then require
  the set of `EventId`s handed back to equal the set pushed, each event equal
  byte for byte (id, recorded time, type, tags, data, metadata). An event handed
  back twice with different bytes is a violation; an identity never pushed is
  deliberately not rejected (see the rule's "Does not reject").
- Errors: `Violation` under `SY-28` (the rule's own clause; SY-27 is its premise).
- Mutant: `tests/wrong_scopes.rs`, a `ScopedPeer` over `MemorySyncPeer` that
  withholds the events matching a `Query` from every pull — the scoped hub /
  filtered spoke. Run with three scopes (by tag, by type, by type+tag) each red
  on exactly this rule; `withheld: None` is the control, green on every rule.
- Alternatives: adding a `Mistake` variant to `wrong_peers.rs` — `Mistake` is
  `Copy` and its matches are exhaustive, so a `Query`-carrying variant touches
  every arm; a separate file keeps the diff additive.

## Test list
- [x] the rule against `MemorySyncFixture` (and every existing mount) is `Ran`
- [x] a peer withholding the matches of a tag query fails exactly this rule (SY-28)
- [x] ... of a type query
- [x] ... of a type-and-tag query
- [x] the faithful scoped wrapper (no query) passes every rule
- [x] every existing mutant in wrong_peers.rs still fails exactly its own rule

## Decisions (H-16)
| Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|
| New pub rule in a published crate | add / not | add (requested; additive minor) | lead (#140) | add |
| Markers | flip / leave | leave; propose text | lead | leave |

## Risks
- A mount (store/remote/kv-capped/local) fails the new rule: run each.
- An existing mutant also fails the new rule: wrong_peers runs the whole enumeration.

## Progress
- Orient: `round_trip_preserves_the_whole_log` did not exist (no match under crates/). The
  clause text, the ledger rows (ledgers.md:332-334) and 13-sync.md:210-214 agree with plan
  row 12: SY-27/28 freeze on this rule, SY-29 renewed past 1.0.
- RED 1: `cargo test -p happenstance-sync-testkit --test wrong_scopes` — 4 failed, each
  `the whole-log rule is not in the enumeration` (wrong_scopes.rs:140, :170). The same run
  shows every existing sync rule `Ok(Ran)` against all three scoped mutants: no rule in the
  suite saw a filtered peer before this one.
- GREEN: rule + `sync_suite/rules/whole_log.rs`; whole testkit `--all-features` green,
  wrong_peers (every existing mutant still fails exactly its own rule) and every mount
  (memory, local, remote, store, kv-capped) green on the new rule.
- Spec Rule lines updated, §7.2 regenerated (dagger gone), CHANGELOG entry (CF-29).
