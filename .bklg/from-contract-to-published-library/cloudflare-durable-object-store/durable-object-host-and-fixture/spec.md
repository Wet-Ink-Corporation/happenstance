---
item: HS-S0053
stage: spec
created: 2026-08-12T13:46:51.378Z
updated: 2026-08-12T13:46:51.378Z
template_sig: 87bbf1d0
rendered_sig: 5e7af0c5
---

# Spec — A Durable Object host and the CloudflareFixture mounted on it

## Scope lock

| Artifact | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` |
| **This spec** | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/durable-object-host-and-fixture/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` — Architecture §4a/§4b/§4d (composition roots, the fixture mapping), §7 (standing detectors); Testing §3 (fixtures and seams), §5 (what a weaker rule set would miss) |
| Signed-off design | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` — **no user-facing surface**, approved 2026-08-12. Nothing in this story renders anything |
| Story map | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` — this row, milestone `durable-object-conformance-run` |
| Roadmap pointer | `RUNBOOK.md:160`, `:4241-4307` — phase 9's goal, proof artefact and exit criteria |

## One-line PR slice

Land the Durable Object host (a test-and-example surface, not a second public API)
and `impl Fixture for CloudflareFixture` — one instance is one object's storage,
each `connect()` one handle onto it, `SECOND_HANDLE` `SUPPORTED`, and every
declined `Capability` carrying a real non-empty reason — all reaching the store
through the one `CloudflareEventStore::new(sql)` constructor.

## Executive summary

The write path and the read path are finished before this story starts
(`durable-object-write-path`, `durable-object-read-path`), so
`CloudflareEventStore` can already store and replay events. What does not exist
is anything to **hang it off** and anything to **hand it to a rule**. Nothing in
the tree provides a Durable Object class — `crates/happenstance-cloudflare/Cargo.toml:19-25`
records that `worker` was deliberately absent — and no `Fixture` implementation
for this adapter exists anywhere.

This PR lands exactly those two things, together, and mounted:

1. **A Durable Object host**: a real DO class plus whatever entrypoint the wasm32
   runner addresses, taking its storage handle off the object's own state and
   handing it to `CloudflareEventStore::new(sql)` — the same constructor the
   crate's documented production wiring uses. It is a test-and-example surface;
   the adapter stays a library type any DO can hold, and no new public API
   promise is made.
2. **`CloudflareFixture`**: `impl Fixture` on the bare `EventStore` flavour, with
   one fixture instance mapped onto one object's storage, `connect()` mapped onto
   one more handle onto *that* object, `SECOND_HANDLE` `SUPPORTED`, and every
   capability it declines carrying a reason that names why **this runtime**
   cannot, not that it cannot.

The delta against the project's briefs is that this story **decides the mapping**
Architecture §4d specifies and Testing §3 inherits; it does not decide the
numbers. `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`, `MAX_EVENTS_PER_BATCH` and
the final `REOPEN` / `MID_BATCH_FAULT` verdicts are measurements against the real
runner and belong to `measured-store-limits`; this story establishes the *shape*
those measurements drop into and forbids the shape's two silent failure modes —
a limit inherited by omission, and a capability declared supported without its
method overridden.

It is delivered inside milestone `durable-object-conformance-run`, whose third
member (`every-rule-under-workerd`) invokes the shipped
`event_store_conformance!` against this fixture. Fixture and host without that
target, or that target without this fixture, is precisely the "build it" / "wire
it in" split `_storymap.md` forbids — they merge as one integrated surface.

## Context pack

Everything below is a decision this story must honor. It is stated here, not
linked, because an implementer must not have to open a file to know it.

**The bare flavour, deliberately.** `Fixture::Store` is bound on `EventStore` —
the flavour with **no** `Send` requirement — because it is the weaker bound and
accepts both kinds of adapter (`crates/happenstance-testkit/src/contract.rs:120-125`).
`CloudflareEventStore` implements the bare `EventStore` directly and is the only
adapter in the workspace that does. Never reach for `SendEventStore` here, never
add a `Send` bound to the fixture or to anything it returns, and never import
both trait names into one module — having both in scope makes method calls
ambiguous (`CLAUDE.md`, binding constraint 4). The `Fixture` trait itself carries
no `Send` bound and is not `trait_variant`-derived, and `wasm32` is the target on
which a bound that crept back in stops compiling rather than merely being
redundant (CF-20; `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:9-13`).

