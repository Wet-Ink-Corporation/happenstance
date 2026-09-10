# Wave `2026-09-09-intake` — placement and adjudication

**Fourteen operations**, ordered so that every atom a later operation links to exists before the
link is written. Five staged files, roughly forty claims, and **four cross-file clusters** —
every one of which collapses to a single destination atom. `00-corpus-match.md` establishes why.

**What this wave does to the accepted decision corpus: nothing destructive.** Three decision
atoms are added. **No accepted body is edited, no `status` is flipped on any decision, and no
decision is superseded** — fifth wave running. One accepted decision, `kb-decision-0036`, is
amended *from outside* on the precedent ADR-0029 set against ADR-0004, at the explicit request
of the file that amends it. A second, `kb-decision-0017`, is *applied* and its falsifier is
deliberately **not** claimed as discharged.

## Ordering

```
Op   1      reference atom       — the evidence, before the decision that rests on it
Ops  2– 4   decision atoms       — ADR-0060 first, because ADR-0025 cites its finding
Ops  5– 6   open-question atoms  — both link decisions signed at ops 3 and 4
Ops  7–14   merges into existing atoms
            ── the Maps phase runs after all fourteen ──
```

ADR-0060 precedes ADR-0025 for one reason worth stating, because the obvious order is numeric:
ADR-0025's scope note *"PS-2's other end is forbidden by the port for both drivers the clause
names"* is ADR-0060's finding, and ADR-0025 must **cite** it rather than restate it. Numeric
order would have written the citation before the atom existed, and the usual repair — restating
the mechanism in both — is exactly the two-copies-one-drifts failure this wave is guarding
against everywhere else. Merges run last because every one writes a link to an atom created
earlier in the same wave.

---

## The operation table

`adr-0025` = `2026-09-08-adr-0025-ladybug-projection-adapter.md` ·
`adr-0060` = `2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md` ·
`adr-0061` = `2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md` ·
`es-11` = `2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md` ·
`ps-2` = `2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md`

| # | Op | Kind | Destination | Sources | Class |
| --- | --- | --- | --- | --- | --- |
| 1 | `create_new` | reference | `reference/ladybug-driver-probes-2026-09.md` | `adr-0025` | extends |
| 2 | `create_new` | decision | `decisions/0060-ps-2s-axis-re-evaluated.md` | `adr-0060`, `ps-2` | requires-new-decision |
| 3 | `create_new` | decision | `decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md` | `adr-0061`, `es-11` | requires-new-decision |
| 4 | `create_new` | decision | `decisions/0025-the-ladybug-projection-adapter.md` | `adr-0025` | requires-new-decision |
| 5 | `defer_open_question` | open_question | `open-questions/one-shot-http-conformance-to-es-11.md` | `adr-0061`, `es-11` | extends |
| 6 | `defer_open_question` | open_question | `open-questions/experiment-raw-output-eaten-by-the-ignore-rule.md` | `adr-0025`, `adr-0061` | extends |
| 7 | `merge_existing` | open_question | `open-questions/probe-read-through-signature-and-live-transaction-seam.md` | `ps-2`, `adr-0060` | extends |
| 8 | `merge_existing` | open_question | `open-questions/es-7-and-vt-9-provisional-markers.md` | `adr-0025`, `ps-2`, `adr-0060` | extends |
| 9 | `merge_existing` | open_question | `open-questions/es-11-ceiling-sample-cost-on-sqlite-read.md` | `adr-0061`, `es-11` | extends |
| 10 | `merge_existing` | open_question | `open-questions/reset-refusal-declension-has-no-clause.md` | `adr-0025` | extends |
| 11 | `merge_existing` | governance | `governance/rewrite-the-referent-never-the-reasoning.md` | `adr-0060` | extends |
| 12 | `merge_existing` | governance | `governance/what-may-refute-a-finding.md` | `es-11`, `adr-0061` | extends |
| 13 | `merge_existing` | playbook | `playbooks/repairing-a-frozen-clause-without-amending-it.md` | `adr-0061` | extends |
| 14 | `merge_existing` | playbook | `playbooks/a-count-or-an-index-nobody-re-derives.md` | `adr-0061` | extends |

On every `merge_existing` row, **keys not listed in the frontmatter block below are carried
forward untouched.** `KbFrontmatter` is a passthrough object and several of these atoms carry
governance keys redkiln neither owns nor validates; stripping one to make an atom "conform"
breaks every atom that references it, and nothing brings the key back.

---

## The adjudications that were not obvious

### Adjudication 1 — ADR-0025 takes a reserved number, and the brief caught this first

**Op 4** · `create_new` · `.kb/decisions/0025-the-ladybug-projection-adapter.md`

