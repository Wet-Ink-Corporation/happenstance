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