**Handles own a refcount; they do not borrow a lifetime.** `Fixture::Store` is an
owned associated type and *not* a GAT, and the reason is not stylistic: `where
Self: 'a` on a GAT implemented for a foreign trait is one of five independently
necessary ingredients of an rustc ICE this repository has already minimised
(rust-lang/rust#158983), which still reproduces on 1.97.1
(`crates/happenstance-testkit/src/contract.rs:97-111`). `MemoryFixture` does this
with an `Arc` clone; a `!Send` fixture does it with an `Rc`
(`crates/happenstance-testkit/src/fixtures.rs:286-291`). `SqlStorage` already
derives `Clone` precisely because the JS object is a handle and cloning it
**aliases the same storage** (`crates/happenstance-cloudflare/src/sql_storage.rs:135-147`)
— that is the mechanism `connect()` is built on, not an accident to work around.

**One instance is one object; two instances share nothing.** This is the single
adapter mistake the fixture shape exists to catch. Pointing every fixture
instance at one backing store is an *adapter's* mistake and no test the testkit
writes about its own fixture could ever observe it, which is why isolation is
itself a conformance rule
(`crates/happenstance-testkit/src/contract.rs:13-23`;
`rules::two_fixture_instances_observe_none_of_each_others_appends`,
`crates/happenstance-testkit/src/suite.rs:210`). Testing §5 names this as the one
genuinely new failure mode this project can introduce and nothing upstream can
catch: *a `CloudflareFixture` that quietly points every fresh instance at the same
Durable Object storage would pass every rule that does not construct two fixture
instances*. A fresh instance therefore means a fresh object id / a fresh
namespace entry — never a shared one, never a cleared one.

**`connect()` is a second handle onto the same object, and `SECOND_HANDLE` is a
MUST.** CF-16 is `[FROZEN]` (`spec/SPECIFICATION.md:7548-7568`) and its rule
`two_handles_observe_each_others_appends` uses `must!` rather than `require!`: a
declined `SECOND_HANDLE` **fails** the rule, quoting the fixture's own words,
rather than skipping (`crates/happenstance-testkit/src/suite.rs:265-271`;
`contract.rs:135-161`). Declining it to turn a red rule green is the escape route
the clause was rewritten to close. It must be `Capability::SUPPORTED`, and
`connect()` must genuinely hand back a second handle onto one object — a second
stub, or a clone of the storage handle whose aliasing `SqlStorage: Clone` already
models — not a second object wearing the same name.

**A declined capability is still emitted as a test, and its reason is the whole
record.** `#[cfg]`-ing a rule out produces a binary in which a skipped rule is
indistinguishable from a passing one, so an author who declines a capability to
turn a red build green gets a green build and no record of the trade
(`crates/happenstance-testkit/src/contract.rs:26-42`, CF-18 at
`spec/SPECIFICATION.md:7597`). `Capability::declined("")` is rejected in a `const
fn`, but for an **associated** const the rejection arrives at *codegen*, not at
`check` or `clippy` — so `cargo build` / `cargo test` catch an empty reason and
`cargo clippy` does not (`contract.rs:400-419`). Write the real reason: *why*
this runtime cannot, not *that* it cannot. Under `__emit_wasm` the line reaches a
human through `RuleOutcome::skip_line` into `console_log!`, because
`RuleOutcome::report`'s `println!` is a measured **no-op** on
`wasm32-unknown-unknown` (`contract.rs:500-531`).

**Declaring a capability supported is a second obligation, not one.** Nothing in
the trait ties `REOPEN: SUPPORTED` to overriding `reopen`, or `MID_BATCH_FAULT:
SUPPORTED` to overriding `arm_mid_batch_fault`; the provided bodies **panic**,
and the reachable path an author actually hits is *declare it supported, forget
the override, run the suite* (`contract.rs:290-307`, `:330-352`). Whatever this
fixture declares supported, it overrides.

**Limits are facts, not trades — and this story does not own the facts.** The
three `Option<usize>` constants are deliberately not `Capability`, because a
store either has a ceiling or it does not and neither answer is a decision
anybody has to justify (`contract.rs:44-54`, `:214-279`). Stating a number is a
promise that exactly that many bytes are accepted and one more is refused as
`AppendError::ExceedsStoreLimit`, and a guessed number fails
`append_reports_exceeded_store_limits` in one direction or the other (CF-40,
`spec/SPECIFICATION.md:7661`). Measuring them is `measured-store-limits`'
(`_storymap.md`, AC-008 split: *the fixture's shape / the measured numbers*).
This story's obligation is the opposite one: the constants must be **stated
deliberately in the fixture**, with the hand-off named, so that no ceiling is
inherited by omission and no number is invented here.

**`REOPEN` is the weaker of the two operations on purpose, and a Durable Object
is why.** Its documentation names this runtime as the motivating case: storage
outlives the isolate, so handle state can be discarded and the store read again,
while restarting the isolate from inside a test is not possible at all
(`contract.rs:163-173`; CF-17 `[PROVISIONAL]` at `spec/SPECIFICATION.md:7570-7586`).
This story declares the constant with the honest first answer against the host it
builds; confirming it against the real runner, alongside `MID_BATCH_FAULT`, is
`measured-store-limits`'.

**The store is reached one way only.** `CloudflareEventStore::new(sql)` takes its
storage by **injection** and must never construct one itself: in production the
handle comes off the object's own state, in the fixture off whatever the harness
has (`crates/happenstance-cloudflare/src/event_store.rs:70-87`, Architecture
§4a). `migrate()` is the schema seam and is what the fixture calls once per
instance. A test that builds a store some other way gives the "one instance, one
object" invariant a second, untested code path (Testing §3).

**The host is a test-and-example surface, not a second public API.** A conformance
run needs a real DO class to hang the store off, and nothing in the tree provides
one. Whether it lives behind a `cfg`, in `examples/`, or in a `tests/` support
module is the implementer's choice; what is **not** a choice is that the
conformance fixture and the documented production wiring reach the store through
the *same* constructor (Architecture §4b). One consequence follows from Rust's
compilation units and is easy to discover late: a bare `#[cfg(test)]` module in
`src/lib.rs` is **not reachable from an integration test**, and the slice-mate's
conformance target is one. Whatever placement is chosen must be reachable across
that boundary.

**The persona slice.** Three people observe this project's output and this story
serves two of them (`_storymap.md`, Backbone). The **adapter author** gets, for
the first time, a fixture they can hand to the shipped macro — the same
five-line shape `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27`
already demonstrates, against a runtime that is not tokio. The **gate reader**
gets, for every guarantee this runtime cannot supply, a line naming the guarantee
and this runtime's reason — rather than a rule that quietly is not there. Nothing
here is user-facing: `_design.md` records **no surface**, approved.

**What this story must not silence** (`_storymap.md`, Standing detectors). The
four `!Send` probe tests including `the_probe_is_not_vacuous`
(`crates/happenstance-cloudflare/src/lib.rs:180-259`) are owned by
`worker-binding-layer`, but a fixture is exactly the kind of new code that can
restore `Send`-ness by accident — a real `JsValue` is `Send + Sync` on non-`atomics`
`wasm32` builds (`lib.rs:33-61`), which is why the handle types hold an `Rc<str>`.
Keep the fixture's whole path `Rc`-shaped; if a probe goes quiet, the change is
wrong even when the gate is green.

## Integration contract

- **Archetype**: `capability`.
- **Slice / milestone**: `durable-object-conformance-run`. Slice-mates, delivered
  in one context as one integrated surface: `every-rule-under-workerd` (invokes
  the shipped macro against this fixture and registers it in the gate step),
  `measured-store-limits` (replaces this story's declared shape with measured
  numbers and the `REOPEN` / `MID_BATCH_FAULT` verdicts).
- **Mount point**: `crates/happenstance-cloudflare/src/lib.rs` — the library's
  composition root and its module list. An item not declared there is not
  compiled by the real build, and the crate's public re-export block (`:128-135`)
  is where the visibility decision for the host and the fixture is actually taken.
  If the host is placed in a `tests/` support module instead, the mount point is
  the test target that declares it (`mod support;`) — never a file no target
  declares. Constructed-but-unmounted is the failure mode this field exists to
  forbid, and here it has a precise shape: a fixture the conformance target
  cannot name.
- **Wires into**:
  - `crates/happenstance-cloudflare/src/event_store.rs:70-87` —
    `CloudflareEventStore::new(sql)` and `migrate()`, the one construction seam.
  - `crates/happenstance-cloudflare/src/sql_storage.rs:135-147` — `SqlStorage`
    and its aliasing `Clone`, the mechanism behind a second handle (real
    `worker` bindings after `worker-binding-layer`).
  - `crates/happenstance-testkit/src/contract.rs:120-353` — `Fixture`,
    `Capability`, `RuleOutcome`; the trait this story implements.
  - `crates/happenstance-testkit/src/fixtures.rs:242-292` — `MemoryFixture`, the
    reference implementation to read before writing this one.
  - `crates/happenstance-testkit/src/lib.rs:312-357` — the
    `event_store_conformance!` arms whose `fixture = …` expression this story
    supplies; `:55-82` for `__emit_wasm`.
  - `crates/happenstance-cloudflare/Cargo.toml` — `wasm-bindgen-test` under
    `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]`, mirroring
    `crates/happenstance-testkit/Cargo.toml`: the attribute resolves in the
    **caller's** scope, never in the testkit.
- **Design-system primitives consumed**: none. `_design.md` declares no surfaces;
  the equivalent here is the testkit's fixture vocabulary
  (`crates/happenstance-testkit/src/fixtures.rs:18-165`), which this story's own
  tests express themselves in rather than reinventing.
- **Renders surfaces**: **none** — `_design.md` records `N/A — no user-facing
  surface`, signed off 2026-08-12. This story renders nothing and must not invent
  a surface to render.
- **Conformance rule(s) that observe this story**:
  `rules::two_fixture_instances_observe_none_of_each_others_appends`
  (`suite.rs:210`) and `rules::two_handles_observe_each_others_appends`
  (`suite.rs:265`) observe the isolation and second-handle mappings directly;
  `rules::acknowledged_writes_survive_a_reopen` (`suite.rs:332`),
  `rules::reopened_store_does_not_reissue_an_event_id` (`suite.rs:2310`) and
  `rules::recorded_time_survives_a_reopen` (`suite.rs:2473`) run or skip on this
  story's `REOPEN` answer; `rules::append_is_atomic_under_a_mid_batch_fault`
  (`suite.rs:2706`) and `rules::arming_a_mid_batch_fault_makes_the_append_fail`
  (`suite.rs:2794`) on its `MID_BATCH_FAULT` answer;
  `rules::append_reports_exceeded_store_limits` on its limit constants. **This
  story adds no rule to the suite** — the obligation is only to run the existing
  ones for real (Testing §5), and a `wasm`-only rule list anywhere in the tree
  fails project AC-002 by construction.
- **Clause(s)**: discharges **CF-16** `[FROZEN]` (`spec/SPECIFICATION.md:7548`),
  **CF-18** `[FROZEN]` (`:7597`) and **CF-23** `[FROZEN]` (`:7920`) for this
  adapter; supplies the fixture-shaped half of **CF-17** `[PROVISIONAL]`
  (`:7570`), **CF-39** `[PROVISIONAL]` (`:7631`) and **CF-40** `[PROVISIONAL]`
  (`:7661`), whose measured halves are `measured-store-limits`'. **No clause is
  amended.** If this runtime cannot satisfy a frozen clause, that is a new
  decision atom and a re-plan, never a line edit (`project.md`, Out of scope).
- **Advances DoD scenario**: initiative **DoD 4** — *"The constrained-runtime
  store passes the suite on its own target: every rule green under the edge
  runtime on `wasm32`, executed in the gate rather than asserted in prose"*
  (`initiative.md:369-372`). This story supplies the fixture and host without
  which no rule can be executed at all; the slice-mate `every-rule-under-workerd`
  is where DoD 4 first goes green.

## PR boundary

**In this PR**

- The Durable Object host: a real DO class and the entrypoint the wasm32 runner
  addresses, taking its storage off the object's own state.
- `CloudflareFixture` and `impl Fixture for CloudflareFixture` — `type Store`,
  `connect()`, the capability constants, and any override a supported capability
  obliges.
- The declaration and visibility decision for both, in the mount point named
  above, plus the crate documentation stating that the host is a
  test-and-example surface and not a public API promise.
- `wasm-bindgen-test` added to this crate's `wasm32` dev-dependency table, and
  whatever `worker` surface the host itself needs beyond what
  `worker-binding-layer` already bound.
- This crate's own targeted tests proving the fixture *contract* holds before the
  full suite is pointed at it: two instances isolated, two handles onto one
  object, a declined reason that is non-empty and runtime-specific, migration run
  once per instance.
- This story's backlog folder (`spec.md` body, `_ledger.md`,
  `implementation-report.md`).

**Explicitly not in this PR**

- The conformance target invoking `event_store_conformance!`, and the crate's
  registration in the gate's step list → `every-rule-under-workerd`.
- The new `xtask` `Step` and the `xtask/src/proof.rs` `Artefact` row →
  `wasm-execution-gate-step` (foundation) and `every-rule-under-workerd`.
- Measured values for `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` /
  `MAX_EVENTS_PER_BATCH`, and the final `REOPEN` / `MID_BATCH_FAULT` verdicts
  against the real runner → `measured-store-limits`.
- Any change to `read`, `append`, `head`, `contains_event_id`, `render_read`,
  `decode_row` or the schema → `durable-object-write-path` /
  `durable-object-read-path`, both merged before this story starts.
- The ES-6 caller-visible error reconstruction → `caller-visible-error-verdict`.
- Any `.kb/` write. ADR-0023, CF-40's ownership resolution and WF-11's atom are
  `adr-0023-and-atom-resolutions`', authored through `/redkiln:kb-ingest` and
  never hand-written into `.kb/decisions/` (`CLAUDE.md`, *Where the work lives*).
- Removing `publish = false`, licence files, README → `publish-ready-crate`.
- Editing any accepted atom, and in particular
  `.kb/decisions/0001-async-port-flavours.md` — accepted atoms are immutable and
  `redkiln validate --kb` checks them against `HEAD`.

The implementer **may** also touch the wiring files named in the Integration
contract to mount this slice — `crates/happenstance-cloudflare/src/lib.rs`'s
module list and re-exports, and `crates/happenstance-cloudflare/Cargo.toml`'s
dev-dependency table. That is mounting, not scope drift. Note that the existing
`wasm32 build of the Cloudflare adapter` step runs `cargo check … --target
wasm32-unknown-unknown` **without** `--tests`
(`xtask/src/main.rs:245-264`), so nothing in `tests/` is type-checked by it
today; extending that step or covering it with the slice's execution step is the
slice-mates' work, but the gap must not be discovered after merge.

**Merge DoD**: `cargo xtask affected --base main` green; a Durable Object host
and a `CloudflareFixture` exist, are declared in a module list the real build
compiles, are reachable from a second compilation unit, and reach the store only
through `CloudflareEventStore::new(sql)`; `SECOND_HANDLE` is `SUPPORTED` and
every declined capability names this runtime's reason; the four `!Send` probes
and `send_flavour::SendStoreWithLocalError` still compile and pass.

**The path fence**, written out here rather than left implicit — `redkiln verify
--grain story` reads the first fenced block of this section:

```
crates/happenstance-cloudflare/src/host.rs
crates/happenstance-cloudflare/src/lib.rs
crates/happenstance-cloudflare/src/js.rs
crates/happenstance-cloudflare/src/event_store.rs
crates/happenstance-cloudflare/src/sql_storage.rs
crates/happenstance-cloudflare/tests/**
crates/happenstance-cloudflare/Cargo.toml
Cargo.lock
xtask/src/proof.rs
standards/rust/**
.bklg/from-contract-to-published-library/initiative.md
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/durable-object-host-and-fixture/**
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/every-rule-under-workerd/**
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/measured-store-limits/**
references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/wasm-execution-gate-step/spec.md
```

**Fence amendment — 2026-08-19, slice `durable-object-conformance-run` repair
pass.** This block did not exist when the story was implemented, and
`standards/rust/**` is the row that needed saying out loud. `8fa4d06` recorded
the identical amendment for `HS-S0049..HS-S0052`: those atoms carry `file:line`
citations pointing *into* `crates/happenstance-cloudflare/src/lib.rs` and now
also into `crates/happenstance-cloudflare/tests/support/mod.rs`, both of which
this story creates or rewrites, and `cargo xtask lint-constitution` is a gate
step. A diff that moves a cited line and leaves the citation stale is red; a diff
that repairs it was out of bounds. Eight such repairs were made across this
slice, every one a line-number change with the anchor string untouched.

**What this amendment does not license.** Normative text, and nothing about what
an atom of the constitution *says* — a citation repair moves `path:LINE` and
leaves the anchor and the prose byte-identical. It also does not reopen the
exclusions above: `spec/SPECIFICATION.md`, `.kb/**`, `publish = false` and the
measured ceilings all remain outside this fence.

**Fence amendment — 2026-08-20, the ADR-0023-A amendment pass.** Six rows were added. **Four
of them are one thing: an acceptance sentence cannot be amended at one grain only.** The
human gate of 2026-08-19 required the amendment to move *"this sentence and
AC-001/AC-002/AC-003 … together through the spec path, as a named decision with its
rationale"*, and the sentences that had to move sit at three grains — `initiative.md`'s
DoD 4, `project.md`'s AC-002 / AC-004 / DoD 1, and the three story specs' own ACs, ledgers
and reports. A fence that admits only one story's folder makes the ordered remedy
unperformable, which is the `8fa4d06` condition — *the fence as written was not satisfiable
by a correct implementation* — arriving one grain up. `references/adr/…0023….md` is the
fifth: its own status line promised *"when the wave runs, this line becomes `accepted`"*,
the wave has run, and a long-form record still reading `proposed` would contradict every
citation this amendment makes. `wasm-execution-gate-step/spec.md` is the sixth and the smallest: it quotes initiative DoD 4 **verbatim** in its *Advances DoD scenario* bullet, as this slice's two specs do, and a verbatim quote of an amended sentence left stale is the two-sources-disagreeing defect this whole pass exists to remove. One line, the quote only; no AC, boundary or verdict of HS-S0048 is touched.

**What this amendment does not license.** Any change to `spec/SPECIFICATION.md`, to a
maturity marker, or to the body of an accepted decision atom — none is touched. It does not
license widening an AC to fit an artefact: the widening it carries is the one an **accepted
decision** ratified, taken in a dedicated pass, with the prior refusal to reword the bar
(`af9eb10`) preserved in the record above rather than deleted.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A Durable Object host exists and is the only way a rule reaches storage | A real DO class plus the entrypoint the wasm32 runner addresses. Placement (a `cfg`-gated module, `examples/`, or a `tests/` support module) is the implementer's; reachability from a *second* compilation unit is not, because the slice's conformance target is one | Architecture §4b (`.bklg/…/cloudflare-durable-object-store/_decomposition.md:292-300`); `crates/happenstance-cloudflare/Cargo.toml:19-25` (why none exists today) |
| The store is constructed by injection, once, through one constructor | The host takes `sql` off the object's own state and hands it to `CloudflareEventStore::new(sql)`. The fixture never builds a store another way, and the documented production wiring uses the same call | `crates/happenstance-cloudflare/src/event_store.rs:70-87`; Testing §3 (`_decomposition.md:620-625`) |
| `migrate()` runs once per fixture **instance**, not per `connect()` | `migrate()` is the schema seam. Running it per handle would make schema creation a per-connection effect and would hide a fixture that hands out handles onto different objects | `crates/happenstance-cloudflare/src/event_store.rs:79-86`; Architecture §4a (`_decomposition.md:282-290`) |
| `impl Fixture for CloudflareFixture` with `type Store = CloudflareEventStore` | Bound on the **bare** `EventStore`, the weaker flavour. No `Send` bound is introduced anywhere on the fixture path; the handle owns a refcount (`Rc`-shaped), never a lifetime, and never a GAT | `crates/happenstance-testkit/src/contract.rs:120-125`, `:76-111`; `crates/happenstance-cloudflare/src/lib.rs:33-61` (why `Rc`) |
| One fixture instance = one Durable Object's storage; two instances share nothing | A fresh instance is a fresh object id / namespace entry. Not a shared object that is cleared between instances — clearing is observably different from isolation the moment two instances are alive at once | `rules::two_fixture_instances_observe_none_of_each_others_appends` (`crates/happenstance-testkit/src/suite.rs:210`); `contract.rs:13-23`; Testing §5 (`_decomposition.md:668-681`) |
| `connect()` = one more handle onto **that** object | A second stub, or a clone of the storage handle whose aliasing `SqlStorage: Clone` already models. Async because a real fixture acquires its handle over I/O; it panics rather than returning `Result`, because a fixture that cannot connect is a broken test environment, not a non-conformant adapter | `crates/happenstance-cloudflare/src/sql_storage.rs:135-147`; `contract.rs:309-321`; `fixtures.rs:286-291` (the reference shape) |
| `SECOND_HANDLE = Capability::SUPPORTED` | A MUST, not a trade. The rule uses `must!` and **panics** on a declined value, quoting the fixture's own reason, so declining it to turn a red rule green fails louder rather than quieter | CF-16 `[FROZEN]` (`spec/SPECIFICATION.md:7548-7568`); `crates/happenstance-testkit/src/suite.rs:265-271`; `contract.rs:135-161` |
| `REOPEN` is answered deliberately, with this runtime's own words | Storage outlives the isolate, so handle state can be discarded and the store read again; the isolate cannot be restarted from inside a test. Supported if the host permits discarding handles, declined with *that exact reason* if not. The verdict against the real runner is `measured-store-limits`' | `contract.rs:163-173`; CF-17 `[PROVISIONAL]` (`spec/SPECIFICATION.md:7570-7586`); `_storymap.md` (`measured-store-limits` row) |
| `MID_BATCH_FAULT` is restated explicitly, never inherited by omission | The default is a decline written for a store with no fault to inject; a Durable Object *can* arm one (a `CHECK` constraint or trigger armed for one write), so silently inheriting the default would put a sentence in the CI log that is not about this store. Whether to claim it is `measured-store-limits`' | `contract.rs:207-211`, `:281-307`; CF-39 `[PROVISIONAL]` (`spec/SPECIFICATION.md:7631`); Architecture §4d (`_decomposition.md:361-366`) |
| The three CF-40 limit constants are declared deliberately, with the hand-off named | Stating a number promises exactly that many bytes accepted and one more refused as `ExceedsStoreLimit`; a guessed number fails the rule in one direction or the other. This story states the shape and names `measured-store-limits` as the owner of the values — it invents no number | `contract.rs:214-279`; CF-40 `[PROVISIONAL]` (`spec/SPECIFICATION.md:7661`); `_storymap.md` AC-008 split |
| A capability declared **supported** has its method overridden | Nothing in the trait ties `REOPEN: SUPPORTED` to overriding `reopen`, or `MID_BATCH_FAULT: SUPPORTED` to overriding `arm_mid_batch_fault`; the provided bodies panic, and the reachable path is *declare supported, forget override, run the suite* | `contract.rs:290-307`, `:330-352` |
| A capability declared **declined** names a real, runtime-specific, non-empty reason | `Capability::declined("")` is rejected in a `const fn`, but for an associated const that fires at **codegen** — `cargo build`/`cargo test` catch it, `cargo clippy` does not. The reason is printed on every run and is the only record of the trade: write *why this runtime cannot*, not *that it cannot* | `contract.rs:374-419`; CF-18 `[FROZEN]` (`spec/SPECIFICATION.md:7597`) |
| A declined rule still runs and its `SKIP` line reaches a human | Never `#[cfg]` a rule out. Under `__emit_wasm` the line goes through `RuleOutcome::skip_line` into `console_log!`, because `report`'s `println!` is a **measured** no-op on `wasm32-unknown-unknown` | `contract.rs:458-537`; `crates/happenstance-testkit/src/lib.rs:55-82` |
| The fixture is consumable by the shipped macro, unchanged | The expression this story supplies is the `fixture = …` argument of the general arm — `mod_name = …, emit = …, fixture = …`. No bespoke harness, no hand-copied rule list; the rule set comes from `for_each_event_store_rule!` alone | `crates/happenstance-testkit/src/lib.rs:312-357`, `:84-89`; the shape at `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27` |
| The host is a test-and-example surface, not a second public API | The adapter stays a library type any DO can hold. The visibility decision is taken in the crate root's re-export block and stated in the crate documentation; an item that became `pub` in passing is a semver promise nobody made | `crates/happenstance-cloudflare/src/lib.rs:128-135`; Architecture §4b (`_decomposition.md:296-300`) |
| The concurrency family is not invoked, and that is stated rather than absent | `event_store_concurrency_conformance!` binds `F::Store: EventStore + Send` and its module is `#[cfg(not(target_arch = "wasm32"))]`; a `!Send` adapter cannot invoke it and is not expected to. This story's fixture documentation says so; *emitting* the reasoned non-invocation in the run is `every-rule-under-workerd`'s | `crates/happenstance-testkit/src/lib.rs:101-110`, `:169-173`; `project.md` AC-003 |
| The standing `!Send` detectors stay green | `CloudflareEventStore`, `SqlRowStream` and `CloudflareEventStoreError` remain `!Send`; `send_flavour::SendStoreWithLocalError` still compiles; `the_probe_is_not_vacuous` still passes. A real `JsValue` is `Send + Sync` on non-`atomics` `wasm32`, so `Send`-ness is restorable by accident — keep the fixture path `Rc`-shaped | `crates/happenstance-cloudflare/src/lib.rs:137-259`, `:33-61`; `_storymap.md`, Standing detectors 1–2 |

## Data and migrations

**No schema change is owned here, and one migration *policy* is.**

The `event` / `event_tag` tables and ADR-0014's identity columns (`origin_store`,
`origin_position`) are landed by `durable-object-write-path`, which merges before
this story (`crates/happenstance-cloudflare/src/event_store.rs:8-28`, `:187-195`;
`.kb/decisions/0014-event-identity-and-recorded-time.md`). This story adds no
column, no table and no DDL.

