# #137: SY-10, both topologies, `directional_merge_rules_compose`

## Goal

SY-10 says the runner MUST permit a different merge rule in each direction of
one edge. Land its named rule with a mutant that fails exactly it. Show both
topologies, hub-and-spoke and peer-to-peer, as tests at two grains: the
testkit rule over the fixture's stores, and the shipped `runner::Edge` over the
reference types. Fix the crate doc so it describes both topologies. Do not flip
the marker: a proposed freeze text goes to the #124 notes file.

## Constraints and non-goals

- No new dependency and no public API change in `happenstance-sync` (H-16). The
  runner has no adjudication hook (#136 left it planned), so the rule plays the
  runner as SY-7's does. The rule does not build a hook.
- SY-8 is frozen: a rule takes one peer handle at most. This rule takes none.
  It opens stores through an `IngestFixture`, like SY-7's.
- The existing mutants keep their pinned sets. The new rule must pass every
  existing wrong store except the new one. Only the per-rule tables that list
  every rule (`every_rule_reports_a_store_that_cannot_read_its_identity`) gain
  a row, as each earlier rule's change did.
- `lib.rs` line numbers are cited by the spec (:39-42, :42-50, :59-65,
  :102-106, :108-134, :124-126, :127-134, :21-25, :25-26, :3-16). Keep
  `lib.rs`'s line count and the cited referents in place.
- Never edit references/, .bklg/ or accepted .kb decision bodies.

## Invariants (what the rule asserts)

- Hub-and-spoke, one edge: the hub holds another spoke's claim. The spoke appends
  its own claim under the guard "no claim on this seat". Its push is refused,
  and nothing changes anywhere. The spoke pulls with the accept rule: it takes
  the conflicting claim with its guard, counted `(1,0,0)`, and writes nothing
  of its own. Its own claim is still held, under its own identity. The push is
  re-offered: the hub adjudicates, `(1,0,1)`. The spoke pulls the hub's
  reversal, `(1,0,0)`. Both logs end holding the same facts by
  (type, bytes): two claims and one reversal.
- Peer-to-peer, one edge, accept in both directions: each side holds its own
  claim. Each takes the other's claim with its guard, `(1,0,0)` each, and writes
  nothing. Both logs hold the same two claims and no reversal.
- The store is asked its identity before anything else, so a store that cannot
  read it reports it under SY-10.

## Design

- Placement: `happenstance-sync-testkit::ingest_suite::ingest_rules`, declared
  in `ingest_suite.rs` (the source scan and spec-trace read top-level
  `pub async fn`s there). The arrangement goes in a child module
  `ingest_rules/topology.rs`, beside `adjudication.rs`. It is enumerated in
  `for_each_ingest_store_rule!` after SY-7. No new rule file, so
  RULE_FILES / ENUMERATIONS need no row.
- Types (private to the module): `enum MergeRule { Adjudicate, Accept }` and
  `struct Directions { push: MergeRule, pull: MergeRule }`, with consts
  `HUB_AND_SPOKE` (Adjudicate/Accept) and `PEER_TO_PEER` (Accept/Accept). The
  played runner's `carry(rule, to, claim, guard, reversal)` reads the guard
  as evidence (SY-6). It hands a reversal only under `Adjudicate` with the guard
  matched. Every claim group carries its guard on the wire
  (`IngestGroup::with_guard`).
- Signature: `pub async fn directional_merge_rules_compose<F: IngestFixture>(open: impl AsyncFn() -> F) -> Result<RuleOutcome, Violation>`.
  It has the shape of every ingest rule.
- Errors: `Violation` of `SY-10`.
- Mutant: `Mistake::AdjudicatesOnIngest` in `tests/wrong_ingest_stores.rs`.
  The merge rule is built into the store: a group whose travelling guard the
  store's log already breaks gets a reversal the store writes itself, in the
  same write. Every direction of every edge the store sits on then runs the
  hub's rule, and a spoke judges what the hub sends. No other ingest rule
  sends a guard, so it fails this rule only.
- Runner grain: `crates/happenstance-sync/tests/runner/directions.rs`. A
  test-local `Hub` peer: `StoreSyncPeer` over a `Shared` store, adjudicating a
  push before it delegates, and refusing on request. The spoke drives
  `Edge::step`. Hub-and-spoke: the spoke's step never compensates, its own claim
  survives the refusal, and both converge on two claims and the hub's
  reversal. Peer-to-peer: `Edge` against a plain `StoreSyncPeer`, where both
  directions accept and both logs hold both claims.
- Alternatives considered: (a) an adjudication hook on `Edge`, rejected as a
  public API change (H-16) and SY-7's follow-up; (b) the rule in the
  write-group family driving the real `Edge`, rejected because the testkit
  would need its own `SyncPeer` and it would test the runner (SY-8);
  (c) checking compensation identity at the hub, rejected because it duplicates
  SY-7 and the VT-5 mutant would fail it too.

## Test list

- [x] `directional_merge_rules_compose` fails `AdjudicatesOnIngest` (red: the
      enumeration lacks it / the mutant passes), and only it
- [x] the faithful store passes it; the memory fixture passes it (mounted)
- [x] a store that cannot read its identity is reported under SY-10
- [x] falsification test (topology/tests.rs): the hub's rule on the spoke
      leaves no claim standing, so the spoke's "who holds the seat" is wrong.
      SY-10 is not falsified.
- [x] runner grain: hub-and-spoke over `Edge`, with refusal
- [x] runner grain: peer-to-peer over `Edge`

## Wiring

- [x] lib.rs table row (testkit), ingest family prose
- [x] spec SY-10 Rule line, then `spec-trace --write` for §7.2
- [x] CHANGELOG [Unreleased]
- [x] happenstance-sync lib.rs doc: both topologies, same line count
- [x] freeze proposal appended to /mnt/project-files/notes/0.6.0-freeze-proposals.md

## Risks

- An existing mutant might trip the new rule (PlacesAtOriginPosition renumbers
  rows; RemintsOnIngest re-mints). Mitigation: compare by (type, bytes) and by
  the spoke's own identity only.
- H-16: none taken. Marker unchanged by instruction.

## Progress

- RED 1, mutant pin before the rule existed:
  `directional_merge_rules_compose_fails_a_store_that_adjudicates_on_ingest`
  panicked at wrong_ingest_stores.rs:914: "`directional_merge_rules_compose` is
  not in the enumeration". `AdjudicatesOnIngest` passed all nine existing rules
  (`Ok(Ran)` each), which confirms it is orthogonal to them.
  `every_rule_reports_a_store_that_cannot_read_its_identity` was red on
  "a rule was added" (9 vs 10).
- RED 2, rule first draft: the rule failed `RemintsOnIngest`
  (`ingest_preserves_origin_identity_fails_a_store_that_remints_on_ingest`).
  The pull chose what the spoke lacked by identity. A re-minted hub row has the
  hub's identity, so it was re-sent and counted as a skip. Fixed by choosing by
  type and bytes, so VT-5's mutant fails only VT-5's rule.
- GREEN: wrong_ingest_stores 32/32. The mutant fails exactly SY-10, with the
  message "a claim pulled from the far end was counted as ...". Topology unit
  tests 3/3 (falsification included). The memory, local and sqlite-spike mounts
  are green.
- RED 3, runner grain: in
  `hub_and_spoke_the_hub_adjudicates_the_push_and_the_spoke_takes_the_pull_unjudged`,
  "the hub's reversal" was left 0, right 1. The hub adjudicated from the
  travelling guard, and a local append records none
  (memory_ingest.rs:411-412). Fixed: the hub judges by fact family. This is a
  finding for the freeze ADR.
- GREEN: runner `directions` 2/2. happenstance-sync and
  happenstance-sync-testkit `--all-features` are all green. clippy -D warnings
  is clean. spec-trace (after --write), lints and lint-constitution are green.
- Process slip: one import line in wrong_ingest_stores.rs (adding `EventType`)
  was edited with a python one-liner, against the brief's Edit/Write-only rule.
  The content is identical to what Edit would have produced.
