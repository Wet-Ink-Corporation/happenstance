---
title: Personas & journeys — From Contract to Published Library
initiative: HS-I0006
slug: from-contract-to-published-library
status: draft-for-planning
promoted: false
scope: problem-space-only
sourcePaths:
  - .bklg/from-contract-to-published-library/_intake-brief.md
  - references/seeds/remaining-runway.md
  - .kb/product/README.md
  - .kb/concepts/torn-reads-and-the-append-condition-boundary.md
  - .bklg/from-contract-to-published-library/_discovery/research/01-dcb-implementations-and-competing-event-stores.md
  - .bklg/from-contract-to-published-library/_discovery/research/02-rust-event-sourcing-crate-landscape-and-sentiment.md
  - .bklg/from-contract-to-published-library/_discovery/research/03-conformance-testkit-as-a-shipped-product.md
  - .bklg/from-contract-to-published-library/_discovery/research/04-storage-agnostic-ports-that-survived-a-second-database.md
  - .bklg/from-contract-to-published-library/_discovery/research/05-position-visibility-in-postgres-and-distributed-logs.md
  - .bklg/from-contract-to-published-library/_discovery/research/08-rust-publication-semver-and-msrv-discipline.md
  - .bklg/from-contract-to-published-library/_discovery/research/09-wasm32-edge-and-non-send-async-ecosystem.md
  - .bklg/from-contract-to-published-library/_discovery/grounding/product-functional-alignment.md
  - .bklg/from-contract-to-published-library/_discovery/grounding/backlog-adjacency.md
---

# Personas & journeys — From Contract to Published Library

Four personas, distilled from the intake brief's named audiences
(`_intake-brief.md`, "Who it is for" in `references/seeds/remaining-runway.md`)
and stress-tested against the nine research angles. None is promoted to
`.kb/product/` yet — `.kb/product/README.md` reserves that for closeout, once
the initiative's own contact with a database and a consumer has either
confirmed or corrected these sketches. Every persona below is a *real,
evidenced audience* in the sense that file demands: goal, context, what they
already do instead, and what they are afraid of, each tied to a citation —
not an invented stock photo.

Problem-space only. Nothing below names a crate, a trait, a wire format or an
implementation choice; where a pain point has an obvious technical fix, the
fix is left out on purpose and flagged instead as a question for planning.

---

## Persona 1 — The application author

**Who they are.** A Rust developer building a domain that needs consistency
across more than one entity at a time (the class of problem DCB names) inside
an application they otherwise control — not a library maintainer, not
evaluating in the abstract. They are the person the intake brief means by
"an application author can `cargo add` and rely on"
(`_intake-brief.md:31`) and the person ADR-0006 says the typed layer exists
for: "happenstance gives an application what it actually wants," not a
five-line facade (`.kb/decisions/0006-bare-name-to-the-typed-layer.md:64-66`).

**Goal.** Model their domain's consistency boundary once, against a contract,
and defer *which database* to a decision they can revisit later without
rewriting the domain.

**What they do today instead.** Every Rust event-sourcing crate with real
adoption forces the database choice first: `cqrs-es` (158k downloads) and
`esrs` (122k downloads, hard-coupled to Postgres/sqlx via its maintainer)
are both storage-coupled opinionated frameworks, not ports
(research 02, key findings 1). The one prior storage-agnostic attempt in
Rust, `eventually-rs`, stalled at exactly one non-memory backend and has
been dead since October 2020 (research 02, key finding 2). So today this
person either accepts a framework's database opinion or hand-rolls their own
port and pays for that decision alone.

