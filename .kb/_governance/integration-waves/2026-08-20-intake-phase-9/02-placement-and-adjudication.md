# Wave `2026-08-20-intake-phase-9` — placement and adjudication

The ordered action plan. Eleven operations: two decision atoms created, two reference atoms
created, two open questions created, two open questions resolved, three open questions
annotated. **No decision atom's body is edited, no decision atom's `status` is flipped, and no
supersession is performed anywhere in this wave.**

Order is load-bearing in five places, and the rule is the one every wave uses: an atom a later
op links to is created first, and mutating ops run strictly serially.

```
Op  1  create_new     .kb/open-questions/no-workerd-class-runner-in-the-gate.md
Op  2  create_new     .kb/open-questions/deny-bans-red-on-the-worker-dependency.md
Op  3  create_new     .kb/reference/wf-11-memory-ceiling-verdict-2026-08.md
Op  4  create_new     .kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
Op  5  create_new     .kb/decisions/0034-the-fixture-contract-has-no-single-owner.md
Op  6  merge_existing .kb/open-questions/cf-40-fixture-limits-ownership.md            (resolve)
Op  7  merge_existing .kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md  (resolve)
Op  8  merge_existing .kb/open-questions/es-6-names-an-unwritable-rule.md             (narrow, NOT resolve)
Op  9  merge_existing .kb/open-questions/poll-count-bounds-the-visibility-rule.md     (annotate, stays Open)
Op 10  create_new     .kb/reference/phase-8-specification-reconciliation-census.md
Op 11  merge_existing .kb/open-questions/nothing-owns-the-post-phase-reconciliation.md (evidence, stays Open)
```

- **Ops 1, 2 and 3 before Op 4** — ADR-0023's `related` names all three by id.
- **Op 1 before Op 3** — the WF-11 verdict's *"what would supply it"* points at the runner
  question rather than describing it a second time.
- **Op 4 before Op 5** — ADR-0034 cites `CloudflareFixture` as its third data point and names
  `kb-decision-0023`.
- **Op 5 before Ops 6 and 9** — both annotations name `kb-decision-0034`.
- **Op 10 before Op 11** — the question's dated section points at the census.

Ops 6, 7, 8, 9 and 11 are the mutating operations and run strictly serially, in that order.
Every one of them touches an `open_question`, which is the only layer this wave mutates.

---

## Standing choices, applied to every atom in the wave

### 1. One atom per ADR, and the long-form record keeps the evidence

Unchanged from waves 2–5. It applies **cleanly** to ADR-0023 for the first time in three waves:
`references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` exists on disk, is
the only untracked addition under `references/adr/`, and modifies nothing there. Link the atom;
cite the record by `file:line`.

It applies **aspirationally** to ADR-0034, which has no `references/adr/` record — Adjudication 7,
and wave 5's Adjudication 8 is the precedent that this does not block the mint.

### 2. `status`, and the value the schema still does not have

ADR-0023 carries a red gate step, an escalated blocking finding and an explicit list of what
its harness does not prove; ADR-0034 carries a named superseding trigger at phase 10.
`KbFrontmatter`'s `status` enum still has no "accepted, provisional", and
`kb-open-question-adr-status-vocabulary-001` records exactly that gap. The convention that
question observes is followed rather than answered: `status: accepted`, with every
qualification and its falsifier folded into `summary` and into a body heading. **No edit to the
status-vocabulary atom.** Fifth wave running.

### 3. `authority_tier`, `phase`, `reversibility`

`authority_tier: decision` for Ops 4 and 5, mirroring all twenty-seven siblings.
`authority_tier: note` for every `reference` and `open_question` atom, mirroring every atom in
those two layers. `phase: 9` for both decisions and for Ops 1, 2 and 3 — this is phase 9's
wave, and the phase-8 census in Op 10 is the one atom that belongs to a different one (a
`reference` atom carries no `phase` key at all, which is correct and is why the distinction
costs nothing). `reversibility` is stated per op.

### 4. `source_paths` keeps the intake path

Every atom this wave writes or amends carries its originating `.kb/_intake/…` path, even though
the ingest clears `_intake` afterwards. The path documents provenance; the git history holds
the file. Wave 1's rule, unchanged, and it is why Op 4 lists **two** intake files (CL-1) and
Op 1 lists two (CL-2).

### 5. `last_reviewed: 2026-08-20` on everything this wave touches

Including the five mutated open questions. On Ops 8, 9 and 11 that is one of only three or four
fields that move.

### 6. What is *not* extracted

No `crates/**` change. **No `spec/SPECIFICATION.md` edit** — every file says so in its own
words: no clause amended, no maturity marker moved, nothing `[FROZEN]` approached. **No wire
format change**, which three of the five files state independently. **No `deny.toml` edit** —
Adjudication 4. **No layer README is edited**, though `cf40-c5` scores 80 against
`.kb/decisions/README.md`: that README is the authority being obeyed.

Three mutable non-atom obligations are named by the intake and are **not** this wave's `.kb/`
ops. They are recorded here so a reader who finds them unperformed can see they were not
forgotten:

- **`RUNBOOK.md:4394-4396`** instructs that ADR-0001's provisional marker be retired and this
  adapter cited. The instruction is stale — the marker was lifted at phase 1 and ADR-0008
  records it. ADR-0023's body records the refusal; the runbook line itself is a mutable file
  and is the integration step's to correct or to leave with a note.
- **`RUNBOOK.md:302`** carries phase 9's ADR-0023 queue row, deliberately untouched by the
  implementing story because *"striking the queue row before the atom exists would point a
  reader at a path that is not there"* (`_implementation.md:1755-1757`). After this wave the
  path exists.
- **Project AC-005's wording** (*"the bound added, or ADR-0009's deferral confirmed"*) predates
  ADR-0009's acceptance. The ES-6 file asks for it to be corrected *in the record*. The record
  is `.bklg/`, and a backlog spec is not a KB atom.

### 7. Reciprocal links are the Maps phase's, not the atom author's

Every new atom is authored with **outbound-only** links. Where a mutual edge is wanted —
`kb-decision-0023` ↔ `kb-open-question-workerd-runner-absent-001`, `kb-decision-0034` ↔ the
CF-40 question — the second half is wired by the Maps phase, as in waves 4 and 5.

---

## Adjudication 1 — the ES-6 verdict is a section, not an atom

The intake explicitly leaves this to the wave and states its own bias. The wave agrees, at
**95**, and the argument is in `00`, CL-1. Two points are worth restating here because they are
what an integrator needs:

**The claim is already in ADR-0023.** `0023-…:80-84` states the `JsThrow`-over-`StringifiedThrow`
decision and its reason, and closes *"See the ES-6 cluster, staged separately."* The ES-6 file
supplies what that bullet points at. A separate atom would leave ADR-0023's rejected-alternative
list incomplete on its own page, which `.kb/decisions/README.md:31-33` forbids in terms.

**The `reference`-split rule does not fire here, and the reason is not size.** Wave 5 split
ADR-0022 from its experiment because the experiment produced a **table of numbers with its own
re-measurement schedule** — the decision above it was expected to move, and the numbers were
expected to be superseded on a different cadence. The ES-6 verdict produces four committed test
names pinned in a gate registry (`xtask/src/proof.rs:854-857`). There is no number to go stale.
Op 4's body carries them and cites the file by path.

