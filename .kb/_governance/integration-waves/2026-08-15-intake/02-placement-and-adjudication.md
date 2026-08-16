# Wave `2026-08-15-intake` — placement and adjudication

The ordered action plan. **Two operations, both `create_new`, both `decision`.** No accepted
decision atom is edited, none is superseded, no `status` flips, no open question is opened or
closed, and no `reference` atom is minted.

Order carries no resolution constraint this wave: neither op links to an atom the other creates in a
way that must exist first, because the one edge between them is a mutual `related` (Adjudication 2)
and both are written by this wave. Op 1 runs first on ADR-number order, which is the order the
`decision-map` row block will read in. Both ops are non-mutating creations at unique paths, so they
may be integrated in parallel; the Maps phase runs strictly after both.

```
Op 1  create_new  .kb/decisions/0020-fold-query-agreement.md
Op 2  create_new  .kb/decisions/0021-payload-evolution-and-codec-tag.md
```

## Standing choices, applied to every atom in the wave

### 1. One atom per ADR, and the long-form record keeps the evidence

`references/adr/0020-…md` is 411 lines and `references/adr/0021-…md` is 454. Each atom is the
~100-line canonical form and cites its record by `file:line`. Unchanged from waves 2, 3 and 4, and
it is what `CLAUDE.md`'s *"link the atom; cite the record by `file:line`"* requires. Neither record
is summarised twice, and neither is copied.

### 2. `status`, and the value the schema still does not have

Neither ADR is provisional, so this wave does not exercise `kb-open-question-adr-status-vocabulary-001`
at all — the first wave since 2026-08-10 for which that is true. Both atoms are plainly
`status: accepted`. **No edit to the status-vocabulary atom**, for the fourth wave running, and this
time because there was nothing to qualify rather than because the qualification was routed elsewhere.

### 3. `authority_tier`, `phase`, `reversibility`

`authority_tier: decision` on both, mirroring all twenty-one siblings. `phase: 7` on both, as both
records state. `reversibility: medium` on ADR-0020 and **`low`** on ADR-0021, both as proposed — and
the difference is load-bearing rather than cosmetic: ADR-0021's own record explains that reversing
it after `0.2.0-alpha.1` means re-siting a tag on events already written into users' stores, whose
only legal route is application-level re-emission, which mints new identities (VT-5) and leaves the
old events in place forever. That asymmetry is one of the three reasons the two atoms do not merge
(`00`, CL-1).

### 4. `source_paths` keeps the intake path

Both atoms carry their `.kb/_intake/…` path in `source_paths`, even though the ingest clears
`_intake` afterwards. The path documents provenance; the git history holds the file. Wave 1's rule,
unchanged. Every other `source_paths` entry on both atoms was tested against this worktree and
resolves (`00`, *Provenance check*).

### 5. What is *not* extracted

No change under `crates/**`, `examples/**`, `xtask/**` or `spec/SPECIFICATION.md`. No maturity
marker moved. No `[FROZEN]` clause line-edited — specifically **no amendment to VT-3** and none to
any `ES-*` or `VT-*` clause. No hook added to `EventStore`. No conformance rule written, and in
ADR-0021's case the absence is itself the decision's content: a testkit rule able to observe the
codec tag would prove the tag is visible below the port, which falsifies the decision rather than
verifying it. No supersession of ADR-0016 or of anything else. No `.bklg/` document touched.

### 6. Nothing is routed to `product/`, `design/`, `concepts/`, `playbooks/` or `governance/`

Checked rather than assumed. `product/` and `design/` hold READMEs only, and this wave carries no
persona, journey or interaction pattern — ADR-0020 is an *API* shape decision, and `design/`'s
README scopes that layer to signed-off interaction patterns for a class of surface, which a trait
signature is not. `concepts/` was considered for *"an event type is a stable identity; the payload is
what evolves"* and refused: it is a commitment with a `MUST NOT` behind it, and
`.kb/decisions/README.md:27-29` says a commitment in a `concept` or `playbook` atom is not a
commitment, because those layers carry no immutability. `playbooks/` was considered for the shared
*two admissible constructions of one value* method and refused under Adjudication 6.
`governance/` carries rules about how the corpus is run, and this wave discovers none.

---

## Op 1 — ADR-0020, the fold/query agreement

**op:** `create_new` · **kind:** `decision` · **classification:** `extends`
**destPath:** `.kb/decisions/0020-fold-query-agreement.md`
**sourceFiles:** `.kb/_intake/0020-fold-query-agreement.md`