`0025` is not the next free number and it is not a gap. `RUNBOOK.md`:392 reads
*"**0025** | 11 | Ladybug: checkpoint placement, how a projection expresses graph mutations, and
the blocking API"* — verbatim this decision — and it has read that since the plan was written.
`references/adr/0025-the-ladybug-projection-adapter.md` already exists as the long form.

The staged brief records its own near-miss in a sentence this wave is repeating because it
generalises: *"This session initially read the listing as '0059 is the highest, so 0060 is next'
and was wrong; a gap in a sequence is invisible to a listing and obvious to a diff."* Wave 10
made the same call for ADR-0024 and had to correct five digests to do it. `0026`, `0027` and
`0028` stay reserved and unclaimed.

```yaml
id: kb-decision-0025
title: The Ladybug projection adapter — a checkpoint node, raw Cypher, and a blocking driver
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0025
reversibility: medium
phase: 11
supersedes: null
superseded_by: null
depends_on:
  - kb-decision-0017
  - kb-decision-0030
related:
  - kb-decision-0060
  - kb-decision-0036
  - kb-reference-ladybug-driver-probes-001
  - kb-open-question-reset-refusal-declension-001
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-probe-read-through-signature-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - references/adr/0025-the-ladybug-projection-adapter.md
  - experiments/ladybug-driver-probes/README.md
  - crates/happenstance-ladybug/src/lib.rs
  - crates/happenstance-ladybug/src/projection_store.rs
  - crates/happenstance-ladybug/tests/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
```

**One atom, not six.** The brief carries five parenthetically separate decisions — checkpoint
placement, write vocabulary, blocking-only, handle shape, and `commit`'s rollback path — plus a
feature gate. `kb-playbook-one-decision-per-adr-title-001` is the standing pressure to split
them, and the answer is that the brief states **one question** and it is the one on the atom's
title: *what must a projection adapter do to satisfy PS-1 on an engine with no transaction
handle type, a blocking driver and Cypher as its only mutation surface?* Every part answers that
question about the same engine, and each one's argument cites another's constraint —
`Arc<Database>` is forced by the file lock, which is what makes `SECOND_HANDLE` a second
`Connection`, which is what makes out-of-connection observability the only way PS-1's coupling
is visible. Split six ways, the atom that explains why the checkpoint is a graph node loses the
sentence about the only atomicity the engine offers.

**The sentence a summary would drop, and must not.** `commit`'s error path issues **no**
`ROLLBACK` after a STATEMENT error — the engine has aborted the transaction itself and a
rollback afterwards is refused, so issuing one masks the first error with a second. **The
qualifier is load-bearing and §7's original wording lacked it.** After a *decode* failure the
statement succeeded, the engine aborted nothing, and a bare `?` inside `commit_inner` returns
with the transaction still open; that path rolls back **best-effort** and discards the result.
The brief says exactly why the unqualified form must not be inherited: it *"reads as licence to
`?` out of an open transaction."*

**What it must not do.** Claim `kb-decision-0017`'s PS-9/PS-11 falsifier. That falsifier names
*a second generic consumer* — library code `happenstance` itself ships that must write into an
unknown adapter's batch — and is owned by a different phase. An adapter is evidence about a
clause's **cost**, not the data point the clause waits on. ADR-0017 is accepted and stays
byte-identical; the crate root has already been corrected to say this in the past tense
(`lib.rs`:151), so the atom records it as history and asks for nothing.

**`mapsImpact`:** `decisionMap`, `domainMap`. Ops 8 and 10 carry the `openQuestionIndex` half.

### Adjudication 2 — nothing is minted asking for an edit that has landed

**Op 4**, and it is the wave's cheapest discipline to get wrong.

Two of the brief's claims are corrections it requests, and both were verified in this worktree
before the plan was written: `crates/happenstance-ladybug/src/lib.rs`:151 now reads *"This crate
is **not** one of PS-9's or PS-11's data points, which this paragraph used to claim"*, and
`RUNBOOK.md`:4917 now reads *"**Fills the write-vocabulary axis**"* and records that it had said
*"fills the batch-shape axis"* and that this is not what it filled. The brief's own citation of
`RUNBOOK.md:4802` is stale in both line and tense, which is what the anchoring playbook predicts
of a line number in a 400,000-line file.

An atom minted asking for either edit would be a **task**, and `open-questions/README.md` is
explicit: *"Work that someone is expected to do is a backlog item in `.bklg/`, not a KB atom."*
Both land as history inside ADR-0025, in the past tense, naming what the prose used to say —
which is the form that keeps a later reader from re-deriving why the sentence changed.

### Adjudication 3 — the PS-2 pair is one decision, and the finding is its argument

