# Plan: #136, SY-7 and SY-23 (adjudicator falsification and merge rule)

## Goal
Land `only_the_adjudicator_compensates` (SY-7) and `event_id_order_is_identical_on_every_peer`
(SY-23) in `happenstance-sync-testkit`, each failing a named wrong implementation kept in
`tests/` beside a faithful control with the exact failing set asserted; and SY-7's falsification
test with two adjudicator configurations, divergence recorded. No marker moves (the freeze PR does).

## Findings that shape the design
- Nothing in the workspace authors a compensation: `Edge::step` and `StorePeer` ingest with
  none, and `store_peer.rs` says the adjudication hook is "planned with SY-7 (#136)". Building
  that hook is a public API addition to `happenstance-sync` (H-16) in files lane #420 rewrites,
  so it is out of scope and reported as a decision.
- So the testkit plays the runner and holds the assignment (as `sync_model` plays the runner).
  What an *adapter* controls under SY-7 is the identity a compensation is minted under: the log
  records authorship as `EventId::store()`. SY-7's rule therefore sits in the ingest family.
- SY-23's order is core's derived `Ord`; what an adapter controls is that every peer ends up
  holding the same identities whatever the arrival order. In the ingest family any SY-23 check
  also fails `RemintsOnIngest`, which `fails_exactly(VT-5)` pins (changing that expectation is
  H-16), so SY-23's rule sits in the peer family, where no existing wrong peer touches identity.
- Existing `fails_exactly` tests run the whole enumeration, so both new rules must pass every
  existing mutant. Claims carry distinct bytes (content deduper), the SY-7 rule never meets the
  two sides (a re-minting store would misattribute the forwarded compensation, VT-5's subject).

## Placement
- `ingest_suite.rs`: rule `only_the_adjudicator_compensates`; helpers in child module
  `ingest_suite/ingest_rules/adjudication.rs` (the runner, the assignment, the falsification test
  as `#[cfg(test)]`, over `MemoryIngestFixture`).
- `sync_suite.rs`: rule `event_id_order_is_identical_on_every_peer`.
- `tests/wrong_ingest_stores.rs`: `Mistake::FilesCompensationUnderAPlaceholder`; `WrongFixture` mints a
  fresh identity per instance (two-store rule).
- `tests/wrong_orders.rs` (new, keeps clear of #420's `wrong_peers.rs`): a peer that dedupes by the
  highest origin position seen, beside its faithful control.

## Test list
- [x] SY-7 rule green on MemoryIngestFixture and every existing ingest mount
- [x] SY-7 rule fails FilesCompensationUnderAPlaceholder, exactly (FilesCompensationUnderTheOrigin was dropped: it also failed SY-30 and SY-18)
- [x] falsification: one adjudicator -> one compensation, logs converge after meeting
- [x] falsification: two adjudicators -> two compensations, two identities, both claims reversed
- [x] SY-23 rule green on memory, store, remote, local, kv-capped peers and every wrong peer
- [x] SY-23 rule fails a watermark-deduping peer, exactly; its faithful control passes all

## HITL
- Adjudication hook in `happenstance-sync` (public API): not built; reported.

## Progress
- Red: `only_the_adjudicator_compensates_fails_a_store_that_files_compensation_under_a_placeholder`
  with the authorship check disabled: "`only_the_adjudicator_compensates` must reject its mutant
  left: Ok(Ran) right: Err("SY-7")". Green with it restored.
- First mutant tried, `FilesCompensationUnderTheOrigin`, was rejected as non-specific: the
  misfiled id lands in the origin's identity space and also fails SY-30 and SY-18 (shadowing).
  Replaced by `FilesCompensationUnderAPlaceholder` (nil `StoreId`): fails SY-7, SY-38 skips.
- Red: falsification test with the second side forced non-adjudicating: "every side's reversal
  is held" left 1 author, right 2. Green restored.
- Red: `event_id_order_is_identical_on_every_peer_fails_a_peer_that_dedupes_by_high_water_mark`
  with both of the rule's comparisons disabled: left Ok(Ran), right Err("SY-23"). With only the
  identity comparison disabled it stays green, because the byte comparison of the zipped,
  shifted sequences catches the missing event too. Green restored.
- Mounts: memory, local, store, remote, kv-capped peers and every wrong peer/topology green;
  sqlite ingest spike green; live Postgres (container) green for the new rule. Postgres's
  ingest family is flaky in parallel on this host with and without the new rule (pre-existing);
  serially all eight rules pass. Neon and workerd not run here.
- Citations: the spec grew 10 lines at SY-7's Rule and 6 more at SY-23's. Every `SPECIFICATION.md:N`
  citation past line 7218 in maintained files was checked against the base: none pointed at its
  referent before this change (all already stale or historical), so none was shifted.