**Why `create_new` and not a merge.** No accepted atom owns the proposition, and the four that come
closest were each verified against the claim rather than against a title. `kb-decision-0008` is a
lexical collision on the word "derivation" and shares no referent (`00`, scoring rule 5).
`kb-decision-0007` owns the *type* a decision model queries with, and ADR-0020 derives that same type
rather than minting a second. `kb-decision-0015` made the constructor this decision routes its
fallibility through, and is cited for it. `kb-decision-0006` allocated the crate this all lives in.
ADR-0020 has its own `adr_id`, its own 411-line record and its own human sign-off at the
`/redkiln:plan` design gate on 2026-08-12.

**Why nothing is superseded.** `supersedes: null`, and the record proposes no supersession. The one
thing ADR-0020 says about the frozen crate is that it will not touch it: *"no `unwrap` and no edit to
`happenstance-core`."* A decision that routes around a shortfall and logs it does not supersede the
decision that created the shortfall.

### Proposed frontmatter

```yaml
id: kb-decision-0020
title: >-
  A decision model folds a domain enum, and its query is derived on a sealed trait the caller
  cannot override
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0020
reversibility: medium
phase: 7
supersedes: null
superseded_by: null
summary: >-
  A DCB handler names its event set twice - once in the query, once in the fold - and nothing
  checks the two agree; the worked example does exactly this today and the gate is green either
  way. The resolution is structural rather than detective: query() is not on DecisionModel at
  all. The derivation is Boundary::query, on a sealed trait blanket-implemented for every
  DecisionModel and macro-implemented for tuples of arity 2..=8, so there is nowhere to put a
  hand-maintained query. It returns Result<Query, InvalidQuery> because QueryItem::new is
  fallible and happenstance-core is frozen; the other route to that error, an empty EVENT_TYPES,
  is turned into a compile error by a const item evaluated per-monomorphisation rather than left
  to the first read. DecisionModel::scope(&self) -> &Tags, so the validation is paid once in the
  constructor the caller already writes, Tags::from_pairs being the only and fallible way in.
  DecisionModel: Clone, not Default, so the command loop re-folds from the pristine model on
  retry without a default-constructible Tags re-opening the invalid-value hole. The residual Err
  is kept and re-read as "this boundary constrains nothing, which is Query::all() and must be
  said out loud"; commit and commit_with absorb it so the first program writes no extra
  question-mark. No unwrap and no edit to the frozen crate: the shortfall is logged as defect
  candidate D-1, that happenstance-core has no infallible QueryItem constructor for pre-validated
  inputs, with VT-18 as its nearest clause subject and a decision record as its route. Rejected:
  a provided method on DecisionModel (overridable, so hand-maintained again); a free
  derive_query::<M>() (ignorable); an infallible query() (unreachable without an unwrap);
  fallible-without-the-const-assertion (defers a compile-time-decidable mistake to the first
  read); scope() -> Tags by value (unwrap inside an infallible signature); scope() -> &[(&str,
  &str)] (revalidates, moves the error to the read); a Default supertrait (re-opens the invalid
  Tags hole); an exported compose! macro (caller-visible ceremony where a tuple would do).
  DT-2 resolves toward explicit declaration, and the price is on the record: 2.4:1 mapping
  ceremony to domain logic in the first program, carried as the falsifiable prediction that
  AC-013's verdict lands "happenstance-macros is in scope for 0.1" - a consequence, not a second
  decision.
depends_on:
  - kb-decision-0003
  - kb-decision-0006
related:
  - kb-decision-0007
  - kb-decision-0015
  - kb-decision-0021
  - kb-open-question-projection-id-unvalidated-001
source_paths:
  - .kb/_intake/0020-fold-query-agreement.md
  - references/adr/0020-fold-query-agreement.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md
  - crates/happenstance-core/src/query.rs
  - crates/happenstance-core/src/tag.rs
  - examples/course-subscriptions/src/main.rs
last_reviewed: 2026-08-15
```

**Edge justification.** `depends_on: kb-decision-0003` — the fold operates on *decoded* events, which
exist as a category only because the contract carries opaque `Bytes` and pushes encoding upward.
`kb-decision-0006` — there is a typed layer to put `DecisionModel` and `Boundary` in. `related:
kb-decision-0007` — ADR-0007's third shape decision is that a projection nominates events with
`Query`, *the same type a decision model uses*; ADR-0020 derives that type instead of hand-writing
it, so the edge records a claim preserved rather than a premise consumed. **`kb-decision-0015` is
added by this wave** and is justified in Adjudication 3. **`kb-decision-0021` is added by this wave**
and is justified in Adjudication 2. `kb-open-question-projection-id-unvalidated-001` — the atom that
owns the unvalidated-`ProjectionId` question whose doc comment supplies this decision's
two-constructor sentence; cited **outbound only**, and not edited (Adjudication 7).

