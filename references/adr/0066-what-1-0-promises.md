# ADR-0066 — What 1.0 promises

- **Status:** accepted, 2026-09-29, on the owner's decisions of the same day,
  §8's three included.
- **Date:** 2026-09-29
- **Phase:** 16, *Define 1.0* (`runbook/phases/16-define-1-0.md`)
- **Takes as input:** D-1, `kb-decision-wi-40b321` — *sync inside 1.0*. This
  record writes D-1 down and does not argue it again.
- **Freezes:** CF-39 (§3). It is the only clause this record moves.
- **Closes:** `kb-open-question-adapter-version-lockstep-001` (§4),
  `kb-open-question-workerd-runner-absent-001` (§6) and
  `kb-open-question-cloudflare-feature-gate-001` (§9). Answers sub-question 4 of
  `kb-open-question-es-6-unwritable-rule-001` (§5) and leaves the rest of that
  question open.
- **Sits beside:** ADR-0067 (the MSRV after 1.0), ADR-0068 (ADR-0022 §§8, 9
  and 16), ADR-0069 (`QueryItem`'s total constructor), ADR-0070 (the runner's
  named chunk) and ADR-0071 (ES-10 stays global). Phase 16 wrote all six, and
  this is the charter the other five hang from.
- **Evidence:** a survey of every non-`[FROZEN]` clause, every open question and
  every charter input, taken on 2026-09-29 by nine read-only agents. Then an
  adversarial pass over each freeze and closure the survey proposed, with two
  lenses per freeze: *evidence* (did the marker's own condition happen in the
  tree?) and *promise* (can a pending phase plausibly change the text?). A
  freeze survived only if neither lens refuted it. Citations below were read at
  `HEAD` on `lane/phase-16-define-1-0`.

## Context

Nothing in the repository said what `1.0.0` is. The nearest statements were
`SECURITY.md`'s *"Pre-1.0, and honestly so"*, `CHANGELOG.md:13-33`'s account of
what semver covers and what it leaves out, ADR-0037's MSRV promise, and
`HS-I0006`'s charter, which named 1.0 as an explicit non-goal. Phases 17 to 21
are all sequenced against 1.0. If nobody writes the target down, each phase will
work it out again, and each will get a different answer.

Phase 21 has to check that 1.0 holds, and it should be able to do that one
clause at a time rather than by judgement. So this record does two things. It
names what is promised, and it gives every provisional or deferred clause on
that surface exactly one fate, in a form a lint can check.

## Decision

### §1 The crate set

1.0 promises **nine crates**, listed by name:

| Crate | Published today | What 1.0 needs |
|---|---|---|
| `happenstance-core` | `0.3.2` | the contract; §2's freezes |
| `happenstance` | `0.3.2` | the typed layer; phase 18 lifts `unstable-projection` or §5 exempts it |
| `happenstance-testkit` | `0.3.2` | the bar; §4's rule semver |
| `happenstance-sqlite` | `0.3.2` | — |
| `happenstance-cloudflare` | `0.3.2` | conformance on `workerd`, §6 |
| `happenstance-postgres` | `0.3.2` | `naive-arm` removed or exempt, §5 |
| `happenstance-neon` | `0.3.2` | its ES-11 standing settled at phase 17 |
| `happenstance-sync` | no | built at phase 13; `publish = false` today (`crates/happenstance-sync/Cargo.toml:12`) |
| `happenstance-sync-testkit` | no | does not exist yet; built at phase 13 |

Sync and its testkit are in the set because of D-1. `examples/*`, `xtask` and
`benchmarks/` are outside it. Each of them sets `publish = false`, and none of
them is a surface anyone depends on.

**"Promised" and "publishable" are different facts, and they are reconciled at
phase 21, not before.** `xtask/src/package.rs:86`'s `PUBLISHABLE` says what *can*
ship. `grep -n '^publish' crates/*/Cargo.toml` returns two lines, ladybug and
sync. This list says what 1.0 *promises*. Today the two lists differ by exactly
the two sync crates, and phase 13 is what closes that gap. The names
`happenstance-sync` and `happenstance-sync-testkit` were **not claimed on
crates.io** when the registry API was queried on 2026-09-29. So phase 13's
claim item (`runbook/phases/13-sync.md:57-60`) is on 1.0's critical path, not a
formality. When the sync crates publish, they must also be added to the semver
job's hand-written package list (`.github/workflows/ci.yml:1081`), to
`SECURITY.md`'s scope and to `PUBLISHABLE`. Phase 21 checks all three.

**`happenstance-ladybug` is outside 1.0.** It can have its own `0.x` line once
versions are independent (§4). Three options were weighed:

1. **Inside 1.0, behind an upstream `lbug` fix.** Rejected. Phase 21 would wait
   on a third party, and the fix was deliberately never filed
   (`RUNBOOK.md:5010-5012`). The adapter's evidence is also weak. Its CI job has
   `if: false` and has never run (`.github/workflows/ci.yml:625`), 42/42 is a
   local result only, and the driver has a known segfault. On top of that,
   `lbug` is a `0.x` driver re-exported under ADR-0044, so every `lbug` break
   would be a ladybug major. That cost exists even if the crate never ships.
2. **Published, with docs.rs building it without `driver`.** Two variants.
   - **2a**, default features only. This would most likely fail on docs.rs
     today, for a reason inside this repository. The crate-root prose has
     intra-doc links to items that are `cfg`-ed out without `driver`
     (`[`GraphWriteSet`]` and `[`LadybugProjectionStoreError::MalformedCheckpoint`]`,
     `crates/happenstance-ladybug/src/lib.rs:139-150`), and the workspace sets
     `broken_intra_doc_links = "deny"`. Nothing in the gate builds this crate's
     docs without the driver. This is a prediction and was not run.
   - **2b**, docs.rs building `features = ["driver"]` with the two `env!`
     values `lbug` reads supplied through `cargo-args`. This is recorded as
     **a route, not a decision, and it is unmeasured**. `lbug`'s build script
     returns early under `DOCS_RS`, and the two failing `env!` calls are plain
     `&'static str` constants, so any string should satisfy them in a build that
     does not link. That is plausible, and it is the cheapest way to publish
     ladybug later on its own line. Nobody has tried it.
