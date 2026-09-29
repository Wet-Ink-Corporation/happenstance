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

**The next free number is 0066.** `ls .kb/decisions/` is the answer that cannot go
stale, and `references/adr/` holds the long-form records; not every atom has one.
Numbers 0026–0028 were reserved by the original queue for phases 13 and 14 and are
still unwritten, which is why they are out of order with the numbers around them.

| ADR | Phase | The question it answers |
|---|---|---|
| **0026** | 13 (published-surface half: 17) | What is a sync *peer* — what may the port assume about a transport it cannot see, and what does ingest promise? (SY-8 – SY-18). Phase 17 must settle the part of this that reaches a published crate: whether `happenstance-core` needs a write path that preserves a foreign `EventId` for `IngestStore` to be implementable (VT-10; `crates/happenstance-sync/src/ingest.rs` holds four `todo!()` bodies blocked on exactly that) |
| **0027** | 13 | How do two logs reconcile — the merge rule, the compensation contract, and whether hub-and-spoke and peer-to-peer are one abstraction or two? (SY-1 – SY-7, SY-19 – SY-31). SY-1 – SY-7 already settle *ingest is unconditional, with compensation*; this ADR records it and settles the rest |
| **0028** | 14 (decision: 17) | What is a store permitted to forget, and how does it say so? (ES-39, CF-27, SY-32). **The decision moves to phase 17** because one of its two answers adds a method to `EventStore`, which breaks every published adapter, and that is only cheap before 1.0. Phase 14 builds what it decides |
| *unnumbered* | 16 | What does 1.0 promise? The charter phase 16 owes: which crates, what happens to every non-`[FROZEN]` clause on a published surface, adapter and testkit versioning (CF-32), and the MSRV policy after ADR-0037 |
| *unnumbered* | 17 | Is `Projection::apply` synchronous, and may a projection issue a statement into a live batch? (`.kb/open-questions/projection-apply-is-synchronous-against-a-live-store.md`). Owed its own record by the note that froze the port (ADR-0063) and owned by nobody until now |
| *unnumbered* | 19b | Does the SQLite schema and append-condition SQL get a home that is not an adapter, once a third copy exists? (`references/seeds/sqlite-on-wasm.md`) |

## Open decisions

The rows of the decision ledger (`RUNBOOK.md:505-619` at `f89e184`) that were still
open when the runbook was split, and the owner each now has. A row leaves this
table when an ADR or a clause settles it, and the archive keeps it either way.

