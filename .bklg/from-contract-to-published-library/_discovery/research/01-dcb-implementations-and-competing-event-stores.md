---
title: "dcb-implementations-and-competing-event-stores — research for From Contract to Published Library"
kind: research
initiative: from-contract-to-published-library
summary: "DCB has no external conformance authority anywhere in the ecosystem, the Rust field is nearly empty, and every production adopter observed shows the same uneven-adapter-spread pattern happenstance's own house rule warns about."
sources:
  - https://dcb.events/specification/
  - https://dcb.events/faq/
  - https://dcb.events/resources/libraries/
  - https://sara.event-thinking.io/2023/05/dynamic-consistency-boundary.html
  - https://github.com/disintegrate-es/disintegrate
  - https://lib.rs/crates/umadb-dcb
  - https://crates.io/crates/umadb-dcb
  - https://umadb.io/
  - https://github.com/umadb-io/umadb
  - https://martendb.io/events/dcb.html
  - https://www.axoniq.io/blog/dcb-in-af-5
  - https://www.axoniq.io/blog/axon-server-future-proof-event-store
  - https://www.kurrent.io/blog/counterexamples-regarding-consistency-in-event-sourced-solutions-part-3/
  - https://docs.eventsourcingdb.io/best-practices/dynamic-consistency-boundaries/
---

# dcb-implementations-and-competing-event-stores

## Findings

- **The DCB specification defines no conformance suite, reference test kit, or
  certification process of any kind.** Its own conformance clause says
  implementations "are not required to use the same terms or function/field
  names — as long as they offer equivalent functionality" — "equivalent
  functionality" is judged by nobody but the implementer. "DCB-compliant" is
  presently a self-asserted label across the entire ecosystem, not a checked one.
- **The Rust field for DCB is nearly empty, and the one entry the spec's own
  site lists is explicitly flagged as not-quite-compliant.** dcb.events'
  official implementations page (`/resources/libraries/`) lists exactly one
  Rust project — Disintegrate — under a note that it takes "a slightly
  different approach, inspired by the original ideas of DCB," i.e. the
  maintainers of the spec itself did not classify it as a strict
  implementation. Disintegrate's own README confirms an `EventStore` trait
  abstraction exists, but its shipped backend is Postgres
  (`disintegrate-postgres`) with no second backend in mainline and no public
  conformance suite adapters must pass.
