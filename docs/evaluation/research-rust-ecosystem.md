# happenstance vs. the Rust event-sourcing ecosystem

Research pass for the "delight for a working Rust developer" dimension. Sources
read via WebSearch/WebFetch on 2026-08-05; happenstance claims cross-checked
against the actual files listed under each finding.

## 1. The landscape, ranked by what actually gets used

| Crate | Downloads (all-time) | Last release | Shape |
|---|---|---|---|
| `cqrs-es` | ~144K, 42 versions | v0.5.0, ~Feb 2026 | Aggregate-based, `impl Future` in trait (moved off `async_trait` in newer versions), Postgres/DynamoDB adapters as sibling crates (`postgres-es`, `mysql-es`) |
| `evento` | ~68K, 88 versions | v1.8.0, ~4 months ago | Monolithic toolkit: event store + CQRS + projections + subscriptions, SQL-backed, all in one crate |
| `esrs` (primait) | moderate | active | Aggregate-based, `sqlx`/Postgres-only, opinionated |
| `disintegrate` | lower, but the closest philosophical relative | v4.x, active | **Also DCB-shaped** — no aggregates, "query first, decide, append conditionally." Macro-driven, Postgres-only |
| `thalo` | — | **unmaintained**, successor is `SierraDB`/`kameo_es` | WASM-compiled aggregates + `sled` |
| `eventually-rs` | low, stale | — | Early aggregate-based abstraction, largely dormant |

Two conclusions this table supports:

- **The market leader (`cqrs-es`) and the highest-volume alternative (`evento`)
  are both aggregate-based and both monolithic** — one crate gets you the
  store, the CQRS wiring, and (for `evento`) projections and subscriptions.
  Nobody who downloads 68K copies of `evento` is assembling five crates
  first.
- **`disintegrate` is the one existing crate that made the same DCB bet
  happenstance makes**, and README.md:161-166 already names it as prior art
  ("Postgres-bound and macro-driven"). It is the single most load-bearing
  comparison point in this whole report, because it proves the *concept*
  translates to Rust cleanly — the question is only whether happenstance's
  execution is more or less pleasant to use than disintegrate's.

## 2. Conventional shape vs. happenstance, divergence by divergence

### a. Aggregate trait vs. no aggregate at all — **genuine improvement, but a bigger leap than the README lets on**

`cqrs-es`'s `Aggregate` trait (`docs.rs/cqrs-es`) is the shape every Rust
developer coming from this ecosystem will have internalized:

```rust
pub trait Aggregate: Default + Serialize + DeserializeOwned + Sync + Send {
    type Command;
    type Event: DomainEvent;
    type Error: Error;
    type Services: Send + Sync;
    const TYPE: &'static str;
    fn handle(&mut self, command: Self::Command, service: &Self::Services, sink: &EventSink<Self>)
        -> impl Future<Output = Result<(), Self::Error>> + Send;
    fn apply(&mut self, event: Self::Event);
}
```

One type, one `handle`, one `apply`. happenstance replaces this with
`Query` + `read_decision_model` + fold-it-yourself + `AppendCondition`
(`crates/happenstance/src/store.rs:198-208`, `README.md:29-50`). This is the
correct DCB-native shape — `disintegrate` converges on the same idea with its
`Decision` trait — but it means a developer arriving with `cqrs-es` priors
has *no* single trait to implement; they assemble the loop from four
primitives (`Query`, `read_decision_model`, fold, `AppendCondition`) with no
guidance in the crate about where the fold code lives or what it should look
like. `disintegrate` ships that assembly as a named `Decision` trait with a
`DecisionMaker` that runs it; happenstance ships the assembly's *ingredients*
in the contract crate and defers the trait to `happenstance-runtime`, which
is currently `todo!()` (`crates/happenstance-runtime/src/lib.rs:44-50`).
**Verdict: right target, but until `happenstance-runtime`'s `DecisionModel`
lands, a new user's first experience is lower-level than both direct
competitors in this niche.** This is squarely inside what RUNBOOK phase 3
already plans — it is a sequencing gap, not a design defect.