**Op 2** · `create_new` · `.kb/decisions/0060-ps-2s-axis-re-evaluated.md` · sources `adr-0060`
**and** `ps-2`

Two files, one commitment. `ps-2` is a **finding**, in its own words: *"Kind: finding against a
`[FROZEN]` clause — reported, not settled"*, *"Owner: PS-2's, not this adapter's"*, and it closes
with a three-option menu it explicitly declines to choose from. `adr-0060` is the choice. Minting
both would file a live question over a settled one, which `open-questions/README.md` names as the
way a resolved decision gets re-litigated.

The mechanisms live in the decision's body because they are what the decision *rests on*: `sqlx`'s
`Transaction::begin` is async and fallible with private fields against a `begin` that is total,
synchronous and infallible; `rusqlite`'s `Transaction<'_>` is `!Send` and costs the
`SendProjectionStore` impl. **Two independent mechanisms, which is why finding one did not predict
the other** — a sentence worth keeping, because a reader who has met only the `sqlx` half will
assume the `rusqlite` half follows from it.

```yaml
id: kb-decision-0060
title: The projection port keeps its gate, and the reason ADR-0036 gave has expired
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0060
reversibility: medium
phase: 11
supersedes: null
superseded_by: null
depends_on:
  - kb-decision-0036
related:
  - kb-decision-0017
  - kb-open-question-probe-read-through-signature-001
  - kb-open-question-provisional-falsifiers-001
  - kb-governance-referent-not-reasoning-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md
  - .kb/_intake/2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md
  - references/adr/0060-ps-2s-axis-re-evaluated.md
  - crates/happenstance-core/tests/probe_live_transaction_shape.rs
  - crates/happenstance-core/src/projection.rs
  - crates/happenstance-neon/tests/neon_projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
```

**`phase: 11` is a judgement call and is recorded as one.** ADR-0036 carries `phase: 6`, because
PS-2 and PS-3 are phase 6's clauses. ADR-0060 decides the same question and its *evidence* is
phases 10b and 11 — `happenstance-neon` passing, and Ladybug arriving as the fourth adapter
without firing the pre-registered condition. The phase field on this corpus records the phase
that owned the work (ADR-0024 is `10`, ADR-0058 is `12`), not the phase that owns the clause, so
`11` is the consistent reading. The decision map should carry both facts on the row rather than
only the number, since a reader coming from ADR-0036 will expect `6`.

**`mapsImpact`:** `decisionMap`, `domainMap`.

### Adjudication 4 — the ES-11 pair is one decision, and the finding says so in its own header

**Op 3** · `create_new` · `.kb/decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md` ·
sources `adr-0061` **and** `es-11`

The same shape as Adjudication 3, with the dedup already performed by the source. `es-11` carries
a callout added above its own first line: *"Settled on 2026-09-08 by ADR-0061… This file stays as
the finding, unedited below this line, because the three options it lists and the reasons two of
them lost are the argument the decision rests on."* This wave takes that at its word.

**The callout also retracts two of its own file's numbers, and the decision atom must inherit the
retractions rather than the originals.** The CI cost is not *"red about 1 run in 40"* but **zero**
— `NEON_CONNECTION` is not a repository secret, so every `live-neon` step is gated on it and the
job prints its own *"proves NOTHING"* notice and stops. And *"104 of 105"* has a soft edge:
`query_items_share_one_snapshot` is exposed to the same race by construction and has not yet lost
it, which is a different statement from passing. An atom that carried `1 in 40` would be citing a
figure its own source struck.

```yaml
id: kb-decision-0061
title: ES-11's sufficiency condition assumed a queue, and one-shot HTTP has none
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0061
reversibility: medium
phase: 10
supersedes: null
superseded_by: null
depends_on:
  - kb-decision-0011
related:
  - kb-decision-0051
  - kb-decision-0013
  - kb-open-question-es-11-sqlite-ceiling-sample-cost-001
  - kb-open-question-one-shot-http-es-11-001
  - kb-playbook-repair-frozen-clause-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - .kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md
  - references/adr/0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - crates/happenstance-neon/src/lib.rs
  - .github/workflows/ci.yml
  - HANDOVER.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