**What they are afraid of.** Being the one who discovers a contract defect
in production, after they have already built on it — the exact failure mode
the seed names: "nothing has used the contract the way an application would
... there is no consumer" (`references/seeds/remaining-runway.md`, "The
problem"). A more specific, evidenced version of that fear: a read they
perform independently of any write — a projection, an export, a replay —
silently building a wrong answer from an inconsistent view of the log, with
nothing about the outcome signalling that anything was wrong. This is not
speculative; it is the documented mechanism behind the append-condition
boundary ("a torn read does not turn into a rejected append; it turns into
an accepted one" — `.kb/concepts/torn-reads-and-the-append-condition-boundary.md`)
and it is the exact situation research digest 5 says "nothing today reads the
way a second, independent consumer would."

**Journey — today.**
1. *Chooses DCB* because it names the consistency problem they already have.
2. *Searches for a Rust library* and finds crates that require picking a
   database before they can find out whether the domain model even works
   (research 02, finding 1; teaching resource `doc.rust-cqrs.org` never
   surfaces storage-agnosticism as an axis at all — research 02, finding 8).
3. *Either* couples early to a database they may regret, *or* starts writing
   their own port — repeating the exact work `happenstance` already claims
   to do, with no way to check their port against anything.
4. *If they proceed anyway*, everything they build sits on a facade that
   has never been exercised by a real application (`crates/happenstance/src/lib.rs:75`
   is a glob re-export today) — so any defect in the contract is *their*
   discovery, in their own codebase, not a defect already found and fixed
   upstream.
5. *If they later add a second, independent reader* (a projection, an
   export, a replay job) they have no documented answer for what that
   reader is allowed to assume about what it sees, and no way to tell a
   transient gap from a permanent one.

**Journey — what the initiative should improve (stated as outcomes, not
mechanism).** The moment in step 2 where "pick a database" and "does the
domain model work" are forced into one decision is the one this initiative
exists to separate. The moment in step 4 where the facade has never been
exercised is what "the contract has been used" (`_intake-brief.md:36-38`) is
meant to close — this person should be able to point at a worked example and
at recorded defects the *project* discovered, not a blank history. The
moment in step 5 is unresolved even in the vision: the initiative's own open
questions (visibility invariant global-vs-boundary, what a store may forget)
say this journey beat does not yet have an honest answer, only a commitment
that silence is not acceptable (`_intake-brief.md:98-104`).

---

## Persona 2 — The adapter author

**Who they are.** Someone implementing the contract against a real storage
system — today, this is exclusively the project's own team (six skeleton
crates, `publish = false`, `37 todo!() bodies... spread across 20 files in
six of them` per `_intake-brief.md:15-16`), but the constraint the persona
represents is general: whoever eventually writes a seventh adapter needs the
same thing the first six needed. CLAUDE.md and the intake brief both name
this audience explicitly: "an adapter author can implement against with a
bar that tells them when they are done" (`_intake-brief.md:32`).

**Goal.** An executable definition of "correct" they can run against their
own storage system, rather than a prose specification they have to
interpret and hope they interpreted the same way as everyone else.

**What they do today instead.** They have a specification
(`spec/SPECIFICATION.md`, 200 clauses) and a conformance suite — but every
implementation that has actually passed it "serialises its writers and
assigns positions under a lock it holds until commit: four adapters, one
storage shape wearing four hats" (`_intake-brief.md:18-20`). A `todo!()`
body type-checks against any signature, so "it compiles" is not yet evidence
an adapter author's work is correct (`_intake-brief.md:15-16`). This is not
an idiosyncrasy of this project: every mature multi-backend conformance
regime surveyed (Kubernetes CSI, OpenDAL, s3-tests, Test262) treats a suite
that has not yet been run against genuinely disagreeing implementations as
unproven by construction (research 03, findings 1-4, 7), and the wider
literature (Rule of Three, "the wrong abstraction") independently warns that
an abstraction tested only against too-similar implementations is unearned
(research 04, finding 6).

**What they are afraid of.** Discovering — late, expensively, and possibly
after other adapters already depend on the same assumption — that the port
they implemented against quietly assumed something their storage system
cannot provide. The nearest real-world domain peer, Marten (Postgres event
sourcing, .NET), did not merely risk this: its author *rejected*
storage-agnosticism for exactly this reason, on grounds research digest 4
calls "identical" to this project's own `ProjectionStore` invariant (research
04, finding 2 and implication 1). That is a live, evidenced objection a
knowledgeable adapter author could raise, not a hypothetical.

**Journey — today.**
1. *Reads the specification and the conformance suite* to learn what
   "correct" means for their storage system.
