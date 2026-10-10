# Ledgers

What is still open, and who owns it. Read this when a clause moves, a phase
exits, or an ADR is written — not at the start of every session; the
[index](README.md) is what a session starts from.

Three ledgers live here. The **ADR queue** is the decisions owed, in write order.
The **open decisions** are the rows of the old decision ledger that no ADR or
clause has settled yet. The **clause ledgers** are held to the specification by
`cargo xtask lints` (`runbook_clause_ledgers_match_the_specification`), which reads
this file: a `[PROVISIONAL]` or `[DEFERRED]` clause missing from its table, a row
naming a clause that has moved, or a row with no owning phase fails the gate.

The full history of all three — the struck rows, the settled decisions, the
queue as it was written — is in the archived monolith, `RUNBOOK.md:348-797` at
`f89e184`. Nothing was deleted to make this file short; it was left behind.

## The ADR queue

**The next free number is 0088; 0081–0087 are reserved by phase 17 lanes in flight.** `ls .kb/decisions/` is the answer that cannot go
stale, and `references/adr/` holds the long-form records; not every atom has one.
Phase 16 wrote 0066 – 0071: the charter (0066, `what-1-0-promises`), the MSRV after
1.0 (0067), ADR-0022's §§8 and 16, with §9 left to phase 17 (0068), `QueryItem`'s total constructor (0069),
the runner's named `Chunk` (0070) and ES-10's scope (0071). Phase 17 opened with
0072, which split it at its release and created phase 17b, and wrote 0073, VT-10's
foreign-identity write path, and 0028, retention, under its reserved number. Then
0074, the `Projection::apply` record, and 0075, the projection port's 1.0 clauses.
Lane L4 wrote 0076: the conformance emitters CF-23 obliges are public API, un-hidden and
renamed without their `__` prefix, and CF-41, minted `[FROZEN]`, pins them.
Lane L5 wrote 0077: `AppendError::Busy` is promised at 1.0, the typed commit loop retries it
inside the same `Retry` bound, and ES-43 is minted.
The ladybug lane wrote 0078 (`happenstance-ladybug` retired on the owner's call, `wi-630032`), and
lane L6b wrote 0079 (a Cloudflare query item binds constant parameters; widths 5 and 90).
Lane L7 wrote 0080: `append` keeps its borrowed batch, and ES-17 is frozen on the two-build
measurement against `happenstance-cloudflare` (`experiments/append-batch-ownership/`).
Lane L10 wrote 0082: `ProjectionId` is validated, `sync/` and `happenstance/` are reserved, VT-35 and PS-39 are minted.
0083 answers a breaking open question, accepted by the owner on 2026-10-08: `Codec` stays
unsealed through 1.x and `UnknownTag` is not split (`should-codec-be-sealed`).
The breaking-questions lane wrote 0084, **accepted** by the owner on 2026-10-08: the projection batch's SQL seam is
`&'static str` plus a named escape hatch at 1.0, `LivePostgresBatch::execute` included, and the
parameter count is stated once, with an adapter-side count check proposed for Postgres. Neon's `0.4.0` narrowing
is the owner's default; the record proposes its shape, and its code lands in its own lane PR.
The VT-6 lane wrote 0086, **accepted** by the owner on 2026-10-08 (Postgres and Neon keep mint-once with a documented
re-mint; mint-per-open declined; default-refusing detection ruled out after 1.0).
Lane L8 wrote 0087, **accepted** by the owner on 2026-10-08: `happenstance-neon` meets ES-11 and ES-12 through a read-settlement fence, measured on the live endpoint (`experiments/es-11-fence/`). It supersedes ADR-0061, and the fence itself lands as a separate change.
Numbers 0026–0028 were reserved by the original queue for phases 13 and 14, which is
why they are out of order with the numbers around them. 0026 and 0027 are still
unwritten.

| ADR | Phase | The question it answers |
|---|---|---|
| **0026** | 13 (published-surface half: 17) | What is a sync *peer* — what may the port assume about a transport it cannot see, and what does ingest promise? (SY-8 – SY-18). ~~Phase 17 must settle the part of this that reaches a published crate~~ **Published half settled at 17 — ADR-0073**: `happenstance-core` needs no foreign-identity write path; the adapter's own row writer takes one, and VT-10 is frozen. ADR-0026 cites it for that half |
| **0027** | 13 | How do two logs reconcile — the merge rule, the compensation contract, and whether hub-and-spoke and peer-to-peer are one abstraction or two? (SY-1 – SY-7, SY-19 – SY-31). SY-1 – SY-7 already settle *ingest is unconditional, with compensation*; this ADR records it and settles the rest |
| **0028** | 14 (decision: 17) | What is a store permitted to forget, and how does it say so? (ES-39, CF-27, SY-32). ~~**The decision moves to phase 17** because one of its two answers adds a method to `EventStore`, which breaks every published adapter, and that is only cheap before 1.0. Phase 14 builds what it decides~~ **Settled at 17 — [ADR-0028](../.kb/decisions/0028-what-a-store-may-forget.md)**: the written refusal with an additive reservation. Deletion stays outside the port through 1.x, and a later report arrives only as a provided method defaulting to `Unknown`, which a compiling spike showed is additive (`experiments/provided-method-spike/`). ES-41 and PS-22 are frozen by it; phase 14 builds the instrument and freezes ES-39, ES-40, SY-32 and CF-27 |
| **0066** | 16 | What does 1.0 promise? The charter phase 16 owes: which crates, what happens to every non-`[FROZEN]` clause on a published surface, adapter and testkit versioning (CF-32), and the MSRV policy after ADR-0037. **Written** as `what-1-0-promises`; the MSRV half is its own record, ADR-0067, and the per-clause answer is [the 1.0 dispositions](#the-10-dispositions) |
| **0086** | 17 | VT-6 for Postgres and Neon: mint-per-open, or not (`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md`). **Settled at 17 — [ADR-0086](../.kb/decisions/0086-postgres-and-neon-keep-mint-once.md)**, accepted 2026-10-08: declined for both; mint-once earned by a documented re-mint; the additive arms are phase 13's |
| ~~*unnumbered*~~ | 17 | ~~Is `Projection::apply` synchronous, and may a projection issue a statement into a live batch? (`.kb/open-questions/projection-apply-is-synchronous-against-a-live-store.md`). Owed its own record by the note that froze the port (ADR-0063) and owned by nobody until now~~ **Settled at 17 — [ADR-0074](../.kb/decisions/0074-projection-apply-is-async.md)**: `apply` becomes `async` on the one trait, with a `SendProjection` flavour through `trait_variant`, handed a position-free `Delivered` event and the batch, on a spike that drove a live transaction (`experiments/apply-shape/`). Phase 18 builds it. PS-9 and PS-11 are frozen by it; PS-15, PS-23 and PS-24 by its companion, [ADR-0075](../.kb/decisions/0075-the-projection-ports-1-0-clauses.md) |
| *unnumbered* | 19b | Does the SQLite schema and append-condition SQL get a home that is not an adapter, once a third copy exists? (`references/seeds/sqlite-on-wasm.md`) |

## Open decisions

The rows of the decision ledger (`RUNBOOK.md:505-619` at `f89e184`) that were still
open when the runbook was split, and the owner each now has. A row leaves this
table when an ADR or a clause settles it, and the archive keeps it either way.

| Decision | Phase | Status |
|---|---|---|
| ~~`conflicting_position`: a promise every adapter owes, or a hint one may omit~~ | 16 | **settled at 16** — a hint, and already so: ES-25 is `[FROZEN]` and ADR-0012 §8 says it. Neon-over-HTTP was the forcing case |
| ~~Is ES-10's global visibility statement what happenstance needs, or would a per-boundary one do~~ | 16 | **settled at 16 — ADR-0071**: ES-10 stays global, because ADR-0063 froze a single-position checkpoint (PS-17, PS-20). Raised by the phase-2 measurement |
| ~~`append`'s ownership of its batch — two builds of the same adapter differing only in ownership (ADR-0012's falsifier item 1, escalated by ADR-0022 §13)~~ | 17 | **settled at 17 — [ADR-0080](../.kb/decisions/0080-append-keeps-a-borrowed-batch.md)**: `&[Event]` kept and ES-17 frozen. The two builds were taken on `happenstance-cloudflare` and a rule fixed before the run fired in 0 of 9 cells; owning saves 2 heap operations per event, and a raw caller resending under by-value pays +90–95% |
| ADR-0022's falsifiers have fired: supersede, re-open (scoped) or ratify its pragma and runtime-seam sections | 17 | **§8 and §16 settled at 16 — ADR-0068**; §9 (the runtime seam, the captured `Handle`) is **reproduced; ADR-0081 accepted 2026-10-08**: a stranded store reports `Worker(JoinError::Cancelled)`, not `NoRuntime`, and the remedy (prefer the executing runtime) is behavioural, so `0.4.0`. Settled by it; `adr-0022-falsifiers-have-fired` is closed. Forced by phase 12 |
| ~~Whether the 2026-09-06 `msrv-premise` ratification supersedes ADR-0037, and what the floor is~~ | 16 | **settled at 16 — ADR-0067**: the floor holds at 1.97.1 and the ratification, never executed, is withdrawn (`msrv-ratification-conflicts-with-the-accepted-floor`) |
| ~~Whether a workerd-class runner enters `cargo xtask ci`, and which platform clauses stay unproven without one~~ | 16 | **settled at 16 — ADR-0066**: a workerd sibling CI job, not a gate step, lands before 1.0 as phase 17's work (`no-workerd-class-runner-in-the-gate`) |
| ~~Who owns reconciling the specification after each phase, and whether `spec-trace` mechanises it~~ | 16 | **settled at 16**: the session protocol's step 6 owns it, and each phase file carries an exit line for it (`nothing-owns-the-post-phase-reconciliation`) |
| ~~Which read fault `PostgresFixture` arms, and whether it needs new capability machinery~~ | 16 | **closed at 16**: answered by `2ed06b4` (`postgres-fixture-read-fault-declension-is-owed`) |
| ~~An infallible pre-validated `Tags` / `Tag` constructor in `happenstance-core`~~ | 16 | **settled at 16 — ADR-0069**: no infallible `Tags`/`Tag` path is added (the outward face is closed as intended, so ADR-0033 stays closed); `QueryItem` gains a total constructor instead, built in phase 17 (`d-1-the-validated-type-has-no-total-path`) |
| ~~A named `Chunk` type and an observation seam for `run_projection`~~ | 16 | **settled at 16 — ADR-0070**: a named `Chunk`, which phase 18 builds before it lifts the gate, and no observation seam at 1.0 — one is additive later (`projection-runner-chunk-type-and-observation-seam`) |
| ~~Whether `remint_identity`'s documented procedure owes an in-process check~~ | 16 | **closed at 16**, with the gap routed to phase 13 (`remint-identity-precondition-is-trust-only`) |
| Is replication whole-log or scoped | 13 | deferred — SY-27, SY-28 |
| Idempotent bulk ingest inside one round trip | 13 | deferred — SY-14 |
| Does a peer declare its own limits | 13 | deferred — SY-18 |
| Where a per-peer watermark lives transactionally | 13 | provisional — SY-31 |
| ~~What is a store permitted to forget, and how does it say so~~ | 17 → 14 | **settled at 17 — ADR-0028**: the written refusal, with a provided-method reservation. Phase 14 builds CF-27's instrument and freezes ES-39, ES-40, SY-32 and CF-27 against it |
| A projection's `Query` changed under its checkpoint | 18 | provisional — PS-25 |
| Snapshotting decision-model state | after 1.0 | deferred — DCB queries are narrow by construction; revisit if replay cost is measured |
| `tracing` spans and metrics | after 1.0 | deferred — purely additive, no port change |

## Where the open clauses and the blocked cases land

Three obligations, all mechanical to check, all previously answerable only by
prose search — and the middle one previously not answerable at all, which is why
phase 12's audit of it had nothing behind it.

### The 12 `[DEFERRED]` clauses

Sixteen rows are listed; four are struck. (This sentence said seventeen until
phase 16 counted them.) Two were settled at phase 2 and their
markers moved in the specification at phase 3; the third, CF-13, was settled at
phase 3 by running the experiment its own marker named; the fourth, PS-33, was
**demoted rather than settled** — §7.2 carries it as `NON-NORMATIVE`, so it is not
a deferral at all any more. **Twelve remain.** The struck rows stay because a
reader arriving from an older commit needs to find them, and because a table that
quietly loses a row cannot be checked against anything.

Three of the twelve — PS-18, PS-27 and PS-30 — are new to this table and were not
new to the specification. They sat in the **provisional** ledger below while §7.2
carried them as deferred, which is most of why that ledger's heading said
forty-nine against a specification that said forty-seven. They did not disappear;
they moved.

| Clause | What it defers | Owning phase |
|---|---|---|
| ~~ES-6~~ | ~~`Error: Send + Sync`~~ | **settled at 2 — ADR-0009**, and `[FROZEN]` in the specification since phase 3 |
| ES-39 | a store reporting history it does not hold | 14 |
| WF-1 | DCB wire interoperability | 13 |
| ~~PS-33~~ | ~~evaluating ADR-0007's falsifier~~ | **demoted at §7.2 to `NON-NORMATIVE`**, and therefore not a deferral. It was a clause about this document's own work rather than about a store's behaviour |
| ~~PS-35~~ | ~~one derivation ADR covering both ports~~ | **settled at 2 — ADR-0008**, and `[FROZEN]` in the specification since phase 3 |
| PS-18 | a refusable reset — `refused_reset_changes_nothing` | 6 — `projection-store-freeze` (HS-P0010), which takes the count when the first adapter over storage this workspace does not control clears the projection suite. Evaluated at the typed layer's phase exit and the count came back **unavailable rather than zero**: the mechanism and its rule both exist; no projection adapter has shipped to implement protection |
| PS-27 | "skip and record" atomicity on a projection failure | 6 — `projection-store-freeze` (HS-P0010), which owns both the suite rule and the port surface a skip record would be written through. The count is **zero**: the alpha's runner halts on the first failure and offers no failure-policy seam, so no path could write a skip record. **Not withdrawn** — the Kestrel Motor shred case still needs it |
| PS-30 | a panicking `apply` rolls back | 6 — `projection-store-freeze` (HS-P0010), once a fan-out runner is buildable at all. The runner is not built, so the MUST binds nothing today. Its contingency is a **benchmark rather than an assertion** and is deliberately outside the gate under CF-34; the harness now exists as `experiments/polling-cost`, which is the artefact that reopens this clause |
| SY-14 | idempotent bulk ingest in bounded round trips | 13 |
| SY-18 | peer-declared limits | 13 |
| SY-27 | scoped versus whole-log replication | 13 |
| SY-28 | scope preservation across a round trip | 13 |
| SY-32 | retention gaps reported rather than silent | 14 |
| ~~CF-13~~ | ~~a fixture that can *fail* the visibility rule~~ | **settled at 3**: `PreCommitPositionStore` fails `nothing_below_an_observed_position_appears_later` deterministically on one thread, the `Send + Sync` sub-trait the marker held in reserve was not needed, and the clause is `[FROZEN]`. The **adapter** far end stays open and is §6.5's position-allocation row, owned by phase 10 |
| CF-14 | durability across a reopen | 8 — the *fixture* half landed early at phase 3 as a named exception (`REOPEN`, `acknowledged_writes_survive_a_reopen`, `LosingFixture`). **Phase 9's reading: the deferral still holds, and one more of the three named implementations has answered with one shape** — `CloudflareFixture` expressed `REOPEN` through the same contract shape `MemoryFixture`, `DurableFixture` and `SqliteFixture` use, and `acknowledged_writes_survive_a_reopen` with its two neighbours ran and passed with **zero** `SKIP` in the wasm32 conformance run, so "durable" needed no grading. **The far end is untouched** — this project supplied no store that can lose an acknowledged write to a *fault* rather than to an instruction, so that half stays phase 8's and `sqlite-durable-store`'s (HS-P0012), and what the run could *not* exercise is in phase 9's session log, *The CF-14 re-read* |
| CF-27 | the suffix-store instrument | 14 — **phase 9's reading: more real, not less, and this runtime is why.** *Eviction* is the wrong hazard — a Durable Object's storage outlives its isolate — but the same `Storage` object this adapter reaches through carries `delete_all()` (`worker-0.8.5/src/durable.rs:449`), so a wholesale purge happens with no `EventStore` method involved and the store passes every rule unchanged afterwards, which is how the specification's **No, and nothing is planned** completeness row comes to understate the exposure by one adapter. The instrument that would settle it — a testkit-adjacent store holding only a suffix and reporting that it does — is **deliberately not built here**: it stays `retention-and-incomplete-logs`' (HS-P0018) and phase 14's, and the argument is in phase 9's session log, *The CF-27 re-read* |

Checked against `SPECIFICATION.md` §7.2's maturity column, and **the two sets are
equal again as of the `0.2.0` release pass**. `cargo xtask spec-trace` counts
**twelve** `[DEFERRED]` clauses — WF-1, ES-39, PS-18, PS-27, PS-30, SY-14, SY-18,
SY-27, SY-28, SY-32, CF-14, CF-27 — and this table now lists exactly those twelve.

Phase 9 read this and recorded the drift rather than repairing it, on the ground
that reconciling a maturity table is a clause question and a prose re-read of
CF-14 and CF-27 had no licence to move a row. That was right then and it is what
made the repair cheap now: the three projection clauses were **named** as missing
rather than merely absent, so this pass had only to move them, and each one's
falsifier and owner came from its own marker rather than from a judgement made
here. **No deferred clause listed here is unowned**, which is the property that
paragraph was written to protect.

**One of them sits on a surface phase 12 publishes**, and it is safe. WF-1 (DCB
wire interoperability) is owned by phase 13, after publication, but the format is
private and WF-8 puts a version first, so phase 13 can change it without a wire
break.

ES-24 was the second and is no longer deferred. It asked how a caller resolves an
unknown outcome after a dropped `append` future, and was deferred on the
assumption that answering it needed an identity `Event` does not carry. It does
not: a conditional append whose events match its own condition is already
at-most-once under verbatim reissue, because the retry's `ConditionViolated` *is*
the answer "it landed". The clause is now `[FROZEN]` at phase 4, with the two
shapes that get no such guarantee stated as limits and a rule pinning each. The
deferral was larger than the question.

### The 30 `[PROVISIONAL]` clauses

Forty-one at the split. CF-39 was frozen at phase 16 by ADR-0066, VT-10 at phase 17
by ADR-0073, ES-41 and PS-22 at phase 17 by ADR-0028, PS-9 and PS-11 at phase 17
by ADR-0074, PS-15, PS-23 and PS-24 at phase 17 by ADR-0075, and ES-17 at phase 17 by
ADR-0080, and ES-11 and ES-12 at phase 17 by ADR-0087. PS-39 was minted provisional at phase 17 by ADR-0082. Their rows stay, struck,
because rule 4 keeps it; `cargo xtask lints` now checks the count in this
heading against §7.2 rather than matching it, so the next freeze changes one number
here and nothing in the lint.

Phase 12 cannot audit "every provisional clause has its falsifier scheduled"
against prose. Grouped by what falsifies them, because they do not fail
independently — most of the `PS` rows waited on one missing adapter until
ADR-0062 built it and ADR-0063 froze what it proved, and counting them as
separate open questions overstated the exposure by that factor.

| Group | Clauses | Falsified by | Owning phase |
|---|---|---|---|
| Identity's queryability and store-assigned time | VT-6, VT-9 | a peer that must dedupe without parsing `metadata`, and a rule that `recorded_at` is non-decreasing with position | 5, exercised 13 |
| The ingest seam's placement | ~~VT-10~~ | ~~a real peer needing a foreign identity through `EventStore::append` after all~~ | **frozen at 17 — ADR-0073.** The SQLite spike runs `IngestStore` through `append`'s own row writer; phase 13 confirms it on two real peers |
| Const-constructible identifiers | VT-14 | `from_static` failing to move the `?` count in the worked example | 5 |
| Store limits | VT-21 – VT-24 | a real adapter that cannot honour a stated minimum | 5, tested 8 and 10 |
| Per-item boundaries on a condition | VT-30 | E2E-04 and E2E-05 still unwritable after phase 4 | 4 |
| `Bytes`' human-readable form | WF-11 | a peer that cannot buffer a payload through any human-readable encoder | 9 |
| The `!Send` flavour and `append` ownership | ES-7 | the Cloudflare adapter, and `dynosaur` failing to erase a generic `append`. The row read *"ES-7, ES-17"* until ES-17 was frozen; its struck row follows | 1 and 4, confirmed 9 |
| `append`'s ownership of its batch | ~~ES-17~~ | ~~two builds of one owning adapter differing only in ownership~~ | **frozen at 17 — ADR-0080.** Taken on `happenstance-cloudflare` with replica arms; 0 of 9 pre-registered cells fired. Reopened by a `workerd` run where allocation dominates, or a large realistic payload |
| No tail seam at 0.1 | ES-32 | a Durable Object making one cheap enough to reopen | 9 (verdict), post-0.1 — **answered**; the one-paragraph verdict is in phase 9's session log. It answers *this* falsifier only: the clause's own benchmark-shaped one — E2E-32's fan-out runner holding N views within their staleness budget — is untouched and stays phase 7's |
| `begin`'s round trip | PS-6 | an adapter that must reserve something from its server and cannot afford the round trip. The row read *"PS-6, PS-9, PS-11, PS-15"* until those three were frozen; their struck row follows | 6, re-tested 11, narrowed 12 |
| The write seam's consumers and the foreign-batch hazard | ~~PS-9, PS-11, PS-15~~ | ~~a second generic consumer (PS-9, PS-11); a zero-cost instance-naming construction (PS-15)~~ | **frozen at 17.** PS-9 and PS-11 by ADR-0074, on `experiments/apply-shape`: a provided `on_error` wrote a skip row through `SqliteBatch` and a live `LivePostgresBatch` with no bound on `Batch`. PS-15 by ADR-0075, narrowed to `commit` and `reset` by ADR-0066's own route, with `rollback` left outside the MUST; its falsifier could fire only as a major, so it is restated as a post-1.0 reopening |
| Reset, chunked rebuild, query drift | PS-16, PS-25 | a rebuild that skips event 1, or a projection that cannot refuse a reset. The row read *"PS-16, PS-22 – PS-25"* until PS-22 was frozen, and *"PS-16, PS-23 – PS-25"* until PS-23 and PS-24 were; their struck rows follow | 6 |
| One id per commit, and the rebuilding variant | ~~PS-23, PS-24~~ | ~~a pair of read models that must be mutually consistent at every instant (PS-23); rebuild in place always wrong (PS-24)~~ | **frozen at 17 — ADR-0075.** A multi-id commit arrives, if ever, as an additive refusing `commit_all` (`experiments/provided-method-spike/`, compiled with a `CommitError` variant; the separate error type ADR-0075 chooses is not yet compiled); `Rebuilding` is kept, because removing it would be a major and a swap protocol is additive |
| Checkpoint regression | ~~PS-22~~ | ~~a legitimate need to move a checkpoint backwards without clearing rows; a compacting store that renumbers~~ | **frozen at 17 — ADR-0028.** ES-38 forbids renumbering, and retention never rewinds a checkpoint over kept rows |
| Compensation's shape and the merge rule's details | SY-7, SY-10, SY-20 – SY-23, SY-29, SY-30 | two unlike peers that cannot both express it | 13 |
| Where a per-peer watermark lives | SY-31 | a peer with no transaction to put it in | 13 |
| Projection-id keys | PS-39 | a store whose key column cannot hold a 255-byte UTF-8 id byte-faithfully | 17b |
| Membership as a port operation | ~~ES-41~~ | ~~an adapter that cannot answer membership without a structure VT-8 does not already oblige. Phase 8 answered the **in-process, connection-holding** half — `happenstance-sqlite` reads the `UNIQUE (origin_store, origin_position)` pair migration 1 already creates. The **transport** half is open: a store with no connection, no interactive transaction and no cursor, for which the probe is a whole extra round trip~~ | **frozen at 17 — ADR-0028.** Neon meets the transport wording's letter: its probe is one extra read-only round trip, over the pair VT-8 indexes, and passes live. It is accepted because no required path pays it — ingest deduplicates inside the write (`crates/happenstance-sync/src/ingest.rs:97-101`) — and ADR-0066:160 dispositioned the half as answered. The completeness half is settled by decision: `false` is store-relative. Held-versus-visible under ES-10's frontier is left open, to phase 13 |
| Checkpoint visibility after commit | PS-38 | a store answering `checkpoint` from a replica that may lag its own `commit` — the shape a projection store over an eventually-consistent read model has. If real, the obligation narrows to "a subsequent read through the same handle" and every rule downstream gains a handle constraint | 7 — the first projection adapter over storage this workspace does not control |
| A fixture's fault-injection promise | ~~CF-39~~ | ~~a real adapter whose only injectable mid-batch fault is one its driver transparently absorbs — a connection killed mid-statement behind a reconnect-and-retry pool — which would make "the append returns `Err`" a promise no fixture over that adapter can keep~~ | **frozen at 16 — ADR-0066.** Owned by 8 and 10, whose row read *"no adapter has armed a fault yet"*; three now do — `happenstance-postgres`, `happenstance-neon` and `happenstance-cloudflare` |
| A fixture's stated capacity ceilings | CF-40 | a real adapter whose ceiling is **not a constant** — a Postgres row whose TOAST threshold moves with the rest of the row, or a KV store whose per-value cap moves with the key — for which a single `Option<usize>` cannot say where the boundary is, and the rule built on it would assert a number the store cannot honour | 9 and 10, **whichever states a varying ceiling first**. Phase 9 has landed and answered the *constant-ceiling* half — `CloudflareFixture` states all three as `Some(…)`, as `SqliteFixture` does — so it added a second constant-ceiling store and left the live half untouched. That half is **phase 10's**: Postgres is where a ceiling that moves with the row first appears |
| Durability's rule shape; benchmarks are not conformance | CF-17, CF-34 | a store that loses an acknowledged write, and an adapter that scans where it should seek and passes every rule | 8 |
| **The portfolio's residual exposure** — the clauses §1.3 names as carrying CF-25's risk in their own markers rather than in a preamble. **Two, not five**: ES-10 was lifted at phase 4 and `[FROZEN]` since, and carrying it here is what made this table's count disagree with the specification's. The row read *"ES-11, ES-12, ES-35, ES-40"* until ES-11 and ES-12 were frozen at phase 17; their struck row follows | ES-35, ES-40 | the far-end **adapter** on each axis, and nothing short of it: transport (ES-11, ES-12) by a one-shot-HTTP store; durability (ES-35) by a store that can lose a write to a fault; completeness (ES-40) by a store holding a suffix. A **fixture** instrument does not falsify any of them — CF-26 says so in terms. ES-11 and ES-12 were answered by ADR-0087 and frozen at phase 17; the struck row that follows records it. | 8 (ES-35), 14 (ES-40); 10 for ES-11 and ES-12 until they froze |
| Transport's residual exposure | ~~ES-11, ES-12~~ | ~~a one-shot-HTTP store at the transport axis's far end~~ | **frozen at 17 — ADR-0087**, which supersedes ADR-0061's keep. `happenstance-neon` meets both through `SqlTransport::reads_settled`: an append waits until every read its transport dispatched earlier has been answered. 172 of 1,500 live trials red without the fence, 0 of 1,500 with it. The claim is scoped to one transport value. Reopened by a red where the append was dispatched no earlier than the read was answered |

Every group names a phase in the [status table](README.md#status). **PS-2 is the single
gate under thirteen of these rows**, which is why phase 6 is worth its six days
and why phase 11's verdict on whether the freeze held is a result either way.

**The five in the last row were missing from this table until phase 3's close,
and the omission is the exact defect the table exists to prevent.** They are the
clauses `SPECIFICATION.md` §1.3 names, by number, as the ones holding the CF-25
exposure — the five phase 4's exit criteria have to cite — and a phase-12 audit
reading this table alone would have found forty-one falsifiers scheduled under a
heading that says forty-six and concluded that everything was owned. The heading
was right; the rows were short. Counted again at phase 3's close, group by group,
against `spec-trace`'s own list of `[PROVISIONAL]` clause IDs: the two sets were
equal **at that commit**.

**They were not equal, and the `0.2.0` release pass repaired it.** Phase 5
recounted and left the rows; phase 5's own note said the repair could not wait
past phase 12's audit, and this is that audit.

**Phase 5's arithmetic was wrong in both directions, which is why the heading
said 49 against a specification that said 47.** It read the delta as *phase 4
added CF-39, CF-40, ES-41 and ES-42, and lifted ES-10 — 46 − 1 + 4 = 49*. Counted
clause by clause against §7.2's generated maturity column, the real delta is
**46 − 4 + 5 = 47**: a fifth addition, **PS-38**, minted by ADR-0030 and listed
nowhere here; and three further removals, **PS-18, PS-27 and PS-30**, which were
not lifted at all — they moved to `[DEFERRED]`, where they are now listed, and
where phase 9 had already named them as missing.

Each new row's falsifier is the clause's own marker rather than a judgement made
here, and six of the eight rows that moved name their owner in the same marker:
PS-18, PS-27, PS-30 and PS-38 to `projection-store-freeze` (HS-P0010), ES-41 to
whichever of phases 9 and 10 lands first, CF-39 to phases 8 and 10.

**ES-42 was the eighth row and is no longer in this table**, because it is no
longer provisional. Its marker named a deadline rather than a phase — *"must be
re-evaluated before phase 12"* — which made it, while it was missing from here,
**the one clause in the specification whose own marker gated the release and
which a phase-12 audit reading this table could not have seen.** That is what
the repair was for, and the re-evaluation it forced ran immediately: no consumer
needs `dyn EventStore` the E11 erasure wrapper cannot box, so the marker was
**earned off rather than allowed to expire** and the clause is `[FROZEN]`. The
count above is 46 rather than 47 for that reason.

**Every row names an owner.** The first pass at this repair recorded CF-40 as
owed one, and that was an error in the repair rather than a gap in the
specification: CF-40's clause is written as `**CF-40.**` in §6 rather than under
a `####` heading, so a search that assumed the heading form found the *nearest
marker after a mention of it* — VT-22's, three thousand lines away — and copied
that clause's falsifier and its silence about phases. CF-40's own marker names
phases 9 and 10 explicitly. Corrected here, and worth leaving on the record:
a clause written in a different shape from its neighbours is exactly what a
mechanical read gets wrong, and the anchor discipline the citation checkers
enforce exists for the same reason.

### The 1.0 dispositions

What `1.0.0` does with every clause that is not `[FROZEN]`: one row per
`[PROVISIONAL]` or `[DEFERRED]` clause in §7.2, each with exactly one of three
dispositions — `freeze-by-N`, frozen before 1.0 by phase N; `renew-past-1.0:`
followed by the falsifier the marker is renewed against; or `outside-1.0:`
followed by the reason the clause is on no surface 1.0 promises. The authority is
[ADR-0066](../.kb/decisions/0066-what-1-0-promises.md), and this table, not the
record, is the live copy, because an accepted record's body cannot change and these
rows will move every time a phase freezes a clause. CF-39 is not here: ADR-0066
froze it. Nor is VT-10: ADR-0073 froze it at phase 17. Nor are ES-41 and PS-22:
ADR-0028 froze both at phase 17. Nor are PS-9 and PS-11, which ADR-0074 froze at
phase 17, nor PS-15, PS-23 and PS-24, which ADR-0075 froze at phase 17, nor ES-17, which
ADR-0080 froze at phase 17.

**This is the schedule; the owner columns above are history.** Rule 4 keeps those
rows, and most of them name phases long `done` — they record who owned a
falsifier when the row was written, not who owns it now. Read this table for that.

`cargo xtask lints` holds it (`runbook_clause_ledgers_match_the_specification`): the
set of clauses here must equal §7.2's non-frozen set, each once; every cell must
parse under the grammar above, and the last two kinds must carry their text; and a
`freeze-by-N` must name a phase in the [status table](README.md#status) that phase
21 waits on and that is not `done`. That last condition is the one that fires later
rather than now — a phase that closes without freezing its clause turns the gate red
on this row.

**One sequencing hazard, stated so phase 13 does not discover it.** SY-27 and SY-28
are `freeze-by-13`, and the instrument they need is the same suffix-holding store
CF-27 names — which [phase 14](phases/14-retention.md) builds, and phase 14 runs
after 13. Phase 13 either builds that instrument itself, or a filtered peer that
answers the same question, or these two rows move to 14 in the same commit that
says why.

| Clause | Disposition | Basis |
|---|---|---|
| VT-6 | freeze-by-13 | Sync-testkit's `restored_peer_does_not_reissue_identities` is the instrument for the marker's harm half. Phase 9 answered the eviction half: Durable Object storage outlives the isolate, so mint-once is available |
| VT-9 | freeze-by-13 | A sync-testkit rule that ingest preserves `RecordedAt`, with a mutant. ADR-0066 restates the clock falsifier, which no longer discriminates |
| VT-14 | freeze-by-17b | Moved from 17 by ADR-0072, additive. An RTL identifier corpus check (Arabic, Hebrew, Persian, with mixed LTR) comes back empty, with the E11 reproduction added to `experiments/` |
| VT-21 | freeze-by-13 | SY-18 is where a floor is first compared across a peer set, and phase 13's two real peers (Durable Object, Neon over HTTP) are the tightest targets in the plan |
| VT-22 | renew-past-1.0: a real domain event that legitimately carries more than 64 tags | The marker's own domain falsifier; the richest scenario event carries 8. A firing is answered by a store's own larger documented limit — every shipped adapter accepts 128 or more — not by raising the floor within 1.x |
| VT-23 | renew-past-1.0: a real decision model that legitimately needs more than 128 items | The marker's own domain falsifier; the largest scenario model uses 4. A firing is answered by the store evaluating more, which every chunking adapter already does with no ceiling |
| VT-24 | freeze-by-13 | Sync ingest is the first consumer that batches by the floor (E2E-35), and SY-14 is where a group larger than 128 would surface |
| VT-30 | freeze-by-17b | Moved from 17 by ADR-0072, additive. ADR-0054's alias and builder-state decided in one pass; limb 2 retired by a record or by a multi-guard bench scenario |
| WF-1 | renew-past-1.0: a DCB implementation publishes a wire-level encoding, or a user needs to read another implementation's log | The format is private and WF-8 versions it, so a bridge is additive: a separate `happenstance-dcb-interop` crate with its own ADR. Phase 13 records the renewal in ADR-0026 |
| WF-11 | renew-past-1.0: a workerd-class isolate must forward a payload another store accepted, at or above about 36.6 MB under a 128 MiB cap (peak is payload × 11/3) | `serialize_str` makes a human-readable encoder hold the whole payload. Phase 17's workerd job exists and runs every event-store rule under workerd and on a deployed object. It does not run the WF-11 memory probe, which is still `tests/wf11_memory_ceiling.rs` on the shim. The row wall it measured, 8,388,637 B deployed, bounds what a Durable Object stores, not what a peer must forward. Phase 13 confirms which encoding the sync transport uses |
| ES-7 | freeze-by-17b | Moved from 17 by ADR-0072, additive under the recommended caret answer; an exact pin is taken at 17 instead. Frozen in the record that answers `trait-variant-caret-resolves-past-the-locked-gate`, its falsifier restated to cover a consumer's unlocked resolve. ES-17's ownership, once grouped with it, was answered alone by ADR-0080 |
| ES-32 | renew-past-1.0: the `experiments/polling-cost` harness, re-run over a round-trip adapter with a stated staleness budget, shows 2N idle reads per interval breaking that budget at realistic N | A tail seam would be an added method, so renewing is additive. Phase 18's fan-out runner (PS-30) is the natural producer of the measurement |
| ES-35 | renew-past-1.0: an adapter fixture arms a real fault against a real medium — a process killed mid-commit, a disk lying about fsync — and observes what survives | The marker's own fault falsifier. The reopen end is answered by four adapters. The fault end is unbuilt, and CF-39's armed faults do not build it: the Postgres, Neon and Cloudflare fixtures raise an in-store trigger that aborts a batch before it is acknowledged, while ES-35 is falsified only by a medium that loses a write after `append` returned `Ok`. `happenstance-postgres` killing a backend mid-commit is the cheapest candidate |
| ES-39 | freeze-by-14 | Decided at 17 by ADR-0028: the refusal plus an additive reservation, so no trait change in `0.4.0`. Frozen at 14 against the CF-27 instrument, as the refusal or with a provided method defaulting to `Unknown`. If 14 slips past 1.0 the row may become renew-past-1.0, because firing it is additive |
| ES-40 | freeze-by-14 | ADR-0028 kept the MAY and rejected a third outcome for 1.x, so nothing flips in `0.4.0`. Frozen with its rule at 14 against the CF-27 instrument |
| PS-6 | renew-past-1.0: an adapter that must reserve server state at `begin` and cannot afford the round trip | The original falsifier fired at phase 10b on sqlx's `BEGIN` (ADR-0062) and the port absorbed it. Firing the residual would relax an adapter obligation, not change a signature |
| PS-16 | freeze-by-18 | A typed-runner rebuild through `reset` against a multi-table or graph read model, with the reset-refusal clause composed |
| PS-18 | freeze-by-18 | A refusable reset implemented by at least one adapter, a CF-39-shaped clause and a `NoopProtectFixture` mutant. If 18 does not deliver, renew past 1.0 as additive |
| PS-25 | freeze-by-18 | Built before `unstable-projection` lifts. Remedy chosen at 17 by ADR-0074: the derived id, with a 64-bit FNV-1a digest in hex, an exposed `checkpoint_id` and a documented adoption procedure, because the digest-in-checkpoint alternative changes the frozen port's `commit`; the derived id's constraints (238-byte name cap, separator in no reserved prefix) are ADR-0082 §D5's |
| PS-27 | freeze-by-18 | Phase 18 builds the failure-policy seam and `skip_and_record_is_atomic` with a mutant. ADR-0074 fixed the seam's shape at 17: a provided `on_error` defaulting to halt, handed the failure and the batch; a skip after a server-side failure on a live batch needs a savepoint |
| PS-30 | freeze-by-18 | With `panicking_apply_rolls_back` and a mutant, if 18 builds the fan-out runner. If it does not, this row becomes `outside-1.0`: a conditional MUST on a runner 1.0 does not ship |
| PS-38 | freeze-by-18 | Settled against the fan-out runner. ADR-0075 froze PS-23 and documented the no-lagging-replica obligation at 17, on `ProjectionStore::checkpoint` and on `happenstance-neon`'s README and constructor |
| PS-39 | freeze-by-17b | Green on memory, SQLite and Postgres at 17; freezes once live Neon has run it green (not a required check until L8) |
| SY-7 | freeze-by-13 | ADR-0027. The falsification test is buildable with `MemorySyncPeer` and two adjudicator configurations |
| SY-10 | freeze-by-13 | Phase 13's exit criteria already require both topologies to be expressible. Fallback: renew against the marker's own test, since a firing makes the permission redundant, not wrong |
| SY-14 | freeze-by-13 | Measured against the Neon peer. Phase 17's foreign-identity spike must not foreclose a bounded-round-trip ingest path |
| SY-18 | freeze-by-13 | Phase 13's work list carries a 128 KiB-capped fixture peer for it (`phases/13-sync.md:139-148`). If that peer is not built, this row is renewed past 1.0 against the Turnstile experiment, which is safe because `PeerLimits` is `#[non_exhaustive]` |
| SY-20 | freeze-by-13 | Phase 13 lands the rule; phase 18, which ungates `Projection`, carries the convergence declaration it needs |
| SY-21 | freeze-by-18 | Decided at 17 by ADR-0074 and reworded to the *arrival* position, `SequencedEvent::position`, because `EventId::position()` is the origin's; built and frozen at 18 on `Delivered`, with the sync rule landed at 13 |
| SY-22 | freeze-by-13 | Phase 13 runs the cost-layers test; the declaration's placement is fixed with SY-21 |
| SY-23 | freeze-by-13 | ADR-0027's merge rule. Fallback: renew against the marker's test, since a firing adds a second order and leaves `EventId`'s derived `Ord` alone |
| SY-27 | freeze-by-13 | ADR-0027. **Sequencing hazard** — see above: its instrument is CF-27's suffix store, which phase 14 builds after 13 |
| SY-28 | freeze-by-13 | Immediately after SY-27, and with the same sequencing hazard |
| SY-29 | freeze-by-13 | Jointly with SY-27, because a peer-supplied `Query` exists on the port only if replication is scoped |
| SY-30 | freeze-by-13 | Where the two unlike real peers, Durable Object and Neon, push real envelopes |
| SY-31 | freeze-by-13 | The runner half. The reserved-`ProjectionId` half is settled by ADR-0082: core refuses `sync/` and mints it only through `ProjectionId::sync_watermark` |
| SY-32 | freeze-by-14 | Surface decided at 17 by ADR-0028: the scalar floor means resumability, not completeness. Built and frozen at 14, whose exit criteria require it no longer `[DEFERRED]` |
| CF-14 | renew-past-1.0: phase 19a's REOPEN verdict for memory, IndexedDB or OPFS storage, or a workerd run observing a real Durable Object eviction, cannot be expressed with the one reopen shape | Four adapters express `REOPEN` with one shape. Since phase 17 the three reopen rules also pass under workerd and on a deployed object, but as a fresh handle off the object's state, not an eviction (`evictDurableObject` is reachable only from the runner, between requests). The eviction instrument is unbuilt, and 19a's comes after 1.0. Freeze jointly with CF-17 |
| CF-17 | renew-past-1.0: a durable adapter cannot express even a reopen through this contract — the candidates are phase 19a's browser storage and the first workerd-class run observing a real eviction | Cloudflare's `REOPEN` now runs under workerd as well as the shim (phase 17's job), still as a fresh handle rather than an observed eviction. So the instrument that could fire it is not yet built |
| CF-27 | freeze-by-14 | The report shape was decided at 17 by ADR-0028: instrument-local, over an arbitrary retained set, with a defaulted-declined removal capability. Built and frozen at 14. Phase 13's SY-27 needs the same instrument, so it may have to be built at 13 |
| CF-34 | renew-past-1.0: a complexity property becomes expressible as a deterministic assertion rather than a timing — an instrumented fixture counting rows examined — and the clause splits | Needs an instrumented rows-examined fixture and a record on the scope of CF-33's operation-count ban (`.kb/open-questions/cf-33-cf-34-scope-outside-the-testkit.md`) |
| CF-40 | freeze-by-13 | Phase 17 builds ADR-0043's `MetadataLen`; phase 13 decides whether `payload_len` — data plus metadata — is the budget unit |

### The blocked cases

Twelve case headers in `E2E-CASES.md` carry a blocked marker; the catalogue's
*"What cannot be written yet"* enumerates eleven **decisions**, which is a
different count of a different thing — E2E-05 shares E2E-04's blocker, and several
decisions block cases whose headers are not marked.

| Case | Blocked on | Unblocked by |
|---|---|---|
| E2E-04, E2E-05 | one condition cannot carry per-item boundaries | phase 4 (VT-30) |
| E2E-11 | `ReadOptions` has no upper bound | phase 4 (ES-16, VT-29) |
| E2E-20 | the apply seam's write vocabulary | phase 6 (PS-9 – PS-11) |
| E2E-25 | a chunked rebuild lying about its own completeness | the port half at phase 6 (PS-24); the runner half at phase 7, which is what calls `head` |
| E2E-33, E2E-34 | the shape of `EventId` | phase 4 (VT-4 – VT-8), exercised phase 13 |
| E2E-38 | `after` cannot cross a store boundary, and there is no translation | phase 13 (ADR-0027's ingest-side condition type) |
| E2E-43 | a store-assigned time | phase 4 (VT-9) |
| E2E-46, E2E-48, E2E-49 | deletion, redaction and retained history | phase 14 |

The previous revision's table named **E2E-37** in E2E-38's row. E2E-37's header is
not marked blocked and its own text says the refusal is checkable today
(`E2E-CASES.md:970-973`) — it needs the sync crate to exist, which is a missing
crate rather than a missing decision. E2E-38 is the blocked one, and what blocks
it is that `after` is store-local with no translation and adding a field is
`error[E0599]` (`E2E-CASES.md:988-993`). Getting this wrong pointed phase 5 at a
case it does not unblock and left phase 13's hardest sync case looking settled.
