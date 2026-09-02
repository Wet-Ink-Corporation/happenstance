---
item: HS-S0049
stage: discover
created: 2026-08-12T13:02:11.463Z
updated: 2026-08-12T13:02:11.463Z
template_sig: 86ce4036
rendered_sig: 345a5827
---

# Discover — The real Durable Object SqlStorage bindings replace the stand-in

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: replace the `worker`-free stand-in with real Durable Object `SqlStorage` bindings, keep all four modelled properties true and every type `Rc`-shaped so the `!Send` probes still hold, and price `worker` against the MSRV floor and `cargo deny` on the way in | `_storymap.md`, **Slices**, `worker-binding-layer` row | Three obligations in one diff, and the third is a gate question rather than a code question |
| AC-001: no `todo!()` on any adapter path and the scoped `#![allow(clippy::todo)]` gone; AC-005: the error carries a real `worker::Error` | `project.md`, **Acceptance criteria**, AC-001 and AC-005 | This story owns the *bindings* half of both. The bodies are the next two stories' and the ES-6 test is `caller-visible-error-verdict`'s |
| `dependsOn` is empty. Its consumers are the three capability stories in the same milestone | `_storymap.md`, **Merge order** §2 | Real substrate, not a double and not a fixme — and the risks it carries must not be smeared across its consumers |
| ARCH-AC-02: the `!Send` probe module and its four tests survive the swap **unchanged in intent** and still pass, and the error type keeps an `Rc`-shaped field if holding a `worker::Error` would otherwise restore `Send`-ness | `_decomposition.md`, Architecture brief, **Acceptance Criteria** | The probes are the acceptance criterion, not a side effect of it |
| Finding 1: `wasm-bindgen` 0.2.126 carries `unsafe impl Send for JsValue` and `unsafe impl Sync for JsValue` under `cfg(not(target_feature = "atomics"))`, and Workers builds `wasm32-unknown-unknown` without atomics — so `worker::Error` is `Send + Sync` there | `crates/happenstance-cloudflare/src/lib.rs:33-61` (quoted verbatim) | The `Rc<str>` is the instrument. An instrument whose `!Send`-ness disappears under a `cfg` cannot falsify a bound |
| The workspace sets `unsafe_code = "forbid"`, so an adapter can only ever *inherit* that escape hatch by holding a `JsValue`, never write it | `crates/happenstance-cloudflare/src/lib.rs:58-61` | The leak, if it happens, arrives through a field type — which is precisely what a `cargo check` cannot see |
| Finding 2: stringifying loses *forward* compatibility, not the one signal a caller branches on — a Durable Object surfaces SQLite's own text through the thrown `Error`'s `message` and exposes no numeric code | `crates/happenstance-cloudflare/src/lib.rs:63-73` | And stringifying makes the error `Send + Sync`, which destroys the workspace's only ES-6 instrument |
| Finding 4: `send_shape::send_flavour::SendStoreWithLocalError` implements `SendEventStore` with a `!Send` `Error`, and compiles | `crates/happenstance-cloudflare/src/lib.rs:87-94`; `crates/happenstance-cloudflare/src/send_shape.rs` | A standing compile-time proof that must keep compiling. ES-6's own `Rejects:` line names it |
| ES-6 is `[FROZEN]` and its clause names this crate's `Rc<str>`-backed error as the thing that makes the decision falsifiable; ADR-0009 settled the bound and moved the strength into a downstream marker | `spec/SPECIFICATION.md:2629-2686`, ledger row `:8588`; `.kb/decisions/0009-error-send-sync.md` | The bound is not reopened here. What is at stake is whether the clause's own instrument survives the swap |
| The four probes are `#[cfg(all(test, not(target_arch = "wasm32")))]`, with the reason given: a dev-dependency existing only to run four assertions is one `cargo deny` clears on every run | `crates/happenstance-cloudflare/src/lib.rs:149-155`, `:156`, `:180` | That argument **inverts** once `wasm-bindgen-test` is a dependency for the conformance target anyway |
| The seam table: the deliberate no-`worker` note in the manifest is replaced, not deleted silently — it records why the stand-in existed | `_decomposition.md`, Architecture brief Notes §1; `crates/happenstance-cloudflare/Cargo.toml:19-25` | Deleting the note deletes the reasoning a reviewer needs to check the swap against |
| The stand-in models four load-bearing properties with a citation each: synchronous `exec`, a cursor that is not a snapshot, `!Send`/`!Sync` throughout, and a single-threaded but re-entrant object | `crates/happenstance-cloudflare/src/sql_storage.rs:1-24` | All four must stay true of the replacement. They are the acceptance test for "is this the same object" |
| ADR-0029 puts the MSRV at 1.97.1 and names the five database crates that declare no `rust-version` as the reason only running the compiler finds a floor violation | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`; `CLAUDE.md`, binding constraint 5 | `worker` widens that surface. Weigh the floor, do not obey it — and if it moves, that is an ADR, not a silent bump |
| The gate already carries a `wasm32 build of the Cloudflare adapter` step, added because the crate's rustdoc claimed a target nothing checked | `xtask/src/main.rs:245-264` | The standing detector that the swap keeps compiling for the target. It is a `cargo check`, so it sees types and not auto traits |

## Questions

**Which target hosts the unit tier once `worker` lands?** The testing brief flags
this and declines to settle it (`_decomposition.md`, Testing brief Notes §2).
Answered here as far as evidence allows and otherwise **deferred to spec**: the
default is that the probes stay host-native, because `worker::Error` can plausibly
be constructed from a thrown `JsValue` without a live Durable Object binding. If
`cargo test -p happenstance-cloudflare` stops linking, they move behind
`#[cfg(all(test, target_arch = "wasm32"))]` and re-emit through `wasm_bindgen_test`.
The requirement that does not move either way: the four assertions must keep
passing somewhere an ordinary `cargo test` reaches (Architecture brief Notes §7.1)
— a probe only a `workerd` run exercises is a probe no contributor's inner loop
ever runs. **This story adds the wasm32 half regardless of how the host question
falls** — see the mutant below, which is the reason.

