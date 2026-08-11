# Exploration: what the thing is, and whether Crux should be under it

Status: **exploration notes**. Third of three. Nothing here is normative, and **nothing here is an
ADR** — no decision has been taken, and none should be recorded as taken.
Date: 2026-08-08.
Companions: `research-crux-integration.md` (how happenstance and Crux meet),
`research-crux-composition.md` (how an application composes).

---

## 0. Why this document exists

The first two documents design a thing without naming it. It is not `happenstance`, it is not Crux,
and it is not a `happenstance-crux` binding crate. It is the architecture that currently gets
rebuilt from scratch on every project that wants a local-first, event-sourced, cross-platform Rust
application — and the interesting question is not how to build it but **what it is made of, and what
it is downstream from**.

Two questions follow, and they turn out to be the same question:

1. What is the scope of the thing, and which layer does Crux enter at?
2. Do we accept Crux as a foundational dependency, fork it, or write our own?

---

## 1. The stack, and where the product actually is

| Layer | Contents | Crux? |
|---|---|---|
| **0** | `happenstance-core` — the contract, ports, value types, conformance suite | no |
| **1** | `happenstance` — the typed layer: `Decider`, `Codec`, `DomainEvent`, `Projection`, `Namespace`, `Convergence` | no |
| **2** | `happenstance-sync` + the sync runner (peer selection, watermark policy, retry) | no |
| **3** | **`happenstance-crux`** — the effects port: `StoreOperation`/`SyncOperation` and their results, the `Operation` impls, the `Facet` wire mirrors, the command builders, and the re-exports of `Command`/`Request` under our own names | **yes, and only here** |
| **4** | **the application framework** — the slice contract, context traits, app scaffold, the invalidation read path, the N-core test harness | via 3 |
| **5** | the application and its shell | via 4 |

**Layer 4 is the product.** It is the bespoke architecture recreated per project, and it is the only
layer with no home today. Layers 0–2 are happenstance and already exist or are specified. Layer 3 is
the seam under negotiation.

**Layer 3 is `happenstance-crux`, and that is the whole of the Crux dependency.** It is small —
operation types, `impl Operation`, the `Facet` mirrors the orphan rule forces anyway, command
builders generic over `Effect`, and a mesh test harness. If it grows fold machinery or domain logic,
layer 1 is underbuilt (a falsifier already recorded in `research-crux-composition.md` §12).

Three consequences worth stating plainly:

- **Layer 4 must import `Command` and `Request` from layer 3, not from `crux_core`.** The slice
  contract's signature is `fn update<Ef, Ev>(…) -> Command<Ef, Ev>`, so those two types reach into
  the framework's public API whether we like it or not. Re-exporting them under our names is what
  keeps the reach auditable — one crate to grep, one crate to change.
- **But be honest about what that buys.** Aliases make a rename cheap and the surface countable.
  They do *not* make layer 4 portable: it still depends on `happenstance-crux` by name, and
  therefore on Crux.
- **Layer 3 contains two things that are currently one crate**, and separating them is the actual
  strategic decision:
  - **3a, the protocol** — the operation and result types as plain data, plus the "request an
    operation, receive its result later, never block, never touch the model" abstraction. Nothing
    about this is Crux-specific.
  - **3b, the binding** — `impl Operation`, `Request`, `Command`, the typegen derives.

  A second host needs its own 3b and can share 3a. §4 step 3 is how we find out whether 3a is real
  or whether the split is speculative generality.

---

## 2. Where Crux actually enters, and why the risk inverts

The intuitive worry is that Crux will eventually have to solve its parent/child composition problem,
will solve it without an event log to mediate with, and will land somewhere incompatible.

That worry is **misdirected, and the direction it is misdirected in is favourable**.

Look at what the design in `research-crux-composition.md` uses: `Command`, `Operation`, `Request`,
and the `From<Request<Op>>` bound. **`map_event` and `map_effect` never appear.** The whole point of
the generic-over-`Ef`/`Ev` slice contract is that it sits *above* their composition story rather than
inside it. If Crux ships a nesting macro, we ignore it. If they build case-path machinery, we ignore
it. If they never solve it, nothing changes for us.

What we depend on instead is their **most settled** API. The Command RFC is adopted; `Command`
landed in 0.13 (April 2025) and became mandatory in 0.17 (March 2026). Meanwhile the parts that have
churned three times in eighteen months are all shell-facing build concerns:

| Surface | Churn | Our exposure | Blast radius |
|---|---|---|---|
| `Command` / `Operation` / `Request` | settled since 0.13, RFC adopted | **heavy** | architectural |
| composition (`map_event`/`map_effect`, nested state machines) | unsolved, will change | **none** | none |
| typegen (`crux_cli` removed, serde-reflection → facet) | high | build step | low |
| FFI (UniFFI → BoltFFI) | high | build step | low |
| middleware / `Layer` | deprecated after being built | moderate | moderate |

Two leaks are real today and are evidence *for* a firewall rather than against the dependency:
`facet = "=0.46.5"` is an exact pin that propagates to every consumer (recorded as G11 in
`research-crux-integration.md`), and using middleware imposes `A::Model: Send + Sync + 'static`,
which forbids `Rc` in projections. Both are Crux design decisions reaching into our types.

**Project facts relevant to the decision**: Apache-2.0, © Red Badger Consulting Limited. ~332k
downloads all-time across 54 versions; two named maintainers (Stuart Harris, Viktor Charypar); the
README states it is pre-1.0 with "occasional breaking changes to the API." Production users include
Red Badger, Proton and Photoroom. Small but real, consultancy-anchored, and the licence makes
forking legally trivial — which means **the licence is not the constraint; maintenance is**.

---

## 3. The three options

### Fork — the worst of the three

It carries the maintenance cost of ownership without the design freedom. Merging against a pre-1.0
dependency that has reshaped its FFI layer annually is a compounding tax, and it would mean forking
the whole project to change a part we do not use. It also means inheriting the bus factor of a
two-maintainer project *and* keeping a divergent copy in sync. Nothing about the architecture
requires a modified Crux.

### Write our own — smaller than it sounds, and that cuts both ways

The important asymmetry: **the replaceable part and the expensive part are different parts.**

- `Command` + `Core` + a resolve registry is an executor over a task slab with effect/event channels
  and a driven poll loop. Bounded — weeks, not years. It is also *nearly* something we need anyway
  for the server host and the DST harness.
- What is genuinely expensive is typegen for four languages plus the FFI boundary. But that is
  `facet-generate` and `boltffi` underneath, which a reimplementation would depend on *directly*
  rather than rewrite. So "write our own" really means **own the executor, reuse the ecosystem** —
  much less frightening than "reimplement Crux."

The honest steelman: our composition model is genuinely different, we would design the effect loop
knowing that, we would avoid the `facet` pin, and we would not inherit `Model: Send + Sync`.

The honest rebuttal, and it is decisive: **nothing in the thesis requires owning the executor.**
Log-as-mediator, DCB deciders, convergent projections, flat slices — every differentiated claim lives
above the effect loop. Spending months there is spending them on the least differentiated layer in
the stack.

### Adopt — with a firewall

Recommended. §4.

---

## 4. The recommendation, and how to make it an empirical question

1. **Adopt Crux as-is at layer 5.** Do not fork.
2. **Confine the dependency to `happenstance-crux` (layer 3).** Layer 4 imports `Command` and
   `Request` from there under our own names; `crux_core` appears in exactly one `Cargo.toml`. This
   costs almost nothing today, makes the surface countable, and makes a rename cheap — but it is
   auditability, not portability, and §1 says so.
3. **Write the server / in-process host early, and let it decide the 3a/3b split.** This is the move
   that converts a strategic question into an empirical one, and it is nearly free because *we need
   the thing regardless*: the DST harness needs a drive loop, the `Buffer` effect-router lane exists
   for exactly this, and a server peer likely wants it. Then read the result:
   - if it duplicates real logic → 3a is real; split `happenstance-crux` into a host-agnostic
     protocol crate plus per-host bindings, and repoint layer 4 at the protocol;
   - if it is a thin adapter → the split is speculative generality, Crux stays as-is, and we have
     proven it rather than assumed it.
4. **Publish the slice pattern upstream.** It works on today's API and is arguably a better answer
   than the nested-state-machines chapter. A blog post, a book PR, or an example. Maintainers who
   know a pattern exists are less likely to break it, and we learn how the relationship works before
   being committed to it. Charypar's substantive reply on issue #79 suggests engagement is available.

---

## 5. Positioning determines downstream-ness

This is a naming decision with load-bearing technical consequences.