**Body sections the integrator should write:** `## Context` (the hazard against real code —
`examples/course-subscriptions/src/main.rs:114-125` builds the query and `:140-155` folds the same
three types again behind a `_ => {}` arm, with nothing signalling a divergence, and the asymmetry
that a query wider than the fold is safe while a fold wider than the query silently narrows the log
the append condition protects); `## Decision` (the five claims as one shape, the seal as what makes
the derivation the *only* one, and the arity ceiling of 8 because rustdoc renders one impl block per
arity); `## Where the fallibility went` (`QueryItem::new` fallible at `query.rs:56`,
`InvalidQuery::UnconstrainedItem` re-read as *"this boundary constrains nothing, which is
`Query::all()`"*, `commit`/`commit_with` absorbing it); `## Alternatives rejected` (all eight, each
with the wrong implementation it admits); `## Consequences` (DT-2 toward explicit declaration; the
2.4:1 ratio as a **falsifiable prediction** owned by project closeout at `RUNBOOK.md:525`, not a
second decision; `EVENT_TYPES` ↔ `event_type()` agreement not compiler-enforced and tested by
`assert_domain_event::<E>(&[…])`); `## What this decision does not decide` (defect candidate D-1 with
VT-18 as its nearest clause subject and AC-012's log as its owner, plus one sentence recording that
whether `happenstance-macros` ships is a prediction here and a verdict at closeout).

**The one-shape claim must be quoted from `crates/happenstance-core/src/projection.rs:152-154`** —
verified in this worktree, and the repaired range rather than the `:47-61` still cited across
`.bklg/`. That is the whole `.kb/` consequence of claim `0020-C5`.

**mapsImpact:** `decisionMap: true`, `domainMap: true`, `openQuestionIndex: false`.

---

## Op 2 — ADR-0021, payload evolution and the codec tag's home

**op:** `create_new` · **kind:** `decision` · **classification:** `extends`
**destPath:** `.kb/decisions/0021-payload-evolution-and-codec-tag.md`
**sourceFiles:** `.kb/_intake/0021-payload-evolution-and-codec-tag.md`

**Why `create_new` and not a merge.** The highest-scoring candidate in the wave is
`kb-decision-0016` at **55**, and it is a two-atom score by the method's own definition — same
subject, different question. Adjudication 1 states the split. The other candidates are premises:
`kb-decision-0003` (payloads are opaque and the contract never parses them), `kb-decision-0006` (the
typed layer exists) and `kb-decision-0007` (decoding is strictly above the port), all three of which
this decision consumes rather than restates.

**Why nothing is superseded.** `supersedes: null`. ADR-0021 takes a byte range inside a field
ADR-0016 legislates the *encoding* of, and legislates nothing about that encoding; ADR-0016 takes no
position on what the bytes mean. Neither atom's admitted-implementation set changes. What ADR-0021
declines to inherit — ADR-0016's `reversibility: high` — is not a correction to ADR-0016 but a
refusal to extend a property whose stated basis (the wire format is private and unpublished) does
not hold for events already written into users' stores. Recording that refusal *inside the new atom*
is exactly what keeps it from reading as a silent inheritance.

### Proposed frontmatter