3. **Outside 1.0 (chosen).** 1.0 waits on nothing ladybug needs. The crate keeps
   `publish = false`, and its crate root keeps saying why.

### §2 The disposition rule

**Every clause on a promised surface that is not `[FROZEN]` has exactly one
disposition**, of one of three kinds:

- **`freeze-by-N`.** N is a phase in phase 21's prerequisite closure: 13, 14,
  17 or 18 (`runbook/README.md:108` gives 21's row as 13, 14, 16, 17, 18, 20).
  That phase makes the clause `[FROZEN]`, or it writes a record that
  re-dispositions it and says why.
- **`renew-past-1.0: <falsifier>`.** The clause ships in 1.0 as
  `[PROVISIONAL]` or `[DEFERRED]`, against a named falsifier that can still
  fire. It can only be renewed if firing the falsifier would be *additive*, or
  would relax an adapter's obligation rather than change a signature. A renewed
  clause whose falsifier would force a break has been given the wrong
  disposition.
- **`outside-1.0: <reason>`.** The clause's surface is not promised.

**The table lives in `runbook/ledgers.md`, in the section *"The 1.0
dispositions"*.** It does not live in this record, because a decision atom is
immutable (`cargo xtask lint-kb`). A table that phases 13, 14, 17 and 18 each
update as they freeze their rows would be edited long after the atom was
accepted. `cargo xtask lints`, through `runbook_clause_ledgers_match_the_specification`
(`xtask/src/lints.rs:3442`), already holds the provisional and deferred ledgers
against §7.2's generated table in both directions. It is to hold this section
the same way. Every non-`[FROZEN]` clause needs a row, every row needs one of
the three shapes above, and a `freeze-by-N` needs an N from the closure. A
clause that freezes leaves the table in the same change that moves its marker.

**The snapshot this record accepted is below.** It is here so the starting
point can be cited once the ledger has moved on. It has 53 rows: the 41
`[PROVISIONAL]` and 12 `[DEFERRED]` clauses that existed at the phase split. The
totals are 17 `freeze-by-13`, 4 `freeze-by-14`, 14 `freeze-by-17`, 7
`freeze-by-18`, 10 `renew-past-1.0`, 0 `outside-1.0` and 1 frozen here. A clause
with a stated fallback carries it in its row. PS-30's fallback is the only place
the `outside-1.0` category is used at all.

| Clause | Marker | Disposition | What does it |
|---|---|---|---|
| VT-6 | P | freeze-by-13 | sync-testkit's `restored_peer_does_not_reissue_identities` is the instrument for the harm half. Phase 9 answered the eviction half: Durable Object storage outlives the isolate, so mint-once is available. |
| VT-9 | P | freeze-by-13 | a sync-testkit rule that ingest preserves `RecordedAt`, with a mutant that overwrites it. The clock falsifier is restated below. |
| VT-10 | P | freeze-by-17 | the foreign-identity spike, with SQLite implementing `IngestStore` beside `append`. Phase 13 confirms it through SY-5 and SY-11. |
| VT-14 | P | freeze-by-17 | an RTL identifier corpus check (Arabic, Hebrew, Persian, with mixed LTR) comes back empty. E11's reproduction goes into `experiments/`. |
| VT-21 | P | freeze-by-13 | SY-18 is where a floor is first compared across a peer set. The tightest shipped target (Neon, 131,072) clears the floor twice over. |
| VT-22 | P | renew-past-1.0: a real domain event that needs more than 64 tags | a firing is answered by a store's own larger documented limit (every shipped adapter accepts 128 or more), **never by raising the floor within 1.x** |
| VT-23 | P | renew-past-1.0: a decision model that needs more than 128 items | a firing is answered by the store evaluating more, which every chunking adapter already does with no ceiling |
| VT-24 | P | freeze-by-13 | sync ingest is the first consumer to batch by the floor (E2E-35). SY-14 is where a group bigger than 128 would show up. |
| VT-30 | P | freeze-by-17 | ADR-0054's alias and builder-state questions decided in one pass. Limb 2 is retired by a record or by a multi-guard benchmark scenario. |
| WF-1 | D | renew-past-1.0: a DCB implementation publishes a wire-level encoding, or a user needs to read another implementation's log | phase 13 records the renewal in ADR-0026 |
| WF-11 | P | renew-past-1.0: a `workerd`-class isolate forwarding a payload that another store accepted, at or above about 36.6 MB under a 128 MiB cap (peak is payload × 11/3) | §6's `workerd` job is its instrument |
| ES-7 | P | freeze-by-17 | the record that answers `trait-variant-caret-resolves-past-the-locked-gate` and ES-17's ownership. Its falsifier is restated to cover a consumer's unlocked resolve. |
| ES-11 | P | freeze-by-17 | the ES-11 record that supersedes ADR-0061's keep, now that Neon is promised (§1) |
| ES-12 | P | freeze-by-17 | the same record. Falsifier: `query_items_share_one_snapshot` red on a one-shot-HTTP adapter. |
| ES-17 | P | freeze-by-17 | ADR-0055's restated two-build measurement, taken and acted on, or `&[Event]` frozen by a record |
| ES-32 | P | renew-past-1.0: the `experiments/polling-cost` harness re-run over a round-trip adapter with a stated staleness budget, showing 2N idle reads per interval breaking it at a realistic N | phase 18's fan-out runner is the natural producer |
| ES-35 | P | renew-past-1.0: a fixture arming a real fault against a real medium that loses a write | the cheapest candidate is a Postgres backend killed mid-commit |
| ES-39 | D | freeze-by-14 | shape decided at 17 (ADR-0028), then built and frozen at 14 |
| ES-40 | P | freeze-by-14 | decided at 17 with ADR-0028, then frozen with its rule against 14's suffix store |
| ES-41 | P | freeze-by-17 | with ADR-0028. The transport half is answered: Neon and Cloudflare each probe in one read-only round trip over the pair VT-8 indexes. |
| PS-6 | P | renew-past-1.0: an adapter that must reserve server state at `begin` and cannot afford the round trip | firing it relaxes an obligation and changes no signature |
| PS-9 | P | freeze-by-17 | the `Projection::apply` record, with the shape of the failure-policy seam. Phase 18 confirms it. |
| PS-11 | P | freeze-by-17 | with PS-9, as the marker's own text says |
| PS-15 | P | freeze-by-17 | `rollback` gets a port-level foreign-batch refusal (breaking), or the MUST narrows to `commit` and `reset` |
| PS-16 | P | freeze-by-18 | a typed-runner rebuild through `reset` against a multi-table or graph read model, with `RESET_REFUSAL`'s clause composed |
| PS-18 | D | freeze-by-18 | a refusable reset in at least one adapter, a CF-39-shaped clause and a `NoopProtectFixture` mutant. **Fallback:** renew-past-1.0, which is safe because the change is additive. |
| PS-22 | P | freeze-by-17 | ADR-0028 states that retention never rewinds a checkpoint over rows it keeps |
| PS-23 | P | freeze-by-17 | freeze *exactly one*, or record that a multi-id commit arrives as an additive defaulted method. Phase 18's fan-out runner is the empirical check. |
| PS-24 | P | freeze-by-17 | `Authority::Rebuilding` frozen as a kept variant. Phase 18 decides whether the runner emits it. |
| PS-25 | P | freeze-by-18 | built before `unstable-projection` lifts. Phase 17 picks the remedy first, because digest-in-checkpoint would change the frozen port. |
| PS-27 | D | freeze-by-18 | the failure-policy seam and `skip_and_record_is_atomic` with a mutant. Phase 17's apply record fixes the shape. |
| PS-30 | D | freeze-by-18 | `panicking_apply_rolls_back` with a mutant, if phase 18 builds fan-out. **Fallback:** `outside-1.0`, because it would be a conditional MUST on a runner 1.0 does not ship. |
| PS-38 | P | freeze-by-18 | settled with PS-23 against the fan-out runner. Phase 17 decides the documented no-lagging-replica obligation. |
| SY-7 | P | freeze-by-13 | the falsification test, with `MemorySyncPeer` and two adjudicator configurations, and the N-compensations divergence recorded (ADR-0027) |
| SY-10 | P | freeze-by-13 | phase 13's exit criterion requires both topologies to be expressible. **Fallback:** renew-past-1.0 against the marker's own test. |
| SY-14 | D | freeze-by-13 | measured against the Neon peer. Phase 17's foreign-identity spike must not foreclose it. |
| SY-18 | D | freeze-by-13 | phase 13 adds a fixture peer capped at 128 KiB, because neither real peer is KV-capped. **Fallback:** renew-past-1.0 against the Turnstile experiment. |
| SY-20 | P | freeze-by-13 | the rule lands at 13, and phase 18 carries the convergence declaration it needs |
| SY-21 | P | freeze-by-18 | decided in phase 17's apply record, which must name it. Built and frozen at 18. The sync rule lands at 13. |
| SY-22 | P | freeze-by-13 | the cost-layers test. The declaration's placement is fixed with SY-21. |
| SY-23 | P | freeze-by-13 | ADR-0027's merge rule. **Fallback:** renew-past-1.0, because a falsification adds an order on the `#[non_exhaustive]` `SequencedEvent` and does not change `EventId`'s `Ord`. |
| SY-27 | D | freeze-by-13 | ADR-0027. Sequencing hazard: its instrument is CF-27's suffix store, which phase 14 builds. Phase 13 builds it or a filtered peer, or SY-27 and SY-28 move to 14 in the same commit. |
| SY-28 | D | freeze-by-13 | straight after SY-27, on the same instrument |
| SY-29 | P | freeze-by-13 | jointly with SY-27 |
| SY-30 | P | freeze-by-13 | the two unlike real peers push real envelopes |
| SY-31 | P | freeze-by-13 | the runner half. The reserved `sync/` prefix goes to phase 17 with `projection-id-is-unvalidated`. |
| SY-32 | D | freeze-by-14 | decided at 17 (ADR-0028), then built and frozen at 14 |
| CF-14 | D | renew-past-1.0: phase 19a's REOPEN verdict for memory, IndexedDB and OPFS storage, or a `workerd` run that observes a real Durable Object eviction | freezes jointly with CF-17 |
| CF-17 | P | renew-past-1.0: the same two events | HS-P0013 and HS-P0014 have answered (below) |
| CF-39 | P | **frozen at 16, by this record** | §3 |
| CF-40 | P | freeze-by-13 | phase 17 builds ADR-0043's `MetadataLen`. Phase 13 decides whether `payload_len` (data plus metadata) is the budget unit. |
| CF-27 | D | freeze-by-14 | report shape decided at 17 (ADR-0028), then built and frozen at 14. The instrument may have to be built at 13 (SY-27). |
| CF-34 | P | renew-past-1.0: `kb-open-question-cf-33-cf-34-scope-001` is answered, which means an instrumented rows-examined fixture plus a record on the scope of CF-33's operation-count ban | the marker's own falsifier is restated (below) |

CF-14 and CF-17 are renewed rather than scheduled for a specific reason. The
event that would freeze them is phase 19a's REOPEN verdict, and phase 19a is
**not** in phase 21's closure. A `freeze-by-19` would name a phase that 1.0 does
not wait for.

#### Thirteen of fourteen proposed freezes were refuted

The survey proposed freezing fourteen clauses now: VT-9, VT-14, VT-30, ES-7,
ES-12, PS-15, PS-16, PS-22, PS-38, CF-14, CF-17, CF-34, CF-39 and CF-40.
Adversarial verification refuted **thirteen**, and only CF-39 survived (§3). The
refutations are recorded here because they all failed in the same way, and that
pattern is the finding. **Each proposed freeze was an argument that the marker's
falsifier was decorative, and none was evidence that the falsifier had been run
and had not fired.** A falsifier that cannot discriminate is a reason for a
record to *restate* it. It is not a reason to freeze the clause it guards.
Summarised:

- **VT-9.** The instrument the marker names, a Durable Object's frozen clock,
  was never used. The Cloudflare suite runs on a Node shim with no I/O gate
  (`crates/happenstance-cloudflare/src/host.rs:41-45`). The ingest-preservation
  MUST has no rule, because `happenstance-sync-testkit` does not exist.
- **VT-14.** E11 tested how the check is built, not whether any legitimate
  value needs a bidi control. ADR-0015 recorded E11 and kept the marker anyway,
  and E11 is not in `experiments/`. The promise lens alone did not refute it.
- **VT-30.** Limb 1 is discharged (Neon puts every guard into one statement).
  Limb 2 is unmeasured, and ADR-0054 has already marked `after`/`after_opt` for
  deprecation, so a freeze would fix names the project decided to change.
- **ES-7.** The evidence lens let it through. The promise lens did not: the
  blanket impl is `trait_variant`'s code, pinned with a caret, and phase 17
  answers that dependency question.
- **ES-12.** It has no mechanism of its own. It is discharged by ES-11's
  ceiling, and ES-11's falsifier has fired on Neon.
- **PS-15.** `rollback` returns `Result<(), Self::Error>`
  (`crates/happenstance-core/src/projection.rs:536`), so it cannot return the
  port-level `ForeignBatch` the MUST names, and no rule checks the rollback leg.
  ADR-0063 had already declined to freeze it (`references/adr/0063-the-projection-port-is-frozen.md:142-144`).
- **PS-16.** The rebuild it waits for, a typed-runner rebuild against a read
  model that is not a single table, has not happened. The promise lens alone did
  not refute it.
- **PS-22.** It is owned by ADR-0028, which is not written and has moved to
  phase 17. A redaction design could plausibly need to rewind a checkpoint over
  rows it keeps.
- **PS-38.** The Neon run uses one primary endpoint, so the lagging-replica
  shape was never exercised. The clause's text also embeds the one-id `commit`
  that PS-23 may change.
- **CF-14 and CF-17.** Cloudflare's reopen swaps a binding over the same
  in-process `:memory:` database (`host.rs:108-109`), and Neon's `reopen` is an
  empty body. That is the `NoopReopenFixture` shape, and CF-17's own text says
  no rule can tell it apart from a real reopen.
- **CF-34.** The rows-examined falsifier was never tried, and
  `kb-open-question-cf-33-cf-34-scope-001` still challenges the clause's text.
- **CF-40.** The Postgres half, a ceiling that moves with the rest of the row,
  was never measured. The survey reworded the falsifier instead.

**This record adopts three restated falsifiers.** Each is written into its own
clause's marker in `spec/SPECIFICATION.md` in the same change as this record.
None of the three changes maturity: all three stay `[PROVISIONAL]`.

- **VT-9.** The clock falsifier no longer discriminates. It has already fired,
  on `wasm32-unknown-unknown`, where `MemoryEventStore` stamps
  `RecordedAt::from_millis(0)`, and it fired with no effect, because the rules
  assert that a recorded time is stable, never that it is recent
  (`crates/happenstance-core/src/memory.rs:264-274`). VT-9's live falsifier is
  now at the ingest seam: *`IngestStore` cannot carry a foreign `RecordedAt`
  unchanged through the phase-13 envelope.* That is what `freeze-by-13` means
  for it.
- **CF-34.** Its falsifier, an instrumented fixture that counts rows examined,
  is **foreclosed by CF-33**. CF-33 is `[FROZEN]` and forbids any conformance
  rule to assert on an operation count (`spec/SPECIFICATION.md:9079-9080`), so
  the rule the falsifier imagines could not be admitted. CF-34's falsifier
  becomes the open question that owns both clauses'
  scope. The marker stays until that question is answered, because every
  plausible answer to it touches CF-34's text.
- **CF-17.** The two adapters named in the marker that waited on them (CF-17's
  own marker, as it stood before this change restated it) have answered:
  HS-P0013 (Cloudflare) and HS-P0014 (Neon), with Postgres as a further data
  point. What stays open is not the adapters but the medium. Cloudflare's
  answer was over an in-process shim, and phase 19a holds the browser-storage
  verdict.

#### Rows the table corrects

`runbook/ledgers.md`'s CF-39 row (`:164`) said that *"no adapter has armed a
fault yet"*, and the sentence at `:199` routes CF-39 to phases 8 and 10. That was
stale before this record. The row is struck in the same change that moves the
marker, and the routing sentence is corrected with it. ES-11's row depends on
§1: Neon is promised, so ADR-0061's reason for keeping ES-11 open no longer
covers it, and phase 17's record must decide whether Neon's conformance claim
carries ES-11 as a named, documented exception.

### §3 CF-39 is frozen

CF-39 requires that a fixture declaring `MID_BATCH_FAULT` fail the *k*-th write
**inside the store's own write path**, by a mechanism the store cannot absorb,
and say what that mechanism is. A fixture whose store can absorb every fault it
can arm must decline (`spec/SPECIFICATION.md:8422-8427`). Its marker was
falsified by *"a real adapter whose only injectable mid-batch fault is one its
driver transparently absorbs"*, named rusqlite and Postgres as the instruments,
and said *"no adapter has armed a fault yet"* (`:8428-8433`).

**Both named instruments have run, and neither driver absorbed the fault. A
third and a fourth adapter arm real in-store faults as well.**

- **rusqlite, phase 8.** The fixture declines the capability *by scope, not by
  incapacity* (`crates/happenstance-sqlite/tests/support/mod.rs:173-184`), so
  that ES-35's residual falsifier is not retired from a story that has no say in
  it. The instrument still ran at crate level. A real `AFTER INSERT … RAISE(ABORT)`
  trigger, installed through a second connection, makes a 16-event append return
  `Err(AppendError::Store(_))` and leaves zero rows
  (`crates/happenstance-sqlite/tests/append.rs:848-886`).
- **Postgres, phase 10.** The fixture declares the capability supported and arms
  an `AFTER INSERT` trigger that raises on the `after + 1`-th row
  (`crates/happenstance-postgres/tests/support/mod.rs:459-473`, the raise at
  `:525`). The `sqlx` pool did not absorb it. The conformance test asserts that
  the capability is supported **and** is not the trait's silent default
  (`crates/happenstance-postgres/tests/postgres_conformance.rs:415-429`). The rule
  runs in CI's `live-postgres` job (`.github/workflows/ci.yml:412`), which makes
  it a claim CI makes rather than one machine's.
- **Cloudflare, phase 9.** A `BEFORE INSERT … RAISE(ABORT, …)` trigger behind a
  countdown. The standing control `the_armed_fault_is_a_real_trigger_inside_the_store`
  reads the trigger's own text back out of the error the caller sees, and that
  string exists only in the SQL trigger body
  (`crates/happenstance-cloudflare/tests/support/mod.rs:193-244`).
- **Neon, phase 10b.** The same trigger, with a transaction-local counter,
  because the `/sql` proxy pools backends across unrelated requests
  (`crates/happenstance-neon/tests/support/mod.rs:154-173`, the raise at `:226`).
  Neon's retry loop matches SQLSTATE `40001` and nothing else
  (`crates/happenstance-neon/src/error.rs:40-48`). The fixture's `RAISE EXCEPTION`
  is `P0001`, so the loop cannot absorb it.

The rule is `arming_a_mid_batch_fault_makes_the_append_fail`
(`crates/happenstance-testkit/src/suite.rs:3109`, registered at
`registry.rs:167`). It can fail: `NoopFaultFixture`
(`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:3625`)
declares the capability, overrides the arming with an empty body, and is
rejected. The clause needs no `†`.

**Why this is a freeze and not a renewal.** The falsifier cannot force a change
to the text. An adapter whose driver absorbs every fault it can arm already has
an instruction in the clause's last sentence: decline, and state that as the
reason. So the worst the falsifier can produce is one more stated declension.
The promise lens checked every pending phase. Phases 13 and 17 might add a write
path, and faulting it would take a *new* capability beside CF-39, not an edit
to CF-39. ADR-0028's possible new method does not touch the fixture's promise.
Phase 19 relies on CF-39 as written.

**What the freeze does not claim, stated so a reader does not assume it:**

- **Every armed fault in the workspace is a SQL trigger.** No store without
  triggers, and no store with an absorbing driver, exists here. The far end of
  the falsifier's axis is unoccupied, which is the gap CLAUDE.md's rule about
  spread warns about. The freeze rests on the decline branch covering that far
  end, and it does cover it, but that is an argument about the text. It is not
  an instrument standing at the far end.
- **Cloudflare's run is on the crate's own JavaScript host, not on `workerd`.**
  The trigger is a trigger under either host, and that is the reason the
  fixture gives for the fault being meaningful. §6 moves the run anyway.
- **"Verified against the live endpoint" for Neon is a doc comment**, and the
  `live-neon` job (`ci.yml:805`) runs only when `NEON_CONNECTION` is present.

**Consequence for a published surface.** Freezing CF-39 fixes what
`Fixture::arm_mid_batch_fault` means in `happenstance-testkit`'s published trait.
Phase 17's breaking window must not change that method's signature, including
the *k* index, unless it writes a record that supersedes this section.

**The marker text.** It becomes `[FROZEN]` *(by ADR-0066. …)*, the form
ADR-0063's freezes use (`spec/SPECIFICATION.md:5206`). Its body names the
falsifier, the three arming adapters, and SQLite's declension by scope. Why a
future absorbing driver can only produce a declension is argued above and is
not repeated in the marker; the clause's own last sentence is what carries it.
§7.2's generated row reads `FROZEN`, and §1.3's census reads 142 / 40 / 12 / 7.

### §4 Versioning

- **`1.0.0` is released in lockstep**, all nine crates at once, for a clean
  starting line. **After it, every crate versions independently.** Each crate
  has its own `version` key, and the shared `workspace.package.version` (0.3.2
  today) stops being the thing that moves them.
- **Every adapter declares the minimum core it needs**, as
  `happenstance-core = "1.N"`. Caret resolution then unifies every adapter on one
  `1.x` core, and that closes the open question's *"two `Event` types that print
  identically"* hole within a major. The hole was a `0.x` artefact, because Cargo
  treats `0.2` and `0.3` as incompatible. A declared lower bound is a guess until
  something resolves against it, so phase 17 adds a **minimal-versions CI job**,
  a sibling of `gate` because it needs a nightly resolver, to keep every bound
  honest. **The release tooling** (`release-plz`, `cargo-release` or another) is
  chosen at phase 21.
- **"Conformant" means *"passes `happenstance-testkit` X.Y"*.** Each adapter's
  README says which X.Y it passed. Without that sentence, the testkit's
  independent number means nothing to a consumer.
- **The testkit's rules keep CF-29 and CF-31's semver.** A new rule that
  *detects a violation of a clause that was already `[FROZEN]`* is a testkit
  **minor**. It finds a defect rather than making a break, because the adapter
  it turns red was never conformant, and a rule saying so is the testkit doing
  its job. A new *requirement*, meaning an obligation no frozen clause stated,
  first needs a clause change, which needs a record and makes a **major**. The
  same policy applies to `happenstance-sync-testkit`. `cargo-semver-checks`
  diffs API, not behaviour, so it cannot see a new rule. CF-29's changelog lint
  stays the only mechanical signal, and CF-30's advice to pin the testkit
  exactly stays good advice, because a dev-dependency pin does not propagate.
- **CF-32 is unchanged.** The testkit keeps its own `version` key. Independent
  versions satisfy CF-32 without amending it, and ADR-0057's dropped root key
  (still at `Cargo.toml:62`) is executed at phase 17.
- **The public-dependency limit (ADR-0044) is a named limit on the promise.** An
  adapter re-exports its driver, so **an adapter's major follows its
  re-exported driver's breaking version**: rusqlite 0.40, sqlx 0.8 and worker
  0.8.5 today. That limit is only affordable because versions are independent.
  Under one shared number, a driver break would drag `happenstance-core` to 2.0
  with an unchanged contract, which is the failure CF-32 names, applied to every
  crate. `happenstance-core`'s own `0.x` public dependency, **`futures-core
  0.3`** (`pub use futures_core;`, `crates/happenstance-core/src/lib.rs:176`),
  is a **named accepted risk**. It has been stable at 0.3 for years, but a 0.4
  would force a core major. `bytes` is at 1 and carries no such risk.

This **closes `kb-open-question-adapter-version-lockstep-001`**. Its
sub-question 1 is answered by the README sentence together with the declared
lower bound. Sub-question 2 is answered by neither amending CF-32 nor sitting
beside it: CF-32 governs the testkit's key, and this section governs what an
adapter's requirement states. Sub-question 3 is answered by the minimum-core
declaration, kept honest by the minimal-versions job.

### §5 What semver does not cover at 1.0

The exemption list is enumerated. It is mirrored into `CHANGELOG.md`'s account
(`:13-33`) and `SECURITY.md` at phase 21.

- **`happenstance`'s `unstable-projection` runner.** Phase 18 lifts the gate. If
  it does not, the runner is declared exempt at 1.0, by name.
- **`happenstance-core`'s empty `unstable-projection = []`**
  (`crates/happenstance-core/Cargo.toml:72`). It is removed in `0.4.0` at phase
  17, so it is **not** on the 1.0 list: it will not exist there.
- **`happenstance-postgres`'s `naive-arm`** (`crates/happenstance-postgres/Cargo.toml:125`)
  is a public, docs.rs-rendered feature that exposes a deliberately unsound
  constructor. Phase 17 removes it, or declares it outside semver in the crate
  root. If it is declared, it joins this list.
- **`conformance` and `ProjectionProbe` are INSIDE the promise**, as ADR-0063
  made them. An adapter author implements `ProjectionProbe`, so a changed
  signature there is a changed obligation.
- `#[doc(hidden)]` items, `Display` and `Debug` text, and rustdoc prose are not
  API. The MSRV is governed by ADR-0067 (§7), not by this list.

**ES-6, sub-question 4.** Driver error payloads re-exported under ADR-0044, such
as `rusqlite::Error` inside `SqliteEventStoreError`, **are part of the promise**,
under the driver-major limit in §4. `#[non_exhaustive]` protects adding a
variant, not changing a variant's payload, so the answer has a semver
consequence. Here it is: a payload change forced by a driver major is an adapter
major, and it is not an exemption. Writing that into ES-6's prose is phase 17's
job. The rest of `kb-open-question-es-6-unwritable-rule-001`, the unwritten
`store_error_crosses_a_join_handle`, stays open. This record does not close it.

§5's exemption list and the ES-6 sub-question 4 answer are this record's own
call, not a recorded owner decision, and may be revised by a superseding record
before phase 21.

### §6 Cloudflare: a `workerd` sibling job before 1.0

**`happenstance-cloudflare`'s 1.0 claim is conformance on the real runtime.**
Today every rule runs on `wasm32-unknown-unknown` against a `node:sqlite` shim
shipped in `crates/happenstance-cloudflare/src/host.rs`. Phase 17 adds a
**`workerd` sibling job** to `.github/workflows/ci.yml`, shaped like
`live-postgres` and `live-neon`: **a sibling of `gate`, never a step inside
it.** That keeps ADR-0023's reason for keeping `workerd` out of `cargo xtask ci`
intact. `workerd` is an external binary with no native Windows story, versioned
by a Node lockfile this repository does not own. ADR-0023's own table said a
runner inside the gate was *not rejected on merit*. This record takes the
sibling shape that sub-question 1 of the open question named, and accepts it as
a claim CI makes.

**What the shim does not prove**, as ADR-0023 lists it
(`references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md:162-169`):
no isolate, no eviction, no hibernation, no I/O gate, no event loop re-entering
the object mid-`await`, and none of the platform's own storage ceilings. Two
measurements ride on the job, and each is taken **before** it is promised:

- **the SQL-text wall**, meaning the statement length at which a Durable Object
  refuses; and
- **ADR-0052's partition widths**, the public `MAX_QUERY_ARMS_PER_STATEMENT` and
  `MAX_QUERY_PARAMETERS_PER_STATEMENT`. `happenstance-cloudflare` adopted these
  unchanged from its SQLite sibling because nothing could locate a wall for
  them. Lowering either after 1.0 would be a semantic break that no signature
  announces, so they are measured in the breaking window or not promised.

The job is also the instrument that WF-11's, CF-14's and CF-17's renewed
falsifiers name.

`runbook/README.md:90` says phase 9 proved *"every rule green under `workerd`"*.
That is false. Phase 9 proved every rule green under a shim, and the row is
corrected.

This **closes `kb-open-question-workerd-runner-absent-001`**. Sub-question 1:
the runner is a sibling job, not a gate step. Sub-question 2: the exclusion list
does not need to become a living inventory, because the runner that would
retire it is now owned. Sub-question 3 belongs to phase 13, which reads WF-11's
renewed row. Sub-question 4 is answered by the measurement above.

### §7 MSRV

Owned by **ADR-0067**, in the 0004 → 0029 → 0037 lineage. The floor holds at
1.97.1. The 2026-09-06 `msrv-premise` ratification is withdrawn, since it was
never executed. After 1.0, the floor rises only in a minor, only to a stable
release at least six months old when that minor ships, and always with a
CHANGELOG entry. ADR-0037's argument was that *"a minor is incompatible below
1.0"*, and that stops being true at `1.0.0`, where `cargo update` takes a 1.x
minor. So the promise now rests on resolver 3's MSRV-aware fallback. Phase 21
checks each crate's documented MSRV statement against ADR-0067.

### §8 Release

- **The `1.0.0-rc.1` soak.** **No time floor.** `1.0.0` follows the release
  candidate as soon as all of these hold on it:
  - all nine crates render on docs.rs *from the registry*;
  - `examples/outside-projection-adapter` and one other example build against
    the registry rc, not by path;
  - the registry semver baseline is clean;
  - phase 21's outside-reader pass has been run against the rc;
  - no defect found in the rc needs an API change.

  Any API change means `rc.N+1`, and the conditions are checked again. A soak
  measured in time would be decorative here, by this project's own rule. On
  2026-09-29, `happenstance-core`'s reverse dependencies were the six crates in
  this workspace, and its total downloads were 213. A 14-day floor was
  recommended and **declined by the owner** for that reason. What the decline
  costs is named: sync will have had the least time on the registry of any
  promised crate, and no window protects a late outside reader.
- **Supported versions after 1.0.** The latest 1.x minor, plus **security fixes
  on the previous major for six months after the next major ships.** That
  matters because adapter majors will follow driver majors (§4). Phase 21
  rewrites `SECURITY.md`'s table and its *"Pre-1.0"* section. The owner's
  decision, 2026-09-29.
- **Licence.** **The nine crates named in §1 stay `MIT OR Apache-2.0` for all of
  1.x.** A crate added later may be licensed otherwise. `0.2.0` through `0.3.2`
  were already granted under that licence, and the grant cannot be revoked; this
  sentence extends it forward as a promise. The owner's decision, 2026-09-29,
  taken over the alternative of saying nothing, which would have kept a
  relicensing path open for the core.

### §9 Not 1.0 questions

- **`references/seeds/measured-not-claimed.md`.** Not a 1.0 question, and
  largely discharged: `benchmarks/` exists, and the README deliberately makes no
  performance claim. The seed touches 1.0 only through clauses whose falsifier is
  a benchmark (ES-17, ES-32, PS-30), and those are in §2's table.
- **`references/seeds/licensing-and-the-commercial-seam.md`.** Not a 1.0
  question beyond §8's licence promise. The commercial seam the seed weighs lies
  outside the nine crates, so it stays a seed.
- **`kb-open-question-trademark-search-001`.** Not a 1.0 question. It stays open
  on its own triggers (filing, registration, physical goods), and 1.0 is none of
  them. Defining *conformant* as *"passes `happenstance-testkit` X.Y"* (§4)
  leaves room at no cost for a later certification policy to build on.
- **`.kb` link resolution.** Recorded as **not owed before 1.0**. `.kb/` ships
  in no `.crate`, and no 1.0 promise reads it. It was measured on 2026-09-29:
  1,023 frontmatter references across `related`, `depends_on`, `supersedes` and
  `superseded_by`, of which 10 do not resolve, and all 10 are placeholders in
  `.kb/_templates/atom.md`. A resolving check would land green, and nothing stops
  anyone adding one. That is a choice anyone may make, not an obligation.
- **`cloudflare-worker-feature-gate`: no features table.** `worker`'s types sit
  on every public construction and error path (`SqlStorage::new`,
  `SqlStorage::from_state`, `JsThrow::from_error`, `JsThrow::error`), so a
  feature that excluded `worker` would exclude the adapter. The crate has one
  backend and one platform. A later feature that gates only *new* items stays
  additive after 1.0. This **closes `kb-open-question-cloudflare-feature-gate-001`**
  and takes the item off phase 17's list.
- **`conflicting_position` is a hint, and that is settled.** ES-25 is `[FROZEN]`
  and already says so: an adapter that detects the conflict without learning the
  culprit MUST be permitted to report `None`, and callers MUST NOT depend on it
  (`spec/SPECIFICATION.md:3947-3950`). ADR-0012 §8 is the record
  (`references/adr/0012-append-shape-and-preconditions.md:462`). Neon-over-HTTP
  was the forcing case, and it exists and conforms. The ledger's open-decision
  row is struck.

### §10 Phase 17's work list: the breaking half

This list is written into `runbook/phases/17-breaking-window.md`. Each item is
answered in its own record or closed with a reason.

**Open questions whose answer is breaking, or must be decided inside the
window:** `should-codec-be-sealed`; `projection-batch-sql-seam-statement-type`;
`projection-id-is-unvalidated`, **together with SY-31's reserved `sync/`
prefix**; `then-empty-emission-idiom-and-the-nothing-to-do-channel`;
`tuple-boundary-heterogeneous-event-type` (option A forecloses B and C);
`es-17-two-adapter-measurement-is-unscheduled`;
`no-fixture-tolerance-for-transient-contention` (a `Busy` variant on the
`#[non_exhaustive]` `AppendError` is additive, but whether 1.0 promises one is
decided in the window); `cf-23-emitter-names-mandatory-and-marked-unstable`,
where this record decides the policy — the `__emit_*` names CF-23 obliges an
adapter to write are **inside** the 1.0 promise, not marked unstable, because a
`[FROZEN]` clause that requires a name cannot also leave it unpromised — and the
renames that policy implies land in 17; and ES-6's sub-question 4, decided in §5,
leaving only the prose to write.

**Reclassified additive, and not on 17's list:**
`read-page-budget-is-unspecified` goes after 1.0.
`trait-variant-caret-resolves-past-the-locked-gate` is answered by a pin, an
assertion or a CI job, but ES-7's freeze rides on the record that answers it,
and that record is written in 17. `cloudflare-worker-feature-gate` is closed
here (§9).

**Items this charter sends to 17:** the `workerd` sibling job and the SQL-text
wall and partition-width measurements (§6); the minimal-versions CI job (§4);
executing ADR-0057; removing `happenstance-core`'s empty `unstable-projection`
(§5); removing `naive-arm` or declaring it exempt (§5); and the ES-11 record,
which supersedes ADR-0061's keep now that Neon is in the set and settles ES-11
and ES-12 together.

**Clause follow-ups:** every `freeze-by-17` row in §2. These are VT-10, VT-14,
VT-30, ES-7, ES-11, ES-12, ES-17, ES-41, PS-9, PS-11, PS-15, PS-22, PS-23 and
PS-24. The decisions 17 takes for clauses another phase freezes are ES-39,
ES-40, SY-32 and CF-27 (ADR-0028), SY-21 (the apply record), PS-25's remedy,
CF-40's `MetadataLen` build (ADR-0043) and PS-38's documented no-lagging-replica
obligation.

## Options considered and rejected

- **Count the crate set rather than naming it.** Rejected. CLAUDE.md records a
  count that drifted twice. A named list is checked against manifests at phase
  21, and a number cannot be.
- **Sync outside 1.0.** Recommended by D-1's own analysis and overridden by the
  owner. It is recorded here, not re-argued.
- **Put the disposition table inside this record.** Rejected. An accepted atom
  is immutable, and the table is meant to shrink phase by phase. The long form
  keeps a snapshot, and the ledger keeps the live table.
- **Freeze every clause whose falsifier looked decorative.** Rejected on
  adversarial evidence, as §2 describes. Thirteen of fourteen such freezes
  rested on argument alone.
- **Renew CF-39 past 1.0 instead of freezing it.** This was the verifier's
  fallback. It lost because the falsifier cannot force a text change, and
  because the testkit's `Fixture` surface is cheapest to fix now.
- **Versioning: full lockstep forever.** Contradicts CF-32 `[FROZEN]`, and
  under ADR-0044 it would turn every driver break into a core major. **Shared
  version for all but the testkit (the status quo).** The same driver objection.
  **Contract family at 1.0, adapters held at `0.x`.** Promises less than the
  crate set names.
- **Testkit: every new rule is a major.** Majors would become frequent and stop
  carrying signal. **Versioned bars in the macro** (`bar = "1.0"`). The only
  option that stops `cargo update` turning adapters red, but it is a signature
  change in 17 that nobody asked for, and exact pinning already costs nothing.
- **A `workerd` runner inside `cargo xtask ci`.** It breaks the gate's
  single-command property on Windows, and the sibling job buys the same evidence.
- **Scope Cloudflare's 1.0 claim to "conformant under the shim".** Rejected by
  the owner. A claim that has to explain which runtime it does *not* hold on is
  not a 1.0 claim.
- **Ladybug options 1, 2a and 2b.** §1.

## Consequences

- Phase 13 is on the critical path twice, for the sync crates themselves and for
  claiming their names. Phases 13, 14, 17 and 18 each owe `freeze-by-N` rows,
  and phase 21 cannot pass with any such row still open.
- The `workerd` job and the minimal-versions job are new CI surface. Both are
  siblings of `gate`, so `cargo xtask ci` stays one command, and each job is a
  claim CI makes rather than one the gate makes.
- After `1.0.0`, a driver's break is an adapter's major. Consumers will see
  adapter majors more often than core majors. That is intended, and the README's
  "passes testkit X.Y" line is what tells them what an adapter version means.
- `SECURITY.md`, `CHANGELOG.md`'s versioning paragraph, `PUBLISHABLE` and the
  semver job's package list all change at or before phase 21.

## Falsifiers

- **§1.** Building sync reveals a seam that `EventStore` must grow and that
  phase 17's spike missed. That is D-1's own falsifier, and after 1.0 it is a
  2.0.
- **§2.** A clause reaches phase 21 without a `freeze-by-N` having frozen it and
  without a re-dispositioning record. Or a `renew-past-1.0` falsifier fires and
  turns out to need a signature change, which would mean the clause was given
  the wrong disposition.
- **§3.** A fixture that declares `MID_BATCH_FAULT` supported turns out to have a
  fault its driver absorbs, so the rule passes on a store that was never
  faulted. That would be a mutant this record says cannot exist.
- **§4.** A consumer still resolves two `happenstance-core` majors under one
  adapter's declared bound. Or the minimal-versions job cannot be kept green
  without raising bounds nobody chose.
- **§6.** The `workerd` job cannot be kept running at a cost the project will
  pay. In that case Cloudflare's 1.0 claim reverts to the shim's scope, and this
  record is superseded to say so.
- **§8.** An outside adopter finds a defect in `1.0.0` that a time floor on the
  rc would have exposed. That reopens the soak for the next major's rc, and
  only for it.

## Evidence

- Survey and verification outputs, 2026-09-29. The phase-16 session keeps them
  outside the tree, and every citation from them above was re-read at `HEAD`.
- `spec/SPECIFICATION.md:8422-8443` (CF-39, its marker, rule and rejects);
  `:9079-9080` (CF-33); VT-9's, CF-17's and CF-34's markers, cited by clause
  because this change rewrites them; `:3947-3950` (ES-25 on `conflicting_position`); `:8984`,
  `:9029` and `:9043` (CF-29, CF-31 and CF-32).
- CF-39's instruments, as cited in §3.
- `crates/happenstance-core/src/memory.rs:264-274`,
  `crates/happenstance-core/src/lib.rs:169,176`, `crates/happenstance-core/Cargo.toml:72`,
  `crates/happenstance-postgres/Cargo.toml:125`, `crates/happenstance-ladybug/src/lib.rs:139-150`,
  `Cargo.toml:60-62`, `xtask/src/package.rs:86`, `xtask/src/lints.rs:3442`.
- `.github/workflows/ci.yml:412` (`live-postgres`), `:625` (`ladybug`), `:805`
  (`live-neon`) and `:1081` (the semver package list).
- `runbook/README.md:90` and `:108`; `runbook/ledgers.md:164` and `:199`;
  `runbook/phases/13-sync.md:57-60`.
- `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md:162-169`;
  `references/adr/0063-the-projection-port-is-frozen.md:142-144`;
  `.kb/decisions/wi-40b321-does-v1-0-include-happenstance-sync-or-does-1-0.md`.
