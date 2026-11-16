# #472: two-devices-one-seat adjudicates on the hook

Goal: central keeps its rule through `happenstance_sync::runner::Adjudicate` mounted on central's
`StoreSyncPeer` (`with_adjudicator`, served by `serve_sync_adjudicated`), so each device's pushed
sale lands in one write with the refund it earns (SY-2, SY-7, SY-10). Drop "by convention" from the
example, `docs/sync-a-device.md` and `docs/choose-a-topology.md`.

## Design

- `domain/decide.rs` (moved out of `domain.rs`): `schedule`, `sell_seat`, and
  `adjudicate(ledger, earlier, arriving) -> Vec<BoxOffice>`: fold what central holds, then the
  batch's earlier groups, then the arriving group, each event only if central does not hold it yet;
  refund each arriving sale that lands past capacity. A re-delivered sale is never refunded.
- `rule.rs`: `CentralRule { central: Central, show: ShowId }` (`Clone`; a store handle and a name),
  bare `Adjudicate` (the `Uplink` is bare too), `RuleError` with `Classify`.
- Refunds are built as unframed `Event`s (no codec tag in metadata): the typed layer has no public
  encoder for an `Event`, and an untagged payload decodes under the reader's codec.
- `Scenario<A = CentralRule>`: `open(dir)` mounts central's rule; `judged_by(dir, rule)` mounts any
  rule whose error is `RuleError`, so the contrast test composes a central with a rule that answers
  nothing. No flag or environment variable; the binary cannot reach it.
- `Scenario::adjudicate` (the convention) is gone. `Scenario::judge_again` asks central's rule about
  everything central holds, as one re-delivered group, and returns how many events it answered.
- `TimelineView::read(app, show)` filters `WHERE show = ?1` on both tables.

## Tests

- [x] red: `the_refunded_sale_is_the_last_to_reach_central` without `adjudicate()` (no refund yet).
- [x] `adjudication_is_idempotent`: `judge_again` answers 0 and re-syncs write nothing.
- [x] `without_adjudication_the_show_stays_oversold`: same assertions, central judged by a rule
  that answers nothing.
- [x] `domain::decide::tests`: a re-delivered sale earns nothing; an arriving sale past capacity is
  refunded; an earlier group in the batch counts.
- [x] `timeline::tests::the_view_reads_only_its_show`.

## Not changed

- `Rule`'s serialised name `"FirstToReachCentralKeepsTheSeat"`: changing it rewrites the pinned
  bytes in `domain/tests.rs` (H-12/H-16). Left for the owner.

## Progress

- Red: `cargo test --test story` failed to compile: `no method named judge_again`, `no associated
  function judged_by`, `unresolved import RuleError`. Red: `the_view_reads_only_its_show` failed to
  compile: `TimelineView::read` takes 1 argument but 2 were supplied.
- Green: 5 unit, 12 story tests.
- Mutant 1 (a rule that always refunds the phone's arriving sale, no capacity order):
  `the_refunded_sale_is_the_last_to_reach_central` failed, left `["phone-1"]`, right `["tablet-1"]`.
  Reverted.
- Mutant 2 (`land` folds an event central already holds, so a re-delivered sale counts as new):
  `adjudication_is_idempotent` failed (`judge_again` answered 3, expected 0), as did
  `a_sale_delivered_again_earns_nothing`, `the_story_converges`, `the_binary_tells_the_story` and
  `the_phone_is_refused_after_converging` (the story stops: "central's rule, applied twice,
  refunds once"). Reverted.
- Changed expectation: `both_devices_sell_the_last_seat_and_central_holds_both` asserted
  `held() == 4`; with the refund landing in the same write as the second sale, seats held is 3 and
  the SY-1 claim is now `sales().count() == 4`. Flagged to the owner.
- `cargo xtask affected --base origin/main` passed; `narrative`, `lint-pages`, `spec-trace`,
  `lint-constitution`, `proof-artefact` passed.
