---
title: Opportunities and product archetypes — From Contract to Published Library
kind: distillation/opportunities
initiative: from-contract-to-published-library
summary: "Ranked problem-space opportunities and product archetypes this initiative unlocks, distilled from nine research angles and three grounding passes. Highest-value, highest-confidence: happenstance is positioned to be the first genuinely storage-agnostic, adopted, conformance-proven DCB library in Rust, for two currently-unserved audiences (contract-first application authors, and edge/!Send developers) who today have zero viable options."
---

# Opportunities and product archetypes

Scope: what this initiative could become worth to the people who would use it,
and what kind of thing it would be recognized as once published — evidence and
gaps only, no architecture, no tech choice, no API shape, no "we should build."
Ranked by **value** (how much the gap matters to the audience that has it) and
**confidence** (how directly the research evidences it, vs. inference).

## How to read the ranking

Value and confidence are each rated High / Medium / Low. An item can be high
value and only medium confidence (real, but inferred rather than directly
evidenced) — that combination is called out explicitly rather than rounded up.

---

## Opportunities, ranked

### 1. Be the first genuinely storage-agnostic, adopted, conformance-proven DCB library in Rust

**Value: High. Confidence: High.**

**What it is.** A Rust developer who wants to build on the Dynamic Consistency
Boundary pattern currently cannot pick "the DCB contract" independently of
picking a database — no shipped, adopted Rust library separates those two
decisions today.

**Evidence.** The spec's own libraries page lists exactly one Rust project
(Disintegrate), explicitly flagged by DCB's own maintainers as "a slightly
different approach... inspired by" DCB rather than a strict implementation,
and it has shipped exactly one backend (Postgres) across its history. The one
other Rust crate built specifically as a storage-agnostic DCB *port*
(`umadb-dcb`) is a client for a single new standalone store product, not a
layer over databases a team already runs — a different shape of thing
entirely. The one prior Rust attempt at exactly this shape, `eventually-rs`,
stalled at one non-memory backend and has been dead since 2020. Every Rust
crate with real adoption (`cqrs-es` at 158k downloads, `esrs` at 122k) is a
storage-*coupled* opinionated framework, not a portable contract.

**Gap it closes.** "Pick the contract, then pick storage" is presently not a
choice available to a Rust developer at all — every real option forces
database-first commitment, or forces adoption of an entirely new store
product. This is not a crowded category with a differentiation problem; it is
an empty category.

---

### 2. Give "DCB-compliant" a checkable meaning, in an ecosystem where the phrase is self-asserted everywhere else

**Value: High. Confidence: High.**

**What it is.** Anyone evaluating a DCB-labelled store today has no way to
check the claim against anything but the vendor's own word.

**Evidence.** The DCB specification defines no conformance suite, reference
test kit, or certification process anywhere; its own conformance clause
requires only "equivalent functionality," self-judged by the implementer.
"DCB-compliant" is, industry-wide, an unverified label — true of every
implementation surveyed, including established production adopters (Axon
Framework 5, Marten, KurrentDB), each of which layered DCB onto one
pre-existing database with uneven internal adapter spread (Axon's own
open-source store excludes DCB entirely; Marten runs two internally
disagreeing storage schemes for the same feature with up to a 92% performance
gap between them).

**Gap it closes.** There is presently nothing outside any given project that
an adopter, a competitor, or a curious bystander can point to and say "this
store actually does what it claims." A conformance suite that is itself
runnable and legible closes that gap — not just for this library, but as the
first available instance of it anywhere in the DCB ecosystem.

---

### 3. Serve the edge / `!Send` Rust developer who is currently hand-rolling event sourcing with no library and documented pain

**Value: High. Confidence: High.**

**What it is.** A Rust developer building event-sourced state on a
memory-constrained, single-threaded edge runtime (the kind Cloudflare
Workers/Durable Objects define) has no storage-agnostic event-sourcing option
today and is solving the problem by hand.