What it does decide is **when the schema seam runs and who owns the store-id
incarnation across a fixture's lifetime**, because those are fixture-shaped
questions and getting them wrong is invisible until a specific rule fails:

- `migrate()` is invoked **once per fixture instance**, at or before the first
  `connect()`, and is idempotent (`CREATE TABLE IF NOT EXISTS`). Invoking it per
  handle would make schema creation a per-connection effect and would mask a
  fixture whose two "handles" are actually two objects.
- ADR-0014 governs store-id incarnation: mint once, re-mint only on a detectable
  restore or clone. The fixture-side consequence is that **`connect()` must not
  mint a new incarnation** — a second handle onto one object is the same store,
  and a fixture that re-minted per handle would change what
  `contains_event_id` answers for the store's own events. A fresh *fixture
  instance*, being a fresh object, does get a fresh incarnation; that is the
  isolation the suite checks.
- If `REOPEN` is declared supported, the reopen path must discard handle state
  **without** discarding what was durably committed, and without re-minting the
  store id — otherwise `reopened_store_does_not_reissue_an_event_id`
  (`crates/happenstance-testkit/src/suite.rs:2310`) and
  `recorded_time_survives_a_reopen` (`:2473`) are answering a question about the
  fixture rather than about the adapter.

There is no data migration, no backfill and no rollback step: nothing published
depends on this crate (`crates/happenstance-cloudflare/Cargo.toml:12`,
`publish = false`, removed later by `publish-ready-crate`), and every store this
story creates is a throwaway object created by a test and never read again.

