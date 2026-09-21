# Wave `2026-09-21-intake` — claims and classification

Every claim the one staged file carries, labelled against the **accepted decision corpus** — the
sixty-two `kind: decision, status: accepted` atoms in `.kb/decisions/`, read rather than inferred
from titles.

The four labels:

- **aligns** — the claim is already true under an accepted decision, and adds no obligation.
- **extends** — the claim adds evidence, scope or consequence to something accepted, without
  contradicting it.
- **conflicts** — the claim contradicts an accepted decision or the tree.
- **requires-new-decision** — the claim is a commitment nothing accepted covers.

**The wave's shape in one line:** one `requires-new-decision`, **one `conflicts` — with an
accepted decision, for the first time in seven waves** — and the rest `extends`. The conflict is
the wave: `kb-decision-0022`'s §11 asserts that five seconds *"did real work and never ran out"*,
and the tree now carries a measurement in which five seconds runs out in seven launches of eight.
That is a decision refuted on its own stated terms, by its own stated trigger, and it is the case
every previous wave managed to avoid.

File abbreviation, used throughout:

`§11` = `.kb/_intake/2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md`

---

## A. The busy timeout is fifteen seconds (claim-1-decision-busy-timeout-15s)

| # | Claim | Label | Against |
| --- | --- | --- | --- |
| A1 | `BUSY_TIMEOUT_MS` moves from `5_000` to `15_000` in `crates/happenstance-sqlite/src/connection.rs` | **requires-new-decision** | `kb-decision-0022`'s summary fixes *"three pragma values … busy_timeout 5,000 ms"* as a documented adapter property. Nothing accepted permits the move; it is a commitment, `must`-shaped, and `decisions/README.md` says a commitment is a decision atom or it is not a commitment |
| A2 | §11's premise — *"`busy = 0` in every row of the 64-contender table, so the timeout … never ran out"* — is false | **conflicts** | `kb-decision-0022`, directly. `kb-reference-busy-timeout-margin-001` recorded `busy > 0` at the same contender count on 2026-09-03, and `kb-open-question-adr-0022-falsifiers-fired-001` has carried the conflict, unresolved, since 2026-09-04. The conflict is with the *premise*, not with the margin figure, which is unaffected |
| A3 | Measured on **this adapter** rather than the experiment's candidate: 5,000 → 7 red of 8; 15,000 → 0 of 16; 30,000 → 0 of 8, at `--test-threads=1` | extends | New evidence, in no atom. It closes caveat 1 of `experiments/busy-timeout-margin/README.md`, which `kb-reference-busy-timeout-margin-001` carries unclosed. Routed to its own reference atom — `02`, Adjudication 2 |
| A4 | Raising the cap is free on the healthy path: the handler returns the instant the lock is acquired; passing runs 4.91–5.65 s at 5 s vs 4.99–5.40 s at 15 s | extends | The argument that decides A1 over ratification. Nothing accepted contradicts it, and nothing accepted contains it |
| A5 | 30,000 ms rejected: no measured gain, triples the pathological wait; the core sweep's ~450x fewer-cores-is-better result means a smaller runner sits in the safer regime | extends | `kb-reference-busy-timeout-margin-001` carries the 450x result as one of its *"two refuted predictions."* Using it to bound a *different* decision is new, and correct: it is the only thing making 15,000 defensible off this one host |
| A6 | Ratifying §11 as still correct was argued and rejected: the semantic property never broke (`committed` correct in every row), so what fired was liveness — but *"the cost is paid by the wrong person"* | extends | `kb-open-question-adr-0022-falsifiers-fired-001` names ratification as the third of three moves and calls it *"defensible, since section 11 fired with a broken premise rather than a broken margin."* The intake takes the fork and says why. A rejected alternative, so it travels with the decision |
| A7 | An unbounded busy handler stays rejected: CF-33 is `[FROZEN]` and forbids the suite a watchdog, so this constant is the system's only liveness bound | **aligns** | CF-33 (`spec/SPECIFICATION.md:8720`), quoted verbatim in `kb-open-question-testkit-contention-tolerance-001`. ADR-0022's original reason, unchanged. Restated rather than assumed — which is what the intake asks for, and what stops the new atom silently inheriting an argument from a record it only partly supersedes |
| A8 | The replacement falsifier names its own instrument: re-open if the adapter's concurrency target goes red on `SQLITE_BUSY` at 15,000 on any host or configuration, reproducing with the sweep's loop; re-open *downward* if the 15 s wait ever makes a real failure undiagnosable | extends | No accepted decision governs falsifier *form*. `kb-open-question-adr-0022-falsifiers-fired-001` establishes the defect empirically — §11's falsifier was *"unfalsifiable for as long as it existed"* and §16's *"cannot fire as written"* — so naming the instrument is the corpus's own lesson applied, two-thirds of the way to a governance rule and not yet one |
| A9 | The supersession is **scoped to §11**; §4, §6, §7, §9, §10 and §15 stand unchanged | extends | `kb-decision-0007` and `kb-decision-0031` are the precedent — both say *"partly supersedes"* in prose and carry `supersedes: null`. The intake's request for `supersedes: [kb-decision-0022]` is the one place this wave declines a staged `should`; `02`, Adjudication 1 |
| A10 | `CONTENDERS` is untouched — §12 says the contender count is *"not this record's to re-open"*, so lowering it to 8 was never on the table | **aligns** | `kb-decision-0022` §12 verbatim, and `kb-open-question-testkit-contention-tolerance-001` re-states it. Recorded as scope, not as a decision |

