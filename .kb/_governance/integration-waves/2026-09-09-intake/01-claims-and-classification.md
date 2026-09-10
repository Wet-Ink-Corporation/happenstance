# Wave `2026-09-09-intake` — claims and classification

Every claim the five staged files carry, labelled against the **accepted decision corpus** — the
fifty-six `kind: decision, status: accepted` atoms in `.kb/decisions/`, read rather than
inferred from titles.

The four labels:

- **aligns** — the claim is already true under an accepted decision, and adds no obligation.
- **extends** — the claim adds evidence, scope or consequence to something accepted, without
  contradicting it.
- **conflicts** — the claim contradicts an accepted decision or the tree.
- **requires-new-decision** — the claim is a commitment nothing accepted covers.

**The wave's shape in one line:** three `requires-new-decision`, **zero** `conflicts`, and
everything else `extends`. Zero conflicts is not luck. Three of the five files are written by
authors who had already read the accepted atom they touch — ADR-0060 devotes a section to why it
is *not* superseding ADR-0036, and the Ladybug brief spends two paragraphs refusing to claim
`kb-decision-0017`'s falsifier. The wave's contestable calls are all about **placement**, not
about authority.

---

## A. ADR-0025 — the Ladybug projection adapter (1 file)

`adr-0025` = `2026-09-08-adr-0025-ladybug-projection-adapter.md`