2. *Implements the trait*, and the compiler accepts a `todo!()` body as
   readily as a real one — so passing the type checker confirms nothing.
3. *Runs the suite* — but every rule in it has so far only ever been
   checked against storage shapes that agree with each other on the axis
   that matters most (who holds a lock, who serialises writers), so a green
   run today is evidence about one storage shape wearing four hats, not
   evidence the port itself is sound (`_intake-brief.md:18-20`).
4. *Has no way to tell*, from inside their own effort, whether the port
   would have looked different if it had been checked against a store that
   disagreed with the others on that axis — because none currently exists
   in a built, passing state.

**Journey — what the initiative should improve.** The gap between step 3 and
a trustworthy "correct" is exactly what the second proof artefact names:
adapters passing the suite "across shapes that genuinely disagree ... at
minimum a real durable store, a store that does not serialise its writers,
and a store with no connection and no cursor" (`_intake-brief.md:39-42`).
Until that contact happens, this persona's fear is not hypothetical caution
— it is the same fear that made Marten's author choose differently, and the
initiative's own standing rule ("a port is only as well-designed as the
*spread* of what implements it," CLAUDE.md) says it must be answered by
contact, not by argument.

---

## Persona 3 — The local-first / edge Rust developer

**Who they are.** A developer building an application that must run inside a
constrained, non-`Send` async runtime — concretely, a Cloudflare Durable
Object on `wasm32`, though the constraint generalises to any single-threaded
JS-event-loop host. The intake brief and seed both call this out as
load-bearing, not incidental: "the local-first and edge case is load-bearing,
not a nice-to-have... work that quietly drops it has changed the product"
(`_intake-brief.md:69-71`).

**Goal.** Event-source their domain on the edge without hand-building the
entire event-sourcing layer themselves, and without giving up the ability to
reason about their domain the same way a server-side Rust developer would.

**What they do today instead.** They hand-roll it. A first-person developer
account of building event sourcing on Cloudflare Durable Objects in Rust is
direct, dated evidence this audience exists and is already absorbing real
friction: `workers-rs` lagging the TypeScript SDK, WASM binary bloat, sparse
docs — concluding "unless you specifically want Rust, TypeScript is probably
the less painful path" (research 02, finding 7; research 09, finding 1-2).
The specific technical wall this audience hits is documented in a live,
unresolved GitHub issue where a developer building a real integration asks
the framework's own maintainers "how do you imagine this should work?"
(research 09, finding 2). A prior serious attempt at this exact niche
(Thalo, a wasm-based event-sourcing runtime) is now unmaintained — evidence
of recurring demand *and* of how hard the niche is to sustain (research 09,
finding 5).

**What they are afraid of.** That committing to Rust on the edge for
event-sourced logic is the less-supported path, confirmed by their own
tooling rather than disproven by it — and, more specifically, that whatever
they build today will be orphaned the way Thalo was, because nothing
storage-agnostic in this niche has yet stayed maintained. They are also, per
the wider ecosystem's own default guidance, swimming against the current:
most Rust library authors are steered toward the Send-injecting default
unless they have a specific reason not to (research 09, finding 3) — so this
persona cannot assume the rest of the ecosystem is optimizing for them.

**Journey — today.**
1. *Picks Rust for an edge-resident event-sourced service*, often for
   reasons outside this initiative's control (existing team skill, a
   monorepo, a performance requirement).
2. *Looks for a storage-agnostic event-sourcing option* and finds none that
   targets their runtime — DCB's own libraries page lists one Rust project,
   shipping one backend (Postgres), explicitly *not* claiming to be a strict
   implementation (research 01, finding 2).
3. *Hand-rolls the event-sourcing layer themselves*, absorbing the
   `workers-rs` gaps directly (research 09, finding 1) — the sync KV API
   that would make this easier exists in the platform's JavaScript surface
   but not yet in its Rust one, an unresolved 2024 feature request (research
   09, finding 4).
4. *Has no comparison point that shares their constraint* — the nearest
   in-language projects (`cqrs-es`, `esrs`) never attempt the `!Send`
   boundary at all (research 02, finding 6).

