# CLAUDE.md

Operating notes for AI assistants working in this repository. Read this before
changing anything.

## What this is

A storage-agnostic, [DCB-compliant](https://dcb.events/specification/) event
sourcing library. `happenstance-core` defines the contract; adapter crates
implement it; `happenstance-testkit` decides whether they did. `happenstance`
itself is the typed layer an application reaches for — today a five-line facade
over the contract, holding the bare name because that is the crate most people
will `cargo add` ([ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md)).

## Who you are working with

The repository owner has 25+ years in distributed systems, databases and
enterprise architecture, and is fluent in C#, Python and JavaScript. They are
**new to idiomatic Rust**.

So: do not explain event sourcing, CQRS, consistency boundaries or concurrency
control — that ground is well covered. **Do** explain Rust-specific reasoning:
why a newtype instead of a type alias, why `NonZeroU64` earns its keep, what
coherence is refusing to allow, why a lint is on, why a lifetime is where it is.
When a construct is unusual, say what the alternative was and why it lost.

## Repository map

```
crates/happenstance-core/        the contract. types, ports, errors, in-memory store.
crates/happenstance/             the typed layer. today a facade over the contract.
crates/happenstance-testkit/     conformance suite. the bar every adapter must clear.
crates/happenstance-sqlite/      the first adapter. event store + projection store.
crates/happenstance-cloudflare/  the second adapter. the workspace's only !Send store. wasm32.
crates/happenstance-ladybug/     🔩 skeleton. graph projection store only.
crates/happenstance-postgres/    the store that does not serialise its writers. event store
                                 real and conformant at phase 10; 🔩 projection store still
                                 a skeleton.
crates/happenstance-neon/        🔩 skeleton. Postgres over one-shot HTTP. host + wasm32.
crates/happenstance-sync/        🔩 skeleton. the replication port + peers + a runner.
examples/course-subscriptions/   the canonical DCB worked example. in memory.
examples/transfers-on-sqlite/    the same library on a real database — the typed layer's
                                 command loop and projection runner against
                                 happenstance-sqlite, one file, read back after every
                                 handle is dropped. the only place the two crates a
                                 consumer installs are compiled together.
examples/outside-projection-adapter/
                                 🔬 the falsifier. a projection adapter written from the
                                 rendered documentation alone, in a crate where the orphan
                                 rule and the non-dev graph behave as they do for a stranger.
examples/rebuilding-read-models/ operating derived state. four views over one log at two
                                 checkpoints; a blue/green backfill and its promotion; reset,
                                 and a rebuild at two chunk sizes; and a view poisoned by a
                                 field the log does not carry, which stalls only itself.
examples/handles-and-quotas/     the boundary with no aggregate. a unique name over an
                                 unbounded set, a per-owner quota, and an idempotent
                                 delivery — three scopes, one append condition. the only
                                 place command-retry idempotency is written down.
examples/telemetry-across-codecs/
                                 the log that outlived its encoding and its schema. JSON and
                                 postcard payloads read by one fold, and a v1 event shape
                                 upcast in `decode`. the only use of its event-type argument.
examples/tickets-over-http/      two processes over one file. an HTTP API and a projection
                                 runner, contended over real sockets, with read-your-writes
                                 answered by 202 until the view catches up. one image, three
                                 roles, and the only lib target under examples/.
xtask/                           `cargo xtask ci` — the whole gate, defined once.
spec/                            SPECIFICATION.md — every clause that is true now.
                                 E2E-CASES.md — the cases stated as observable behaviour.
standards/rust/                  the Rust constitution. router + 27 atoms.
experiments/                     measurements. reproducible, and not in the gate.
references/                      evidence kept for citation, binding nothing.
  evaluation/                      the fourteen reviews, and the Crux explorations.
  scenarios/                       six deployments the contract was walked against.
  adapter-shapes.md                what the six skeletons told the type checker.
  adr/                             the full decision records. cite these by line.
  architecture/                    the workspace drawn — one traced command loop, the
                                   in-process boundary, and where operator-owned
                                   storage begins. self-contained HTML and its source.
  seeds/                           raw material for `/redkiln:initiative`.
docs/                            user documentation. nothing else.
RUNBOOK.md                       the plan of record, and how far it has got.

.kb/                             the knowledge base — what is settled.
  decisions/                       the ADRs, one atom each. accepted ones are immutable.
  concepts/ governance/            explanations; the binding constraints, one atom each.
  playbooks/ reference/            transferable practice; measurements and pointers.
  open-questions/                  what is deliberately not settled.
  maps/                            the indexes: domain, decisions, open questions.
  product/ design/                 personas and journeys; signed-off design patterns.
  _intake/                         staging. `/redkiln:kb-ingest` consumes and clears it.
.bklg/                           the backlog — work in motion. `redkiln status`.
.redkiln/                        config, the pinned process pack, templates, telemetry.
```

**🔩 skeleton** means real associated types and `todo!()` bodies, `publish =
false`, and a scoped `#![allow(clippy::todo)]` naming the phase that removes it.
A skeleton exists to be disagreed with by a type checker — it is an *instrument*
first and a target second, and it is not an adapter until it has run the
conformance suite. **Two have stopped being skeletons**: `happenstance-sqlite` at
phase 8 and `happenstance-cloudflare` at phase 9, and both are published.

`happenstance-postgres` is the **half case**, and the marker is split rather than
dropped. Its *event store* has run the suite — 101 of 101 against a live
PostgreSQL 17.10, including the concurrency family at 64 contenders — so by the
rule above it is an adapter, and the arm it buys ES-10 with is recorded in
ADR-0024 rather than still open. Its *projection store* is untouched `todo!()`,
which is what the crate's remaining `#![allow(clippy::todo)]` now covers, and
`publish = false` stands until `deskeleton-and-package-readiness` removes it. The
**three** that carry the marker whole — ladybug, neon, sync — have run nothing,
and the marker is the claim.

Dependency rule: **everything depends on `happenstance-core`; `happenstance-core`
depends on nothing in this workspace.** No adapter may depend on another adapter.

One deliberate exception, and it is a port relationship rather than a dependency
between adapters: `happenstance-sync` is itself a port crate. Peer adapters
depend on it the way store adapters depend on `happenstance-core`, and its
conformance suite will live in `happenstance-sync-testkit`, which does not exist
yet. It stays out of the contract crate so that publishing `happenstance-core`
never waits on replication.

## Where the work lives

This repository is Redkiln-managed. Three trees, and they answer different
questions — putting something in the wrong one is how it stops being findable.

- **`.kb/` — what is settled.** Durable knowledge as *atoms*: markdown with
  frontmatter that `redkiln validate --kb` checks. An **accepted decision atom is
  immutable** — validation checks each one against `HEAD`, so correcting one means
  writing a new atom that supersedes it, never editing the body. That is the
  discipline the ADRs were always written under and nothing previously enforced.

  **A decision lives in two places on purpose.** `.kb/decisions/` holds the
  seventeen *atoms* — canonical, ~100 lines each, carrying the frontmatter, the
  status and the supersession graph that `validate --kb` enforces.
  `references/adr/` holds the full original records, up to 1,508 lines, carrying
  the compiler transcripts, the rejected alternatives and the measurement tables
  a summary cannot hold. Link the atom; cite the record by `file:line`. Deleting
  the second because the first exists would discard about 78% of the corpus, and
  `spec-trace` will catch you, because `spec/SPECIFICATION.md` cites line ranges
  that only exist in the long form.

  Atoms are authored by `/redkiln:kb-ingest` from `.kb/_intake/`, not by hand:
  hand-writing them produces the directory layout of the process without the
  process, which is why the first attempt at this was reverted (`0269720`).
- **`.bklg/` — work in motion.** Initiatives decompose into projects, projects into
  stories. `redkiln status`, `redkiln next`, `redkiln board`.
- **`.redkiln/` — the engine.** Config, the process pack and templates (both pinned
  at `init`), and committed telemetry.

**The CLI is the only writer of an item's system frontmatter.** Never hand-edit
`id`, `stage`, `status`, `updated` or `links`; drive every state change through
`redkiln <command>`. A `PreToolUse` hook denies the edit, and the prose body of an
artifact is yours to write freely.

Six templates under `.redkiln/templates/` are deliberately customised — `spec.md`,
`_design.md`, `_intake-brief.md`, `discover.md` and the two gate checklists — so
`redkiln doctor` reports six `template-drift` advisories forever. That is expected,
and the `backlog` CI job asserted the set was **exactly** those six: a seventh is a
template someone changed without deciding to, and a missing one is a customisation
reverted by `adopt --templates`.

**That job is disabled as of 2026-09-04** — this repository has moved off the
redkiln version it pins, so the pinned CLI reports drift against a process the
repository no longer runs. It is `if: false` rather than deleted, so it shows as
*skipped* rather than vanishing: the job's own argument is that a check which
quietly stops running is worth less than none, because the green tick keeps
arriving. **While it is off, nothing enforces `.kb` frontmatter validation, the
immutability of accepted decision atoms, hand-edited item frontmatter, or the
six-template assertion above** — all four merge green. The restore path and the
full cost are written at the job in `.github/workflows/ci.yml`.

**Never run `redkiln adopt --templates`.** `redkiln upgrade` recommends it, and it
is wrong here: it would overwrite all six customisations with the bundled defaults,
silently — and the CI assertion above would then fail on the *absence* it created.
The customisations are the repository's own gate bars, and one of them (the
one-line checklist boxes) exists because the parser matches line-by-line and a
wrapped box can never match.