**Evidence.** This is a platform-forced constraint, not a design preference:
Durable Objects structurally forbid the multi-threaded async runtimes
(Tokio/async-std) every incumbent Rust ES crate assumes, because the whole
handler runs as one JS-event-loop-bound future. A public GitHub issue
(workers-rs #485) shows a developer directly asking maintainers "how do you
imagine this should work?" after a real Axum+R2 integration failed on this
exact boundary. A first-person developer write-up on hand-rolling event
sourcing on Durable Objects in Rust names the concrete friction (lagging
SDK, WASM bloat, sparse docs) and concludes, today, that "unless you
specifically want Rust, TypeScript is probably the less painful path." A
prior serious attempt at this niche (Thalo, a wasm/event-sourcing runtime) is
now unmaintained — evidence of recurring demand *and* of how hard the space is
to sustain.

**Gap it closes.** This audience is not hypothetical or inferred — it is
dated, documented, and presently being told (by its own best evidence) to
give up on Rust for this use case. A viable option changes that answer for
the first time.

*Caveat carried into planning:* Rust's own async ecosystem currently steers
most library authors toward the Send-only default; the design pattern that
serves this audience is a documented minority choice, not the ecosystem norm,
so the value of the property will need active explanation rather than
speaking for itself. This is a durability/communication risk to hold, not a
reason to discount the opportunity.

---

### 4. Turn "storage-agnostic and safe" into a claim backed by disagreement, not just breadth

**Value: High. Confidence: Medium-High.**

**What it is.** The credible version of "works across any storage backend" is
not "we tested it against four stores" but "we tested it against stores that
actively disagree with each other on the axis most likely to break the
contract" — and that framing is itself a differentiator, because most
real-world storage-agnostic ports never get checked against a genuinely
disagreeing backend before their design calcifies.

**Evidence.** Every mature multi-backend precedent surveyed (Apache Arrow's
`object_store` across S3/GCS/Azure/local, OpenDAL across 50+ services,
Kubernetes CSI's sanity suite) treats backend disagreement as the *point* of
the proof, not a risk to average away. Conversely, the nearest domain peer,
Marten (Postgres event sourcing for .NET), deliberately *rejected*
storage-agnosticism, and its author's stated reason is the same invariant at
stake here: one native transaction across event capture, projection write,
and checkpoint. Even long-lived, widely-adopted agnostic ports carry
undetected drift for years after shipping — Go's `database/sql` interfaces
still lack context after years of production use; Rails' ActiveRecord
adapters silently disagreed on table-listing semantics for years before
anyone reconciled it. Mainstream design literature (the Rule of Three,
Sandi Metz's "wrong abstraction") independently names the same failure mode:
an abstraction proven only against too-similar implementations is unearned.

**Gap it closes.** It closes the credibility gap between "claims to be
storage-agnostic" and "has actually been shown to survive a backend that
disagrees" — a gap wide enough that the nearest comparable project chose to
abandon agnosticism rather than close it. Confidence is medium-high rather
than high because the *value* of this specific framing to an outside adopter
(as opposed to its methodological soundness) is argued from precedent and
literature rather than observed directly from a happenstance user.

---

### 5. Answer questions the field has left silent, instead of adding another silent implementation

**Value: Medium-High. Confidence: High (that the gap exists) / Medium (that answering it is what adopters will value first).**

**What it is.** Two questions — what an independent reader of a replicated
store may assume about position identity across a boundary, and what a reader
may assume about a log that has holes in it — are open industry-wide, not
just here.

**Evidence.** The DCB specification itself is silent on both: it scopes
`SequencePosition` and the append condition to one store and says nothing
about deletion, retention, or cross-store meaning. The wider field hasn't
converged either: Kafka's own mirroring tool (MirrorMaker2) treats
offset/position as cluster-local and builds a lossy translation map, with a
documented, named failure mode ("an unfixed amount of duplicate consumption
of records" on failover) on the Kafka mailing list itself; EventStoreDB
practitioners, in a thread with EventStoreDB's own creator, report that
ordering is explicitly sacrificed first across a replication boundary.
Retention is equally unsettled: crypto-shredding (the most-recommended GDPR
pattern) is legally contested; Kafka's own tombstone-expiry can leave a
lagging consumer permanently and silently wrong; a shipped framework
(Ecotone) found it necessary to invent a three-way taxonomy of gap causes
because a single undifferentiated "gaps may exist" rule was not enough in
practice, and production write-ups describe silently skipped events
discovered only months later via a wrong customer-facing number.

**Gap it closes.** Nobody in this space — including DCB's own authors, who
describe themselves as still "experimenting" with parts of this — has a
stated, reasoned position on either question. A library that gives an
honest answer (a decision *or* an explicit, reasoned refusal) occupies
ground that is presently unoccupied by anyone, not just by happenstance.
Confidence on the underlying gap is high; confidence that this is what draws
adopters (versus the conformance proof or the edge story) is more inferential
and rated accordingly.

---

### 6. Make publication itself the trust event, not an afterthought

**Value: Medium. Confidence: High.**

**What it is.** For a library whose whole pitch is "the contract can be
relied on," the moment private engineering opinions (surface, MSRV,
semver boundary) become checked, external, unwithdrawable promises is itself
part of the product — not a release-mechanics footnote.

**Evidence.** Semver breakage in Rust is common and invisible to ordinary
review: a study of 14,000+ crate releases found roughly 1-2 accidental
breaking changes land in popular crates every week despite maintainers'
best efforts, and the tooling that catches this (`cargo-semver-checks`) is
still being merged into `cargo` itself as of 2026 — evidence the ecosystem
does not yet trust review alone. Whether an MSRV bump even counts as
breaking is a live, unresolved argument even for mature, popular crates: a
public `clap` MSRV controversy shows real, opposed costs on both sides, with
the maintainer calling the negotiation "one of the more draining
interactions" they have. A crate can also `cargo publish` successfully while
looking unfinished on its own landing page (missing README render, no
docs.rs build) with nothing in the tooling stopping it.

**Gap it closes.** Most Rust libraries treat these as mechanical release
steps; none of the research surfaced an ecosystem norm strong enough to lean
on. A library that treats "audited against a registry baseline, not against
prose" as part of what "done" means is differentiated rigor an adopter
evaluating trust can actually observe — but it is a supporting, not
headline, opportunity: value is rated medium because it reinforces claim #1
and #2 rather than opening new ground of its own.

---

## Product archetypes this unlocks

These are recognizable *kinds* of thing the initiative would put into the
world once real — durable shapes an outside observer would name it as, not
technical designs.

### A. The contract-first library
A library chosen because of what it promises about consistency and
portability, with the storage decision made *after*, not before. This
archetype does not presently exist in Rust's event-sourcing space at all —
every real option today forces the database choice first (§1, §4).

### B. The portable proof
A conformance suite that functions as a public trust artifact in its own
right — consulted by adapter authors, evaluators, and possibly competitors,
independent of anyone reading the library's own source. Precedented outside
Rust (TCK, Kubernetes conformance, Jepsen's whole premise) but without a
dominant Rust idiom yet, which is itself part of the opportunity: being early
to a genre rather than filling an existing one (§2).

### C. The edge-resident write model
Event-sourced state that can live and be reasoned about inside a
memory- and single-thread-constrained edge runtime without its author
abandoning Rust for the use case. Presently a documented gap with recent,
named casualties (Thalo) rather than an occupied space (§3).

### D. The honestly-graded contract
A public ledger of what is settled versus still provisional or deliberately
refused, kept current against reality rather than left as prose that ages
silently. No comparable adopter-facing device was found in any peer project
surveyed — the closest analogues (spec maturity markers, TCK certification)
exist in much larger, better-resourced ecosystems (§2, §6).

### E. The boundary with a stated answer
A replicated or federated deployment where what a reader on the other side of
a store boundary may assume — about identity, ordering, and what may have
been forgotten — is a written position rather than an emergent property
nobody decided. Presently unoccupied ground industry-wide, not merely
locally (§5).

---

## Risks

- **The comparison set is thin but fast-moving.** `umadb-dcb` published a new
  release the day before this initiative's own intake and already has
  meaningful download volume; it is a different shape of thing (a client for
  a new standalone store, not a port over existing databases) but will likely
  be the first thing an external observer reaches for as "the competitor."
  Silence on how happenstance differs from it is likely to be read as an
  oversight, not neutrality.
- **The nearest domain peer rejected this initiative's central premise.**
  Marten's author explicitly chose *not* to be storage-agnostic, on grounds
  that are structurally identical to the invariant `ProjectionStore` exists
  to protect. A knowledgeable evaluator will ask "why not do what Marten
  did," and the research did not surface an existing answer on file.
- **"Proven against a second, disagreeing backend" is a snapshot claim, not
  a durable one.** Both cited long-lived, widely-adopted precedents (Go's
  `database/sql`, Rails' ActiveRecord) carried live or previously-undetected
  semantic drift for years after broad multi-backend adoption. Whatever
  confidence gets published about a freeze holding should be scoped to what
  was actually checked, not read as a permanent guarantee.
- **The audience this library would most differentiate for (edge/`!Send`)
  is also the audience least served by the Rust ecosystem's own defaults** —
  most public-trait guidance steers authors toward the Send-only pattern, so
  this property needs to be actively explained rather than assumed legible
  on sight.
- **The product/design layer of the knowledge base is currently empty.**
  No persona or journey atom exists yet for "application author," "adapter
  author," or the edge/local-first audience named repeatedly across this
  research — these opportunities are argued from research evidence and the
  intake brief, not from an already-adjudicated, promoted persona record.

## Open questions for the planning team

- Which of the two lead opportunities (§1 contract-first proof, §3 edge/
  `!Send` audience) should carry the *headline* claim at publish, given they
  pull toward different first impressions (portability-and-trust vs.
  platform-enabling)? The research does not resolve this; both are high
  value and high confidence but distinct pitches.
- Should this initiative state a position on `umadb-dcb` and Disintegrate
  explicitly (a comparison), or let the conformance proof stand alone and
  let outside observers draw the comparison themselves? Silence has a cost
  (risk above); a stated comparison has its own cost (freezing a
  characterization of a fast-moving external project).
- Is answering "why not reject storage-agnosticism the way Marten did" a
  documentation-stage question or does it belong earlier, as something the
  ADR queue for the `ProjectionStore` freeze should address on the record?
- Do the product archetypes above (§A-E) warrant promotion into `.kb/product/`
  persona/journey atoms as part of this initiative's own closeout, given that
  layer is currently unpopulated — or is that explicitly out of scope and
  left to a later initiative?
- Which of the two silent-industry-questions opportunities (replication
  identity, retention/forgetting) should be answered first if both cannot be
  fully resolved within this initiative's boundary — the brief already
  carries both as open questions, but this distillation surfaces them as
  *product* opportunities, which may argue for a different priority than an
  engineering-only view would give them.
