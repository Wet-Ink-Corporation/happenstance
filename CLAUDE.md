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
crates/happenstance-ladybug/     graph projection store only. real and conformant at
                                 phase 11, on the real driver. publish = false, and here
                                 that means CANNOT: `lbug` does not render on docs.rs.
crates/happenstance-postgres/    the store that does not serialise its writers. both roles
                                 real and conformant at phase 10b. published from `0.2.0`,
                                 by the release-set decision recorded at `e597c34`. carries
                                 TWO projection stores: the buffered one an application
                                 uses, and `LivePostgresProjectionStore` — a live `sqlx`
                                 transaction as the batch — the instrument that stood at the
                                 far end of PS-2's axis and let ADR-0063 freeze the port.
crates/happenstance-neon/        Postgres over one-shot HTTP: no connection, no interactive
                                 transaction, no cursor. real and conformant at phase 10b
                                 against a live endpoint — and the adapter that falsified
                                 ES-11. host + wasm32. published from `0.2.0`.
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
  seeds/                           raw material for future planning. problem and vision only.
docs/                            user documentation. nothing else.
runbook/                         the plan of record, and how far it has got. start a
                                 session at runbook/handover.md, then the status table in
                                 runbook/README.md. one file per open phase; the roadmap
                                 to 1.0 in runbook/roadmap.md.
RUNBOOK.md                       the frozen monolith the runbook was split from. phases
                                 0–12 in full. never shrink it: ~2,250 `RUNBOOK.md:N`
                                 citations resolve against its line numbers.

.kb/                             the knowledge base — what is settled.
  decisions/                       the ADRs, one atom each. accepted ones are immutable.
  concepts/ governance/            explanations; the binding constraints, one atom each.
  playbooks/ reference/            transferable practice; measurements and pointers.
  open-questions/                  what is deliberately not settled.
  maps/                            the indexes: domain, decisions, open questions.
  product/ design/                 personas and journeys; signed-off design patterns.
  _intake/                         staging for atoms, written into .kb/ by hand.
.bklg/                           the redkiln backlog, frozen 2026-09-28. read, never edit.
.redkiln/                        redkiln's config and telemetry, frozen. redkiln is retired here.
```

**🔩 skeleton** means real associated types and `todo!()` bodies, `publish =
false`, and a scoped `#![allow(clippy::todo)]` naming the phase that removes it.
A skeleton exists to be disagreed with by a type checker — it is an *instrument*
first and a target second, and it is not an adapter until it has run the
conformance suite. **Four have stopped being skeletons**: `happenstance-sqlite`
at phase 8, `happenstance-cloudflare` at phase 9, and `happenstance-postgres` and
`happenstance-neon` at phase 10b. **All four are in the release set**, the last two
by the decision recorded at `e597c34`, which re-opened a set the owner had settled
at five. What that episode left behind is a habit worth keeping: a `publish = false`
saying "not in this release" and one saying "not finished" look identical in a
manifest, so a crate that carries the key says which in its own crate root.

`happenstance-postgres` was the **half case** and is no longer one. Its event
store ran 101 of 101 against a live PostgreSQL 17.10 including the concurrency
family at 64 contenders, and ADR-0024 records the arm it buys ES-10 with; its
projection store now clears the projection suite, and it is the first projection
adapter over storage this workspace does not control to do so. Both crates'
`#![allow(clippy::todo)]` left with their last stub, which is the contract those
allows were written under.

**One carries the marker whole, and it is `happenstance-sync`.** Read each crate's
own root rather than this paragraph for which side of the line it is on — a count
in a file that loads on every task is a count nobody re-reads, which this file
already says one section down and has now been wrong about twice.

**Two kinds of `publish = false` live in this workspace and they are not the same
fact.** `happenstance-sync` is unfinished. `happenstance-ladybug` is finished and
**cannot** be published: `lbug`'s build script returns early under `DOCS_RS` before
emitting the `cargo:rustc-env` lines its own `lib.rs` requires, so an undefined
`env!` makes the docs.rs build a compile error — and rendering on docs.rs is phase
12's bar for a published crate. The manifests look identical; each crate root says
which it means.

There was a **third** kind until `e597c34` — finished, and held out of this release
— and it had two members, `happenstance-postgres` and `happenstance-neon`. That
category is now empty, and it is recorded here rather than deleted because the two
crate roots argued from it for a while after it stopped being true. `grep -n
'^publish' crates/*/Cargo.toml` is the answer that cannot go stale, and it returns
exactly two lines.

Dependency rule: **everything depends on `happenstance-core`; `happenstance-core`
depends on nothing in this workspace.** No adapter may depend on another adapter.