**What this must not become.** Folding the verdict into ADR-0023 is not folding it into
`kb-decision-0009`, and `_implementation.md:1713-1718` makes that refusal a condition of the
wave's gate: *"Never fold the ES-6 cluster into `.kb/decisions/0009-error-send-sync.md`."*
ADR-0009 receives **no operation of any kind** — not a `related` edge added to its own
frontmatter, not a line, not a `last_reviewed` bump. The edge is outbound from ADR-0023.

---

## Adjudication 2 — the numbers, and the four digits that mean two things

**ADR-0023 is fixed by an artefact.** `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`
exists, and `_plan.md:169` allocates the number to the story that staged this intake. No
judgement is required.

**The CF-40 resolution has no allocated number, and takes ADR-0034.** Nothing in `.bklg/`
names a number for it; `_plan.md:169` says *"CF-40 and WF-11 **atoms** resolved"*, which names
a layer and not an ADR. So wave 5's third rule applies: highest-taken + 1.

| Decision | Number | Fixed by |
| --- | --- | --- |
| The `SqlStorage` mapping and the off-tokio harness | **ADR-0023** | `references/adr/0023-…`, which already exists, and `_plan.md:169` |
| The fixture contract has no single owner | **ADR-0034** | highest-taken + 1 |

**0024–0028 are not backfilled, and this time none of them is even free.** All five are
reserved by planned projects: ADR-0024 (`_storymap.md:64`, position-visibility mechanism),
ADR-0025 (`_plan.md:196`, ladybug's three answers), ADR-0026 and ADR-0027 (`_plan.md:229-230`,
sync), ADR-0028 (`_plan.md:257`, retention). Taking one would collide with a planned initiative
exactly as wave 5 warned taking 0023 would have.

**The trap this wave sets for its own integrator, stated so it is not sprung.**
`.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md` is prefixed `0034` and **mints no
ADR** — its second sentence says *"nothing here should be ingested as a decision"*, and it
becomes Op 10, a `reference` atom with no `adr_id`. Independently, `.kb/decisions/0034-…` is
where the **CF-40 resolution** lands. Two unrelated things wearing the same four digits in one
wave. The intake prefix is a staging sequence and has never been an ADR number; wave 5 recorded
the same rule after `0033` minted ADR-0022.

---

## Adjudication 3 — why the CF-40 resolution is its own decision atom

Three placements were live and each is refused for a different reason.

**Not a section of ADR-0023.** ADR-0023's own scope note disclaims it — *"CF-39, CF-40 and
WF-11 are discharged or resolved elsewhere"* — and, more strongly, ADR-0023's title already
carries an "and" that it justifies by citing `kb-playbook-one-decision-per-adr-title-001`'s
exception: *"it stops being worth the split when both halves are settled by the same
evidence."* ADR-0023's two halves rest on **one conformance run**. CF-40's ownership rests on
**a three-ADR pattern observed across three phases**. Adding it would break the very exception
the atom invokes for itself, in the same document, which is the most self-defeating shape a
wave could produce.

**Not a `governance` atom.** `.kb/governance/` holds rules about how *this corpus* is run —
its one atom is `rewrite-the-referent-never-the-reasoning`. The fixture contract is a
**specification** artefact: `CF-16` through `CF-40` are clauses in `spec/SPECIFICATION.md`, and
who mints one is an architecture-record question, not a KB-process question.