**Net:** one `requires-new-decision` (A1), one `conflicts` (A2), two `aligns` (A7, A10), six
`extends`. The conflict is with an **immutable** atom, which is what forces `create_new` +
partial supersession rather than any merge.

## B. Amend the falsifiers-fired open question (claim-2-amend-falsifiers-fired-open-question)

| # | Claim | Label | Against |
| --- | --- | --- | --- |
| B1 | `kb-decision-0022` is accepted and immutable, so a fired falsifier cannot edit its body | **aligns** | KB authority rule 1; `decisions/README.md`; and the target atom's own *"What is not decided"* paragraph, which makes the same argument at greater length |
| B2 | `kb-open-question-adr-0022-falsifiers-fired-001` is **amended, not closed**: §11 resolved by supersession, §9 and §8/§16 still open | extends | The atom asks *"which of three moves resolves it … and who takes that call."* One third of it now has an answer and an owner. `open-questions/README.md` reserves `withdrawn`/`superseded` for a question that is *answered*; two thirds of this one is not, so `status` does not move |
| B3 | §9 (captured tokio `Handle` → `SqliteEventStoreError::NoRuntime` unreachable) stays open, still owed its twenty-line reproduction, because the falsifier's wording is *"a deployment shows"* and no deployment in this tree shows it | **aligns** | The atom's own §9 paragraph, verbatim in substance: *"Until it exists this firing is a hazard rather than a fact."* The intake re-states rather than moves it |
| B4 | §8/§16 (append-condition SQL shape, falsifier unfireable as written) stays open | **aligns** | The atom's §16 paragraph. Untouched by this wave, and the one place where the repair *would* be a `[FROZEN]` clause edit (ES-27's `Rejects:` prose) — which is why it is not attempted here |

**Net:** three `aligns`, one `extends`. No conflict, and no `status` change. The amendment is
additive and dated.

## C. The contention instrument exists (claim-3-amend-testkit-contention-open-question)

| # | Claim | Label | Against |
| --- | --- | --- | --- |
| C1 | `happenstance_testkit::FaultyStore::contend_next` refuses the next *n* appends as `AppendError::Store(FaultyStoreError::Contended)`, landed 2026-09-21 | extends | Verified at `faulty.rs:333`, `:429`, `:556`. `kb-open-question-testkit-contention-tolerance-001` closes on *"Both live arms are gated by the same missing instrument"* and describes this decorator almost exactly. The gate is now open |
| C2 | `tests/contended_store_instruments.rs` shows both affected rules reject a merely-contended store, each on a different assertion — supporting the atom's *"a tolerance would change what those rules assert, not add a private-enum arm"* | extends | `two_rules_reject_a_store_that_is_merely_contended` (`:346`) and `the_same_fixture_passes_the_rule_when_it_is_not_contended` (`:390`) are the assertion and its negative control. The atom's *"three rules, not one classification arm"* claim was reasoned; it is now executed |
| C3 | The underlying question stays open: fixture-side, contract-side, or no tolerance. Raising the cap narrows the conflation's **rate**, not the conflation — CF-33 denies the clock | **aligns** | The atom's whole thesis, and the sweep page's third *"What this does not show"* says it independently. `status` unchanged |
| C4 | **Not in the digest, found in the tree:** `a_retry_loop_gated_on_the_dcb_signal_never_retries_a_busy_store` (`:290`) shows the port offers a caller exactly one classifier, `AppendError::is_condition_violated`, which answers `false` for a busy store exactly as for a broken one — so the **per-error arm cannot be built in the testkit at all** without a contract change, and the per-fixture arm can | extends | This is strictly stronger than C2 and it is the half that matters. The atom says the instrument *"discriminates the two arms as well as gating them"* — a prediction. The file's own module doc states the result: *"That asymmetry is a fact about the two designs, not a preference between them."* An atom that recorded only C1 and C2 would leave the prediction unresolved while the tree had resolved it |