One deliberate exception, and it is a port relationship rather than a dependency
between adapters: `happenstance-sync` is itself a port crate. Peer adapters
depend on it the way store adapters depend on `happenstance-core`, and its
conformance suite will live in `happenstance-sync-testkit`, which does not exist
yet. It stays out of the contract crate so that publishing `happenstance-core`
never waits on replication.

## Where the work lives

**Redkiln is retired in this repository as of 2026-09-28.** Tracking is manual
until `redkiln-rs` is live, which is after happenstance's 1.0. Do not run
`redkiln` commands or the `/redkiln:*` skills here: the trees they wrote are now
records, and a command run against them re-opens a process nobody is following.

- **[`runbook/`](runbook/README.md) — what is being done, and how far it has got.**
  The only tracker. Start at `runbook/handover.md`; the status table in
  `runbook/README.md` is held to the changelog and the registry by
  `cargo xtask lints`.
- **`.kb/` — what is settled.** Durable knowledge as *atoms*: markdown with
  frontmatter. **An accepted decision atom is immutable** — correcting one means
  writing a new atom that supersedes it, never editing the body. Atoms are now
  written by hand, from `.kb/_intake/`: copy the frontmatter shape of a
  neighbouring atom of the same kind, add the atom to the right map under
  `.kb/maps/`, and remove the intake file it came from.

  **Accepted decisions are checked; nothing else in `.kb` is.** `cargo xtask
  lint-kb`, a gate step, fails when an atom under `.kb/decisions/` that read
  `status: accepted` at the base commit has a changed body or has gone. In its
  frontmatter only `status`, `superseded_by`, `last_reviewed` and the pointer
  keys `source_paths` and `related` may change, and `accepted` may become only
  `superseded`, naming its successor, so a supersession passes (`wi-5f78c4`).
  Nothing outside `.kb/decisions/` is checked, and one body repair passes: a
  repointed `path.ext:N` citation, as `kb-governance-referent-not-reasoning-001`
  allows (`wi-5fce24`). CI's
  `gate` job checks out full history so the base is there (`wi-80cba0`), and the
  step fails, rather than skipping, on a clone with no base.

  **A decision lives in two places on purpose.** `.kb/decisions/` holds the
  numbered *atoms*, ~115 lines each, carrying the status and the supersession
  graph. `references/adr/` holds the full original records, up to 1,508 lines,
  carrying the compiler transcripts, the rejected alternatives and the measurement
  tables a summary cannot hold. Link the atom; cite the record by `file:line`.
  Deleting the second because the first exists would discard about 76% of the
  corpus, and `spec-trace` will catch you, because `spec/SPECIFICATION.md` cites
  line ranges that only exist in the long form. Not every atom has a long-form
  record, and `ls .kb/decisions/` is the count — it is deliberately not written
  here.
- **`.bklg/` — a frozen record.** The backlog as redkiln left it on 2026-09-28: three
  initiatives (one an empty template), 190 stories, none at done, because phases 10–12 and releases
  `0.2.0` – `0.3.2` were run from the runbook rather than through it. Its
  `spec.md` bodies remain useful planning material — phases 13, 14 and 20 point
  at them — but its stages and statuses are not current and are not to be edited.