`_design.md` is the bundled design stage repurposed. Redkiln ships it because
nothing else in its pipeline could perceive what a screen looks like; a library has
the same hole in another medium, so here it asks for the **public API surface** —
signatures, visibility decisions, what the shape costs a caller, and a doctest in
place of a mock. Every other check in this repository is satisfied by an API that
is correct and unusable. `design.capture` is deliberately absent from
`.redkiln/config.yaml`, which makes the perceptual review a *skip* rather than a
silent pass: there is no app to screenshot.

## Binding constraints

These come from `.kb/decisions/`. Changing one means writing a new ADR, not editing
code around it.

1. **Never introduce `#[async_trait]`.** It injects `+ Send`, which makes the
   `wasm32` / Cloudflare Workers target impossible. Ports are defined once
   without a `Send` bound and `trait_variant` derives the `Send` flavour.
   (ADR-0001)
2. **Never put `serde` in `happenstance-core`'s default features.** Payloads are
   opaque `Bytes`. The `serde` feature covers envelope types only, for
   replication. (ADR-0003)

   Read the crate name carefully: this constrains **`happenstance-core`**, and
   after ADR-0006's rename it says the opposite of what it used to. `happenstance`
   is now the *typed* layer, whose entire job is encoding — it is the crate that
   will depend on `serde`, and forbidding it there would forbid the thing the
   split exists to allow.