```

**`phase: 10`, and the work was phase 10b.** The schema takes an integer or null, and `10b` is
neither. The body says 10b; the field says 10. This is the second wave to hit it and it is worth
one sentence in the map rather than a schema argument.

**No rate, and this is a constraint both source files impose in their own voices.** *"No rate is
stated, and the atom minted from this brief must not add one."* The reason is the corpus's own
discipline — a measurement lives beside its raw output under `experiments/` — and the first
draft's figures cited only a doc comment restating them. The finding rests on the **direction**
of failure, which is invariant: the read always sees the later append. That is what the atom
carries.

**The two refused ways out are the half a summary loses.** Putting one-shot HTTP outside ES-11's
scope is a *widening*, and ES-11 and ES-12 *"reduce to ES-10 plus a ceiling"*, so it would reach
append-condition correctness. Minting a `Capability` so the rule reports a skip misuses
`Capability`, which says what a fixture can **arm** and not whether a store provides a guarantee
— it would print "skipped" where the truth is "does not conform". `kb-decision-0051` is the
precedent that makes the second argument legible, which is why it is in `related`.

**`mapsImpact`:** `decisionMap`, `domainMap`, `openQuestionIndex` (Op 5 adds a bullet, Op 9
changes one).

### Adjudication 5 — ADR-0036 is amended from outside and is not superseded

**Op 2**, and it is this wave's most contestable call.

`kb-decision-0036` scores 92 against `adr-0060` and is **accepted and immutable**. The reading
that would supersede it is not unreasonable: ADR-0060 says in as many words that ADR-0036's
reason *"has expired"*, and a decision whose stated reason has expired looks like a decision that
has expired. It has not, and the source is the authority on that — `adr-0060` carries a section
titled `## Why ADR-0036 is not superseded` whose whole content is the argument:

> Its decision stands. Only its reason is replaced, and the two are recorded separately rather
> than merged: *"only one adapter has run the suite"* was true when written, and is exactly the
> kind of sentence that would otherwise be quietly rewritten into something it never said.

Three things follow, and each is checked rather than assumed.

**The decision is unchanged.** ADR-0036 decided that `ProjectionStore` is not frozen at `0.2.0`
and ships behind `unstable-projection`. ADR-0060 decides the same thing. Nothing a consumer or
an adapter must do differs by one character, which is the mechanical test
`kb-playbook-repair-frozen-clause-001` states one layer down and `decisions/README.md` states
here: *"a correction is a repair if the set of implementations the decision admits is
unchanged."*

**The reason genuinely expired, and it expired by evidence rather than by argument.**
`kb-governance-what-may-refute-a-finding-001`'s third consequence makes an accepted decision's
currency a computation. ADR-0036's part-2 finding cites the adapter set; run the check and the
set has moved: `projection_store_conformance!` is invoked by **four** storage adapters in this
worktree — `happenstance-sqlite`, `happenstance-postgres`, `happenstance-neon` and
`happenstance-ladybug` — against ADR-0036's *"exactly one storage adapter has run
`projection_store_conformance!`"*. The sentence was true when written and is false now. That is
a refutation of the *reason*, and the governance atom is explicit that a partial refutation
restates the residual rather than striking the finding.

**And the residual survives, which is why the gate does.** ADR-0036 gated on scarcity; ADR-0060
gates on a compiled cause — freezing `begin`, `probe_write` and `probe_read_through` would make a
semver promise out of precisely the signatures that forbid the second shape. **Freezing them
would make a promise out of the defect.** The new reason is indifferent to a fifth adapter
arriving, which the old one was not, and the falsifier says so: *not* reopened by another
adapter at the buffered end passing.

The precedent for the shape is `kb-decision-0029` against `kb-decision-0004` and
`kb-decision-0035` against `kb-decision-0001` — amend from outside, edit nothing, and let the
map row carry a second "amended by" annotation rather than a flip. **`kb-decision-0036` stays
`accepted`, `superseded_by: null`, and byte-identical.**

### Adjudication 6 — the probe seam is the owner, and its cause is replaced rather than its gap

**Op 7** · `merge_existing` · `.kb/open-questions/probe-read-through-signature-and-live-transaction-seam.md`

`kb-open-question-probe-read-through-signature-001` scores 85 and is the direct owner: it already
holds the compiled table for this exact seam, already names the recommended signature change,
and already asks the corollary question about judging PS-2's part 2 on a capability an adapter
had to declare false. The merge does three things the atom cannot currently say.

**It replaces the attributed cause without touching the gap.** The atom's own summary says
ADR-0036 *"attributed the gap to scarcity — nothing built yet; this atom adds a second, compiled
cause."* `ps-2` supplies the third and strongest: `begin`'s totality forbids the `sqlx`
expression outright, independently of any probe, and `rusqlite`'s `!Send` transaction forbids the
other named driver by a different mechanism. Scarcity is no longer even a contributing cause.

**It widens the scope from one method to the seam.** This is `adr-0060`'s contribution and it is
the sentence the existing atom most needs: the recommended fix is *"not sufficient alone"* —
`probe_write` being synchronous and infallible still forces a live-transaction store to buffer,
so the honest scope is the whole probe seam. An atom recommending a one-method fix that does not
work is worse than one recommending nothing, because the fix looks affordable.