**`worker`'s own `rust-version` against the 1.97.1 floor.** Measured at spec, on
introduction, not discovered later by a red `msrv` job. If it forces the floor
higher, that is a new decision atom amending ADR-0029, authored through the
runbook's ADR queue — never a silent edit (`CLAUDE.md`, binding constraint 5).

**`cargo deny`'s widened licence and advisory graph.** `Cargo.toml:19-25`
pre-registered this cost — `worker` drags a large `wasm-bindgen`/`js-sys`/`web-sys`
surface. Checked once at spec, so it is not discovered by the Tuesday `advisories`
cron after merge (`_decomposition.md`, Deployment brief §4).

**Live `JsThrow` or `StringifiedThrow`?** Answered: keep it live. Stringifying
would make the error `Send + Sync` and destroy the only type in the workspace that
can fail a `Send + Sync` bound on `EventStore::Error`. A deviation must argue the
case in ADR-0023, not in a commit message (`_decomposition.md`, Architecture brief
Notes §6).

**CF-39 / CF-40's fixture-limits ownership, and the off-tokio harness shape.**
Neither is this story's. The ceilings are `measured-store-limits`', the atom is
`adr-0023-and-atom-resolutions`' and is coordinated with `sqlite-durable-store`
(HS-P0012); the harness shape is `every-rule-under-workerd`'s and ADR-0023's.
Named here so they are not settled in passing by a manifest edit.

## Decision

The crate is an instrument that models a Durable Object rather than an adapter
that talks to one, and every finding it has made — the `JsValue` auto-trait leak,
the stringification trade, the missing `ConditionViolated` variant, the `Send`
flavour with a `!Send` error — was made against a stand-in with no dependency on
`worker`. This slice replaces the stand-in with the real bindings while keeping
every property the stand-in was built to hold: a synchronous `exec`, a cursor that
is not a snapshot, `!Send` and `!Sync` throughout, and a re-entrant single-threaded
object. It is deliberately its own story because it carries three risks that would
otherwise be smeared across the write path, the read path and the error verdict —
`worker`'s MSRV, `cargo deny`'s widened graph, and the `Send`-ness that
`wasm-bindgen`'s `unsafe impl` can restore by accident on the one target this crate
exists for. The spec will cover: the exact `worker` API surface bound and its
feature set; the mapping from each stand-in item to its real counterpart with the
obligation each carries; the shape of the error's payload field and why it stays
`Rc`-backed; the placement of the `!Send` probes after the swap, including the
`wasm32` twin this story adds; the MSRV and `cargo deny` findings recorded rather
than discovered; and the manifest note that replaces — not deletes — the
deliberate no-`worker` rationale. Nothing `[FROZEN]` is amended: ES-6 is confirmed
against a real `worker::Error`, as ADR-0009 already decided it would be.

