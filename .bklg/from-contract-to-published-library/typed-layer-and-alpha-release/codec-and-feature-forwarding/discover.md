---
item: HS-S0022
stage: discover
created: 2026-08-12T13:01:43.099Z
updated: 2026-08-12T13:01:43.099Z
template_sig: 86ce4036
rendered_sig: 3931af49
---

# Discover — Codec, JSON by default, CBOR and postcard behind forwarded features

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land `Codec` with JSON by default and CBOR/postcard behind **forwarded** features in `crates/happenstance/Cargo.toml`'s `[features]` block, events carrying the tag ADR-0021 sited, round-trip tests per codec — payloads stay `Bytes` at the port and encoding never routes through `happenstance-core/serde` | `_storymap.md:53` (M3 row) | Three obligations: the trait, the manifest, and the negative one — no encoding through the contract crate's serde feature |
| AC-005 — `Codec` ships JSON by default and CBOR/postcard behind features; an event carries a codec tag | `project.md:178-180` | The tag's *home* is ADR-0021's; that it exists is this story's |
| `depends_on: adr-0021-payload-evolution-and-codec-tag` — supplies the tag's home (`Event::metadata` or `Tags`), the version-suffix answer and the upcaster answer, as an accepted atom | `_storymap.md:53`, `:118-119` | The public surface is **invariant** under the tag-home choice (`_design.md:674-679`), so the trait can be specified now and the siting consumed at implementation |
| The binding shape: `Codec { const TAG: &'static str; encode<T: Serialize>; decode<T: DeserializeOwned> }`, with `Json` / `Cbor` / `Postcard` unit structs each behind its own feature, and a **concrete** `CodecError` carrying a boxed `#[source]` rather than an associated error type | `_design.md:439-469`, `:643` | An associated `Error` would add a third type parameter to `CommandError`, `Boundary::absorb`, `Decision` and the runner. A `String` is forbidden outright (RS-30-2) |
| AC-A04 — every feature this project adds is **forwarded**, and none changes what `default-features = false` means. `crates/happenstance/Cargo.toml`'s dependency on `happenstance-core` deliberately names **no** features; its own manifest comment states the failure mode — the facade and the contract crate disagreeing about what `default-features = false` means, *"the kind of difference nobody discovers until a `no_std` build fails three crates away"* | `_decomposition.md:381-390` | The manifest already records the rule and the reason. This story either honours it or silently breaks a promise nobody re-reads |
| `serde` in `happenstance` is the split working, not a violation — ADR-0003 constrains `happenstance-core`, whose `serde` feature covers envelope types only. The standing guards are already in the gate: the `--no-default-features` doc build of the contract crate and the manifest lint | `_decomposition.md:588-595`; `_grounding.md:38-44`; `crates/happenstance/src/lib.rs:29-33` | Getting the crate name backwards forbids the thing ADR-0006's split exists to allow |
| Two of the three codec dependencies are already declared in the root `[workspace.dependencies]`; `postcard` currently sits under the `# --- dev / tooling only ---` header and would move above it. A **CBOR** crate is not declared at all and is a new dependency decision with three consequences to check *before* it is taken: `cargo deny check licenses`, `deny.toml`'s `multiple-versions` posture, and the MSRV | `_decomposition.md:596-606` | `Cbor` is **conditional**: it ships only if `cargo deny check licenses` passes, otherwise the alpha ships `json` + `postcard` and `cbor` is deferred with the reason recorded (`_design.md:739`) |
| The MSRV lesson is procedural: ADR-0029 raised the floor because of a *dependency's build script*, and `cargo hack --rust-version` cannot protect a floor against crates that declare no `rust-version`. Impact is established by **running the compiler at 1.97.1** | `_decomposition.md:607-614`; `CLAUDE.md`, binding constraint 5 | This project does not own the MSRV promise (HS-P0016 does) but it can break it |
| **This project's own merge gate is silent on the axis this story lives on.** `cargo xtask ci --fast` drops the feature powerset, which is the only instrument that proves the codec features compose independently and that forwarding is correct | `_decomposition.md:759-765`, `:859-873` (testing brief, *The feature powerset is not this project's gate*) | The story must run `cargo hack check --feature-powerset -p happenstance -p happenstance-core` locally; the tool resolves on this machine (`CLAUDE.md`, *Commands*) |
| Feature-gated items must render their gate on docs.rs. `crates/happenstance/Cargo.toml` has **no** `[package.metadata.docs.rs]` block today, while `happenstance-core` and `happenstance-testkit` both do | `_design.md:718-725` | Without it every gated item this story adds renders with no gate badge at all — AC-U14's failure |
| A feature only ever **adds** (RS-51-1); turning one off must leave every unrelated item compiling | `_decomposition.md:381-390`; `_design.md:647` | `--no-default-features` is a state the crate-root page must still read completely in (`_design.md:924`) |

## Questions

**Answered here.**

- *Associated error type or concrete?* Concrete `CodecError` with a boxed `#[source]`
  (`_design.md:643`). The rejected alternative is not merely verbose — an associated type
  propagates into four other public shapes.
- *Does `Codec` route through `happenstance-core/serde`?* No, and this is the load-bearing
  negative. Payloads stay `Bytes` at the port and `Codec` operates strictly above it
  (`_decomposition.md:588-595`). The contract crate's `serde` feature is for envelope types
  and replication, not for payloads.
- *Is JSON the default?* Yes — `json` is a default feature, and `commit` (the convenience
  entry point) is gated on it while `commit_with` is ungated (`_design.md:290-300`,
  `:641-642`). A first program names a domain before it names a codec.

