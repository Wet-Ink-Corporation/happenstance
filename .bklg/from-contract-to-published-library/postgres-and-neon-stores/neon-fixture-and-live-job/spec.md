---
item: HS-S0068
stage: spec
created: 2026-08-12T13:47:06.902Z
updated: 2026-08-12T13:47:06.902Z
template_sig: 87bbf1d0
rendered_sig: aa38c197
---

# Spec — NeonFixture against a live branch, and its credentialed CI job

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 6 (a store with no connection, no interactive transaction and no cursor passes the suite, or the contract is amended by decision record); BR-02, BR-13 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the ten-project portfolio, its traceability matrix and its DAG |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-007, AC-011; DR-4, DR-5, DR-6, DR-8, DR-9; the risk row "the Neon endpoint gets faked with a pooled Postgres connection" |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-fixture-and-live-job/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — *Architecture brief* §5 (Neon's contracts and data flow), §6 (the two fixtures), §7 (gate and CI wiring), §9.2 (what implements `SqlTransport`, consumed here, not re-decided); *Testing brief* Notes §5, §6, §7; *Deployment brief* Notes ("CI implication, concretely") |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **N/A by sign-off**: this project records no user-facing surface, approved 2026-08-12. This story renders none. |
| Story map row | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md` — slice `neon-live-suite`, row 1; and the `neon-fixture-and-live-job` paragraph under *What each story is* |
| Discover | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-fixture-and-live-job/discover.md` — the signal ledger, the four questions deferred **to this spec** (§Questions 5–8), and the named wrong implementations |
| Roadmap pointer | `RUNBOOK.md:4356-4363` — phase 10's work items: live infrastructure in its own CI job, and "every rule Neon cannot pass is a reported capability skip or an amended clause" |

## One-line PR slice

`NeonFixture` against a live branch: `SECOND_HANDLE` answered as the MUST it is,
`REOPEN` and `MID_BATCH_FAULT` answered honestly, the 64 MiB-anchored ceilings
stated as numbers, plus the credentialed Neon CI job that runs it.

## Executive summary

**Lands:** the first fixture in this workspace whose backing store is not on the
machine running the tests — and the job that gives it somewhere to run.

Today `crates/happenstance-neon/` has no `tests/` directory and **no
`[dev-dependencies]` at all** (`crates/happenstance-neon/Cargo.toml:14-22`). Its
capability limits — no connection, no transaction handle, no cursor, one round
trip per operation, a hard 64 MiB response cap — exist only as a markdown table
in a doc comment (`crates/happenstance-neon/src/lib.rs:18-24`). A doc comment
cannot be skipped, cannot be reported, and cannot be wrong in a way anything
notices. This PR turns that table into `Capability` and `Option<usize>` constants
that the conformance harness reads, prints and is bound by.

**Delta against the project charter.** The two ACs this story traces to are
halves of two different criteria. `project.md` **AC-007** ("nothing Neon cannot
do passes silently") splits at `_storymap.md` *Coverage*: this story owns
*capabilities declared with real reasons*, and `neon-append-and-read-over-http`
owns *every rule present in the run and reporting*. `project.md` **AC-011** ("the
default gate stays Docker-free and network-free") splits by job: `postgres-schema-and-live-fixture`
owns the Postgres job, and this story owns the Neon one — which differs from its
Postgres sibling in the one way that matters operationally, because it needs a
**credential** rather than a daemon, and a credential is not available to every
event that can trigger a workflow.

**Delta against the story map.** The map's rule is that "a fixture no macro is
invoked with is not delivered," so this PR also creates the tests target that
invokes `event_store_conformance!(NeonFixture::…)`. It does **not** make that
invocation pass: `conditional_append_request`, `decode_append_response` and
`decode_read_response` are still `todo!()`
(`crates/happenstance-neon/src/event_store.rs:159-165`, `:249-255`, `:257-263`)
and become real in the slice-mate `neon-append-and-read-over-http`. The slice is
green as a slice, in one context; this story is green as the thing the slice-mate
mounts into.

**What it consumes rather than decides.** `neon-sql-transport` (HS-S0067) is a
hard `depends_on` and it answers *Architecture brief* §9.2's three questions —
ships or dev-dependency, `cargo deny` admissibility, `wasm32` `cfg`-gating —
before this story starts. `NeonFixture` builds its store over whatever that story
produced. Re-opening any of the three here is scope drift, not thoroughness.

## Context pack

The load-bearing decisions this story must honor. Everything deeper is a
signposted anchor; nothing below is optional reading.

**1. The fixture's backing store is a real Neon `/sql` endpoint, and this is an
acceptance criterion rather than an implementation note.** DR-4 states it in
terms: substituting a pooled Postgres connection "destroys the exact axis the
adapter exists to occupy and is not an acceptable fixture." The reason it has to
be *criterion*-grade is that **no automated check in this repository can tell the
two apart**. A `SqlTransport` shim that speaks to the `testcontainers` Postgres
the sibling story already stands up, and returns Neon-shaped JSON, makes every
existing check pass and pass *better* — faster, never flaky, `capability_skips_are_reported`
satisfied, the whole suite green. And then `spec/SPECIFICATION.md`'s residual
exposure on ES-11 and ES-12, which is `[PROVISIONAL]` **on the transport axis
specifically** (`spec/SPECIFICATION.md:3003`, `:371`), is reported as discharged
by an instrument that never left the machine. The endpoint, the branch and the
secret are therefore part of the acceptance criteria (`discover.md`, *The wrong
implementation*).

**2. `SECOND_HANDLE` is a MUST and Neon answers it `SUPPORTED`.** CF-16
(`spec/SPECIFICATION.md:7548`) requires every fixture to be able to open a second
handle, and the enforcement is inside the rule rather than in a meta-test:
`two_handles_observe_each_others_appends` **panics** on a decline, quoting the
fixture's own words, rather than reporting a skip
(`crates/happenstance-testkit/src/contract.rs:135-161`;
`crates/happenstance-testkit/src/suite.rs:265`). For Neon the answer is cheap and
honest — a second client against the same branch — because every request is
authenticated and routed on its own and there is no session to share
(`crates/happenstance-neon/src/lib.rs:18-24`). What makes it real is that the
second handle addresses the **same table set** on the **same branch**, which is
the same decision as §3's isolation scheme seen from the other side.

**3. Isolation is per fixture instance, and over one-shot HTTP the only lever is
the table name.** CLAUDE.md's fixture rule is that one fixture instance is one
isolated backing store, and `two_fixture_instances_observe_none_of_each_others_appends`
(`crates/happenstance-testkit/src/suite.rs:210`) is the rule that catches "every
fixture points at one temporary path." The Postgres sibling can buy isolation
with a per-instance schema and a `search_path` set on the pool's `after_connect`;
**Neon cannot**, because there is no connection to configure and every statement
arrives standalone. The lever that does exist is `NeonConfig::with_event_table`
(`crates/happenstance-neon/src/config.rs:38-43`), which the store threads into
every statement it compiles — so a per-instance table name, or a per-instance
schema plus fully qualified names, is how one instance stops being another.
Whichever is chosen is recorded with its reason, together with how it is torn
down (`discover.md` Q8).

**4. The ceiling to state is not the constant.** `MAX_RESPONSE_BYTES` is 64 MiB
and hard, "a hard ceiling rather than a default: there is no cursor to fall back
on when a result set exceeds it" (`crates/happenstance-neon/src/transport.rs:49-54`).
But parameters cross the wire as JSON, so a `bytea` is rendered as a `\x…` hex
string, which "roughly doubles its size against `MAX_RESPONSE_BYTES` **in both
directions**" (`crates/happenstance-neon/src/transport.rs:90-94`). Copying the
constant into `MAX_EVENT_DATA_LEN` is the arithmetic mutant `discover.md` names:
the rule appends exactly the stated number of bytes and **requires acceptance**,
so an over-stated ceiling is reported as a defect in the adapter
(`crates/happenstance-testkit/src/contract.rs:236-247`,
`crates/happenstance-testkit/src/suite.rs:4282`). The spec's obligation is the
**derivation**, written down, not just the number.

**5. A ceiling is a fact; a decline is a trade; and the two are different types
on purpose.** The three ceilings are `Option<usize>` and not `Capability`
(`crates/happenstance-testkit/src/contract.rs:213-279`): `None` is a store
reporting a fact about itself, with nothing it could have done differently and no
reason it owes anybody, and asking it to write one "would put a fiction in the CI
log." What `None` shares with a decline is only the *reporting* obligation — it
still emits a `RuleOutcome::Skipped` carrying `NO_STORE_LIMITS` / `NO_CEILING_REASON`
(`contract.rs:442,454`). CF-40's ownership is contested between ADR-0012 and
ADR-0015 (`.kb/open-questions/cf-40-fixture-limits-ownership.md`) and DR-6 says
**consume, do not settle**: if no sibling has settled it when this lands, state
the ceilings honestly and record the unresolved ownership rather than minting a
policy here.

**6. A stated ceiling and the guaranteed minimum are independent, and both are
owed an answer.** VT-21 makes 65,536 bytes a floor every store must clear
(`crates/happenstance-core/src/limits.rs:22`; `spec/SPECIFICATION.md:1483`), with
64 tags per event and 128 events per batch beside it (`limits.rs:27,43`). A store
**MAY** state a ceiling below the floor and will then correctly fail
`store_accepts_the_guaranteed_minimum_payload`
(`crates/happenstance-testkit/src/suite.rs:3775`) — the rules are independent
(`contract.rs:249-252`). So "set the number low to be safe" is not safe: it
drops Neon below VT-21 without anyone deciding to, and quietly.

**7. Every capability constant is an answer; a silent default is the failure
mode.** `REOPEN` is a SHOULD and Neon is durable, so a decline needs a real
reason, not a convenient one — and note the trap `MemoryFixture` documents at
`crates/happenstance-testkit/src/fixtures.rs:275-284`: "a fixture that 'reopened'
by doing nothing would pass `acknowledged_writes_survive_a_reopen` **vacuously**."
`MID_BATCH_FAULT` **defaults to declined** (`contract.rs:207-211`), so taking the
default is indistinguishable from never having thought about it; the architecture
brief singles that out as the outcome to avoid (§6). And `Capability::declined("")`
is rejected by an `assert!` in a `const fn` that, for an *associated* const, fires
at **codegen** — `cargo build` and `cargo test` catch it, `cargo clippy` does not
(`contract.rs:404-412`). A green clippy proves nothing about these constants.

**8. The decline path is the only sanctioned way for Neon to not pass a rule.**
DR-5: a rule Neon cannot pass is either a reported capability skip carrying the
fixture's stated reason, or a clause amended by an accepted decision record with
the suite re-run — **never a silent pass and never a `#[cfg]`-ed-out test**. A
declined capability still emits a test returning `RuleOutcome::Skipped`
(`crates/happenstance-testkit/src/contract.rs:32-37`), because a test that was
never emitted is indistinguishable in CI output from one that passed. Gating the
**whole invocation** on infrastructure availability is a different act from making
one rule vanish, and only the second is forbidden (*Architecture brief* §7).

**9. `connect` panics, and for this fixture the panic message is an operational
interface.** `Fixture::connect` returns `impl Future<Output = Self::Store>` with
no `Result` (`crates/happenstance-testkit/src/contract.rs:309-321`), deliberately:
a fixture that cannot connect is a broken **test environment**, not a
non-conformant adapter, and a `Result` would put "the endpoint is down" into the
same channel as "the adapter is wrong." This is the first fixture in the workspace
where the environment includes a third party's uptime and a rotating secret, and
the message is read from a CI log by someone who cannot reproduce it locally
without the credential. It must name what was reached and what was missing — and
never the secret's value.

**10. The Neon job needs a credential, which is what makes it unlike its Postgres
sibling.** The nearest in-tree precedent is the `backlog` job reaching a private
repository through `secrets.REDKILN_TOKEN` (`.github/workflows/ci.yml:102-131`).
Two consequences the Postgres job does not have: the job cannot run where the
secret is unavailable (a fork pull request being the ordinary case), and a secret
that is present but wrong looks exactly like an endpoint that is down. DR-9 and
`project.md`'s risk table forbid the easy answer — "a flaky live job is fixed or
reported, never quietly made non-blocking" — so the secret-absent behaviour must
be a **stated design**, not an emergent one (`discover.md` Q7).

**11. A live job that is green because it ran nothing is the quiet failure this
story ships if it is careless.** It is the direct consequence of §8: the same
whole-invocation gate that keeps `cargo test --workspace --all-features` green
with no credential makes a misconfigured job exit zero with `running 0 tests`.
The in-tree pattern for refusing that already exists — `xtask/src/proof.rs` lists
a target's tests with `cargo test -- --list` and asserts the expected names are
present *before* running them, written because "`cargo test` exits 0 on `running 0
tests`, so an emptied file passes a step that a deleted one fails."

**12. Neon implements the bare flavour, so the concurrency family is not a
decline — it is unspellable.** `NeonEventStore` implements bare `EventStore` on
both targets, and a second `SendEventStore` impl is `error[E0119]` against
`trait_variant`'s blanket impl (`crates/happenstance-neon/src/lib.rs:79-89`).
`event_store_concurrency_conformance!`'s bound is `F::Store: EventStore + Send`
and "a `!Send` adapter cannot invoke it and is not expected to"
(`crates/happenstance-testkit/src/lib.rs:101-110`). Its absence from this story's
tests target is therefore a type-level fact recorded in a comment, not a capability
declined and not a gap (DR-8).

**13. Nothing here pre-empts a question another story owns.** There is no
`Capability` for `conflicting_position`, and inventing a decline-shaped workaround
would settle by omission what `neon-conflicting-position-verdict` must settle by
evidence (AC-008). `ProbeThenWriteStore` (`crates/happenstance-neon/src/event_store.rs:388-503`)
is consumed by a later story, never rebuilt and never fixed here. The ES-10
mechanism and its migration-1 column are `adr-0024-position-visibility-mechanism`'s;
Neon inherits both and chooses neither.

**The persona-journey slice.** The "user" is an adapter author
(`_storymap.md` preamble: "the 'user' here is an adapter author and a consuming
application, not a screen"). Before this story, everything the workspace knows
about Neon's limits is a claim in a doc comment that nothing reads. After it,
there is a fixture that states those limits where the harness must obey them, and
one command — run by one job, against one real branch — that prints them back,
rule by rule, with the reason attached to every skip. The adapter author stops
having to take the crate's word for what it cannot do.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `foundation` — real in-tree substrate consumed by the capability stories beside it. No double, no shim, no fixme. |
| **Slice / milestone** | `neon-live-suite`. Slice-mates: `neon-append-and-read-over-http`, `neon-conflicting-position-verdict`. Implemented together in one context and mounted as one integrated surface. `depends_on`: `neon-sql-transport` (HS-S0067), merged first, supplying the real `SqlTransport`. |
| **Mount point** | `crates/happenstance-neon/tests/neon_conformance.rs` (**new**) — Root B of *Architecture brief* §2: the file that invokes `event_store_conformance!(NeonFixture::…)`. The macro takes an **expression building a `Fixture`** precisely so a fixture can be handed an endpoint URL and a credential rather than reaching into the environment from an argument-less constructor (`crates/happenstance-testkit/src/lib.rs:265-274`, `:312`). Second mount, inseparable from the first: the new live-Neon job in `.github/workflows/ci.yml`, a sibling of `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver` (`:279`) and `advisories` (`:325`). A fixture no macro is invoked with, or a macro no workflow runs, is not delivered. |
| **Wires into** | `Fixture`, `Capability`, `RuleOutcome`, `NO_STORE_LIMITS` / `NO_CEILING_REASON` (`crates/happenstance-testkit/src/contract.rs:120-353`, `:372-454`); `event_store_conformance!` (`crates/happenstance-testkit/src/lib.rs:312`); `NeonEventStore::new(transport, config)` and its `transport()` / `config()` accessors (`crates/happenstance-neon/src/event_store.rs:88-107`); `NeonConfig` and `with_event_table` (`crates/happenstance-neon/src/config.rs:5-56`); `SqlTransport` / `SqlRequest` / `SqlStatement` / `HttpResponse` and `MAX_RESPONSE_BYTES` (`crates/happenstance-neon/src/transport.rs:54,97,129,219,245`); the real transport implementation delivered by `neon-sql-transport`; `MIN_SUPPORTED_EVENT_DATA_LEN` / `MIN_SUPPORTED_TAGS_PER_EVENT` / `MIN_SUPPORTED_EVENTS_PER_BATCH` (`crates/happenstance-core/src/limits.rs:22,27,43`); `MemoryFixture` as the reference shape to copy, not to import (`crates/happenstance-testkit/src/fixtures.rs:270-292`). |
| **Renders surfaces** | **none.** `_design.md` records this project as having no user-facing surface, signed off 2026-08-12 — every `## Items` / `## Signatures` / `## Anti-patterns` section reads "N/A — no user-facing surface". There is no surface id to claim and nothing to perceive; `design.capture` is deliberately absent from `.redkiln/config.yaml:75-81`, which makes the perceptual review a declared skip rather than a silent pass. |
| **Conformance rule(s)** | This story adds **no rule** to `happenstance-testkit` — the testkit is a must-not-change seam here (*Architecture brief* §1). It makes existing rules *reachable* against a store with no connection, and it fixes what several of them will do. Directly governed by this story's constants: `two_handles_observe_each_others_appends` (`suite.rs:265`, panics rather than skips on a declined `SECOND_HANDLE`); `two_fixture_instances_observe_none_of_each_others_appends` (`suite.rs:210`, the isolation scheme is answerable to it); `acknowledged_writes_survive_a_reopen` (`suite.rs:332`), `reopened_store_does_not_reissue_an_event_id` (`suite.rs:2310`) and `recorded_time_survives_a_reopen` (`suite.rs:2473`), all three gated on `REOPEN`; `append_is_atomic_under_a_mid_batch_fault` (`suite.rs:2706`), gated on `MID_BATCH_FAULT`; `append_reports_exceeded_store_limits` (`suite.rs:4282`) and `store_accepts_the_guaranteed_minimum_payload` (`suite.rs:3775`), both gated on the three ceilings. Rule **outcomes** belong to `neon-append-and-read-over-http`; reachability and the honesty of every constant belong here. |
| **Clause(s)** | Discharges none on its own. It makes the phase-10 owner rows for **ES-11** and **ES-12** (`RUNBOOK.md:606`) reachable by putting a store at the far end of the **transport** axis — the axis `spec/SPECIFICATION.md:3003` and `:371` name as empty at both ends and as the reason those clauses are `[PROVISIONAL]`. It amends **no** clause: no `[FROZEN]` marker is touched, so **no ADR is owed by this story** (`discover.md`, *Decision*). CF-16 (`:7548`), CF-18 (`:7597`) and CF-40 (`:7661`) bind the code it writes; CF-33 (`:8236`, no rule reads a clock) binds any retry or wait it is tempted to add. |
| **Advances DoD scenario** | Initiative **DoD 6** — "a store with no connection, no interactive transaction and no cursor passes the suite, or the contract is amended by decision record and the suite re-run." This story does not observe DoD 6; it is what makes observing it possible, since every remaining step needs a fixture pointed at a real endpoint and a job that runs it. |

## PR boundary

**In this PR**

- `NeonFixture` in the Neon crate's own `tests/`: the `Fixture` impl, `connect()`
  building a `NeonEventStore` over the real `SqlTransport` against a live branch,
  and an owning handle that carries a refcount rather than borrowing the fixture.
- All six constants answered deliberately — `SECOND_HANDLE`, `REOPEN`,
  `MID_BATCH_FAULT`, and the three `Option<usize>` ceilings — each with its reason
  or its derivation written where a reader of the fixture meets it.
- The per-instance isolation scheme on a shared branch (per-instance table name,
  or per-instance schema with qualified names), the DDL that creates it, and its
  teardown — including what happens to the leavings of a crashed run.
- The schema DDL the Neon fixture owns, and the note reconciling it with the
  Postgres crate's migration 1 (see *Data and migrations*).
- The tests target that invokes `event_store_conformance!(NeonFixture::…)` over
  its full expansion, plus the whole-invocation gating that keeps it out of the
  default gate.
- The Neon crate's first `[dev-dependencies]`, target-scoped so the `wasm32`
  steps stay green, and checked against `deny.toml`'s allowlist.
- The new live-Neon job in `.github/workflows/ci.yml`: its secret, its trigger,
  its branch lifecycle, its `--show-output`, and the assertion that it cannot pass
  without having executed the gated tests.
- A rustdoc note recording the ceiling derivation beside the constant it is
  derived from, so the next reader does not re-derive it from `MAX_RESPONSE_BYTES`
  and get it wrong.
- This story's own backlog folder (`spec.md`, `_ledger.md`).

**Explicitly not in this PR**

- `conditional_append_request`, `decode_append_response` and `decode_read_response`
  — they stay `todo!()` (`crates/happenstance-neon/src/event_store.rs:159-165`,
  `:249-255`, `:257-263`); `neon-append-and-read-over-http` owns them, and with
  them every conformance rule *outcome*.
- The `SqlTransport` implementation itself, its ship-vs-dev-dependency verdict,
  its licence check and its `cfg` scoping — `neon-sql-transport` (HS-S0067) owns
  all four (*Architecture brief* §9.2). This story consumes the answer.
- `ProbeThenWriteStore` (`crates/happenstance-neon/src/event_store.rs:388-503`) —
  not rebuilt, not fixed, not deleted, and not wired into a negative test here;
  that is `neon-conflicting-position-verdict`'s.
- The `conflicting_position` verdict, and any capability invented to stand in for
  it. There is no `Capability` for it and there must not be one (Context pack §13).
- ADR-0024, the ES-10 mechanism, or the migration column it adds.
- Any change to `crates/happenstance-testkit/**` or `crates/happenstance-core/**`.
- `publish = false`, `PUBLISHABLE`, the scoped `#![allow(clippy::todo)]`
  (`crates/happenstance-neon/src/lib.rs:101-105`), or the licence/README files —
  `deskeleton-and-package-readiness` owns every one of them.
- `crates/happenstance-postgres/**`, and any dependency edge from this crate to
  it. An adapter may not depend on another adapter (CLAUDE.md, *Dependency rule*),
  and a dev-dependency is still an edge.
- Any widening of `deny.toml`'s licence allowlist. A dev-dependency that fails
  `cargo deny check` is a finding to report, not an allowlist to grow.

The implementer MAY touch the composition-root and wiring files named in the
Integration contract — the new tests target, `crates/happenstance-neon/Cargo.toml`,
the root `Cargo.toml`/`Cargo.lock` and `.github/workflows/ci.yml` — to mount this
slice. That is the mount, not scope drift.

```
crates/happenstance-neon/src/**
crates/happenstance-neon/tests/**
crates/happenstance-neon/Cargo.toml
Cargo.toml
Cargo.lock
.github/workflows/ci.yml
.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-fixture-and-live-job/**
```

`crates/happenstance-neon/src/**` is in the boundary for the ceiling-derivation
rustdoc note and nothing else; every `todo!()` under it is listed above as
out of scope.

**Merge DoD.** `cargo xtask ci` is green on a clean checkout with no Neon
credential and no network, with the gated tests reported as *ignored* rather than
absent; the new live-Neon job is green on the same tree with a nonzero count of
executed tests against a real `/sql` endpoint; the `wasm32` build of
`happenstance-neon` (`xtask/src/main.rs:271`) is still green and `wasm_steps()`'s
by-name selection (`xtask/src/main.rs:784-791`) is untouched; and every `todo!()`
the crate held before it is still there.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **`NeonFixture` is built from an endpoint and a credential, as an expression** | `event_store_conformance!` takes an expression rather than a type exactly so a fixture that needs a URL does not have to reach into the environment from an argument-less constructor. The environment read belongs in the tests target's own setup, in one place, where a missing variable can be reported once and clearly. | `crates/happenstance-testkit/src/lib.rs:265-274`, `:312` |
| **`connect()` builds a `NeonEventStore` over the real transport** | `NeonEventStore::new(transport, config)` is `const` and takes the transport by value; the store exposes `transport()` and `config()` as `const fn`, which is the seam this story's own tests use to send raw SQL while `append` is still `todo!()`. | `crates/happenstance-neon/src/event_store.rs:88-107` |
| **The handle owns a refcount, not a borrow** | `Fixture::Store` is an ordinary associated type. The borrowing GAT shape (`type Store<'a> where Self: 'a`) on a foreign trait is one of five independently necessary ingredients of a rustc ICE that still reproduces on 1.97.1. A client handle cloned out of the fixture is the shape to copy. | `crates/happenstance-testkit/src/contract.rs:97-111`; `crates/happenstance-testkit/src/fixtures.rs:286-291` |
| **Nothing but a real Neon `/sql` endpoint** | No pooled Postgres, no local shim returning Neon-shaped JSON, no `NullTransport`. The substitution is the project risk table's row 3 and DR-4's named failure, and no automated check in the repository can detect it — the job definition is the only instrument. | `project.md` DR-4 and *Risks* row 3; `discover.md`, *The wrong implementation* |
| **One fixture instance is one isolated backing store** | Over one-shot HTTP there is no session and no `search_path`, so the lever is the table name threaded through `NeonConfig`. Per-instance table name, or per-instance schema with fully qualified names. Record which, and why. | `crates/happenstance-neon/src/config.rs:9-23,38-43`; `crates/happenstance-testkit/src/suite.rs:210` |
| **The isolation namespace is created and torn down** | The fixture issues its own DDL over the transport. Teardown must survive a crashed run: a leftover table from a killed job may not poison the next one, so names are unique per instance and there is a stated sweep (age-based drop at job start, or drop-if-exists per name). | *Architecture brief* §6; `discover.md` Q8 |
| **Branch lifecycle is a stated design, not an emergent one** | Three arms: one long-lived CI branch with per-instance table namespacing (needs only the `/sql` credential); a branch per job run via Neon's control API (a second credential and a second HTTP surface, outside `SqlTransport`'s remit); a branch per fixture instance (the suite constructs many, so this is a cost claim that must be measured before it is chosen). Pick one, say why, say how it is cleaned up. | `discover.md` Q8; *Deployment brief* Notes, "CI implication, concretely" |
| **`SECOND_HANDLE = Capability::SUPPORTED`** | A second client against the same branch and the **same table set**. It is the only MUST among the capabilities, and declining it makes the rule panic quoting the fixture's own words rather than skip. | `crates/happenstance-testkit/src/contract.rs:135-161`; `spec/SPECIFICATION.md:7548` (CF-16) |
| **`REOPEN` is answered, and if supported it discards something real** | Neon's durable medium is a real Postgres branch and the handle carries only a client and a config, so a reopen that builds a **fresh client** genuinely re-reads the medium — unlike `MemoryFixture`, which declines precisely because "a fixture that 'reopened' by doing nothing would pass `acknowledged_writes_survive_a_reopen` vacuously." An empty `reopen()` body is the vacuous answer; a declared `SUPPORTED` with no override **panics** through the provided body. | `crates/happenstance-testkit/src/contract.rs:167-173,326-345`; `crates/happenstance-testkit/src/fixtures.rs:275-284` |
| **`MID_BATCH_FAULT` is answered, not defaulted** | It defaults to declined, so silence is indistinguishable from not having thought about it. The Neon-specific reasoning is that an append is **one non-interactive statement**, so there is no point between two rows a caller can reach — but the fixture can send DDL over the same transport, and a trigger armed to raise on row *n* is the shape Postgres uses. Either answer is admissible; the reason must be Neon's, and if supported, `arm_mid_batch_fault` must be overridden or the provided body panics. | `crates/happenstance-testkit/src/contract.rs:207-211,281-307`; *Architecture brief* §6 |
| **The three ceilings are derived, and the derivation is written down** | Anchor: `MAX_RESPONSE_BYTES = 64 MiB`, hard, no cursor behind it. Adjustment: hex `bytea` rendering roughly doubles a payload against that anchor **in both directions**. The stated `MAX_EVENT_DATA_LEN` is therefore materially below the constant, and `MAX_EVENTS_PER_BATCH` interacts with it (a batch is many payloads in one request). Copying the constant is the arithmetic mutant. | `crates/happenstance-neon/src/transport.rs:49-54,90-94`; `discover.md`, *The arithmetic mutant* |
| **A stated number is a commitment the adapter must honour** | The rule appends exactly the stated bytes and requires **acceptance**, then one more and requires `AppendError::ExceedsStoreLimit { limit: StoreLimit::… }` — never `AppendError::Store`, never truncation. Since `append` is `todo!()` in this story, the numbers stated here are what `neon-append-and-read-over-http` must then enforce; a number chosen carelessly here is a bug reported against that story. | `crates/happenstance-testkit/src/contract.rs:236-247`; `crates/happenstance-testkit/src/suite.rs:4282` |
| **The VT floors are cleared or the failure is deliberate** | 65,536 bytes, 64 tags, 128 events per batch. A store MAY state a ceiling below a floor and will then correctly fail `store_accepts_the_guaranteed_minimum_payload`; the two rules are independent and both are owed an answer. If Neon's derived ceiling sits below a floor, that is a finding recorded in the ledger, not a number nudged upward to make a rule green. | `crates/happenstance-core/src/limits.rs:22,27,43`; `crates/happenstance-testkit/src/suite.rs:3775`; `crates/happenstance-testkit/src/contract.rs:249-252` |
| **No `Capability::declined("")`, and no reason of convenience** | The empty string is rejected by an `assert!` in a `const fn`, but for an associated const it fires at **codegen**: `cargo build` and `cargo test` catch it, `cargo clippy` does not. A plausible-but-untrue reason is caught by nothing at all, which is the half of AC-007 that only review enforces. | `crates/happenstance-testkit/src/contract.rs:404-412` |
| **`connect` panics informatively, and never prints the secret** | No `Result` by design. The message names the endpoint host, the branch, the table namespace and whether a credential was present — never its value — so a reader of the CI log can tell "environment" from "adapter" without a local reproduction they cannot perform. | `crates/happenstance-testkit/src/contract.rs:309-321`; `discover.md`, signal row on `connect` |
| **The macro is invoked over its full expansion** | `event_store_conformance!(NeonFixture::…)` from the crate's own `tests/`, no hand-picked subset — otherwise "no rule is absent from the run" is unproven by construction. The rule list comes from `for_each_event_store_rule!` and is written in exactly one place. | `crates/happenstance-testkit/src/lib.rs:85-90`, `:312`; `crates/happenstance-testkit/src/registry.rs:94` |
| **The concurrency family is not invoked, and the reason is type-level** | `NeonEventStore` implements the bare `EventStore` only; a second `SendEventStore` impl is `error[E0119]` against `trait_variant`'s blanket impl. The concurrency macro's bound is `F::Store: EventStore + Send`, and a `!Send` adapter "cannot invoke it and is not expected to." Record it as a comment at the mount, not as a declined capability. | `crates/happenstance-neon/src/lib.rs:79-89`; `crates/happenstance-testkit/src/lib.rs:101-110`; `project.md` DR-8 |
| **The model family's absence is a recorded choice** | `event_store_model_conformance!` is behind the testkit's `proptest` feature and `not(target_arch = "wasm32")`. No project AC asks for it against Neon (AC-006 names `event_store_conformance!`), and generated operation sequences against a metered remote endpoint is a cost claim nobody has made. Not invoking it is fine; not saying so is not. | `crates/happenstance-testkit/src/lib.rs:93-100`, `:181-183`; `project.md` AC-006 |
| **The gating is whole-invocation** | `#[ignore]` (run in the job with `-- --ignored`), or an env read inside the test binary that fails loudly rather than passing quietly. **Never** a `#[cfg]` hiding a rule out of the macro's expansion (DR-5). Note the mechanical trap the Postgres sibling recorded: `required-features` does not work, because the default gate runs `--all-features` and therefore *enables* the required feature and runs the target against infrastructure that is not there. | *Architecture brief* §7; *Testing brief* Notes §6; `xtask/src/main.rs:116,143` |
| **The default gate compiles the mount, and runs nothing that needs a credential** | `clippy --workspace --all-targets --all-features -D warnings` compiles the new tests target and therefore the macro's full expansion, which is what stops the mount from rotting; `cargo test --locked --workspace --all-features` must still exit zero with no endpoint reachable. | `xtask/src/main.rs:116,143`; `.redkiln/config.yaml:40,55` |
| **The job is a sibling, not a step** | A new job in `.github/workflows/ci.yml` beside `gate`, `backlog`, `wasm-conformance`, `msrv`, `semver`, `advisories`. `xtask/src/main.rs`'s `REQUIRED` array is unchanged in kind, and `wasm_steps()` selects steps **by name** and panics on a miss — so nothing here may repoint `cargo xtask wasm`. | `.github/workflows/ci.yml:31,102,204,241,279,325`; `xtask/src/main.rs:784-791,816-823` |
| **The credential is an Actions secret, and its absence is designed for** | The `backlog` job reaching a private repository through `secrets.REDKILN_TOKEN` is the nearest precedent for "a job that needs a credential the default `gate` does not." A fork pull request has no secret; the job may not report success in that case, may not `continue-on-error`, and may not be quietly made non-blocking. Which trigger it runs on, and what a credential-less event produces, are stated in the workflow. | `.github/workflows/ci.yml:102-131`; `project.md` DR-9 and *Risks*, last row |
| **The job cannot be green having run nothing** | It asserts a nonzero executed-test count and the presence of the expected rule names before running them — the in-tree shape is `cargo test -- --list` followed by a name assertion, written because `cargo test` exits 0 on `running 0 tests`. Reuse the shape; do **not** add an `ARTEFACTS` row, which would put a live endpoint inside the default gate. | `xtask/src/proof.rs`; `crates/happenstance-testkit/src/contract.rs:32-37` (CF-18's argument) |
| **`--show-output` so the declines are readable** | A declined capability's reason is the deliverable of this story, and it is only observable in the harness output. The job runs with `--show-output` and the log is the evidence AC-007's "written reason visible in the harness output" asks for. | *Testing brief* Notes §6; `project.md` AC-007 |
| **The wasm32 steps do not regress** | `happenstance-neon` is the only crate that must build for both targets and is checked twice. New dev-dependencies must be target-scoped so `cargo check -p happenstance-neon --target wasm32-unknown-unknown` stays green, and the `wasm32 feature powerset` step is what catches a feature that is not target-scoped. | `xtask/src/main.rs:271,564`; `project.md` DR-8 |
| **New dev-dependencies clear the licence allowlist** | `deny.toml` allows MIT / Apache-2.0 / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib with `[graph] all-features = true`. The measured in-tree note is that `sqlx`'s `tls-rustls` fails on `webpki-roots`, *not* on `ring`. Run `cargo deny check` before committing to a pin; growing the allowlist is a gate weakening and is not this story's to take. | `deny.toml:8-20`; `crates/happenstance-postgres/src/lib.rs:38-47` |
| **No adapter depends on another adapter** | The Neon crate may not gain a dependency — including a dev-dependency — on `happenstance-postgres`. That is why the fixture owns its own DDL rather than reading migration 1 from the sibling crate. | CLAUDE.md, *Dependency rule*; *Architecture brief* §1, §9.1 |
| **No rule reads a clock** | CF-33. Any retry, backoff or availability wait this story is tempted to add around a remote endpoint lives in the **fixture or the workflow**, never inside a conformance rule, and never as a timeout the suite depends on. | `spec/SPECIFICATION.md:8236`; `xtask/src/main.rs:361` |
| **The `todo!()`s stay** | The three request/decode bodies and the scoped `#![allow(clippy::todo)]` are untouched. A green run of this PR's own tests proves the fixture, the namespace and the job — not the adapter. | `crates/happenstance-neon/src/event_store.rs:159-165,249-263`; `crates/happenstance-neon/src/lib.rs:101-105` |

## Data and migrations

**Real, and awkwardly so: this story authors a schema it does not own the
canonical copy of.**

Neon is Postgres over one-shot HTTP and inherits migration 1 — the identity and
time columns, the tag storage and the append-condition SQL — which is exactly why
Postgres and Neon are one project and not two (`project.md`, *Coupling notes*).
The canonical migration is authored by `postgres-schema-and-live-fixture` under
`crates/happenstance-postgres/`. **This crate cannot read it**: no adapter may
depend on another adapter (CLAUDE.md, *Dependency rule*), a dev-dependency is
still an edge, and reaching across the workspace with `include_str!("../../happenstance-postgres/…")`
would break `cargo package` for a crate that `deskeleton-and-package-readiness`
is about to make publishable (`xtask/src/package.rs`, `REQUIRED_FILES` /
`PUBLISHABLE`).

So the Neon fixture **owns its own copy** of the DDL, and the copy is a known,
recorded coupling rather than an accident:

- The DDL lives with the fixture, under `crates/happenstance-neon/tests/`, and is
  applied over the transport — DDL is a statement like any other, and the endpoint
  runs it in the same one-shot way it runs everything else.
- It must produce the **same column set** migration 1 produces, because the
  slice-mate compiles the CTE at `crates/happenstance-neon/src/lib.rs:54-68` and
  `crates/happenstance-neon/src/event_store.rs:134-148` against it. A divergence
  does not fail this story; it fails the slice-mate, which is a worse place to
  discover it, so the copy carries a comment naming the file it mirrors and the
  story that owns it.
- **No second migration, and no migration framework.** `sqlx` is not in this
  crate's dependency graph at all and must not enter it — the Neon crate's whole
  claim is that it owns no connection. Applying DDL through `SqlTransport` is the
  only mechanism consistent with that claim.

**Per-instance namespacing is the schema decision this story actually makes.**
The Postgres sibling can put each fixture instance in its own schema and set
`search_path` on the pool's `after_connect`. Neon has no connection to configure,
so isolation has to be visible in every statement — which means the table name
`NeonConfig` carries (`crates/happenstance-neon/src/config.rs:9-11,38-43`). Two
admissible arms:

| Arm | What the fixture does | What it costs |
| --- | --- | --- |
| Per-instance **table** | `with_event_table("event_<unique>")` and DDL creating that table on the shared branch | Cheapest. Every instance's table sits in one schema, so the sweep is a name-prefix drop and the branch accumulates tables until it runs |
| Per-instance **schema**, qualified names | `CREATE SCHEMA hs_<unique>` plus `with_event_table("hs_<unique>.event")` | One `DROP SCHEMA … CASCADE` cleans up everything including the checkpoint table; costs a qualified name in every statement, which the config already supports |

Whichever is chosen, the uniqueness source must not be a clock read inside a
conformance rule (CF-33) — the fixture is not a rule, and generating the name at
fixture construction is where that distinction is kept.

**No backfill, no compatibility window, no consumer.** `happenstance-neon` is
`publish = false` (`crates/happenstance-neon/Cargo.toml:12`) and has never shipped
a schema to anyone; the branch this story writes to exists to be written to by CI.
The only durable data question is **teardown**, and it is an operational one: a
crashed job leaves a namespace behind on a real, billed, third-party branch, so
the sweep is part of the deliverable rather than a nicety (see *Behavior and
interfaces*, "The isolation namespace is created and torn down").

**Deliberately absent.** No visibility-mechanism column and no `xid8` anything —
that column belongs to migration 1 and to `adr-0024-position-visibility-mechanism`,
which has not landed. Adding one here would encode an answer this story has no
standing to give, and would put the copy out of step with the canonical migration
in the same stroke.

## Acceptance criteria

The "user" of this story is an **adapter author** on the *Learn when you are
finished* journey (`.bklg/from-contract-to-published-library/initiative.md:243-245`),
with the **constrained-runtime developer** (*Event-source at the edge*, `:246-247`)
downstream of every number stated here and the **evaluator** (*Decide in one
sitting*, `:248-249`) reading the CI log as public evidence. Each criterion is
that person's goal crossing the whole stack — fixture, endpoint, harness output,
workflow — not a capability in isolation.

Two ACs (**AC-002**, **AC-004**) name a conformance rule as a *later* verifier as
well as a first-party one. That is deliberate and is the slice contract: `append`
is `todo!()` in this PR, so the story-grain proof runs raw SQL through
`NeonEventStore::transport()` (`crates/happenstance-neon/src/event_store.rs:88-107`),
and the same property is re-proved by the rule itself once
`neon-append-and-read-over-http` lands in the same slice. Ledger evidence for
those rows cites the first-party test; the rule is the slice's own check, not
this story's excuse.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author who needs to know whether the contract survives a store with no connection, no interactive transaction and no cursor, **WHEN** the Neon suite is run, **THEN** the store under test reached a real Neon `/sql` endpoint over the transport `neon-sql-transport` delivered — and the fixture cannot be satisfied by a pooled Postgres connection, a local shim returning Neon-shaped JSON, or `NullTransport`, because the endpoint is read from the job's credentialed environment and a loopback or non-`/sql` endpoint is rejected at construction. | `crates/happenstance-neon/tests/neon_fixture.rs::fixture_refuses_a_local_endpoint` (constructs the fixture with a loopback URL and asserts the panic names the substitution). Plus the `neon` job in `.github/workflows/ci.yml`, which stands up **no** database service — the job definition is the only instrument that can tell the real endpoint from a convincing shim (`project.md` DR-4; `discover.md`, *The wrong implementation*), so this row also carries a named reviewer check in the slice review. |
| AC-002 | **GIVEN** an adapter author running the suite twice over on one shared CI branch, **WHEN** the harness constructs two `NeonFixture` instances, **THEN** neither instance can see a row the other wrote — isolation bought by a per-instance namespace threaded through `NeonConfig` into every statement, because there is no connection and no `search_path` to set. | `crates/happenstance-neon/tests/neon_fixture.rs::two_fixture_instances_do_not_share_a_namespace` — two instances, rows inserted through each one's `store.transport()` with raw SQL, each instance's count unchanged by the other's write. Re-proved by `two_fixture_instances_observe_none_of_each_others_appends` (`crates/happenstance-testkit/src/suite.rs:210`) once the slice-mate lands. |
| AC-003 | **GIVEN** a maintainer who pays for a real, billed third-party branch, **WHEN** a job finishes normally *or* is killed mid-run, **THEN** the namespace it created is gone — dropped at teardown in the normal case, and swept by name prefix at the start of the next run in the killed case — and a leftover namespace can never be silently adopted by a later instance instead of a fresh one. | `crates/happenstance-neon/tests/neon_fixture.rs::namespace_is_dropped_at_teardown` and `::a_leftover_namespace_is_never_adopted` (pre-creates a namespace matching the prefix, asserts the next instance mints a distinct name). The prefix sweep itself is a step in the `neon` job, asserted by `::sweep_removes_only_the_test_prefix`. |
| AC-004 | **GIVEN** an adapter author checking that Neon can meet the one capability the suite treats as a MUST, **WHEN** the harness asks for a second handle, **THEN** `NeonFixture::SECOND_HANDLE` is `Capability::SUPPORTED` and the handle it hands back is a second client addressing the **same branch and the same table set**, so the second handle observes what the first wrote — never a decline, which would make `two_handles_observe_each_others_appends` panic quoting the fixture's own words rather than report a skip. | `crates/happenstance-neon/tests/neon_fixture.rs::second_handle_addresses_the_same_table_set` — writes through handle A's transport, reads through handle B's, asserts the row is visible and that both handles resolve the same qualified table name. A companion assertion pins `<NeonFixture as Fixture>::SECOND_HANDLE` as supported. Re-proved by `two_handles_observe_each_others_appends` (`crates/happenstance-testkit/src/suite.rs:265`) once the slice-mate lands. |
| AC-005 | **GIVEN** an adapter author who has been burned by a fixture that "reopened" by doing nothing, **WHEN** they read `NeonFixture::REOPEN` and its `reopen()`, **THEN** the answer is deliberate and non-vacuous: `SUPPORTED` with a `reopen()` that discards the client and builds a fresh one against the same endpoint and namespace, so `acknowledged_writes_survive_a_reopen` re-reads a durable medium rather than an untouched in-process handle — and never an empty `reopen()` body, and never a `SUPPORTED` declaration left on the provided body, which panics. | `crates/happenstance-neon/tests/neon_fixture.rs::reopen_builds_a_fresh_client` — asserts the post-`reopen()` handle is a distinct client (not the same instance) and that a row written before the reopen is read back through it by raw SQL. Should the implementer's live evidence force a decline instead, the same test asserts a non-empty, Neon-specific reason and the reversal is recorded under *Clarifications*. Re-proved by `acknowledged_writes_survive_a_reopen` (`suite.rs:332`), `reopened_store_does_not_reissue_an_event_id` (`:2310`) and `recorded_time_survives_a_reopen` (`:2473`). |
| AC-006 | **GIVEN** an adapter author who cannot tell "we decided" from "nobody looked", **WHEN** they read `NeonFixture::MID_BATCH_FAULT`, **THEN** it is written explicitly in the impl with Neon's own reason — not inherited from the trait's in-memory default, whose text is about a store that "has no fault to inject" and would be a fiction here — and if the answer is `SUPPORTED`, `arm_mid_batch_fault` is overridden rather than left on the panicking provided body. | `crates/happenstance-neon/tests/neon_fixture.rs::mid_batch_fault_is_an_explicit_answer` — asserts the constant's reason string differs from the default at `crates/happenstance-testkit/src/contract.rs:207-211` and is non-empty. The reason's *truth* is a reviewer check in the slice review, and the rejected arm (a DDL-armed trigger) is recorded in the fixture's rustdoc so the reason is real rather than convenient. |
| AC-007 | **GIVEN** a constrained-runtime developer deciding whether Neon can carry their payloads, **WHEN** they read the three ceilings, **THEN** each is a number derived from `MAX_RESPONSE_BYTES` **after** the hex `bytea` doubling and measured against the live endpoint — not the 64 MiB constant copied, and not a small number chosen "to be safe" — with the derivation written as rustdoc beside the constants it is derived from. | `crates/happenstance-neon/tests/neon_fixture.rs::ceilings_are_derived_not_copied` — asserts each of `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH` is `Some(_)`, that `MAX_EVENT_DATA_LEN` is strictly below `MAX_RESPONSE_BYTES / 2` (`crates/happenstance-neon/src/transport.rs:49-54,90-94`), and that it is not equal to `MAX_RESPONSE_BYTES`. A live bisection test, `::stated_ceiling_is_where_the_endpoint_actually_refuses`, sends the stated size and the stated size plus one through the transport and requires acceptance then refusal. The rustdoc is compiled by the gate's `docs` step (`xtask/src/main.rs`). |
| AC-008 | **GIVEN** an application author reading whether Neon clears the minima every store must clear, **WHEN** they compare the stated ceilings against VT-21/VT-22/VT-24, **THEN** either all three clear `MIN_SUPPORTED_EVENT_DATA_LEN` (65,536), `MIN_SUPPORTED_TAGS_PER_EVENT` (64) and `MIN_SUPPORTED_EVENTS_PER_BATCH` (128), or the shortfall is recorded as a finding with its measured number in this story's ledger evidence — never a number nudged upward to make `store_accepts_the_guaranteed_minimum_payload` green. | `crates/happenstance-neon/tests/neon_fixture.rs::stated_ceilings_are_compared_against_the_guaranteed_minima` — compares each constant against `happenstance_core::limits::{MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_TAGS_PER_EVENT, MIN_SUPPORTED_EVENTS_PER_BATCH}` (`crates/happenstance-core/src/limits.rs:22,27,43`) and fails with the shortfall named rather than asserting a pass. Re-proved by `store_accepts_the_guaranteed_minimum_payload` (`crates/happenstance-testkit/src/suite.rs:3775`), which the two rules' independence (`contract.rs:249-252`) makes a separate answer. |
| AC-009 | **GIVEN** an adapter author reading a red CI log for a job they cannot reproduce locally without a credential they do not hold, **WHEN** `connect()` fails, **THEN** the panic tells them it is the **environment** and not the adapter — naming the endpoint host, the branch, the namespace, and whether a credential was present — and the credential's *value* appears nowhere in the message or the log. | `crates/happenstance-neon/tests/neon_fixture.rs::connect_failure_names_the_environment` (`#[should_panic(expected = …)]` against an unreachable endpoint, asserting host, namespace and credential-presence appear) and `::connect_failure_never_prints_the_credential` (constructs with a sentinel secret value, asserts the sentinel is absent from the captured panic payload). |
| AC-010 | **GIVEN** a contributor on a clean checkout with no Neon credential and no network, **WHEN** they run `cargo xtask ci` before saying "done", **THEN** it exits zero — the new tests target and the macro's **full expansion** are compiled by clippy and by the doc build, so the mount cannot rot, and the credentialed tests are reported as *ignored* rather than being absent from the binary. | `cargo xtask ci` on a credential-less checkout (`xtask/src/main.rs:105`'s `REQUIRED`, `clippy --workspace --all-targets --all-features -D warnings` at `:116`, `tests` at `:143`). Concretely evidenced by `cargo test --locked --workspace --all-features -p happenstance-neon` reporting the gated tests as `ignored` with a nonzero ignored count and a zero failure count, and by the presence of exactly one `event_store_conformance!` invocation in `crates/happenstance-neon/tests/neon_conformance.rs` with no hand-picked rule subset beside it. |
| AC-011 | **GIVEN** a maintainer merging a change that could break Neon, **WHEN** CI runs, **THEN** a `neon` job — a sibling of `gate`, `backlog`, `wasm-conformance`, `msrv`, `semver` and `advisories`, not a step inside `gate` — runs the suite against the live branch using an Actions secret, and its behaviour where that secret is unavailable is **stated in the workflow**: red rather than a quiet skip where the secret should be present, and not scheduled at all on a fork pull request, which is merged through a branch where it does run. It carries no `continue-on-error` and is never made non-blocking without a recorded decision. | Review of the `neon` job in `.github/workflows/ci.yml` against the `backlog` precedent (`:102-131`, "Absent, this job is RED rather than skipped"). Mechanically asserted by `crates/happenstance-neon/tests/neon_fixture.rs::the_neon_job_is_not_continue_on_error`, which parses the workflow from the repository root and fails on `continue-on-error` under the `neon` job or on the job's absence — a cheap check because DR-9's failure mode is a one-line edit under time pressure. |
| AC-012 | **GIVEN** an evaluator reading the CI log as public evidence that Neon's limits are real, **WHEN** the `neon` job goes green, **THEN** it demonstrably executed the expected rules rather than nothing — the expected test names are asserted out of `cargo test -- --list` *before* the run, and the run's executed count is nonzero — and the log carries `--show-output`, so every declined capability's stated reason is readable rather than swallowed by the harness's default capture. | The `neon` job's own two steps in `.github/workflows/ci.yml`: a `--list` name assertion following the argument at `xtask/src/proof.rs:15-21` (`cargo test` exits 0 on `running 0 tests`), then `cargo test -p happenstance-neon --all-features -- --ignored --show-output` (*Testing brief* Notes §6). The reason text reaching the log is what `project.md` AC-007's "written reason visible in the harness output" asks for. |

## Interaction quality

**Composition family: N/A by sign-off.** This project's signed-off design
(`.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md`,
approved 2026-08-12) records **no user-facing surface** — every `## Items`,
`## Signatures` and `## Anti-patterns` section reads "N/A — no user-facing
surface". There is no composition, transience policy, density budget, hierarchy
or named visual anti-pattern to honour, and `design.capture` is deliberately
absent from `.redkiln/config.yaml:75-81` so the perceptual review is a declared
skip rather than a silent pass. Inventing a surface here to fill this section
would contradict a design a human has already signed off.

**State family: real, in the medium this story actually has.** The adapter
author's only observable is the harness output and the panic text, and the
state-invariant vocabulary maps onto it exactly rather than by analogy. Every
invariant below is already carried by an **AC row in the table above** — none is
introduced here, because a bullet in this section would get no ledger row and
would never be gated.

| Invariant | Reads here as | Carried by | Verified by |
| --- | --- | --- | --- |
| **Non-occlusion** | A declined capability's reason is the deliverable, and the harness's default capture hides it. Nothing the story produces may be visible only to a process that already knows the answer. | **AC-012** | `--show-output` in the `neon` job, plus the `--list` name assertion that proves the output came from a run |
| **In-place, not a context jump** | A contributor running the default gate is never bounced into acquiring a credential or a network to get a green result; the mount still compiles in place. | **AC-010** | `cargo xtask ci` on a credential-less checkout; gated tests reported `ignored`, not absent |
| **Reversibility** | Every side effect this story has on a real, billed third-party branch is undoable — at teardown in the normal case, by prefix sweep after a kill. | **AC-003** | `::namespace_is_dropped_at_teardown`, `::sweep_removes_only_the_test_prefix` |
| **Preserved state across a re-entry** | A "reopen" that preserved *everything* would be vacuous; a reopen must discard the client and preserve only what the medium holds. | **AC-005** | `::reopen_builds_a_fresh_client` |
| **Legibility of a failure, and the right channel for it** | An environment fault must read as an environment fault, at the point of failure, with enough to act on and without leaking the credential. | **AC-009** | `::connect_failure_names_the_environment`, `::connect_failure_never_prints_the_credential` |
| **Reachability** | The keyboard analogue: every rule in the family is reachable from the one command a person types, with no hand-picked subset standing between them and the registry. | **AC-010**, **AC-012** | Exactly one `event_store_conformance!` invocation; `--list` name assertion |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The endpoint/credential environment variable is absent in the test binary. | The gated tests do not run and are reported as **ignored**; they never pass quietly. Inside the `neon` job, where the variable is expected, absence is a hard failure naming the variable — not a skip (AC-010, AC-011). |
| EC-002 | The endpoint is unreachable — DNS failure, connection refused, 5xx, or a timeout. | `connect()` panics with a message naming the host, the branch and the namespace, and saying a credential *was* present. No `Result`, by design: "the endpoint is down" must not enter the same channel as "the adapter is wrong" (`crates/happenstance-testkit/src/contract.rs:309-321`). AC-009. |
| EC-003 | A credential is present but wrong or expired — 401/403. | Distinguished in the message from EC-002, because a rotated secret and a dead endpoint are the same symptom to a reader who cannot reproduce either. The message says the credential was rejected; it never prints it. AC-009. |
| EC-004 | A response exceeds `MAX_RESPONSE_BYTES`. | Surfaced as the transport's error. There is no cursor to fall back on (`crates/happenstance-neon/src/transport.rs:49-54`), so nothing may truncate and call it success. The transport's own handling is `neon-sql-transport`'s; this story's obligation is that the stated ceilings keep the suite from provoking it (AC-007). |
| EC-005 | The namespace DDL fails because a name already exists — the leavings of a killed run. | The instance mints a distinct name rather than adopting the existing one; adoption would break AC-002's isolation silently. The prefix sweep at job start is what stops the branch accumulating (AC-003). |
| EC-006 | Teardown fails — the network dies between the last rule and the `DROP`. | Teardown failure is reported but does **not** turn an otherwise-green conformance run red: a rule outcome and a cleanup outcome are different claims. The next run's prefix sweep is what makes this recoverable rather than cumulative (AC-003). |
| EC-007 | `Capability::declined("")` reaches the fixture. | Rejected by the `assert!` in a `const fn` — but for an *associated* const it fires at **codegen** (`crates/happenstance-testkit/src/contract.rs:404-412`), so `cargo build` and `cargo test` catch it and `cargo clippy` does not. A green clippy is not evidence for AC-005 or AC-006. |
| EC-008 | `Capability::SUPPORTED` declared without the matching override. | The provided `reopen()` / `arm_mid_batch_fault()` bodies **panic** (`contract.rs:281-307,326-345`). This is the intended failure and must not be softened; AC-005 and AC-006 both assert against it. |
| EC-009 | Two `neon` job runs overlap on the shared branch. | The job declares its own `concurrency` group so they cannot, following the workflow's existing reasoning at `.github/workflows/ci.yml:20-26`. Without it, one run's prefix sweep would delete another run's live namespace, and the failure would look like an adapter defect. |
| EC-010 | The credential appears in a log line, an annotation or an uploaded artifact. | Forbidden outright. AC-009's second test asserts it for the panic path; the job masks the secret and uploads nothing containing it. |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| NF-001 | The default gate's wall-clock and dependency surface are unchanged: no network, no Docker, no credential (`project.md` AC-011, DR-9). | `cargo xtask ci` on a clean checkout. This is the one requirement proven by the **absence** of infrastructure (*Testing brief*, AC-011 row). |
| NF-002 | The live job's request and namespace budget is bounded and stated. The suite constructs many fixture instances and every operation is one round trip (`crates/happenstance-neon/src/lib.rs:18-24`), so the branch lifecycle arm must not scale namespaces or branches with rule count in an unbounded way. | The chosen arm — one long-lived CI branch, one namespace per fixture instance — is stated with its cost in *Clarifications*; the sweep bounds accumulation (AC-003). |
| NF-003 | No conformance rule reads a clock (CF-33, `spec/SPECIFICATION.md:8236`). Any retry, backoff or availability wait around a remote endpoint lives in the **fixture or the workflow**. Namespace uniqueness is generated at fixture construction, which is where that line is kept. | `cargo xtask spec-trace`; the rule bodies are untouched by this story. |
| NF-004 | `happenstance-neon` still builds for `wasm32-unknown-unknown`. New dev-dependencies are target-scoped, and no new feature escapes host scoping. | `cargo xtask wasm` and the gate's `wasm32` build (`xtask/src/main.rs:271`); the `wasm32 feature powerset` step (`:564`) is what catches a feature that is not target-scoped. `wasm_steps()`'s by-name selection (`:784-791`) panics on a miss, so nothing here may rename those steps. |
| NF-005 | New dev-dependencies clear `deny.toml`'s allowlist (MIT / Apache-2.0 / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib, `[graph] all-features = true`, `deny.toml:8-20`) **without widening it**. The measured in-tree note is that `sqlx`'s `tls-rustls` fails on `webpki-roots`, not on `ring`. | `cargo deny check`, which resolves on this machine and therefore runs rather than skipping (CLAUDE.md, *Commands*). A failure is a finding to report, not an allowlist to grow. |
| NF-006 | The MSRV floor of 1.97.1 is not moved by anything this story adds. Dev-dependencies are invisible to the consumer-facing `--no-dev-deps` pass but not to the full test at 1.97.1, which is the pass that would catch a dev-dependency needing more. | The `msrv` job (`.github/workflows/ci.yml:241`); ADR-0029 (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`) on why a floor moves only by decision. |
| NF-007 | No adapter depends on another adapter, dev-dependencies included (CLAUDE.md, *Dependency rule*). This is why the fixture owns its own DDL rather than reading migration 1 from `happenstance-postgres`. | `cargo tree -p happenstance-neon` shows no `happenstance-postgres` edge; the gate's `package-check` (`xtask/src/package.rs`) is what would meet it later. |
| NF-008 | The credential is never written to a log, an annotation or an artifact, and the secret is masked in the job. | EC-010; AC-009's sentinel test. |

## Implementation notes (non-prescriptive)

Shape suggestions only. Where one of these disagrees with an AC, the AC wins.

- **Where the fixture lives.** `NeonFixture` is needed by two test targets — the
  macro mount and the fixture's own live assertions — so a shared, non-target
  module is the cheapest layout: `crates/happenstance-neon/tests/support/mod.rs`,
  with `mod support;` in both `neon_conformance.rs` and `neon_fixture.rs`. Cargo
  treats only top-level `.rs` files under `tests/` as targets, so a directory
  module is safe. The in-tree precedent for a test target with a companion
  directory is `crates/happenstance-testkit/tests/mutation_coverage.rs` beside
  `crates/happenstance-testkit/tests/mutation_coverage/`.
- **Copy `MemoryFixture`'s shape, do not import it.**
  `crates/happenstance-testkit/src/fixtures.rs:270-292` is the reference: the
  handle `connect()` hands out **owns** a refcount rather than borrowing the
  fixture, because `Fixture::Store` is an ordinary associated type and the
  borrowing GAT shape on a foreign trait is one of five ingredients of an ICE
  that still reproduces on 1.97.1 (`crates/happenstance-testkit/src/contract.rs:97-111`).
- **One environment read, in one place.** The macro takes an *expression*
  building a fixture precisely so the endpoint and credential can be handed in
  rather than reached for from an argument-less constructor
  (`crates/happenstance-testkit/src/lib.rs:265-274`). Read the variable once in
  the tests target's setup, report a missing one clearly there, and pass the
  value down. The spec calls the variable `HAPPENSTANCE_NEON_URL`; the final name
  is the implementer's, and it must appear in exactly two places — that setup
  function and the `neon` job.
- **Gating.** `#[ignore]` plus `-- --ignored` in the job is the simplest arm and
  is what the *Testing brief*'s merge-gate command already assumes. Note the trap
  the Postgres sibling recorded: `required-features` does **not** work here,
  because the default gate runs `--all-features` and therefore enables the
  required feature and runs the target against infrastructure that is not there
  (`xtask/src/main.rs:116,143`). Whatever arm is chosen, the gating is
  **whole-invocation**; a `#[cfg]` hiding one rule out of the macro's expansion
  is DR-5's forbidden move.
- **Namespace naming.** A process-unique suffix generated at construction — a
  counter plus a run identifier from the environment is enough and avoids a clock
  entirely, which keeps NF-003 uncontroversial. Prefix it distinctively (`hs_test_`)
  so the sweep can be exact rather than heuristic.
- **The DDL copy carries its provenance.** The comment above it names
  `crates/happenstance-postgres/` migration 1 and the story that owns it, so a
  divergence is found here rather than in the slice-mate's CTE
  (`crates/happenstance-neon/src/lib.rs:54-68`).
- **Ceilings are measured, then written.** Derive the candidate from
  `MAX_RESPONSE_BYTES / 2` less the JSON envelope, then bisect against the live
  endpoint and write the number the endpoint actually produced. A derivation that
  was never run against the branch is the arithmetic mutant with extra steps.
- **The `--list` assertion is a job step, not an `Artefact` row.** `xtask/src/proof.rs`
  is the shape to copy; adding a row to its list would put a live endpoint inside
  the default gate and break AC-010.

## Tests and CI (merge gate)

| tier | command / path | proves |
| --- | --- | --- |
| **Static / process (default gate)** | `cargo xtask ci` on a clean checkout, no Docker, no credential | AC-010, NF-001. The one bar proven by the absence of infrastructure (*Testing brief*, AC-011 row) |
| **Static — lint (default gate)** | `cargo clippy --workspace --all-targets --all-features -D warnings` (`xtask/src/main.rs:116`) | The new tests target and the macro's full expansion compile, so the mount cannot rot (AC-010). Note it does **not** catch `Capability::declined("")` — EC-007 |
| **Static — docs (default gate)** | the gate's `docs` step | The ceiling derivation rustdoc compiles beside the constants (AC-007) |
| **Unit / integration, host, no network (default gate)** | `cargo test --locked --workspace --all-features` (`xtask/src/main.rs:143`) | The gated tests are present and reported `ignored`, never absent; the codegen `assert!` on an empty decline reason fires here if it is going to (AC-010, EC-007) |
| **wasm32 (default gate)** | `cargo xtask wasm`; the `wasm32` build of `happenstance-neon` (`xtask/src/main.rs:271`) and the feature-powerset step (`:564`) | New dev-dependencies are target-scoped and no new feature escapes host scoping (NF-004, `project.md` DR-8) |
| **Supply chain (default gate, tool present)** | `cargo deny check` | New dev-dependencies clear the licence allowlist without widening it (NF-005) |
| **Fixture-live (the `neon` job)** | `cargo test -p happenstance-neon --all-features --test neon_fixture -- --ignored --show-output` | AC-001 – AC-009: the real endpoint, isolation, teardown and sweep, the second handle, `REOPEN`, `MID_BATCH_FAULT`, the derived ceilings and their comparison against VT-21/22/24, and the connect-failure message |
| **Conformance (the `neon` job)** | a `--list` name assertion, then `cargo test -p happenstance-neon --all-features --test neon_conformance -- --ignored --show-output` | AC-012: the run executed the expected rules rather than nothing, with every skip's stated reason readable. **Rule outcomes are the slice-mate's** — `append` is `todo!()` here, so the assertion this story owns is reachability and a nonzero executed count |
| **Workflow shape (default gate)** | `crates/happenstance-neon/tests/neon_fixture.rs::the_neon_job_is_not_continue_on_error` (runs on the host, no credential) | AC-011: the job exists, is a sibling job, and has not been quietly made non-blocking (DR-9) |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `verify.affected_gate`) | Only what this diff could break, which is the grain redkiln wires for a story |
| **Project grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml`, `verify.integration_scoped`) | The non-terminal project bar. Neither grain includes the live families, by construction (*Testing brief* Notes §6) |

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The endpoint is faked with a pooled Postgres connection** | Live HTTP in CI is genuinely awkward, and a `SqlTransport` shim over the sibling's `testcontainers` Postgres makes every check pass *better* — faster, never flaky. No automated check in this repository can tell the two apart (`project.md` DR-4, risk row 3) | AC-001 makes the endpoint a criterion rather than setup; the `neon` job stands up no database service; the substitution is a named reviewer check in the slice review, because the job definition is the only instrument |
| **The ceilings are copied from `MAX_RESPONSE_BYTES`** | It is the most defensible-looking number in the file — documented, named, public — and it is wrong by roughly a factor of two once hex `bytea` rendering is counted in both directions (`crates/happenstance-neon/src/transport.rs:90-94`). The failure surfaces as a *defect in the adapter* in the slice-mate, not here | AC-007's relational assertion plus the live bisection; the derivation written as rustdoc so the next reader does not re-derive it and get it wrong |
| **A decline of convenience** | "Every decline carries a real reason" is the half of `project.md` AC-007 that only review enforces; the empty string is caught at codegen, a plausible-but-untrue sentence is caught by nothing | AC-005 and AC-006 assert non-vacuity mechanically (fresh client; reason differs from the default); the rejected alternative is written into the rustdoc so the reason can be argued with |
| **The DDL copy drifts from migration 1** | The Neon fixture owns its own copy because no adapter may depend on another adapter, and the slice-mate compiles a CTE against the column set. A divergence fails the slice-mate, which is a worse place to find it | The copy carries a comment naming the file it mirrors and the story that owns it; the slice is implemented in one context, so the divergence surfaces within the slice |
| **The live job goes flaky and gets quietly weakened** | Two of this project's four gates need a server; the one-line edit under time pressure is `continue-on-error: true` (`project.md` DR-9 and risk table, last row) | AC-011's workflow assertion runs in the **default** gate, where it costs nothing and cannot be forgotten. DR-9 sanctions no "revert to skip" path |
| **The job is green having run nothing** | The same whole-invocation gate that keeps the default gate credential-free makes a misconfigured job exit zero on `running 0 tests` (`xtask/src/proof.rs:15-21`) | AC-012's `--list` name assertion before the run, and a nonzero executed count after it |
| **CF-40's contested ownership stalls the numeric declaration** | Two accepted decisions claim and disclaim it (`.kb/open-questions/cf-40-fixture-limits-ownership.md`) | DR-6: consume, do not settle. State the ceilings honestly, record the unresolved ownership, and mint no policy here |
| **Scope creep into the slice-mates** | `ProbeThenWriteStore` and the three `todo!()` bodies are right there, and fixing them makes the suite green | The PR boundary lists all four as out of scope; a green run of this PR proves the fixture, the namespace and the job — not the adapter |
| **Cost and leavings on a billed third-party branch** | A killed job leaves a namespace behind on infrastructure someone pays for | AC-003's teardown plus the prefix sweep at job start; EC-009's concurrency group so a sweep cannot delete a live run's namespace |

## Dependencies

**Blocks on**

- **`neon-sql-transport`** (HS-S0067) — hard. It supplies the real `SqlTransport`
  implementation `connect()` builds the store over, already proven by one small
  live round trip, and it answers *Architecture brief* §9.2's three questions
  (ships or dev-dependency, `cargo deny` admissibility, `wasm32` `cfg`-gating)
  before this story starts. Re-opening any of the three here is scope drift.
  Merged first (`_storymap.md`, *Merge order* item 4).

**Unlocks**

- **`neon-append-and-read-over-http`** — its `depends_on` names this story. It
  fills the three `todo!()` bodies and turns every rule this story made
  *reachable* into a rule that reports an outcome; the ceilings stated here are
  what it must then enforce.
- **`neon-conflicting-position-verdict`** — downstream of the slice-mate, and the
  only story permitted to settle whether the in-tree CTE keeps
  `conflicting_position`. This story must not pre-empt it with a decline-shaped
  workaround (Context pack §13).
- **`far-end-discharge-record`** — cannot record the status of ES-11 and ES-12
  until a store has actually stood at the far end of the transport axis.
- **`deskeleton-and-package-readiness`** — owns `publish = false`, `PUBLISHABLE`,
  the scoped `#![allow(clippy::todo)]` and the licence/README files, none of which
  this story touches.

**Not a dependency, deliberately**

- **`postgres-schema-and-live-fixture`** authors migration 1, and this fixture
  copies its column set rather than depending on it. An adapter may not depend on
  another adapter, and a dev-dependency is still an edge (CLAUDE.md, *Dependency
  rule*).
- **`adr-0024-position-visibility-mechanism`** owns the ES-10 mechanism and its
  migration column. Neon inherits both and chooses neither; nothing here adds a
  visibility column.

## Anchors (progressive disclosure)

Every path below exists in this worktree. Link, do not paste — but none of these
is optional at the moment its row names.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/contract.rs` | The whole `Fixture` contract: the six constants (`:135-161`, `:167-173`, `:207-211`, `:213-279`), the provided `reopen()` / `arm_mid_batch_fault()` bodies that panic when declared `SUPPORTED` without an override (`:281-345`), `connect`'s no-`Result` design (`:309-321`), and the codegen-time `assert!` on an empty decline reason (`:404-412`) | Before writing a single line of `NeonFixture` — this is the file the fixture is answerable to | AC-004, AC-005, AC-006, AC-007, AC-009 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` is the reference implementation: the refcount-owning handle (`:286-291`) and the note at `:275-284` on why a fixture that "reopened" by doing nothing passes `acknowledged_writes_survive_a_reopen` **vacuously** | While shaping `connect()` and `reopen()` — copy the shape, do not import the type | AC-005 |
| `crates/happenstance-neon/src/transport.rs` | `MAX_RESPONSE_BYTES` and the reason it is hard rather than a default (`:49-54`), and the hex `bytea` note that roughly doubles a payload against it **in both directions** (`:90-94`) — the two facts the ceiling derivation is built from | Before choosing any number for the three ceilings | AC-007, EC-004 |
| `crates/happenstance-neon/src/config.rs` | `NeonConfig` and `with_event_table` (`:38-43`) — the only lever isolation has over one-shot HTTP, because there is no connection and no `search_path` | Before deciding the namespacing arm | AC-002, AC-003 |
| `crates/happenstance-neon/src/event_store.rs` | `NeonEventStore::new` and the `const fn` `transport()` / `config()` accessors (`:88-107`) — the seam this story's own tests write raw SQL through while `append` is `todo!()`. Also the three `todo!()` bodies (`:159-165`, `:249-263`) that must still be there when this PR merges | When writing the first-party live tests, and again before committing | AC-002, AC-004, AC-005, AC-007 |
| `crates/happenstance-neon/src/lib.rs` | The capability table that has only ever been a doc comment (`:18-24`) — this story's whole premise — and the bare-`EventStore`-only impl with the `error[E0119]` note (`:79-89`) that makes the concurrency family unspellable rather than declined | At the start, for the premise; at the mount, for the comment recording why no concurrency macro is invoked | AC-001, AC-010 |
| `crates/happenstance-testkit/src/lib.rs` | `event_store_conformance!` and why it takes an **expression** building a fixture (`:265-274`, `:312`); the concurrency family's `EventStore + Send` bound and "a `!Send` adapter cannot invoke it and is not expected to" (`:101-110`) | When writing the mount file and its comment | AC-010 |
| `crates/happenstance-testkit/src/suite.rs` | The rules this story's constants govern: `two_fixture_instances_observe_none_of_each_others_appends` (`:210`), `two_handles_observe_each_others_appends` (`:265`), the three `REOPEN`-gated rules (`:332`, `:2310`, `:2473`), `append_reports_exceeded_store_limits` (`:4282`) and `store_accepts_the_guaranteed_minimum_payload` (`:3775`) | When deciding each capability answer — read the rule the answer switches on before answering | AC-002, AC-004, AC-005, AC-007, AC-008 |
| `crates/happenstance-core/src/limits.rs` | `MIN_SUPPORTED_EVENT_DATA_LEN` (`:22`), `MIN_SUPPORTED_TAGS_PER_EVENT` (`:27`) and `MIN_SUPPORTED_EVENTS_PER_BATCH` (`:43`) — the floors a stated ceiling is compared against, and may legitimately sit below | Immediately after the ceilings are measured, before they are written down | AC-008 |
| `xtask/src/proof.rs` | The in-tree answer to "`cargo test` exits 0 on `running 0 tests`" (`:15-21`): assert the expected names out of `--list` **before** running them. The shape to copy into the job — not a row to add to its list | When writing the `neon` job's first step | AC-012 |
| `.github/workflows/ci.yml` | The job topology this one joins — `gate` (`:31`), `backlog` (`:102`), `wasm-conformance` (`:204`), `msrv` (`:241`), `semver` (`:279`), `advisories` (`:325`) — and, at `:102-131`, the only in-tree precedent for a job needing a credential `gate` does not, with its explicit "Absent, this job is RED rather than skipped". The `concurrency` reasoning at `:20-26` is the model for EC-009 | Before writing the `neon` job, and again before choosing its trigger | AC-011, AC-012, EC-009 |
| `xtask/src/main.rs` | The gate as it is actually defined: `REQUIRED` (`:105`), clippy (`:116`), tests (`:143`), the `wasm32` build of `happenstance-neon` (`:271`), the feature-powerset step (`:564`), and `wasm_steps()`'s by-name selection that panics on a miss (`:784-791`) | Before touching any dev-dependency or feature, and to confirm what the default gate will and will not run | AC-010, NF-004 |
| `deny.toml` | The licence allowlist (`:8-20`) with `[graph] all-features = true`. A new dev-dependency that fails it is a finding to report, not an allowlist to grow | Before pinning any new dev-dependency | NF-005 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | *Architecture brief* §6 (the two fixtures, and the "either answer is admissible, the reason must be real" bar) and §7 (whole-invocation gating); *Testing brief* Notes §6 (the merge-gate commands and the `required-features` trap) and §7 (`NeonFixture`'s stated shape); *Deployment brief* Notes, "CI implication, concretely" | Before answering `REOPEN`/`MID_BATCH_FAULT`, and before writing the job | AC-005, AC-006, AC-010, AC-011 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` | DR-4 (the endpoint is not substitutable), DR-5 (the decline path is the only sanctioned non-pass), DR-6 (consume CF-40, do not settle it), DR-8 (the `!Send` story), DR-9 (a flaky live job is fixed or reported), and the risk table's row 3 | At the start, and whenever a shortcut starts to look reasonable | AC-001, AC-005, AC-006, AC-011 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-fixture-and-live-job/discover.md` | The signal ledger, the four questions deferred to this spec (Q5–Q8), and the three named wrong implementations — including the arithmetic mutant and the pooled-Postgres substitution, written out in full | Before implementing, and again at self-review as a mutant checklist | AC-001, AC-007 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | Two accepted decisions claim and disclaim ownership of the numeric-limit declaration. DR-6 says consume, not settle — this atom is what "record the unresolved ownership" points at | When writing the ceilings' rustdoc, so the note cites the open question rather than inventing a policy | AC-007, AC-008 |
| `spec/SPECIFICATION.md` | CF-16 (`:7548`, the second-handle MUST), CF-18 (`:7597`), CF-40 (`:7661`), CF-33 (`:8236`, no rule reads a clock), and the `[PROVISIONAL]` markers on ES-11/ES-12 whose stated reason is the empty transport axis (`:371`, `:3003`) | When a retry or wait is tempted, and when writing the story's report | AC-002, AC-004, NF-003 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The MSRV is 1.97.1 and moves only by a recorded decision — a dev-dependency needing more is a finding, not a bump | Only if a new dev-dependency refuses to build at 1.97.1 | NF-006 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | The in-tree layout precedent for a test target with a companion module directory, and the home of `capability_skips_are_reported` (`:3184`) — the meta-test that fails the build when a decline goes unreported | When laying out `tests/`, and when reasoning about what already enforces the reporting half of AC-007 | AC-010, AC-012 |

## Clarifications resolved during spec

**The AC set is exactly the twelve the front half enumerated** — AC-001 through
AC-012, none added and none dropped. The ledger carries the same twelve.

**Q5 (deferred from `discover.md`) — `REOPEN`.** Resolved: **`Capability::SUPPORTED`**.
Neon's medium is a real Postgres branch and the handle carries only a client and
a config, so a `reopen()` that discards the client and builds a fresh one against
the same endpoint and namespace genuinely re-reads the medium — the exact thing
`MemoryFixture` cannot do, and the reason it declines
(`crates/happenstance-testkit/src/fixtures.rs:275-284`). Declining here would be a
reason of convenience, which `project.md` AC-007 exists to forbid. If live
evidence contradicts this — a Neon behaviour that makes a fresh client observably
not a reopen — the reversal is admissible, and AC-005's test is written to accept
either verdict provided the reason is Neon-specific and non-empty. The reversal
must be recorded in the ledger evidence, not merely in code.

**Q5 — `MID_BATCH_FAULT`.** Resolved: **declined, with Neon's own reason**. An
append is one non-interactive statement over one round trip; there is no point
between two rows a caller can reach, because there is no interactive transaction
and no cursor. The alternative was considered and rejected explicitly: the fixture
*could* send DDL over the same transport to arm a trigger that raises on row *n*,
which is the shape the Postgres sibling will use — but the fault that trigger
injects is Postgres's, not the transport's, and the axis `happenstance-neon`
exists to occupy is the transport one. That rejection is written into the
fixture's rustdoc so the reason is arguable rather than merely stated. What is
**not** admissible is inheriting the trait default (`contract.rs:207-211`), whose
text is about an in-memory store having no fault to inject and would be a fiction
here; AC-006 asserts the reason differs from it.

**Q6 — the ceilings' values.** Resolved as a **derivation plus a measurement
obligation**, because the number itself needs a live branch. The anchor is
`MAX_RESPONSE_BYTES` = 64 MiB, hard, with no cursor behind it; the adjustment is
the hex `bytea` doubling in both directions; the candidate is therefore below
`MAX_RESPONSE_BYTES / 2` less the JSON envelope, and the **stated** number is the
one a live bisection against the endpoint produces (AC-007). The comparison
against VT-21's 65,536, VT-22's 64 and VT-24's 128 is a separate, independent
answer (AC-008): if the derived ceiling sits below a floor, that is a finding
recorded with its number, not a number nudged upward. CF-40's contested ownership
is consumed, not settled (DR-6).

**Q7 — the job where the secret is unavailable.** Resolved as a **stated design**,
following the `backlog` job's precedent verbatim (`.github/workflows/ci.yml:102-131`,
"Absent, this job is RED rather than skipped"): where the secret should be present
— a push to `main`, a pull request from a branch in this repository,
`workflow_dispatch`, the weekly schedule — a missing or rejected credential makes
the job **red**, naming the variable. On a **fork** pull request the job is not
scheduled at all (`if: github.event.pull_request.head.repo.full_name ==
github.repository`), because a job that does not run is not a green tick it did
not earn, and a fork contribution is merged through a branch in this repository
where the job does run. No `continue-on-error`, and no path to non-blocking
without a recorded decision (DR-9). AC-011 asserts the absence of
`continue-on-error` from the **default** gate, where it costs nothing.

**Q8 — branch lifecycle and namespacing.** Resolved: **one long-lived CI branch,
one namespace per fixture instance**. The branch arm wins because it needs only
the `/sql` credential; branch-per-run needs Neon's control-plane API, which is a
second credential and a second HTTP surface outside `SqlTransport`'s remit, and
branch-per-fixture-instance is a cost claim nobody has measured against a suite
that constructs many instances (NF-002). The namespacing arm is a **per-instance
schema with fully qualified table names** — `CREATE SCHEMA hs_test_<unique>` plus
`with_event_table("hs_test_<unique>.event")` — rather than a per-instance table
name, because one `DROP SCHEMA … CASCADE` removes everything the instance created
including any checkpoint table, and `NeonConfig` already carries a qualified name
(`crates/happenstance-neon/src/config.rs:38-43`). Teardown drops the instance's
schema; a prefix sweep at job start removes the leavings of a killed run; and the
job declares its own `concurrency` group so a sweep can never delete a live run's
schema (EC-009). Uniqueness is generated at fixture construction from a counter
and a run identifier — not a clock — which keeps CF-33 uncontroversial (NF-003).

**Q9 and Q10 stay where `discover.md` put them.** The ES-10 visibility mechanism
belongs to `adr-0024-position-visibility-mechanism`, and the `conflicting_position`
verdict to `neon-conflicting-position-verdict`. This story adds no visibility
column and invents no capability that would settle the second by omission.

**No `[FROZEN]` clause is touched, so no ADR is owed by this story**
(`discover.md`, *Decision*). Should the measured ceilings force an amendment to a
frozen clause, that is DR-7's second decision record and a different story's work,
not an edit made here.

**One thing this spec deliberately leaves open.** The exact environment-variable
name and the exact test-target layout are the implementer's, subject only to "one
environment read, in one place" and "exactly one `event_store_conformance!`
invocation". The paths named in the acceptance table are the ones the ledger will
cite; a different but equivalent layout is fine provided the ledger's
`verifying_test` values are updated to the paths that actually exist.
