# #133 — VT-6 `restored_peer_does_not_reissue_identities`, with a negative control

## Goal
A sync-testkit rule that restores a peer's storage to an earlier point, follows the
adapter's documented post-restore procedure, appends, and fails if the peer hands back an
`EventId` it already issued for a different event. Green on the in-memory reference and on
the Postgres peer; red on a re-issuing mutant and on Postgres with the re-mint skipped.

## Constraints and non-goals
- Additive public API in `happenstance-sync-testkit` only. No new dependency, no change to a
  published port, no change to any existing test's expectation (H-12, H-16).
- No literal positions asserted. No payload decoded (SY-35): events are compared whole.
- VT-6 stays `[PROVISIONAL]`; the freeze text goes to the notes file for #124.
- Not Neon, not the Durable Object: they inherit nothing (separate family) and are follow-ups.
- No CI change: the Postgres mount lives in the lib target CI's live-postgres job already runs.

## Invariants
- After a restore and the procedure, every identity the peer hands back either names the event
  it named before the restore, or was never handed back before.
- The rule can tell a real restore from a no-op: the kept event survives with its identity, the
  lost event is gone, the new event is there. Otherwise the fixture is reported, not passed.

## Design
- Placement: a **new family**, `src/restore_suite.rs` (`restore_rules`) + `src/restore_fixture.rs`
  (`RestoreFixture`, `MemoryRestoreFixture`), mounted by `restore_conformance!`.
  - Why not the sync-peer family with a `RESTORE` capability: every mutant harness there
    (`wrong_peers`, `wrong_scopes`, `wrong_orders`, `wrong_limits`) asserts every non-target rule
    is `Ok(Ran)`; their peers wrap `MemorySyncPeer`, which issues no identity and cannot restore,
    so the new rule would be `Skipped` there and every existing test would need its expectation
    changed (H-12/H-16). Same for the ingest and write-group families.
  - The rule needs fixture co-operation no other family needs: a local writer, a backup and a
    restore. A trait of its own makes them required methods, not defaulted capabilities.
- `pub trait RestoreFixture { type Peer: SyncPeer; connect; append_local(&self, &[Event]);
  back_up(&self); restore(&self) }`, each fallible as `Result<(), Violation>`. `restore` =
  put storage back to the backup **and** run every step the adapter documents for after a restore
  (VT-6's re-mint on a mint-once adapter). The rule connects a fresh peer after it.
- `MemoryRestoreFixture`: `StoreSyncPeer<MemoryIngestHandle, MemoryIngestHandle>` over a
  `MemoryIngestStore` held in a `RefCell<Arc<_>>`; backup = its write groups read whole; restore =
  a fresh `MemoryIngestStore::new()` (the re-mint: a fresh incarnation) into which the backed-up
  groups are ingested, which keeps each event's identity, time and group.
- Postgres: `impl RestoreFixture for PostgresPeerFixture` in a new `restore_family.rs` beside
  `peer_family.rs`; backup = sequence `last_value`/`is_called` + identity row; restore = delete rows
  above, `setval`, restore the row, then `remint_identity()` (the documented procedure, ADR-0086).
  Negative control: the same fixture with the re-mint skipped, asserted `Err("VT-6")`.
- xtask: `RULE_FILES` gains `restore_suite.rs`; the VT-6 `Scheduled` entry in
  `UNRESOLVABLE_RULE_NAMES` goes (68 → 67).

## Test list (drive in this order)
- [x] memory: `restore_conformance!(MemoryRestoreFixture::new())` green
- [x] mutant control: faithful device peer passes
- [x] mutant: identity keyed on the device (no re-mint) → VT-6, "reissued"
- [x] mutant: restore that restores nothing → VT-6, the lost event survived
- [x] mutant: restore that re-stamps history under the new identity → VT-6, kept event's id moved
- [x] meta: `no_orphan_restore_rules`
- [x] Postgres (live): rule green; negative control (no re-mint) red with VT-6

## Decisions (H-16)
| Decision | Options | Recommendation | Decided by | Outcome |
|---|---|---|---|---|
| New public items in the testkit | additive | additive is pre-approved by the caller | caller | proceed |

## Risks
- A restore fixture that does nothing would pass a reissue-only check: the rule's three
  restore-shape checks catch it, pinned by the `RestoresNothing` mutant.
- Postgres sequence semantics: `setval(.., is_called)` restores exactly; the live run proves it.

## Progress
- Memory mount green first run (2 tests: tokio + blocking emitters).
- Red 1: reissue check disabled → `..._keys_its_identity_on_the_device` FAILED at
  wrong_restores.rs:248 ("the rule must reject its mutant": the rule returned Ran).
- Red 2: kept-id and lost-event checks disabled → `..._restores_nothing` and
  `..._stamps_its_incarnation_on_read` FAILED at wrong_restores.rs:248.
- Red 3 (Postgres, live): mount with the re-mint skipped → `VT-6 violated: the store issued
  c826258e…:2 to a LostToTheRestore event and again to a WrittenAfterTheRestore event …`.
- Green: all four mutant tests, memory mount, Postgres mount + negative control
  (55/55 ignored lib tests in happenstance-postgres against 17.10).
- xtask: RULE_FILES, ENUMERATIONS, UNRESOLVABLE (68 → 67), spec_trace.rs line-neutral;
  one `.kb/open-questions` citation into lints.rs repointed (:2591 → :2597, CITATION_SCAN_EXCLUDE).
- Gate GREEN (a40db4659cf9633b). Committed f348cbf9.