## Acceptance criteria

Framed from the two personas this story serves — the **adapter author**, whose
journey is *learn when you are finished* (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`;
`initiative.md:210-214`, `:245-246`), and the **gate reader**, who must be able to
see what ran and what did not (`_storymap.md`, Backbone rows C and D). Each
criterion crosses the full stack this story owns: host → constructor → fixture →
the shipped macro's expectations.

One new integration target, `crates/happenstance-cloudflare/tests/fixture_contract.rs`,
carries the verifying tests. It is an integration target on purpose: it is a
*second compilation unit*, which is the only place the host's reachability
constraint can actually be observed, and it is the same boundary the slice-mate's
conformance target sits on. Where `worker`'s linking forces it (Testing brief §2,
`_decomposition.md:587-616`), a test named below is emitted as
`#[wasm_bindgen_test]` under `#![cfg(target_arch = "wasm32")]` rather than as a
plain `#[test]`; the test **name** is the contract, its host target is not.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The adapter author has something to hang the store off, and it is reachable from where a suite lives.** GIVEN a `happenstance-cloudflare` whose read and write paths are complete but which no Durable Object hosts, WHEN the adapter author compiles the crate's test targets, THEN a Durable Object host class and the entrypoint the wasm32 runner addresses exist, are declared in a module list the real build compiles (`crates/happenstance-cloudflare/src/lib.rs`), and are **nameable from a second compilation unit** — not a bare `#[cfg(test)]` module in `src/lib.rs`, which an integration test cannot reach | `crates/happenstance-cloudflare/tests/fixture_contract.rs::the_host_is_reachable_from_an_integration_test` — an integration target that names the host and drives one append through it. Compiling at all is half the proof; the assertion is that the host answers | 
| AC-002 | **The fixture the author hands to the macro is bound on the weaker flavour, and stays `!Send`.** GIVEN the only `!Send` adapter in the workspace, WHEN `CloudflareFixture` implements `Fixture` with `type Store = CloudflareEventStore`, THEN the bound is the bare `EventStore` (never `SendEventStore`), only one of the two trait names is in scope per module, `Store` is an owned associated type rather than a GAT, every handle the fixture holds is `Rc`-shaped, and the four standing `!Send` probes including `the_probe_is_not_vacuous` still pass | `crates/happenstance-cloudflare/src/lib.rs::not_send_probe` (`:180-259`) unchanged and green, plus `crates/happenstance-cloudflare/tests/fixture_contract.rs::the_fixture_store_is_not_send` — a negative-bound probe over `<CloudflareFixture as Fixture>::Store` in the same shape as the existing module, whose positive control makes it non-vacuous |
| AC-003 | **Two fixture instances share nothing, so the suite's isolation rule can bite.** GIVEN the one adapter mistake no upstream test can observe — every fresh fixture instance quietly pointed at the same Durable Object storage (`_decomposition.md:668-681`) — WHEN two `CloudflareFixture` instances are alive at the same time and each appends, THEN neither instance's store reads any event the other appended, because a fresh instance is a fresh object id / namespace entry and **not** a shared object cleared in between | `crates/happenstance-cloudflare/tests/fixture_contract.rs::two_instances_alive_at_once_observe_none_of_each_others_appends` — both instances constructed *before* either appends, which is what distinguishes isolation from clearing. The shared-suite counterpart is `rules::two_fixture_instances_observe_none_of_each_others_appends` (`crates/happenstance-testkit/src/suite.rs:210`), run by the slice-mate |
| AC-004 | **`connect()` gives a second handle onto one object, and the schema seam runs once per object.** GIVEN CF-16 `[FROZEN]` (`spec/SPECIFICATION.md:7548-7568`), whose rule uses `must!` and fails rather than skips on a declined value, WHEN the adapter author calls `connect()` twice on one fixture instance, THEN `SECOND_HANDLE` is `Capability::SUPPORTED`, each handle observes the other's appends, `migrate()` has run exactly once for that instance, and no second store-id incarnation was minted for the second handle (ADR-0014) | `crates/happenstance-cloudflare/tests/fixture_contract.rs::two_handles_from_one_instance_observe_each_others_appends`, `::migrate_runs_once_per_instance_not_per_connect`, and `::a_second_handle_does_not_re_mint_the_store_id` — the last asserting `contains_event_id` still answers *own* for an event appended through the first handle |
| AC-005 | **The gate reader is told what this runtime cannot do, and why it cannot.** GIVEN that a `#[cfg]`-ed-out rule is indistinguishable from a passing one in the emitted binary (`crates/happenstance-testkit/src/contract.rs:26-42`, CF-18 `[FROZEN]` at `spec/SPECIFICATION.md:7597`), WHEN any `Capability` on this fixture is declined, THEN its reason is non-empty and names *why this runtime cannot* rather than *that it cannot*, no rule is `#[cfg]`-ed out of this crate, and the reason survives into a line a human reads — `RuleOutcome::skip_line` into `console_log!` under `__emit_wasm`, because `report`'s `println!` is a measured no-op on `wasm32-unknown-unknown` | `crates/happenstance-cloudflare/tests/fixture_contract.rs::every_declined_capability_names_this_runtime` — walking `SECOND_HANDLE`, `REOPEN` and `MID_BATCH_FAULT` through `Capability::reason()` (`contract.rs:430`), asserting each `Some` reason is non-empty and mentions the Durable Object / isolate / storage vocabulary rather than a generic refusal. The associated-const codegen check is that the target **builds** at all (`contract.rs:400-419`) — `cargo clippy` does not catch this, `cargo test` does |
| AC-006 | **Nothing this fixture claims is claimed without the method behind it.** GIVEN that the trait ties no `SUPPORTED` constant to its override and the provided bodies **panic** (`contract.rs:290-307`, `:330-352`), and that the reachable author path is *declare supported, forget the override, run the suite*, WHEN `REOPEN` or `MID_BATCH_FAULT` is declared `SUPPORTED`, THEN `reopen()` / `arm_mid_batch_fault()` is overridden with a real seam; and WHEN either is declined, THEN it is **restated explicitly in this impl** with this runtime's own words rather than inherited from the trait default, so no sentence written for a different store appears in this run's log | `crates/happenstance-cloudflare/tests/fixture_contract.rs::a_supported_capability_has_its_method_overridden` — calling the method for each capability the fixture declares supported and asserting it does not panic with the provided body's message; and `::mid_batch_fault_is_restated_not_inherited`, asserting the declared reason differs from `contract.rs:207-211`'s default text |
| AC-007 | **The shipped macro can consume this fixture unchanged, with its limits stated deliberately and its hand-offs named.** GIVEN that a limit is a fact rather than a trade and a guessed number fails `append_reports_exceeded_store_limits` in one direction or the other (`contract.rs:214-279`, CF-40 `[PROVISIONAL]` at `spec/SPECIFICATION.md:7661`), WHEN the adapter author writes the five-line `event_store_conformance!` invocation, THEN `fixture = CloudflareFixture::new()` type-checks against the general arm unchanged, `MAX_EVENT_DATA_LEN` / `MAX_TAGS_PER_EVENT` / `MAX_EVENTS_PER_BATCH` are each **written out** in this impl — never inherited by omission — with `measured-store-limits` named as the owner of the values, and the concurrency family's non-invocation is documented with its reason rather than left as an absence | `crates/happenstance-cloudflare/tests/fixture_contract.rs::the_fixture_expression_satisfies_the_macro_arm` — a compile-level check that binds `CloudflareFixture::new()` behind the same `F: Fixture` bound the macro's arm imposes (`crates/happenstance-testkit/src/lib.rs:312-357`), plus `::the_three_store_limits_are_stated_here`, asserting each constant is written in this impl. The prose obligations (owner named, concurrency family reasoned) are read in `cargo doc` output and at review |

**Coverage of the traced project ACs.** Project **AC-003** (*every declined
capability reports the fixture's stated reason*) — this story owns the *declares*
half (`_storymap.md`, Coverage) and discharges it through **AC-005** and
**AC-006**; the *emits* half is `every-rule-under-workerd`'s. Project **AC-008**
(*CF-39 and CF-40 discharged*) — this story owns the *fixture's shape* half and
discharges it through **AC-006** (CF-39's `MID_BATCH_FAULT` restated deliberately)
and **AC-007** (CF-40's three constants stated deliberately, hand-off named); the
measured numbers are `measured-store-limits`', the atom resolution is
`adr-0023-and-atom-resolutions`'.

## Interaction quality

**The composition family is `N/A` for this story, and that is a signed-off
determination rather than a skip.** `_design.md` (approved by the repository owner
on 2026-08-12 at the `/redkiln:plan` design sign-off gate) records
`N/A — no user-facing surface` for every one of its ten sections, and states the
finding it rests on: the only "surfaces" anywhere in this project's briefs are Rust
module boundaries, and its mounts are *a constructor, a Durable Object host class,
a test target and a `Fixture` impl — code, not screens*
(`.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md:12-18`,
`:53-57`, `:106-111`). There is therefore no presentation to compose, no placement
to decide, no transience policy, no density budget and no named visual
anti-pattern. Inventing one here would contradict a design a human has already
approved, which this story is forbidden to do. `design.capture` stays a **declared**
skip (`CLAUDE.md`, *Where the work lives*).

**The state family survives in one form, and it is load-bearing.** The gate
reader's only channel is the run's own output, so the analogue of *non-occlusion*
— the guarantee that a thing which happened is visible, and that a thing which did
not happen is distinguishable from a thing which passed — applies exactly, and it
is the reason CF-18 is frozen. It is carried by criteria in the table above, not by
bullets here:

| Invariant (state family) | Carried by | How it is verified |
| --- | --- | --- |
| **Non-occlusion** — a guarantee this runtime cannot supply is *visible as a named line*, never an absence. No rule is `#[cfg]`-ed out; a declined capability still emits a test | **AC-005** | `::every_declined_capability_names_this_runtime`, plus the structural fact that the rule set comes only from `for_each_event_store_rule!` (`crates/happenstance-testkit/src/lib.rs:84-89`) |
| **Legibility of the declined state** — the reason reaches a human on *this* target, where `println!` writes nowhere. The skip travels through `RuleOutcome::skip_line` into `console_log!` | **AC-005** | `contract.rs:500-531` is the path; the assertion is on the reason's content, and the emitter half is the slice-mate's to exercise |
| **No silent success** — a capability claimed but not implemented must fail loudly rather than quietly. The provided bodies panic; a `must!` capability panics rather than skipping | **AC-004**, **AC-006** | `::a_supported_capability_has_its_method_overridden`; `SECOND_HANDLE = SUPPORTED` under a `must!` rule (`suite.rs:265-271`) |
| **Reversibility** — every value this story states is one the slice-mate can overturn without a redesign: the three limit constants and the two capability verdicts are single-line facts with a named owner, not shapes other code depends on | **AC-006**, **AC-007** | `::the_three_store_limits_are_stated_here` ensures they are *stated in this impl*, which is what makes `measured-store-limits`' replacement a one-line edit rather than an archaeology exercise |
| **Preserved context across a second handle** — a second handle must not reset what the store knows about itself (its incarnation), the analogue of a re-render that loses selection | **AC-004** | `::a_second_handle_does_not_re_mint_the_store_id` |

Keyboard reachability, focus/scroll preservation, in-place-versus-context-jump and
transience have no referent in a crate with no rendered surface, and are recorded
here as `N/A` rather than omitted, per `_design.md`'s own discipline of stating a
skip instead of taking one.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | A `Capability::declined("")` is written for an **associated** const | Rejected — but the rejection arrives at *codegen*, so `cargo check` and `cargo clippy` pass and only `cargo build` / `cargo test` catch it (`crates/happenstance-testkit/src/contract.rs:400-419`). The merge gate must therefore include a step that **builds or runs** the fixture, not merely checks it. Note the standing gap: the existing `wasm32 build of the Cloudflare adapter` step is a `cargo check` **without** `--tests` (`xtask/src/main.rs:245-264`), so it would not catch this |
| **EC-002** | A capability is declared `SUPPORTED` and its method is not overridden | The provided body panics with a message naming both possibilities — *either declare it declined, or override the method* (`contract.rs:297-307`, `:339-352`). AC-006's test must reach the panic, not merely read the constant, or the whole failure mode is untested |
| **EC-003** | `SECOND_HANDLE` is declined to make a red rule green | Fails, loudly: `two_handles_observe_each_others_appends` uses `must!` and panics quoting the fixture's own reason (`suite.rs:265-271`). This is the intended behaviour, not a defect to work around — CF-16 was rewritten to close exactly this escape route |
| **EC-004** | `connect()` cannot acquire a handle (no host, no binding, no object) | Panic. `connect()` returns `Self::Store`, not `Result` (`contract.rs:309-321`), because a fixture that cannot connect is a broken *test environment*, not a non-conformant adapter, and reporting it as a rule failure would mislabel it. The panic message must name the host and the missing binding |
| **EC-005** | The host is placed where the slice-mate cannot name it — a bare `#[cfg(test)]` module in `src/lib.rs` | A compile error in the slice-mate's target, discovered after this story's own gate is green. AC-001 exists to move that discovery earlier: the reachability test lives in an integration target *in this PR* |
| **EC-006** | A store-limit constant is stated with a guessed number | `append_reports_exceeded_store_limits` fails in one direction or the other — either a payload the store accepts is refused, or one it refuses is accepted (`contract.rs:214-279`; CF-40, `spec/SPECIFICATION.md:7661`). This story's mitigation is not to guess: state the constant with the hand-off named, per AC-007 |
| **EC-007** | Two "handles" are actually two objects, or two "instances" are actually one object | Invisible to every rule that does not construct two of the thing in question — which is precisely why AC-003 constructs both instances *before* either appends, and why AC-004 asserts appends cross between handles rather than merely that two handles exist |
| **EC-008** | A `Send` bound is restored by accident through the fixture path | Not a compile error on `wasm32` without `atomics`, where a real `JsValue` is `Send + Sync` (`crates/happenstance-cloudflare/src/lib.rs:33-61`). The detector is the probe module, and `the_probe_is_not_vacuous` is what stops a broken probe from reporting success. If a probe goes quiet, the change is wrong even when the gate is green (`_storymap.md`, Standing detectors) |

## Non-functional

| id | requirement | why it is stated here |
| --- | --- | --- |
| **NF-001** | **No `Send` bound is introduced anywhere on the fixture path**, and no `unsafe` is used to hold a handle. The crate's `unsafe_code = "forbid"` stands | The whole project exists to exercise the `!Send` flavour end to end (`project.md`, DR-2). A fixture is exactly the kind of new code that can undo it |
| **NF-002** | **No new public API promise.** The host and the fixture's visibility are decided in the crate root's re-export block (`crates/happenstance-cloudflare/src/lib.rs:128-135`) deliberately; `publish = false` stays until `publish-ready-crate` | An item that became `pub` in passing is a semver promise nobody made, and this crate will be published (`Cargo.toml:12`) |
| **NF-003** | **No new required dependency beyond what the host needs.** `wasm-bindgen-test` goes in `[target.'cfg(target_arch = "wasm32")'.dev-dependencies]` only, mirroring `crates/happenstance-testkit/Cargo.toml`; any `worker` surface added must clear `cargo deny`'s licence/advisory graph and ADR-0029's 1.97.1 floor | The MSRV floor moved once already because of a *dependency's* build script, and five database crates in this workspace declare no `rust-version` at all (`CLAUDE.md`, binding constraint 5; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`) |
| **NF-004** | **The rule set stays a single enumeration.** This story adds no rule to the testkit and creates no `wasm`-only list anywhere in the tree | A `wasm`-only rule list fails project AC-002 by construction (`_storymap.md`, Standing detectors 4; `crates/happenstance-testkit/src/lib.rs:84-89`) |
| **NF-005** | **A fixture instance is cheap and disposable.** One instance is one throwaway object; nothing persists between test binaries and no cleanup step is owed | The whole suite constructs instances per rule; a fixture whose setup is expensive makes the conformance run's cost scale with the rule count, which is the count this project is committed to keeping complete |
| **NF-006** | **The four `!Send` probe assertions must keep passing somewhere reachable by an ordinary `cargo test`** | A probe that only runs under `workerd` is a probe an ordinary contributor's inner loop never exercises (Testing brief §2, `_decomposition.md:613-616`) |
| **NF-007** | `cargo xtask affected --base main` is green at merge, and the existing `wasm32 build of the Cloudflare adapter` step is not broken | The story grain the `verify:` block wires (`CLAUDE.md`, *Commands*) |

## Implementation notes (non-prescriptive)

These are latitude, not instruction. The architecture brief already records that the
`worker` API surface bound, the cursor iteration style and the marshalling are
ADR-0023's material rather than a brief's (`_decomposition.md:424-462`).

- **Read `MemoryFixture` before writing this one.** `crates/happenstance-testkit/src/fixtures.rs:242-292`
  is the reference implementation and shows the `Arc`-clone shape this story
  reproduces with an `Rc`. Its doc comment at `:230-242` is also the model for *how
  a declined `REOPEN` is worded* — it declines with the real reason rather than a
  generic one, which is the bar AC-005 sets.
- **Copy the invocation shape verbatim, do not invent one.**
  `crates/happenstance-testkit/tests/memory_conformance_wasm.rs:19-27` is five lines
  and is the thing the slice-mate will write against this fixture. If the fixture
  needs anything the macro's arm does not offer, that is a finding about the
  fixture, not a reason for a bespoke harness.
- **Placement of the host is genuinely open** — `#[cfg]`-gated module, `examples/`,
  or a `tests/` support module (`_decomposition.md:292-300`). The only closed
  question is reachability across a compilation-unit boundary (AC-001, EC-005). A
  `tests/support/` module declared by both this story's `fixture_contract.rs` and
  the slice-mate's conformance target satisfies it with no `cfg` gymnastics; a
  `#[cfg(feature = "…")]`-gated `pub` module in `src/` also does, at the cost of a
  feature `cargo hack`'s powerset then has to clear.
- **`MID_BATCH_FAULT` is the one capability here that would be new coverage for the
  *suite*, not just for this adapter** — a `CHECK` constraint or trigger armed for
  exactly one write is available to a Durable Object and to nothing else in the
  workspace so far (`_decomposition.md:361-366`, `:630-637`). If you claim it, note
  that `mutation_coverage.rs`'s CF-29 lint requires a changelog entry for any rule a
  fixture newly exercises for real (`xtask/src/main.rs:20-24`). If you decline it,
  decline it in *this* impl with this runtime's words — inheriting the default is
  the failure AC-006 names.
- **`REOPEN` is the one verdict most likely to be overturned by the slice-mate.**
  State the honest answer against the host you build and keep it to one line, so
  `measured-store-limits` replaces a constant rather than a design.
- **Do not settle CF-40's ownership here.** `.kb/open-questions/cf-40-fixture-limits-ownership.md`
  is contested between ADR-0015 and ADR-0012 and `sqlite-durable-store` (HS-P0012)
  merges one position ahead; whichever project reaches the answer first owns it and
  the other cites it (`_storymap.md`, *Why these milestones*; `project.md`, Risks).
  This story names the hand-off and stops.
- **The two silent failure modes are shape, not values.** A limit inherited by
  omission and a capability declared supported without its override are both
  invisible in a green build. Everything else this story decides announces itself.

## Tests and CI (merge gate)

Grounded in the Testing brief's tier table (`_decomposition.md:509-533`) and its
merge-gate section (`:649-666`). This story reaches four of the five tiers; the
conformance-run tier is the slice-mate's, and that is the point of the slice.

| tier | command / path | proves |
| --- | --- | --- |
| static — format and lint | `cargo fmt --all --check`; `cargo clippy --workspace --all-targets --all-features -- -D warnings` | House style and the absence of a new warning. **Does not** prove AC-005's non-empty reasons — an associated const's rejection is a codegen event and clippy never reaches it (`contract.rs:400-419`) |
| static — wasm32 shape | `cargo xtask wasm` (the `wasm32 build of the Cloudflare adapter` step, `xtask/src/main.rs:245-264`) | The crate still checks on its own target. Note the standing gap this story must not rely on: the step passes no `--tests`, so nothing under `tests/` is type-checked by it today (see Risks) |
| unit / integration — the fixture contract | `cargo test -p happenstance-cloudflare --test fixture_contract` (or its `wasm32` / `wasm-bindgen-test` emission where `worker`'s linking forces it, Testing brief §2) | AC-001…AC-007. This is the tier that *builds* the fixture and therefore the only one that catches an empty declined reason (EC-001) |
| unit — standing detectors | `cargo test -p happenstance-cloudflare` (`not_send_probe`, `crates/happenstance-cloudflare/src/lib.rs:180-259`; `send_shape::send_flavour`, `:87-94`) | NF-001, NF-006, EC-008 — that this story did not restore `Send`-ness. `the_probe_is_not_vacuous` is what makes the other three mean anything |
| contract — upstream unchanged | `cargo test -p happenstance-core` (the two `read`-shape tests in `crates/happenstance-core/src/memory.rs`); `cargo test -p happenstance-testkit` (`registry::no_orphan_rules`) | `CLAUDE.md` constraint 3 and NF-004 — that no rule list forked and no port shape moved |
| story grain — the merge bar | `cargo xtask affected --base main` | NF-007. The command `.redkiln/config.yaml`'s `verify:` block wires to the story grain, run whether or not anyone types it |
| project grain — after the slice | `cargo xtask ci --fast` (`REQUIRED` only — the bar a non-terminal project meets, `CLAUDE.md` *Commands*) | The slice's own bar, exercised once the slice-mates land. Recorded here because the Testing brief flags a real consequence: if the new `workerd` execution step lands in `REQUIRED`, every `--fast` run during this project needs a working runner locally (`_decomposition.md:657-666`) |
| trace — clauses stay honest | `cargo xtask spec-trace` | CF-16, CF-17, CF-18, CF-23, CF-39 and CF-40 all carry citations this story touches; the markers may not rot into decoration (`_storymap.md`, Standing detectors 5) |

**Not run by this story:** `event_store_conformance!` itself. Pointing the shipped
macro at this fixture and executing it under a real runtime is
`every-rule-under-workerd`'s, by the slice's own design — and the reason this
story's own targeted tests exist at all is that the fixture *contract* must be
provable before the suite is aimed at it.

## Risks and coupling (PR-scoped)

- **The `wasm32` gate step does not type-check `tests/`.** `xtask/src/main.rs:245-264`
  runs `cargo check … --target wasm32-unknown-unknown` with no `--tests`, so an
  integration target that does not compile on the real target is not caught by the
  existing gate at all. This story's test tier must be run explicitly; extending the
  step (or covering it with the slice's execution step) is the slice-mates' work,
  but the gap must not be *discovered* after merge. **Mitigation:** run the fixture
  target for the `wasm32` target locally before merge and record the command in the
  implementation report.
- **The unit tier's host target may move under this story's feet.** Once `worker` is
  a real dependency, `cargo test -p happenstance-cloudflare` with no `--target` may
  stop linking, which would push both the probe module and this story's tests behind
  `#[cfg(target_arch = "wasm32")]` and `wasm_bindgen_test`. The Testing brief flags
  this as open and recommends host-native absent evidence otherwise
  (`_decomposition.md:587-616`). **Mitigation:** NF-006 is the invariant that
  survives either answer — the four assertions must remain reachable by an ordinary
  `cargo test`. Decide it once, in this PR, and say which way it fell.
- **Accidental restoration of `Send`.** A real `JsValue` is `Send + Sync` on
  non-`atomics` `wasm32` builds (`crates/happenstance-cloudflare/src/lib.rs:33-61`),
  so a fixture that holds a binding directly instead of behind an `Rc<str>`-shaped
  handle can undo the project's central property without any diagnostic.
  **Mitigation:** EC-008's probe, and keeping the whole fixture path `Rc`-shaped.
- **Isolation implemented as clearing.** A fixture that reuses one Durable Object and
  clears it between instances passes any test that constructs instances
  sequentially, and fails only when two are alive at once. **Mitigation:** AC-003
  constructs both before either appends. This is the failure mode the Testing brief
  names as the only genuinely new one this project can introduce
  (`_decomposition.md:668-681`).
- **Slice coupling is real and deliberate.** This story's output is unobservable by
  the conformance suite until `every-rule-under-workerd` merges, and its declared
  numbers are provisional until `measured-store-limits` does. That is the slice
  contract, not drift — but it means a defect in the fixture's *shape* that this
  story's own tests do not catch surfaces two stories later, in a diff that looks
  like it is about something else. **Mitigation:** the seven ACs are all about
  shape, and each names a wrong implementation it rejects.
- **CF-40's answer must not be minted twice.** `sqlite-durable-store` (HS-P0012)
  merges one position ahead and its open-question atom names phase 8 as the forcing
  phase (`.kb/open-questions/cf-40-fixture-limits-ownership.md`). **Mitigation:**
  this story writes no `.kb/` file at all and names the hand-off in prose only.
- **`REOPEN` answered too confidently.** Declaring it `SUPPORTED` obliges an override
  that discards handle state without discarding what was committed and without
  re-minting the store id; getting that subtly wrong makes
  `reopened_store_does_not_reissue_an_event_id` (`suite.rs:2310`) and
  `recorded_time_survives_a_reopen` (`:2473`) answer a question about the fixture
  rather than about the adapter. **Mitigation:** if the override is not genuinely
  right, decline with the real reason — a declined `REOPEN` is a `SHOULD` and skips
  legibly, unlike `SECOND_HANDLE`.
- **No coupling to the ES-6 work.** `caller-visible-error-verdict` touches the same
  crate but a different file and a different question; neither blocks the other, and
  neither may absorb the other's scope.

## Dependencies

**Blocks on** (must merge first — both are `_storymap.md`'s `depends_on` for this
row, and both are in the preceding milestone `real-worker-bindings`):

- `durable-object-write-path` — `migrate`, `append`, `head` and `contains_event_id`
  against the real bindings, plus ADR-0014's store-id incarnation and the identity
  columns. Without it the fixture has nothing to `migrate()` and no append for
  AC-003 or AC-004 to observe.
- `durable-object-read-path` — `read` as ADR-0011's ceiling-and-page. Without it
  AC-003's and AC-004's assertions cannot *read back* what was appended, which is
  the half that makes isolation and handle-sharing observable at all.

Both depend transitively on `worker-binding-layer`, which is what makes
`SqlStorage`'s aliasing `Clone` a real binding rather than a stand-in.

**Unlocks** (both are slice-mates in `durable-object-conformance-run`):

- `every-rule-under-workerd` — declares `durable-object-host-and-fixture` in its own
  `depends_on` and is where project AC-002 and AC-004 close. It supplies the
  `fixture = …` expression this story makes available.
- `measured-store-limits` — replaces this story's declared limit constants and the
  `REOPEN` / `MID_BATCH_FAULT` verdicts with measured ones.

Transitively downstream: `wf-11-memory-ceiling-falsifier`,
`deferral-re-reads-and-es-32-verdict`, `adr-0023-and-atom-resolutions` and
`publish-ready-crate`, all of which reach this story through
`every-rule-under-workerd` (`_storymap.md`, Merge order 3–5).

## Anchors (progressive disclosure)

Everything load-bearing is stated in the Context pack. These are the artefacts to
open when you reach a specific decision — link, do not paste. Every path was
confirmed to exist in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-testkit/src/contract.rs` | The `Fixture` trait, `Capability`, `RuleOutcome` and the doc comments that explain *why* each shape is what it is — the GAT/ICE reasoning at `:76-111`, the `must!`-vs-`require!` split at `:135-161`, the store limits at `:214-279`, the panicking provided bodies at `:290-352`, and the codegen-timing note at `:400-419`. This story implements this file | Before writing `impl Fixture` — the first thing opened, and re-opened at each capability constant | AC-002, AC-004, AC-005, AC-006, AC-007 |
| `crates/happenstance-testkit/src/fixtures.rs` | `MemoryFixture` at `:242-292` is the reference implementation the repository points every adapter author at, including the `Arc`-clone shape this story reproduces with an `Rc` and the wording of an honest declined `REOPEN` at `:230-242` | Immediately before writing `CloudflareFixture` — read it, then write | AC-002, AC-004, AC-005 |
| `crates/happenstance-cloudflare/src/event_store.rs` | `CloudflareEventStore::new(sql)` and `migrate()` at `:70-87` are the one construction seam; `:187-195` carries ADR-0014's identity columns that decide what `connect()` must not re-mint | Before the host wires storage into the store, and again when deciding where `migrate()` is called | AC-001, AC-004 |
| `crates/happenstance-cloudflare/src/sql_storage.rs` | `SqlStorage`'s `Clone` at `:135-147` and its comment that cloning **aliases the same storage** — the mechanism `connect()` is built on rather than an accident. `:13-17` records the non-snapshot cursor caveat | When implementing `connect()`, and before assuming a second handle needs a second acquisition | AC-004 |
| `crates/happenstance-cloudflare/src/lib.rs` | The mount point: the module list, the re-export block at `:128-135` where visibility is decided, the four stand-in findings at `:33-95` (including why `JsHandle` holds an `Rc<str>`), and the probe module at `:180-259` | At mount time, and again before adding any type that holds a binding | AC-001, AC-002 |
| `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` | The five-line invocation at `:19-27` the slice-mate will write against this fixture, and the `#![cfg(target_arch = "wasm32")]` / `wasm_bindgen_test` shape a `wasm32`-emitted test in this story takes | Before choosing where the host lives, and before deciding the test target's cfg attributes | AC-001, AC-007 |
| `crates/happenstance-testkit/src/lib.rs` | The macro arms at `:312-357` (the `fixture = …` expression's exact obligation), `__emit_wasm` at `:55-82`, the single rule enumeration at `:84-89`, and the concurrency family's `Send` bound and `cfg` at `:101-110`, `:169-173` | When writing the compile-level macro-arm check, and when writing the documented non-invocation of the concurrency family | AC-007, NF-004 |
| `crates/happenstance-testkit/src/suite.rs` | The rules that observe this story's mappings: `:210` (instance isolation), `:265-271` (two handles, `must!`), `:332`, `:2310`, `:2473` (the `REOPEN` family), `:2706`, `:2794` (the `MID_BATCH_FAULT` family) | When deciding a capability verdict — open the rule that will run on that verdict before choosing it | AC-003, AC-004, AC-006 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` | Architecture §4 at `:277-401` — the composition roots, the host's test-and-example status at `:292-300`, and the full fixture mapping at `:341-375`. Testing §3 at `:618-647` — the seams; §5 at `:668-681` — the one failure mode nothing upstream can catch; §2 at `:587-616` — the unit tier's open target question | Before AC-001's placement decision, and before AC-003's isolation implementation | AC-001, AC-003, AC-004, AC-006, AC-007 |
| `spec/SPECIFICATION.md` | The clause text this story discharges or half-discharges: CF-16 `[FROZEN]` at `:7548-7568`, CF-17 `[PROVISIONAL]` at `:7570-7586`, CF-18 `[FROZEN]` at `:7597`, CF-39 at `:7631`, CF-40 at `:7661`, CF-23 `[FROZEN]` at `:7920-7951` | When a capability verdict feels like a trade — the clause says whether it is one. **Never edit a `[FROZEN]` clause**; that is a new atom and a re-plan | AC-004, AC-005, AC-006, AC-007 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted atom behind CF-1…CF-29, the fixture shape, and how rules are emitted for runtimes that are not tokio — the authority above any brief when the two disagree | Before disagreeing with anything the testkit's docs say about fixture shape | AC-005, AC-007 |
| `.kb/decisions/0014-event-identity-and-recorded-time.md` | Store-id incarnation: mint once, re-mint only on a detectable restore or clone. The fixture-side consequence is that `connect()` must not mint a new one | Before implementing `connect()` and before writing `::a_second_handle_does_not_re_mint_the_store_id` | AC-004 |
| `.kb/decisions/0001-async-port-flavours.md` | Why the bare `EventStore` flavour exists and why this target is the reason. **Accepted and immutable** — `redkiln validate --kb` checks it against `HEAD`, so any evidence this story produces is cited *from* a future ADR-0023, never by editing this file | Before touching any trait bound on the fixture path, and any time an edit to this atom is contemplated | AC-002 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The contested ownership of the fixture-limits question, and the reason this story may not answer it: `sqlite-durable-store` merges one position ahead and the atom names phase 8 as forcing | Before writing anything about who owns the limit values — read it, name the hand-off, write no `.kb/` file | AC-007 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_design.md` | The signed-off no-surface determination (`:106-111`), which is what makes the composition family of interaction quality `N/A` rather than skipped | Before inventing any rendered surface, and when writing the implementation report's design section | AC-001 (Interaction quality) |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_storymap.md` | The slice contract at `:52-54` (this row and its two slice-mates), the coverage split at `:114`, `:119` (which half of project AC-003 and AC-008 is this story's), and the standing detectors at `:171-189` | When the boundary between this story and a slice-mate is unclear | AC-005, AC-006, AC-007 |
| `xtask/src/main.rs` | The gate's step list and, specifically, `:245-264` — the `wasm32` check that passes no `--tests`, which is why this story's integration target is not covered by the existing gate | When deciding how to prove the fixture target compiles on `wasm32` before merge | NF-007, EC-001 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The adapter author's *learn when you are finished* journey, the source the acceptance criteria are framed from. No `.kb/product/` atom exists yet — this is the citable form, and promoting it is closeout's | When a criterion needs to be restated in terms of a person rather than a capability | AC-001, AC-005 |

## Clarifications resolved during spec

1. **The seven AC ids are exactly the front half's.** AC-001…AC-007 as recorded in
   the first pass; none added, none dropped. They partition into host (AC-001),
   fixture shape (AC-002), the two mappings the suite can actually catch (AC-003,
   AC-004), the two capability-honesty obligations (AC-005, AC-006) and
   consumability plus the limits' shape (AC-007).
2. **The Interaction quality composition family is `N/A`, and is stated rather than
   skipped.** `_design.md` is signed off with no user-facing surface. Rather than
   leave the section empty — which reads identically to an omission — the state
   family's surviving analogue (non-occlusion of a declined guarantee, no silent
   success, reversibility of a stated verdict) is mapped onto AC ids that already
   exist in the table. No invariant was invented and no new AC row was added for it,
   because every one of them was already an obligation this story carried.
3. **The verifying tests live in one new integration target.**
   `crates/happenstance-cloudflare/tests/fixture_contract.rs`, chosen because
   AC-001's reachability constraint can only be observed from a second compilation
   unit and because it puts this story's tests on the same boundary the slice-mate's
   conformance target will sit on. Whether it emits plain `#[test]`s or
   `#[wasm_bindgen_test]`s is left to the implementer per Testing brief §2; the test
   *names* are the contract.
4. **`REOPEN` and `MID_BATCH_FAULT` verdicts are this story's to state and the
   slice-mate's to confirm.** The split is `_storymap.md`'s AC-008 split — *the
   fixture's shape* here, *the measured numbers* there. This story therefore writes
   both constants explicitly with honest reasons and does not treat a later reversal
   as a defect.
5. **No number is invented for the three CF-40 constants.** AC-007 requires them
   **stated** in this impl with the owner of the values named; it does not require
   them to be right, because being right is a measurement
   `measured-store-limits` owns. Stating `None` deliberately, with a reason, is an
   acceptable answer to AC-007; inheriting `None` by omission is not.
6. **No `.kb/` write occurs in this story.** CF-40's ownership, ADR-0023 and WF-11's
   atom are `adr-0023-and-atom-resolutions`', authored through `/redkiln:kb-ingest`.
   This story names hand-offs in prose only.
7. **The `--tests` gap in the existing `wasm32` step is recorded, not fixed.**
   `xtask/src/main.rs:245-264` checks this crate without `--tests`, so this story's
   integration target is outside the existing gate. Extending the step is the
   slice-mates' work; this story's obligation is to run the target explicitly and
   say so in the implementation report, so the gap is never *discovered* later.