| Decision | Phase | Status |
|---|---|---|
| `conflicting_position`: a promise every adapter owes, or a hint one may omit | 16 | open — Neon-over-HTTP was the forcing case and now exists; phase 16's clause audit reads it |
| Is ES-10's global visibility statement what happenstance needs, or would a per-boundary one do | 16 | open — raised by the phase-2 measurement; ADR-0063's single-position `Checkpoint` may have answered it, and phase 16 decides whether it did |
| `append`'s ownership of its batch — two builds of the same adapter differing only in ownership (ADR-0012's falsifier item 1, escalated by ADR-0022 §13) | 17 | open — ES-17 stays `[PROVISIONAL]` until measured or renewed; ADR-0055 kept the borrowed batch at `0.2.0` |
| ADR-0022's falsifiers have fired: supersede, re-open (scoped) or ratify its pragma and runtime-seam sections | 16 | open — forced by phase 12, now past; given an owner by phase 15 (`adr-0022-falsifiers-have-fired`) |
| Whether the 2026-09-06 `msrv-premise` ratification supersedes ADR-0037, and what the floor is | 16 | open — "`0.2.0` is the only cheap moment" has passed; goes with phase 16's MSRV item (`msrv-ratification-conflicts-with-the-accepted-floor`) |
| Whether a workerd-class runner enters `cargo xtask ci`, and which platform clauses stay unproven without one | 16 | open — phase 12 made "conformant on Cloudflare" a promise (`no-workerd-class-runner-in-the-gate`) |
| Who owns reconciling the specification after each phase, and whether `spec-trace` mechanises it | 16 | open — forced by phase 6 and phase 12, both past (`nothing-owns-the-post-phase-reconciliation`) |
| Which read fault `PostgresFixture` arms, and whether it needs new capability machinery | 16 | open — owed at phase 10's remainder (`postgres-fixture-read-fault-declension-is-owed`) |
| An infallible pre-validated `Tags` / `Tag` constructor in `happenstance-core` | 16 | open — additive; an ADR owed by ADR-0020, forced by the first API change after `0.1` (`d-1-the-validated-type-has-no-total-path`) |
| A named `Chunk` type and an observation seam for `run_projection` | 16 | open — due at the `ProjectionStore` freeze, now past; free while `happenstance`'s `unstable-projection` gates the runner, so decided before phase 18 lifts it (`projection-runner-chunk-type-and-observation-seam`) |
| Whether `remint_identity`'s documented procedure owes an in-process check | 16 | open — first forced at phase 5, now past (`remint-identity-precondition-is-trust-only`) |
| Is replication whole-log or scoped | 13 | deferred — SY-27, SY-28 |
| Idempotent bulk ingest inside one round trip | 13 | deferred — SY-14 |
| Does a peer declare its own limits | 13 | deferred — SY-18 |
| Where a per-peer watermark lives transactionally | 13 | provisional — SY-31 |
| What is a store permitted to forget, and how does it say so | 17 → 14 | deferred — ES-39, CF-27, SY-32 |
| A projection's `Query` changed under its checkpoint | 18 | provisional — PS-25 |
| Snapshotting decision-model state | after 1.0 | deferred — DCB queries are narrow by construction; revisit if replay cost is measured |
| `tracing` spans and metrics | after 1.0 | deferred — purely additive, no port change |

## Where the open clauses and the blocked cases land

Three obligations, all mechanical to check, all previously answerable only by
prose search — and the middle one previously not answerable at all, which is why
phase 12's audit of it had nothing behind it.

### The 12 `[DEFERRED]` clauses

Seventeen rows are listed; four are struck. Two were settled at phase 2 and their
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

### The 41 `[PROVISIONAL]` clauses

Phase 12 cannot audit "every provisional clause has its falsifier scheduled"
against prose. Grouped by what falsifies them, because they do not fail
independently — most of the `PS` rows waited on one missing adapter until
ADR-0062 built it and ADR-0063 froze what it proved, and counting them as
separate open questions overstated the exposure by that factor.