## The wrong implementation

**The property asserted on the host and lost on the target.** `worker` links for
`wasm32`; the natural way to keep `cargo test -p happenstance-cloudflare` building
is a `cfg` — the real `worker::Error`-carrying variant behind
`#[cfg(target_arch = "wasm32")]`, and today's `Rc<str>` stand-in kept behind
`#[cfg(not(target_arch = "wasm32"))]` so the host still compiles. Now count what
each check sees. `cargo test` compiles the **stand-in**, so the four probes at
`crates/happenstance-cloudflare/src/lib.rs:180-259` pass exactly as they do today,
including `the_probe_is_not_vacuous`. `cargo clippy --all-targets -- -D warnings`
is green. The gate's `wasm32 build of the Cloudflare adapter` step
(`xtask/src/main.rs:245-264`) type-checks the real variant and never asks whether
it is `Send`, because `Send`-ness is not a compile error — it is the *absence* of
an obligation, and only the autoref probe can observe an absent auto trait. The
result is a workspace whose only ES-6 instrument is an instrument for the host, on
a platform that has no Durable Objects, while the target it exists for silently
inherits `unsafe impl Send for JsValue` under
`cfg(not(target_feature = "atomics"))` — the very leak `lib.rs:33-61` quotes
verbatim and the very reason `JsHandle` holds an `Rc<str>` instead. ES-6 is
`[FROZEN]` and its clause cites this crate's error type as what makes the decision
falsifiable (`spec/SPECIFICATION.md:2629-2686`); under this mutant the clause's own
citation points at code the target never runs.

**Where the detector must live.** On `wasm32`, in this crate's own tree: a
`#[cfg(all(test, target_arch = "wasm32"))]` twin of `not_send_probe`'s four
assertions, emitted through `wasm_bindgen_test` — the dev-dependency the crate is
taking anyway for the conformance target, which is exactly the argument
`lib.rs:149-155` makes in the other direction and which inverts the moment the
dependency is already there. It must carry `the_probe_is_not_vacuous` with it: a
positive control on the host proves nothing about a probe compiled for a different
target. The workspace's existing negative control stays and is not weakened —
`send_shape::send_flavour::SendStoreWithLocalError` (`lib.rs:87-94`), named by
ES-6's own `Rejects:` line, is the compiled proof that a `Send` flavour does not
imply a `Send` error, and if it stops compiling the swap is wrong even with a
green gate. None of this belongs in
`crates/happenstance-testkit/tests/mutation_coverage.rs`: that registry's rows are
stores that fail named conformance rules, and "`!Send` on one target only" is not
a rule the suite states — which is exactly why the assertion has to live in this
crate and has to run where the claim is made.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

**Box 6.** No conformance rule is added here. The tests this story adds are
auto-trait probes over types; they assert on `Send`-ness, never on a position, so
the specification's permission for gaps is not engaged.

**Box 7.** ES-6 is `[FROZEN]` and this story touches it. It is **not changed**:
ADR-0009 already settled that `Error` keeps `core::error::Error + 'static` on both
ports and flavours, and this story's job is to keep the clause's own instrument
genuinely `!Send` once it carries a real `worker::Error`. If holding a
`worker::Error` turned out to make that impossible, the outcome is a new decision
atom and a re-plan — never an edit to `.kb/decisions/0009-error-send-sync.md`,
which `redkiln validate --kb` checks against `HEAD`.

**Box 8.** No conformance rule here seems wrong. One piece of in-tree reasoning
does invert and is corrected in the same change with its reason stated: the
argument at `lib.rs:149-155` for gating the probes **off** `wasm32` — that a
dev-dependency existing only to run four assertions is one `cargo deny` clears on
every run — stops holding the moment `wasm-bindgen-test` is already present for
the conformance target.