**Deferred to `spec`, each with its decider.**

- *Whether `Cbor` ships at all.* Conditional on a licence verdict against this workspace's
  allowlist (`_design.md:739`). If refused, the alpha ships `json` + `postcard` and the
  reason is recorded — a deferral with a reason, not a silent omission.
- *The tag's encoding once ADR-0021 has chosen its home.* Explicitly not prescribed
  (`_decomposition.md:712-723`), and invariant for the signatures above.
- *Whether the powerset run is a story step or a pre-publish step.* Discovery's position:
  run it locally while implementing (it finds the mutant below), and require it at the
  release boundary, which is `publish-0-2-0-alpha-1`'s (`_decomposition.md:1025-1054`).

**Not blocked.** Neither `trybuild` nor `MemoryProjectionStore` is an input to this story.

## Decision

The problem this slice solves is that the typed layer's whole reason to exist is encoding —
`crates/happenstance/src/lib.rs:29-33` already says the discriminator is encoding — and
until a `Codec` exists, every payload in this repository is built with `format!` and parsed
by hand, as `examples/course-subscriptions/src/main.rs:97-100` and `:231` demonstrate. This
story lands the encoding vocabulary and, just as importantly, the *manifest* discipline
that keeps it from leaking downward: JSON on by default, CBOR and postcard behind features
declared in `happenstance`'s own `[features]` block and forwarded rather than enabled on the
dependency, so `default-features = false` means the same thing on both sides of the crate
boundary. The spec for this story covers the `Codec` trait and its `TAG` constant, the three
codec types and their gates, `CodecError`'s variants and its boxed `#[source]`, per-codec
round-trip tests, the `[features]` block and the forwarding rule with the manifest comment's
stated failure mode restated where it will be read, the `[package.metadata.docs.rs]` block
and `#![cfg_attr(docsrs, feature(doc_cfg))]` that `happenstance` lacks today, the licence
verdict gating `Cbor`, and a local `cargo hack --feature-powerset` run over `happenstance`
and `happenstance-core` because this project's own merge gate does not run one. No
`[FROZEN]` clause is amended; payloads remain opaque `Bytes` at the port, which is VT-3
observed rather than altered.

## The wrong implementation

**The mutant: making `Codec` work by enabling the feature on the dependency.**

```toml
# crates/happenstance/Cargo.toml
[dependencies]
happenstance-core = { workspace = true, features = ["serde"] }
```

One line, and it makes everything build. It passes `cargo xtask ci --fast` — this project's
own DoD-6 bar — completely: fmt, clippy `-D warnings`, `cargo test --workspace
--all-features` (which enables everything anyway, so the difference is invisible), all four
`wasm32` steps, the docs build, `spec-trace`, the packaging assertion, and even the
`--no-default-features` doc build of `happenstance-core`, because *that* build is of the
contract crate alone and this change did not touch the contract crate's own defaults.

It is wrong in the exact way the manifest already warns about. Cargo unifies features across
the graph, so any application that depends on `happenstance` now compiles
`happenstance-core` **with `serde` on, unconditionally** — including the adapter author who
deliberately pinned `happenstance-core` with `default-features = false` for a `no_std` or
`wasm32` target and never mentioned `happenstance` at all. Their build grows a dependency
they refused, and the failure surfaces three crates away with nothing pointing back here.
That is P2's stated fear — *not be disturbed; they pin `happenstance-core`, which this
project must not grow* (`_decomposition.md:49`) — realised by a manifest edit that no
reviewer reads twice.

The instrument that rejects it is `cargo hack check --feature-powerset -p happenstance -p
happenstance-core`, and it sits in `OPTIONAL`, which `--fast` drops
(`_decomposition.md:859-873`). So the mutant survives this project's *entire* merge gate and
is caught only by a full `cargo xtask ci` — which is exactly why the powerset run is written
into this story's spec as a local obligation rather than left to the release boundary.

**A second mutant: a codec that encodes through `happenstance-core`'s `serde` feature.**
Using the contract crate's envelope `Serialize`/`Deserialize` impls to encode a payload
compiles, round-trips, and passes every test — and it makes the payload's *shape* a thing
the contract crate knows about, which is the precise line ADR-0003 draws and ADR-0006's
split exists to keep drawn. Payloads are `Bytes` at the port; `Codec` sits strictly above
it. Nothing in the gate distinguishes the two, because both produce valid bytes.

**A third mutant: a `cbor` feature that subtracts.** Shipping `Cbor` by writing
`#[cfg(all(feature = "cbor", not(feature = "json")))]` on the JSON path — so enabling `cbor`
*replaces* JSON rather than adding to it — compiles cleanly in every configuration anyone
tests one at a time, and breaks the moment two consumers in one graph want different codecs:
feature unification turns on both, and one of them silently loses its encoder. RS-51-1's
rule is that a feature only ever adds, and the powerset is again the only instrument that
sees it.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule; its
tests are per-codec round trips (encode → decode → equal) that never construct or observe a
`SequencePosition`. Ticked as vacuously true, checked rather than assumed. **Frozen
clauses:** VT-3 — payloads and metadata stay opaque to every store — is the clause this
story lives next to, and it is *obeyed*: payloads remain `Bytes` at the port and the codec
operates above it. Nothing under `crates/happenstance-core/src/**` is touched, and the
manifest change is confined to `happenstance`'s own `[features]` block.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
