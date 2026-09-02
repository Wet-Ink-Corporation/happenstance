---
item: HS-S0067
stage: spec
created: 2026-08-12T13:47:06.010Z
updated: 2026-08-12T13:47:06.010Z
template_sig: 87bbf1d0
rendered_sig: b46b45bd
---

# Spec — A real SqlTransport, host and wasm32

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 6; BR-02, BR-12, BR-13 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG and the traceability matrix |
| Project | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/project.md` — AC-006, AC-011, DR-4, DR-8, DR-9 |
| This spec | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-sql-transport/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` — Architecture §9.2 (the three ordered questions), §7 (gate and CI wiring), Testing brief Notes §5 and §6, Deployment brief Notes (feature flags, `cargo deny`, wasm32) |
| Grounding | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_grounding.md` — tension 5, "the Neon HTTP transport does not exist yet" |
| Story map / slice | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_storymap.md` — *Slices*, `neon-transport`; *Merge order* item 2 |
| Signed-off design | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` — **N/A by sign-off**: this project records no user-facing surface. Nothing here renders a surface id. |
| Prior stage | `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-sql-transport/discover.md` — the signal ledger, the four answered questions, the three wrong implementations |
| Roadmap | `RUNBOOK.md:4311-4390` (phase 10 in full); `RUNBOOK.md:685-702` (the instrument portfolio and the empty transport far end) |

## One-line PR slice

A real `SqlTransport` — host client and `wasm32` client, correctly `cfg`-scoped —
proven by one live `/sql` round trip, with ship-vs-dev-dependency, `cargo deny`
admissibility and the wasm32 build all answered **before any fixture depends on it**.

## Executive summary

`crates/happenstance-neon/src/transport.rs:245-265` defines `SqlTransport` as a
one-method trait, and `NullTransport` (`:267-298`) is its only in-tree
implementation: it fails every round trip by design. The crate that exists to
occupy the far end of the transport axis has therefore never spoken to anything.

**This PR lands the far end of a wire.** It adds two real `SqlTransport`
implementations to `happenstance-neon` — one for the host target over an HTTPS
client, one for `wasm32-unknown-unknown` over the host environment's `fetch` — and
proves the host one against a real Neon `/sql` endpoint with a single small round
trip. `NullTransport` stays exactly as it is; nothing about the trait's shape
changes.

The delta over the briefs is that Architecture §9.2's three questions stop being
questions. This PR **decides and records**, in the crate's own rustdoc and in the
story ledger: (1) whether the transport ships in `[dependencies]`, behind an
off-by-default feature, or lives in `[dev-dependencies]`; (2) whether `cargo deny
check` admits the chosen client against `deny.toml:8-19` without growing the
allowlist; (3) how the host client is `cfg`-scoped so that `cargo xtask wasm`'s
step named "wasm32 build of the Neon adapter" stays green *and* the optional
wasm32 feature-powerset step has nothing to catch.

What this PR deliberately does **not** land: no fixture, no `event_store_conformance!`
invocation, no CI job, and none of the three remaining `todo!()`s in
`crates/happenstance-neon/src/event_store.rs`. Those are `neon-fixture-and-live-job`
(HS-S0068) and `neon-append-and-read-over-http` (HS-S0069). The ordering is the
deliverable as much as the code — Testing brief Notes §5: the transport is proven
by a much smaller test *before* the fixture is wired to the macro, "so a transport
bug and a conformance failure are never debugged as the same failure".

## Context pack

Read this section and you can start. Everything below it is either a boundary or a
signposted anchor.

### The decision this story exists to make, and the two it must not unmake

**Decision 1 — where the client lives in the manifest, and what that publishes.**
`crates/happenstance-neon/Cargo.toml` has **no `[dev-dependencies]` table at all**
today, and its only features are `event-store` / `projection-store`, both default-on
and both empty. So every option is a first manifest decision, not an adjustment to
an existing one. The trade is stated in Architecture §9.2 point 1: a client in
`[dependencies]` makes `happenstance-neon` batteries-included and changes what
AC-012's `publish = false` removal actually publishes (a published crate that can
make live HTTP calls at runtime is a materially different deployment posture from
one that cannot — Deployment brief Notes, *Feature flags / config gating*). A client
in `[dev-dependencies]`, or behind an off-by-default feature, keeps the published
dependency surface exactly as it is today. **Decide it, take the consequence
deliberately, and write the reason where the next reader meets it** — the crate's
`transport` module rustdoc, which already carries the "why a trait and not a client"
argument this decision answers (`crates/happenstance-neon/src/transport.rs:1-15`).
Whichever way it goes, `neon-fixture-and-live-job` must be able to reach the chosen
implementation from `crates/happenstance-neon/tests/`; a transport nothing
downstream can name has not been delivered.

**Decision 2 — the licence answer is measured, not assumed, and the offender is not
the obvious one.** The workspace manifest already carries the measurement, in full,
with the rejection text pasted in: `sqlx`'s `tls-rustls` fails `cargo deny check
licenses` on **`webpki-roots`** (`CDLA-Permissive-2.0`) — *not* on `ring`, which is
`Apache-2.0 AND ISC` and passes (`Cargo.toml:56-72`; the same note restated at
`crates/happenstance-postgres/src/lib.rs:40-47`). The allowlist is MIT /
Apache-2.0 / Apache-2.0-WITH-LLVM-exception / BSD-2 / BSD-3 / ISC / Unicode-3.0 /
Zlib with `[graph] all-features = true` (`deny.toml:1-19`). **Run `cargo deny check`
against the candidate before committing to it, not after.** If the chosen client
needs the allowlist to grow, that is a gate weakening: it is not in this PR's
boundary, it is escalated with the rejection text attached, and the client is
changed or the platform trust store is used instead. `standards/rust/50-dependency-hygiene.md`
RS-50-4 is exactly this rule, and RS-50-1 is why a new version pin belongs in
`[workspace.dependencies]` rather than in the member manifest.

**Decision 3 — `cfg` scoping is structural, not tidiness.** The crate documents why
it owns no client: a host client drags in a TLS stack, and `wasm32-unknown-unknown`
has none — the only way out of the sandbox is the host's `fetch`
(`crates/happenstance-neon/src/transport.rs:5-11`, `src/lib.rs:91-99`). One
implementation cannot serve both targets, so there are two, each behind a target
`cfg`. The failure mode to design against is named in
`standards/rust/52-wasm32-and-target-cfg.md` **RS-52-2: a feature is not
target-scoped.** A host-only client behind a plain feature compiles on wasm32 with
the feature *off*, so the mandatory gate step passes and only the **optional**,
`cargo hack`-gated wasm32 feature-powerset step (`xtask/src/main.rs:558-593`)
notices — and where `cargo hack` does not resolve, that step prints `skipped` and
the crate ships claiming a target it cannot build for.

**The two things this story must not unmake:**

- **No `Send` bound, anywhere, by any route.** `round_trip` is spelled
  `-> impl Future<Output = …>` rather than `async fn` precisely so none is implied,
  "a `wasm32` implementation over `fetch` cannot supply one"
  (`crates/happenstance-neon/src/transport.rs:253-256`, module preamble `:37-43`).
  This is ADR-0001 (`.kb/decisions/0001-async-port-flavours.md`) and the project's
  DR-8. A `Send` bound reintroduced on a helper, a `Box<dyn Future + Send>` in an
  internal seam, or a host client whose future is only usable behind
  `tokio::spawn`, all defeat the reason the crate is written on the bare flavour.
  `standards/rust/21-send-is-not-inherited.md` and `22-rpitit-and-lifetime-capture.md`
  are the two atoms to pull before writing either signature.
- **The bare-flavour claim.** `NeonEventStore` implements bare `EventStore` on both
  targets and satisfies `SendEventStore` on neither, *even where the transport
  happens to be `Send`* — adding a second impl is `error[E0119]` against
  `trait_variant`'s blanket impl, compiled rather than reasoned about
  (`crates/happenstance-neon/src/lib.rs:79-89`,
  `crates/happenstance-neon/src/event_store.rs:3-34`). A host transport that *is*
  `Send` does not license widening that claim, and nothing in this PR may try.

### The one sentence an idiomatic HTTP wrapper violates by reflex

> "A non-2xx status is **not** one of these — it is an answer, and it is where Neon
> puts SQL errors, so it must reach the decoder intact."
> (`crates/happenstance-neon/src/transport.rs:249-250`)

`Self::Error` is for *no HTTP answer existing*: DNS, TLS, a rejected `fetch`, a
timeout (`:246-251`, mirrored by `NeonError::Transport`'s doc at
`crates/happenstance-neon/src/error.rs:66-72`). `error_for_status()` — the first
line most Rust authors write around an HTTP client — compiles, passes a `SELECT 1`
happy-path test, and destroys the adapter: every SQLSTATE Neon returns as 4xx
becomes a transport failure before `decode_append_response` sees a row,
`AppendError::ConditionViolated` becomes unreachable
(`crates/happenstance-neon/src/event_store.rs:194-197`), and AC-008's whole question
is answered "no" by an accident in a layer that was not supposed to have an opinion.
Nothing mechanical enforces this, because `Self::Error` is the implementor's own
type and any mapping into it type-checks. **The round-trip test is the enforcement**:
it sends a statement that Neon will reject and asserts the round trip returns
`Ok(HttpResponse)` with the 4xx status and a body that deserialises as
`NeonSqlError` with a `code` (`crates/happenstance-neon/src/error.rs:16-55`).

### The quieter sibling, and the tension this story surfaces but does not settle

The isolation level travels as the `Neon-Batch-Isolation-Level` **header**, not in
the JSON body, and applies only when `SqlRequest::statements` holds more than one
element — "a single statement is its own implicit transaction and the header is
ignored" (`crates/happenstance-neon/src/transport.rs:56-60`). `SqlRequest::headers()`
already encodes exactly that, returning an empty vec below two statements
(`:190-209`). A transport that folds isolation into the body compiles, behaves
identically for every single-statement operation, and silently runs a *batch* at the
endpoint's default — a lost update under contention, with no error and no failing
test. **The transport sends what `SqlRequest::body()` and `SqlRequest::headers()`
return, and adds nothing of its own beyond `Content-Type` and credentials.**

That collides with something the crate also says: the single-statement CTE that
keeps `ConditionViolated::conflicting_position` "needs `IsolationLevel::Serializable`
to be sound, which is why `NeonConfig`'s default is `Serializable`"
(`crates/happenstance-neon/src/lib.rs:74-77`, `src/config.rs` `Default`) — while a
one-statement request is precisely the case where the header is ignored. **This
story records the observation and routes it; it does not resolve it.** The verdict
is `neon-conflicting-position-verdict` (HS-S0070), with `neon-append-and-read-over-http`
(HS-S0069) as the consumer. Writing an answer here would settle AC-008 in a story
that does not own it.

### The persona slice

The "user" of this project is an **adapter author**, and the only medium this
repository has for user-observable is a public API a caller meets and a test that
runs and reports (`_storymap.md`, preamble). This story's slice of that journey:
an adapter author who wants `happenstance-neon` to talk to their Neon branch can
construct a transport from an endpoint URL and a credential, hand it to
`NeonEventStore::new`, and get a real answer back — including the *rejections*,
structurally, rather than an opaque "the round trip did not complete". Everything
downstream of that (a fixture, a suite run, a verdict) is another story's.

### Where this sits in the plan

`neon-transport` has **no inbound `depends_on`** and can start on day one, in
parallel with the whole `postgres-live-suite` slice — "the transport work does
**not** wait on the Postgres schema… which is the one place this plan buys real
parallelism" (`_storymap.md`, *Merge order* item 2; Architecture §8 point 3). It
blocks HS-S0068, which blocks HS-S0069, which blocks HS-S0070. Slipping it slips the
entire Neon half of the project.

## Integration contract

| Field | Value |
| ----- | ----- |
| **Archetype** | `foundation` — real in-tree substrate consumed by the `neon-live-suite` capability slice in this same project. Not a double, not a fixme. |
| **Slice / milestone** | `neon-transport`. **Slice-mates: none** — this milestone holds exactly one story (`_storymap.md`, *Slices*). It is mounted alone and merged alone. |
| **Mount point** | `crates/happenstance-neon/src/lib.rs` — the crate root is this crate's composition root: it declares `pub mod transport` (`:109`) and re-exports the transport's public items by name (`:120-123`). A transport module that compiles but is not declared and re-exported here is constructed-but-unmounted, and `neon-fixture-and-live-job` cannot name it. Any target `cfg` on the new implementations is written **here and in `transport.rs`**, matching RS-52-3 ("a `cfg` covers the probe *and* its caller"). |
| **Wires into** | `crates/happenstance-neon/src/transport.rs` — `SqlTransport`, `SqlRequest`, `SqlStatement`, `HttpResponse`, `IsolationLevel`, `MAX_RESPONSE_BYTES` (the trait is implemented, never edited). · `crates/happenstance-neon/src/error.rs` — `NeonError::Transport` is the variant a transport `Error` lands in; `NeonSqlError` is what a 4xx body must still deserialise into. · `crates/happenstance-neon/src/wire.rs` — `ResponseBody` / `ResultSet`, which the round-trip test decodes through to prove the answer arrived intact. · `crates/happenstance-neon/src/config.rs` — `NeonConfig` carries no endpoint or credential, so the transport type owns both. · `Cargo.toml` `[workspace.dependencies]` — where any new third-party pin is written (RS-50-1). · `xtask/src/main.rs:264-282` — the gate step named "wasm32 build of the Neon adapter", selected **by name** (`:783-791`), so renaming it is a gate change. |
| **Renders surfaces** | **none.** `_design.md` records no user-facing surface for this project and that determination is signed off (`_design.md:10-20`, `:86-95`); `design.capture` is deliberately absent from `.redkiln/config.yaml`, making the perceptual review a declared skip. There is no surface id to claim. |
| **Public items** | `_design.md`'s `## Items` block is `N/A — no user-facing surface`, so there is no signed-off item list to implement. The public items this story *adds* are therefore named here and are subject to the same bar `standards/rust/40-public-surface-and-evolution.md` sets: the host transport type, the wasm32 transport type, and each one's `Error` type — every one `#[non_exhaustive]` where it is a struct or enum a caller might match, every fallible constructor and `round_trip` carrying an `# Errors` section naming conditions rather than types (`standards/rust/70-rustdoc-obligations.md`). |
| **Conformance rule(s)** | **None, and deliberately.** This story adds no rule to `crates/happenstance-testkit/src/suite.rs` and defines no `EventStore`, so it is not adapter-observable through the suite (Testing brief Notes §5 and §8: the transport "is **not** itself proven by the conformance tier"). The instruments that reject the three named wrong implementations are, respectively: the live round-trip test asserting a rejected statement's body reaches the decoder; a unit assertion on the request the transport actually builds; and the reported output of the wasm32 build plus the feature-powerset step. Nothing is owed to `crates/happenstance-testkit/tests/`. |
| **Clause(s)** | None read, none amended, none `[FROZEN]` touched — `discover.md`, *Decision*, final sentence. **No ADR is owed by this story**, and none may be written as a side effect: the project's decision record is ADR-0024, owned by `adr-0024-position-visibility-mechanism`, and the DoD-6 amendment (if the endpoint ever forces one) is HS-S0070's. The three §9.2 decisions are recorded in crate rustdoc and the story ledger, which is what "recorded, not merely made" means at this grain (`_storymap.md`, *Grain notes*, second bullet). |
| **Advances DoD scenario** | Initiative **DoD 6** — "a store with no connection, no interactive transaction and no cursor passes the suite, or the contract is amended by decision record and the suite re-run" (`initiative.md:375-376`). This story does not move it to green; it removes the one blocker that makes it unreachable, and it is the only story in the project that can. Project **AC-006** (the real endpoint reached — the suite run against it is HS-S0069's) and project **AC-011** (the wasm32 third: whatever the transport is, it must not cost the crate its second target). |

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any
file changed outside it.

```
crates/happenstance-neon/src/**
crates/happenstance-neon/tests/**
crates/happenstance-neon/Cargo.toml
Cargo.toml
Cargo.lock
.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-sql-transport/**
```

**In this PR**

- Two real `SqlTransport` implementations in `crates/happenstance-neon/src/transport.rs`
  (or a `transport/` submodule split from it), each behind a target `cfg`, each with
  its own `Error` type, declared and re-exported at the mount point.
- The manifest decision executed: the client's dependency table and any feature,
  target-scoped, with the version pin written in `[workspace.dependencies]`
  (`Cargo.toml`) and `Cargo.lock` updated.
- One live `/sql` round-trip test in `crates/happenstance-neon/tests/` (the crate has
  no `tests/` directory today), gated **whole-invocation** on the credential's
  presence — never a `#[cfg]` hiding one assertion.
- Offline unit assertions on the request the transport builds (body shape, headers,
  ceiling refusal) that run in the default gate with no network.
- The three §9.2 answers written into `transport.rs`'s module rustdoc, replacing the
  "the crate owns no socket" framing at `:13-15` and `src/lib.rs:91-99` with what is
  now true — and the recorded `cargo deny check` and wasm32 evidence in the story
  ledger.

**Explicitly not in this PR**

- `NullTransport` (`transport.rs:267-298`) — kept, unchanged. It is what makes the
  crate's doctests compile without a client (`event_store.rs:76-80`).
- Any edit to the `SqlTransport` trait, `SqlRequest`, `SqlStatement`, `HttpResponse`
  or `IsolationLevel`. This story implements the seam; it does not renegotiate it.
- `NeonFixture`, any `event_store_conformance!` invocation, and the credentialed Neon
  CI job — `neon-fixture-and-live-job` (HS-S0068). No change to
  `.github/workflows/ci.yml` and no change to `xtask/src/main.rs`'s `REQUIRED` array.
- The three remaining `todo!()`s (`conditional_append_request`,
  `decode_append_response`, `decode_read_response`) — HS-S0069.
- The `conflicting_position` / isolation-header verdict — HS-S0070. Observed and
  routed here, not answered.
- `publish = false`, `#![allow(clippy::todo)]`, `PUBLISHABLE` — `deskeleton-and-package-readiness`.
- **`deny.toml`.** It is outside the boundary on purpose. If the chosen client cannot
  clear the existing allowlist, that is a gate weakening: stop, attach the rejection
  text, and either change the client or escalate for a recorded decision — do not
  widen the list to get green.

**Merge DoD.** `cargo xtask affected --base main` and `cargo xtask ci --fast` are
green on a machine with **no network and no Neon credential**; `cargo xtask wasm`'s
"wasm32 build of the Neon adapter" step is green; `cargo deny check` is green with
`deny.toml` unmodified; and the live round trip has been run once against a real
Neon `/sql` endpoint with its output recorded in the ledger.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **A host transport reaches a real `/sql` endpoint** | A type owning the endpoint URL, the credential and the HTTP client implements `SqlTransport` for the host target. `NeonConfig` carries neither URL nor credential (`config.rs`, all four fields are table names, isolation and a byte ceiling), so the transport type owns both — and its constructor is where an invalid URL or a missing credential is refused, not `round_trip`. | `crates/happenstance-neon/src/transport.rs:245-265`; `crates/happenstance-neon/src/config.rs` |
| **A non-2xx answer is an answer** | `round_trip` returns `Ok(HttpResponse { status, body })` for **every** response the endpoint produced, whatever the status. `Self::Error` is returned only when no HTTP response was obtained. No `error_for_status()`, no status-based branch that maps into `Self::Error`. | `crates/happenstance-neon/src/transport.rs:246-251`, `:258-260`; `crates/happenstance-neon/src/error.rs:16-36`, `:66-72` |
| **The request is sent as `SqlRequest` describes it** | Body from `SqlRequest::body()` — bare `{"query", "params"}` for one statement, `{"queries": […]}` for many. Headers from `SqlRequest::headers()` — empty below two statements, otherwise `Neon-Batch-Isolation-Level` and `Neon-Batch-Read-Only`. The transport adds `Content-Type` and credentials and nothing else; isolation never enters the JSON body. | `crates/happenstance-neon/src/transport.rs:172-209`, `:56-60` |
| **The response ceiling is observed before the body is buffered whole** | "A body over `MAX_RESPONSE_BYTES` never becomes one of these" — so the transport refuses rather than allocating 64 MiB and then complaining. The refusal is a *distinct* variant of the transport's own `Error`, not folded in with DNS/TLS, because a caller acts differently on it. Hex `bytea` rendering roughly doubles a payload against the ceiling in both directions, so the effective ceiling is lower than the constant; state that, do not encode a second number. `NeonConfig::max_response_bytes` is the store's clamp of the same constant and is applied one layer up — the transport enforces the hard endpoint ceiling. | `crates/happenstance-neon/src/transport.rs:49-54`, `:212-216`, `:87-94`; `crates/happenstance-neon/src/config.rs` |
| **A `wasm32` transport over `fetch`** | A second implementation, `cfg`-scoped to `target_arch = "wasm32"`, reaching the network through the host environment's `fetch`. Its future is not `Send` and is not required to be. It compiles under `cargo check --target wasm32-unknown-unknown -p happenstance-neon`; it is **not** executed by this story (running the suite under a real edge runtime is `cloudflare-durable-object-store`'s, per `project.md` *Out of scope*). | `crates/happenstance-neon/src/lib.rs:91-99`; `crates/happenstance-neon/src/event_store.rs:3-12`; `xtask/src/main.rs:264-282` |
| **No `Send` bound is reintroduced by any route** | Neither implementation, nor any helper either one calls, attaches `Send` to a future, a boxed trait object or a generic parameter. `NeonEventStore<HostTransport>` remains bare-`EventStore`-only; no `SendEventStore` impl is added (it would be `error[E0119]` against `trait_variant`'s blanket impl anyway). | `.kb/decisions/0001-async-port-flavours.md`; `crates/happenstance-neon/src/lib.rs:79-89`; `crates/happenstance-neon/src/event_store.rs:14-34`; `standards/rust/21-send-is-not-inherited.md`, `standards/rust/22-rpitit-and-lifetime-capture.md` |
| **The manifest decision is executed and recorded** | One of: `[dependencies]`, an off-by-default feature, or `[dev-dependencies]` — chosen, with the published-surface consequence stated. If it is a feature, the feature is **target-scoped**, so that the wasm32 feature powerset has nothing to catch (RS-52-2). Whatever is chosen must be reachable from `crates/happenstance-neon/tests/` by HS-S0068. | `crates/happenstance-neon/Cargo.toml` (no `[dev-dependencies]` today; `default = ["event-store", "projection-store"]`); `_decomposition.md` Architecture §9.2 point 1, Deployment brief Notes; `standards/rust/51-features-and-no-std.md` |
| **The licence answer is measured against the candidate** | `cargo deny check` run against the chosen client, output recorded. `deny.toml` unmodified. The known trap is `webpki-roots` (`CDLA-Permissive-2.0`), not `ring`; a client taking its roots from the platform trust store avoids the CA bundle entirely, and that is the shape to reach for first. | `deny.toml:1-19`; `Cargo.toml:56-72`; `crates/happenstance-postgres/src/lib.rs:40-47`; `standards/rust/50-dependency-hygiene.md` RS-50-4 |
| **The wasm32 gate step stays green, under its existing name** | `cargo xtask wasm` selects steps by name and panics on a miss, "precisely so that inserting a step cannot silently repoint it". Do not rename "wasm32 build of the Neon adapter". The optional feature-powerset step's result is **reported as evidence** in the ledger, not assumed — where `cargo hack` does not resolve it prints `skipped`, and a non-target-scoped feature would ship. | `xtask/src/main.rs:264-282`, `:558-593`, `:783-791`; `standards/rust/52-wasm32-and-target-cfg.md` RS-52-2, RS-52-3 |
| **The default gate stays network-free and credential-free** | The live round-trip test is gated whole-invocation (`#[ignore]`, a `required-features` flag, or an env read inside the test binary — the implementer's call). `cargo test --workspace --all-features` exits zero with no credential present. The gating may **never** be a `#[cfg]` removing an assertion from an expansion; that distinction is DR-5's. | `_decomposition.md` Architecture §7, Testing brief Notes §6; `project.md` DR-5, DR-9, AC-011 |
| **The isolation-header observation is surfaced and routed** | The live round trip records what a one-statement request does with the isolation header, and the record names HS-S0070 as its owner. This story states the observation; it does not conclude from it. | `crates/happenstance-neon/src/transport.rs:56-60`; `crates/happenstance-neon/src/lib.rs:74-77`; `discover.md`, *Questions* item 5 |
| **The crate stops saying it owns no socket** | `transport.rs:1-15` and `lib.rs:91-99` currently document, honestly, that the crate owns no HTTP client and "cannot demonstrate that a licence-clean client exists for both targets". After this story it can, or it can say precisely which half it demonstrated. Leaving that prose unchanged makes the crate's most load-bearing documentation false. | `crates/happenstance-neon/src/transport.rs:1-15`; `crates/happenstance-neon/src/lib.rs:91-99`; `standards/rust/70-rustdoc-obligations.md` |

## Data and migrations

**N/A — no schema, no migration, no persisted data.**

This story adds a wire, not a table. `happenstance-neon` owns no schema of its own:
it inherits migration 1 from `happenstance-postgres` — the identity and time columns
(`RUNBOOK.md:249-253`), the tag storage and the append-condition SQL — which is the
stated reason Postgres and Neon are one project and not two (`project.md`, *Coupling
notes*). The intended `CREATE TABLE event` is documented at
`crates/happenstance-neon/src/event_store.rs:36-53` and is authored by
`postgres-schema-and-live-fixture`, not here.

Three adjacent things that are *not* migrations and should not be mistaken for them:

- **The round-trip test's target.** The live proof needs no `event` table. A
  statement the endpoint can answer with no schema at all (`SELECT 1`) plus a
  statement it will *reject* is sufficient and is deliberately preferable: it keeps
  this story's test independent of a schema owned by a story it does not depend on.
- **Credentials are configuration, not data.** The endpoint URL and credential are
  constructor arguments to the transport type, read from the environment by the
  gated test. Nothing is committed, and no credential enters `NeonConfig` (which has
  no field for one).
- **`Cargo.lock`** changes when the client is added. That is a lockfile update inside
  the PR boundary, not a data migration.

## Acceptance criteria

The "user" is the **adapter author** — the persona whose journey is *learn when you
are finished*, "from a signature that type-checks to a suite that says pass or fail
and names why" — and, for the wasm32 half, the **local-first / edge developer**
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`;
`initiative.md:210-219`, `:245-249`). The medium in which this repository makes a
thing user-observable is a public API a caller meets and a test that runs and
reports (`_storymap.md`, preamble), so each criterion below crosses the whole stack
that exists here: a caller's construction, a real wire, and an assertion.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **The adapter author's transport reaches their own Neon branch.** GIVEN an adapter author holding a Neon `/sql` endpoint URL and a credential, WHEN they construct the host transport from those two values and call `round_trip` with a one-statement `SqlRequest` the endpoint can answer with no schema at all (`SELECT 1`), THEN they get `Ok(HttpResponse)` with a 2xx status and a body that deserialises through `wire::ResponseBody` into a `ResultSet` holding the row — the first time anything in this workspace has spoken to a real endpoint. | `crates/happenstance-neon/tests/live_sql_round_trip.rs::select_one_decodes_through_result_set`, gated whole-invocation on the credential, run once against a live endpoint with its output pasted into `_ledger.md`. |
| **AC-002** | **A rejected statement reaches the author as a rejection, not as "the network failed".** GIVEN the same transport, WHEN `round_trip` sends a statement the endpoint refuses (a select against a table that does not exist), THEN it returns `Ok(HttpResponse)` carrying the non-2xx status and a body that deserialises as `NeonSqlError` with a `code`, and `Self::Error` is **not** constructed — so `decode_append_response` can still tell `AppendError::ConditionViolated` from `AppendError::Store`. | `crates/happenstance-neon/tests/live_sql_round_trip.rs::rejected_statement_returns_ok_with_the_sql_error_body`; plus review that no status-classifying branch (`error_for_status` or a hand-rolled equivalent) exists anywhere in the transport. |
| **AC-003** | **The isolation the author configured is the isolation the endpoint runs.** GIVEN a `SqlRequest` of two statements at `IsolationLevel::Serializable`, WHEN the transport turns it into an HTTP request, THEN the bytes sent are exactly `SqlRequest::body()`, the headers sent are exactly `SqlRequest::headers()` plus `Content-Type` and credentials, no isolation key appears anywhere in the JSON body; and GIVEN a one-statement request, THEN neither `Neon-Batch-*` header is sent. | Offline unit tests over the transport's request-building function in `crates/happenstance-neon/src/transport.rs` (or the `transport/` submodule it is split into): `request_parts_carry_the_batch_headers`, `single_statement_request_sends_no_batch_headers`, `isolation_never_enters_the_body`. No network. |
| **AC-004** | **An oversize answer is refused as an oversize answer.** GIVEN a response whose body exceeds `MAX_RESPONSE_BYTES`, WHEN the transport handles it, THEN it returns its own distinct over-ceiling error variant — one a caller can separate from DNS, TLS, a rejected `fetch` or a timeout — rather than folding it in with them, and it does not buffer the whole body before deciding. | Offline unit test over the ceiling helper in `crates/happenstance-neon/src/transport.rs`: `refuses_a_body_over_max_response_bytes`, fed a declared length and a chunk sequence, asserting both the variant and that no more than the ceiling was retained. |
| **AC-005** | **The edge developer's target survives the arrival of a host client.** GIVEN a developer targeting `wasm32-unknown-unknown`, WHEN `cargo xtask wasm` runs, THEN the step named **"wasm32 build of the Neon adapter"** is green under that exact name, and what compiles under it is the `fetch` implementation — the host client is absent from the wasm32 dependency graph entirely, not merely unused. | `cargo xtask wasm` (`xtask/src/main.rs:264-282`, selected by name at `:783-791`); plus `cargo tree --target wasm32-unknown-unknown -p happenstance-neon` recorded in `_ledger.md` showing the host client absent. |
| **AC-006** | **The `!Send` promise the crate is written on still holds.** GIVEN a caller on a single-threaded runtime, WHEN they drive `NeonEventStore` through the **bare** `EventStore` flavour inside a `tokio::task::LocalSet`, holding the future across an await, THEN it compiles and runs; and no `Send` bound appears on either implementation, on any helper either calls, or on any boxed future, and `NeonEventStore` gains no `SendEventStore` impl. | `crates/happenstance-neon/tests/not_send_is_preserved.rs::drives_on_a_local_set_through_a_generic_bound`, written against a generic `T: SqlTransport` so the obligation is discharged at the definition rather than at one concrete type, and instantiated with `NullTransport` so it runs offline. |
| **AC-007** | **What a consumer gets when they `cargo add happenstance-neon` is a decision somebody made.** GIVEN a consumer reading the manifest and the `transport` module rustdoc, WHEN they look for the HTTP client, THEN they find it in exactly one of `[dependencies]`, a **target-scoped** off-by-default feature, or `[dev-dependencies]`, with the published-surface consequence stated in prose; the version pin is in `[workspace.dependencies]`; the chosen shape is reachable from `crates/happenstance-neon/tests/`; and the wasm32 feature-powerset step's **actual** result — green or `skipped` — is recorded rather than assumed. | `crates/happenstance-neon/Cargo.toml` + `Cargo.toml` review; the live test file compiling under the chosen configuration is the reachability proof; `cargo xtask ci`'s "wasm32 feature powerset" step (`xtask/src/main.rs:558-593`) with its literal output in `_ledger.md`. |
| **AC-008** | **The licence gate is not weakened to make this land.** GIVEN `deny.toml` byte-identical to `main`, WHEN `cargo deny check` runs over the tree with the chosen client in it, THEN it exits zero and the output is recorded; and if the candidate cannot clear the existing allowlist, the **client** changes, not the allowlist — the rejection text is attached and escalated instead. | `cargo deny check` (the gate's own step, `xtask/src/main.rs:596-600`), run explicitly because `--fast` drops it; `git diff --stat deny.toml` empty. |
| **AC-009** | **Every other project's gate stays runnable on a laptop with no network.** GIVEN a clean checkout with no Neon credential, no Docker and no network, WHEN `cargo xtask affected --base main` and the full `cargo xtask ci` run, THEN both exit zero, the live round trip does not execute, and the mechanism that skipped it is **whole-invocation** — no `#[cfg]` removed an assertion from a compiled test binary. | `cargo xtask affected --base main` and `cargo xtask ci` on an offline machine, both recorded; plus review that the gating is `#[ignore]`, `required-features`, or an env read *inside* the test body — DR-5's distinction. |
| **AC-010** | **The next reader is told the truth about what this crate now owns.** GIVEN a reader opening `happenstance-neon`'s docs after this PR, WHEN they read the crate-level and `transport` module rustdoc, THEN the "the crate owns no HTTP client… cannot demonstrate that a licence-clean client exists for both targets" prose has been replaced by what is now true; the three §9.2 answers and their reasons are stated where the decision is met; every new public item carries the crate's rustdoc bar (an `# Errors` section naming conditions rather than types, `#[non_exhaustive]` where a caller might match); and the one-statement isolation-header observation is written down and explicitly routed to `neon-conflicting-position-verdict`, not concluded here. | `cargo doc --workspace --all-features --no-deps` clean under `-D warnings` (`missing_docs` and `clippy::missing_errors_doc` are `warn` workspace-wide at `Cargo.toml:101-117`, and the gate denies warnings) — that half is mechanical; the prose half is a review read of `crates/happenstance-neon/src/lib.rs:91-99` and `src/transport.rs:1-15` against the diff, recorded in `_ledger.md`. |

**Coverage of the traced project ACs.** Project **AC-006** ("Neon runs the suite over
one-shot HTTP") — this story owns *the real endpoint reached*, which is AC-001,
AC-002, AC-003, AC-004 and AC-010 above; the suite run against it is
`neon-append-and-read-over-http`'s (`_storymap.md`, *Coverage*, AC-006 row). Project
**AC-011** ("the default gate stays Docker-free and network-free… the wasm32 build of
`happenstance-neon` is still part of the default gate") — this story owns *the wasm32
build that must not regress* plus the network-free default gate, which is AC-005,
AC-007, AC-008 and AC-009 above. AC-006 above additionally discharges DR-8, which
project AC-011 leans on but does not itself assert.

## Interaction quality

**Composition invariants: N/A by sign-off, and that is a determination, not an
omission.** `_design.md` records `## Surfaces` as "**N/A — no user-facing surface**"
and `## Items`, `## The doctest` and `## Sign-off` likewise, approved by the
repository owner on 2026-08-12 at the `/redkiln:plan` design gate
(`_design.md:10-20`, `:86-95`). `design.capture` is deliberately absent from
`.redkiln/config.yaml`, which makes the perceptual review a **declared skip** rather
than a silent pass (`CLAUDE.md`, *Where the work lives*). This story renders no
surface, claims no surface id, and there is no composition, density budget,
transience policy or hierarchy to honour or violate. Nothing below invents one.

**State invariants do apply, in this repository's medium.** `CLAUDE.md` records why
`_design.md` exists here at all: a library "has the same hole in another medium", so
the design stage asks for the public API surface — "every other check in this
repository is satisfied by an API that is correct and unusable". The RFC §6.7 state
family has exact analogues at a wire, and each one below is carried by an
acceptance-criteria row above, never by a bullet here.

| State invariant | Its analogue at this seam | Carried by |
| --- | --- | --- |
| **Non-occlusion** — the answer is not covered by the chrome around it | A non-2xx response is the endpoint's answer and must arrive intact; a transport that classifies status occludes the SQL error with its own error type, and the caller can never see past it | **AC-002** |
| **Preserved state** — nothing the user had is silently dropped | The body the endpoint sent is the body the decoder receives: no truncation, no re-encoding, no re-ordering, and headers that were absent stay absent | **AC-002**, **AC-003** |
| **In-place, not a context jump** — the caller stays where they were | `round_trip` returns the whole answer to the caller that asked; failure does not reroute the caller into a different error universe than the one `NeonError::Transport` documents | **AC-002**, **AC-004** |
| **Reversibility** — a wrong input is refused where it can still be corrected | An invalid endpoint URL or a missing credential is refused at **construction**, where the author still has both values in hand, rather than at the first `round_trip` deep inside a fixture | **AC-001**, **EC-005** |
| **Reachability** — the capability can actually be got at from where callers live | The chosen implementation is nameable and constructible from `crates/happenstance-neon/tests/` and re-exported at the mount point; a transport nothing downstream can name has not been delivered | **AC-007** |
| **Presentation exists at all** — the library analogue of "not bare markup" | Every new public item carries real rustdoc: an `# Errors` section naming *conditions*, `#[non_exhaustive]` where a caller might match, and the module preamble corrected so it no longer describes a crate that owns no client | **AC-010** |
| **Density / weight budget** — what the surface costs the person who takes it | The published dependency surface is the number this story spends: the manifest decision states what a consumer now pulls in, and `cargo deny` bounds it at the licence axis | **AC-007**, **AC-008** |
| **Transience** — what is always present versus revealed on demand | Which of the two implementations is compiled is decided by **target**, not by taste; if a feature is used it is target-scoped, so the wasm32 powerset has nothing to catch | **AC-005**, **AC-007** |

**Named anti-patterns** (from `discover.md`, *The wrong implementation*): the
`error_for_status()` transport, rejected by **AC-002**; the transport that folds
isolation into the JSON body, rejected by **AC-003**; and the host-only client behind
a feature that is not target-scoped, rejected by **AC-005** and **AC-007** together —
AC-005 catches it only if the powerset step ran, which is why AC-007 requires that
step's literal output rather than its assumed result.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | No HTTP answer exists: DNS failure, TLS handshake failure, connection refused, a rejected `fetch`, a timeout. | The only case that becomes `Self::Error`. The type is `core::error::Error + 'static` (`transport.rs:251`) and no more — no `Send + Sync` bound, per `.kb/decisions/0009-error-send-sync.md`'s shape and DR-8. It lands in `NeonError::Transport` one layer up (`error.rs:66-72`). |
| **EC-002** | The endpoint answers with a non-2xx status. | **Not an error.** `Ok(HttpResponse { status, body })`, body untouched. This is the condition the whole story is arranged around; see AC-002. Explicitly including 401/403 — an expired credential surfaces as an answer the decoder reports, not as a transport failure, because the transport cannot tell it from a permission error on one table. |
| **EC-003** | The response body exceeds `MAX_RESPONSE_BYTES`. | A distinct variant of the transport's own `Error`, refused before the whole body is buffered. It is a *different* fact from EC-001 and a caller acts differently on it, so it must be separable by pattern match, not by string. The store's own `NeonError::ResponseTooLarge` and `NeonConfig::max_response_bytes` clamp are one layer up and are not this story's. |
| **EC-004** | `SqlRequest::body()` returns a `serde_json::Error` because a parameter would not serialise. | Surfaces before any I/O is attempted, distinguishable from EC-001 — the request never left the process, and telling the author "the network failed" when their parameter is at fault is a diagnosis they cannot act on. |
| **EC-005** | The endpoint URL is malformed, or the credential is empty/absent. | Refused at **construction** with a named error, not at the first `round_trip`. Constructing a transport that can never succeed is a state the type system can decline to represent (`standards/rust/10-newtypes-and-niches.md`; `standards/rust/30-error-taxonomy.md`). |
| **EC-006** | A 2xx response whose body is not the JSON the crate expects — a proxy's HTML page, a truncated body. | Still `Ok(HttpResponse)`. Decoding is `wire.rs`'s and the store's job; a transport that starts validating shape has taken an opinion the trait says it must not have ("this crate owns the SQL and the decoding, and nothing else", `transport.rs:242-243`). |
| **EC-007** | The live test runs on a machine with no credential. | The **whole invocation** does not run, visibly — reported by the harness as ignored/filtered, never as a pass. It may not be a `#[cfg]` that removes an assertion from the compiled binary (DR-5); the distinction is that a rule which vanishes at codegen cannot be seen to be missing. |
| **EC-008** | The live endpoint is reachable but the branch has been suspended or scaled to zero and answers slowly. | The transport carries a bounded timeout so the live job fails rather than hangs; a hang is the failure mode that gets a live job quietly made non-blocking, which DR-9 and the project risk table forbid. |

## Non-functional

| id | requirement | why it is live here |
| --- | --- | --- |
| **NF-001** | The chosen client and its whole transitive tree must compile at the MSRV, **1.97.1**. | ADR-0029 raised the floor because of a *dependency's build script*, and five of the five database crates in this workspace declare no `rust-version` at all, so `cargo hack --rust-version` cannot protect the floor against them (`CLAUDE.md`, *Binding constraints* 5). CI's `msrv` job is where this is found; finding it there rather than here costs a round trip. |
| **NF-002** | The published dependency surface must be justified, not merely admissible. | AC-012's `publish = false` removal publishes whatever this story adds. A client that takes its roots from the **platform trust store** avoids the CA bundle that is the known licence trap, and is smaller besides; that is the shape to reach for before the shape that bundles roots. |
| **NF-003** | No duplicate TLS or crypto stack enters the workspace graph. | `deny.toml` runs with `[graph] all-features = true`; `happenstance-postgres` already brings `sqlx`, and two TLS implementations in one lockfile is a supply-chain and binary-size cost nobody chose. Check `cargo deny check bans` output, not just `licenses`. |
| **NF-004** | No `unsafe` is introduced in `happenstance-neon`. | `unsafe_code = "forbid"` workspace-wide (`Cargo.toml:101-102`). A `wasm32` client reached through `wasm-bindgen` must be usable without this crate writing any. |
| **NF-005** | The default gate gains no measurable wall-clock from this story. | The offline unit tests do no I/O. AC-009's bar is that a laptop with no network is not slowed down or blocked — a transport whose *construction* resolves DNS eagerly would violate this without failing anything. |
| **NF-006** | The credential never appears in a `Debug`, a `Display`, or an error message. | The transport owns the credential (AC-001), the live job prints with `--show-output`, and CI logs are readable. Hand-write `Debug` to redact it rather than deriving one — `standards/rust/12-manual-impls-and-derive-traps.md` is the atom, and a derived `Debug` on a struct holding a connection string is exactly the trap it names. |
| **NF-007** | Determinism: the offline tests must not depend on map ordering or on the endpoint's behaviour. | Header assertions compare against `SqlRequest::headers()`'s returned `Vec` order, and body assertions compare parsed JSON, not byte strings — a byte comparison of serialised JSON is a test that fails on a `serde_json` patch release. |

## Implementation notes (non-prescriptive)

None of this is binding; the acceptance criteria are. These are the traps the
grounding turned up, written down so they are not rediscovered.

- **Split `transport.rs` before it grows two clients.** A `transport/mod.rs` holding
  the trait, `SqlRequest`, `SqlStatement`, `HttpResponse`, `IsolationLevel` and
  `NullTransport` unchanged, with `transport/host.rs` and `transport/wasm.rs` behind
  `#[cfg(not(target_arch = "wasm32"))]` / `#[cfg(target_arch = "wasm32")]`, keeps the
  seam legible and keeps the `cfg` attributes in one place each. RS-52-3 asks that
  the `cfg` cover the probe *and* its caller — the `pub use` at
  `crates/happenstance-neon/src/lib.rs:120-123` is the caller.
- **AC-003 is only testable offline if request-building is separable from sending.**
  A private `fn request_parts(&self, request: &SqlRequest) -> Result<(Url, Vec<(&str, String)>, Vec<u8>), Error>`
  — pure, no I/O — is what makes "the transport adds nothing of its own" assertable
  without a network or a mock server. Building the request inline inside the async
  send is what forces the alternative: a local HTTP listener in the default gate,
  which AC-009 will then argue with.
- **The `--fast` trap, stated plainly.** `.redkiln/config.yaml` wires this project's
  non-terminal bar to `cargo xtask ci --fast`, and `--fast` drops **exactly** the
  `OPTIONAL` steps — the two feature powersets and `cargo deny check`
  (`xtask/src/main.rs:840-846`). Those are the two steps that catch this story's two
  most dangerous defects. Run the **full** `cargo xtask ci` at least once and record
  it; a green `--fast` proves neither AC-007 nor AC-008.
- **AC-006's test wants a generic bound, not a concrete type.** The lesson is already
  paid for in `happenstance-core`: an assertion at a concrete type passes by
  auto-trait leakage whatever the trait said, and only a bound written *at the
  definition* discharges the obligation before monomorphisation (`CLAUDE.md`,
  *Binding constraints* 3). Write the helper as `async fn drives<T: SqlTransport>(t: T)`,
  hold the future across an await, and run it inside `LocalSet` with `NullTransport`.
- **The wasm32 client compiles but is not run here.** Executing anything under a real
  edge runtime is `cloudflare-durable-object-store`'s (`project.md`, *Out of scope*).
  Say so in the rustdoc rather than letting a reader assume the `fetch` path has been
  exercised — an unrun implementation that claims otherwise is the same lie a
  `todo!()` body tells about a signature.
- **Where the three answers get written.** `transport.rs:1-15` and `lib.rs:91-99` are
  the two places a reader currently meets "this crate owns no HTTP client". Those are
  the paragraphs AC-010 replaces. The ledger carries the *evidence* (command output);
  the rustdoc carries the *reason*. Neither substitutes for the other, and neither is
  an ADR — no decision record is owed by this story and none may be written as a side
  effect (`_storymap.md`, *Grain notes*; the repository's standing rule that ADR
  authorship belongs to the runbook's ADR pass).
- **Name the live test file for what it is.** `crates/happenstance-neon/tests/` does
  not exist yet; the first file in it sets the convention
  `neon-fixture-and-live-job` will follow. A name that says *live* saves the next
  author from wondering why it is ignored.

## Tests and CI (merge gate)

Grounded in the Testing brief's two-list structure (`_decomposition.md`, *Testing
brief* Notes §5 and §6): a tree-local gate every story runs, and live infrastructure
that runs outside it. This story adds nothing to the second list — the **Neon CI job
is `neon-fixture-and-live-job`'s** (`_storymap.md`, *Slices*) — so its live run is a
recorded one-off, not a job.

| tier | command / path | proves |
| --- | --- | --- |
| **Unit (offline)** | `cargo test -p happenstance-neon --all-features` → `crates/happenstance-neon/src/transport.rs` (`mod tests`) | AC-003 (body shape, headers present/absent, isolation never in the body), AC-004 (the ceiling refusal and its distinct variant), NF-007 (parsed-JSON comparison). Runs with no network. |
| **Type-level (offline)** | `crates/happenstance-neon/tests/not_send_is_preserved.rs` | AC-006. Drives the store through a generic `T: SqlTransport` inside `tokio::task::LocalSet` with `NullTransport`, holding the future across an await. Also the standing guard that no helper quietly acquired `Send`. |
| **Live round trip (one-off, gated)** | `cargo test -p happenstance-neon --all-features -- --ignored --show-output` → `crates/happenstance-neon/tests/live_sql_round_trip.rs` | AC-001, AC-002. The same invocation shape the Neon CI job will use later (Testing brief Notes §6), run here by hand against a real branch, output recorded in `_ledger.md`. Not wired into CI by this story. |
| **Story grain** | `cargo xtask affected --base main` | AC-009. The `.redkiln/config.yaml` `verify:` story grain — only what this diff could break, green with no network and no credential. |
| **Project grain** | `cargo xtask ci --fast` | AC-009 for the bar `.redkiln/config.yaml:50-55` sets for a non-terminal project. Necessary and **not sufficient** — see the next two rows. |
| **Full gate** | `cargo xtask ci` | AC-007 and AC-008. `--fast` drops exactly `cargo deny check` and the two feature powersets (`xtask/src/main.rs:840-846`), which are the steps that decide this story. Record the literal output of the "wasm32 feature powerset" step, including a `skipped`. |
| **wasm32** | `cargo xtask wasm`; `cargo tree --target wasm32-unknown-unknown -p happenstance-neon` | AC-005. The step named "wasm32 build of the Neon adapter" (`xtask/src/main.rs:264-282`, selected by name at `:783-791`) green under its unchanged name, and the host client provably absent from that target's graph. |
| **Licence / supply chain** | `cargo deny check`; `git diff --stat deny.toml` | AC-008, NF-003. Zero exit with `deny.toml` unmodified, `bans` output read as well as `licenses`. |
| **Docs** | `cargo doc --workspace --all-features --no-deps` under `-D warnings`; review read | AC-010. `missing_docs` and `clippy::missing_errors_doc` are `warn` workspace-wide and the gate denies warnings, so the mechanical half is enforced; the prose half — that the module preamble now says something true — is a human read against the diff. |
| **MSRV (CI)** | the `msrv` job: `cargo hack check --no-dev-deps --rust-version` at 1.97.1, then `cargo test --workspace --all-features` at 1.97.1 | NF-001. The second half exists because `--no-dev-deps` hides exactly the dev-only crates, which is the shape this story may put the client in. |

**Merge DoD** is stated in *PR boundary* above and is not restated here; the one
addition this section makes is that the **full** `cargo xtask ci` — not only
`--fast` — must have been run and recorded.

## Risks and coupling (PR-scoped)

| risk | why it is live in *this* PR | mitigation inside the boundary |
| --- | --- | --- |
| `error_for_status()` is written by reflex and everything looks fine | It is the first line most Rust authors write around an HTTP client; it compiles, and a `SELECT 1` test passes | AC-002's live assertion sends a statement the endpoint *rejects* and requires `Ok` back. The happy-path test alone would not catch it, which is why both are in the same file |
| The candidate client fails `cargo deny check` and the allowlist gets widened to unblock | `deny.toml` is one line away and the failure arrives late, after the client is already wired in | AC-008 plus the `deny.toml` exclusion from the PR boundary. Run `cargo deny check` against the candidate **first**, before the code exists to be attached to it |
| A host-only client behind a non-target-scoped feature ships, and the gate says nothing | The mandatory wasm32 step passes with the feature off; only the OPTIONAL powerset step notices, and it prints `skipped` where `cargo hack` is absent | AC-005 and AC-007 together: the powerset step's literal output is ledger evidence, and `cargo tree --target wasm32-unknown-unknown` is the independent check that does not depend on a tool being installed |
| A `Send` bound sneaks back in through a helper or a boxed future | The host client's own futures are `Send`, so every host-side signature will compile with the bound attached, and nothing complains until `wasm32` | AC-006's generic-bound test, and the two standards atoms named in the context pack pulled *before* either signature is written |
| The story grows a fixture | `NeonFixture` is the obvious next keystroke and the crate has no `tests/` directory to anchor the boundary | The PR boundary's *Explicitly not in this PR* list, and the ordering rationale in Testing brief Notes §5. A fixture here means a transport bug and a conformance failure get debugged as one failure |
| The isolation-header tension gets settled in passing | The live round trip is the cheapest place in the whole plan to look at it, so the temptation is at its maximum exactly here | AC-010 requires the observation to be **recorded and routed** to `neon-conflicting-position-verdict`. Writing a verdict here settles AC-008 (project) in a story that does not own it |
| A credential is committed, or printed into a CI log | The live job prints with `--show-output`, and the endpoint URL and credential are new values entering the tree for the first time | NF-006 (hand-written redacting `Debug`), EC-005 (refused at construction), and the credential read from the environment only — nothing in `NeonConfig`, which has no field for one |
| The wasm32 gate step gets renamed while the module is reorganised | `cargo xtask wasm` selects by name and panics on a miss, so a rename is a gate change wearing a refactor's clothes | Named in the Integration contract's *Wires into* row; AC-005 asserts the name is unchanged |

**Coupling.** Downstream: `neon-fixture-and-live-job` (HS-S0068) constructs whatever
this story names, from `crates/happenstance-neon/tests/`; if the manifest decision
makes the implementation unreachable from a test target, that story is blocked on a
manifest change rather than on its own work. Upstream: nothing. Sideways: this story
must not touch `xtask/src/main.rs`, `.github/workflows/ci.yml`, `deny.toml` or the
`SqlTransport` trait — every one of them is another story's or another project's, and
each is listed in the PR boundary for that reason.

## Dependencies

**Blocks on:** none. `depends_on: []` — this story has no inbound edge and can start
on day one, in parallel with the entire `postgres-live-suite` slice. That parallelism
is deliberate and is the only place this project's plan buys any (`_storymap.md`,
*Merge order* item 2; `_decomposition.md` Architecture §8 point 3).

**Unlocks:**

- `neon-fixture-and-live-job` (HS-S0068) — directly. `NeonFixture::connect` needs a
  real transport to build a client on (`_decomposition.md`, Testing brief Notes §7).
- `neon-append-and-read-over-http` (HS-S0069) — transitively, through HS-S0068.
- `neon-conflicting-position-verdict` (HS-S0070) — transitively, and it additionally
  consumes the isolation-header observation AC-010 records.
- `deskeleton-and-package-readiness` — transitively; it cannot honestly remove
  `publish = false` from a crate whose transport is `NullTransport`, and the manifest
  decision AC-007 records is an input to what removing it publishes.

**Not a dependency, deliberately:** the Postgres schema. Neon inherits migration 1
from `happenstance-postgres`, but this story touches no schema at all (see *Data and
migrations*), which is precisely why it can run first.

## Anchors (progressive disclosure)

Everything load-bearing is in the *Context pack*; these are the artefacts to open at
the moment named, not before. Link, never paste.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-neon/src/transport.rs` | The seam being implemented, and the sentence at `:249-250` the whole story is arranged around. Also `SqlRequest::body()`/`headers()` (`:172-209`), the ceiling (`:49-54`, `:212-216`) and the isolation-header rule (`:56-60`). | Before writing either `impl SqlTransport` — first file open, and read `:1-15` and `:240-298` in full. | AC-002, AC-003, AC-004 |
| `crates/happenstance-neon/src/error.rs` | `NeonError::Transport` is where a transport `Error` lands, and `NeonSqlError` is what a 4xx body must still deserialise into — the concrete assertion AC-002 makes. | While writing the live test's rejection case. | AC-002 |
| `crates/happenstance-neon/src/wire.rs` | `ResponseBody` / `ResultSet` are what the happy-path round trip must decode through, so "the answer arrived intact" is an assertion rather than a status check. | While writing the live test's `SELECT 1` case. | AC-001 |
| `crates/happenstance-neon/src/config.rs` | `NeonConfig` carries no endpoint and no credential — four fields, all table names, isolation and a byte ceiling. It is the proof that the transport type must own both, and that no credential has anywhere else to live. | Before deciding the transport's constructor signature. | AC-001, EC-005 |
| `crates/happenstance-neon/src/lib.rs` | The mount point (`:109`, `:120-123`), the bare-flavour claim that must not widen (`:79-89`), and the "owns no HTTP client" prose AC-010 replaces (`:91-99`). | At mount time, and again when correcting the rustdoc. | AC-005, AC-006, AC-010 |
| `crates/happenstance-neon/src/event_store.rs` | Shows the bare-`EventStore` impl the transport is consumed by, `NullTransport`'s role in keeping doctests compiling (`:76-80`), and the `Conflict` → `ConditionViolated` mapping AC-002 protects (`:194-197`). | When checking that no `Send` bound is implied by the consuming impl. | AC-002, AC-006 |
| `crates/happenstance-neon/Cargo.toml` | No `[dev-dependencies]` table exists and the only features are `event-store`/`projection-store`, both default-on and empty — so every option is a first decision, not an adjustment. | Before the manifest decision, and again when writing the pin. | AC-007 |
| `Cargo.toml` | `[workspace.dependencies]` is where a new pin belongs (RS-50-1); `:56-72` carries the measured `sqlx`/`tls-rustls` licence note with the rejection text pasted in; `:101-136` carries the lints that make AC-010's mechanical half real. | Before adding any dependency. | AC-007, AC-008, AC-010 |
| `deny.toml` | The exact allowlist the candidate must clear (`:1-19`), with `[graph] all-features = true`. It is outside the PR boundary on purpose. | Before choosing the client — run `cargo deny check` against the candidate first. | AC-008 |
| `crates/happenstance-postgres/src/lib.rs` | `:40-47` restates the measurement: the offender is `webpki-roots` (`CDLA-Permissive-2.0`), **not** `ring`. Reaching for the wrong lesson costs a client swap. | When shortlisting clients and their TLS backends. | AC-008, NF-002 |
| `xtask/src/main.rs` | The gate itself: the Neon wasm32 step at `:264-282` selected by name at `:783-791`; the OPTIONAL wasm32 feature powerset at `:558-593`; `cargo deny check` at `:596-600`; and `:840-846`, which states what `--fast` drops. | Before claiming any gate evidence — and specifically before assuming `--fast` proved anything. | AC-005, AC-007, AC-008, AC-009 |
| `standards/rust/52-wasm32-and-target-cfg.md` | RS-52-2 (a feature is not target-scoped) is this story's most likely silent defect; RS-52-3 (a `cfg` covers the probe *and* its caller) is the mount-point rule. | Before writing the first `#[cfg]`. | AC-005, AC-007 |
| `standards/rust/50-dependency-hygiene.md` | RS-50-4 is the "measure the licence before committing" rule; RS-50-1 puts the pin in `[workspace.dependencies]`. | Before adding the client. | AC-007, AC-008 |
| `standards/rust/21-send-is-not-inherited.md` | The atom that explains why a host future being `Send` licenses nothing, and how a bound leaks in through a helper. | Before writing either `round_trip` signature. | AC-006 |
| `standards/rust/22-rpitit-and-lifetime-capture.md` | `round_trip` is RPITIT; what it captures and what a `-> impl Future` in a trait implies is exactly the ground where an accidental bound appears. | Alongside the atom above, same moment. | AC-006 |
| `standards/rust/51-features-and-no-std.md` | If the manifest decision is "a feature", this is the atom that says how to spell it so it is additive and target-correct. | Only if a feature is the chosen answer. | AC-007 |
| `standards/rust/70-rustdoc-obligations.md` | The bar AC-010's public items must clear: `# Errors` naming conditions rather than types, and what the gate enforces mechanically versus what it does not. | While writing the new public items' docs. | AC-010 |
| `standards/rust/40-public-surface-and-evolution.md` | `#[non_exhaustive]` and what a published surface commits to — live because AC-012 later publishes whatever this story adds. | When deciding each new type's shape. | AC-007, AC-010 |
| `standards/rust/30-error-taxonomy.md` | How to shape the transport's `Error` so EC-001, EC-003 and EC-004 are separable by a caller rather than by a string. | When defining each implementation's `Error` type. | AC-004 |
| `standards/rust/12-manual-impls-and-derive-traps.md` | The derived-`Debug` trap, which on a struct holding a connection string is a credential in a CI log. | When deriving anything on the transport type. | AC-001 |
| `.kb/decisions/0001-async-port-flavours.md` | The Accepted atom behind the whole two-flavour design, and the reason `#[async_trait]` and any `+ Send` are forbidden here. | Before AC-006's test, if the reason for the rule is not already obvious. | AC-006 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_decomposition.md` | Architecture §9.2's three ordered questions (`:420-442`) are this story's spine; §7 (`:332-362`) is the gate wiring; Testing brief Notes §5 and §6 (`:584-611`) are the ordering rationale and the offline-gate constraint. | At the start, and again before claiming AC-007 or AC-009. | AC-007, AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/neon-sql-transport/discover.md` | The signal ledger, the four answered questions, the routed fifth, and the three named wrong implementations in full. | If any acceptance criterion here reads as arbitrary — the reason is there. | AC-002, AC-003, AC-005 |
| `.bklg/from-contract-to-published-library/postgres-and-neon-stores/_design.md` | The signed-off "no user-facing surface" determination that makes the composition family N/A rather than skipped. | Only if a reviewer asks why *Interaction quality* declares no composition invariants. | AC-010 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The adapter author and edge-developer personas the criteria above are framed from; the `.kb/product/` layer is still empty, so this is the only citable source. | When judging whether a criterion is stated from user intent or from capability. | AC-001, AC-005 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated**; nothing was added or
   dropped. Twelve behaviour rows collapse to ten criteria: the wasm32 gate step's
   name is folded into AC-005 (the step and the target it proves are one fact), and
   the isolation-header observation is folded into AC-010, whose subject is *what
   this story writes down* — the rustdoc correction, the three §9.2 answers, and the
   routed observation are the same deliverable in three places.
2. **`Interaction quality`'s composition family is N/A by sign-off, not skipped.**
   `_design.md` records no surface and the repository owner approved that
   determination on 2026-08-12. The state family *does* apply, translated into this
   repository's medium per `CLAUDE.md`'s own account of why `_design.md` exists for a
   library, and every applicable invariant is carried by an AC row — none is left as
   a prose bullet.
3. **`cargo xtask ci --fast` is not sufficient for this story**, and that is a new
   observation rather than a restatement. `--fast` drops exactly the `OPTIONAL`
   steps, which are `cargo deny check` and the two feature powersets
   (`xtask/src/main.rs:840-846`) — the two steps AC-007 and AC-008 depend on. The
   project's non-terminal bar is `--fast`; this story's bar is the full gate.
4. **The live round trip is a recorded one-off, not a CI job.** The Neon job belongs
   to `neon-fixture-and-live-job` (`_storymap.md`, *Slices*), so `.github/workflows/ci.yml`
   stays out of the PR boundary. The invocation is written in the same shape the job
   will use so that story inherits it rather than inventing one.
5. **Which client, and which dependency table, is left open on purpose.** AC-007
   requires *a* decision with its consequence stated; it does not prescribe one,
   because the deciding input is `cargo deny check`'s output against a real candidate
   and that has not been run yet. NF-002 states the shape to reach for first — a
   client taking its roots from the platform trust store — as guidance, not as a
   requirement.
6. **No ADR is owed and none may be written here.** The three §9.2 answers are
   recorded in crate rustdoc and the story ledger, which is what "recorded, not
   merely made" means at this grain (`_storymap.md`, *Grain notes*). The project's
   decision record is ADR-0024 and belongs to `adr-0024-position-visibility-mechanism`;
   any DoD-6 amendment belongs to `neon-conflicting-position-verdict`.
7. **`crates/happenstance-neon/tests/` does not exist yet.** Both test file paths
   named above are new. That is stated so the implementer does not go looking for a
   directory to add to, and so `neon-fixture-and-live-job` inherits a convention
   rather than setting a second one.