**Not an `open_question` left standing.** The verdict arrives **taken**, with everything
`.kb/decisions/README.md` asks of a decision: the alternative named (a single named owner, the
way `EventStore` and `ProjectionStore` each have one), the evidence counted (ADR-0015 minting
CF-40 without owning it, ADR-0012 owning CF-39 by adjacency, ADR-0023 amending the contract
without claiming it — three ADRs, no collision), the cost stated (nobody can answer *which
document owns CF-40* without reading three ADRs, and a reader locating a fixture-contract
clause reads §6 by subject), and a reopen trigger identified (phase 10's `POLL_BUDGET`).

So: a decision atom. And the thing it decides is a **position about a pattern**, which is
unusual for this corpus and is why the atom must say so in its own first section — the pattern
is ratified, not invented, and what changed is that it is now recorded rather than
rediscovered.

**What it must not do.** Resolve ADR-0015's self-contradiction by editing one half of it out.
ADR-0015 is accepted; both halves stay, and `kb-open-question-cf-40-ownership-001` keeps its
body quoting both. ADR-0034 adjudicates **between** them from outside, on evidence neither
had.

---

## Adjudication 4 — the `deny.toml` call is not the wave's to make

ADR-0023's intake is unusually direct: *"the atom must choose one and say which … Either is a
decision; leaving it undecided while the gate is red is not."* And `_implementation.md:1629-1633`
lists the `worker` → `async-trait` collision as one of three questions ADR-0023 owns.

**The wave declines, and routes it to Op 2.** The reason is wave 5's Adjudication 4, and it is
the same test: a decision the wave may transcribe arrives with the choice already made. This one
arrives with **both shapes named and neither taken** — by the document that says the atom must
take it. Authoring the choice here would be the wave ratifying an exemption to a **binding
constraint** (`CLAUDE.md`'s constraint 1, ADR-0001) against a **red gate**, with no human, no
record and no rejected-alternative argument beyond the two sentences the intake supplies. That
is what `.kb/decisions/README.md:31-33` forbids: *"A decision recorded without its rejected
options is indistinguishable from an accident."*

**What ADR-0023 does instead**, and it has a precedent one wave old: ADR-0022 recorded **two
subjects as non-verdicts with named owners** rather than settling them, and one of those two
was CF-40. ADR-0023 records the finding — the ban is red, `worker` 0.8.5 and `worker-macros`
depend on `async-trait` unconditionally, and **no happenstance port gains a `Send` bound from
it**, because `worker` uses `#[async_trait]` for its own `DurableObject` trait — and names
`kb-open-question-worker-async-trait-ban-001` as its owner.

**Why the open question is not an invention.** `deny.toml`'s own comment already states the
pending decision, in the file, where no map can find it:

```
`wrappers` and not a blanket allow. … Exempting it **by name** is what keeps the ban real: a
second route into the graph — a manifest that names the crate itself, or any other crate that
pulls it in — is a new wrapper this list does not carry, and the check fails until someone
decides it should.
```

Op 2 gives that sentence a home in the layer that indexes unanswered questions. **It is carried
in `unresolved` as well**, because the intake asked for a verdict and got a question, and the
human should be able to overrule that in one edit.

---

## Adjudication 5 — WF-11 is superseded although one sub-question is unreached

This is the wave's most contestable call and the one most open to disagreement.

The question `kb-open-question-human-readable-encoding-limits-001` asks three things. Phase 9
answers sub-question 1 (**sharpened, and answered in the negative for this store**: no payload
this adapter would accept can fire the falsifier — 36,604,834 bytes against a 1 MiB ceiling, a
factor of 35) and sub-question 3 (**none** — the probe forwards bytes it never inspects). It
explicitly does **not** reach sub-question 2, the choice between a streaming scheme, a declared
capability, and JSON scoped to diagnostics.

**A question with an unreached sub-question is normally left Open.** The rule this wave writes
down for when it is not:

> **A resolution may supersede a question whose sub-questions are not all answered, provided
> every unanswered residual is given a named home — and the resolution atom names each home.**

The residuals here, and their homes:

| Residual | Home |
| --- | --- |
| Sub-question 2 — the eventual fix's shape | **HS-P0017 `replication-identity-and-ingest`**, named by the intake, and it *"needs a decision record first."* A named backlog owner is a task, and `.kb/open-questions/README.md:37-38` refuses a task in this layer in terms |
| The falsifier remains unfired **and unfireable on the gate's runner** | **Op 1**, the `workerd`-runner question, which is where the missing property lives |
| WF-11's `[PROVISIONAL]` marker and its `MUST` | **Unmoved, deliberately.** Op 3's body carries C8's must-not list verbatim in substance: the resolution does not say WF-11 is safe, does not move its marker, does not re-scope its `MUST` |

`superseded` rather than `withdrawn`, as in waves 4 and 5: `withdrawn` would say the question
stopped mattering, and it was answered. `superseded_by` is **not** added — it is a
decision-only key, no `open_question` in the corpus carries one, and inventing it here would put
a key in the corpus that nothing validates and every later wave reads as fact.

**If a reviewer disagrees**, the cheap alternative is to leave the question `accepted` and
append only the dated section. That is a one-field difference and is reversible in one edit,
which is why it is carried in `unresolved` rather than settled by this file's confidence.

---

## Adjudication 6 — ES-6 is narrowed and is explicitly not resolved

The mirror image of Adjudication 5, in the same wave, on a question with the same shape. The
ES-6 verdict is a verdict on **ADR-0009's prediction**, not on **ES-6's rule**.
`store_error_crosses_a_join_handle` remains what
`kb-open-question-es-6-unwritable-rule-001` says it is: unwritten, unowned, and named by a
`[FROZEN]` clause. Three independent instructions say so — the intake's own *"Do not let the
wave read this document as answering that question"*, the atom's own *"What is not decided"*,
and `_implementation.md:1716-1718` as a gate condition.

So Op 8 is an **annotation, not a resolution**: `status` stays `accepted`, the bullet on
`open-questions-index.md` stays **Open**, and the dated section says what phase 9 settled and
what it did not. The atom's body already contains a paragraph of exactly this kind — *"Both
ADRs are now imported as decision atoms … and between them they narrow this question without
closing it"* — so the shape is the atom's own, not this wave's invention.

**Why the annotation is worth performing at all**, rather than a bare `related` edge: without
it, the next reader meets an atom saying ADR-0009 *"makes the rule writable"* and no record that
the rule's premise has since been observed under execution on a real `!Send` error. The
scheduling gap is unchanged; the evidence behind it is not.

---

## Adjudication 7 — ADR-0034 has no `references/adr/` record, and ADR-0023 does

`references/adr/` stops at 0023 plus 0029/0030, verified. ADR-0023 has a record; ADR-0034 does
not, and its evidence is the intake document plus the three accepted atoms it cites plus
`crates/happenstance-cloudflare/tests/support/mod.rs`.

This does not block the wave, and it is not silently normalised — wave 5's Adjudication 8 is
the precedent, three atoms deep. What it means is that **ADR-0034 is the canonical form rather
than a summary of one**, so its body carries the rejected alternative in full rather than
pointing at a section number, and its `source_paths` names the evidence it actually has.

---

## Adjudication 8 — the three store-limit numbers are not adjudicated by anything staged

`_implementation.md:1629-1635` names three questions ADR-0023 owns. Two are settled by the
staged documents: the `workerd`-versus-shim substitution (ADR-0023 ratifies the shim harness,
states what it does not prove, and escalates the runner — Op 1), and the `async-trait`
collision (Adjudication 4 — Op 2). **The third is not.**

The question is *"the three store-limit numbers (AC-001/AC-002/AC-003 — declare `None` with the
falsification finding, hold, or ratify the 'declared refusal policy' reading)"*, and slice 3's
surviving finding is that they were **read off a platform page**. What the staged documents
say: the CF-40 file **reports** `CloudflareFixture` declaring `MAX_EVENT_DATA_LEN = 1 MiB`,
`MAX_TAGS_PER_EVENT = 1024`, `MAX_EVENTS_PER_BATCH = 1024` and a real SQLite trigger for
`MID_BATCH_FAULT`, and uses that as evidence for an **ownership** argument. Neither it nor
ADR-0023 argues the three arms, names a loser, or takes a position on provenance.

**The wave does not invent an atom for it and does not let ADR-0023 imply one.** ADR-0034 may
state the numbers as *observed fact about the fixture* — which is what its evidence needs —
and must not state that they are ratified. Carried in `unresolved`. `_implementation.md:1746-1748`
already names where it surfaces: slice 3's review is owed, and a re-launch after the wave
re-enters it *"because the resolution will exist."*

---

## Op 1 — no `workerd`-class runner exists inside the gate

**op:** `create_new` · **kind:** `open_question` · **classification:** `extends`
**destPath:** `.kb/open-questions/no-workerd-class-runner-in-the-gate.md`
**sourceFiles:** `.kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`,
`.kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md`

**Why it exists** — `00`, CL-2. One unowned gap reached from two directions in two files, with
two observed consequences and a real tradeoff. **Why it is not a paragraph inside ADR-0023**:
wave 3's rule — a decision naming a gap is not the same knowledge as the question that owns it
— plus `.kb/decisions/README.md:41-43`, and the fact that WF-11's reference atom needs a
citable owner for *"what would supply it"* that is not a decision atom's prose.

**Why it is not a task.** No backlog item owns it; `_implementation.md:1738-1740` records that
whether (c) is acceptable *"or a deferral until a real runner exists"* was *"not settled here"*,
and the tradeoff is genuine — `workerd` has no Windows-native story and is versioned by a Node
lockfile this repository does not own, where every other gate tool is rustup-pinned or
`cargo install`ed from `Cargo.lock`.

### Proposed frontmatter

```yaml
id: kb-open-question-workerd-runner-absent-001
title: The gate executes every Cloudflare rule on a shim, and nothing owns the runner it is not
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Phase 9 runs the whole conformance suite against a real Durable Object SqlStorage mapping on
  wasm32-unknown-unknown, under wasm-bindgen-test-runner over Node against a node:sqlite-backed
  DurableObjectState shim shipped in crates/happenstance-cloudflare/src/host.rs, inside one
  cargo xtask ci. A workerd-class runner inside the gate was considered and is not rejected on
  merit — it is an escalated blocking finding: workerd is an external binary with no
  Windows-native story, versioned by a Node lockfile this repository does not own, where every
  other gate tool is either rustup-pinned or cargo installed from Cargo.lock. What is true today
  is that the harness proves the mapping and proves nothing about the platform: no isolate, no
  eviction, no hibernation, no I/O gate, no event loop re-entering the object mid-await, and none
  of the platform's own storage ceilings. Two consequences are already observed rather than
  feared. ADR-0023 states the exclusion list in its own body rather than leaving it to be
  discovered. And WF-11's memory-ceiling falsifier could not be made to fire at all — a Node
  isolate has no per-isolate memory cap, which is exactly the property the falsifier needs, so
  the verdict is that the condition is not constructible on this runtime. What is not decided is
  whether a workerd-class runner ever enters cargo xtask ci, what it would cost the gate's
  single-command property to admit one, and what stays unproven for as long as it does not.
  Forced by the next clause that needs a platform behaviour rather than a storage behaviour, and
  by phase 12, where first publish turns "conformant on Cloudflare" into a promise.
depends_on: []
related:
  - kb-decision-0010
  - kb-decision-0016
source_paths:
  - .kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - .kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - crates/happenstance-cloudflare/src/host.rs
  - crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs
  - xtask/src/proof.rs
last_reviewed: 2026-08-20
```

`related` is deliberately short and outbound-only: `kb-decision-0010` (the suite's own proof
obligation, which is the standard this harness is held to) and `kb-decision-0016` (WF-11's
owning decision). `kb-decision-0023` and `kb-reference-wf-11-memory-ceiling-verdict-001` are
**not** listed — they do not exist when this op runs; the Maps phase wires them inbound.

**Body:** the layer README's four headings — *What is true today* (the runner, the shim, the
three-row registry, and the exclusion list quoted from the crate root), *What is not decided*,
*What forces it*, *Ordered sub-questions* (whether a `workerd`-class runner enters the gate at
all; if not, whether the unproven set is enumerated somewhere a gate can see it; whether
`AC-011`-shaped verdicts of "not constructible on this runtime" are acceptable indefinitely).