**It records that the suite cannot tell the ends apart.** A conformant live-transaction adapter
must declare `READS_THROUGH_BATCH = false` — `probe_live_transaction_shape.rs`'s own comment
calls it *"a false statement about this store"* — so it reports the same capability profile as a
buffering one. PS-2's bar asks for two adapters at opposite ends of an axis the instrument cannot
distinguish.

```yaml
id: kb-open-question-probe-read-through-signature-001
title: probe_read_through's signature cannot be implemented correctly by a live transaction
kind: open_question
status: accepted
authority_tier: note
related:
  - kb-decision-0036
  - kb-decision-0060                       # added
  - kb-decision-0017                       # added
  - kb-open-question-provisional-falsifiers-001   # added
  - kb-open-question-projection-batch-no-apply-001
  - kb-open-question-ps-19-scope-narrower-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/probe-read-through-and-the-live-transaction-end.md
  - .kb/_intake/2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md   # added
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md                      # added
last_reviewed: 2026-09-09
```

**It is not resolved, and the distinction matters.** ADR-0060 declines the signature change and
says whose call it is. The question the atom asks — does the signature move, or does the adapter
get built against today's shape — is still open, and is now open with a wider scope than when it
was written. `status` stays `accepted`.

**`mapsImpact`:** `openQuestionIndex`.

### Adjudication 7 — the cluster three files share and no digest named

**Op 8** · `merge_existing` · `.kb/open-questions/es-7-and-vt-9-provisional-markers.md`

This is the wave's real dedup, and all three per-file digests missed it. The Ladybug digest
routed PS-4's Rust-level finding into ADR-0025's body; the `ps-2` digest routed the
thirteen-clauses claim to `defer_open_question` with *"(no dedicated existing owner found)"*.
Both are the same defect in two different clauses, and an owner exists.

`kb-open-question-provisional-falsifiers-001` is titled **"Two provisional markers whose
falsifiers can no longer falsify"** and its transferable observation is *"a falsifier that has
already occurred without changing anything is a marker that has quietly become decoration, and
`spec-trace` cannot detect it — it sees that a marker exists, not whether its condition has been
met."* The two incoming instances are the same family and are **stronger**, because these
falsifiers never could fire:

- **PS-4's Rust-level condition** — *"if its write handle must exist before a traversal that the
  projection's own logic depends on"* (`spec/SPECIFICATION.md`:5152-5154) — is foreclosed by the
  port for every batch shape, because `Projection::apply` is synchronous and a traversal is I/O.
  Not a fact about Ladybug: equally true of the SQLite and Postgres adapters.
- **PS-2's live-transaction end** is forbidden by `begin`'s signature for both drivers PS-2
  names, so the bar's second half cannot be met by the population the clause identifies — and
  **thirteen `[PROVISIONAL]` clauses are gated on PS-2 alone.** `ps-2`'s own framing is the one
  to keep: *"a provisional group waiting on an adapter nobody can build waits forever."*

The observation the atom carries gets one clause sharper: a falsifier can be decoration *a
priori* and not only in retrospect, and `spec-trace` sees neither shape — it checks that
citations resolve.

```yaml
id: kb-open-question-provisional-falsifiers-001
title: Two provisional markers whose falsifiers can no longer falsify
kind: open_question
status: accepted
authority_tier: note
related:
  - kb-decision-0025                       # added
  - kb-decision-0060                       # added
  - kb-open-question-probe-read-through-signature-001   # added
  - kb-playbook-repair-frozen-clause-001
  - kb-reference-port-traits-compiled-findings-001
  - kb-open-question-cf-17-cf-14-markers-001
  # every existing related id is carried forward
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md                   # added
  - .kb/_intake/2026-09-08-ps-2-live-transaction-axis-is-forbidden-not-unbuilt.md   # added
  - .kb/_intake/2026-09-08-adr-0060-ps-2s-axis-re-evaluated.md                      # added
  # every existing source path is carried forward
last_reviewed: 2026-09-09
```

**The title stays.** It names two markers and the merge adds two more, which is an argument for
retitling and a stronger argument against: the atom is cited by eleven others under this title,
and `kb-playbook-anchoring-citations-001` is the standing reason not to move a referent that
resolves. The body says four; the title says the two it was minted for.

**`mapsImpact`:** `openQuestionIndex`.

### Adjudication 8 — a sub-question is answered and the atom is not resolved

**Op 9** · `merge_existing` · `.kb/open-questions/es-11-ceiling-sample-cost-on-sqlite-read.md`

`kb-open-question-es-11-sqlite-ceiling-sample-cost-001`'s **third ordered sub-question** reads:
*"Should this finding change ES-11's `[PROVISIONAL]` marker or its stated falsifier condition, or
does it simply sit as supporting evidence until a HTTP-transport adapter also reports against
it?"* Its body says the SQLite measurement is *"a second, native data point for that review, not
an independent trigger."*

