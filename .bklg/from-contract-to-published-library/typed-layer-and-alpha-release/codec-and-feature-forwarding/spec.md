---
item: HS-S0022
stage: spec
created: 2026-08-12T13:46:18.550Z
updated: 2026-08-12T13:46:18.550Z
template_sig: 87bbf1d0
rendered_sig: 2735bd82
---

# Spec — Codec, JSON by default, CBOR and postcard behind forwarded features

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/project.md` |
| This spec | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/codec-and-feature-forwarding/spec.md` |
| Key brief — architecture, ux, testing, deployment (one file) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` |
| Key brief — **binding** API surface design (human-approved, DoD 5) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` |
| Story map (slices, merge order, coverage) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_storymap.md` |
| Roadmap pointer — phase 7's work list and exit criteria | `RUNBOOK.md:3971-4102` |
| Roadmap pointer — why the first release is `0.2.0`, and why the alpha lands here | `RUNBOOK.md:4108-4162` |

Traces to project **AC-005** (*payloads are typed and can evolve*). Depends on
**`adr-0021-payload-evolution-and-codec-tag`** (HS-S0019, M1). Blocks
**`command-loop`** (HS-S0023) and **`projection-trait-and-runner`** (HS-S0026).

## One-line PR slice

Land `Codec` with JSON by default and CBOR/postcard behind forwarded features in
`crates/happenstance/Cargo.toml`'s `[features]` block (AC-A04; a feature only ever
*adds*), events carrying the tag ADR-0021 sited, round-trip tests per codec —
payloads stay `Bytes` at the port and encoding never routes through
`happenstance-core/serde`.

## Executive summary

This PR is where **`serde` arrives in the workspace's application-facing crate**,
and where `crates/happenstance/` stops being a five-line facade in a way a manifest
can see. It lands four things and mounts all four:

1. the `Codec` trait and its `TAG` constant, plus `CodecError`, exactly as
   `_design.md`'s `## Signatures` writes them;
2. the concrete codecs — `Json` (on by default), `Postcard` (gated), and `Cbor`
   **conditionally**, behind a licence verdict this story is required to take
   before adding the dependency rather than after;
3. the `[features]` block that forwards every one of them under AC-A04's rule, so
   `happenstance` and `happenstance-core` never disagree about what
   `default-features = false` means;
4. the **codec tag's** write and recovery path at the seam ADR-0021 sited, such
   that one store can hold two encodings at once and **no adapter has to understand
   the tag to serve a read**.

Delta against the tree at HEAD, so the implementer knows what is new versus what is
being changed in place: `crates/happenstance/Cargo.toml` today declares exactly four
features — `default = ["std", "memory"]`, `std`, `serde`, `memory` — and a single
dependency on `happenstance-core` with no `features` key, carrying a five-line
comment that already states AC-A04's failure mode in the manifest's own words. That
comment is the contract this story extends, not a thing to rewrite.
`crates/happenstance/src/lib.rs:35-52` carries five **"Planned, and specified in
`spec/SPECIFICATION.md`"** bullets; the first of them is `Codec`, and this story is
the one that turns that bullet into an intra-doc link to a real item, in place. And
`crates/happenstance/Cargo.toml` has **no** `[package.metadata.docs.rs]` block, where
both sibling publishable crates do (`crates/happenstance-core/Cargo.toml:54-56`,
`crates/happenstance-testkit/Cargo.toml:56-58`) — so the first feature-gated item
this crate ever ships would render on docs.rs with no gate badge at all unless this
story adds it. It is this story's because this story is the first to gate anything.

What it deliberately does **not** land: `commit`/`commit_with`, the retry loop and
`Committed` (slice-mate `command-loop`), and `DomainEvent`/`DecisionModel`/`Boundary`
themselves (M2, already merged when this runs).

## Context pack

Everything below is a decision already taken. Internalize it before writing code;
open an anchor only when its row says to.

**The `serde` dependency belongs here, and getting that backwards forbids the thing
the crate split exists to allow.** ADR-0003's prohibition attaches to
**`happenstance-core`**, whose `serde` feature covers envelope types only
(`crates/happenstance-core/Cargo.toml:37-49`). After ADR-0006 gave the bare name to
the typed layer, `happenstance` is the crate *whose entire job is encoding* —
`crates/happenstance/src/lib.rs:29-33` already states the discriminator in the tree:
*"Anything that knows how a payload is shaped belongs here."* The binding
consequence is a routing rule, not a permission: **the codec must not reach payload
encoding through `happenstance-core/serde`.** Payloads stay `Bytes` at the port and
`Codec` operates strictly above it. Two standing gate steps are the guard and neither
is this story's to touch: the `--no-default-features` doc build of the contract crate
(`xtask/src/main.rs:495-520`) and the manifest lint that asserts
`happenstance-core` *names* `serde/alloc` and `base64/alloc` rather than inheriting
them (`xtask/src/main.rs:440-455`).

**A feature only ever adds — and `happenstance`'s existing `serde` feature is a
*forwarding* feature, not a "turn on serde" switch.** RS-51-1 is the rule
(`standards/rust/51-features-and-no-std.md:12`); RS-51-2 (`:58`) is the mechanism —
reach an optional dependency with `dep:` and `?/`, because a bare `dep/feature`
inside a default feature silently enables the dependency. The trap specific to this
crate: `serde` the *crate* becomes a **non-optional** dependency here, because
`Codec::encode<T: serde::Serialize>` sits on the ungated trait, while the feature
*named* `serde` must keep meaning exactly what it means today —
`happenstance-core/serde`, the wire-format derives for replication. Those two things
share a name and must not be conflated; whichever way the implementer resolves it,
the manifest says which, in a comment, beside the one already there. **A feature that
subtracts, or a `default-features = false` that means one thing to `happenstance` and
another to `happenstance-core`, fails AC-A04 outright**
(`_decomposition.md:381-390`).

**JSON is on by default because DT-2 was resolved that way, and the entry-point pair
is the resolution.** `_design.md` chose `commit` (JSON, gated `json`) *plus*
`commit_with` (any `C: Codec`, ungated) over a single entry point taking `&C`,
because with one entry point *"every first program then names a codec before it names
a domain."* A `Codec` **enum** was also rejected: shorter, but it forbids a
user-supplied encoding at the alpha for no gain. So `Codec` is a **trait, not
sealed** — a user codec is legitimate — and `json` is in `default`. This story owns
the codec half of that pair; `command-loop` owns the two functions.

**`Codec::Error` is a concrete `CodecError` with a boxed `#[source]`, never an
associated type and never a `String`.** An associated `Error` type would add a third
type parameter to `CommandError`, `Boundary::absorb`, `Decision` and the projection
runner. A boxed `dyn core::error::Error + Send + Sync` is still a typed `#[source]`;
a `String` is forbidden outright (RS-30-2, `standards/rust/30-error-taxonomy.md:71`).
`CodecError` is `#[non_exhaustive]` so a fourth variant is not breaking, and its four
variants are written out in `_design.md`'s `## Signatures` — `UnknownTag`,
`UnknownEventType`, `Encode`, `Decode`. `UnknownTag` is not decorative: it is the
variant that exists **because** something in this story resolves a recovered tag back
to a codec, and if nothing produces it, the tag path was not built.

**The codec tag has exactly two admissible homes, ADR-0021 picks one, and the public
surface is invariant under the choice.** `_design.md` states this in terms:
`Codec::TAG` is a `&'static str` either way and no signature changes, *"so the design
does not wait on it, and the M1 story decides it."* The two homes and what each
costs: **`Event::metadata`** (`crates/happenstance-core/src/event.rs:378-402` — set
by `with_metadata`, read as `Option<&Bytes>`) is opaque to every store, which is what
VT-3 requires, but it is the same field an application wants for
causation/correlation, so the typed layer would be defining private structure inside
a public opaque field; **`Tags`** is queryable and validated, but a tag participates
in DCB query semantics and in every adapter's tag index, so a codec tag would widen
consistency boundaries that have nothing to do with encoding and would consume tag
budget (`crates/happenstance-core/src/limits.rs`). **The constraint that binds
whichever won: no adapter may need to understand the tag.** A design in which a store
must read the codec tag to serve a read puts domain knowledge into the adapter
population — ADR-0003's own reasoning, and the ground on which ADR-0007 rejected a
decoding projection store. Read ADR-0021 as your first act; do not re-decide it, and
do not implement both.

