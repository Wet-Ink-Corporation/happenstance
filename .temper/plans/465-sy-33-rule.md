# #465, second half: SY-33's rule, `refusal_is_never_condition_derived`

## Goal
SY-33 is `[FROZEN]` and names a rule that was never written (§7.2's †). Write it in the
sync peer family, prove it fails a peer whose refusal varies with its log, and repair the
spec's evidence. Building the rule the clause already names changes no meaning.

## Constraints and non-goals
- No change to `SyncFixture` (a published trait): the rule uses `refusals(peer)` twice on
  one peer whose log it grows in between. If the trait had to change: BLOCKED (H-16).
- SY-34 is not touched (ADR-0108, #487 settled it).
- SY-38's identity-integrity refusal is the one permitted log-dependent refusal: the rule
  must never count it as condition-derived.
- No new dependency.

## Invariants
- Every refusal the fixture provokes answers the same (accepted, or refused with the same
  `FailureClass`) against the fresh receiver and against the receiver once it holds the
  rule's seed events.
- A push the receiver answered on its empty log is answered the same when re-sent (it now
  holds it), and when a fresh event under the same guard is pushed after the receiver holds
  events that violate that guard.
- The rule compares refusals only, never acks: SY-1 owns what an ack says, so
  `EvaluatesTheGuard` (acks nothing, no `Err`) stays SY-1's alone.

## Design
- Placement: `rules::refusal_is_never_condition_derived` in `sync_suite.rs` (the
  four-space `pub async fn` the orphan test and spec-trace read), delegating to a private
  `sync_suite/rules/condition_free.rs`, as SY-32's `retention.rs` does (sync_suite.rs is
  already ~1,970 lines; ARC-06).
- Registered in `for_each_sync_peer_rule!` under its own SY-33 heading.
- Private `enum Answer { Accepted, Refused(FailureClass) }`, built from a borrowed
  `Result<_, E: Classify>`, reading the class once per error (SY-36's rule owns the
  read-twice check).
- Origins `0xC1`, `0xC2`, `0xC3` (probe, seed, late probe), unused by any other rule.
  SY-38: the rule pushes only these foreign identities, so no push it makes is an own-claim;
  a fixture-declared SY-38 refusal stays unheld before and after (a refused claim writes
  nothing, and no push can make the receiver hold its own identity), so it answers the same
  both times and the carve-out is never mistaken for a condition-level refusal.
- Errors: `Violation` of SY-33, each message naming the provocation and both answers.
- Alternatives considered: two peers from two fixtures (compares two instances, not one log
  changing; and the fixture's provocations may depend on instance identity); a new
  `SyncFixture` method to seed the log (trait change, H-16, unnecessary since the port's own
  `push` grows the log).

## Test list (drive in this order)
- [x] wrong peer refusing a push that carries an event it already holds -> SY-33 only
- [x] wrong peer whose oversized-push class is `Split` on an empty log and `Stop` after -> SY-33 only
- [x] wrong peer refusing (`Err`) a group whose guard its log violates -> SY-1 and SY-33
- [x] faithful wrapper passes every rule (control)
- [x] every existing wrong-peer test still fails exactly its rule (new rule in the enumeration)
- [x] the memory, KV-capped, local (!Send), remote and store mounts are green
- [x] Postgres, Neon, Cloudflare and workerd mounts compile

## Decisions (H-16)
| Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|
| Change `SyncFixture`? | add a seeding hook / reuse `refusals` twice | reuse; no trait change | implementer, per the delegation | no trait change |

## Risks
- A live adapter whose far side refuses guarded or re-sent pushes would turn red in
  `live-*`; SY-1's and SY-31's rules already push both shapes there, so no new exposure.
- A fixture whose provocation is stateful in something other than the log (a quota spent by
  the first call) would read as condition-derived. None in the workspace is; documented.

## Progress
- (filled in below as the work runs)
- Red (2026-11-18): `tests/wrong_refusals.rs` written before the rule. `cargo test -p
  happenstance-sync-testkit --all-features --test wrong_refusals`: 1 passed, 3 failed, each
  `refusal_is_never_condition_derived is not in the enumeration`, and in every verdict list
  all eleven existing rules passed `RefusesWhatItHolds` and `ClassFollowsTheLog`: no rule
  before this one could fail either.
- First green attempt: `ClassFollowsTheLog` also failed SY-32's rule (it re-classed the
  pull's `Reseed` as `Stop`); the mutant now re-classes push refusals only, so it is SY-33's
  alone. `RefusesAViolatedGuard` fails SY-1 and SY-33, as designed.
- Green: the whole testkit (`--all-features`), all wrong_* files with the new rule in the
  enumeration; the rule ran and passed on the memory, KV-capped, local, remote and store
  mounts. xtask: SYNC_UNRESOLVABLE_RULE_NAMES 21 -> 20 (SY-33 entry and OWED_BY_465 gone),
  CLOUDFLARE_UNIT_TESTS names the served mount's new test, `spec-trace --write` dropped the †.
  `cargo xtask spec-trace`, `cargo xtask lints`, `cargo test -p xtask` green.
- Gate (TEMPER_BASE=5f71d44, --no-cache): GREEN, fingerprint fdc7bc352fb18024. Its three
  advisories (sqlite lib.rs:340, pruned_replica.rs:45, ingest_fixture.rs:195) are in files
  this change does not touch. spec-trace and lints green after the last edit.