The HTTP-transport adapter has reported. ADR-0061 is that review, and it answers sub-question 3
in both halves: the marker does move — rewritten to **record** a fired falsifier rather than
predict one — and it stays `[PROVISIONAL]`, because what is now open is whether a conformant
one-shot-HTTP shape exists at all.

**Sub-questions 1 and 2 are untouched and the atom is not resolved.** They are about the SQLite
remedy — whether the `Clone`/`connect()` decision sequences first, and whether a waker-registration
rewrite risks reintroducing the failure the two named tests catch. ADR-0061 decides nothing about
`happenstance-sqlite`, and the 635 ms stall is exactly where it was. Resolving the atom on a
one-in-three answer would discard both.

The merge also records what ADR-0061 makes *newly* relevant to the SQLite half: ES-11's own
`Rejects:` bullet ended *"It is conformant today"*, written 2026-08-06 and untouched through two
phases, and it was repaired in ADR-0061's change. The atom quotes ES-11's marker; whoever reads
it next needs to know the marker has moved under the quotation.

**`mapsImpact`:** `openQuestionIndex`.

### Adjudication 9 — the population that was empty is not, and it declines

**Op 10** · `merge_existing` · `.kb/open-questions/reset-refusal-declension-has-no-clause.md`

`kb-open-question-reset-refusal-declension-001` closes its own summary with a dated premise:
*"The population that could be lying about `RESET_REFUSAL` today is empty (no projection adapter
has run the suite), which is why the cost of leaving this open is low rather than zero, and why
it should close before the first one does."*

The first one has arrived. Four storage adapters now run `projection_store_conformance!`, and
`happenstance-ladybug`'s pre-registered verdict — committed before any body, so it could not be
written to fit the outcome — predicted `RESET_REFUSAL` **declined**, and it held.

**The merge is narrow on purpose, because the news cuts both ways.** A declining adapter cannot
be lying about the capability, so the hazard the atom describes — declared supported, overridden
with an empty body, `refused_reset_changes_nothing` passing on a store that was never asked to
protect anything — still has an empty population. What has changed is the atom's own *cost
argument*: the window it names (*"before the first one does"*) has closed, and the next adapter
to declare `RESET_REFUSAL` supported will do so with nothing checking it. That is a sharpening of
urgency, not an answer, and the atom stays `accepted`.

**`mapsImpact`:** `openQuestionIndex`.

### Adjudication 10 — two governance atoms and two playbooks, each taking a different half

**Ops 11–14** · `merge_existing`

Four merges into transferable-practice atoms, and the temptation in each is to mint a new atom
because the incoming lesson has a memorable sentence. The default bias is merge, and each of the
four has a direct owner whose own test the lesson sharpens.

**Op 11 — `governance/rewrite-the-referent-never-the-reasoning.md`.** Its test is *"ask whether
the edit changes what the document asserts, not whether it changes the document"*, and it holds
three worked instances: rewritten in place (a rename), left verbatim under a superseded banner,
and reasoning stands while a bundled sub-decision reverses. `adr-0060` supplies a fourth the test
implies and none of the three shows: **the decision stands and its reason expires.** The atom's
own sentence about ADR-0002 is the parallel — rewriting *"only one adapter has run the suite"*
into *"four have"* would convert a true claim about the state of the tree in 2026-09 into a false
one about what ADR-0036 found. Same failure, one layer up: the reason, not the referent.

**Op 12 — `governance/what-may-refute-a-finding.md`.** Its `## A fourth consequence` already runs
the standard of evidence in both directions and closes with *"a partial refutation restates the
residual rather than striking the finding."* `es-11` supplies the case where the *predecessor* is
what was retracted: `HANDOVER.md` records ES-11 escalated in error and retracted (`2e0a0ae`) on
the claim that *no async driver can conform*, and commit `0341467`'s message asserting the
falsifier had fired is, in the file's own words, *"wrong and is history, not guidance."* The new
claim is narrower — *a driver with no shared ordering primitive between its operations cannot* —
and what separates them is a **measurement rather than an argument**: `happenstance-neon` already
does the thing that refuted the old claim, spawning at call time, and fails anyway. The
transferable rule: a retracted claim does not poison its own neighbourhood, and the way to show
it is to apply the old refutation's remedy and show the failure survives it.