**VT-3 does not forbid the metadata home, and the implementer must not "fix" it.**
`spec/SPECIFICATION.md:629-637` says *the contract layer, a store adapter, and a peer*
MUST NOT parse `data` or `metadata`. The typed layer is none of those three. What VT-3
actually forbids is exactly the failure this story must avoid: a value a store, a
peer, a conformance rule or a query *must be able to see*, hidden in an opaque field.
The codec tag is read by the decoding side of this same crate and by nothing else,
which is why it can live there — and why a `SequencedEvent` reaching a store carrying
a tag the store inspects would be a real violation. `VT-3` is `[FROZEN]`; nothing
here amends it.

**CBOR is a dependency decision with three checks taken *before* it is taken, not
after.** `serde_json` and `postcard` are already declared in the root `Cargo.toml`
`[workspace.dependencies]` — the single source of truth for every third-party version
(RS-50-1) — but `postcard` currently sits under the `# --- dev / tooling only ---`
header at `Cargo.toml:80`, and if a postcard codec ships **that entry moves above the
header**; the manifest records the precedent and its reasoning for `tokio` in place
(*"No longer dev-only, and the move is the point"*). `ciborium` — or whichever CBOR
crate — is **not declared at all**, and the three checks are `cargo deny check
licenses` against this workspace's allowlist, `deny.toml`'s `multiple-versions =
"warn"` posture (`deny.toml:22-24`), and the MSRV, established by **running the
compiler at 1.97.1**, not by reading metadata (ADR-0029's lesson is procedural; five
of five database crates here declare no `rust-version` at all). RS-50-4 is the atom,
and the workspace manifest carries the worked example of a licence rejection nobody
predicted — `webpki-roots`, CDLA-Permissive-2.0, reached through `sqlx`'s
`tls-rustls` (`Cargo.toml:53-70`). `_design.md` pre-authorises the outcome: **if the
licence check refuses, the alpha ships `json` + `postcard`, and `cbor` is deferred
with the reason recorded.** That is a passing outcome for this story, not a blocked
one. What is not permitted is adding the dependency and discovering the refusal in
`cargo xtask ci`'s `cargo deny` step, which `--fast` does not even run.

**Every gated item must render its gate, and no intra-doc link may resolve in only
some feature configurations.** AC-U14 (`_decomposition.md:188-192`): *"a reader must
be able to tell what a feature turns on without opening `Cargo.toml`."* Mechanically
that is RS-70-4 + RS-51-5 — `#![cfg_attr(docsrs, feature(doc_cfg))]` in `lib.rs` plus
`[package.metadata.docs.rs] all-features = true` / `rustdoc-args = ["--cfg",
"docsrs"]` in the manifest — and RS-70-2 (`standards/rust/70-rustdoc-obligations.md:93`),
which is not hypothetical here: three links to `MemoryEventStore` were broken without
the `memory` feature for as long as that gate existed, and rustdoc treats a broken
intra-doc link as a **hard error**, not a warning (`xtask/src/main.rs:495-501`).
`_design.md`'s anti-pattern 6 is the reviewable form: *a feature-gated item's page
shows no gate badge while `Cargo.toml` gates it*.

**The persona-journey slice.** This is the application author's first hour, journey
*"Choose a contract before a database"*, activity **A3 — run a command against a
store** (`_storymap.md`, Backbone), and specifically its first verb: **encode a
payload**. What this story makes true for that person is that `cargo add happenstance`
gets them a working payload encoding with **no feature flag typed and no codec named**
— and that when they later need a compact wire format, turning on `postcard` adds a
type and takes nothing away. The evaluator persona (*"Decide in one sitting"*) meets
the same work as one table on the crate-root page: what each feature turns on, in one
clause each.

**The alternative that lost is named once, where the reader is.** RS-70-5, and
`CLAUDE.md`'s *Who you are working with*: the audience is fluent in event sourcing and
**new to idiomatic Rust**. The rejected alternatives above — the `Codec` enum, the
associated error type, the single codec-taking entry point, the other tag home — each
belong in exactly one doc comment, not repeated across the module (AC-U15).

## Integration contract

- **Archetype**: `capability` — a user-observable slice: a `cargo add happenstance`
  user encodes and decodes a typed payload without naming a codec, and reaches CBOR
  or postcard by adding a feature.
- **Slice / milestone**: **M3 `codec-and-command-loop`**. Slice-mate: **`command-loop`**
  (HS-S0023). The two are implemented in one context and mounted as one surface —
  `command-loop` consumes `Codec` in `commit`/`commit_with`, and the crate-root
  first-program doctest is not runnable until both have landed.