**Journey — what the initiative should improve.** The gap between step 2 and
"none that targets their runtime" is what the `!Send` flavour and the CF-*
clauses exist to close — but as of the intake brief, `happenstance-cloudflare`
is still a skeleton (`_intake-brief.md`, repository map), so this persona's
journey has not actually been walked by the project either. Any claim that
the contract "met" a Durable Object will be meeting a target whose own
Rust-vs-JS parity is unfinished — a fact about the target's maturity, not
this project's own gap, but one this persona will feel regardless (research
09, implication 3).

---

## Persona 4 — The evaluator (pre-adoption)

**Who they are.** A Rust developer who has *not yet* decided to adopt
anything — doing due diligence before choosing a library, likely comparing
`happenstance` against `cqrs-es`, `esrs`, `umadb-dcb`/UmaDB, or Disintegrate.
This persona is implicit rather than named verbatim in the intake brief, but
it is squarely inside "an application author can `cargo add` and rely on"
(`_intake-brief.md:31`) — the moment *before* the `cargo add`, where "rely
on" has to be earned in public rather than assumed. Research digest 1's own
framing states the stakes directly: this audience's first question is "does
this actually work across more than one real backend," not "why not the
existing library" (research 01, implication 2).

**Goal.** Decide, in a bounded amount of research time, whether
`happenstance`'s claims (storage-agnostic, DCB-compliant, edge-capable) are
real, without being able to run the project's own internal test suite
themselves or take the maintainers' word for it.

**What they do today instead.** They read whatever the crate's registry
page and documentation show them at a glance. This is a thin signal
industry-wide: `cargo publish` does not stop a crate from looking unfinished
on its own landing page (missing README rendering, no docs.rs build,
threadbare license metadata) even when the underlying code is solid
(research 08, finding 7). For DCB claims specifically, there is nothing
external to check against: the DCB specification itself defines no
conformance suite, test kit, or certification process, so "DCB-compliant" is
industry-wide an unverified, self-asserted label (research 01, finding 1).
Two DCB-labelled stores (the base spec and UmaDB) already disagree with each
other on a load-bearing position semantic — gaps permitted vs. gapless —
that an evaluator could easily miss and depend on by accident (research 01,
finding 3).

**What they are afraid of.** Adopting a library on the strength of a claim
("DCB-compliant," "storage-agnostic") that turns out to be true for only one
storage shape, discovered after they have already built on it — the same
fear Persona 1 carries, but earlier, before any code is written, when the
only available evidence is what the project chooses to publish and how it is
presented.

**Journey — today (hypothetical, since nothing is published yet).**
1. *Searches for a Rust DCB or storage-agnostic event-sourcing library* and
   finds a thin, active field: one non-strict Rust DCB implementation
   (Disintegrate, Postgres-only), one storage-agnostic *port* whose only
   implementation is a standalone server product rather than an in-process
   library (`umadb-dcb`/UmaDB), and two storage-coupled frameworks with real
   download volume but no storage-agnosticism claim at all (research 01
   findings 2-3; research 02 findings 1-3).
2. *Has no external body to check "DCB-compliant" against* — no TCK
   equivalent exists for DCB the way it does for Java, no shared certification
   the way Kubernetes conformance provides (research 03, findings 1-2 vs.
   research 01, finding 1).
3. *If they reach `happenstance`'s registry page*, finds whatever
   completeness and proof the release actually shipped with — the same
   thing every other candidate on their list is judged by.

**Journey — what the initiative should improve.** This persona is the reason
the intake brief's proof artefact is framed as public evidence rather than
an internal gate: "a suite green against `MemoryEventStore` alone proves
nothing about the contract... only a *passing implementation* is a far end"
(`_intake-brief.md:139-147`) is written for exactly the reader who cannot run
the suite themselves and has to trust what the project shows. Whatever this
initiative decides to make legible on first contact (a passing multi-backend
result, a stated position on MSRV-as-promise, a registry page that does not
look unfinished) is this persona's entire journey — they do not get a second
pass.

---

## Cross-persona tensions