> If the framework's identity is **"Crux plus event sourcing"**, we are downstream by construction —
> every Crux release is our release.
>
> If its identity is **"a log-mediated local-first architecture that currently ships a Crux host"**,
> Crux is an implementation detail of one binding.

The code can be identical. The difference is that the second framing *forces* the seam to stay
honest and makes a second host a roadmap item rather than a someday. It is also the more accurate
description: the differentiated claim — that the log is the state, the sync mechanism, and the
composition seam — is true independent of what drives the effect loop.

---

## 6. Revisit triggers

So this is a decision with an expiry rather than a permanent bet. Any one of these reopens §3:

1. Crux announces 1.0, or makes a breaking change to `Command` / `Operation` / `Request`.
2. The `-server` host turns out to be non-trivial — i.e. step 3 of §4 returns "duplicates real logic".
3. The `facet = "=0.46.5"` pin causes an actual version conflict in a real dependency tree.
4. Crux goes quiet for two quarters, or the maintainer count drops.
5. `Model: Send + Sync` (or any successor constraint) blocks something the architecture needs.

---

## 7. The supporting toolchain

Assessed against this architecture specifically, not in general.

### Considered

**`proptest` — yes, and badly under-used today.** Eight laws over the query/tag algebra is a good
start on the wrong problem. Three that matter and do not exist: the `Convergence` merge property
(`research-crux-integration.md` §6.2); SY-20 interleaving independence, which needs a genuinely
interesting strategy — generate per-origin sequences, then shuffle only the *interleaving between*
origins; and wire-format round-trips across JSON and postcard, which would have caught defect D1's
five `skip_serializing_if` attributes and D6's `Query::All`-serialises-to-`null` on the day they
landed. Keeping it `cfg(not(target_arch = "wasm32"))` is right.

**`insta` — yes, low risk, and one use is stronger than the obvious ones.** The obvious: `view()`
output (the Crux weather example uses `assert_yaml_snapshot!`) and projection regression. The strong
one: **wire-format drift**. WF-1..12 defines a private format with two encodings and the repo already
carries two live defects in it; a snapshot over serialised envelopes is a cheap standing guard. Note
that SY-20 itself does *not* want insta — convergence compares two live values, so `assert_eq!` is
the assertion; insta catches "this changed and nobody meant it to". Incidental coherence:
`difficient` uses `similar`, same toolchain, if the diffing read path is ever taken.

**`nutype` — not in `happenstance-core`; yes for the typed layer and domain crates.** Core's
newtypes are done and their details are deliberate in ways a macro will not reproduce: the
hand-written `Debug` on `EventType`/`Tag`, `SequencePosition::FIRST`/`next()`,
`Tag::key()`/`value()`/`key_value()`. nutype gives validation, not domain methods, so the `impl`
block survives either way. Three frictions to check before adopting it anywhere: `no_std` support
(core is `no_std + alloc`); whether generated public items carry docs, since `missing_docs` is a
warning and CI denies warnings; and whether it composes with `facet` derive, which anything crossing
the FFI needs. Where it earns its place is **domain crates** — `CourseId`, `StudentId` — with
`sanitize` + `validate` enforcing "this is a legal tag value" at the type level. That pairs directly
with `Namespace` (G3): a validated ID type that knows how to render itself into a namespaced tag
closes the composability hole from both ends.

**`rpds` — conditionally, and later.** Three uses, ascending by how convincing they are: (1) cloning
Model-resident projections for a pre/post `view()` diff — Photoroom pay "a small penalty" here at
animation rates, so possibly a non-problem; (2) row-2 re-folds
(`research-crux-integration.md` §5.3) — keep cheap checkpointed states and re-fold from the nearest
rather than from the insertion point backwards, an algorithmic win; (3) **the DST harness** — snapshot
the model at every step for free, so an invariant failing at step 4,000 comes with its whole history
rather than a seed and a prayer. (3) is the one worth building it for. Two gotchas: rpds
parameterises over its sharing pointer and middleware's `Model: Send + Sync` means the `Arc` flavour,
not `Rc`; and persistent structures lose to plain collections on pure iteration, so read-heavy
rarely-cloned projections would be pessimised. `imbl` (the maintained fork of `im`) is the
alternative with a friendlier API. Do not reach for any of it until step 3 or 4 of the spike shows
the clone cost is real.

### Missing