### b. `#[trait_variant::make]` instead of `#[async_trait]` — **genuine improvement, and ahead of where the ecosystem's flagship crate is**

`cqrs-es`'s own getting-started guide still lists `async-trait = "0.1.52"` as
a direct dependency for consumers (`doc.rust-cqrs.org/getting_started.html`),
even though the library's own `Aggregate::handle` has since moved to native
`-> impl Future<...> + Send`. That `+ Send` is baked into the trait
unconditionally — `cqrs-es` has no path to a `!Send` target at all.
happenstance's two-flavour split (`store.rs:1-53`) is the only design among
the crates surveyed that makes a `wasm32`/Durable-Object adapter possible in
principle. Nothing in the ecosystem asks a newcomer to import "the right one
of two trait names" (constraint 4), so this is unfamiliar — but it is
unfamiliar in service of a real capability nobody else offers, not
novelty for its own sake. **Genuine improvement**, and the crate-level doc
comment at `store.rs:31-45` that pre-empts the E0034 ambiguity error by name
is exactly the kind of teaching a newcomer needs and none of the surveyed
crates bother to provide.

### c. Opaque `Bytes` payloads vs. typed events everywhere — **correct, but currently reads as raw**

Every competitor puts a typed `Event` associated type or a `#[derive(Event)]`
macro (disintegrate) front and center. `cqrs-es`: `type Event: DomainEvent`.
`disintegrate`: `#[derive(Event)] enum DomainEvent`. happenstance's `Event`
(`event.rs:183-247`) holds `data: Bytes` and the worked example hand-builds
JSON by string formatting — `examples/course-subscriptions/src/main.rs`
constructs events by string literal, not a struct-to-bytes call (confirmed
by the module doc at `main.rs:1-19` and the `use` list at line 24-27,
which imports no codec). A developer who has just read `disintegrate`'s
README, where `Decision::process` returns `Vec<Self::Event>` from typed enum
variants, will land in happenstance and immediately need to hand-roll
serialization. **This is a real, currently-open gap, and it is the single
biggest ergonomics deficit relative to both direct competitors** — not
because opaque bytes at the contract layer is wrong (it is the right call,
see ADR-0003, and it is what makes the replication story in
`happenstance-sync` clean), but because the *typed* layer that is supposed
to hide this from application code does not exist yet. RUNBOOK phase 3 lists
`Codec` and `DomainEvent` as planned, and explicitly defers a derive macro
("`happenstance-macros` stays deliberately absent... out of scope for 0.1",
`RUNBOOK.md:234`) — meaning even after phase 3, a happenstance user hand-writes
the `EventType`/`Tags` mapping that `disintegrate` users get from
`#[derive(Event)]`. That is a defensible sequencing choice, not a design
flaw, but it is worth being explicit that "batteries included" will not be
true for typed events at 0.1 even after phase 3 completes, only after a
macro crate that phase 3 explicitly excludes.

### d. `Query`/`AppendCondition` primitives exposed directly — **unusual, and probably right for this crate's audience**

None of the aggregate-based crates expose anything like `Query::Items` or
`AppendCondition::fail_if_events_match` to application code — that machinery
is hidden inside the store's internals. happenstance and `disintegrate` both
surface it, because DCB's entire value proposition *is* that per-decision
query. This is not really a "happenstance diverges from the ecosystem"
finding so much as "DCB libraries diverge from aggregate libraries by
necessity" — it is core to the model, not a stylistic choice, so it is not
something to soften.

## 3. What people actually complain about, and where happenstance stands

Search coverage here was thinner than I'd like — I could not find live Reddit
or users.rust-lang.org threads with concrete quotes (searches returned no
substantive results; see Sources). The clearest signal came from the
`cqrs-es` GitHub issue tracker itself:

- **#259 "doc.rust-cqrs.org out of date"** (open, filed Jul 2026) and **#216
  "Example with multiple intercommunicating aggregates?"** (open, filed Sep
  2025) — the two live open issues on the ecosystem's most-downloaded crate
  are *both* documentation/example gaps, not code defects. The recurring
  complaint in this niche is "I don't know how to structure this," not "the
  API is broken."
  - happenstance's answer to this is currently thin in the opposite
    direction: one worked example (`course-subscriptions`), no second
    example showing a second shape (e.g., a projection-consuming read
    model, which is the exact "multiple aggregates" pain point #216 is
    about — DCB dissolves that specific problem, which is worth
    demonstrating explicitly, not just asserted in prose).
  - **This complaint happenstance is structurally immune to for the
    *cross-aggregate* case** — DCB has no aggregates to intercommunicate —
    but it is not yet demonstrated to a skeptical reader, and the module
    docs I'd expect a skeptic to reach for (`store.rs`'s own doctest,
    `README.md`'s worked example) are strong on *why*, thinner on "here is
    a second, different-shaped decision to build confidence this
    generalizes."
- **#216's underlying complaint — "how do commands read from more than one
  aggregate" — is exactly the problem class DCB was invented to solve**, and
  happenstance's `README.md:15-53` states this directly and well. This is a
  case where happenstance's core bet is aimed squarely at the ecosystem's
  most-cited confusion. Worth keeping and stating loudly in any
  crates.io-facing copy, since it is a real differentiator, not a claim.