- **Persona 2 vs. Persona 1, same worry, different vantage.** Both fear a
  contract defect discovered too late; Persona 2 discovers it while
  implementing a port, Persona 1 discovers it while depending on one already
  implemented. The initiative's four desired outcomes address both, but in
  sequence (contract used → adapters proven → published → replication/
  retention answered), so at any given moment one persona's fear is more
  addressed than the other's. Worth naming explicitly in planning rather
  than assuming the same evidence satisfies both simultaneously.
- **Persona 3 is a special case of both 1 and 2, not a fourth concern
  layered on top.** They are simultaneously an application author (wants to
  model a domain) and, in effect, an adapter author's downstream user (wants
  the Cloudflare adapter to actually exist and be proven) — but they carry
  an additional, evidenced fear (orphaned tooling, per Thalo) that neither
  of the other two personas carries in the research.
- **Persona 4 is time-boxed and one-shot in a way the others are not.** An
  application or adapter author can revise their understanding through
  contact with the code over weeks. An evaluator's decision window is
  whatever they spend reading a registry page and a README once; research
  digest 8's finding that registry-facing completeness fails silently if
  skipped applies to this persona specifically.
- **All four personas share one unresolved beat:** none of them currently
  has anywhere to look for an answer on "what happens when a read is
  independent of the write that produced it" (torn reads, retention gaps,
  cross-store replication). That is not a persona-specific pain — it is a
  structural gap in what the project has said publicly, evidenced
  independently by research digests 5, 6, and 7 and by the concept atom on
  torn reads.

## Risks

- **Persona 3 and Persona 4 are the two least-tested against real evidence.**
  Persona 3 rests on one first-person blog post and one GitHub issue thread
  — real and dated, but thin compared to the download-count evidence behind
  Persona 1. Persona 4 is inferred from research framing rather than from a
  named individual's stated experience anywhere in the source material;
  planning should treat it as directionally right but not over-specify
  behavior from it.
- **Conflating Persona 1 and Persona 2 in acceptance criteria would hide a
  real difference.** An application author's trust is earned by the typed
  layer and a worked example; an adapter author's trust is earned by the
  conformance suite's spread. Framing both from one generic "user" persona
  would blur which proof artefact actually addresses which fear.
- **None of these four personas has been directly interviewed or observed**
  by this project — every citation is secondary evidence (download counts,
  public blog posts, GitHub issues, an adjacent project's own postmortem).
  That is consistent with a pre-publication library with no existing users,
  but it means these personas describe an *inferred* audience, and
  `.kb/product/README.md`'s own bar ("a persona nobody researched is a stock
  photo with a name") should be re-checked at closeout once real contact
  (a worked example, an actual adapter author, actual downloads) exists to
  confirm or correct them.

## Open questions for the planning team

- Should Persona 4 (the evaluator) be promoted as its own persona at
  closeout, or folded into Persona 1 as an earlier stage of the same
  journey? The research evidence treats them as distinct concerns (public,
  pre-code trust vs. in-code, post-adoption trust) but they may not warrant
  separate acceptance criteria.
- Persona 2's journey currently describes the project's *own* team as the
  only adapter authors who exist. Should planning treat "adapter author" as
  purely internal for this initiative's timeframe, or does any acceptance
  criterion need to hold up for a hypothetical external adapter author too?
- The cross-persona tension item ("what happens when a read is independent
  of the write that produced it") touches all four personas but is not
  currently owned by any single desired outcome in the intake brief — worth
  checking whether outcome 4 ("honest written answers on replication and on
  what a store may forget") is meant to cover it, or whether it is a fifth,
  unnamed concern.
- Persona 3's fear of orphaned tooling (per Thalo) has no corresponding
  proof artefact in the intake brief's list — none of the four desired
  outcomes speaks to long-term maintenance signal. Is that intentionally out
  of scope for this initiative, or a gap in how the outcome was framed?
- Should the reconciliation of the two disputed clause-count sources
  (`_intake-brief.md:123-127`) happen before or after acceptance criteria are
  framed from these personas, given at least one persona (the evaluator)
  would see whichever count is eventually published?