**Net:** three `extends`, one `aligns`. The merge carries C4 as its centre of gravity, not C1.

## D. The adapter cap sweep (claim-4-reference-adapter-cap-sweep)

| # | Claim | Label | Against |
| --- | --- | --- | --- |
| D1 | `experiments/busy-timeout-margin/results/adapter-cap-sweep.md` measured `BUSY_TIMEOUT_MS` on the **real adapter**, at `--test-threads=1` with all 64 contenders on one machine: 5,000/8/7 red; 15,000/16/0; 30,000/8/0 | extends | New. Distinct instrument from `kb-reference-busy-timeout-margin-001` — one bit per launch instead of a per-contender wait vector, and the adapter's own `Mutex`-across-transaction write path instead of the experiment's candidate |
| D2 | It closes caveat 1 of `experiments/busy-timeout-margin/README.md` (*"It is not `happenstance-sqlite`"*) | extends | The caveat is quoted inside the sweep page and answered. `kb-reference-busy-timeout-margin-001` carries the caveat set unclosed, and — being a snapshot of 2026-09-03 — correctly keeps carrying it |
| D3 | Raising the cap is free on the healthy path (duration rows), so the cap bounds only the tail | extends | Same measurement, different table. Evidence, not conclusion — the conclusion is A4 and lives in the decision |
| D4 | Fewer cores measured ~450x better on the sibling sweep, so a smaller CI runner sits in the safer regime | **aligns** | `kb-reference-busy-timeout-margin-001`'s *"Fewer cores is not safer"* section, already in the corpus. Cited by the new atom, **not copied into it** — rule 3, *"never copy paragraphs into a second atom; link instead"* |
| D5 | The 5,000 ms cell was run twice — 7 of 8 and 8 of 8, *"15 of 16 red between them"* — and both are printed rather than averaged | extends | Not in the digest. Worth carrying, because it is the page's own honesty about a shared host and it makes the reference atom's headline defensible as a range rather than a point — the same discipline `kb-reference-busy-timeout-margin-001` applies to *"1.3x to 1.4x"* |

**Net:** four `extends`, one `aligns`. Nothing here commits anything, which is the whole reason
it is a reference atom and not a paragraph in the decision.

---

## Cross-file dedup

**There is one file, so there is no cross-file dedup to do** — and that is worth one paragraph
rather than none, because the absence of the wave's usual hardest job is not the absence of a
duplication risk. The risk here is *cross-layer*: A3 and D1 are the **same measurement**, and A4
and D3 are the **same duration table**. A careless wave writes the sweep into the decision *and*
into a reference atom and creates the two-copies problem `reference/README.md` names by name:
*"Two copies of a measurement is two things to update and one that quietly goes stale, and the
stale one is the one that gets cited."*

They are split on the layer line `decisions/README.md` draws: the **number** goes in the
reference atom, the **conclusion drawn from the number** goes in the decision, and the decision
cites the reference by id. The decision quotes the three-row sweep table because a decision
without the figure that decided it is unreadable, and that is a *citation* of a figure rather
than a mirror of the page — the conditions, the method, the restore trap, the parallelism control
and the 5,000 ms double-run all stay in one place.

Likewise C1 appears in the decision's *"what this does not fix"* and in the contention open
question's merge. Same rule: the decision names the instrument in one clause and points; the open
question is where the instrument's consequences are worked out.

## What the corpus says that the intake does not

Two things this phase found by reading rather than by extracting, both recorded so the
integration phase does not have to rediscover them:

1. **The fourth reading already exists in the corpus, for a different section.**
   `kb-open-question-adr-0022-falsifiers-fired-001` records: *"A fourth reading has since been
   proposed for section 8 specifically: supersede it **in part**, items 1 and 2 only, leaving
   sections 4, 6, 7, 9, 10, 11 and 15 untouched — which is narrower than superseding ADR-0022 and
   wider than ratifying it, and shows that 'which of three moves' was itself too coarse a
   question."* The intake proposes exactly this shape for §11 without citing it. The wave is
   therefore not inventing a fourth move; it is taking one the corpus had already named and
   applying it to the other fired section. Note the irony worth recording: that sentence lists
   §11 among the sections left untouched, because it was written about §8.

2. **`kb-decision-0064` is related but the host is not the same host.** 0064 declares a dedicated
   Linux measurement host. The sweep ran on the Windows i9-13905H host every other page in
   `experiments/busy-timeout-margin/` used, which 0064 explicitly fences: *"A Windows absolute is
   never compared with a Linux absolute — only same-run ratios cross that boundary."* Nothing in
   this wave compares across that boundary — the sweep is a red-count, not an absolute, and its
   only cross-page comparison is to the core sweep on the same host. The new reference atom says
   so in terms rather than leaving a reader to assume the declared-conditions host.