```yaml
id: kb-decision-0021
title: >-
  The codec tag lives in Event::metadata, event types do not carry versions, and upcasting
  happens at decode
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0021
reversibility: low
phase: 7
supersedes: null
superseded_by: null
summary: >-
  Three answers, none of them taken at the design gate, because the public surface is invariant
  under all three - Codec::TAG is a &'static str either way. The codec tag lives in
  Event::metadata, inside a versioned framing region the typed layer owns and no store parses,
  with everything after it the application's own causation and correlation metadata, carried
  through unmodified. The decision turns on VT-3's second half rather than its first: anything a
  store, a peer, a conformance rule or a query must see belongs in EventType or Tags, and
  nothing below the port needs to see the codec tag because decoding is strictly above it. The
  falsifier is name the adapter change this choice forces, applied to both candidates: metadata
  forces none, while Tags obliges every adapter to store, index, match on and budget the tag,
  spending one of the 64 tags every store must accept on every event forever, and makes
  re-encoding change which QueryItems an event satisfies - so a consistency boundary silently
  changes shape as a consequence of a storage-format migration that was supposed to be
  invisible. EventType carries no version suffix: matching is exact equality, so CourseDefined.v2
  is invisible to every query already written and the first upcast makes existing decision models
  read an empty log with their append conditions matching nothing; an event type is a stable
  identity and the payload is what evolves. No read-path hook is needed, and the strategy that
  earns it is decode-time tolerance, which DomainEvent::decode already permits because it
  receives the event type and the raw bytes; its falsifier is an upcast needing information from
  outside the event being decoded, and if that is ever reached the route is a defect entry with a
  clause ID under AC-012, never a line edit of the frozen EventStore. An event with no framing
  region decodes with the codec in hand - Json under commit, C under commit_with - and not
  UnknownTag, because refusing it would make every event written before the typed layer existed
  unreadable with no legal repair, and because that fallback is the mechanism any future
  re-siting would reuse. UnknownTag is left meaning exactly one thing: a tag was written and this
  build cannot honour it. Rejected: the tag in Tags; either-home-is-fine; happenstance owning the
  whole metadata field; a serde-encoded framing region; a bare version suffix; a version suffix
  with a query-naming rule; an unearned no-hook answer; adding a hook to EventStore; UnknownTag
  for untagged events; answering in ADR-0016's terms. Reversibility is low, not high: reversing
  it after 0.2.0-alpha.1 means re-siting a tag on events already written into real stores, which
  VT-3 forbids any adapter from doing on the user's behalf.
depends_on:
  - kb-decision-0003
  - kb-decision-0006
  - kb-decision-0007
related:
  - kb-decision-0016
  - kb-decision-0020
  - kb-open-question-human-readable-encoding-limits-001
source_paths:
  - .kb/_intake/0021-payload-evolution-and-codec-tag.md
  - references/adr/0021-payload-evolution-and-codec-tag.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-core/src/store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-15
```

**Edge justification.** `depends_on: kb-decision-0003` — the contract carries opaque `Bytes` and has
no evolution problem, which is why the typed layer inherits the whole of one. `kb-decision-0006` —
the typed layer is where the framing region's owner lives. **`kb-decision-0007` is promoted from the
intake's `related` to `depends_on`** by this wave; Adjudication 3. `related: kb-decision-0016` —
Adjudication 1. **`kb-decision-0020` is added by this wave** — Adjudication 2.
`kb-open-question-human-readable-encoding-limits-001` — cited **outbound only** to mark the phase-9
seam, and not edited (Adjudication 7).

**Body sections the integrator should write:** `## Context` (a typed layer has an evolution problem
from its first commit — a decoded `Enrolment::Defined { capacity: u32 }` breaks when a field is
added, and a store holding two encodings at once needs a way to tell them apart *that no adapter is
allowed to understand*; AC-005 requires the tag to exist and this record decides where it lives);
`## VT-3, read in both directions` — **the section that decides, and the atom must carry it**: the
first half licences nothing on its own, the second half sites the tag, the prior question is *does
anything below the port need to see it*, the answer is no because decoding is strictly above the
port and `QueryItem::matches` reads `event_type` and `tags` only (`query.rs:112-115`), and VT-3's
`Rejects:` line is the mirror case rather than a counter-example; `## Decision` (the three answers
and the rule they imply, with the framing region's three `must`s and *exact bytes are M3's*);
`## The falsifier, applied to both homes` (the five-row table — store, index, match, budget, migrate
— and the finding that `Tags` forces a change in every adapter not by making one parse the tag but by
obliging all of them to store, index, match on and budget it); `## The accepted cost, discharged`
(one caller shape pays: an application reading `event.metadata()` at the *contract* level gets the
framing bytes and must skip them); `## Alternatives rejected` (all ten); `## The seam against
ADR-0016` (0016 moves the bytes and never reads them, 0021 reads them and never moves them, a
replicated framing region travels opaque and `happenstance-sync` never learns it exists, and
`reversibility: high` **does not transfer**); `## Consequences` (M3 inherits three answers and a rule
rather than a gap; **no conformance rule, and the absence is the decision's own content**; event-type
strings join the public commitment surface because Decision 2 makes them un-renameable; claim 4's
fallback is what makes any future re-siting additive rather than impossible).

**The `EventStore` enumeration must be the corrected one** — `read`, `append`, `head`,
`contains_event_id` (`store.rs:119`, `:213`, `:248`, `:268`), with `read_decision_model` named as the
**free function** at `:321` that it is, not as a trait method. `_design.md`, `_decomposition.md` and
the story's spec all recite it as one; Decision 3 is unaffected because neither spelling contains a
hook, and the atom uses the correct enumeration. A repair, not an amendment.

**mapsImpact:** `decisionMap: true`, `domainMap: true`, `openQuestionIndex: false`.

---

## Adjudications

### 1. ADR-0021 and ADR-0016 stay two atoms, at the wave's highest score