**Op 13 — `playbooks/repairing-a-frozen-clause-without-amending-it.md`.** Its test is mechanical:
*"a correction to a `[FROZEN]` clause is a repair if the set of implementations the clause admits
is unchanged, and otherwise it is a gap, which is an ADR's."* ADR-0061's amendment is the case
that sharpens it, because every instinct says repair — the MUST, the maturity marker, the `Rule`
and the `Cases` are all untouched, and **nothing an adapter must do gets easier**. It is still an
amendment, because it *removes a route to a conformance claim*: an adapter that could previously
have argued conformance from "spawned at the first poll" no longer can. The admitted set moved
even though no obligation did, and the correct handling was an ADR — which is what happened.

**Op 14 — `playbooks/a-count-or-an-index-nobody-re-derives.md`.** Four passages the falsifier's
arrival made false: two `SPECIFICATION.md` §6.5 rows calling shipped adapters phase-2 skeletons,
a §3 cell saying transport was *"empty at both ends"*, and ES-11's own `Rejects:` bullet ending
*"It is conformant today"*, written 2026-08-06 and untouched through the two phases in which that
adapter was built and shipped. The playbook's root is *"a written claim decays the moment the
thing it counts changes and nobody is looking"*, and this is its third shape — not a cardinal
number and not a curated index, but a **status claim about an axis**. The generalisable half is
ADR-0061's own: *a falsifier firing invalidates prose that names the axis, and nothing mechanical
looks for it.* `spec-trace` is a gate step and it checks that citations **resolve**, not that
prose is **current**.

**The alternative that lost, for Op 14.** A new playbook atom — *"a falsifier firing invalidates
the prose that names its axis"* — is a real candidate and scores nothing against any existing
atom on its own words. It loses on the default bias and on the root: the repair pattern is
identical to the count playbook's (*"remove the count where a command already answers it… spell a
list by its members once a count has drifted"*), and splitting one root across two playbooks is
how the next author finds neither.

**`mapsImpact`** for all four: `domainMap`.

### Adjudication 11 — one new open question the decision itself names

**Op 5** · `defer_open_question` · `.kb/open-questions/one-shot-http-conformance-to-es-11.md`

ADR-0061 states its own residual in one sentence: *"What is open is no longer whether this shape
fails — it does — but whether a conformant one-shot-HTTP shape exists at all."* ES-11 keeps
`[PROVISIONAL]` precisely to hold that question open, so the marker and the atom have to agree or
the marker is decoration — which is the defect Op 8 is about.

No atom owns it. `kb-open-question-es-11-sqlite-ceiling-sample-cost-001` is about a native
adapter's cost under lock contention. `kb-open-question-off-poll-adapter-visibility-001` is about
an instrument the suite lacks. This is a third question: whether the obligation is *meetable* by
a transport with no ordering primitive the store honours, and what an adapter would have to
supply if it were.

```yaml
id: kb-open-question-one-shot-http-es-11-001
title: Whether any one-shot-HTTP adapter can satisfy ES-11, now that the first one cannot
kind: open_question
status: accepted
authority_tier: note
depends_on: []
related:
  - kb-decision-0061
  - kb-open-question-es-11-sqlite-ceiling-sample-cost-001
  - kb-open-question-provisional-falsifiers-001
  - kb-decision-0011
source_paths:
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - .kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md
  - crates/happenstance-neon/src/lib.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
```

**What forces it**, and the atom must say so: nothing today, because `NEON_CONNECTION` is not a
repository secret and `live-neon` never runs. The day someone adds the secret, a strict job
begins reddening for a non-regression reason, and ADR-0061's own decision to keep it strict
*"should be re-read on that day rather than before it."*

**`mapsImpact`:** `openQuestionIndex`, `domainMap`.

### Adjudication 12 — the wave discovered a defect, and says so rather than citing past it

**Op 6** · `defer_open_question` ·
`.kb/open-questions/experiment-raw-output-eaten-by-the-ignore-rule.md`

This one is not a claim in any staged file. It surfaced in `00`'s provenance check and it has to
be recorded, because the alternative is Op 1 citing a path that does not resolve.

`experiments/ladybug-driver-probes/README.md`:9 states *"`results/probe-output.txt` is its
output, verbatim."* There is no `results/` directory. `git check-ignore -v` names the cause:
`.gitignore`:69's `*-output.txt`, written at the pre-publication sweep because five transcripts
carrying `C:\Users\<name>` were sitting untracked-and-unignored one `git add -A` away from a
repository about to go public. The rule is good and the collision is invisible: **321 files under
`experiments/**/results/` are tracked**, none of them suffixed `-output.txt`, so every other
experiment escapes by accident of naming.

It matters to this wave in particular. ADR-0061 refuses to state a failure rate on the stated
ground that *"a measurement lives beside its raw output under `experiments/`"* — and ADR-0025's
own evidence is in exactly the position that rule forbids, while reading as though it is not.