**mapsImpact:** `openQuestionIndex: true`, `domainMap: true` (a Cloudflare-adapter bullet),
`decisionMap: false`.

---

## Op 2 — `cargo deny check bans` is red, and the exemption is unratified

**op:** `create_new` · **kind:** `open_question` · **classification:** `requires-new-decision`
**destPath:** `.kb/open-questions/deny-bans-red-on-the-worker-dependency.md`
**sourceFiles:** `.kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`

Adjudication 4. **Why not merged with Op 1** (score 40): a runner the gate does not have and a
transitive dependency the gate's ban forbids are different subjects with different settling
conditions.

### Proposed frontmatter

```yaml
id: kb-open-question-worker-async-trait-ban-001
title: The worker dependency re-opens deny.toml's async-trait ban, and neither shape is chosen
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Taking the real worker crate in place of the hand-written stand-in was reversed on a
  measurement rather than on taste, and its price is recorded rather than hidden: the graph
  gains about 40 crates, cargo deny check licenses advisories stays green, and cargo deny check
  bans is red. deny.toml bans async-trait under ADR-0001, which chose trait_variant over it
  because #[async_trait] injects + Send and forecloses wasm32; worker 0.8.5 and worker-macros
  depend on async-trait unconditionally. What is true today is that deny.toml's wrappers list
  names exactly one crate, wasm-bindgen-test, reached only as a dev-dependency of the conformance
  harnesses and present in no published artifact — and the file's own comment states that a
  second route into the graph is a new wrapper the list does not carry and that the check fails
  until someone decides it should. Taking worker as a production dependency is that second route.
  What the red ban does not mean, since it invites the wrong inference: worker uses
  #[async_trait] for its own DurableObject trait, no happenstance port gains a Send bound from
  it, and happenstance-core still declares each port once without a Send bound and lets
  trait_variant derive the second flavour. What is not decided is which of two shapes is taken —
  ratify a wrappers entry naming worker and worker-macros with the argument written into
  deny.toml beside it, or refuse and let the ban stay red with the exception recorded. Either is
  a decision; leaving it undecided while the gate is red is not. Forced now: publish-ready-crate
  cannot claim a green gate while the ban is red, and its AC-012 and its project's DoD both say
  so.
depends_on: []
related:
  - kb-decision-0001
  - kb-decision-0029
source_paths:
  - .kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - deny.toml
  - xtask/src/main.rs
last_reviewed: 2026-08-20
```