55 is the top of the "same subject, different method" band and the one merge a careless pass would
make: both decisions legislate `Event::metadata`, both are about bytes crossing a boundary, and
`kb-decision-0016`'s summary already contains the sentence *"`Event::data` and `Event::metadata`
encode as standard base64 in human-readable formats and raw bytes otherwise."* A reader could
reasonably expect to find ADR-0021's content there.

It is two atoms, and the intake states the discrimination better than a score can:
**ADR-0016 owns how the metadata bytes cross a peer boundary; ADR-0021 owns what those bytes mean to
the typed layer. ADR-0016 moves them and never reads them; ADR-0021 reads them and never moves
them.** The two are orthogonal in the strict sense — a framing region rides through base64 exactly
as an application's own metadata does, and `happenstance-sync` never learns it exists. Three further
facts make the split structural rather than editorial:

1. **`kb-decision-0016` is accepted and immutable.** Whatever the score, there is no merge that is
   not a gate failure (`redkiln validate --kb` checks each accepted decision body against `HEAD`).
2. **The reversibilities differ, and one of them is derived from a fact the other does not share.**
   ADR-0016 is `high` *because the format is private and unpublished*; ADR-0021 is `low` because its
   subject is bytes already written into users' stores. Merging would force one number over two
   different bases.
3. **The record rejects "answering in ADR-0016's terms" by name**, as its tenth rejected
   alternative — *a second competing answer in a corpus that already has one*. A merge would perform
   exactly the alternative the decision rejects.

`related: kb-decision-0016` on Op 2, outbound. `kb-decision-0016` gains nothing: an accepted
decision's frontmatter is not edited to record an inbound edge, and the map is where the relation
becomes navigable.

### 2. The two new atoms link to each other — the wave's one cross-file finding

Neither intake file proposes an edge between them, and the extract pass did not raise one. The wave
adds a **mutual `related`**, because the two decisions interact in a way that is invisible from
either record alone:

> ADR-0021's Decision 2 charges a price — *an event type is a stable identity; a genuinely new fact
> takes a new type name, and every query that must see it names it deliberately.* Read on its own
> that is a recurring tax: every live type named in every query, forever, which is the exact cost
> ADR-0021 uses to reject *"a version suffix with a query-naming rule."* Under ADR-0020 the price is
> paid in **one** place — the model's `EVENT_TYPES` — because `Boundary::query` derives the query
> instead of a human maintaining it. The tax ADR-0021 accepts is the tax ADR-0020 had already
> collapsed, one slice earlier in the same design.

This is `related` and not `depends_on` in either direction: each decision stands and is readable
without the other, and neither's sentences lose a referent if the other is superseded. It is also
not a merge — the finding is *about* the pair, which is precisely the relation an edge exists to
carry. Both atoms are authored by this wave, so the mutual edge costs no accepted atom an edit.

### 3. Two edges the wave adjudicates against the intake's proposal

Both intake files say in terms that *"the ingest run authors the frontmatter; this is the proposal it
authors from."* Two proposals are changed, and each change is an edge's strength, never a claim.

**`kb-decision-0015` is added to Op 1's `related`.** ADR-0020's claim 3 rests on `Tags::from_pairs`
being fallible and being the only way in — which is not a fact of nature but ADR-0015's decision,
the one that validated every identifier constructor in `happenstance-core`. Leaving the edge out
would leave a reader asking *why is the only constructor fallible* with nowhere in `.kb/` to go.
It is `related` and **not** `depends_on`: the corpus's `depends_on` test, as wave 4 stated it, is
*without this atom the sentence has no referent*, and ADR-0020's sentences keep their referents —
`Tags` and `QueryItem` exist whoever decided their constructors' shape. The record's own heading for
this material is *"The constructors this decision is built out of, and cannot change"* (`:84`), which
is the description of a **cited fact of a frozen crate**, not of a premise being reasoned from.