**`rand_chacha::ChaCha8Rng`, specifically.** `SmallRng` is documented as *not* reproducible across
rand releases. A DST harness whose seeds stop meaning anything after a `cargo update` is worse than
no harness — and Photoroom already flag that adding a dice roll invalidates every seed; the crate
should not do it silently as well.

**`async-channel`.** Blessed by name in Crux's docs ("a lot of universal async code, like async
channels for example, work just fine") and used in their spawn example. It is also the mechanism
behind Photoroom's port pattern — a long-lived task plus a remote-control handle sharing a channel
pair — which is the shape of the sync runner living in a `Command`.

**`getrandom` with its wasm feature wired up.** `StoreId` is 128 random bits and this is the classic
wasm papercut: compiles, then panics at runtime in a browser. Wire it at the same time as G0 so both
wasm surprises surface together.

**`serde_bytes` + `#[facet(typegen::bytes)]` on every byte field crossing the FFI.** Without them a
`Vec<u8>` generates as an array of numbers in Swift and Kotlin. `crux_http::HttpRequest::body`
carries both attributes for exactly this reason, and payloads are the thing being moved.

**`cargo-semver-checks` — the notable absence.** CF-32 already gives the testkit its own version line
because adding a conformance rule is a semver-minor that reddens every passing adapter's CI. That is
a repo that has thought harder about semver than most, with no tool checking it. It slots into
`cargo xtask ci` beside `cargo-hack` and `cargo-deny` under the same probe-and-skip pattern.

**A benchmark, narrowly scoped.** `research-crux-integration.md` §4.3 asserts "sub-millisecond" for
the effect round-trip and that is **unmeasured**. E2E-CASES is right that a benchmark harness is not
a blocked case, but that number is load-bearing for whether store-as-effect is acceptable on the
interaction path. `divan` is lighter than `criterion` and enough to answer it.

### Deliberately not added

**A network simulator.** `turmoil` and `madsim` exist to make socket I/O deterministic. The network
here is already an effect and effects are values — we have what they provide, without a tokio
dependency and without their constraints. Worth writing down, because it is the kind of thing
someone reaches for reflexively at spike step 4.

**`tracing` inside the core.** Handler-side tracing is fine and valuable. But an event-sourced core
is largely self-observing — much of what one would trace is already a fact in the log, with an
`EventId` and an origin. Photoroom had to *build* fine-grained observability across the FFI; a
meaningful fraction of it is free here, and it is worth measuring how much before adding a subscriber
that a DST run then has to be made deterministic around.

---

## Open questions

1. **Does layer 3 split into 3a and 3b, and when?** §4 step 3 answers it empirically — the second
   host is the instrument. Until then it is one crate, `happenstance-crux`, with the re-exports doing
   the auditing.
2. **What is the thing called?** §5 argues the name is a technical decision. "Crux + event sourcing"
   and "log-mediated local-first architecture" are different products with the same source code.
3. **Does layer 4 live in the happenstance repo or its own?** Layers 0–2 are a library with a
   conformance suite and a publication runway; layer 4 is an application framework with a different
   audience and a much faster release cadence. Sharing one workspace means sharing `cargo xtask ci`,
   one MSRV, and one `deny.toml` — which is rigour for layer 4 and drag for layers 0–2.
4. **Is `happenstance` the right dependency to be locked to?** Accepted for now on the grounds that
   we own it and DCB is young enough that a stable third-party alternative does not exist. Worth
   noting that the abstraction already exists — `happenstance-core` *is* a contract crate with a
   conformance suite — so the port is there; only a second implementation is missing.

---

## Sources

- [crux LICENSE — Apache-2.0, © Red Badger Consulting Limited](https://github.com/redbadger/crux/blob/master/LICENSE) · [crux_core on crates.io](https://crates.io/crates/crux_core) · [releases](https://github.com/redbadger/crux/releases)
- [RFC: Command API](https://redbadger.github.io/crux/rfcs/command.html) · [RFC: Effect Router](https://redbadger.github.io/crux/rfcs/effect-router.html) · [crux issue #79](https://github.com/redbadger/crux/issues/79) · [CHANGELOG](https://github.com/redbadger/crux/blob/master/crux_core/CHANGELOG.md)
- [facet-generate](https://github.com/redbadger/facet-generate) · [BoltFFI](https://www.boltffi.dev/)
- In-repo: `research-crux-integration.md`, `research-crux-composition.md`, `spec/SPECIFICATION.md` (CF-32, WF-1..12, SY-20), `references/evaluation/review-blind-spots.md`