3. **`EventStore::read` returns the stream at the top level and is not
   `async`.** Nesting it inside a future silently drops `+ Send` from the
   stream on the `Send` flavour, defeating the entire two-trait design.
   **Two** tests in `memory.rs` assert this and it takes both; if you find
   yourself deleting either, stop.
   `send_flavour_stream_is_send_in_generic_code` writes the bound at the
   definition, so the obligation is discharged before monomorphisation — that
   is the fix for the old test, which asserted `Send` on a *concrete* stream
   and passed by auto-trait leakage whatever the trait said. But it is not
   sufficient on its own: under the `async fn read` refactor the outermost
   item is the future, `trait_variant` marks the future `Send`, and the
   assertion is satisfied by the wrong thing. `spawns_from_generic` is what
   rejects that refactor, because it holds the stream across an await inside a
   real `tokio::spawn`. (ADR-0001, ADR-0008)
4. **Bind `EventStore`, not `SendEventStore`, in generic code.** It is the
   weaker requirement and accepts both flavours. Import only one of the two
   names per module — having both in scope makes method calls ambiguous.
5. ~~**No let-chains.**~~ **The MSRV is 1.97.1**, raised from 1.85 at phase 2
   ([ADR-0029](.kb/decisions/0029-msrv-raised-to-1-97-1.md), amending ADR-0004).
   Let-chains stabilised in 1.88 and are now available.

   The instruction that replaced it is the same instruction, one level up:
   **weigh the floor, and from `0.2.0` it is also a promise.** The old text here
   said *"nothing is published, so no downstream consumer is pinned to anything"*,
   and that was ADR-0004's whole reason for carrying a **provisional** marker.
   `0.2.0` is what the marker named as its own end: five crates are on crates.io,
   consumers are pinned, and raising the floor is now a breaking change that
   needs a decision record rather than a commit message. Raising it at phase 2
   was a deliberate trade recorded in an ADR, which is what the old text asked
   for; what stays forbidden is moving it in silence, and the bar for moving it
   at all is higher than it was.

   Two things follow that are easy to miss. The MSRV now **equals**
   `rust-toolchain.toml`'s pin, so the `msrv` CI job proves nothing until the two
   diverge — it is kept for the day they do, and says so. And the reason the
   floor moved was a *dependency's build script*, not our code: five of the five
   database crates in this workspace declare no `rust-version` at all, so
   `cargo hack --rust-version` cannot protect a floor against them and neither
   can `resolver = "3"`. Only running the compiler finds it.