- **`.redkiln/` — frozen.** Config, the pinned process pack and templates, and
  telemetry. The `backlog` CI job that checked it has been `if: false` since
  2026-09-04 and stays that way.

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
   **weigh the floor, because since `0.2.0` it has been a promise.** The old
   text here said *"nothing is published, so no downstream consumer is pinned to
   anything"*, and that was ADR-0004's whole reason for carrying a **provisional**
   marker. `0.2.0` is what the marker named as its own end, and it shipped on
   2026-09-10. Measured against the registry on 2026-09-29: all seven crates are
   published at `0.2.0`, `0.3.0`, `0.3.1` and `0.3.2`; the three `0.2.0-alpha.1`
   releases are yanked; each `0.0.0` reservation still stands; and
   `max_stable_version` reads `0.3.2` everywhere. So **a consumer can be pinned to
   the floor now**, and the registry semver baseline has run against the published
   versions since `86a410c`. Since that tag, raising the floor
   is a breaking change that needs a decision record rather than a commit message.
   Raising it at phase 2 was a deliberate trade recorded in an ADR, which is what
   the old text asked for; what stays forbidden is moving it in silence, and the bar
   for moving it at all is higher than it was before `0.2.0`. From `1.0.0` the rule is
   [ADR-0067](.kb/decisions/0067-msrv-after-1-0-rises-are-bounded.md)'s. A rise ships
   only in a minor, only to a stable at least six months old, and always with a
   CHANGELOG entry. This is needed because a 1.x minor reaches a consumer through
   `cargo update`, which a 0.x minor never did.

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
the old `factory =` spelling is gone with no deprecated arm, because nothing was
published then. One fixture instance is one isolated backing store; each
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
```

`affected` is the scoped gate for a change in progress and `ci --fast` the bar for
an intermediate step; neither replaces `cargo xtask ci` before a merge.

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
and a `cargo package --list` assertion that each of the **seven** publishable
crates — `happenstance-core`, `happenstance`, `happenstance-testkit`,
`happenstance-sqlite`, `happenstance-cloudflare`, `happenstance-postgres` and
`happenstance-neon` — carries both licence files
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
citations cannot rot into decoration. [`runbook/`](runbook/README.md) says
**who settles what is still open, and when**. Where a summary below disagrees
with a clause, the clause wins; the summaries are orientation only.

Changing a `[FROZEN]` clause requires a new ADR, not an edit.

- ~~**The projection store port is provisional.**~~ Frozen by
  [ADR-0063](references/adr/0063-the-projection-port-is-frozen.md), after
  [ADR-0062](references/adr/0062-the-probe-seam-moves-and-the-far-end-is-built.md)
  moved `begin` and the probe seam and built the far end of PS-2's batch-shape
  axis — `LivePostgresProjectionStore`, a `sqlx` transaction as the batch,
  passing all seventeen rules against a live server. The invariant — read-model
  write and checkpoint write in one transaction — is documented on the trait
  and enforced by `commit_is_atomic_with_the_read_model`. What was **still
  open** one layer up is now decided:
  [ADR-0074](.kb/decisions/0074-projection-apply-is-async.md) makes the typed
  layer's `Projection::apply` async, handed a `Delivered<E>` and a batch it can
  issue statements through, on `experiments/apply-shape`'s evidence against a
  live Postgres. Phase 18 builds it; until then the shipped `apply` is still
  synchronous, which is why `happenstance`'s `unstable-projection` still gates
  the *runner*, while `happenstance-core`'s feature of the same name is retained
  empty for `0.2.0` manifests and gates nothing until `0.4.0` removes it.
- ~~**SQLite driver** (`rusqlite` vs `sqlx`).~~ Settled at phase 2 by building
  both: `happenstance-sqlite` is `rusqlite`, `happenstance-postgres` is `sqlx`,
  and the two are in the tree for different reasons rather than as candidates.
  ~~The **append-condition SQL strategy**~~ is ADR-0022's as amended by
  [ADR-0065](.kb/decisions/0065-adr-0022-s11-busy-timeout-is-fifteen-seconds.md)
  (§11, the busy timeout) and
  [ADR-0068](.kb/decisions/0068-adr-0022-sections-8-9-16-settled.md) (phase 16):
  §8 is settled on the correlated `EXISTS` chain seeded by the most selective
  tag, and the rest is ratified. **§9 alone stays open** — whether the runtime
  `Handle` a store captures at construction strands a read that outlives its
  runtime — and its reproduction is phase 17's, because the remedy may change
  two published adapters' constructors.
- **Replication semantics.** `SequencePosition` is meaningful only within one
  store, so positions cannot be replicated as-is. Whether ingest re-checks
  append conditions is **no longer open**. SY-1 – SY-6 are frozen: ingest never
  refuses, a conflict is compensated by an ordinary append, and the origin's
  condition is evidence rather than an instruction. SY-7, which assigns
  compensation authorship to one peer, is provisional. The peer port's shape is
  frozen too: one relationship per `SyncPeer`, and hub-ness is not a type
  (SY-8, SY-9). SY-10, which makes hub-and-spoke and peer-to-peer both
  first-class, is provisional. What is still open is deferred by clause: whole-log
  or scoped replication (SY-27), bulk-ingest cost (SY-14), peer limits (SY-18)
  and the retention floor (SY-32). Phase 13 builds against all of them.
- ~~**How a Postgres adapter buys position visibility.**~~ `nextval()` allocates
  outside the transaction, so a Postgres store violates the visibility invariant
  by construction unless it does something about it. Settled by
  [ADR-0024](.kb/decisions/0024-position-visibility-mechanism.md), which chose
  `xid8` + `pg_snapshot_xmin` on a measurement rather than a preference. The
  invariant's *scope* is settled too:
  [ADR-0071](.kb/decisions/0071-es-10-stays-global.md) (phase 16) keeps ES-10
  global, because ADR-0063 froze a single-position checkpoint, and names what
  reopens ADR-0024 — the cluster-wide staleness coupling, not throughput.
- ~~**Whether `happenstance-runtime` is the right name and the right seam.**~~
  Settled and executed: [ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md)
  gave the bare name to the typed layer and renamed the contract to
  `happenstance-core`; [ADR-0007](.kb/decisions/0007-projection-runner-decodes.md)
  corrected where the projection runner lives. Kept here struck through rather
  than deleted, because the crate names in older commits only make sense with it.
