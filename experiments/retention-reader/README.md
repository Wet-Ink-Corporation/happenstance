# `experiments/retention-reader`

Does a reader of a store that has forgotten part of its own log need the port to
**say** so? ES-39 defers exactly this question to phase 14 (#156). It asks for two
readers, a projection runner and an ingest path, to be run against CF-27's
completeness instrument, and for a record of whether either one needs a
port-level report. ADR-0028 decision 3 adds a constraint: any such report must
default to `Unknown`, so a reader can never rely on it.

The verdict picks one of two outcomes, both pre-authorised by ADR-0028
decision 9:

- **Refusal:** ES-39 freezes as "the port does not report completeness, and a
  caller MUST NOT infer it from any port value".
- **Method:** a provided `EventStore` method defaulting to `Unknown`, plus the rule
  `a_store_reports_the_history_it_does_not_hold`.

This section was committed **before any test existed**, so the criteria and the
predictions are visibly pre-registered in history. The provided-method spike's
"stated before anything ran" is the precedent.

## Decision criteria (fixed before any run)

The experiment examines four readers, R1 to R3 below, with the ingest path split
into two:

- **R1**, the typed projection runner;
- **R2-src**, the ingest path pulling *from* a holed store;
- **R2-dst**, the ingest path writing *into* a holed store;
- **R3**, a decision model, recorded only and not part of the verdict.

A reader **needs a port-level report** if and only if it meets all four of C1 to
C4 in at least one scenario.

- **C1, harm.** Over the holed log, the reader does something its own contract
  calls wrong. A smaller output is not harm. ES-38 says `Query::all()` means "what
  this store holds", so a view built from fewer events is correct relative to its
  store.
  - Harm means a stated guarantee breaks: a clause (PS-, SY-, VT- or ES-), or the
    reader's own rustdoc promise. Examples: an event applied twice; an edge that
    stops with a false cause; a checkpoint that skips events the store holds.
  - The README names the clause or the doc line that each harm breaks.
- **C2, undetectable today.** No value the reader can already reach lets it detect
  or avoid the harm. That covers every `EventStore` method, every `IngestStore` /
  `WriteGroupStore` / `SyncPeer` value, and its own state. The instrument's
  inherent `withheld()` does not count, because it is not port.
- **C3, a report defaulting to `Unknown` would cure it, generically.** Both parts
  must hold.
  - **(a)** The reader keeps today's behaviour on `Unknown`. ADR-0028 forces this:
    all four published adapters, and `MemoryEventStore`, forget outside the port.
    They would answer `Unknown` for ever, so a reader that refuses on `Unknown`
    refuses everywhere.
  - **(b)** On a truthful answer, the reader does something materially better,
    which its caller **could not get from the store's concrete type**. The report
    must cross a boundary where the type is erased, such as code generic over
    `S: EventStore` in a library. A store that could answer truthfully has pruned
    through its own API, and its caller can already ask that API. If that holds
    for every store that could answer truthfully, (b) fails.
- **C4, `EventStore` is the seam.** The missing information belongs on
  `EventStore`. It does not belong on `IngestStore` or `WriteGroupStore` (the
  adapter's own ingest internals), on `SyncPeer` or `PeerLimits` (a remote peer,
  which is SY-32's seam), on the typed layer's checkpoint, or on the pruner's
  inherent API. A cure that belongs elsewhere is a finding for that seam's owner,
  not ES-39's method.

**Verdict rule:**

- **Method:** some reader meets C1, C2, C3 and C4.
- **Refusal:** no reader meets all four.
- **Escalate:** some reader meets C1, C2 and C4, and fails C3 *only* because
  `Unknown`-on-default is unacceptable. That is ADR-0028's falsifier: the report
  would have to be required. Stop and hand to the owner. Do not write a method.

**Findings are recorded whatever the verdict.** Any C1 harm that fails C3 or C4
goes into this README with the seam that owns its cure, and becomes a candidate
issue.

## Predictions (written before running)

| Reader | Scenario | Predicted C1 | Predicted verdict line |
|---|---|---|---|
| R1 | rebuild over holed vs full | no harm: smaller view, correct relative to the store (ES-38) | fails C1 |
| R1 | prune behind the checkpoint, then continue | no harm: view equals the full log's | fails C1 |
| R1 | prune ahead of the checkpoint, then continue | no harm under ES-38: pruned events never delivered | fails C1 |
| R1 | a floor reader over a scattered purge | the floor misdescribes holes above it (why ADR-0028 rejected the floor) | evidence for ES-39's *Rejects* line |
| R1 | source switch, store A then B, one checkpoint | **harm**: silent skip/replay (`runner.rs` warns; nothing enforces) | fails C2 (`store_id()` exists) and C4 (the typed checkpoint's seam) |
| R2-src | spoke pulls from a holed hub | no harm under ES-38/SY-27: spoke holds the hub's retained set; `SyncStatus` says synced | fails C1. If counted as harm, fails C4 (`PeerLimits`/SY-32) |
| R2-dst | spoke prunes foreign events, restored edge re-pulls from the start | **harm**: pruned events re-admitted at the tail, a projection applies them **twice** | fails C3(b) and C4 (dedupe is inside `IngestStore::ingest`, keyed on origin identity, not local position) |
| R2-dst | spoke prunes its **own** confirmed events, restored edge re-pulls | **harm**: `IngestError::UnheldOwnIdentity` → `FailureClass::Stop`, a false "restored/rewound" (SY-38). Whole batch rolled back, so the edge is wedged | fails C4 (the check lives in the adapter's ingest; a cure is a tombstone there) |
| R3 | decision over a holed log | admits what the full log refuses (ES-40, vacuous) | recorded, not in the verdict. Refusing on `Unknown` would refuse on every adapter (C3a) |

**Predicted verdict: refusal**, with two harms found that ES-39's method cannot
cure. Both are owner-level findings for #158 and the replication seam. R2-dst is
new information: the 0.7.0 prior did not mention it.

- `runner/watermark.rs` says *"a re-read is harmless, because ingest skips what it
  holds (SY-11)"*. That holds only while the store never forgets.
- `Edge::restored` ("pulls from the start of the far side's log") is the
  documented restart path.
- So a deployment where SQLite acts as a pruned buffer breaks on its first
  restart.

## Conditions (what was run, and how it differs from the plan)

Everything below was added **after** the sections above were committed at
`4eb3532a`; the criteria and the predictions are unchanged. Measured on
rustc 1.97.1 against the tree at `1cf9d418` (`results/environment.txt`: no
file under `crates/` modified). Reproduce with `bash run.sh`; the transcript of
the run recorded here is `results/test-all.txt` (23 tests, all passing, each
asserting what was observed), and `results/clippy.txt` is the lint run.

- **Layout.** `src/` holds the support: `domain.rs` (the claim events, the
  `Counter` projection that counts applications per `EventId`, the
  `ClaimFile` decision model), `shared.rs` (an `Arc` handle on
  `MemoryIngestStore`), `holed.rs` (`WithholdingStore` with `WriteGroupStore`
  and `IngestStore` added), `prune.rs` (a raw `DELETE` on a closed SQLite
  file), `seams.rs` (every port answer, reduced to a position-free `Shape`)
  and `group_read.rs` (below). The scenarios are one test binary,
  `tests/it/`, one module per reader, rather than one binary per file.
- **The SQLite store's `read` is served from its write groups.**
  `SqliteEventStore::read` needs a tokio runtime on first poll
  (`SqliteEventStoreError::NoRuntime` otherwise), and this manifest has no
  `tokio`. Every other SQLite port method the experiment calls runs inline. So
  the projection over the SQLite spoke reads through `GroupRead`, which
  answers `read` from `WriteGroupStore::read_groups` over the same rows. It is
  exact only where every group was **written** with one event, which holds in
  every R2-dst scenario (one event per append, one per hub group). Holding one
  event now is not enough: SQLite names a group by the position it was written
  at (`coalesce(write_group, position)`), so the survivor of a cut two-event
  group reads back under the deleted row's position. `GroupRead` refuses a
  group of more than one event, a locally minted event whose own position is
  not its group's name (how a cut local group shows itself;
  `a_cut_group_of_local_events_is_refused`), names that do not strictly
  increase, and backward reads. A cut group of *ingested* events cannot be
  detected, because a foreign id carries its origin's position, so for those
  the one-event-per-written-group precondition is the scenarios' to keep, not
  the adapter's to check. The runner, the ingest and the edge are the
  published code, unwrapped.
- **R2-dst-1's restored edge** is built as
  `Edge::restored(Watermark::new(hub, confirmed))`: `MemorySyncPeer` never
  names its store, so `Edge::watermark()` stays `None`, and the caller who
  knows the hub's identity builds the watermark, as that method's docs say.
- **R3** uses the typed layer's `commit`. The plan described it as "refuses to
  note an unregistered claim"; staged that way, missing history makes the
  decision refuse *more*, not admit. The prediction row ("admits what the full
  log refuses") is realised by the uniqueness decision ES-40 is about:
  "register claim `a` unless it is registered".
- **One scenario beyond the predictions**, R2-dst-3, found while staging
  R2-dst-2: the same refusal on the hub side.

## Results

Columns: C1 harm; C2 undetectable with every value the reader can reach
today; C3a keeps today's behaviour on `Unknown`; C3b a truthful report does
something materially better that the store's concrete type could not; C4
`EventStore` is the seam. "–" means not reached, because an earlier criterion
already failed.

### R1, the projection runner (`tests/it/r1_runner.rs`, `tests/it/floor.rs`)

| Scenario (test) | Observed | C1 | C2 | C3a | C3b | C4 | Seam owning a cure |
|---|---|---|---|---|---|---|---|
| R1-1 `rebuild_over_a_scattered_purge_is_smaller_and_says_nothing` | 7 of 12 applied, exactly the retained ids, none twice; checkpoint at the highest retained position; port shape identical to a young twin | no | – | – | – | – | none needed |
| R1-2 `pruning_behind_the_checkpoint_changes_nothing` | after withholding the 6 applied events and appending 2, the run applies 2; the view equals the full log's 14, none twice | no | – | – | – | – | none needed |
| R1-3 `pruning_ahead_of_the_checkpoint_is_never_delivered` | 4 of the 6 later events applied; the checkpoint passes the 2 withheld positions; neither ever delivered | no | – | – | – | – | none needed |
| Floor `a_floor_misdescribes_a_scattered_purge` | the floor is the lowest survivor's position, and all 6 withheld positions sit above it | evidence for ES-39's *Rejects* line and ADR-0028 decision 4 | | | | | |
| R1-4 `a_source_switch_resumes_against_the_wrong_store` | one projection store, checkpoint from store A resumed on store B: B's 4 own events silently skipped, 1 event already applied from A applied again | **yes**: a checkpoint skips events the store holds (PS-20, `spec/SPECIFICATION.md:6119`, resumes "strictly after the checkpoint's position", of a different store; ES-29, `:4420`) | no, but only by the criterion's letter: `run_projection` binds `EventStore`, which has no `store_id`; the identities differ and are reachable through `IngestStore::store_id`, a value C2 lists | – | **no, and this decides it**: both stores are complete, so a completeness report has nothing to say | **no, and this decides it**: the missing fact is the checkpoint's source store | the typed layer's checkpoint (record the source store) |

### R2-src, ingest pulling from a holed hub (`tests/it/r2_ingest_source.rs`)

| Scenario (test) | Observed | C1 | C2 | C3a | C3b | C4 | Seam owning a cure |
|---|---|---|---|---|---|---|---|
| R2-src-1 `a_spoke_pulling_a_scattered_hub_holds_the_retained_set_and_reports_synced` | spoke holds exactly the 7 retained ids; edge synced, `Pending::Exactly(0)`; the spoke's watermark for the hub is the highest retained origin position, no trace of the holes; port shape identical to a young twin | no (ES-38, SY-27: the spoke holds what the hub holds) | – | – | – | – | if counted as harm, `PeerLimits` / SY-32, not `EventStore` |
| R2-src-2 `withholding_after_a_pull_leaves_the_spoke_with_more_than_the_hub` | spoke keeps 10, hub view holds 7, edge synced; nothing reconciles | no (SY-1, SY-34) | – | – | – | – | none |
| R2-src-3 `a_purge_that_cuts_a_write_group_ships_the_remainder` | a two-event group with its first event withheld arrives as a group of one | no, on this README's reading of SY-30 (`:8152`): "group whole" binds groups as the store holds them | – | – | – | – | if counted, `WriteGroupStore` (question for #158) |

### R2-dst, ingest into a pruned store (`tests/it/r2_ingest_destination.rs`)

| Scenario (test) | Observed | C1 | C2 | C3a | C3b | C4 | Seam owning a cure |
|---|---|---|---|---|---|---|---|
| R2-dst-1 `a_restored_edge_resurrects_pruned_foreign_events` | after a raw `DELETE` of 2 of 4 foreign events (one the origin's highest), the spoke's origin watermark regresses; `Edge::restored` re-pulls from the start; `Ingested.appended == 2`, both at the tail above the pre-prune head; the watermark recovers; **the projection applies both a second time** | **yes**: the documented restart path's promise breaks, *"a re-read is harmless, because ingest skips what it holds (SY-11)"* (`crates/happenstance-sync/src/runner/watermark.rs:20-24`, and `runner.rs:113-114`); VT-8's uniqueness (`:937`) held only among rows still present | yes: inside ingest the row is simply absent | yes | **no**: the information missing is "which *origin identities* were once held", keyed on the origin pair, and no report about local positions identifies a re-delivery; the only store that could answer truthfully pruned through its own API | **no** | `IngestStore::ingest` in each adapter (a pruned-identity tombstone), or a persisted pull token with the edge |
| R2-dst-2 `a_restored_edge_stops_on_a_pruned_own_event` | after a raw `DELETE` of one of the spoke's own events the hub had confirmed, the restored edge's pull brings it back; it lands, and ingest refuses the whole call: `StepError::Ingest(IngestError::UnheldOwnIdentity { id })` with the pruned id; edge `Stopped`; nothing landed, not even a foreign event in the same pull; a second step returns `StepError::Stopped` without calling the peer (counted: one pull on the refused step, none after). The refused page, pulled directly, carried both the pruned id and the foreign event | **yes**: SY-38 (`:7500`) refuses with the stated cause *"restored or rewound without re-minting its identity"* (`ingest/error.rs:118-121`), which is false: the store was pruned, and its remedy, re-minting, is the wrong one; the edge is wedged | yes: the edge sees only the error | yes: on `Unknown` it stops, as today | **no**: the check runs inside the adapter's ingest transaction; a generic edge consulting a report after the error is a read-then-act VT-8 already refuses to build on; the adapter's own prune record is concrete-type knowledge | **no** | the adapter's ingest (a record of pruned own positions consulted where SY-38 is checked), and SY-38's wording |
| R2-dst-3 `a_fresh_edge_pushing_into_a_pruned_hub_stops` (not predicted) | a SQLite hub prunes one of its own events a spoke holds; a spoke edge without its watermark (`Edge::new`) pushes its whole log back; the hub's ingest refuses as `StorePeerError::Ingest(IngestError::UnheldOwnIdentity { id })` and the spoke's edge stops | **yes**: as R2-dst-2, on the hub | yes | yes | no, as R2-dst-2 | no | as R2-dst-2 |
| Control `control_a_fresh_edge_pushing_into_an_unpruned_hub_is_a_no_op` | R2-dst-3's staging with nothing pruned: the fresh edge offers its whole log back, the hub skips it all (head unchanged), `appended == 0`, the edge keeps running | no harm: the stop in R2-dst-3 is the prune's doing | | | | | |
| Control `instrument_control_resurrection_is_masked` | the same staging over `Holed` (withheld, not deleted): the re-pull skips both, `appended == 0` | no harm visible | | | | | |
| Control `instrument_control_false_rewind_is_masked` | the same staging over `Holed`: the re-pull skips, the edge keeps running | no harm visible | | | | | |

The controls show that #144's stated fidelity gap
(`crates/happenstance-testkit/src/withholding.rs:150-154`, "ingest
deduplication") is material: the decorator hides both ingest-side harms. Claims
about ingest (#151, #154, #158) need physical removal, the #147 capability, not
the decorator.

### R3, a decision model (recorded, not in the verdict; `tests/it/r3_decision.rs`)

| Scenario (test) | Observed |
|---|---|
| `a_decision_over_a_holed_log_admits_what_the_full_log_refuses` | "register `a` unless registered": the full store refuses (`CommandError::Refused`); with the registration withheld the holed store admits a second registration, its condition passing vacuously (ES-40's MAY, `:4923`); the port shape is identical to a young twin. A decision layer that refused on `Unknown` would refuse on every published adapter, which would answer `Unknown` for ever (C3a). |

### Predictions against observations

Every predicted row was observed as predicted. The two differences are the
R3 staging described under *Conditions*, and R2-dst-3, a third harm of
R2-dst-2's class that the predictions did not name.

## Verdict: refusal

By the rule above, mechanically:

- **No reader meets all four criteria.** The four harms observed (R1-4,
  R2-dst-1, R2-dst-2, R2-dst-3) each fail C4, and each fails C3 or C2 as well;
  every other scenario fails C1.
- **Not escalate.** Escalation needs a reader meeting C1, C2 and C4; none
  meets C4, so ADR-0028's falsifier ("the report would have to be required")
  did not fire.

So ES-39 resolves to ADR-0028 decision 9's refusal: **the port does not
report completeness, and a caller MUST NOT infer it from any port value.** The
seam evidence agrees: in R1-1, R2-src-1 and R3 a pruned store and a young twin
holding the same events gave identical answers at every seam the port has, the
position values aside, which ES-38 forbids anyone to read.

This experiment does not move ES-39's marker; per the plan's D3 that is #163's
freeze, with ES-40, SY-32 and CF-27.

## Findings (candidate issues; whatever the verdict)

### F1. Replica pruning breaks re-pull: resurrection (SY-11, VT-8, SY-31)

A store that deletes foreign events outside the port re-admits them, at the
tail, the next time an edge pulls from the start, which `Edge::restored`
always does; a projection already past them applies them again.

Reproduction (`a_restored_edge_resurrects_pruned_foreign_events`):

1. A `MemorySyncPeer` hub seeded with four single-event groups minted by a
   third store `X`; a `SqliteEventStore` spoke; `Edge::new()` stepped until
   synced; a `Counter` projection run over the spoke (4 applied).
2. Keep `status().confirmed`; drop the spoke; `DELETE` the rows of `X`'s 2nd
   and 4th events (`prune::prune_outside_port`); reopen.
3. `Edge::restored(Watermark::new(hub, confirmed))`, stepped until synced:
   `Ingested.appended == 2`, both at positions above the pre-prune head.
4. Run the projection again: 2 applied; `Counter::applied_twice()` is exactly
   `X2, X4`.

Breaks the promise at `runner/watermark.rs:20-24` ("a re-read is harmless")
for any store that forgets. The mechanism is shared: Postgres, Neon and
Cloudflare dedupe on the same origin pair, though only SQLite was run here.
Seams that could own a cure: a pruned-identity tombstone in each adapter's
`IngestStore::ingest`, or a persisted pull token. Not `EventStore`.

### F2. Replica pruning wedges an edge: a false `UnheldOwnIdentity` (SY-38)

A store that deletes one of its own events after the far side confirmed it
gets that event back on the next pull from the start; it lands, and SY-38
refuses the whole ingest with class stop, naming a restore or rewind that did
not happen. The edge is stopped for good, and nothing else in the batch lands.
The hub side fails the same way when a spoke pushes a hub's pruned event back.

Reproductions:

- Spoke side, `a_restored_edge_stops_on_a_pruned_own_event`: three local
  appends on a `SqliteEventStore` spoke, synced to a `MemorySyncPeer` hub
  (pushed and confirmed); the hub then learns a foreign event; drop the spoke,
  `DELETE` its first own row, reopen; `Edge::restored(Watermark::new(hub,
  confirmed)).step(..)` returns
  `StepError::Ingest(IngestError::UnheldOwnIdentity { id })` for the pruned id;
  the state is `EdgeState::Stopped`; the head is unchanged and the foreign
  event did not land, though the page the edge pulled (read directly with
  `hub.pull(None)`) carried it beside the pruned id; the next step is
  `StepError::Stopped` and calls the peer not at all.
- Hub side, `a_fresh_edge_pushing_into_a_pruned_hub_stops`: a
  `StoreSyncPeer` over a `SqliteEventStore` hub with three own events, synced
  to an in-memory spoke; drop the hub, `DELETE` its first row, reopen; a fresh
  `Edge::new()` on the spoke pushes its log back, and the step fails as
  `StepError::Push(StorePeerError::Ingest(IngestError::UnheldOwnIdentity { id }))`
  with the edge stopped. Its control,
  `control_a_fresh_edge_pushing_into_an_unpruned_hub_is_a_no_op`, stages the
  same push with nothing pruned: every event is skipped and the edge runs on.

This is the deployment where a SQLite file acts as a pruned buffer, failing on
its first restart. Seams that could own a cure: the adapter's ingest (a record
of pruned own positions where SY-38 is checked) and SY-38's wording, which
today names only a restore or rewind. Not `EventStore`.

### F3. A projection checkpoint is not pinned to its source store

R1-4 (`a_source_switch_resumes_against_the_wrong_store`): one projection and
one projection store resumed against a second event store silently skip that
store's own events below the old checkpoint and re-apply one above it. No
completeness report bears on it; the cure is the typed checkpoint recording
the source store's identity. The deployment note's candidate issue 1.

**Closed by #485** (PS-40, ADR-0109): the typed runner now pins a checkpoint to
the store it was built from, and R1-4 asserts the refusal,
`ProjectionError::SourceChanged`, where it recorded the hazard. Its name and its
history are kept.

### F4. Questions for #158, not harms on this reading

- R2-src-3: a purge that cuts a write group ships the remainder as a group of
  one. Whether SY-30's "group whole" binds groups as written or as held.
- R2-src-2: a hub that forgets after a spoke pulled leaves the spoke holding
  more than the hub, and nothing reconciles it (expected under SY-1 and
  SY-34).

## What this does not license

- **Other adapters for R2-dst.** Only SQLite was pruned. Postgres, Neon and
  Cloudflare dedupe on the same origin pair and check SY-38 the same way, so
  the mechanism is shared, but none was run.
- **The decorator for ingest-side claims.** The two controls show CF-27's
  instrument masks both ingest harms. R2-src runs through it and is faithful,
  because a source only reads; R2-dst does not, and used a real `DELETE`.
- **A position-report method by other means.** The refusal is about
  `EventStore`. F1 and F2 may well need state somewhere, a tombstone or a
  token; this experiment says where it does not belong, not what it is.
- **The SQLite store's own `read` path.** R2-dst's projection read the spoke
  through `GroupRead` (see *Conditions*), exact where every group was written
  with one event; the
  ingest and edge paths were the adapter's own.
