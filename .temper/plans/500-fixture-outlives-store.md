# #500: an ingest rule keeps its fixture alive as long as the store

## Goal

SY-7's `only_the_adjudicator_compensates` (`side`, `adjudication.rs:184`) and
SY-10's `directional_merge_rules_compose` (`end`, `topology.rs:302`) write
`let store = open().await.connect().await;`. The fixture is a temporary, so it
drops at the end of that statement while the rule keeps the store. A fixture that
tears its backing down on drop (the SQLite fixture's temp file; #499 tolerated
os error 32 on Windows) does so under a live handle.

## Design

- `Side<S>` and `End<S>` gain a fixture field. Both are generic over the store
  only, so each gains a second type parameter `F`, and the store type is read as
  `F::Store` at the constructor. Field order: `store` first, `fixture` last, so
  the fixture drops after the store (struct fields drop in declaration order).
  Both types are private to their modules: no public API change (H-16 not hit).
- Owned fixture, not borrowed: the opener returns it by value and nothing else
  holds it, so the struct is its owner (OWN-02). No `Rc`/`Arc`.
- No other site: `grep -rn 'open().await.connect()\|\.await\.connect()\.await'`
  over `crates/happenstance-sync-testkit` finds exactly these two.

## Test list (TST-01)

- [x] `tests/wrong_fixture_lifetimes.rs`: `DropsUnderALiveHandle`, a faithful
  fixture over `MemoryIngestFixture` whose handles carry a shared liveness count
  and whose `Drop` records, in a shared tally, every drop while a handle it gave
  out is still alive. The whole ingest enumeration runs against it: every rule
  must pass as before, and the tally must be zero for every rule.
- [x] Red at the base: SY-7 and SY-10 record a drop under a live handle.
- [x] Green after the fix.

## Progress

- Red (before the fix), `cargo test -p happenstance-sync-testkit --test wrong_fixture_lifetimes`:
  `no_rule_drops_its_fixture_under_a_live_handle` failed with
  `rules that dropped a fixture under a live handle: [("only_the_adjudicator_compensates", 2), ("directional_merge_rules_compose", 4)]`.
- Green after keeping the fixture in `Side` and `End`: 3 of 3 in that binary,
  and every test in `-p happenstance-sync-testkit --all-features` passes.
- The field is `_fixture`, private: a field read by nothing trips `dead_code`
  under `-D warnings`, and a leading underscore is the drop-guard idiom that
  says so without an `#[allow]` or `#[expect]` (H-05).
- `Side`/`Conflict`/`End`/`Edge` gain a second, unbounded type parameter `F`
  (API-21), and every helper over them gains it too.

## HITL decisions

None. Private types only; CHANGELOG line under Unreleased / Changed.