**Body:** *What is true today* (the ban, its scoped `wrappers` entry quoted, the two routes, and
the honest note `deny.toml` already carries that `cargo deny` is a probed step so
`crates/happenstance/tests/flavours.rs` is the unskippable guard), *What is not decided* (the
two shapes, stated as the intake states them), *What forces it* (`publish-ready-crate`'s AC-012
and the project DoD), *Ordered sub-questions* (ratify or refuse; if ratify, whether the entry
is scoped to `worker`/`worker-macros` by name or whether a broader policy is owed; whether the
`flavours.rs` guard is sufficient to make the ban's strength honest either way).

**A note for the integrator.** `related: kb-decision-0029` is not obvious and is deliberate:
ADR-0029 is the corpus's worked example of a *dependency* forcing a workspace-level constraint
to move, and of that move being recorded rather than made in silence. It is the nearest
precedent in shape for whichever way this resolves.

**mapsImpact:** `openQuestionIndex: true`, `domainMap: false`, `decisionMap: false`.

---

## Op 3 — WF-11's memory-ceiling verdict

**op:** `create_new` · **kind:** `reference` · **classification:** `extends`
**destPath:** `.kb/reference/wf-11-memory-ceiling-verdict-2026-08.md`
**sourceFiles:** `.kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md`

**Why `reference` and not `decision`.** The intake's own must-not list settles it: the atom may
not say WF-11 is safe, may not move its marker, may not re-scope its `MUST`, and may say nothing
about what the wire format should become. A document forbidden to commit anything is evidence.

**Why `create_new` and not an append to `kb-reference-wire-format-measurements-001`** (score
**88**, the highest the wave declines): `.kb/reference/README.md:19-27` and `:35-38`. That atom
is a 2026-08-09 snapshot of `experiments/wire-format/` on the host toolchain; this is a
2026-08-19 measurement on `wasm32-unknown-unknown` inside the gate. **The reproduction's entire
value is that it is a second, independent observation** — fold it into the first and the thing
it proves is destroyed.

### Proposed frontmatter

```yaml
id: kb-reference-wf-11-memory-ceiling-verdict-001
title: WF-11's falsifier fired at, and the memory ceiling it needs is not constructible on this runtime
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-9 measurement that answers WF-11's peer condition, run 2026-08-19 from
  crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs on wasm32-unknown-unknown inside
  cargo xtask ci, choosing among three admissible shapes through an enum rather than through
  prose. The verdict is (c), the condition is not constructible on this runtime, and it is a
  verdict because it names what is missing rather than reporting an absence: the staircase asked
  the host for 2,047 pages and was granted every one, taking linear memory to 2,169 pages =
  142,147,584 bytes — past Cloudflare's own documented 128 MiB per-isolate limit — refusing
  nothing, and the walk stopped on the probe's own page budget rather than on a refusal. The
  cause is the runner: wasm-bindgen-test-runner over Node against a node:sqlite-backed
  DurableObjectState shim, and a Node isolate has no per-isolate memory cap, which is exactly
  the property the falsifier needs. What would supply it is a real Workers isolate or a runner
  flag capping the linear memory a test module may grow to. The arithmetic changes the question
  even for a real isolate: at 128 MiB the firing payload is 36,604,834 bytes, peak being payload
  times 11/3 — the payload plus its roughly 4/3 rendered string plus the roughly 4/3 serialiser
  buffer — which is thirty-five times the 1 MiB MAX_EVENT_DATA_LEN this adapter's fixture
  declares, so no payload this store would accept can fire it and it can only fire on a payload
  another store accepted and this peer is asked to forward. The category finding is untouched and
  was re-confirmed at six sizes: serde's data model has no streaming entry point for a string, so
  any human-readable payload encoding materialises whole. The published cost table reproduced
  exactly on wasm32 from the published seed — a 348,160-byte payload costs 464,218 bytes through
  the human-readable path against 348,163 through the binary one — and the memory claim held at
  31 pages against 17 on identical bytes, with the cheaper path run first so the comparison is
  handicapped against the claim.
depends_on: []
related:
  - kb-decision-0016
  - kb-decision-0003
  - kb-reference-wire-format-measurements-001
  - kb-open-question-human-readable-encoding-limits-001
  - kb-open-question-workerd-runner-absent-001
source_paths:
  - .kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md
  - crates/happenstance-cloudflare/tests/wf11_memory_ceiling.rs
  - crates/happenstance-cloudflare/tests/support/
  - xtask/src/proof.rs
  - references/adr/0016-the-wire-format.md
last_reviewed: 2026-08-20
```

**Body sections:** `## What was measured, and against what` (the test file, the target, the
runner, the date — the dating rule is satisfied **in the body**, not only by `last_reviewed`),
`## The verdict, and why (c) is a verdict` (the three-variant enum; the missing element; what
would supply it), `## The number that changes the question` (the 11/3 arithmetic and the 35×
gap), `## The category finding, re-confirmed rather than re-derived` (six sizes; the reproduced
table cited against `kb-reference-wire-format-measurements-001` by id),
`## What this does not say` (WF-11 is not safe, its marker does not move, its `MUST` is not
re-scoped, and nothing here bears on what the wire format becomes — that is HS-P0017's and
needs a decision record), `## Sub-question 3, answered: ADR-0003 is observed, not tested`.

**mapsImpact:** `domainMap: true`, `openQuestionIndex: false` (Op 7 carries it),
`decisionMap: false`.

---

## Op 4 — ADR-0023, the `SqlStorage` mapping and the off-tokio harness

**op:** `create_new` · **kind:** `decision` · **classification:** `extends`
**destPath:** `.kb/decisions/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`
**sourceFiles:** `.kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md`,
`.kb/_intake/es-6-verdict-against-adr-0009s-prediction.md`

**Why `create_new`.** No accepted atom owns the proposition. `kb-decision-0001` scores 70 and
is the nearest — it owns the *two-flavour port design*, not how one adapter maps onto one
platform's storage — and it is immutable. ADR-0023 has its own `adr_id`, its own long-form
record and its own phase. **Why `supersedes: null`**: nothing is superseded. The one document
that instructs an edit to an accepted atom (`RUNBOOK.md:4394-4396`) is a runbook, and the
refusal is recorded in the body.

**Two intake files, one atom** — Adjudication 1.

### Proposed frontmatter

```yaml
id: kb-decision-0023
title: The SqlStorage mapping and the off-tokio harness, settled by one body of evidence
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0023
reversibility: medium
phase: 9
supersedes: null
superseded_by: null
summary: >-
  How an event store maps onto a Cloudflare Durable Object's SqlStorage, and what harness proves
  it, given that neither tokio nor a thread is available. The title's conjunction is the
  exception kb-playbook-one-decision-per-adr-title-001 states rather than the smell it names:
  both halves are settled by one body of evidence — the conformance suite executing against the
  real worker bindings, off tokio, inside one cargo xtask ci. The mapping's four modelled
  properties are now checked against real bindings rather than a stand-in. exec is a plain fn
  returning Result<SqlCursor>, no future and no connection to acquire, which is the one storage
  in the workspace for which EventStore::read not being async is free rather than awkward —
  ADR-0001's two-trait design paid off rather than tolerated. The cursor is not a snapshot and
  that is preserved rather than papered over: SqlError::CursorInvalidated reports it, and
  ADR-0011's ceiling-first-then-page mechanism is what keeps a cursor off a suspension point.
  Everything is held !Send and !Sync through an Rc, because worker declares unsafe impl Send for
  its storage and cursor and holding either bare would hand this crate Send-ness through an
  escape hatch its own lint policy denies, with four probes including the_probe_is_not_vacuous
  stopping that becoming a claim nobody checks. Shared state is reached through a RefCell that is
  tried, so a re-entrant borrow reports rather than taking the object down. The thrown value is
  retained rather than stringified at the boundary, and ES-6 is the evidence for that: four
  reconstruction tests executed on wasm32 and pinned in xtask/src/proof.rs's registry show a
  caller recovering constraint violation from transport fault without Error carrying Send + Sync,
  so ADR-0009's decision holds and this adapter is the evidence for it rather than the exception
  to it. The harness's shape is a finding rather than a choice: the suite executes on
  wasm32-unknown-unknown under wasm-bindgen-test-runner against a node:sqlite-backed
  DurableObjectState shim, driven by one row added to a three-row executed-target registry,
  inside one cargo xtask ci — replacing the runbook's queued vitest-pool-workers CI job, which
  loses to the requirement that the run be in the same run as the rest of the gate. What the
  harness does not prove is stated rather than left to be discovered: no isolate, no eviction, no
  hibernation, no I/O gate, no event loop re-entering the object mid-await, and none of the
  platform's storage ceilings. ADR-0001 is cited and not lifted — its provisional marker was
  already lifted at phase 1 and ADR-0008 records it, so RUNBOOK.md's instruction to retire it
  again is read and not obeyed. Two subjects are recorded as non-verdicts with named owners
  rather than settled here: a workerd-class runner inside the gate, and the red cargo deny check
  bans that taking the worker dependency produced.
depends_on:
  - kb-decision-0001
  - kb-decision-0011
related:
  - kb-decision-0008
  - kb-decision-0009
  - kb-decision-0010
  - kb-decision-0012
  - kb-decision-0015
  - kb-decision-0016
  - kb-decision-0022
  - kb-open-question-workerd-runner-absent-001
  - kb-open-question-worker-async-trait-ban-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-open-question-cf-40-ownership-001
  - kb-reference-wf-11-memory-ceiling-verdict-001
  - kb-playbook-one-decision-per-adr-title-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - .kb/_intake/es-6-verdict-against-adr-0009s-prediction.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - crates/happenstance-cloudflare/src/sql_storage.rs
  - crates/happenstance-cloudflare/src/host.rs
  - crates/happenstance-cloudflare/src/lib.rs
  - crates/happenstance-cloudflare/tests/durable_object_conformance.rs
  - xtask/src/proof.rs
  - deny.toml
  - RUNBOOK.md
last_reviewed: 2026-08-20
```

**Edge justification.** `depends_on: kb-decision-0001` — the two-flavour design this adapter is
the first real-runtime evidence for; `kb-decision-0011` — ceiling-and-page is what makes a lazy
read expressible on an invalidating cursor, so the mapping is unreadable without it.
`related: kb-decision-0008` (records the phase-1 lift, which is why the runbook's instruction is
stale), `kb-decision-0009` (the ES-6 verdict's subject — **an outbound edge only; ADR-0009
receives nothing**), `kb-decision-0010` (the standard the harness is held to),
`kb-decision-0012` and `kb-decision-0015` (CF-39 and CF-40, the fixture-contract clauses this
adapter is the first to satisfy in full), `kb-decision-0016` (WF-11), `kb-decision-0022` (the
`index_arms()` rejection, agreed one adapter over), the four questions, the WF-11 verdict, the
playbook whose exception the title invokes, and the governance atom that makes C4's refusal a
citation rather than an edit.

**Body sections:** `## The one question, and why its title carries an "and"` (C1, citing the
playbook's exception in terms — the next reader will check the conjunction and should find the
exception already cited), `## Decision — the mapping` (the four properties, each with the test
or type that holds it), `## Decision — the harness` (the shape that landed, and the three
alternatives that lost on merit), `## What the harness does not prove` (the exclusion list,
stated positively), `## The ES-6 verdict, and why it lives here` (the four reconstruction tests
by name; the negative controls — two wrong classifiers compiled into the *real* `classify_write`
and rejected by two and three tests; that no public item was added), `## Alternatives rejected`
(the ceiling capture, `index_arms()`, `StringifiedThrow`, the hand-written stand-in with its
measured price), `## ADR-0001 is cited, not lifted` (C4, with the reason the refusal is written
down), `## Two non-verdicts with named owners` (the `workerd` runner → Op 1; the red ban →
Op 2), `## What this decision does not touch` (C6).

**Three things the atom must get right, from the provenance check:**

1. **The registry is three rows and this adapter added one.** `xtask/src/proof.rs:589-600` says
   *"Three rows, and the shape is the deliverable"*, and that `every-rule-under-workerd`
   *"adds the Cloudflare conformance target here … and inherits the runner wiring, the version
   check, the exhaustive enumeration check and both gate steps without writing any of them
   again."* Write "one row in a three-row registry", or say nothing about the count.
2. **The `deny.toml` finding is recorded and not decided.** Adjudication 4. The atom must not
   contain the words that would read as a ratification.
3. **The three store-limit numbers are not ratified here.** Adjudication 8.

**mapsImpact:** `decisionMap: true` (a new `## 2026-08-20 …` wave section and one row),
`domainMap: true` (a Cloudflare-adapter section — the domain map has no phase-9 entry yet),
`openQuestionIndex: false` (Ops 1, 2 and 8 carry it).

---

## Op 5 — ADR-0034, the fixture contract has no single owner

**op:** `create_new` · **kind:** `decision` · **classification:** `requires-new-decision`
**destPath:** `.kb/decisions/0034-the-fixture-contract-has-no-single-owner.md`
**sourceFiles:** `.kb/_intake/cf-40-fixture-contract-ownership-resolution.md`

Adjudications 2, 3 and 7. **Why not merged into ADR-0015** (score 65): accepted, immutable, and
it is where the contradiction physically is. **Why not into ADR-0023** (35): its scope note
disclaims CF-40, and its title's exception does not extend to a different body of evidence.

### Proposed frontmatter

```yaml
id: kb-decision-0034
title: The fixture contract has no single owning document, and CF-40 is ADR-0015's clause
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0034
reversibility: high
phase: 9
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-cf-40-ownership-001 on evidence neither claimant had. First: CF-40 is
  ADR-0015's clause, as its own header and decision-8 prose assert. The hedge in that same
  document's Consequences section — that decision 8 sets out both homes and declines to choose —
  is superseded by use rather than by argument, because two later decisions cite CF-40 to
  ADR-0015 and neither claims it: ADR-0022 records CF-40's clause home as a non-verdict with a
  named owner, twice, and ADR-0023 amends the fixture contract without claiming ownership of it
  either. Neither ADR-0015 nor ADR-0012 nor ADR-0022 is edited to record this; all three are
  accepted, and this atom cites them from outside. Second, and it is the durable half: the
  fixture contract has no single owning document, and that is now a recorded position rather than
  an unanswered one. A CF- clause is minted by the decision that first needs the capability, and
  it carries the reason there, beside the adapter that needed it. The alternative — a single
  named owner for CF- clauses, the way EventStore and ProjectionStore each have one — loses to
  three observations rather than to argument: ADR-0015 minted CF-40 and declined to own it,
  ADR-0012 owns CF-39 and MID_BATCH_FAULT by adjacency, and ADR-0023 is the third amendment
  without a claim. Three ADRs, three phases, no collision. The cost is stated with the position
  rather than discovered later: nobody can answer which document owns CF-40 without reading
  three ADRs, and a reader locating a fixture-contract clause reads the specification's section
  6 by subject rather than one decision record. The evidence that turns the argument into an
  observation is phase 9's CloudflareFixture, the first fixture in the workspace to declare all
  three of CF-40's numeric ceilings and claim CF-39's MID_BATCH_FAULT at once, where every prior
  fixture left all three constants at None and append_reports_exceeded_store_limits reported a
  skip everywhere and certified nothing. What is not settled is the position's next test:
  phase 10's POLL_BUDGET-shaped capability, ADR-0013 section 8's concern, which
  kb-open-question-poll-count-rule-strength-001 owns and which ADR-0013 itself says should be
  decided by whoever owns the fixture contract — a premise this decision answers by denying it.
  If POLL_BUDGET collides with piecemeal minting, that collision is the evidence that would
  supersede this decision, which is why its reversibility is high and its trigger is named.
depends_on:
  - kb-decision-0015
related:
  - kb-decision-0012
  - kb-decision-0022
  - kb-decision-0023
  - kb-decision-0013
  - kb-open-question-cf-40-ownership-001
  - kb-open-question-poll-count-rule-strength-001
source_paths:
  - .kb/_intake/cf-40-fixture-contract-ownership-resolution.md
  - references/adr/0015-validated-identifiers-and-store-limits.md
  - references/adr/0012-append-shape-and-preconditions.md
  - references/adr/0022-append-condition-strategy.md
  - references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
  - crates/happenstance-cloudflare/tests/support/
  - crates/happenstance-testkit/src/fixtures.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-20
```

**`depends_on: kb-decision-0015`** — this atom is unreadable without the clause ADR-0015 mints,
and the dependency is what shows a reader that ADR-0015 is confirmed rather than corrected.
**`supersedes: null` and no flip on ADR-0015**, deliberately and this is the atom's most
consequential frontmatter choice: ADR-0015's admitted set of implementations is unchanged, both
halves of its text stay standing, and a reader landing on a `superseded` ADR-0015 would lose
the validated-identifier decisions that are implemented and in force. The corpus's mechanical
test (`.kb/decisions/README.md:20-24`) is satisfied — this is not even a repair of ADR-0015.

**`reversibility: high` — the wave's proposal, and the source states none.** An adjudication
that ratifies an observed pattern and names the single collision that would overturn it is the
most reversible kind of decision there is; reversing it means one later ADR taking ownership of
the `CF-` range, which nothing has yet needed. Flagged because it is the one `reversibility` in
the wave with no source behind it.

**Body sections:** `## Context` (the question, ADR-0022's non-verdict quoted from
`kb-decision-0022:93-94`, and the Branch B coordination result — HS-P0012 merged one position
ahead, could have minted this and did not, so there is exactly one minting across both
projects), `## Decision — CF-40 is ADR-0015's` (superseded by use, with both later citations
named), `## Decision — the contract has no single owner` (the pattern, the reason it is recorded
now, and the cost stated with it), `## The three observations` (ADR-0015, ADR-0012, ADR-0023 —
and `CloudflareFixture`'s four declarations as the third data point), `## Alternatives rejected`
(a single named owner; moving CF-40 to ADR-0012 beside CF-39), `## What this decision does not
do` (edits no accepted atom; ratifies no fixture's *numbers* — Adjudication 8; settles no clause
and moves no marker), `## The named superseding trigger` (phase 10's `POLL_BUDGET`).

**mapsImpact:** `decisionMap: true` (one row in the 2026-08-20 section),
`domainMap: true` (the testkit / fixture-contract area), `openQuestionIndex: false`
(Ops 6 and 9 carry it).

---

## Op 6 — CF-40's question is answered

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:** `.kb/open-questions/cf-40-fixture-limits-ownership.md`
**sourceFiles:** `.kb/_intake/cf-40-fixture-contract-ownership-resolution.md` · **score: 95**

**Frontmatter delta plus one appended section. The body above it is never touched** — the
intake's own hard requirement: the `git diff` against `main` shows frontmatter hunks and one
appended section, and zero hunks inside the existing prose.

```yaml
status: accepted  →  superseded
related:  + kb-decision-0034            # appended; existing entries kept in order
          + kb-decision-0023
source_paths:
  + .kb/_intake/cf-40-fixture-contract-ownership-resolution.md
last_reviewed: 2026-08-10  →  2026-08-20
summary: + a closing clause, in the voice of the atom's own last sentence:
  "Resolved 2026-08-20 by ADR-0034 (kb-decision-0034), which confirms CF-40 as ADR-0015's clause
  — the hedge in that document's own Consequences superseded by use rather than by argument —
  and records the deeper answer as a position: the fixture contract has no single owning
  document, a CF- clause is minted by the decision that first needs the capability, and the cost
  of that is stated with it. Sub-question 3 is not closed and moves to
  kb-open-question-poll-count-rule-strength-001, where phase 10's POLL_BUDGET is the position's
  named next test."
```

**Body: append one `## Resolved 2026-08-20 — ADR-0034 answers sub-questions 1 and 2` section.**
It must carry, and must not carry more than: which atom answers it; that sub-question 1 is
answered by confirmation rather than by movement, so CF-40 does **not** go to ADR-0012; that
sub-question 2 is answered in the negative — there is no single owner and that is now the
recorded position; that sub-question 3 stays live and its home is named; and that no accepted
decision was edited to produce any of it.

**`superseded` over `withdrawn`**, as in waves 4 and 5. **`superseded_by` is not added** —
decision-only key, no precedent on an `open_question`.

**mapsImpact:** `openQuestionIndex: true` (the bullet at `:170-173` moves Open →
**Superseded**, naming ADR-0034, **annotated in place and never removed** — the intake's
requirement), `decisionMap: false`, `domainMap: false`.

---

## Op 7 — WF-11's question is answered on the runtime it named

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:**
`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md`
**sourceFiles:** `.kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md` ·
**score: 95**

Adjudication 5. **Frontmatter delta plus one appended section; the body above it is never
touched** — the intake requires a frontmatter-only diff against `main` in its own words, and
the appended dated section is the shape both prior waves used for a resolution.

```yaml
status: accepted  →  superseded
related:  + kb-reference-wf-11-memory-ceiling-verdict-001
          + kb-open-question-workerd-runner-absent-001
source_paths:
  + .kb/_intake/wf-11-human-readable-encoding-measured-on-this-runtime.md
last_reviewed: 2026-08-10  →  2026-08-20
summary: + a closing clause:
  "Answered 2026-08-20 by kb-reference-wf-11-memory-ceiling-verdict-001: the condition is not
  constructible on the runtime this question assigned it to. The staircase was granted every page
  it asked for, past Cloudflare's documented 128 MiB, because the gate's runner is
  wasm-bindgen-test-runner over Node and a Node isolate has no per-isolate memory cap. Even on a
  real isolate the firing payload is 36,604,834 bytes, thirty-five times this adapter's declared
  1 MiB ceiling, so the falsifier can only fire on a payload another store accepted and this peer
  is asked to forward. The category finding is untouched and was re-confirmed at six sizes.
  Sub-question 2 is not reached and belongs to replication-identity-and-ingest; the missing
  runtime property is owned by kb-open-question-workerd-runner-absent-001. WF-11 stays
  PROVISIONAL: nothing here moves its marker or re-scopes its MUST."
```

**Body: append one `## Answered 2026-08-20 — (c), the condition is not constructible here`
section.** It must say what the verdict is and **what is missing**, must name the two homes for
the residuals, and must not say WF-11 is safe, that its marker moves, that its `MUST`
re-scopes, or anything about what the wire format should become.

**mapsImpact:** `openQuestionIndex: true` (the bullet moves Open → **Superseded**, annotated in
place), `decisionMap: false`, `domainMap: false`.

---

## Op 8 — ES-6 is narrowed and stays Open

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:** `.kb/open-questions/es-6-names-an-unwritable-rule.md`
**sourceFiles:** `.kb/_intake/es-6-verdict-against-adr-0009s-prediction.md` · **score: 92**

Adjudication 6. **`status` does not move. The index bullet stays Open.**

```yaml
status: accepted            # UNCHANGED — this is the point of the operation
related:  + kb-decision-0023
source_paths:
  + .kb/_intake/es-6-verdict-against-adr-0009s-prediction.md
last_reviewed: 2026-08-10  →  2026-08-20
summary: + a closing clause:
  "Narrowed and not closed on 2026-08-20 by ADR-0023: ADR-0009's prediction was judged against a
  real !Send error carrying a live JavaScript value — the first runtime that can produce one —
  and it held, a caller recovering constraint violation from transport fault without Error
  carrying Send + Sync. store_error_crosses_a_join_handle is still unwritten, still unowned, and
  still named by a FROZEN clause; the scheduling gap this atom describes is unchanged."
```

**Body: append one short `## Narrowed 2026-08-20 — ADR-0009's prediction judged, the rule still
unwritten` section**, in the register of the atom's existing *"between them they narrow this
question without closing it"* paragraph. It carries: what ADR-0023 supplies (the four
`es6_reconstruction` tests, executed on `wasm32` and pinned in `xtask/src/proof.rs`); that the
verdict is about **ADR-0009's decision**, not ES-6's rule; and one sentence stating that the
rule remains a testkit change against ADR-0009's marker that no phase owns.

**mapsImpact:** `openQuestionIndex: true` (the bullet stays **Open** and gains *"narrowed
2026-08-20 by ADR-0023"* — the index must not read as though it moved),
`decisionMap: false`, `domainMap: false`.

---

## Op 9 — the poll-count question loses a premise

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:** `.kb/open-questions/poll-count-bounds-the-visibility-rule.md`
**sourceFiles:** `.kb/_intake/cf-40-fixture-contract-ownership-resolution.md` · **score: 75**

The smallest operation in the wave, and it exists because that atom's body quotes ADR-0013
saying the `POLL_BUDGET` call *"should be made by whoever owns the fixture contract, with an
adapter in front of them."* **ADR-0034 answers that clause by denying its premise**: there is
no such owner, and the decision that needs the capability mints it. A question carrying an
unanswered premise the corpus has since answered is a question whose next reader re-derives the
answer.

```yaml
status: accepted            # UNCHANGED — stays Open, owned by phase 10
related:  + kb-decision-0034
source_paths:
  + .kb/_intake/cf-40-fixture-contract-ownership-resolution.md
last_reviewed: 2026-08-10  →  2026-08-20
summary: + one clause:
  "Amended 2026-08-20: ADR-0034 records that the fixture contract has no single owning document,
  so ADR-0013's 'whoever owns the fixture contract' has no referent — a POLL_BUDGET capability
  would be minted by the decision that needs it, and this question is that position's named next
  test."
```

**Body: no new section.** One clause in `summary` and two frontmatter fields. Appending a
section here would restate ADR-0034 inside a question that does not own it, which is the "never
copy paragraphs into a second atom" rule.

**mapsImpact:** `openQuestionIndex: true` (the bullet stays **Open**, gaining the ADR-0034
pointer), `decisionMap: false`, `domainMap: false`.

---

## Op 10 — the phase-8 specification reconciliation census

**op:** `create_new` · **kind:** `reference` · **classification:** `extends`
**destPath:** `.kb/reference/phase-8-specification-reconciliation-census.md`
**sourceFiles:** `.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md`

**Why `create_new` rather than an append to `kb-reference-phase-4-5-spec-reconciliation-001`**
(score 70): the reference README's dating rule, and a stronger reason specific to this pair —
**401 checked / 80 anchored is a number whose entire meaning is comparison against 358 / 69 at
an earlier commit.** One atom holding both is a mirror someone must keep current; two atoms are
a series, and the sibling's shape is the template. **Why not a `playbook`**: wave 5's
Adjudication 5, applied again — these are findings stated with the conditions that produced
them, and `kb-playbook-anchoring-citations-001` already holds the method.

**Why no ADR, and the atom must say so.** The file's own second sentence: *"nothing here should
be ingested as a decision — the two arguments below are inputs to an ADR that does not exist
yet."*

### Proposed frontmatter

```yaml
id: kb-reference-phase-8-spec-reconciliation-001
title: The phase-8 specification reconciliation — what one hand-run of the standing criterion cost
kind: reference
status: accepted
authority_tier: note
summary: >-
  The second census of the standing post-phase reconciliation criterion (RUNBOOK.md:3810-3820),
  hand-run for phase 8 against baseline 53a4764 and recorded in the story's own _reconciliation.md.
  What it cost: 8 clauses in the phase's computed range plus 15 further passages whose prose cites
  the phase's crate and had to be read anyway; 3 verdicts unchanged, 5 repaired in range and 15
  of 15 repaired outside it; 0 normative gaps and 2 escalations, neither of them a clause; one
  pass, one context, no tooling written. What it found: 6 stale file:line citations, every one of
  them still resolving and every one pointing at the wrong subject; 4 factually false counts; and
  1 PROVISIONAL falsifier naming an event that had already occurred. Finding 1 is mechanical
  rather than editorial and is the strongest the pass produced: of 401 spec-trace citations
  checked this pass only 80 are anchored to their subject, against 69 of 358 in the phase-4/5
  pass, because subject_before declines whenever the nearest code span is a type name, a quoted
  phrase or another citation — so all six wrong-subject citations went unreported while the tool
  behaved exactly as documented. checked is a coverage number and anchored is the only quality
  number, and a reader taking a green spec-trace as "the citations are right" is reading a claim
  about a fifth of them. Finding 2: the criterion's arithmetic bullet had nothing to close
  against, because kb-decision-0022 states no clause range in any form and RUNBOOK.md:301's queue
  row for it is the only row with no parenthesised range, so the second set had to be derived from
  the phase's own three disagreeing statements — the phase-4 precedent is the identical defect
  from the other side, 35 clauses named against 64 discharged with 29 invisible. Finding 3: of 20
  repairs only 6 were citation line numbers a machine could plausibly catch and 14 were sentences
  that were simply false, and no extension of spec-trace catches "seven" when the answer is
  twelve. Three mechanisations are floated and none is implemented — an explicit anchor comment
  beside a citation whose subject the heuristic cannot derive, a per-run report of the unanchored
  fraction, and a clauses list in a decision atom's frontmatter. No citation baseline was
  committed, no gate step or xtask check was added, no ANCHOR_SLACK was widened, no clause was
  amended, no marker was moved, and no ADR was written.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-playbook-anchoring-citations-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-reference-spec-trace-has-suite-001
  - kb-decision-0022
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/0034-what-the-phase-8-reconciliation-cost.md
  - RUNBOOK.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-08-20
```

**Body sections:** `## What this is a census of` (the criterion, the baseline `53a4764`, the
story's `_reconciliation.md` as the evidence this atom points at and does not copy —
`.kb/reference/README.md:14-17`), `## What it cost` (the table verbatim; the dating rule is
satisfied by the baseline sha **in the body**), `## Finding 1 — a green spec-trace is not
evidence that a citation is right` (the six, named; the `subject_before` mechanism; 80 of 401
against 69 of 358), `## Finding 2 — the arithmetic bullet had nothing to close against`,
`## Finding 3 — the pass's value was in prose nothing checks`, `## Candidate mechanisations,
none implemented`, `## What was deliberately not done` (mirroring the sibling atom's own
closing section).

**One thing the atom must not do:** answer sub-question 3 of
`kb-open-question-post-phase-reconciliation-001`. The `clauses:` key is recorded as a
*candidate*, and the file says answering it in passing *"is what this document refuses to do."*
It is also, independently, an invented-key risk — the authoring rule forbids putting a key
`KbFrontmatter` does not declare on an atom this wave writes, and a census that recommends one
must not be read as licence to add it.

**mapsImpact:** `domainMap: true` (beside the phase-4/5 census), `openQuestionIndex: false`
(Op 11 carries it), `decisionMap: false`.

---

## Op 11 — the post-phase reconciliation question gains its second data point

**op:** `merge_existing` · **kind:** `open_question` · **classification:** `extends`
**destPath / mergeTargetPath:** `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`
**sourceFiles:** `.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md` · **score: 78**

**Stays Open.** One point below the collapse threshold, and the source is unambiguous: *"It
settles nothing. In particular it does not answer any of the five ordered sub-questions."*

```yaml
status: accepted            # UNCHANGED
related:  + kb-reference-phase-8-spec-reconciliation-001
source_paths:
  + .kb/_intake/0034-what-the-phase-8-reconciliation-cost.md
last_reviewed: 2026-08-10  →  2026-08-20
summary: + a closing clause:
  "A second pass has now run, for phase 8, and its census is
  kb-reference-phase-8-spec-reconciliation-001. It supplies evidence for three of the five
  sub-questions without answering any: 14 of 20 repairs were false sentences no gate step could
  catch, which argues against a gate step replacing the pass and for the pass having an owner;
  the clause-range arithmetic had nothing to close against because the phase's sole ADR states no
  range in any form; and the anchored fraction is 80 of 401, against 69 of 358 at phase 4/5."
```

**Body: append one `## Second data point, 2026-08-19 — the phase-8 pass` section**, short and
pointing rather than restating. The atom's own rule applies to itself: *"a question carrying its
own copy of a census is a second census, and the two would drift."* The section carries the
three bearings — on sub-question 1 (checkbox against gate step), sub-question 3 (machine-readable
clause ranges, with the `clauses:` key named as a candidate and not adopted), and sub-question 5
(own ADR against runbook process only) — and **no counts**, which is
`kb-playbook-ratchet-gate-landing-001`'s rule and is also why sub-question 2 gets no new
evidence.

**mapsImpact:** `openQuestionIndex: true` (the bullet stays **Open**, gaining the second
census's pointer), `domainMap: false`, `decisionMap: false`.

---

## What the Maps phase inherits

| Map | Work |
| --- | --- |
| `decision-map.md` | A new `## 2026-08-20 — phase 9` section with **two rows** (ADR-0023, ADR-0034). **No supersession lineage this wave** — the partial-supersession-chain section is untouched, which is worth one sentence there, because five consecutive waves of chain growth make its absence look like an omission |
| `domain-map.md` | A **Cloudflare-adapter domain has no section yet** — the Maps phase decides whether ADR-0023 opens one or joins the contract-ports section. Plus a testkit/fixture-contract bullet for ADR-0034, a reference bullet for Op 3 beside `kb-reference-wire-format-measurements-001`, a reference bullet for Op 10 beside the phase-4/5 census, and a bullet for Op 1 |
| `open-questions-index.md` | **Six bullet changes.** Two new (Ops 1, 2). Two Open → **Superseded**, annotated in place and never removed (CF-40 at `:170-173`, and WF-11 two bullets below it, with the DCB-wire-format bullet between them untouched). Three stay **Open** with an annotation (ES-6, poll-count, post-phase reconciliation) — and the annotation matters most on ES-6, where an index that reads as resolved would be the exact misreading three separate documents forbid |
| Reciprocal links | `kb-decision-0009` receives **nothing** — Adjudication 1. The inbound edges to wire are: Op 1 ← Ops 3 and 4; Op 2 ← Op 4; Op 3 ← Op 4 and Op 7; Op 4 ← Op 5, Op 6 and Op 8; Op 5 ← Ops 6 and 9; Op 10 ← Op 11 |