**`kb-decision-0007` is promoted on Op 2 from `related` to `depends_on`.** ADR-0021's Decision 1
turns on a single premise — *nothing below the port needs to see the codec tag, because decoding is
strictly above the port* — and that premise is ADR-0007's content, not a background fact. Remove
ADR-0007 and the sentence has no referent: there is no decode boundary for "above" to be relative
to, and VT-3's second half stops being answerable. That is the `depends_on` test, met exactly.
Note that the same atom stays `related` on **Op 1** for a different reason and correctly so
(Op 1's edge justification): ADR-0020 preserves a claim of ADR-0007's, while ADR-0021 consumes one.

### 4. The map placements — one wave section, and a **new** domain

Both are collapsed from four claims (`0020-C2`, `0020-C3`, `0021-C5` twice) into two `mapsImpact`
flags, and both need a decision the intake files do not make.

**`decision-map.md` — one new `##` section, two rows.** The map's *Adding a row* section says a new
atom gets a row *"under the wave section that introduced it (open a new `##` section per wave)"*.
One wave, one section — **not** two sections, and **not** rows appended to the existing
`## 2026-08-15 checkpoint-progress ADR (ADR-0030)`, which belongs to a different wave that happened
to run on the same day. The section needs a name that disambiguates it from its same-day sibling, so
date alone will not do: **`## 2026-08-15 typed-layer ADRs (ADR-0020, ADR-0021)`**, placed after the
ADR-0030 section, with the two rows in ADR-number order inside it. The map's ADR numbering is
already non-monotonic across sections (ADR-0029 sits inside the 2026-08-10 import, ADR-0030 before
these two) and the *Adding a row* rule is what makes that correct rather than untidy; one sentence of
section prose should say so, along with the fact that **neither atom supersedes any row on this map**
and that both are phase 7, the first rows on the map above phase 6.

**`domain-map.md` — a new domain section, not an append.** The extract flagged this as a placement
call and it is the right flag. The map has two sections and neither is the typed layer: *Specification
governance & conformance* is about `spec/SPECIFICATION.md` itself, and *Contract ports, conformance,
and the ADR corpus* frames itself around `happenstance-core`'s async ports — its own text says so —
even though ADR-0003, -0006 and -0007 sit in it. The map decides its own case in *Adding a domain*:
*"Append a new `##` section rather than editing this one — a domain is a subject area, not a wave,
and sections should outlive the ingest that first populated them."* Two atoms about `DecisionModel`,
`Boundary`, `Codec` and payload evolution are a subject area, and ADR-0021's *"beside ADR-0020's"*
confirms the intake expects them adjacent.

So: a third section, **`## The typed layer: decision models, codecs, and payload evolution`**, with
both atoms under a **Decisions** heading and a one-sentence orientation each. Two things it must
*not* do. It must not move or copy `kb-decision-0006` or `kb-decision-0007` out of the ports domain —
the map's own summary says entries are not removed, only annotated, and both atoms genuinely belong
where they are; the new section cites them as the boundary it stands on. And it must not restate
either decision: *"the atom itself is the source of truth, not this map."*

### 5. No `reference` atom, for either record — and the criterion rather than the precedent

Both intake files raise the question and both defer it here, citing ADR-0029 as the precedent that
`source_paths` alone can discharge a long record. The precedent is real but it is the weaker
argument, because the corpus also runs the other way: ADR-0013 has
`kb-reference-position-visibility-experiment-001` and ADR-0016 has
`kb-reference-wire-format-measurements-001`. Precedent alone predicts nothing.

The criterion is in `reference/README.md`, and it is about what the layer *holds*: a **measurement**
(what was run, on what, on what date, what it returned), a **census** (a count at a commit, with the
tool that produced it), or a **pointer** into evidence that lives elsewhere. Applied to these two
records:

- **ADR-0021 contains no measurement at all.** Its falsifier — *name the adapter change this choice
  forces* — is an argument, and its five-row table is a table of obligations, not of numbers. There
  is nothing to date and nothing a later count could be compared against. Its own consequences
  section states that there is deliberately **no conformance rule**, so there is not even a test to
  point at.
- **ADR-0020 contains one number, 2.4:1, and it is not evidence.** It is a hand count of a doctest
  in a signed-off design document, and the record itself classifies it as a **falsifiable
  prediction** whose verdict is taken later, over a different artefact — the rewritten example, not
  the doctest — and recorded at project closeout (`RUNBOOK.md:525`). Filing a prediction in
  `reference/` would be filing it as a measurement, which is precisely the reading the record spent
  a paragraph forbidding. It would also violate the dating rule in the way that rule is most
  dangerous: it reads as current, it gets cited as current, and nothing in the repository can detect
  that AC-013's verdict superseded it.

Neither record has a section the pointer case would name, either — no `experiments/` run, no
`references/evaluation/` sweep. Verified by reading both records' headings end to end (`00`,
*Provenance check*). So: **no reference atom**, and the standing rule for a later wave is the
criterion above, not "ADR-0029 did not have one".

### 6. ADR-0021's three-answer title clears `one-decision-per-adr-title`, and earns no edge for it

The playbook warns that an "and" in a decision's title is usually a strong decision and a weaker one
bundled together, and that the weaker half is the one likely to be reversed. ADR-0021's title has two
"ands" and three answers, so the bar has to be applied rather than assumed.