| Group | Clauses | Falsified by | Owning phase |
|---|---|---|---|
| Identity's queryability and store-assigned time | VT-6, VT-9 | a peer that must dedupe without parsing `metadata`, and a rule that `recorded_at` is non-decreasing with position | 5, exercised 13 |
| The ingest seam's placement | VT-10 | a real peer needing a foreign identity through `EventStore::append` after all | 13 |
| Const-constructible identifiers | VT-14 | `from_static` failing to move the `?` count in the worked example | 5 |
| Store limits | VT-21 – VT-24 | a real adapter that cannot honour a stated minimum | 5, tested 8 and 10 |
| Per-item boundaries on a condition | VT-30 | E2E-04 and E2E-05 still unwritable after phase 4 | 4 |
| `Bytes`' human-readable form | WF-11 | a peer that cannot buffer a payload through any human-readable encoder | 9 |
| The `!Send` flavour and `append` ownership | ES-7, ES-17 | the Cloudflare adapter, and `dynosaur` failing to erase a generic `append` | 1 and 4, confirmed 9 |
| No tail seam at 0.1 | ES-32 | a Durable Object making one cheap enough to reopen | 9 (verdict), post-0.1 — **answered**; the one-paragraph verdict is in phase 9's session log. It answers *this* falsifier only: the clause's own benchmark-shaped one — E2E-32's fan-out runner holding N views within their staleness budget — is untouched and stays phase 7's |
| The write seam's consumers, the foreign-batch hazard, and `begin`'s round trip | PS-6, PS-9, PS-11, PS-15 | each its own, since ADR-0063 froze PS-4, PS-5 and PS-12 on both ends of the axis passing: a second generic consumer (PS-9, PS-11); a zero-cost instance-naming construction (PS-15); an adapter that must reserve something from its server and cannot afford the round trip (PS-6) | 6, re-tested 11, narrowed 12 |
| Reset, checkpoint regression, chunked rebuild, query drift | PS-16, PS-22 – PS-25 | a rebuild that skips event 1, or a projection that cannot refuse a reset | 6 |
| Compensation's shape and the merge rule's details | SY-7, SY-10, SY-20 – SY-23, SY-29, SY-30 | two unlike peers that cannot both express it | 13 |
| Where a per-peer watermark lives | SY-31 | a peer with no transaction to put it in | 13 |
| Membership as a port operation | ES-41 | an adapter that cannot answer membership without a structure VT-8 does not already oblige. Phase 8 answered the **in-process, connection-holding** half — `happenstance-sqlite` reads the `UNIQUE (origin_store, origin_position)` pair migration 1 already creates. The **transport** half is open: a store with no connection, no interactive transaction and no cursor, for which the probe is a whole extra round trip | 9 or 10, whichever adapter lands first. Completeness has an instrument at neither end |
| Checkpoint visibility after commit | PS-38 | a store answering `checkpoint` from a replica that may lag its own `commit` — the shape a projection store over an eventually-consistent read model has. If real, the obligation narrows to "a subsequent read through the same handle" and every rule downstream gains a handle constraint | 7 — the first projection adapter over storage this workspace does not control |
| A fixture's fault-injection promise | CF-39 | a real adapter whose only injectable mid-batch fault is one its driver transparently absorbs — a connection killed mid-statement behind a reconnect-and-retry pool — which would make "the append returns `Err`" a promise no fixture over that adapter can keep | 8 and 10. **No adapter has armed a fault yet** |
| A fixture's stated capacity ceilings | CF-40 | a real adapter whose ceiling is **not a constant** — a Postgres row whose TOAST threshold moves with the rest of the row, or a KV store whose per-value cap moves with the key — for which a single `Option<usize>` cannot say where the boundary is, and the rule built on it would assert a number the store cannot honour | 9 and 10, **whichever states a varying ceiling first**. Phase 9 has landed and answered the *constant-ceiling* half — `CloudflareFixture` states all three as `Some(…)`, as `SqliteFixture` does — so it added a second constant-ceiling store and left the live half untouched. That half is **phase 10's**: Postgres is where a ceiling that moves with the row first appears |
| Durability's rule shape; benchmarks are not conformance | CF-17, CF-34 | a store that loses an acknowledged write, and an adapter that scans where it should seek and passes every rule | 8 |
| **The portfolio's residual exposure** — the clauses §1.3 names as carrying CF-25's risk in their own markers rather than in a preamble. **Four, not five**: ES-10 was lifted at phase 4 and `[FROZEN]` since, and carrying it here is what made this table's count disagree with the specification's | ES-11, ES-12, ES-35, ES-40 | the far-end **adapter** on each axis, and nothing short of it: transport (ES-11, ES-12) by a one-shot-HTTP store; durability (ES-35) by a store that can lose a write to a fault; completeness (ES-40) by a store holding a suffix. A **fixture** instrument does not falsify any of them — CF-26 says so in terms. **ES-11's falsifier has now FIRED, on the adapter its own marker named** (2026-09-08): `happenstance-neon` runs the suite and `read_result_is_stable_under_concurrent_append` fails intermittently — less often over a single HTTP/2 connection than over HTTP/1.1, always in the same direction, and no committed log backs a rate, so none is quoted. Not for the reason the marker anticipated: the read does **not** self-paginate, so ES-12 holds by construction. It fails because a read and an append are two independent requests to a pooled proxy, so ES-11's own sufficiency condition for asynchronous drivers — *"a read spawned at its first poll and an append spawned afterwards land in the same queue in that order"* — is false where there is no shared queue. One-shot HTTP is a third shape and the clause has two. **Owed an ADR**, staged at `.kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md`, and not amended here: check it against the escalation `HANDOVER.md` records as made in error and retracted, which claimed something different and weaker | 10 (ES-11, ES-12), 8 (ES-35), 14 (ES-40) |

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