- `thalo`'s abandonment (its own repo now redirects users to `SierraDB`/
  `kameo_es`) is a reminder that this niche's crates die when a single
  maintainer's design bet stops being maintained. happenstance's
  conformance-suite-as-the-bar-for-existing (CLAUDE.md, "the rule that
  matters") is a structural answer to a real, observed failure mode in this
  ecosystem — adapters that half-work and nobody can tell.

## 4. Batteries inventory

Rated against what a Rust developer building a production event-sourced
system currently has to assemble by hand if the library doesn't provide it,
using `evento` and `disintegrate` as the "what's normal to expect" baseline.

| Battery | Status | Evidence |
|---|---|---|
| Derive macro for domain events | **absent, and explicitly deferred past 0.1** | RUNBOOK.md:234, "`happenstance-macros` stays deliberately absent until there is something worth deriving... out of scope for 0.1." `disintegrate` ships `#[derive(Event)]` today. |
| Typed decision models | **planned, phase 3, not started** | RUNBOOK.md:235-238, `happenstance-runtime/src/lib.rs:31-34` |
| Command bus / handler abstraction | **planned, phase 3** | RUNBOOK.md:239-241, "the command loop... Retry policy is a decision worth stating in the docs" |
| Projection runner | **planned, phase 3** (port frozen in phase 2) | RUNBOOK.md:242, `crates/happenstance/src/projection.rs` marked PROVISIONAL |
| Subscriptions / catch-up | **absent, unplanned** — no mention anywhere in RUNBOOK's decision ledger or phase list | grep of RUNBOOK.md and CLAUDE.md's open questions turns up nothing; `evento` ships this today as a first-class feature |
| Snapshots | **absent, unplanned** | no mention in RUNBOOK.md; `cqrs-es` has snapshot support via its `View`/query-repository split |
| Upcasting / event versioning | **absent, unplanned** | not mentioned; this is a known sharp edge in every long-lived event store and the RUNBOOK does not flag it even as an open question |
| Outbox / integration events | **absent, unplanned** | not mentioned |
| Idempotency helpers | **partially present, one narrow slice** — `happenstance-sync`'s RUNBOOK entry names "idempotent ingest" as an open question for replication only (RUNBOOK.md:365-366); nothing for command-side idempotency (e.g., a client-supplied idempotency key on append) |
| Retry policy | **named but not designed** | RUNBOOK.md:241, "Retry policy is a decision worth stating in the docs, not just implementing" — acknowledged as a gap, not filled |
| `tracing` integration | **absent, unplanned** | no `tracing` dependency anywhere in the Cargo.tomls read; not in RUNBOOK's decision ledger at all — notable, since this is close to a checklist item for "production-ready" in the Rust ecosystem generally |
| Metrics | **absent, unplanned** | same as above |
| Testing helpers (given/when/then) | **absent, unplanned for the typed layer** — the *store* conformance macro (`happenstance_testkit::event_store_conformance!`) is excellent for adapter authors, but there is nothing analogous for *application* authors testing their decision logic the way `cqrs-es`'s `TestFramework::given_no_previous_events().when(cmd).then_expect_events(...)` does | `crates/happenstance-testkit/src/lib.rs:1-56` is store-adapter-only by design; `docs.rs/cqrs-es` `test` module confirmed to ship exactly this given/when/then pattern today |
| CLI / migration tooling | **absent, unplanned** | `xtask` is a dev-workflow tool (fmt/clippy/tests), not a user-facing CLI; no migration tooling mentioned anywhere |
| serde/codec support | **present at the envelope level, planned at the payload level** | `happenstance/serde` feature exists today for `Event`/`SequencedEvent` wire format (`event.rs:301-399`, `Cargo.toml:31`); the payload `Codec` (JSON/CBOR/postcard) is phase-3-planned, not present |
| Multi-tenancy | **absent, unplanned** | not mentioned anywhere; `Tags` (`tag.rs`) could carry a tenant tag by convention today, but nothing in docs or RUNBOOK calls this out as a supported pattern vs. an accident of the tag system being generic |

**Reading this table as a whole**: happenstance's "batteries" are heavily
weighted toward what the *contract* needs (conformance suite, opaque
payloads, two-flavour ports) and correspondingly light on what an
*application developer* touches day to day (typed events, testing DSL,
tracing, retry policy, subscriptions). That is defensible as sequencing —
CLAUDE.md's repo map literally marks `happenstance-runtime` as the
un-started "named seam" for most of this — but the RUNBOOK's own phase 3
scope (RUNBOOK.md:221-253) does not include subscriptions, snapshots,
upcasting, tracing, metrics, or a testing DSL even as stretch goals. Every
competitor surveyed ships at least one of these today. If "batteries
included" is a 0.1 claim rather than a north star, the gap between the
RUNBOOK's actual phase-3 scope and this list is worth reconciling explicitly
— either add the missing items to phase 3 (or a phase 3.5), or soften the
public-facing "batteries" language until they exist, because right now the
RUNBOOK plans a typed layer that is still thinner than `disintegrate`'s
*current* feature set, let alone `evento`'s.

## 5. Adoption mechanics

- **The `-core` split**: ADR-0006's serde_core/serde and futures-core/futures
  precedent is real and current — `serde_core` was split out of `serde` in
  2024 specifically so proc-macro-heavy downstream crates could depend on the
  types without pulling in `serde_derive`'s compile cost, and `futures-core`
  has carried the trait definitions since `futures` 0.3. Both are read by the
  ecosystem as "the `-core` crate is for library/adapter authors; the bare
  name is what applications add to `Cargo.toml`." ADR-0006's bet matches this
  convention exactly, which is the right call — but note that neither
  precedent case renames an *already-shipped* bare crate the way this
  migration does (nothing has been published, so there's no cost here; just
  confirming the pattern reads correctly to a Rust developer who has seen
  `serde_core` show up in their `Cargo.lock` and wondered why).
- **README shape**: `README.md` already does the two things that matter most
  for a skimming Rust developer — a runnable code block in the first screen
  (`README.md:29-50`) and a status table that is honest about what's stubbed
  (`README.md:63-74`). This is better than `cqrs-es`'s own README, which
  leads with prose before any code. One gap: no `[dependencies]` version-pin
  guidance beyond `happenstance = "0.1"` (README.md:84) — not yet publishable
  since nothing is on crates.io, fine for now.
- **Examples directory**: one example (`course-subscriptions`). `evento` and
  `cqrs-es` both ship multiple examples covering at least one aggregate-style
  and one projection/read-model-style flow. Given finding 3 above (the
  ecosystem's top pain point is "I don't know how to structure a second
  scenario"), a second example — ideally one that shows a projection being
  built from `read_decision_model` output, once the projection port is
  frozen in phase 2 — would directly counter the most common complaint
  found in this research.
- **Feature-flag naming**: `std`, `memory`, `serde` (`Cargo.toml:26-34`) reads
  as entirely conventional — matches the `no_std`-with-`std`-opt-in pattern
  used by `bytes`, `futures-core`, etc. No notes; this is right.
- **MSRV policy**: 1.85 pinned via ADR-0004, stated in CLAUDE.md and enforced
  in CI per RUNBOOK.md:15-16. This is more rigorous than most crates surveyed
  (`cqrs-es`'s docs don't state an MSRV policy at all in the pages fetched).
  Genuine strength, not a divergence to flag.

## Bottom line for the "delight" question

happenstance's foundational bet (DCB, opaque bytes, two-flavour ports, a
conformance suite as the definition of "adapter") is not unfamiliar
academic novelty — it is the same bet `disintegrate` already validated in
production Rust, executed with more rigor (a published, adapter-agnostic
conformance suite; a wasm-reachable port design that not even `cqrs-es` has).
The risk to "delight" is not the core design. It is that **everything a Rust
developer currently reaches for first** — a typed event, a way to test a
decision without touching bytes, `tracing` spans around the command loop —
lives entirely in the `todo!()` crate. A developer who clones the repo today
and reads the README's own quick-start gets a real, working, well-documented
low-level primitive; the "batteries" promise is aimed at a layer that does
not exist yet and, per the RUNBOOK's actual phase-3 scope, will still be
missing several items every direct competitor already ships (subscriptions,
snapshots, a testing DSL, tracing). That's a sequencing question for the
RUNBOOK to answer explicitly, not a flaw in what's built so far.

## Sources

- [cqrs-es — crates.io](https://crates.io/crates/cqrs-es)
- [cqrs-es — docs.rs, `Aggregate` trait](https://docs.rs/cqrs-es/latest/cqrs_es/trait.Aggregate.html)
- [cqrs-es — docs.rs, `test` module (TestFramework given/when/then)](https://docs.rs/cqrs-es/latest/cqrs_es/test/index.html)
- [CQRS and Event Sourcing using Rust — doc.rust-cqrs.org getting started](https://doc.rust-cqrs.org/getting_started.html)
- [serverlesstechnology/cqrs GitHub issues](https://github.com/serverlesstechnology/cqrs/issues)
- [disintegrate — GitHub](https://github.com/disintegrate-es/disintegrate)
- [esrs — crates.io](https://crates.io/crates/esrs)
- [thalo — GitHub (unmaintained, redirect notice)](https://github.com/thalo-rs/thalo)
- [primait/event_sourcing.rs — GitHub](https://github.com/primait/event_sourcing.rs)
- [get-eventually/eventually-rs — GitHub](https://github.com/get-eventually/eventually-rs)
- [evento — crates.io](https://crates.io/crates/evento)
- [lib.rs — event-sourcing keyword listing](https://lib.rs/keywords/event-sourcing)
- Local: `README.md`, `CLAUDE.md`, `docs/RUNBOOK.md`, `.kb/decision/0006-bare-name-to-the-typed-layer.md`,
  `crates/happenstance/src/{store.rs,event.rs,Cargo.toml}`,
  `crates/happenstance-runtime/{src/lib.rs,Cargo.toml}`,
  `crates/happenstance-testkit/src/lib.rs`,
  `examples/course-subscriptions/src/main.rs`