It clears, on the playbook's own test — **different strength** — which the three answers do not
exhibit. They share one reversibility (`low`), one falsifier structure, and one cause: the design
gate handed all three over together *because the public surface is invariant under all three*,
`Codec::TAG` being a `&'static str` whichever way each is settled. That is the opposite of the
bundling pattern the playbook describes, where two answers ride one title because one of them was
never argued. Here all three are argued at length in a 454-line record, and no single one of them
can be reversed without the other two being re-opened: move the tag out of `metadata` and the
untagged-event fallback has no referent; put a version on `EventType` and decode-time tolerance
stops being the strategy that earns "no hook".

**No `related` edge to the playbook.** The record does not cite it, the playbook gains no instance it
does not already carry, and — unlike wave 4's ADR-0030, which was *judged against* two playbooks by
name in its own rejected alternatives — nothing here was decided by consulting it. An edge added
because a wave happened to apply a bar is an edge that says a decision rests on a guideline it never
read.

### 7. Neither cited open question is edited, and "reciprocal backlink" is not this corpus's convention

The extract proposes adding `kb-decision-0020` to
`kb-open-question-projection-id-unvalidated-001`'s `related:` frontmatter, *"per this corpus's
reciprocal-backlink convention"*, and labels `0021-C6` `defer_open_question`. Both are declined, and
for the same reason: **the convention runs the other way.** Wave 4 stated it in terms, for both
tiers — an accepted decision gains nothing from an inbound edge, and of the two `note`-tier atoms
that could have taken a reciprocal edge, *"this plan does not ask for one — neither claims the
reference."* Edges in this corpus are **outbound-only**, and the map is where a relation becomes
navigable in both directions.

Three further reasons, each independently sufficient:

1. **Both intake files forbid it in terms.** *"It is **not** resolved, edited or deleted by this
   wave"*; *"cited as `related` **only** … left exactly as it is."* An edit adding a link is still an
   edit, and it would change the atom's `last_reviewed` without anyone having reviewed it.
2. **`defer_open_question` means the opposite of what the extract used it for.** In this plan's
   vocabulary that disposition *creates* an `open_question` atom. `0021-C6` asks for no atom to be
   created and none to be touched, which is not a disposition at all — it is the absence of one, and
   filing it as an op would put a mutation in the manifest that no file receives.
3. **Neither question is answered.** `ProjectionId::new` is still infallible after ADR-0020, which
   deliberately does not edit the frozen crate; whether a human-readable encoding is available on a
   memory-limited peer is still phase 9's. A `status` flip would be false in both cases, and a
   `related` edge without one invites a later reader to think a flip was owed and forgotten.

`openQuestionIndex: false` on both ops follows directly: no bullet changes, no bullet is added.

### 8. Defect candidate D-1 stays inside ADR-0020's atom, and stays out of `open-questions/`

This is the wave's one genuine judgement call, and it comes with a live precedent on each side. D-1 —
*`happenstance-core` has no infallible `QueryItem` constructor for pre-validated inputs* — is a
shortfall in a frozen crate whose repair route the record itself states as *a decision record, never
a line edit*. That has the shape of a decision not yet taken, and
`.kb/decisions/README.md` is explicit that a decision not yet taken is an `open_question`, not
something filed inside a decision.

Wave 4 met the same fork twice and split it: PS-32 became an atom (a compiler-falsified sentence,
with a `[FROZEN]` `MUST` in the specification asking for the correction, outliving two waves), and
the PS-3/PS-31/PS-36 candidate stayed in `unresolved` (an observation whose source withheld mandate
in terms). **D-1 matches the second profile on every axis:**

- **No conflict.** Nothing accepted says `happenstance-core` *has* such a constructor, and nothing
  says it should. The decision routes around the gap and says so.
- **No `MUST` anywhere asks for it.** VT-18 is named as the *nearest clause subject*, which is a
  pointer, not an obligation. The specification does not carry the correction as owed.
- **It has a live owner outside `.kb/`.** AC-012's defect log is a running channel in this same
  project, and the record routes D-1 into it explicitly. `open-questions/` exists so that *"we
  looked, and this is genuinely open"* has somewhere to land instead of being rediscovered; a
  finding with an active owner and a scheduled reader is not being lost.
- **It is a first appearance.** The argument that carried PS-32 into an atom was that it had
  recurred in three consecutive manifests and landed nowhere.

So: **no atom.** One sentence inside Op 1's `## What this decision does not decide`, naming D-1, its
clause subject and its route — which is a scope boundary a decision is *required* to state — and one
line in `unresolved`, so that a human can overrule in a sentence rather than having to rediscover the
candidate. If AC-012 closes without disposing of D-1, the next wave should open the atom, and this
paragraph is where it will find the argument already made.

### 9. The repaired citations are correct, and the stale ones are somewhere this wave cannot reach