## The rule that matters

**Any new event store adapter must invoke the conformance suite and pass it
before it is considered to exist.**

```rust
happenstance_testkit::event_store_conformance!(MyFixture::new());
```

The expression builds a **`Fixture`**, not a store — that changed at phase 3 and
the old `factory =` spelling is gone with no deprecated arm, because nothing is
published yet. One fixture instance is one isolated backing store; each
`connect()` on it is one handle onto that store. A fixture also declares
`SECOND_HANDLE` and `REOPEN` as `Capability` associated constants, and a rule
whose capability is declined still runs, reporting the fixture's stated reason
rather than vanishing from the binary. `happenstance_testkit::fixtures::MemoryFixture`
is the reference implementation.

An adapter that compiles but has not run the suite is not an adapter. If a rule
seems wrong, fix the rule and explain why in the same change — do not skip it.

When adding a conformance rule, never assert on literal position values
(`[1, 2, 3]`). The specification permits gaps, and a conformant adapter may
leave them. Compare against positions the store actually assigned.

Two corollaries, both learned the expensive way and both easy to violate by
accident:

**A rule that no adapter can fail is decorative.** Before adding one, name a
plausible wrong implementation it rejects, and write that implementation into
the testkit's own `tests/` if one does not already exist there.

**A port is only as well-designed as the *spread* of what implements it.**
`MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object all
serialise their writers and assign positions under a lock — four adapters, one
storage shape, and any port frozen against them is frozen against SQLite wearing
four hats. Before freezing a port, name the axis it is most likely to be wrong
about and check that something in the workspace sits at the other end of it. That
is what `happenstance-postgres` (positions assigned outside the transaction) and
`happenstance-neon` (no connection, no interactive transaction, no cursor) are
for. They are instruments first and targets second.

## House style

**It lives in [`standards/rust/`](standards/rust/README.md) — the Rust constitution.**
Twenty-seven atoms, each carrying rules with a compiled example and a named
wrong implementation. Read the router first and pull the one to three atoms your
task needs; do not load the corpus.

The four bullets that used to sit here are [`70-rustdoc-obligations.md`](standards/rust/70-rustdoc-obligations.md)
and [`00-prime-directives.md`](standards/rust/00-prime-directives.md), in full and with
the mechanism attached. They are not repeated here because this file loads on
*every* task, including the ones that will never write a doctest — and because
two copies of a style guide is two things to update and one that goes stale.

Start with the trigger table in the router. `cargo xtask lint-constitution`
checks the corpus's citations and shape; `cargo test -p xtask --doc` compiles
every example in it.

## Commands

```console
cargo xtask ci                          # the whole gate — run this before saying "done"
cargo xtask ci --fast                   # REQUIRED only; the bar a non-terminal project meets
cargo xtask affected --base main        # the story grain: only what this diff could break
cargo test --workspace --all-features
cargo run -p course-subscriptions        # the worked example
cargo xtask wasm                        # just the wasm32 check
cargo xtask spec-trace                  # just the specification's cross-references