- **A second, more direct Rust competitor exists but was not surfaced by the
  spec's own listings page: `umadb-dcb`.** It is a published, storage-agnostic
  trait/port crate (`DcbEventStoreAsync` / `DcbEventStoreSync`, query/event/
  append-condition types, `DcbError`/`DcbResult`) at v0.7.5 with 47 releases on
  crates.io, dual MIT/Apache-2.0 licensed, targeting the 2024 edition. But its
  reference implementation, UmaDB, is a **standalone gRPC server** ("Small
  footprint. Big performance," Copy-on-Write MVCC storage) built to be talked
  to over the network, not an embeddable in-process library with pluggable
  storage backends — nothing in its public materials (site or README)
  mentions wasm32, edge, or embedded deployment.
- **UmaDB's own design commitment is gapless, monotonically increasing
  positions** ("stores events in an append-only sequence, indexed by
  monotonically increasing gapless positions") — the opposite of what the DCB
  specification requires and the opposite of what this repository's own
  conformance discipline forbids asserting on (`CLAUDE.md`: "never assert on
  literal position values… the specification permits gaps, and a conformant
  adapter may leave them"). Two DCB-labelled stores already disagree with each
  other on a load-bearing semantic that is easy for an application author to
  quietly depend on.
- **Every established, production-grade event-sourcing product observed here
  added DCB on top of an existing single-vendor database, not as a
  storage-agnostic contract with even adapter spread:**
  - **Axon Framework 5** makes DCB a first-class concept, but its own
    open-source JPA event store "solely supports the aggregate-based
    approach… does not support DCB." Only the commercial Axon Server
    (2025.1.0+) fully supports DCB; the in-memory store that does support the
    concept is explicitly marked unsuitable for production.
  - **Marten** (.NET/PostgreSQL) shipped DCB as additive tagging on top of its
    existing per-stream model, with **two competing internal storage schemes**
    for the same feature (`TagTables`, the default, vs. an opt-in `HStore`
    mode that its own benchmarks show up to 92% faster on multi-tag queries) —
    and a required schema migration in a point release (9.4) to fix a
    consistency-checking defect in the shipped scheme.
  - **KurrentDB** (formerly EventStoreDB) is building DCB support around a
    bespoke query language, EventQL, going well beyond the base spec's plain
    type/tag filters.
- **The concept is young and still being actively renegotiated by vendors.**
  It originates from Sara Pellegrini's April 2023 "Kill Aggregate" talk and
  the May 2023 blog post that named it; the specification site itself credits
  three named individuals (Bastian Waidelich, Sara Pellegrini, Paul Grimshaw)
  rather than a standards body. Its own FAQ states plainly that traditional
  event stores cannot use DCB "out of the box," and that the maintainers are
  themselves only "experimenting with an adapter layer" using pessimistic
  locking as a possible bridge — i.e. even DCB's authors treat
  backward-compatibility with pre-existing event stores as unsolved.

## Evidence & citations

- No conformance suite/testkit in the spec, and the "equivalent functionality"
  conformance bar: https://dcb.events/specification/
- FAQ — DCB doesn't work "out of the box" with traditional stores; maintainers
  "experimenting with an adapter layer" using pessimistic locking; performance
  is not the primary goal; ~1,000 events/sec early Postgres benchmark:
  https://dcb.events/faq/
- Official implementations/libraries listing, including the single Rust entry
  (Disintegrate) with its "slightly different approach" caveat, and the
  commercial/open-source DCB-compliant store table (Axon Server, EventSourcing
  Database, UmaDB): https://dcb.events/resources/libraries/
- Origin of the term and concept, Sara Pellegrini, May 2023:
  https://sara.event-thinking.io/2023/05/dynamic-consistency-boundary.html
- Disintegrate's `EventStore` trait abstraction and Postgres-primary backend:
  https://github.com/disintegrate-es/disintegrate
- `umadb-dcb` as a storage-agnostic trait/port crate (`DcbEventStoreAsync`/
  `DcbEventStoreSync`), version/release history:
  https://lib.rs/crates/umadb-dcb and https://crates.io/crates/umadb-dcb
- UmaDB positioned as a standalone gRPC server with Copy-on-Write MVCC
  storage: https://umadb.io/
- UmaDB's gapless monotonically increasing position indexing, and DCB
  specification credit to Waidelich/Pellegrini/Grimshaw:
  https://github.com/umadb-io/umadb
- Marten's DCB as additive tagging, `TagTables` vs `HStore` storage modes, the
  92% multi-tag-query benchmark gap, and the 9.4 schema-migration requirement:
  https://martendb.io/events/dcb.html
- Axon Framework 5's DCB support split across event stores (JPA store
  excludes it, Axon Server 2025.1+ supports it, in-memory store is
  non-production): https://www.axoniq.io/blog/dcb-in-af-5 and
  https://www.axoniq.io/blog/axon-server-future-proof-event-store
- KurrentDB building EventQL as its DCB consistency-condition mechanism:
  https://www.kurrent.io/blog/counterexamples-regarding-consistency-in-event-sourced-solutions-part-3/
  and https://docs.eventsourcingdb.io/best-practices/dynamic-consistency-boundaries/

## Implications for the idea

- **No one outside this project is going to vouch for "DCB-compliant."**
  There is no certifying body, no cross-vendor test suite, and no shared
  fixture set anywhere in the ecosystem — every implementer, including the
  well-resourced commercial ones, is grading its own homework. Whatever
  credibility "From Contract to Published Library" earns for the phrase
  "DCB-compliant" has to come entirely from what this initiative's own proof
  artefact demonstrates (a suite green across genuinely disagreeing stores),
  because there is nothing external to lean on and nothing external the
  library can be checked against later either.
- **happenstance is likely to be read as one of the first storage-agnostic,
  embeddable, multi-backend DCB libraries in Rust, not as one competitor among
  several.** The direct comparison set is thin: one Rust project the spec's
  own maintainers describe as not-quite-compliant (Disintegrate), and one
  genuinely storage-agnostic Rust port crate (`umadb-dcb`) whose only known
  implementation is a standalone server product with no stated embedded/edge
  story. That reframes the audience's likely first question away from "why
  not the existing Rust DCB library" and toward "does this actually work
  across more than one real backend" — which is exactly the question the
  initiative's chosen proof artefact (three disagreeing stores, one suite) is
  built to answer, and it is likely to be the single most load-bearing claim
  the library makes on first contact with a stranger.
- **Every production DCB adopter surveyed here reproduces, independently, the
  exact failure mode this repository's own house rule warns against** ("a port
  is only as well-designed as the spread of what implements it"): Axon's own
  JPA store excluded from its own framework's flagship feature, Marten
  shipping two competing storage schemes for the same capability with a
  visible performance gap between them, a schema bug needing a point-release
  migration. This is independent, cross-ecosystem field evidence — not just an
  in-repo aphorism — that a DCB port validated comfortably against one backend
  reliably turns out not to be the real contract the moment a second, harder
  backend shows up. It strengthens the case for treating the port-freeze
  discipline as load-bearing rather than procedural.
- **Position semantics are already a live portability trap between two
  DCB-labelled stores** (UmaDB's gapless guarantee vs. the base specification
  and this project's stance that gaps are permitted). Someone evaluating
  happenstance against something else in this space is evaluating projects
  that make different promises about a detail easy to depend on by accident.
  That argues for the plain-language promise about positions being something
  a newcomer can actually find and read before they build on it — a
  documentation/clarity concern, not an architectural one.
- **DCB is roughly three years old and still being reinterpreted by each new
  adopter** — vendors are already extending it with their own query surface
  (KurrentDB's EventQL) or their own storage strategy (Marten's two tag
  schemes) rather than treating the base specification as sufficient on its
  own. A library that deliberately stays close to the bare specification, with
  no bespoke query language and no vendor-specific storage opinion, is making
  a legible but non-default choice in this landscape; it may be worth this
  initiative stating that choice and its reasoning somewhere a newcomer would
  find it, rather than leaving it implicit.