Claim `0020-C5` is the only claim in the wave whose subject is not in `.kb/` at all. Both repairs
were verified line by line in this worktree rather than taken on trust — `projection.rs:152-154` is
the two-constructor sentence, `:47-61` is `ProjectionId`'s docstring and constructor, and
`RUNBOOK.md:525` is the `happenstance-macros` row while `:524` is an unrelated line about a store
holding only a suffix of its own log. **The wave relies on the repairs and both atoms cite the
corrected ranges.**

What the verification also found is that the repair is **recorded, not performed**: `:47-61` is still
live in `_design.md:638`, `_decomposition.md:544`, four story `spec.md` files and two `_ledger.md`
files, and `RUNBOOK.md:524` in ten more places, and the story's own `report.md:24` says why — *"the
stale ranges remain in `_decomposition.md` and `_design.md`, which are signed-off artefacts this
story may not edit."* No `.kb/` atom cites either stale range, so nothing here is corrupted and no
corrective op exists for this wave to run. It goes to `unresolved` because the obligation is real,
has no owner, and would otherwise be visible only to whoever next reads a range that no longer says
what it is cited for.

---

## What the Maps phase inherits

| Map atom | Change | From |
| --- | --- | --- |
| `kb-map-decision-001` | **One** new section, `## 2026-08-15 typed-layer ADRs (ADR-0020, ADR-0021)`, placed after the ADR-0030 section, carrying **two** rows in ADR-number order: `ADR-0020` · `kb-decision-0020` · *A decision model folds a domain enum, and its query is derived on a sealed trait the caller cannot override* · `accepted` · `7` · `—`; and `ADR-0021` · `kb-decision-0021` · *The codec tag lives in Event::metadata, event types do not carry versions, and upcasting happens at decode* · `accepted` · `7` · `—`. Section prose should carry what the table cannot: that this is the **second** wave on 2026-08-15 and why it gets its own section rather than rows in its sibling; that neither atom supersedes any row on this map; that these are the map's first phase-7 rows; and that the two are linked to each other by `related` for the reason in Adjudication 2 | Ops 1, 2 |
| `kb-map-domain-001` | **A new third domain section**, `## The typed layer: decision models, codecs, and payload evolution`, with a **Decisions** grouping listing both atoms and one orientation sentence each. It must state the boundary it stands on — `kb-decision-0006` allocated the crate and `kb-decision-0007` drew the decode line — **by citation, without moving either atom out of the ports domain**, and must not restate either decision's content | Ops 1, 2 |
| `kb-map-open-questions-index-001` | **Nothing.** No question is opened, resolved, withdrawn or annotated. The two questions this wave cites keep their bullets exactly as they read today | — |

Reciprocal links the Maps/backlink phase must wire, none of which any op writes twice:

- `kb-decision-0020` ↔ `kb-decision-0021` (`related`, **both directions** — both are authored by this
  wave, so both edges are written at creation and neither is an edit to an existing atom)
- `kb-decision-0020` → `kb-decision-0003`, `kb-decision-0006` (`depends_on`); `→ kb-decision-0007`,
  `kb-decision-0015`, `kb-open-question-projection-id-unvalidated-001` (`related`) — **outbound
  only. None of those five atoms gains anything**, and the two `note`-tier ones are not edited
  either (Adjudication 7)
- `kb-decision-0021` → `kb-decision-0003`, `kb-decision-0006`, `kb-decision-0007` (`depends_on`);
  `→ kb-decision-0016`, `kb-open-question-human-readable-encoding-limits-001` (`related`) —
  outbound only, same reason

## Validation the integrate pass must clear

```
redkiln validate --kb    # KbFrontmatter on 2 new atoms; accepted-decision immutability on 21
redkiln doctor           # expects exactly six template-drift advisories, per CLAUDE.md
```

`validate --kb` is the check that would catch this wave's one plausible failure mode: an edit to an
accepted decision body — most likely `kb-decision-0016`, absorbing ADR-0021's framing region under
the 55-point score, or `kb-decision-0007`, absorbing ADR-0020's derived query. Neither op touches
one, and the plan's count of **zero** accepted-decision edits, **zero** frontmatter flips and
**zero** open-question edits is the assertion to re-check after integration rather than before.

Two integration-time checks the tooling cannot make, both cheap:

1. **Both atoms quote `crates/happenstance-core/src/projection.rs:152-154`**, the repaired range —
   not the `:47-61` that `.bklg/` still carries in sixteen places.
2. **ADR-0021's atom enumerates `EventStore` as four methods** — `read`, `append`, `head`,
   `contains_event_id` — and names `read_decision_model` as the free function it is. The story's own
   inputs recite it as a fifth trait method, and the atom must not inherit the error.