redkiln status                          # the backlog roll-up
redkiln next                            # what is actually actionable
redkiln validate --kb && redkiln doctor # the backlog and knowledge base check
```

The middle two are not conveniences — they are the commands `.redkiln/config.yaml`'s
`verify:` block wires to redkiln's story and integration grains, so they run whether
or not anyone types them.

`cargo xtask ci` runs: fmt, clippy with `-D warnings`, tests, four wasm32 steps —
the build of `happenstance-core`, which is the standing guard on constraint 1 and
names the contract crate on purpose, a check of the conformance harnesses, and
builds of `happenstance-cloudflare` and `happenstance-neon`, both of which claim
that target in their own documentation and neither of which was checked by
anything until phase 2 — docs, `cargo xtask spec-trace` over
`SPECIFICATION.md`, a `--no-default-features` **and** a default-features doc
build of `happenstance-core` (three configurations in all with the workspace
`--all-features` one, because a link from a `memory` page into a `conformance`
item is broken at neither end of that range and only in the middle, which is
where a consumer stands),
and a `cargo package --list` assertion that each of the **five** publishable
crates — `happenstance-core`, `happenstance`, `happenstance-testkit`,
`happenstance-sqlite` and `happenstance-cloudflare` — carries both licence files
and a README. The number is spelled with its members now because it had already
drifted once: this sentence read *"three"* through phase 9's promotion of
`happenstance-cloudflare` and did not move, so a count on its own turned out to
be a claim nobody re-reads. `xtask/src/package.rs`'s `reconcile` is what
actually holds the set honest, in both directions. Then, where the tool or toolchain is
present: `cargo hack` feature-powerset, `cargo deny`, a wasm32 feature-powerset
check above the mandatory plain one, and a nightly `--cfg docsrs` rustdoc build.
It is defined once in `xtask/src/main.rs` and is exactly what CI runs.

`cargo-hack` and `cargo-deny` both resolve on this machine, so those steps run
rather than printing `skipped`: a green local gate now proves more than it used
to, not less. A step skips only when its probe fails to find the tool — a tool
that runs and finds a problem always fails the gate.

The MSRV is the one thing the local gate still does not check. CI carries a
dedicated `msrv` job that runs `cargo hack check --no-dev-deps --rust-version` on
a pinned 1.97.1 toolchain, which is what a *consumer* sees, and then a full
`cargo test --workspace --all-features` at 1.97.1, because `--no-dev-deps` is
exactly the flag that hides `proptest` and `tokio` — both of which declare
`rust-version = "1.85"`, leaving no headroom at all. **That job now runs the
same compiler the gate runs** and proves nothing until the pin and the floor
diverge again; ADR-0029 explains why it is kept rather than deleted.

## Open questions, deliberately unresolved

Do not settle these silently in passing; they need their own pass and probably
their own ADR. Two files carry the answers, and they answer different questions.
[`spec/SPECIFICATION.md`](spec/SPECIFICATION.md) says
what is **true now** — 201 numbered clauses, each carrying a maturity marker
(frozen, provisional, deferred, or demoted to non-normative prose) and each
naming the conformance rule that checks it and the wrong implementation it
forbids. `cargo xtask spec-trace` is a gate step precisely so those markers and
citations cannot rot into decoration. [`RUNBOOK.md`](RUNBOOK.md) says
**who settles what is still open, and when**. Where a summary below disagrees
with a clause, the clause wins; the summaries are orientation only.

Changing a `[FROZEN]` clause requires a new ADR, not an edit.

- **The projection store port is provisional.** It has no conformance suite yet,
  and a port without one is a guess. It gets frozen when the first real
  projection adapter can be built against it. The invariant it must preserve —
  read-model write and checkpoint write in one transaction — is documented on
  the trait.
- ~~**SQLite driver** (`rusqlite` vs `sqlx`).~~ Settled at phase 2 by building
  both: `happenstance-sqlite` is `rusqlite`, `happenstance-postgres` is `sqlx`,
  and the two are in the tree for different reasons rather than as candidates.
  The **append-condition SQL strategy** is still open and is ADR-0022's; notes
  are in `crates/happenstance-sqlite/src/`.
- **Replication semantics.** `SequencePosition` is meaningful only within one
  store, so positions cannot be replicated as-is. Whether ingest re-checks
  append conditions is the central unanswered question; it is written up in
  `crates/happenstance-sync/src/lib.rs`, along with the shape of the peer port
  itself and whether hub-and-spoke and peer-to-peer are one abstraction or two.
- **How a Postgres adapter buys position visibility.** `nextval()` allocates
  outside the transaction, so a Postgres store violates the visibility invariant
  by construction unless it does something about it. `xid8` +
  `pg_snapshot_xmin`, transaction-scoped advisory locks and a serialised sequence
  table each cost something real, and the choice is owed a measurement rather
  than a preference.
- ~~**Whether `happenstance-runtime` is the right name and the right seam.**~~
  Settled and executed: [ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md)
  gave the bare name to the typed layer and renamed the contract to
  `happenstance-core`; [ADR-0007](.kb/decisions/0007-projection-runner-decodes.md)
  corrected where the projection runner lives. Kept here struck through rather
  than deleted, because the crate names in older commits only make sense with it.