| # | Claim | Label | Against |
| --- | --- | --- | --- |
| A1 | The five-part decision bundle: `__hs_checkpoint` node over `UINT64` written last before `COMMIT`; raw parameterised Cypher with a `raw()` hatch; blocking-only at `0.2.0`; `Arc<Database>` with a second `Connection` per handle; `commit` issues no `ROLLBACK` after a STATEMENT error and a best-effort one after a decode failure; `lbug` behind an off-by-default feature | **requires-new-decision** | rests on `kb-decision-0017` (owned batch) and `kb-decision-0030` (PS-38's progress obligation); neither states any of this |
| A2 | `UINT64` round-trips `u64::MAX - 1` exactly, so `PositionOutOfRange` is unreachable and `MalformedCheckpoint` narrows to a stored zero | extends | body of A1; a consequence of `kb-decision-0013`'s `NonZeroU64` position type |
| A3 | The module doc's stated reason for blocking-only — that `spawn_blocking` is available because the store is `'static` — is **wrong**; availability is a decision about the store's *fields*, not its call sites | extends | body of A1. A correction to crate prose, not to any decision |
| A4 | The `ROLLBACK` qualifier is load-bearing and §7's original wording lacked it: after a *decode* failure the statement succeeded, the engine aborted nothing, and a bare `?` returns with the transaction open | extends | body of A1, and the sharpest sentence in the brief — the unqualified form reads as licence to `?` out of an open transaction |
| A5 | PS-4's **Cypher-level** falsifier did not fire: a transaction gives read-your-own-writes within itself, so a deferred write set answers it — a fact about this adapter on this engine, not a discharge of a clause generalising over write-behind shapes | extends | PS-4 is `[PROVISIONAL]` and owned by the Ladybug phase. This is its owner reporting |
| A6 | PS-4's **Rust-level** falsifier is foreclosed by the port for *every* batch shape, because `Projection::apply` is synchronous and a traversal is I/O — equally true of SQLite and Postgres | extends | genuinely general. Same shape as `kb-open-question-provisional-falsifiers-001`, and sharper than either instance it holds |
| A7 | Ladybug is **not** one of PS-9/PS-11's data points — that falsifier names a second *generic consumer*, owned by a different phase. An adapter is evidence about a clause's **cost** | **aligns** | `kb-decision-0017` states this falsifier verbatim; the claim applies it and asks for no change to it |
| A8 | The crate root has **already been corrected** to say A7 in the past tense (`lib.rs`:151); nothing is owed | extends | verified landed in this worktree |
| A9 | A pre-registered verdict: four capability predictions (`SECOND_HANDLE` supported, `RESET_REFUSAL` declined, `COMMIT_FAULT` supported, `READS_THROUGH_BATCH` false) and four named "it did not hold" conditions, each citing its clause, committed before any body | extends | bears on `kb-open-question-reset-refusal-declension-001`, whose stated population was empty |
| A10 | Ladybug is the **fifth** owned-buffered-batch implementer, not PS-2's second shape; what phase 11 fills is the **write-vocabulary** axis, not the batch-shape axis. `RUNBOOK.md`:4917 has already been corrected and says what it used to say | extends | agrees with, and is cited by, the ADR-0060 pair below |
| A11 | The measurement that forces `Arc<Database>`: a second `Database::new` on one directory is refused by a file lock, so `SECOND_HANDLE` is a second `Connection` | extends | evidence. Belongs in a `reference` atom by `decisions/README.md`'s separation rule |

## B. PS-2's axis — the finding and the decision (2 files)

`ps-2` = `2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md` ·
`adr-0060` = `2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md`

| # | Claim | Source | Label | Against |
| --- | --- | --- | --- | --- |
| B1 | PS-2's live-transaction end is not unbuilt but **forbidden by the port's own signatures**, and the two named drivers are forbidden by two *independent* mechanisms — so finding one did not predict the other | `ps-2` | extends | `kb-open-question-probe-read-through-signature-001` attributes the same gap to **scarcity**; this replaces the attribution and keeps the gap |
| B2 | `sqlx`: `begin` is total, synchronous and infallible; `Transaction::begin` is async and fallible with private fields; `Pool::try_acquire` yields a `PoolConnection` and `BEGIN` is still a round trip. There is no total synchronous expression of `Transaction<'static, Postgres>` | `ps-2` | extends | body of B1 |
| B3 | `probe_write` and `probe_delete_all` close the second door independently — both synchronous and infallible, so even *given* a live transaction there is no seam to issue a statement into it | `ps-2`, `adr-0060` | extends | widens the existing open question's scope from one method to the whole probe seam |
| B4 | `rusqlite`: `Connection` is `!Sync`, `Transaction<'_>` is `!Send`, so binding a live handle costs the `SendProjectionStore` impl — already recorded in `happenstance-sqlite`'s own module doc | `ps-2` | **aligns** | `kb-decision-0017` rests on exactly this; the sub-claim restates a documented fact |
| B5 | No compiler said so for a phase because `todo!()` has type `!` and `!` coerces to everything: the associated type was real *and uninhabitable*, and a skeleton cannot tell those apart | `ps-2` | extends | genuinely new, and the sharpest sentence in the file |
| B6 | RS-90-1's exemplar of a good skeleton was this exact binding, and it was refuted — which is what makes RS-90-1 and RS-90-2 two rules rather than one restated | `ps-2` | extends | **already landed** at `standards/rust/90-skeletons-and-todo.md`:82, outside `.kb` |
| B7 | One end of the axis **is** occupied: `happenstance-neon` is a cannot-hold-across-await adapter and it passes the projection suite. The adapter population went from one to four, and phase 11's pre-registered condition did not fire | `adr-0060` | extends | partly refutes `kb-decision-0036`'s "part 2 unmet" finding *by evidence*, which the governance atom's currency computation permits without an edit |
| B8 | A conformant live-transaction adapter must declare `READS_THROUGH_BATCH = false` — a false statement about itself — so it reports the same capability profile as a buffering one and **the suite cannot tell the ends apart** | `adr-0060` | extends | `kb-open-question-probe-read-through-signature-001` already carries the compiled table; this states the consequence for PS-2's *bar* |
| B9 | **The port keeps its gate at `0.2.0`, and the reason is new.** ADR-0036's scarcity reason has expired; the gate stays because freezing `begin`/`probe_write`/`probe_read_through` would make a semver promise out of the very signatures that forbid the second shape. PS-3 keeps `[PROVISIONAL]` | `adr-0060` | **requires-new-decision** | reaffirms `kb-decision-0036`'s decision and replaces its reason — an amendment from outside, never an edit |
| B10 | A signature change is proposed and **deliberately not made**: `probe_read_through` as `&mut Self::Batch`, async, returning `Result`. Breaking, not this lane's call, and **not sufficient alone** | `adr-0060`, `ps-2` | extends | the existing open question already recommends the same fix for one method; "not sufficient alone" is what is new |
| B11 | ADR-0036 is **not superseded**: its decision stands, only its reason is replaced, and the two are recorded separately because *"only one adapter has run the suite"* was true when written | `adr-0060` | extends | a fresh worked instance for `kb-governance-referent-not-reasoning-001` |
| B12 | Falsifier: reopened by a live-transaction adapter that passes **and** can truthfully declare `READS_THROUGH_BATCH = true`, or by a fifth adapter finding a field the port cannot express. **Not** reopened by another buffered-end adapter passing | `adr-0060` | extends | body of B9, and it is what makes B9 indifferent to the thing that expired B9's predecessor |
| B13 | Thirteen `[PROVISIONAL]` clauses are gated on PS-2 alone, and **none of them moves** — the finding is about how the gate opens, not whether it opens | `ps-2` | extends | a statement of absence; body of B9 and evidence for the provisional-falsifiers merge |
| B14 | Three remedies are named for PS-2's owner: reword and narrow the population; move the port to `async fn begin`; or record the falsifier as unfalsifiable in its stated terms | `ps-2` | extends | the decision space ADR-0060 resolves. Not a separate commitment |

## C. ES-11's falsifier — the finding and the decision (2 files)

`es-11` = `2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md` ·
`adr-0061` = `2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md`

| # | Claim | Source | Label | Against |
| --- | --- | --- | --- | --- |
| C1 | ES-11's `[PROVISIONAL]` marker named the adapter that would falsify it — *"the first one-shot-HTTP adapter that cannot meet this in one round trip"* — and `happenstance-neon` is that adapter | `es-11` | extends | the predicted event, arriving. Bears on `kb-open-question-es-11-sqlite-ceiling-sample-cost-001`'s sub-question 3 |
| C2 | ES-11 contains a sentence that is **false as written**: *"a read spawned at its first poll and an append spawned afterwards land in the same queue in that order."* It is a fact about *pooled* drivers presented as a fact about async ones | `es-11`, `adr-0061` | extends | ES-11 is `[PROVISIONAL]`; no accepted decision states the sufficiency condition |
| C3 | The failure is not the one the clause anticipated: the read does **not** self-paginate — it buffers a whole result set in one round trip, as ES-12 asks. It fails because a read and an append are two independent HTTPS requests to a pooled proxy and nothing orders one backend's snapshot against another's commit | `es-11`, `adr-0061` | extends | body of C2 |
| C4 | Both transport configurations redden intermittently; HTTP/2 over one multiplexed connection reddens markedly less than HTTP/1.1 over a default pool. **No rate is stated and the atom must not add one** — the direction of failure is what carries the finding | `es-11`, `adr-0061` | extends | an explicit authoring constraint. `reference/README.md`'s dating rule is why it binds |
| C5 | The claim was checked against a **retracted** predecessor: `HANDOVER.md` records ES-11 escalated in error and retracted (`2e0a0ae`), on the claim that *no async driver can conform*. The present claim — *a driver with no shared ordering primitive between its operations cannot* — is narrower, and the thing that refuted the old one is present here and does not help | `es-11`, `adr-0061` | extends | a fresh instance for `kb-governance-what-may-refute-a-finding-001`'s fourth consequence |
| C6 | **The decision, four parts:** amend the sufficiency condition to *spawned at the first poll **and** ordered against a later append by something the store itself honours*; record that `happenstance-neon` does not satisfy ES-11 as a stated limitation; keep `live-neon` strict; keep ES-11 `[PROVISIONAL]` with its marker rewritten to record a fired falsifier | `adr-0061` | **requires-new-decision** | ES-11's MUST, maturity, `Rule` and `Cases` are untouched; ES-12's normative content is untouched |
| C7 | The amendment is a **narrowing** — it removes a route to a conformance claim rather than admitting a shape the MUST would reject. Nothing an adapter must do gets easier | `adr-0061` | extends | body of C6, and the case that sharpens `kb-playbook-repair-frozen-clause-001`'s mechanical test |
| C8 | Two ways out refused: putting one-shot HTTP outside ES-11's scope is the weakening the clause's own text warns against (ES-11/ES-12 reduce to ES-10 plus a ceiling); minting a `Capability` misuses it, since it says what a fixture can **arm** and not whether a store provides a guarantee | `adr-0061`, `es-11` | extends | body of C6. `kb-decision-0051` is the precedent for the second |
| C9 | ES-12 got the same falsifier and **survived it, with a soft edge**: one statement per `read` whatever the item count makes its predicted defect unreachable, but the rule is structurally exposed to the same race and sixty runs separate luck from immunity poorly. The honest figure is **104 of 105 observed**, with a 105th exposed and not yet having lost | `adr-0061` | extends | body of C6 |
| C10 | **The cost of strictness is zero today**, which the finding did not know: `NEON_CONNECTION` is not a repository secret, every `live-neon` step is gated on it, and the job prints its own *"proves NOTHING"* notice and stops. Consumers pay nothing either — gated tests carry `#[ignore]` naming the requirement | `adr-0061` | extends | **retracts** `es-11`'s own *"red about 1 run in 40"*, in `es-11`'s own header callout |
| C11 | Four passages the falsifier's arrival made false were repaired in the same change — two `SPECIFICATION.md` §6.5 rows, a §3 cell, and ES-11's own `Rejects:` bullet ending *"It is conformant today"*, written 2026-08-06 and untouched through two phases. **`spec-trace` checks that citations resolve, not that prose is current** | `adr-0061` | extends | a third shape under `kb-playbook-count-or-index-nobody-re-derives-001`'s root |
| C12 | What is open is no longer whether this shape fails — it does — but **whether a conformant one-shot-HTTP shape exists at all** | `adr-0061` | extends | genuinely new. No atom owns it |
| C13 | The `es-11` file's header callout, added after its own resolution: *"Settled on 2026-09-08 by ADR-0061… This file stays as the finding, unedited below this line… Two claims below did not survive the pass"* | `es-11` | extends | the file's own instruction to this wave: it is the argument, not a second decision |

## D. Claims that earn no operation

Recorded so the next wave does not re-derive them.

| Claim | Source | Why no operation |
| --- | --- | --- |
| RS-90-1's exemplar was refuted (B6) | `ps-2` | Outside `.kb`, and **already landed** at `standards/rust/90-skeletons-and-todo.md`:82, verbatim. Evidence and provenance only |
| The crate-root and RUNBOOK corrections (A8, A10) | `adr-0025` | Both landed in this worktree, in the past tense, naming what they used to say. An atom asking for either would be a task, and `open-questions/README.md` puts tasks in `.bklg/` |
| `rusqlite`'s `!Send` transaction (B4) | `ps-2` | Restates a fact `kb-decision-0017` already rests on and `happenstance-sqlite`'s module doc already records. The file says so in its own voice |
| Thirteen `[PROVISIONAL]` clauses, none moving (B13) | `ps-2` | A statement that nothing changed. It lands inside ADR-0060's body and as one sentence of the provisional-falsifiers merge; an atom for an absence of consequence is a worry, not knowledge |
| The three-remedy menu (B14) | `ps-2` | The decision space ADR-0060 resolves. Recording it separately would file a live question over a settled one |
| A failure rate for ES-11 (C4) | `es-11`, `adr-0061` | Both files forbid it, and there is no committed raw output to cite. A dated reference atom with no measurement behind it is the *false reference* `reference/README.md` names |
| PS-2's compiled refutations as a reference atom (B2, B3) | `ps-2` | The existing open question already holds the compiled table for this seam. Two homes for one argument is the failure mode `reference/README.md` opens with |
| `es-11`'s *"1 run in 40"* (C10's referent) | `es-11` | Retracted by its own header callout before this wave read it. The corrected figure — zero — is what ADR-0061's atom carries |