```yaml
id: kb-open-question-experiment-raw-output-ignored-001
title: .gitignore's *-output.txt eats the raw output experiments/ requires be committed
kind: open_question
status: accepted
authority_tier: note
depends_on: []
related:
  - kb-decision-0025
  - kb-decision-0061
  - kb-reference-ladybug-driver-probes-001
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - .gitignore
  - experiments/ladybug-driver-probes/README.md
last_reviewed: 2026-09-09
```

**Its provenance is stated in its own body**, so a later reader does not go looking for the brief
that raised it: this is a wave-discovered finding, verified by `git check-ignore -v` and by
counting tracked files under `experiments/`, and it names no remedy — renaming the file, carving
an exception, or deciding the transcription in the README is sufficient are three different
answers with three different costs, and choosing among them is not an ingest's to make.

**`mapsImpact`:** `openQuestionIndex`, `domainMap`.

### Adjudication 13 — the evidence gets an atom, and the failure rate does not

**Op 1** · `create_new` · `.kb/reference/ladybug-driver-probes-2026-09.md`

`decisions/README.md` is explicit that a decision does not carry its own evidence: *"The
measurement or compilation a decision rests on is a `reference` atom that this one cites.
Separating them is what lets a decision be superseded without invalidating the evidence
underneath it."* ADR-0025 rests on four probe results and one build story, and no existing
reference atom mentions `lbug` at all.

```yaml
id: kb-reference-ladybug-driver-probes-001
title: What lbug 0.20.3 actually does — the four probes ADR-0025 could not be written without
kind: reference
status: accepted
authority_tier: note
depends_on: []
related:
  - kb-decision-0017
  - kb-open-question-reset-refusal-declension-001
source_paths:
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - experiments/ladybug-driver-probes/README.md
  - experiments/ladybug-driver-probes/probes.rs
last_reviewed: 2026-09-09
```

**The dating rule, applied literally.** `reference/README.md`: *"A measurement without a commit
sha or a date is not a weaker reference, it is a false one."* The run is
`2026-09-08`, `x86_64-pc-windows-msvc`, rustc 1.97.1, `lbug` 0.20.3, and all four go in the body.

**And the atom says the raw file is absent.** It cites the README's verbatim transcription and
records that `results/probe-output.txt` is not in the tree and why (Op 6). A reference atom that
pointed at an ignored path would be precisely the *false reference* the layer's own rule names —
it reads as citable, it gets cited, and nothing detects that the citation never resolved.

**One generalisation the atom must not make, and it is carried to `unresolved[]`.** *"Two
`Database`s over one directory is refused by a file lock"* is one run on one platform, and the
refusal it records is `Error: 33`. `Arc<Database>` is forced by that observation on that host;
whether the lock is the engine's property or the platform's is a distinction a single-host run
cannot draw, and the atom states the observation rather than promoting it into a property of the
driver.

**What does not belong in it.** The conclusion. `Arc<Database>` with a second `Connection` per
handle is ADR-0025's decision and lives there, citing this.

**`mapsImpact`:** `domainMap`.

---

## What the Maps phase inherits

| Map atom | Work |
| --- | --- |
| `maps/decision-map.md` | Three rows — ADR-0025 (phase 11, a reserved row finally filled), ADR-0060 (phase 11) and ADR-0061 (phase 10). A second "amended by" annotation on `kb-decision-0036`'s existing row, **not** a status flip. One section, since all three are the same 0.2.0 closeout pass |
| `maps/domain-map.md` | Two sections: the projection-port area gains ADR-0025, ADR-0060 and the reference atom; the conformance/specification-governance area gains ADR-0061, both new open questions and the four transferable-practice merges |
| `maps/open-questions-index.md` | Two new bullets (Ops 5, 6) and four changed ones (Ops 7–10). **No bullet is struck** — nothing is resolved, withdrawn or superseded this wave |

## What this wave leaves unresolved

Three, and none of them is guessed at. They are carried to `unresolved[]` in the manifest.

1. **Whether `lbug`'s file lock is the engine's or the platform's.** One run, one host,
   `Error: 33`. ADR-0025's decision is forced by the observation either way; the *generality* of
   the observation is not established, and the reference atom states one host rather than a
   property.
2. **What to do about `.gitignore`:69.** Recorded as an open question (Op 6) rather than answered.
   Rename the file, carve an exception, or accept the README's transcription as the evidence —
   three answers with three different costs, and an ingest chooses none of them.
3. **Whether ADR-0060's `phase` is 11 or 6.** Both readings are defensible and the corpus's
   convention is not written down anywhere. `11` is proposed on the precedent of ADR-0024 and
   ADR-0058 (the phase that owned the work), and flagged here because a reader arriving from
   ADR-0036's `phase: 6` will expect the other answer.
