---
item: HS-S0053
stage: discover
created: 2026-08-12T13:02:15.374Z
updated: 2026-08-12T13:02:15.374Z
template_sig: 86ce4036
rendered_sig: b941146a
---

# Discover — A Durable Object host and the CloudflareFixture mounted on it

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: land the Durable Object host (a test-and-example surface, not a second public API) and `impl Fixture for CloudflareFixture` — one instance is one object's storage, each `connect()` one handle onto it, `SECOND_HANDLE` `SUPPORTED`, every declined capability carrying a real non-empty reason — all reaching the store through the one `CloudflareEventStore::new(sql)` constructor | `_storymap.md`, **Slices**, `durable-object-host-and-fixture` row | The host and the fixture are one story on purpose: a fixture with nothing to mount on is the "build it / wire it in" split the map is required not to make |
| AC-003 (every declined capability reports the fixture's stated reason) and AC-008 (CF-39/CF-40 discharged) — this story owns the *declaring* half of both | `project.md`, **Acceptance criteria**; `_storymap.md`, **Coverage** | The run *emits* the reasons and the numbers are measured elsewhere; the fixture is where they are stated |
| `dependsOn: durable-object-write-path, durable-object-read-path` — together they supply a store with no `todo!()` on any path the fixture drives | `_storymap.md`, **Slices** | A fixture over a `todo!()` store panics at the first rule instead of reporting, so this edge is not bureaucratic |
| ARCH-AC-05: one `impl Fixture` in which a fixture instance is one Durable Object's storage and each `connect()` is one handle onto it; `SECOND_HANDLE` `SUPPORTED`; `REOPEN`, `MID_BATCH_FAULT` and the three `Option<usize>` ceilings each carrying a real answer | `_decomposition.md`, Architecture brief, **Acceptance Criteria** | The mapping *is* the design decision, and the testing brief inherits it |
| `SECOND_HANDLE` is a MUST, and the rule requiring it **panics** on a declined value rather than reporting a skip — because declining it is not a trade, it is a fixture that does not meet the contract | `crates/happenstance-testkit/src/contract.rs:135-161`, `:145` | The one capability with no honest decline. The type cannot tell a MUST from a trade, which is why the panic is in `suite.rs` |
| `REOPEN`'s own documentation names a Durable Object as the motivating case: storage outlives the isolate, so handle state can be discarded and the store read again, while restarting the isolate from inside a test is not possible | `crates/happenstance-testkit/src/contract.rs:163-173` | This runtime is the one the capability was written for, so its answer here is evidence rather than housekeeping |
| `MID_BATCH_FAULT` defaults to declined, and the provided `arm_mid_batch_fault` **panics** with a message naming both ways to be wrong | `crates/happenstance-testkit/src/contract.rs:207-211`, `:294-307` | A `SUPPORTED` constant with an empty arm turns `append_is_atomic_under_a_mid_batch_fault` into a vacuous green, and the suite says so at `suite.rs:2820-2831` |
| `Capability::declined("")` is rejected — but the assertion is an **associated** const, so it fires at codegen rather than at `cargo check` or `cargo clippy` | `crates/happenstance-testkit/src/contract.rs:400-418` | A build-and-run catches it; the static half of the gate does not. On a target whose run step is still being built, that is a real window |
| Two fixture instances sharing nothing is itself a conformance rule, and pointing every instance at one backing store is named as the *adapter's* mistake no test over the testkit's own fixture could see | `crates/happenstance-testkit/src/contract.rs:17-23`; `_decomposition.md`, Testing brief Notes §5 | The genuinely new failure mode this story must not commit, and the rule that catches it already exists |
| A declined capability is still emitted as a test and reports the fixture's stated reason; `#[cfg]`-ing it out would make a skipped rule indistinguishable in CI output from a passing one | `crates/happenstance-testkit/src/lib.rs:46-50`; `crates/happenstance-testkit/src/contract.rs:473-483` | BR-13's whole content, and this project is its first real exerciser against a runtime that genuinely cannot do some of what the suite asks |
| The host is a test-and-example surface, not a second public API, and the fixture and the documented production wiring must reach the store through the *same* `CloudflareEventStore::new(sql)` constructor | `_decomposition.md`, Architecture brief §4a and §4b; `crates/happenstance-cloudflare/src/event_store.rs:75-87` | Otherwise a test verifies a shape production does not use |
| `capability_skips_are_reported` proves the *answering* half of CF-18 only; the *emission* half is checkable only from outside the process, via `cargo test -- --list`, and "belongs to an `xtask` step, not to a `#[test]`" | `crates/happenstance-testkit/tests/mutation_coverage.rs:3167-3183` | The join with `wasm-execution-gate-step`'s `Artefact` row, and it should be planned rather than discovered |
| `registered_second_handle` is a standing guard on future registrations: it fires the day someone registers an instrument that declines the MUST | `crates/happenstance-testkit/tests/mutation_coverage.rs:2174`, `:3206-3216` | Confirms the MUST is enforced in two places, neither of which runs in an adapter's own CI |

## Questions

**`REOPEN`: supported or declined?** Deferred to **spec**, because the answer
depends on the host this story is still choosing. `contract.rs:163-173` splits the
case for us: discarding a *handle* and reading the store again is what the
capability asks for, and restarting the *isolate* from inside a test is what is
impossible — those are not the same thing, and a runner that lets a stub be
dropped and re-obtained supports the capability. Whichever way it falls, the reason
is the fixture's own words and it is the input `deferral-re-reads-and-es-32-verdict`
reads for CF-14.

**`MID_BATCH_FAULT`: an honest `SUPPORTED`, or declined with a reason?** Deferred
to **spec**, jointly with `measured-store-limits`. A Durable Object *can* arm one
— a `CHECK` constraint or trigger armed for exactly one write — and doing so would
make this the first adapter in the workspace to exercise CF-39 for real
(`_decomposition.md`, Architecture brief §4d). The cost is named and is not
symmetric: a `SUPPORTED` constant with an empty or fake arm is worse than an
honest decline, because it converts a rule into a vacuous green
(`crates/happenstance-testkit/src/suite.rs:2820-2831`).

**The three numeric ceilings.** This story wires the constants; it does **not**
choose their values. They are measurements taken against the real runtime under
the conformance run, and they belong to `measured-store-limits`
(`crates/happenstance-testkit/src/contract.rs:214-279`). What this story must not
do is leave them at their `None` default and call that a declaration — for a
runtime with a documented storage cap, `None` is a false statement, not a neutral
one.

**Where does the host live — `#[cfg(test)]`, `examples/`, or a `tests/` support
module?** Deferred to **spec**; the architecture brief calls it an implementation
choice explicitly (`_decomposition.md`, Architecture brief §4b). What is *not* a
choice: the fixture and the documented production wiring reach the store through
the same constructor.

**CF-39 / CF-40's fixture-limits ownership.** Not settled here. The numbers are
`measured-store-limits`'; the ownership atom is `adr-0023-and-atom-resolutions`',
coordinated with `sqlite-durable-store` (HS-P0012) so it is minted once
(`.kb/open-questions/cf-40-fixture-limits-ownership.md`).

**The off-tokio harness shape.** Not this story's, but it constrains it: the
fixture must be constructible from inside whatever runtime
`every-rule-under-workerd` selects, which means it takes its `SqlStorage` from the
host rather than constructing one.

## Decision

The conformance suite is handed `impl AsyncFn() -> F`, not a made store, precisely
so a rule that can make two isolated stores can check that they are isolated — and
that only means anything if there is something real to make. This slice builds
both ends: a Durable Object host to hang the store off (nothing in the tree
provides one, because `worker` was deliberately absent), and the `CloudflareFixture`
that maps the suite's vocabulary onto it — one instance is one object's storage,
each `connect()` is one handle onto that storage, `SECOND_HANDLE` is `SUPPORTED`
because it is a MUST, and every capability this runtime declines carries a reason
in the fixture's own words. It is also this project's first contact with BR-13 as
something other than policy: this is the first adapter in the workspace facing a
runtime that genuinely cannot do some of what the suite asks, so "a declined
guarantee is reported with a stated reason, never by silent absence" stops being a
principle and becomes a diff. The spec will cover: the host's shape and where it
lives; the fixture's construction and its one-instance-one-object invariant; each
`Capability` constant with the exact reason string it declines with, if it does;
the three numeric ceilings wired but not yet valued; and the constructor discipline
that keeps the fixture and production on the same path. Nothing `[FROZEN]` is
amended.

## The wrong implementation

**The fixture that is one Durable Object wearing many hats.** `CloudflareFixture::new()`
returning a handle onto one fixed object — `DurableObjectId::from_name("conformance")`,
a `thread_local!` the host hands out, or simply a namespace entry created once —
so two fixture instances share storage. Most of the suite passes: every rule that
seeds and then reads within one instance is unaffected, the append and query
families are untouched, and the run is green. Exactly one thing in the tree can
see it, and the testing brief names it as the failure mode nothing upstream can
catch: the two-instance isolation rule at
`crates/happenstance-testkit/src/contract.rs:17-23`. It is worse than a plain
failure because it is order-dependent — a rule that appends and then counts is
fine until another rule runs before it — so it presents as flakiness rather than
as a defect. No new rule is owed for it; the obligation is to *run* the existing
one for real, which is this project's whole point.

**And the one this project is the first in a position to commit: a declined
capability whose reason never reaches anyone.** `Capability::declined("")` is
rejected (`crates/happenstance-testkit/src/contract.rs:412-418`) — but the
assertion sits in an **associated** const, and the file says exactly what that
costs: an associated const is evaluated lazily, at codegen, so `cargo check` and
`cargo clippy -- -D warnings` both pass it and only a build-and-run catches it
(`contract.rs:400-411`). On a target whose run step is the thing this project is
still building, that is a live window. One step further out is the mutant
`crates/happenstance-testkit/src/lib.rs:46-50` exists to forbid: `#[cfg]`-ing a
rule invocation out rather than declining its capability, which removes the rule
from the binary entirely and makes it indistinguishable in CI output from a rule
that passed.

**Where the detectors live.** The isolation mutant needs nothing new — the rule is
in the shared suite and the testing brief already concludes no new rule is owed
(`_decomposition.md`, Testing brief Notes §5). The empty-reason and
`#[cfg]`-ed-out mutants are **not** conformance-rule failures and must not be
added to `crates/happenstance-testkit/tests/mutation_coverage.rs`, whose rows are
stores that fail named rules; that file states the correct home itself, in the
documentation of `capability_skips_are_reported`: the emission half "is only
checkable from outside the process, via `cargo test -- --list`. That belongs to an
`xtask` step, not to a `#[test]`" (`mutation_coverage.rs:3167-3183`). That `xtask`
step is the `Artefact` row `wasm-execution-gate-step` builds, and the join between
the two slices should be named in this story's spec rather than found later.

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

**Box 6.** No conformance rule is added here. The fixture declares capabilities
and hands the suite stores to run against; it asserts nothing itself, and in
particular nothing about positions — which matters because a fixture that seeded a
known position layout would be the beginning of exactly the literal-position
assumption `CLAUDE.md` forbids, and the store this fixture wraps is free to leave
gaps.

**Box 7.** Nothing `[FROZEN]` is changed. The capability contract this story
implements against — `SECOND_HANDLE` as a MUST, `REOPEN` as a SHOULD,
`MID_BATCH_FAULT` and the three ceilings — is honoured as written. If this
runtime could not supply `SECOND_HANDLE`, the rule panics by design and the
correct response is a new decision atom and a re-plan, not a fixture that declines
a MUST.

**Box 8.** No conformance rule here seems wrong. The one design that looks like a
gap and is not — `Capability::declined("")`'s assertion firing at codegen rather
than at `check` — is documented at `contract.rs:400-411` with its reason, and this
story's answer is to make sure the fixture is built and run rather than to move the
assertion.