- **Mount point**: **`crates/happenstance/src/lib.rs`** — the crate root, which is
  *"the only render path the library has"* (`_decomposition.md`, *Composition roots*,
  #1). Every item lands `pub use`d at the root beside the surviving `pub use
  happenstance_core::*;` (`:75`, AC-A01), and the module doc's `Codec` bullet
  (`:38-40`) becomes an intra-doc link to the real item **in place**. Co-equal
  feature mount: **`crates/happenstance/Cargo.toml` `[features]`**
  (`_decomposition.md`, *Composition roots*, #6). A codec that compiles but is not
  re-exported at the root, or a feature that exists in code but not in the manifest,
  is unmounted and this story is not done.
- **Wires into**:
  - `crates/happenstance-core/src/event.rs` — `Event`/`SequencedEvent`,
    `EventType::from_static` (`:108`, already `const`), and the metadata seam at
    `:378-402` if ADR-0021 sited the tag there.
  - `crates/happenstance-core/src/tag.rs` — `Tags::from_pairs` (`:304`), the only way
    in and fallible, if ADR-0021 sited the tag there instead.
  - `crates/happenstance-core/src/limits.rs` — the tag budget the `Tags` home spends.
  - `crates/happenstance/src/lib.rs:126`-reachable `pub use bytes;` from
    `crates/happenstance-core/src/lib.rs:126` — payloads are `bytes::Bytes` and the
    re-export already exists; do not add a second `bytes` dependency.
  - `Cargo.toml` `[workspace.dependencies]` — `serde` (`:48`), `serde_json` (`:52`),
    `postcard` (`:93`, currently under the dev/tooling header).
  - M2's `happenstance::DomainEvent`, whose `encode`/`decode` name `C: Codec` and
    `CodecError` in their signatures.
- **Public items** (from `_design.md` `## Items`): `happenstance::Codec`,
  `happenstance::Json`, `happenstance::Postcard`, `happenstance::Cbor`,
  `happenstance::CodecError`, and the feature row
  `happenstance [features] json / cbor / postcard / unstable-projection` — this story
  owns `json`, `cbor` and `postcard`; `unstable-projection` is
  `projection-trait-and-runner`'s. It also changes
  `happenstance (crate module doc)` in the two regions named under *Renders surfaces*.
  **Ownership note the implementer needs**: M2 lands `DomainEvent` one milestone
  earlier, and its `encode`/`decode` signatures reference `Codec` and `CodecError`.
  If M2 landed a minimal declaration to make itself compile, this story **completes
  it in place** — the `TAG` constant, the impls, the features, the tag siting — and
  does not introduce a second definition. `_design.md` `## Signatures` is the arbiter
  of the final shape either way.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** The codec
  sits strictly above the port: payloads are opaque `Bytes` to every store by VT-3,
  and the architectural constraint ADR-0021 works under is that **no adapter may need
  to understand the tag** — so an adapter that behaves correctly is one that cannot
  observe this story at all. Adding a rule to `crates/happenstance-testkit/src/suite.rs`
  here would be a rule no adapter can fail, which this repository has explicitly
  named as decorative (`CLAUDE.md`, *The rule that matters*). The story is observed
  instead by per-codec round-trip unit tests and by the feature powerset
  (`_decomposition.md`, testing brief, AC-005 row).
- **Clause(s)**: discharges none; **honors** `VT-3` (`spec/SPECIFICATION.md:629-637`,
  `[FROZEN]`) — payload and metadata stay opaque to the contract layer, every store
  adapter and every peer. `_design.md` cites VT-3 against both `DomainEvent` and
  `Codec`. **No clause is edited by this story**, and `spec/SPECIFICATION.md` is
  outside its PR boundary. There is no typed-layer clause namespace — the
  specification's namespaces are `VT-`, `ES-`, `PS-`, `SY-`, `WF-`, `CF-` — so this
  surface's only contract is `_design.md` plus ADR-0021 (`_design.md`, header).
- **Advances DoD scenario**: initiative **DoD 1** (*@smoke — the worked example runs
  end to end*) — `examples/course-subscriptions` cannot move off
  `format!("…").into_bytes()` payloads until a codec exists, and DoD 1 is the
  scenario that proves it did. Contributes to **DoD 10** (*the published crate looks
  finished* — the rendered documentation build green **under all features**, which is
  the first build this crate has ever had features to render) and to **DoD 13**
  (`cargo xtask ci` green on the assembled whole).

## PR boundary

```
crates/happenstance/src/**
crates/happenstance/tests/**
crates/happenstance/Cargo.toml
Cargo.toml
Cargo.lock
.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/codec-and-feature-forwarding/**
```

**In this PR.** The `Codec` trait and `Codec::TAG`; `CodecError`; `Json`, `Postcard`
and — subject to the licence verdict — `Cbor`; the codec-tag write and recovery path
at the seam ADR-0021 sited; the `[features]` block in
`crates/happenstance/Cargo.toml` and the `[package.metadata.docs.rs]` block beside
it; `#![cfg_attr(docsrs, feature(doc_cfg))]` and the root `pub use`s in
`crates/happenstance/src/lib.rs`; the module doc's `Codec` bullet becoming a real
intra-doc link and the new `# Features` region; the root `Cargo.toml` workspace-
dependency edits (`postcard` moving above the dev/tooling header, a CBOR crate added
or explicitly not added); `Cargo.lock`; per-codec round-trip tests; and this story's
own backlog folder, which carries the ledger.

The implementer **may** also touch the composition-root and wiring files named in the
Integration contract to mount this slice — that is the mount, not scope drift, and
`crates/happenstance/Cargo.toml`, `Cargo.toml` and `crates/happenstance/src/lib.rs`
are all inside the fence above for exactly that reason.

**Explicitly not in this PR.**

- `commit`, `commit_with`, `Retry`, `Committed`, `CommandError` and the retry loop —
  slice-mate `command-loop` (HS-S0023), same context, separate story.
- `DomainEvent`, `DecisionModel`, `Boundary` and the composition macro — M2, already
  merged.
- `unstable-projection`, `Projection`, `run_projection` — `projection-trait-and-runner`
  (HS-S0026); this story adds no forwarding for a feature whose gating is that
  story's to state (`_design.md`, *Shape decision*, projection-runner-gating row).
- **Any edit under `crates/happenstance-core/src/**`.** Admissible only as the
  recorded outcome of AC-012's route, never as a convenience discovered
  mid-implementation (AC-A02). If the codec path wants something the frozen contract
  does not offer, that is a defect-log entry naming its clause, not a patch.
- `spec/SPECIFICATION.md`, `xtask/src/main.rs` and `xtask/src/proof.rs`. In
  particular the **fifth `wasm32` gate step** compiling `happenstance` with
  `--no-default-features --features std,json` is `edge-flavour-and-wasm-claim`'s
  (HS-S0027, M7) to *register*; this story only has to leave it able to pass.
- `crates/happenstance/README.md` and `CHANGELOG.md` — `publish-0-2-0-alpha-1`'s.
- `deny.toml`. If a CBOR crate cannot pass the allowlist, the answer is to not ship
  `cbor`, not to widen the allowlist.

**Merge DoD.** `cargo xtask ci --fast` is green, plus `cargo hack check
--feature-powerset -p happenstance` and `cargo deny check licenses` run locally
(neither is in `--fast`), the crate-root page renders `Codec` as a real link with a
`# Features` table, and every gated item carries a `doc_cfg` badge under `--cfg
docsrs`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| `Codec` is a public, **unsealed** trait carrying `const TAG: &'static str` plus `encode<T: Serialize>` and `decode<T: DeserializeOwned>` | Signature is fixed by the design, item for item; a user-supplied codec is legitimate and is why the trait was chosen over an enum. `TAG` is the value written into an event so a reader knows how to decode it — ADR-0021 owns *where* it is written, and `TAG` is invariant under that choice | `.bklg/…/typed-layer-and-alpha-release/_design.md` `## Signatures` (`Codec`), `## Shape decision` (*Codec selection*), `## Visibility and stability` |
| `Json` ships in `default`; `Postcard` and `Cbor` are unit structs behind `postcard` / `cbor` | `default = [… "json"]`, so a first program names no codec. `#[cfg(feature = "json")] pub struct Json;` and siblings, each `pub use`d at the crate root | `_design.md` `## Signatures` (`:454-456`-shaped block), `## Items` |
| The `[features]` block forwards, and every feature **adds** | Every core feature stays forwarded (`std`, `serde`, `memory` today) and the new codec features are reached with `dep:` spelling, never a bare `dep/feature` inside a default. Turning any feature off must leave every unrelated item compiling; `--no-default-features` must still build | `crates/happenstance/Cargo.toml` (the existing comment states the failure mode), `standards/rust/51-features-and-no-std.md:12` (RS-51-1), `:58` (RS-51-2), `_decomposition.md:381-390` (AC-A04) |
| The feature named `serde` keeps meaning `happenstance-core/serde` | `serde` the crate becomes a non-optional dependency of `happenstance` (the `Codec` trait bounds are ungated), while the *feature* `serde` continues to forward the contract crate's wire-format derives only. The manifest says which, in a comment beside the one already there | `crates/happenstance/Cargo.toml` `[features]`; `crates/happenstance-core/Cargo.toml:37-49` |
| Encoding never routes through `happenstance-core/serde` | Payloads stay `bytes::Bytes` at the port; `Codec` operates strictly above it. `bytes` is reached through the existing re-export, not a second dependency | `_decomposition.md`, *Dependencies, features, and what the MSRV lesson means here*; `crates/happenstance-core/src/lib.rs:126`; `.kb/decisions/0003-opaque-payloads.md` as scoped by `.kb/decisions/0006-bare-name-to-the-typed-layer.md` |
| An event carries a codec tag, written on the encode side and recovered on the decode side | Sited by ADR-0021 in **one** of `Event::metadata` or `Tags` — never both; two constructors enforcing different rules is the defect this repository already names in prose. One store therefore holds more than one encoding at a time, which is what makes a payload migration possible | `crates/happenstance-core/src/event.rs:378-402`; `crates/happenstance-core/src/tag.rs:304`; `crates/happenstance-core/src/limits.rs`; `crates/happenstance/src/lib.rs:38-40` (the promise already in the tree) |
| No adapter needs to understand the tag | A store serving a read never inspects it. The architectural constraint holds whichever home ADR-0021 chose, and is the ground ADR-0007 rejected a decoding projection store on | `spec/SPECIFICATION.md:629-637` (VT-3, `[FROZEN]`); `.kb/decisions/0007-projection-runner-decodes.md` |
| An unrecognised tag is a typed refusal, not a panic and not a guess | `CodecError::UnknownTag { tag }` is produced by the tag-recovery path. If nothing in the crate can produce it, the tag path was not built | `_design.md` `## Signatures` (`CodecError`) |
| `CodecError` is `#[non_exhaustive]`, boxed `#[source]`, no `String` | Four variants: `UnknownTag`, `UnknownEventType`, `Encode`, `Decode`. Concrete rather than an associated type, so `CommandError`, `Boundary::absorb`, `Decision` and the runner do not each grow a third type parameter | `standards/rust/30-error-taxonomy.md:71` (RS-30-2); `_design.md` `## Shape decision` (*`Codec::Error`* row) |
| Every fallible public item documents `# Errors` by **condition**, not by type | And every unusual construct names the alternative that lost, once, in its own doc comment | `standards/rust/70-rustdoc-obligations.md:243` (RS-70-5); `standards/rust/00-prime-directives.md`; `_decomposition.md` AC-U15 |
| CBOR ships only if the licence check passes, and the refusal is recorded either way | `cargo deny check licenses` is run **before** the dependency is added, with `deny.toml`'s `multiple-versions = "warn"` posture and the MSRV checked by running the compiler at 1.97.1. On refusal the alpha ships `json` + `postcard` and the reason is written down — a passing outcome, not a blocked one | `_design.md` `## Visibility and stability` (`Cbor` row); `standards/rust/50-dependency-hygiene.md:131` (RS-50-4); `Cargo.toml:53-70` (the worked rejection); `deny.toml:22-24`; `.kb/decisions/0029-msrv-raised-to-1-97-1.md` |
| `postcard` stops being dev-only when a postcard codec ships | Its `[workspace.dependencies]` entry moves above `# --- dev / tooling only ---`; `tokio`'s move is the recorded precedent and states the reasoning in place | `Cargo.toml:80`, `:93`, `:71-77` |
| The crate root renders the surface: `Codec`'s roadmap bullet becomes a real link, and a `# Features` region appears | Module-doc regions 4 and 5 of `crate-root-rustdoc`. The `# Features` table gives each feature one clause. No bulleted list under the word "Planned" survives for the items this story lands | `_design.md` `## Composition` → `crate-root-rustdoc` (regions 4-5), `## Anti-patterns` 1 and 6; `crates/happenstance/src/lib.rs:35-52` |
| Every gated item renders its gate on docs.rs, and no intra-doc link is feature-conditional | `#![cfg_attr(docsrs, feature(doc_cfg))]` in `lib.rs` and `[package.metadata.docs.rs] all-features = true` + `rustdoc-args = ["--cfg", "docsrs"]` in the manifest — **neither exists in this crate today**, and both siblings have them. A broken intra-doc link is a hard rustdoc error, not a warning | `standards/rust/70-rustdoc-obligations.md:195` (RS-70-4), `:93` (RS-70-2); `standards/rust/51-features-and-no-std.md:188` (RS-51-5); `crates/happenstance-core/Cargo.toml:54-56`; `xtask/src/main.rs:495-520` |
| No new item shadows a contract name | `pub use happenstance_core::*;` stays and every new item lands beside it. No `happenstance::Query`, `EventStore`, `Tags` or `Event` of our own — two entries with the same name is a silent breaking change to a published facade and an ambiguity in every existing doctest | `crates/happenstance/src/lib.rs:75`; `_design.md` `## Placement and re-export`, `## Anti-patterns` 14 (AC-A01) |
| The codec is flavour-neutral and introduces no `Send` bound | `Codec` is a pure, synchronous trait that touches no port, so it serves the bare and the `Send` flavour identically and there is no `#[async_trait]` to introduce. Stated rather than asserted: this story has no generic code binding a port, and the four existing `wasm32` steps stay green | `CLAUDE.md`, binding constraints 1, 3, 4; `standards/rust/20-two-flavour-ports.md` |
| The codec's dependency graph builds on `wasm32` | Verified locally with the exact invocation M7 will register — `happenstance`, `--no-default-features --features std,json` — so the fifth gate step is green on the day `edge-flavour-and-wasm-claim` adds it. `_design.md` calls the step *"nearly free"* precisely because `serde`/`serde_json` already build there; a codec dependency that does not build on that target is a finding this story must surface, not inherit | `_design.md` `## Shape decision` (*`wasm32` claim* row); `xtask/src/main.rs:105`, and the four existing steps |
| The feature powerset — not `--fast` — is what actually proves this story | `cargo xtask ci --fast` drops both feature powersets and `cargo deny` (`xtask/src/main.rs:830-860`). The implementer runs `cargo hack check --feature-powerset -p happenstance -p happenstance-core` and `cargo deny check licenses` locally; both tools already resolve on this machine | `_decomposition.md`, *The feature powerset is not this project's gate*; `.redkiln/config.yaml`; `CLAUDE.md`, *Commands* |

## Data and migrations

**No schema and no migration — but there is a wire consequence, and it is the point
of the story.**

There is no database, no stored schema and no deployed data: this project's only
store is `MemoryEventStore` (`project.md`, *Out of scope*), nothing is published to
the registry yet, and the first alpha is cut two milestones later by
`publish-0-2-0-alpha-1`. So there is nothing to migrate *from* and no compatibility
window to keep open.

What this story does create is the **format contract for every event written from
now on**, and three properties of it are decided elsewhere and merely realized here:

| Datum | Where it lives | Who decided | Consequence if it drifts |
| --- | --- | --- | --- |
| The payload | `Event::data`, opaque `bytes::Bytes` at the port | `VT-3`, `[FROZEN]` (`spec/SPECIFICATION.md:629-637`) | A store or peer that parses it breaks ADR-0003 at the exact point ADR-0003 claims to win |
| The codec tag | one of `Event::metadata` or `Tags` — never both | **ADR-0021** (the M1 dependency) | Two encodings can no longer coexist in one store, and the migration the crate root already promises (`crates/happenstance/src/lib.rs:38-40`) becomes false |
| Whether `EventType` carries a version suffix, and whether an upcaster needs a read-path hook `EventStore` does not have | `EventType` (`crates/happenstance-core/src/event.rs`) | **ADR-0021** — this is the *payload evolution* half of that record, distinct from the tag's home | If ADR-0021 answered "an upcaster needs a hook the frozen port lacks", that is a **defect-log entry naming its clause and routed to a decision record** (AC-012, AC-A02) — never a patch to `happenstance-core` |

An event written **without** a tag is only reachable in this tree as an event some
other code wrote — nothing in `crates/` produces one today, and the worked example's
`format!("…").into_bytes()` payloads (`examples/course-subscriptions/src/main.rs:96-99`)
are deleted rather than wrapped by `worked-example-on-typed-layer` (M6). The decode
path's behavior for an untagged or unknown-tagged event is therefore a **design
choice this story makes explicit rather than a migration it performs**, and
`CodecError::UnknownTag` is the vocabulary it makes it in.

## Acceptance criteria

Each criterion is a persona goal crossing the full stack — the application author's
first hour (P1, journey *"Choose a contract before a database"*, activity **A3**), the
adapter author who must stay free of domain knowledge (P2), and the evaluator with one
bounded sitting (P4), as
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`
writes them. Verification names a real path; four test files are **new** and land
inside this PR's fence.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an application author who has just run `cargo add happenstance` and typed no feature flag, **WHEN** they encode a typed payload with the crate's default encoding and decode it back into the same Rust type, **THEN** the value round-trips equal, the call site names no codec at all, and `happenstance::Codec` and `happenstance::Json` both resolve from the crate root under default features — because `json` is in `default` and DT-2 was resolved so that a first program names a domain before it names an encoding. | `crates/happenstance/tests/codec_round_trip.rs::json_round_trips_under_default_features` (new), run by `cargo test -p happenstance` with **default** features — not `--all-features`, which would hide a `json` that is only reachable when everything is on. |
| AC-002 | **GIVEN** that same author months later, needing a compact wire format for an edge deployment, **WHEN** they add `features = ["postcard"]` (and `["cbor"]` where AC-008 permits it), **THEN** the new codec type appears at the crate root and round-trips, and **every item that compiled before still compiles** — a feature adds a type and takes nothing away (RS-51-1). | `crates/happenstance/tests/codec_round_trip.rs::postcard_round_trips` (new, `#[cfg(feature = "postcard")]`) plus `cargo hack check --feature-powerset -p happenstance`, which is the only instrument that compiles the combination a user actually picked. |
| AC-003 | **GIVEN** an integrator who writes `happenstance = { version = "…", default-features = false }` because their build cannot afford `std`, **WHEN** they compare the switches they get against a direct dependency on `happenstance-core`, **THEN** the two agree item for item: every core feature stays forwarded, the feature *named* `serde` still means `happenstance-core/serde` and nothing else, every optional dependency is reached with `dep:` / `?/` rather than a bare `dep/feature`, and no feature this story adds subtracts anything (AC-A04, RS-51-1/RS-51-2). | `cargo hack check --feature-powerset -p happenstance -p happenstance-core`, plus `crates/happenstance/tests/manifest_contract.rs::features_forward_and_only_add` (new) reading `include_str!("../Cargo.toml")` and asserting each core feature is forwarded and that `serde` maps to `happenstance-core/serde`. |
| AC-004 | **GIVEN** an adapter author who must never learn a payload's shape, **WHEN** the typed layer encodes a domain value and hands it to `EventStore::append`, **THEN** what crosses the port is opaque `bytes::Bytes` reached through the existing `pub use bytes;` re-export, the encode call is a pure synchronous act that performs no I/O and mutates no store — so a caller can build, inspect and discard a payload with no consequence, the append staying the single irreversible act (AC-U12) — and nothing on the codec path requires `happenstance-core`'s `serde` feature to be on. | `crates/happenstance/tests/codec_round_trip.rs::payload_crosses_the_port_as_bytes` (new), asserting the appended event's `data` is exactly the codec's output; plus `cargo check -p happenstance --no-default-features --features std,json` and the standing `--no-default-features` doc build of the contract crate (`xtask/src/main.rs:495-520`). |
| AC-005 | **GIVEN** an application author whose store already holds events written under one encoding and who now writes new events under another, **WHEN** they read that consistency boundary back through the typed layer, **THEN** each event decodes under the codec **its own tag** names — the tag written on encode and recovered on decode at the single seam ADR-0021 sited, never at both admissible homes — and both generations fold into one decision model without the caller branching on encoding anywhere. | `crates/happenstance/tests/codec_tag.rs::two_encodings_coexist_in_one_store` (new): seed `MemoryEventStore` with one JSON-tagged and one postcard-tagged event, read the boundary, fold both. |
| AC-006 | **GIVEN** a reader that meets an event whose codec tag names an encoding this build does not carry — a postcard-tagged event in a `json`-only build — **WHEN** it attempts to decode, **THEN** it receives `CodecError::UnknownTag { tag }` whose `Display` renders the tag's **actual value**, a typed refusal that names what it could not do: never a panic, never a silent guess at JSON, and never a category message such as *"unsupported encoding"* (AC-U08, RS-30-4). | `crates/happenstance/tests/codec_tag.rs::unknown_tag_is_a_typed_refusal` (new), asserting the variant matches and that `to_string()` contains the offending tag. |
| AC-007 | **GIVEN** an adapter author implementing a store against the frozen contract, **WHEN** events the typed layer wrote are appended through their store and read back, **THEN** their adapter never reads, parses or branches on the codec tag: the conformance suite passes unchanged, no file under `crates/happenstance-core/src/**` is touched by this PR, and no store has to understand encoding to serve a read (VT-3, `[FROZEN]`). | `cargo test -p happenstance-testkit` green with **no edit to the suite** (a rule no adapter can fail would be decorative — `CLAUDE.md`, *The rule that matters*), `cargo xtask affected --base main`, and an empty `git diff --stat main -- crates/happenstance-core/src` asserted at review. |
| AC-008 | **GIVEN** an evaluator with one bounded sitting who reads the feature table to decide whether this crate can pass their licence policy, **WHEN** they look for CBOR, **THEN** they find either a `cbor` feature whose dependency cleared `cargo deny check licenses` **before** it was added, or no `cbor` feature at all together with the recorded reason it was declined — and in neither case has `deny.toml`'s allowlist been widened to make a crate fit. | `cargo deny check licenses` run before the dependency lands (it is `OPTIONAL`, so `--fast` never runs it), `crates/happenstance/tests/manifest_contract.rs::cbor_is_shipped_or_declined_with_a_reason` (new), and an empty `git diff -- deny.toml`. |
| AC-009 | **GIVEN** an evaluator landing on the crate-root page after the alpha, **WHEN** they scan it for what this crate encodes with, **THEN** they meet the real surface and not a roadmap: the `Codec` bullet at `crates/happenstance/src/lib.rs:38-40` has become an intra-doc link to the real item **in place** — same position, same order, same discriminator prose — a `# Features` region gives each feature one clause, every new item is `pub use`d beside the surviving `pub use happenstance_core::*;` and shadows no contract name, and the density budget holds: each new item's first doc sentence ≤ 80 characters, each identifier ≤ 24 characters, any code inside a doc fence ≤ 72 columns, and the module doc ≤ 130 lines total. | `crates/happenstance/tests/doc_surface.rs::crate_root_renders_the_codec_surface` (new) over `include_str!("../src/lib.rs")`, measuring each budget and asserting no `Planned` bullet survives for an item this story landed; plus `cargo doc -p happenstance --no-deps`. |
| AC-010 | **GIVEN** the same evaluator reading the published docs.rs page rather than the source, **WHEN** they open a feature-gated item, **THEN** it carries a `doc_cfg` badge naming the feature that turns it on — the manifest declaring `[package.metadata.docs.rs] all-features = true` and `rustdoc-args = ["--cfg", "docsrs"]`, and `lib.rs` carrying `#![cfg_attr(docsrs, feature(doc_cfg))]`, neither of which exists in this crate today — and the page renders complete and warning-free in all three configurations, with no intra-doc link that resolves in only some of them (RS-70-2/RS-70-4/RS-51-5). | `cargo doc -p happenstance --no-deps`, the same `--no-default-features`, and `cargo +nightly doc -p happenstance --no-deps --all-features` under `RUSTDOCFLAGS="--cfg docsrs -D warnings"`; plus `crates/happenstance/tests/manifest_contract.rs::docs_rs_metadata_is_declared` (new). |

**Coverage of the traced project AC.** Project **AC-005** (*payloads are typed and can
evolve*) is discharged by AC-001 (JSON by default), AC-002 (CBOR/postcard behind
features), AC-003 (forwarded under AC-A04), AC-005 (the tag, and therefore evolution),
AC-006 (the refusal that makes an unknown tag legible) and AC-008 (the CBOR half, ship
or decline). Its remaining clause — *whether `EventType` carries a version suffix and
whether an upcaster needs a read-path hook* — is **ADR-0021's**, decided in the
dependency story and only realized here.

## Interaction quality

Every invariant that applies is carried by an **AC row above**; this section says which
row carries which, and how it is observed. Nothing here is a free-floating bullet,
because a bullet in this section would get no ledger row, no gate and no test.

**STATE invariants.**

| Invariant | Carried by | How it is observed |
| --- | --- | --- |
| **In place, not a context jump** | AC-009, AC-006 | The roadmap bullet becomes a link *at its own line*, in the same order and with the same prose — the page is not restructured and no new page is introduced. And a decode failure is actionable at the call site: the error carries the tag, so nothing sends the reader to a second document (AC-U10). |
| **Non-occlusion — a filter must not hide what it filtered** | AC-006, AC-010 | An unresolvable tag is *named*, not swallowed; a gated item does not vanish from the rendered page, it renders with the badge that says which feature would produce it. The library analogue of AC-U11, and the same principle as the testkit's declined-capability line (`crates/happenstance-testkit/src/contract.rs`). |
| **Preserved selection / preserved state** | AC-003, AC-009, AC-002 | `default-features = false` means after this PR exactly what it meant before, to both crates; `pub use happenstance_core::*;` survives and no new name shadows a contract name (anti-pattern 14, AC-A01); every item that compiled before a feature was added still compiles after. |
| **Reversibility** | AC-004 | Encoding performs no I/O and mutates no store, so building, inspecting and discarding a payload has no consequence. The append remains the single irreversible act (AC-U12). |
| **Reachability without a search engine** (the library reading of keyboard reachability) | AC-009, AC-010 | Every item this story adds is reachable from the crate root's module doc by intra-doc link, in **every** feature configuration — no link resolves in only some of them, and rustdoc treats a broken one as a hard error rather than a warning. |

**COMPOSITION invariants**, taken from the project's signed-off
`_design.md` — this story renders into `crate-root-rustdoc`, regions 4 and 5.

| Invariant | Carried by | How it is observed |
| --- | --- | --- |
| **Presentation exists at all** | AC-009 | The surface is composed rustdoc — a linked vocabulary bullet and a `# Features` table with one clause per feature — not a bare `pub use` that happens to be public. A crate whose only change is re-exports would satisfy every compile assertion and fail this. |
| **Composition / placement** | AC-009 | Regions in the order `_design.md`'s `## Composition` fixes: the `Codec` bullet stays in region 4 in its existing position; the `# Features` table is region 5, below the vocabulary; region 7 (the adapter-author pointer) stays **last** and is not displaced. |
| **Transience** | AC-009, AC-010 | `Json` / `Postcard` / `Cbor` and `CodecError` are **revealed** — reachable in one click from an item page and badged — never promoted onto the landing page (`_design.md`, *Transience policy*). The feature rows are a **revealed** table plus a `doc_cfg` badge on each gated item, satisfying AC-U14's *"without opening `Cargo.toml`"*. Nothing this story adds is persistent chrome; `Codec` appears in the first program only through the codec the default already supplies. |
| **Density budget, with its numbers** | AC-009 | First doc sentence ≤ **80** characters (rustdoc truncates the item-table column near there — anti-pattern 5); identifier ≤ **24** characters; code inside a doc fence ≤ **72** columns (rustdoc `pre` blocks scroll rather than wrap — anti-pattern 3); doc prose ≤ **80** columns; module doc ≤ **130** lines; code ≤ **100** columns (rustfmt's default, `rustfmt.toml` setting only the edition). |
| **Hierarchy** | AC-009 | The link *is* the emphasis — nothing is bolded that is not also reachable — and the Features table stays recessive: below the vocabulary, in table form rather than prose, with no link pointing back up into the first program. |
| **Named anti-patterns** | AC-009 (1, 3, 5, 14), AC-010 (6) | 1: no bulleted list under the word *"Planned"* survives for an item this story landed. 3: no fence on the page needs horizontal scrolling at 1024px. 5: no item summary in the crate's item table ends in an ellipsis *for an item this story adds* — the mock's finding 3 records that inherited `happenstance-core` re-exports already breach it and are frozen by AC-A02, so the budget is checked against this story's own items. 6: no feature-gated item's page lacks a gate badge. 14: no `happenstance::Query`/`EventStore`/`Tags`/`Event` of our own. |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | An event's recovered codec tag names an encoding this build does not carry, or the tag is absent where one was expected. | `CodecError::UnknownTag { tag }`, rendering the tag's value. No panic, no fallback to JSON, no `unwrap`. Carried by AC-006. |
| EC-002 | The payload bytes are not a valid `T` for the codec that produced them — a truncated write, a type renamed under a stable tag. | `CodecError::Decode(source)`, the underlying codec error preserved as a boxed `#[source]` so the chain is walkable. Never a `String` (RS-30-2), never a re-worded message that drops the cause. |
| EC-003 | A value cannot be serialised by the chosen codec — a map with non-string keys under JSON, a borrowed type postcard refuses. | `CodecError::Encode(source)`, same boxing rule. The codec's own error is the diagnosis and must survive to the caller. |
| EC-004 | The CBOR crate fails `cargo deny check licenses`, or drags in a duplicate major version under `deny.toml`'s `multiple-versions = "warn"` posture. | The alpha ships `json` + `postcard`; `cbor` is **deferred with the reason recorded** in the manifest beside the feature block. This is a **passing** outcome for the story (AC-008). Widening `deny.toml` is not an available answer. |
| EC-005 | An `EventType` reaches the decode path that this domain type does not know. | `CodecError::UnknownEventType { event_type }`. The variant is M2's `DomainEvent::decode` to produce; this story must leave it *producible* — a `CodecError` whose `UnknownEventType` arm no code can construct is a variant that has already rotted. |
| EC-006 | A codec dependency does not build for `wasm32-unknown-unknown`. | Surfaced as a finding in this story, never inherited: the edge developer (P3) is the persona whose crate this is, and `edge-flavour-and-wasm-claim` (M7) registers the gate step on the assumption that this story left it green. Declining the codec is preferable to registering a step that cannot pass. |
| EC-007 | The codec path wants something the frozen contract does not offer (an infallible constructor, a read-path hook). | A **defect-log entry naming its clause**, routed to a decision record for AC-012 — never a patch under `crates/happenstance-core/src/**` (AC-A02). |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| NF-001 | **Any new dependency's MSRV impact is established by running the compiler at 1.97.1**, not by reading `rust-version` metadata. | ADR-0029's lesson is procedural: five of five database crates in this workspace declare no `rust-version` at all, so `cargo hack --rust-version` cannot protect the floor against them. `cargo +1.97.1 check -p happenstance --all-features` before the dependency is accepted. |
| NF-002 | **A dependency is justified by counting nodes**, and pinned once in `[workspace.dependencies]`. | RS-50-1. `serde_json` and `postcard` are already there; a CBOR crate is a new entry with a new subtree, and `postcard` moves above the `# --- dev / tooling only ---` header when it stops being dev-only (the `tokio` entry is the recorded precedent, `Cargo.toml:71-77`). |
| NF-003 | **The codec introduces no `Send` bound and no async.** | `Codec` is a pure synchronous trait that touches no port, so it serves the bare and the `Send` flavour identically. No `#[async_trait]` anywhere (binding constraint 1); this story's generic code binds `EventStore`, never `SendEventStore`, and imports one flavour name per module. |
| NF-004 | **Three rustdoc configurations build warning-free.** | Default, `--no-default-features`, and `--all-features --cfg docsrs` on nightly. A broken intra-doc link is a hard rustdoc error; a conditional item without a conditional doc line is the warning the design mock actually caught (`_design.md`, mock finding 2). |
| NF-005 | **No `unwrap`/`expect`/`panic!` on the codec path in library code.** | `-D warnings` clippy under the workspace lints; a panicking codec turns a wire-format problem into a process failure inside somebody else's request handler. |
| NF-006 | **`cargo package --list` still shows both licence files and a README for `happenstance`.** | The packaging step (`xtask/src/main.rs`) already asserts it for all three publishable crates; adding a feature and a `[package.metadata.docs.rs]` block must not disturb the include set. |

## Implementation notes (non-prescriptive)

*Shape decisions are settled above and in `_design.md`; these are the traps, not
instructions.*

- **Read ADR-0021 first, as your first act.** The tag's home is decided, not open, and
  implementing both homes "to be safe" is the two-constructors defect this repository
  names in prose (`crates/happenstance-core/src/projection.rs:47-61`).
- **`serde` the crate and `serde` the feature are two different things sharing a
  name.** `Codec::encode<T: serde::Serialize>` sits on the ungated trait, so `serde`
  the crate is a **non-optional** dependency of `happenstance`; the feature named
  `serde` must keep meaning `happenstance-core/serde`. Whichever way it is resolved,
  say which in a comment beside the one already at `crates/happenstance/Cargo.toml:15-19`
  — that comment is the contract this story extends, not a thing to rewrite.
- **`dep:` and `?/`, never a bare `dep/feature` inside a default feature** (RS-51-2).
  The bare spelling silently enables the optional dependency, which is how a feature
  ends up subtracting the thing it claimed to add.
- **Conditional doc lines are probably needed and no brief asked for them.** The design
  mock found that intra-doc links to gated items emit warnings when the gate is off;
  `#![cfg_attr(not(feature = "json"), doc = "…")]` is the shape that made all three
  builds clean. Budget time for it rather than discovering it in the docs step.
- **M2 may have landed a minimal `Codec` declaration to make `DomainEvent` compile.**
  If so, complete it **in place** — the `TAG` constant, the impls, the features, the
  tag siting — and do not introduce a second definition. `_design.md`'s `## Signatures`
  is the arbiter of the final shape either way.
- **Run `cargo deny check licenses` before adding the CBOR crate, not after.** Neither
  it nor the feature powerset is in `--fast`, so a green local gate says nothing about
  either. The `webpki-roots` / CDLA-Permissive-2.0 comment at `Cargo.toml:53-70` is the
  worked example of a rejection nobody predicted.
- **Name the alternative that lost, once, where the reader is** (RS-70-5): the `Codec`
  enum, the associated `Error` type, and the tag home that was not chosen — one doc
  comment each, not repeated across the module.

## Tests and CI (merge gate)

Grounded in the project testing brief's **AC-005 row** (*"Per-codec round-trip unit
tests … the feature powerset is what actually proves the features compose
independently"*) and its warning that this project's own bar is `--fast`.

| tier | command / path | proves |
| --- | --- | --- |
| Unit / integration | `cargo test -p happenstance` → `crates/happenstance/tests/codec_round_trip.rs` | AC-001, AC-002, AC-004 — per-codec round trip under the **default** feature set, and that the bytes crossing the port are the codec's own. |
| Integration | `cargo test -p happenstance` → `crates/happenstance/tests/codec_tag.rs` | AC-005, AC-006 — two encodings in one `MemoryEventStore`, and the typed refusal for a tag this build cannot resolve. |
| Unit (manifest) | `cargo test -p happenstance` → `crates/happenstance/tests/manifest_contract.rs` | AC-003, AC-008, AC-010 — forwarding, the CBOR ship-or-decline record, and the docs.rs metadata block. |
| Unit (rendered surface) | `cargo test -p happenstance` → `crates/happenstance/tests/doc_surface.rs` | AC-009 — the composition and density invariants an unstyled render would otherwise satisfy silently. |
| Doctest | `cargo test -p happenstance --doc` | `Codec`'s own example compiles; a `?`-using example closes with `# Ok::<(), E>(())` and any `.await` example gets a hidden runtime (RS-62-2), and every `compile_fail` fence is paired with a compiling one (RS-62-1). |
| Feature powerset (**not** in `--fast`) | `cargo hack check --feature-powerset -p happenstance -p happenstance-core` | AC-002, AC-003 — the combination a user actually picked. `cargo xtask ci --fast` drops both powersets (`xtask/src/main.rs:535-594` is `OPTIONAL`), so this is run by hand. |
| Licences (**not** in `--fast`) | `cargo deny check licenses` | AC-008, EC-004 — before the CBOR dependency lands, not after. |
| Docs, three configurations | `cargo doc -p happenstance --no-deps`; the same `--no-default-features`; `cargo +nightly doc -p happenstance --no-deps --all-features` with `RUSTDOCFLAGS="--cfg docsrs -D warnings"` | AC-009, AC-010, NF-004 — a broken intra-doc link is a hard error, and the badge only exists under `docsrs`. |
| `wasm32` | `cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features --features std,json` | EC-006, NF-003 — the exact invocation `edge-flavour-and-wasm-claim` (M7) will register; this story leaves it able to pass, and does not register it. |
| MSRV | `cargo +1.97.1 check -p happenstance --all-features` | NF-001 — the floor is checked by running the compiler, per ADR-0029. |
| Story grain (wired) | `cargo xtask affected --base main` | fmt + `clippy -D warnings` + tests for the touched packages and their dependents; AC-007's "the testkit is untouched and still green". |
| Integration grain (wired) | `cargo xtask ci --fast` | This project's declared bar (`.redkiln/config.yaml`), including all four existing `wasm32` steps and the `--no-default-features` doc build of `happenstance-core`. |

**Merge DoD.** `cargo xtask ci --fast` green, **plus** the powerset and `cargo deny
check licenses` run locally, **plus** the three doc builds. A green `--fast` alone does
not discharge AC-002, AC-003 or AC-008 and must not be reported as if it did.

## Risks and coupling (PR-scoped)

| risk | why it bites here | mitigation inside this PR |
| --- | --- | --- |
| **ADR-0021 has not been ingested when implementation starts.** | The tag's home is the story's central unknown and an accepted atom is immutable, so guessing produces a second atom later, not an edit. | Hard block. `adr-0021-payload-evolution-and-codec-tag` is M1 and `/redkiln:kb-ingest` is a **human handoff**, not a step inside this story (`_storymap.md`, *Merge order*). Do not start the tag path before the atom exists. |
| **Slice-mate coupling with `command-loop`.** | The two are implemented in one context; `commit`/`commit_with` consume `Codec`, and the crate-root first program is not runnable until both land. | Land the codec surface complete and self-testing (round trips, tag, features) so `command-loop` consumes a finished trait rather than co-evolving one. Do not borrow `commit` into this story's tests. |
| **M2 landed a placeholder `Codec` to make `DomainEvent` compile.** | Two definitions, or a `TAG` bolted on beside an existing one, is a silent duplicate on a published surface. | Complete it in place; `_design.md` `## Signatures` arbitrates. If the placeholder disagrees with the design, the design wins and the placeholder is edited, not forked. |
| **The CBOR licence verdict arrives late.** | Adding the dependency and discovering the refusal in a `cargo deny` step `--fast` never runs is rework at exactly the wrong moment. | AC-008 makes the verdict a **precondition** of the dependency, and makes declining a passing outcome. |
| **`postcard` moving above the dev/tooling header changes the dependency graph.** | It stops being dev-only, so `--no-dev-deps` runs now see it; a `cargo hack` step that passed may not. | Run the powerset after the move, not before, and record the move's reason in the manifest as the `tokio` entry does. |
| **The `[package.metadata.docs.rs]` block does not exist in this crate today.** | This story is the first to gate anything, so it is the first for which the block's absence is observable — every gated item would render with no badge at all. | AC-010 makes the block and `#![cfg_attr(docsrs, feature(doc_cfg))]` part of the definition of done, verified by a nightly build rather than assumed. |
| **Conditional doc lines add real complexity to `lib.rs`.** | RS-70-2 is satisfiable only with them once the vocabulary links point at gated items. | Budgeted in the implementation notes; the design mock already established the shape that makes all three builds warning-free. |
| **Nothing under `crates/happenstance-core/src/**` may move.** | The codec path will want an infallible constructor at some point; the convenient edit is one line. | EC-007's route: a defect-log entry naming its clause (AC-012), never a patch (AC-A02). |

## Dependencies

**Blocks on** (must be merged first):

- `adr-0021-payload-evolution-and-codec-tag` (HS-S0019, M1) — the codec tag's home, the
  `EventType` version-suffix question and the upcaster hook question. This story
  *realizes* that decision; it does not take it. Sequenced first because AC-016 requires
  the atom to exist before the code it governs, and because an accepted atom is
  immutable.

Also merged before this story runs, by milestone order rather than by a declared edge:
**M2** (`domain-event-and-decision-model`, `decision-model-composition`), whose
`DomainEvent::encode`/`decode` name `Codec` and `CodecError` in their signatures.

**Unlocks**:

- `command-loop` (HS-S0023, M3, slice-mate) — `commit` is `#[cfg(feature = "json")]` and
  `commit_with` takes `&C: Codec`; neither compiles without this story.
- `projection-trait-and-runner` (HS-S0026, M5) — `run_projection` takes a `&C: Codec`
  and the runner decodes (ADR-0007).
- Downstream of those: `worked-example-on-typed-layer` (M6) can delete
  `format!("…").into_bytes()` rather than wrap it, and `edge-flavour-and-wasm-claim`
  (M7) can register the fifth `wasm32` step against a crate that already builds there.

## Anchors (progressive disclosure)

Everything load-bearing that is **not** in the context pack. Open a row when its
*when to open* column says to — not before, and not all at once.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/adr-0021-payload-evolution-and-codec-tag/spec.md` | The dependency story's own spec — the shape of the decision this story realizes. The **atom does not exist yet** (`.kb/decisions/` holds 0001-0016 and 0029), so this is where the decision's scope lives until `/redkiln:kb-ingest` runs; read the ingested atom instead the moment it exists. | **First act, before writing any tag code.** | AC-005 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md` | `## Signatures` fixes `Codec`, `CodecError` and the three unit structs item for item; `## Visibility and stability` carries the CBOR licence condition; `## Composition` fixes regions 4-5; `## Transience policy`, `## Density budget` and `## Anti-patterns` are the composition invariants. Human-approved (DoD 5) and binding. | Before writing the trait (Signatures) and again before touching `lib.rs` (Composition onward). | AC-001, AC-008, AC-009, AC-010 |
| `standards/rust/51-features-and-no-std.md` | RS-51-1 (`:12`) *a feature adds*; RS-51-2 (`:58`) the `dep:` / `?/` mechanism and the bare-`dep/feature` trap; RS-51-5 (`:188`) the docs.rs manifest configuration. Each rule carries a compiled example and a named wrong implementation. | Before editing `[features]`. | AC-002, AC-003, AC-010 |
| `standards/rust/70-rustdoc-obligations.md` | RS-70-2 (`:93`) no intra-doc link may resolve in only some configurations — a broken link is a hard error; RS-70-4 (`:195`) `doc_cfg` behind `cfg_attr(docsrs, …)`; RS-70-5 (`:243`) name the alternative that lost, once. | Before writing the module doc and the item docs. | AC-009, AC-010 |
| `standards/rust/30-error-taxonomy.md` | RS-30-2 (`:71`) carry a foreign error with `#[source]`, never a `String`; RS-30-4 (`:187`) hand-write `Display` when a carried field would otherwise not be rendered — which is exactly `UnknownTag { tag }`. | Before writing `CodecError`. | AC-006 |
| `standards/rust/50-dependency-hygiene.md` | RS-50-1 (`:12`) pin once in `[workspace.dependencies]` and justify by counting nodes; RS-50-4 (`:131`) a licence rejection is a dependency choice and the offender is rarely the crate you expect. | Before adding a CBOR crate or moving `postcard`. | AC-008 |
| `standards/rust/62-doctests-and-harnesses.md` | RS-62-1 pair a `compile_fail` fence with a compiling one and do not trust its error code; RS-62-2 close a `?`-using example with `# Ok::<(), E>(())`. Governs the codec's own doc example. | While writing `Codec`'s doc comment. | AC-001, AC-009 |
| `crates/happenstance/Cargo.toml` | The four features that exist today and the five-line comment that already states AC-A04's failure mode in the manifest's own words. It is the contract this story extends, not rewrites. | Before the first manifest edit. | AC-003, AC-008, AC-010 |
| `crates/happenstance/src/lib.rs` | The mount point. `:35-52` carries the five *"Planned"* bullets (the first is `Codec`); `:75` is `pub use happenstance_core::*;`; `:10` is the doctest-gated README that must keep compiling. | Before touching the crate root. | AC-004, AC-009 |
| `crates/happenstance-core/src/event.rs` | The metadata seam — `with_metadata` and the `Option<&Bytes>` read at `:378-402` — and `EventType::from_static` at `:108`. One of ADR-0021's two admissible tag homes. | Only if ADR-0021 sited the tag in metadata. | AC-005 |
| `crates/happenstance-core/src/tag.rs` | `Tags::from_pairs` (`:304`) is the only way in and is fallible; the other admissible tag home, with the DCB query-semantics cost. | Only if ADR-0021 sited the tag in `Tags`. | AC-005 |
| `spec/SPECIFICATION.md` | VT-3 at `:629-637`, `[FROZEN]`: the contract layer, a store adapter and a peer MUST NOT parse `data` or `metadata`. The clause this story honors and does not amend — and the reason the typed layer is not one of those three. | When siting the tag, and again if tempted to let a store read it. | AC-004, AC-007 |
| `.kb/decisions/0003-opaque-payloads.md` | The prohibition that attaches to `happenstance-core`, not to `happenstance` — read alongside `.kb/decisions/0006-bare-name-to-the-typed-layer.md`, which is what re-scoped it. Getting this backwards forbids the thing the crate split exists to allow. | If anyone questions why `serde` is arriving in this crate. | AC-004 |
| `.kb/decisions/0007-projection-runner-decodes.md` | The ground on which a *decoding* projection store was rejected — the same reasoning as *no adapter may need to understand the tag*. | When judging whether a tag design has leaked domain knowledge into the adapter population. | AC-007 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | Why the floor moved (a dependency's build script) and why the lesson is procedural: metadata cannot protect a floor, only running the compiler can. | Before accepting any new dependency. | AC-008 |
| `deny.toml` | The allowlist and the `multiple-versions = "warn"` posture (`:22-24`) the CBOR verdict is taken against. It is **outside** the PR boundary: it is read, never widened. | With `cargo deny check licenses`. | AC-008 |
| `Cargo.toml` | `[workspace.dependencies]` — `serde` (`:48`), `serde_json` (`:52`), `postcard` (`:93`, under the `# --- dev / tooling only ---` header at `:80`); the `tokio` move precedent at `:71-77`; the `webpki-roots` licence-rejection comment at `:53-70`. | When declaring or moving a codec dependency. | AC-002, AC-008 |
| `crates/happenstance-core/Cargo.toml` | `:37-49` scopes the contract crate's `serde` feature to envelope types; `:54-56` is the `[package.metadata.docs.rs]` block this crate lacks and the sibling precedent for adding it. | When writing the feature block and the docs.rs metadata. | AC-003, AC-010 |
| `xtask/src/main.rs` | `OPTIONAL` at `:535-601` — the two feature powersets and `cargo deny`, all dropped by `--fast`; `:495-520` the `--no-default-features` doc build of the contract crate; `:105` `REQUIRED`, which this story does **not** edit. | When deciding what "green" proves, and before claiming the gate passed. | AC-002, AC-003, AC-008, AC-010 |
| `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_decomposition.md` | AC-A04 (`:381-390`) the forwarding rule; the testing brief's AC-005 row and *The feature powerset is not this project's gate*; *The codec tag has two possible homes* (`:548-571`); *Dependencies, features, and what the MSRV lesson means here* (`:586-614`). | When the spec above is silent and you need the brief's own reasoning. | AC-003, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P1's *"pick a database and does the domain model work are one decision"*, P3 the edge developer, P4's one bounded sitting — the goals every criterion above is framed from. | If a criterion's intent is unclear, or before rewording one. | AC-001, AC-002, AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated** — AC-001 … AC-010. None
   added, none dropped. The ledger carries the same ten ids.
2. **No conformance rule is added, and that is a decision, not an omission.** The codec
   sits above the port and no adapter may observe the tag, so a testkit rule here would
   be one no adapter can fail — decorative by this repository's own standard
   (`CLAUDE.md`, *The rule that matters*). AC-007 instead asserts the suite passes
   **unchanged**, which is the observable form of "an adapter cannot see this story".
3. **The story's own tests are new files under `crates/happenstance/tests/`** — the
   directory does not exist today (verified). Four files: `codec_round_trip.rs`,
   `codec_tag.rs`, `manifest_contract.rs`, `doc_surface.rs`. All are inside the PR
   fence declared above.
4. **The composition ACs are checked by a test that reads the crate's own source**, not
   by a new `xtask` lint. `xtask/src/main.rs` and `xtask/src/proof.rs` are outside this
   story's boundary (the fifth `wasm32` step is M7's to register), so a
   `crates/happenstance/tests/doc_surface.rs` using `include_str!("../src/lib.rs")` is
   the in-boundary instrument. It is a real test: an unstyled, uncomposed render — bare
   `pub use`s with a surviving roadmap bullet — passes every compile and ARIA-equivalent
   assertion and fails this one.
5. **ADR-0021's atom does not exist yet**, so no anchor cites `.kb/decisions/0021-*`.
   The dependency story's spec is cited instead, with the instruction to switch to the
   atom once `/redkiln:kb-ingest` has run. Inventing the atom path would have produced a
   dead anchor at the exact moment the implementer needed a live one.
6. **Anti-pattern 5 is scoped to this story's own items.** The design mock's finding 3
   records that inherited `happenstance-core` re-exports already breach the 80-character
   summary and 24-character identifier budgets and are frozen by AC-A02. AC-009
   therefore measures the budget against the items this story adds, which is the only
   scope in which it is both checkable and actionable.
7. **`cbor` not shipping is a pass.** AC-008 is written so that a licence refusal
   discharges the criterion with a recorded reason. What fails it is adding the
   dependency first and discovering the refusal in a gate step `--fast` does not run.
