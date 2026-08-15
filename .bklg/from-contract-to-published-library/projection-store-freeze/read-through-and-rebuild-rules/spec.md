---
item: HS-S0012
stage: spec
created: 2026-08-12T13:46:06.709Z
updated: 2026-08-12T13:46:06.709Z
template_sig: 87bbf1d0
rendered_sig: bc7a68bb
---

# Spec — Read-your-writes, chunk-size invariance and rebuild authority

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project charter | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/read-through-and-rebuild-rules/spec.md` |
| This story's discover | `.bklg/from-contract-to-published-library/projection-store-freeze/read-through-and-rebuild-rules/discover.md` |
| Key briefs | `.bklg/…/projection-store-freeze/_decomposition.md` — Architecture **AC-A02** (`:299-302`), **AC-A06** (`:317-319`), **AC-A07** (`:320-325`), the mount-point table (`:340-360`), Note 4's `READS_THROUGH_BATCH` gate (`:476-487`), Note 5's fixture contract (`:517-544`), Note 6's rule skeleton and the increment (`:546-577`), Note 9's "never assert literal positions" (`:705-707`), Note 10 item 1 (`:713-715`); UX **AC-U08 – AC-U11** (`:159-195`); Testing brief tiers (`:769-786`) |
| Signed-off design | `.bklg/…/projection-store-freeze/_design.md` — `surfaces: []`, no user-facing surface, approved 2026-08-12 |
| Story map row | `.bklg/…/projection-store-freeze/_storymap.md:64` (slice `reset-and-rebuild-rules`, merge order 5); AC-005 split at `:83`, AC-003 split at `:81` |
| Specification (wins on conflict) | `spec/SPECIFICATION.md:5052-5074` (PS-12 + CF-18's gate), `:5075-5085` (PS-13), `:5086-5098` (PS-14), `:5330-5352` (PS-24), `:5658-5691` (§4.11's rule table, the three hostile stores, CF-5's variant), `:4632-4719` (§4.0's target trait) |
| Cases | `spec/E2E-CASES.md:554-575` (E2E-21), `:577-596` (E2E-22), `:654-675` (E2E-25) |
| Roadmap pointer | `RUNBOOK.md:3904-3921` — phase 6's read-your-writes and rebuild work; phase 6 in full at `RUNBOOK.md:3848-3965` |

## One-line PR slice

`batch_reads_reflect_pending_writes` gated on
`ProjectionProbe::READS_THROUGH_BATCH` and emitted as a **reported skip** when an
adapter declares `false` (CF-18), `rebuild_is_chunk_size_invariant` replaying a
fixed probe sequence at chunk sizes 1, 3 and whole-log, and
`rebuilding_is_distinguishable_from_live`, each with the store that fails it —
including the batch `get` that answers from committed state.

## Executive summary

Three of `ProjectionStore`'s guarantees are about **reading**, and all three are
unobservable in the tree today: whether a batch can see its own pending writes
(PS-12), whether a rebuild produces the same read model however it is chunked
(PS-13 / PS-14, both `[FROZEN]`), and whether a store rebuilding a projection is
distinguishable from one serving it live (PS-24). This PR writes those three
rules, registers a store that fails each, and — the half that makes the
initiative's AC-05 real rather than machinery — supplies the **first genuine
declined capability in the projection suite**, `READS_THROUGH_BATCH = false`,
emitted as a reported skip naming the constant an author can change.

**Pointer, then delta.** The suite's shape, its enumeration, its emitters, the
`Declared` registry and the exactness meta-tests already exist and are not
re-argued here: `projection-suite-entry-point` (HS-S0007) landed
`for_each_projection_store_rule!` and the three harnesses,
`projection-mutant-registry` (HS-S0009) landed the projection `REGISTRY` and its
three meta-tests, and `projection-capability-skips` (HS-S0008) landed the skip
machinery and the projection capability set. What is new here is *content in
those slots*:

- **three rules** in the projection rules module, each added to the single
  enumeration (an unenumerated rule fails the orphan meta-test, which is what
  "mounted" means for a rule);
- **three mutants** — `CommittedReadBatchStore`, `FirstWriteWinsBatchStore`,
  `LiveOnlyCheckpointStore` — with registry rows whose `fails` sets are exact
  and whose provenance names the real adapter shape that makes each plausible;
- **one conformant variant**, `NoBatchReadStore`, whose only distinguishing
  property is that it declares `READS_THROUGH_BATCH = false` and therefore
  exercises the skip arm of two rules that would otherwise be dead code.

**What this PR does not settle.** No `[FROZEN]` clause is edited: PS-13 and PS-14
are frozen and this story *implements the rules they state*
(`spec/SPECIFICATION.md:5077,5087`). No maturity marker moves and
`spec/SPECIFICATION.md` is not touched — that is
`unstable-projection-gate-and-clause-disposition`'s. No verdict is made about
whether the port is frozen, and none about the `unstable-projection` exposure.
The full replay-at-commit second batch shape is `buffering-conformant-variant`'s
(HS-S0011), and this story deliberately does not pre-empt it (D6).

## Context pack

Read this section before writing code. Everything deeper is a signposted anchor.

### D1 — Three rules, one gate, and the gate is a **probe** const, not a fixture const

The three rules and what each rejects are pre-specified by §4.11's table
(`spec/SPECIFICATION.md:5674-5676`) and must not be re-derived:

| Rule | Clause(s) | The implementation it rejects |
| --- | --- | --- |
| `batch_reads_reflect_pending_writes` | PS-12 `[PROVISIONAL]` | a batch `get` answering from committed state |
| `rebuild_is_chunk_size_invariant` | PS-13, PS-14 — both `[FROZEN]` | a projection reading out of band |
| `rebuilding_is_distinguishable_from_live` | PS-24 `[PROVISIONAL]` | rebuild in place with one position field |

PS-12's gate is `ProjectionProbe::READS_THROUGH_BATCH`
(`spec/SPECIFICATION.md:5004`, `:5059-5065`) — a const on the **store's** probe
impl, reached from a rule through the fixture's `Store` associated type. It is
*not* a `ProjectionFixture` associated const, and
`projection-api-design-record` records it that way by name
(`.bklg/…/projection-api-design-record/spec.md:301`). Two consequences:

- `suite.rs`'s existing `require!` macro cannot serve it: it expands to
  `<$fixture as Fixture>::$capability.reason()` and stringifies the const's own
  identifier (`crates/happenstance-testkit/src/suite.rs:36-46`). The projection
  family needs the sibling that reads
  `<F::Store as ProjectionProbe>::READS_THROUGH_BATCH` and reports the
  capability name `ProjectionProbe::READS_THROUGH_BATCH`.
- The capability name is therefore a **string const**, not a stringified
  identifier — and that is precedent, not invention: `NO_STORE_LIMITS` is a
  public `&'static str` naming the constants a reader must go and change,
  written that way because *"a skip naming only one of them would send an
  adapter author to look for the constant they did set"*
  (`crates/happenstance-testkit/src/contract.rs:435-442`). Reuse the const
  `projection-capability-skips` landed if it exists; otherwise add one mirroring
  `NO_STORE_LIMITS`. **Do not fork the skip vocabulary** (Architecture AC-A06,
  UX AC-U08): `RuleOutcome::Skipped` is reused unchanged, one line shape, one
  format.

If the rules cannot reach `ProjectionProbe` from `F::Store` because the fixture
trait bound only `ProjectionStore`, the minimum repair is a `where F::Store:
ProjectionProbe` clause at these three rule definitions — the emitters call
rules by path and add no bounds of their own, so this costs the enumeration
nothing. Prefer whatever binding `projection-suite-entry-point` actually landed;
report a mismatch at the slice boundary rather than widening the fixture trait
here.

### D2 — **Both** read-path rules gate on `READS_THROUGH_BATCH`. This is a decision, and here is the alternative that lost

`rebuild_is_chunk_size_invariant`'s prescribed shape defines the probe's write as
**an increment of what the batch can see**
(`spec/SPECIFICATION.md:5088-5093`; `_decomposition.md:574-577`). The increment
is read through `probe_read_through`, which exists only when
`READS_THROUGH_BATCH` is `true` and *"may be `unimplemented!()` otherwise"*
(`spec/SPECIFICATION.md:5011-5012`). So the chunk-size rule is gated on the same
const as PS-12's rule, and a declining fixture reports **two** skips.

The alternative — running a *blind* write sequence for declining adapters so the
rule reports `Ran` — was considered and rejected: blind writes are chunk-size
invariant by construction, so that arm is a rule no adapter can fail, which is
`CLAUDE.md`'s first corollary arriving through the back door. Calling a skip a
skip is the honest option CF-18 exists to make cheap.

The residual is real and must be **stated, not hidden**: chunk-size invariance is
unverified for a write-behind adapter, because such an adapter has no read path
for a projection's read-modify-write to use in the first place — which is exactly
what §4.4 says goes wrong (`spec/SPECIFICATION.md:5032-5051`) and exactly what
PS-12's falsifier, owned by the Ladybug phase, is about (`:5055-5058`). Write
that sentence into the rule's own doc comment so the next reader does not mistake
a skip for coverage.

### D3 — The fixed sequence and the increment: this is the rule that most looks correct while checking nothing

The rule replays one fixed sequence three times, against **three isolated
stores** — `open()` is called once per run, because rules take
`impl AsyncFn() -> F`, "not a fixture, but how to make one"
(`crates/happenstance-testkit/src/suite.rs:1-13`; `_decomposition.md:541-544`).

Each step of the sequence is a read-modify-write **through the batch**:

```text
value = probe_read_through(&batch, key).unwrap_or(0) + 1
probe_write(&mut batch, key, value)
```

The sequence must contain at least one key written **twice inside one chunk** at
chunk size 3 and once per chunk at chunk size 1, or the arithmetic cannot
diverge. A sequence that satisfies that: `a, a, b, a, b, a` — correct final state
`a = 4, b = 2` at every chunking, while a store answering reads from committed
state produces `a = 2` at chunk size 3 and `a = 1` at whole-log.

Three sizes, and why exactly these (discover Q3, settled here): **1** is the
degenerate case where pending and committed state coincide, so it is the run a
broken store gets right; **3** is the smallest size that puts a repeated key both
inside one chunk and across a boundary; **whole-log** is the case a rebuild
actually runs at scale, and the one where a committed-state read is maximally
wrong. The trap named in discover is the *plain `set`*: last-writer-wins is
chunk-insensitive by construction, so a rule written that way passes on a store
that cannot read through its own batch and certifies PS-13 and PS-14 on nothing.

The comparison at the end is `probe_read(key)` per key, per run — an **out-of-band
read by the suite**, after commit. That does not violate PS-13, which constrains
what a *projection's* `apply` may read while it is writing
(`spec/SPECIFICATION.md:5075-5081`); the rule is the reader of record here, not
the projection.

### D4 — Never assert a literal position; the invariant is over the **read model**

The commits in these rules carry positions the rule chooses (there is no event
store in the loop), and the discipline still binds: positions are **supplied,
never asserted** (`CLAUDE.md`, *The rule that matters*; Architecture Note 9,
`_decomposition.md:705-707`). Use a deliberately **non-contiguous** position
sequence, so a later edit that grows a `position == index + 1` assumption is
visible; the specification permits gaps and `GappedPositionStore` exists in this
same test tree to convict a rule that forgets
(`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:110-125`).
Positions must be strictly increasing across chunks, or the rule trips
`commit_rejects_a_regressing_position`'s clause (PS-22,
`spec/SPECIFICATION.md:5297-5316`) for a reason that has nothing to do with what
it is testing.

`rebuild_is_chunk_size_invariant` compares **read models between runs**, never
checkpoints and never positions. `rebuilding_is_distinguishable_from_live`
compares a `Checkpoint` **variant**, never the `through` value it carries.

### D5 — Rebuild authority is expressible through `commit` alone. There is no runner here

`rebuilding_is_distinguishable_from_live` is: reset, commit two chunks with
`Authority::Rebuilding` asserting `Checkpoint::Rebuilding` after each, then commit
a third with `Authority::Live` and assert `Checkpoint::Live`
(`spec/SPECIFICATION.md:5339-5343`). Everything it needs is on the port's own
signature — `commit(batch, id, position, authority)` and
`checkpoint(id) -> Checkpoint` (`spec/SPECIFICATION.md:4696-4714`). This answers
discover Q4: **no rebuild driver, no runner.** §4.11 puts the six runner-dependent
rules in the workspace e2e crate under CF-36
(`spec/SPECIFICATION.md:5693-5703`) and they belong to
`typed-layer-and-alpha-release`; a rule here that needed a runner would be in the
wrong crate.

Two details the specification fixes and this story must not soften:

- **A rebuild that has committed nothing reads `NeverRun`, not `Rebuilding`**, and
  that is correct: both mean the rows are not authoritative and the reader's
  decision is the same (`spec/SPECIFICATION.md:5348-5352`). Do not assert
  `Rebuilding` immediately after the reset.
- The rule uses `reset` as a **step**, not as a subject. `reset` is part of §4.0's
  trait, landed by `owned-batch-port-shape`; the *rules* that interrogate it are
  the slice-mate `reset-rules`'s. The two stories are independent within the slice
  in either order (`_storymap.md:111`).

### D6 — Both arms of the gate need a fixture, or one arm is dead code. This story builds the declining one

This is discover Q1 and Q2, settled here:

- **`MemoryProjectionStore` declares `true`.** It applies on write and really
  answers a pending read; its own spec states that flipping it to `false` would be
  a finding for this story rather than a quiet convenience
  (`.bklg/…/memory-projection-store/spec.md:411`). This story consumes that; it
  does not change it.
- **The `false` arm is a new instrument built here**: `NoBatchReadStore` and its
  fixture, registered as a **conformant variant** (`Kind::ConformantVariant`,
  `fails: &[]`) beside `GappedPositionFixture` in
  `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:225-247`.
  Building an instrument to keep a rule non-vacuous is this repository's
  established move (`_decomposition.md:420-426`;
  `crates/happenstance-testkit/src/fixtures.rs:234-241`).

**Its defect-free difference is the whole point**: it exposes *no read path on the
batch at all*, which PS-12 explicitly permits as the second arm of its `MUST`
(`spec/SPECIFICATION.md:5052-5054`), so `probe_read_through` is
`unimplemented!()` — the specification's own spelling, and enforceable rather than
stylistic, since this workspace denies `clippy::todo` and does not lint
`unimplemented`. It is **not** the buffering replay-at-commit shape: that shape is
`buffering-conformant-variant`'s deliverable and the far end of §6's batch-shape
axis (`spec/SPECIFICATION.md:5686-5691`). Keeping this instrument minimal is what
stops this story from silently claiming AC-004's second batch shape one slice
early. If the buffering variant also declares `false` when it lands, that is a
second instance and not a duplicate to delete — **the skip arm must never end up
with zero fixtures behind it.**

The named hazard, from discover: an adapter declaring `false` because the rule is
inconvenient. CF-18 is still satisfied — the rule is emitted, the skip reported,
nothing vanishes from the binary — and yet if *every* fixture declares `false`,
both rule bodies are code no test executes and their mutants are never run. The
structural guard is the one above: one fixture on each arm, in the same run.

### D7 — Three mutants, declared exactly. The registry is the arbiter, not the author's confidence

`Declared` rows carry `name`, `kind`, the **exact** `fails` set, non-empty
`provenance` and optional per-rule `expect` pins naming the assertion the mutant
is expected to trip (`crates/happenstance-testkit/tests/mutation_coverage.rs:141-186`).
Three exactness meta-tests hold the two lists together — every rule has a mutant,
the registry is exhaustive, and each mutant fails **exactly** its declared rules
(`_decomposition.md:680-683`; ADR-0010, `.kb/decisions/0010-the-suite-must-prove-itself.md`).
The rows this story adds:

| Store | Kind | Fails exactly | Provenance (the shape that makes it plausible) |
| --- | --- | --- | --- |
| `CommittedReadBatchStore` | mutant | `batch_reads_reflect_pending_writes`, `rebuild_is_chunk_size_invariant` | the batch `get` that answers from committed state — the *natural* shape, because one round trip on the same connection is what you want, and it is the one that silently loses a write (`spec/SPECIFICATION.md:5069-5074`; E2E-21) |
| `FirstWriteWinsBatchStore` | mutant | `rebuild_is_chunk_size_invariant` | a batch that stages writes into a map with `entry().or_insert(…)`, keeping the **first** value staged for a key rather than the last — the natural spelling when the batch is thought of as a dedup buffer. Its reads through the batch are honest, so it passes PS-12's rule and fails only the chunked replay |
| `LiveOnlyCheckpointStore` | mutant | `rebuilding_is_distinguishable_from_live` | rebuild in place with a single position field — "the obvious reading of the port", and what `Option<SequencePosition>` permitted before `Checkpoint` existed (`spec/SPECIFICATION.md:5344-5352`; E2E-25) |
| `NoBatchReadStore` | conformant variant | — (`fails: &[]`) | an adapter that buffers its writes and offers no read path on the open batch, declining PS-12's capability honestly rather than answering from committed state |

Two disciplines that apply to all four rows. First, `LiveOnlyCheckpointStore`
must preserve `NeverRun` for an id it has never seen — its defect is exactly that
`Authority` is ignored and `Rebuilding` is unrepresentable — or it also fails
`fresh_projection_has_no_checkpoint` and its declaration becomes false. Second,
if any mutant turns out to fail a rule it does not declare, the repair is to
**narrow the defect or widen the declaration**, decided and written down; it is
never a reason to relax the exactness meta-test. And per ADR-0010, **no pass rate
is ever quoted** over this table.

### D8 — What "mounted" means for a conformance rule

A library has no render tree; the architecture brief fixes the composition root
as the places an item is either reachable or invisible (`_decomposition.md:339-346`).
For a rule that is four places, and three of them fail loudly if missed:

1. the **projection rules module** — the rule is a `pub async fn` taking
   `impl AsyncFn() -> F` and returning `RuleOutcome`;
2. the **single enumeration** `for_each_projection_store_rule!` in
   `crates/happenstance-testkit/src/registry.rs` — a rule in the module but not in
   the enumeration is caught by the orphan meta-test, which is a textual scan of
   the rules source (`registry.rs:346-366`, `:410-436`; project AC-001);
3. the **three harnesses** — tokio, blocking and `#![cfg(target_arch = "wasm32")]`
   — which the enumeration drives through the existing emitters, and where the
   wasm one is type-checked by the mandatory `cargo xtask ci` step
   (`xtask/src/main.rs:231-240`). This is the whole of AC-016's mechanism: a new
   rule costs a harness *nothing*, and that is the point;
4. the **mutant registry** row plus the `for_each_mutant!` entry — the third edit
   the registry's own doc names (`mutation_coverage.rs:187-196`).

An item at one and not the others is a rule nobody runs, or a store nobody drives.

### D9 — The text surface: what a run says, and the assertion that actually checks it

The one line a human ever sees from this story is the skip:

```text
SKIP batch_reads_reflect_pending_writes: fixture declines `ProjectionProbe::READS_THROUGH_BATCH` — <the reason>
```

produced by `RuleOutcome::skip_line`, reused unchanged
(`crates/happenstance-testkit/src/contract.rs:500-507`; UX AC-U08). Three
obligations follow, and they are the reason this story owns AC-005's *real*
instance rather than its machinery (`_storymap.md:83`):

- **Assert on values, not stdout** (UX AC-U09). `report` writes to stdout, which
  libtest suppresses for a passing test unless `--show-output` is passed — the
  port's own doc says plainly this *"makes the line reachable by a human; it does
  not make anyone read it"* (`contract.rs:515-523`). The machine-checked half is a
  `RuleOutcome` value assertion, the shape
  `mutation_coverage::capability_skips_are_reported` already uses
  (`mutation_coverage.rs:3160-3184`).
- **Name the switch the author set** (UX AC-U10): the `capability` field carries
  `ProjectionProbe::READS_THROUGH_BATCH`, so a reader is sent to the constant they
  can change — not to a fixture const that does not exist.
- **The reason survives wasm32** (UX AC-U11): `report` is a measured no-op on
  `wasm32-unknown-unknown`, so the wasm harness routes `skip_line` through
  `console_log!` the way `__emit_wasm` already does (`contract.rs:525-531`). If
  the projection wasm harness landed by `projection-suite-entry-point` does not,
  that is a defect this story is the first to be able to observe — report it.

A declined capability must carry a **non-empty** reason; `Capability::declined`
is a `const fn` `assert!`, and on an *associated* const it fires at codegen, so
`cargo build`/`cargo test` catch an empty reason and `cargo check`/`clippy` do
not (`contract.rs:400-419`; UX AC-U12).

### D10 — The persona-journey slice

`_design.md` records **no user-facing surface** (`surfaces: []`, approved
2026-08-12), and this story renders none. The user is **P2, the adapter author**,
whose goal is *"an executable definition of 'correct' they can run against their
own storage system, rather than a prose specification they have to interpret"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`),
and whose fear is *"that the port quietly assumed something their storage cannot
provide, discovered late"* — the objection Marten's author acted on when he
rejected storage-agnosticism outright. This story is the answer to that fear in
its exact shape: an adapter whose batch cannot read its own pending writes is not
told "you fail", it is told *which constant declares the trade* and gets a green
suite with two reasoned skips. **P3, the local-first / edge developer**
(`…/personas-and-journeys.md:182-189`) is served by the same three rules running
under the wasm32 emitter in the same gate run — and by the skip reason surviving
that target, which is the one place the text surface is weakest.

### D11 — The reshape triggers. Report them; do not absorb them

Architecture Note 10 (`_decomposition.md:709-727`), narrowed to what can fire
here:

1. **A projection rule that observes the read model without the probe.** These are
   the read-side rules, so if it exists it shows up here first. PS-11 would be
   over-built and the seam should shrink — *an ADR paragraph, not a quiet
   deletion*.
2. **Chunk invariance passing on every store the tree has.** If neither mutant
   diverges, the sequence or the increment is wrong before the rule is right;
   re-derive the arithmetic in D3 rather than weakening the assertion.
3. **A pairing defect in PS-13 / PS-14.** Any such defect was found by
   `ps-clause-pairing-sweep` and repaired by the atom accepted in
   `projection-decision-atoms`, in slice 1. Meeting a *new* one here is a report
   at the slice boundary and never a line edit to a `[FROZEN]` clause
   (`CLAUDE.md`, *Open questions*).

## Integration contract

| Field | Value |
| ----- | ----- |
| **Archetype** | `capability` — a user-observable slice: after this PR an adapter author invoking one macro gets three more named rules, and a declining adapter gets two reasoned skips instead of silence. |
| **Slice / milestone** | `reset-and-rebuild-rules`. Slice-mate implemented in the same context and mounted as one integrated surface: `reset-rules` (independent of this story; either order within the slice, `_storymap.md:111`). |
| **Mount point** | `crates/happenstance-testkit/src/registry.rs` — the single enumeration `for_each_projection_store_rule!`. A projection rule that is not in it is not emitted by any harness and is caught by the orphan meta-test (`registry.rs:346-366`, `:410-436`). Paired mounts, all four required together (D8): the projection rules module the entry point created, the three harnesses (tokio / blocking / `#![cfg(target_arch = "wasm32")]`), and `crates/happenstance-testkit/tests/mutation_coverage.rs`'s projection `REGISTRY` + `for_each_mutant!`. |
| **Wires into** | `crates/happenstance-core/src/projection.rs` — `ProjectionStore::{begin, commit, checkpoint, reset}`, `Checkpoint`, `Authority`, `ProjectionId` (§4.0's shape, landed by `owned-batch-port-shape`). `ProjectionProbe` behind `happenstance-core`'s `conformance` feature — `READS_THROUGH_BATCH`, `probe_write`, `probe_read`, `probe_read_through` (`projection-probe-conformance-feature`). `crates/happenstance-testkit/src/contract.rs` — `Capability` and `RuleOutcome` **reused unchanged** (AC-A06, `_decomposition.md:317-319`), plus the `NO_STORE_LIMITS`-style capability-name const. The `ProjectionFixture` trait and `projection_store_conformance!` (`projection-suite-entry-point`); the projection `Declared`/`REGISTRY` shape and its three exactness meta-tests (`projection-mutant-registry`); the skip machinery and capability set (`projection-capability-skips`). `MemoryProjectionStore` as the `READS_THROUGH_BATCH = true` fixture. No borrowing GAT anywhere in a fixture (`contract.rs:97-111`, the minimised rustc ICE). |
| **Renders surfaces** | **none.** `_design.md` declares `surfaces: []` and answers `N/A — no user-facing surface` to `## Items`, `## Signatures` and `## The doctest`; the perceptual review is a *declared* skip (`design.capture` absent from `.redkiln/config.yaml`, per `CLAUDE.md`). The binding sources for this story's shapes are therefore `spec/SPECIFICATION.md` §4.4 / §4.7 / §4.11 and the text surface `_design.md:33-38` names. |
| **Conformance rule(s)** | Adds `batch_reads_reflect_pending_writes` (PS-12), `rebuild_is_chunk_size_invariant` (PS-13, PS-14) and `rebuilding_is_distinguishable_from_live` (PS-24) to `for_each_projection_store_rule!`, each with a registered store that fails it (D7). Re-asserts, does not change, the projection registry's three exactness meta-tests. |
| **Clause(s)** | Discharges **PS-12** (`spec/SPECIFICATION.md:5052-5074`), **PS-13** (`:5075-5085`), **PS-14** (`:5086-5098`), **PS-24** (`:5330-5352`) and CF-18's projection instance (`:7597-7605`). **No clause text, maturity marker or rule citation is edited** — PS-13 and PS-14 are `[FROZEN]` and are implemented, not amended; the marker sweep is `unstable-projection-gate-and-clause-disposition`'s. `cargo xtask spec-trace` must be **green**, with its `BEGIN/END GENERATED` §7.1–§7.2 region **regenerated rather than left stale** — that region is machine-authored and a separate gate step fails on a stale one, so "unaffected" was never available to a story that adds a rule name (amended 2026-08-15; see AC-008). |
| **Advances DoD scenario** | Initiative **DoD 7** — *"The projection suite discriminates… a deliberately wrong implementation… fails it, by name"* (`initiative.md:377-380`): this story adds three of the discriminating rules and the three stores that fail them. It also carries initiative **AC-05** (*"told, with a reason, where a guarantee does not apply"*, `initiative.md:320-322`) from machinery to a real instance, and contributes to **DoD 13** (`cargo xtask ci` green on the assembled whole, `initiative.md:396-397`) via the wasm32 harness step, which it must not regress. |

## PR boundary

**In this PR**

- Three rules in the projection rules module, with rustdoc naming the clause,
  the rejected implementation and (for the two gated ones) what a skip does
  *not* cover — D2's residual sentence.
- Three entries in `for_each_projection_store_rule!`
  (`crates/happenstance-testkit/src/registry.rs`).
- The `READS_THROUGH_BATCH` gate helper — a `require!`-shaped macro or equivalent
  reading `<F::Store as ProjectionProbe>::READS_THROUGH_BATCH` — and the
  capability-name const if `projection-capability-skips` did not already land one.
- `CommittedReadBatchStore`, `FirstWriteWinsBatchStore`, `LiveOnlyCheckpointStore`
  in `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`, and
  `NoBatchReadStore` + its fixture in `…/variants.rs`; their `for_each_mutant!`
  entries and their `Declared` rows in
  `crates/happenstance-testkit/tests/mutation_coverage.rs`.
- The `RuleOutcome`-value assertion covering the two skips (D9), beside the
  existing `capability_skips_are_reported`.
- This story's own backlog folder (ledger, report).

**Explicitly not in this PR**

- `reset_clears_rows_and_checkpoint_together`, `reset_is_scoped_to_one_projection`,
  `refused_reset_changes_nothing`, `fresh_projection_has_no_checkpoint`,
  `reset_is_not_commit_at_first` and `TruncatingResetStore` — the slice-mate
  `reset-rules`. This story *calls* `reset`; it does not interrogate it.
- The buffering replay-at-commit conformant variant and the two-unlike-shapes
  claim (AC-004 / DoD 2) — `buffering-conformant-variant` (HS-S0011). D6 is
  deliberate: `NoBatchReadStore` is a minimal declining instrument and must not
  be presented as the second batch shape.
- The PS-3 finding — `ps3-batch-shape-finding`.
- Any **authored** edit to `spec/SPECIFICATION.md`, any maturity marker, the
  `unstable-projection` gate — `unstable-projection-gate-and-clause-disposition`.
  The `BEGIN/END GENERATED` region of §7.1–§7.2 is the one exception and is **in**
  this PR, because it is machine-authored by `cargo xtask spec-trace --write` and
  a gate step fails on a stale one; the boundary block and AC-008 both scope it
  (amended 2026-08-15).
- Any change to `ProjectionProbe`, `ProjectionStore`, `Checkpoint`, `Authority` or
  `MemoryProjectionStore`, including flipping `READS_THROUGH_BATCH` — slice 2. A
  rule that cannot be written against the landed port is a **finding**, not a
  licence to edit the port.
- Any change to `Capability`, `RuleOutcome`, the three emitters, the `Declared`
  shape or any event-store rule, fixture or mutant. Touching one of those is a
  change to the event-store suite and must be justified as such
  (`_decomposition.md:362-370`).
- Any ADR. ADR-0017/0018/0019 landed in slice 1 and are a precondition; an ADR
  written as a side effect of this story is a process violation.

**Merge DoD one-liner** — `cargo xtask ci` is green whole; each of the three new
rules is emitted by all three harnesses; each has a registered store that fails
it by name under the exactness meta-tests; and one run shows
`MemoryProjectionStore` passing both gated rules while `NoBatchReadStore` reports
both as skips carrying its stated reason.

```
crates/happenstance-testkit/src/**
crates/happenstance-testkit/tests/**
CHANGELOG.md
spec/SPECIFICATION.md   # ONLY inside the BEGIN/END GENERATED region of §7.1–§7.2
standards/rust/*.md     # ONLY line-number re-pointing into the two globs above
.bklg/from-contract-to-published-library/projection-store-freeze/read-through-and-rebuild-rules/**
```

The implementer **may** additionally touch the composition-root files named in
the Integration contract to mount this slice — `registry.rs`'s enumeration, the
harness files and the mutant registry, all already inside the block above — and
that is not scope drift. Widening the block beyond it (`crates/happenstance-core/**`
in particular) is a decision to be made here, in the spec, or a reason to stop.

**The last three lines were added on 2026-08-15, and they are an admission rather
than a widening.** Commit `5be22ab` — this story's own implementation — took all
three and disclosed none of them, because the block above named none of them and
the implementation report was silent. Each is **compelled by the gate**, not
chosen:

- `spec/SPECIFICATION.md`'s `BEGIN/END GENERATED` region: `cargo xtask spec-trace
  --write` regenerates it whenever a rule name starts or stops existing, and a
  *separate* gate step fails on a stale region. `5be22ab` removed four `†`. A
  boundary forbidding that hunk demands the story leave stale exactly what the
  gate demands it regenerate — which is the same argument the slice-mate's AC-010
  settled at `5f2cf02` (`reset-rules/spec.md:397-415`) and this entry applies here.
- `CHANGELOG.md`: CF-29's lint fails the gate unless every rule in `RULE_FILES`
  has an entry, so a story that adds three rules cannot avoid it. `5be22ab` added
  49 lines.
- `standards/rust/*.md`: `cargo xtask lint-constitution` fails when a cited line
  in `crates/happenstance-testkit/**` moves. **Line-number re-pointing only** —
  rule text, evidence selection, retirement and new atoms stay out of boundary.
  Precedent `aef8990`; `unstable-projection-gate-and-clause-disposition` admitted
  the identical entry into its own boundary at `08a2299`.

The pattern is settled once at project level (`_slices.md`, *Run 6's record*) so
that the next rule-bearing story neither re-litigates it nor takes it silently.
The standing obligation is **disclosure in the implementation report**, which is
what was actually missing here.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **`batch_reads_reflect_pending_writes`** | `pub async fn …<F: ProjectionFixture>(open: impl AsyncFn() -> F) -> RuleOutcome`. Gate first; then `open()` → `connect()` → `begin()` → `probe_write(&mut batch, k, v)` → `probe_read_through(&batch, k) == Some(v)` **before** commit. Asserts the pending value, not merely non-`None`. | `spec/SPECIFICATION.md:5059-5065`; `crates/happenstance-testkit/src/suite.rs:1-13` (rule shape) |
| **The gate, and where it is read** | `READS_THROUGH_BATCH` is read from `<F::Store as ProjectionProbe>`, not from the fixture. On `false` the rule returns `RuleOutcome::Skipped { capability: "ProjectionProbe::READS_THROUGH_BATCH", reason }` and does nothing else. The rule is still **emitted** as a test — the branch lives in the rule body, never in an emitter, because an emitter can only branch with `#[cfg]` and a rule absent from the binary is indistinguishable from one that passed. | `spec/SPECIFICATION.md:5004`, `:7597-7605` (CF-18); `crates/happenstance-testkit/src/suite.rs:15-46` |
| **The capability name is a string const** | Mirrors `NO_STORE_LIMITS` (`pub const … &str`), so the skip names the constant the author can change rather than a stringified fixture identifier that does not exist. Reuse the one `projection-capability-skips` landed; do not fork. | `crates/happenstance-testkit/src/contract.rs:435-442`; `_decomposition.md:181-188` (UX AC-U10) |
| **`rebuild_is_chunk_size_invariant`** | Same gate. Replays one fixed sequence at chunk sizes **1, 3 and whole-log** against **three isolated stores** (`open()` per run), each step a read-modify-write through the batch (`probe_read_through(…).unwrap_or(0) + 1`), each chunk one `begin`/`commit` carrying `Authority::Rebuilding`. Compares `probe_read(key)` per key **across runs**. | `spec/SPECIFICATION.md:5086-5098`; `_decomposition.md:574-577`; D3 |
| **The sequence diverges under the named defect** | At least one key is written twice inside one chunk at size 3 (e.g. `a, a, b, a, b, a` → `a = 4, b = 2`). A committed-state read yields `a = 2` at size 3 and `a = 1` at whole-log. A plain `set` instead of an increment makes the rule pass on that store — the failure mode this row exists to forbid. | `spec/SPECIFICATION.md:5088-5093`; `discover.md:102-110` |
| **The suite's out-of-band read is not PS-13's subject** | The end-of-run comparison uses `probe_read`, which is the *suite* reading after commit. PS-13 constrains what a projection's `apply` may read while writing; the rule's own read-modify-write is entirely through the batch. Stated in the rule's rustdoc so a later reader does not "fix" it. | `spec/SPECIFICATION.md:5075-5085` |
| **No literal positions** | Positions are supplied by the rule, deliberately non-contiguous, strictly increasing across chunks, and **never asserted**. No assertion anywhere compares a checkpoint's `through` to a literal. | `CLAUDE.md`, *The rule that matters*; `_decomposition.md:705-707`; `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:110-125` |
| **`rebuilding_is_distinguishable_from_live`** | Ungated (no read path needed). `reset` → commit two chunks with `Authority::Rebuilding`, asserting `Checkpoint::Rebuilding` after each → commit a third with `Authority::Live`, asserting `Checkpoint::Live`. Asserts the **variant**; the `through` value is not compared to anything. A rebuild that has committed nothing reads `NeverRun` and the rule does not assert otherwise. | `spec/SPECIFICATION.md:5330-5352`; `:4696-4714` (the signatures it needs) |
| **No runner, no driver** | Everything is expressible through `commit`'s `Authority` argument. The six runner-dependent rules are CF-36's and belong to the workspace e2e crate, not here. | `spec/SPECIFICATION.md:5693-5703`; `discover.md:51-55` |
| **Three mutants, exact declarations** | `CommittedReadBatchStore` → fails `batch_reads_reflect_pending_writes` **and** `rebuild_is_chunk_size_invariant`; `FirstWriteWinsBatchStore` → fails `rebuild_is_chunk_size_invariant` only; `LiveOnlyCheckpointStore` → fails `rebuilding_is_distinguishable_from_live` only, and preserves `NeverRun` so it does not also fail `fresh_projection_has_no_checkpoint`. Each row carries non-empty `provenance` and, where a rule has more than one failing assertion, an `expect` pin. | `crates/happenstance-testkit/tests/mutation_coverage.rs:141-186`; `spec/SPECIFICATION.md:5674-5685`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| **One conformant variant, both arms alive** | `NoBatchReadStore` is `Kind::ConformantVariant` with `fails: &[]`: it passes every projection rule, exposing no read path on the batch (`probe_read_through` = `unimplemented!()`, the specification's spelling — `clippy::todo` is denied workspace-wide, `unimplemented` is not linted) and declaring `READS_THROUGH_BATCH = false` with a non-empty reason. `MemoryProjectionStore` holds the `true` arm. | `spec/SPECIFICATION.md:5052-5054`, `:5011-5012`; `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:225-247`; `.bklg/…/memory-projection-store/spec.md:411` |
| **The skip is asserted as a value** | A `#[test]` drives the two gated rules against the declining fixture and asserts `RuleOutcome::Skipped { capability, reason }` — both fields — rather than reading stdout. libtest suppresses a passing test's stdout without `--show-output`, so the printed line is reachable, not checked. | `crates/happenstance-testkit/src/contract.rs:458-537`; `mutation_coverage.rs:3160-3184`; `_decomposition.md:170-180` (UX AC-U09) |
| **A declined capability names a non-empty reason** | `Capability::declined("")` is a `const fn` `assert!` that fires at **codegen** for an associated const — caught by `cargo build`/`cargo test`, not by `check` or `clippy`. The reason is written for a human reading a CI log: why this store cannot, not that it cannot. | `crates/happenstance-testkit/src/contract.rs:374-419`; `standards/rust/91-adapter-authoring-recipe.md` (RS-91-3) |
| **Mounted at all four places** | Rules module + `for_each_projection_store_rule!` + three harnesses + registry rows & `for_each_mutant!`. The orphan meta-test and the exhaustiveness meta-test are what convert "I forgot one" into a red build. | `crates/happenstance-testkit/src/registry.rs:346-366`, `:410-436`; `mutation_coverage.rs:187-196`; `_decomposition.md:339-360` |
| **wasm32, same run, no new gate step** | The three rules reach the `wasm32` harness through the existing enumeration, and that harness is type-checked by `cargo xtask ci`'s mandatory conformance-harness step. The skip line reaches a human there via `console_log!`, because `RuleOutcome::report` is a measured no-op on that target. | `xtask/src/main.rs:231-240`; `crates/happenstance-testkit/src/contract.rs:515-531`; project AC-016 |
| **Both flavours, one name per module** | The rules bind the **bare** `ProjectionStore`/`ProjectionProbe` — the weaker requirement, which accepts both flavours through `trait_variant`'s blanket impl. A module imports `ProjectionStore` **or** `SendProjectionStore`, never both, or `F::Store::Batch` and the method calls are ambiguous. | `CLAUDE.md` binding constraints 1 and 4; `.kb/decisions/0001-async-port-flavours.md` |
| **Nothing regresses** | The event-store suite, every existing harness, `Capability`, `RuleOutcome`, the emitters and the `Declared` shape are untouched. `cargo xtask spec-trace` is unaffected: no clause text, marker or citation changes in this diff. | `_decomposition.md:362-370`; `xtask/src/main.rs` (mandatory step list) |

## Data and migrations

**N/A — no schema, no stored data, no wire format, no persisted artefact.** This
story adds conformance rules and test-only stores to `happenstance-testkit`;
every store it introduces is an in-memory instrument living in that crate's own
`tests/`, and none is published, serialised or read by anything outside the test
binary that defines it. ADR-0003 is untouched — no dependency is added anywhere,
least of all `serde`.

Two adjacent facts, named so neither is mistaken for a migration. First, the
testkit **carries its own version** precisely because its macro arms are a public
surface (`crates/happenstance-testkit/Cargo.toml`); adding rules to
`for_each_projection_store_rule!` changes what an adapter's suite invocation
expands to, which is an additive change to an unpublished crate and creates no
consumer obligation today — the obligation is `publication-and-positioning`'s,
at first publish. Second, the only "data" this story defines is the fixed probe
sequence of D3, and it lives in the rule body as a constant rather than in a
fixture, so that all three chunk sizes and all four stores replay literally the
same input.

## Acceptance criteria

Every criterion is written from **P2, the adapter author** — the persona whose
goal is *"an executable definition of 'correct' they can run against their own
storage system"* and whose fear is *"discovering, late, that the port quietly
assumed something their storage cannot provide"*
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`)
— or from **P3, the local-first / edge developer** (`…/personas-and-journeys.md:182-189`),
who meets the same suite through the `wasm32` emitter. A criterion is satisfied
only when it is observable from *outside* the crate that implements it: by
invoking one macro, or by reading one line of a gate run.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** an adapter author whose storage lets an open batch see its own pending writes, and whose `ProjectionProbe` therefore declares `READS_THROUGH_BATCH = true`, **WHEN** they invoke `projection_store_conformance!` with their fixture, **THEN** a test named `batch_reads_reflect_pending_writes` runs, writes a value through the open batch and asserts `probe_read_through(&batch, k) == Some(v)` for **that value** *before* commit — so a store whose batch `get` answers from committed state fails it by name rather than passing on a `None`-vs-`Some` technicality. | Integration: the rule driven against `MemoryProjectionStore` through the tokio and blocking harnesses (`cargo test -p happenstance-testkit`). Unit: `CommittedReadBatchStore`'s row in the projection `REGISTRY` under `mutants_fail_exactly_their_declared_rules` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2889`), which is what proves the rule rejects the named wrong implementation (`spec/SPECIFICATION.md:5674`). |
| **AC-002** | **GIVEN** an adapter author who will rebuild a projection in production at whatever chunk size fits their memory budget, and who must not have to discover that the chunk size changed the answer, **WHEN** the suite runs against their store, **THEN** `rebuild_is_chunk_size_invariant` replays **one fixed sequence** (`a, a, b, a, b, a`) at chunk sizes **1, 3 and whole-log** against **three isolated stores** (`open()` per run), each step a read-modify-write **through the batch** (`probe_read_through(…).unwrap_or(0) + 1`), and asserts the three runs' read models are identical per key — so a store that cannot read through its own batch diverges (`a = 2` at size 3, `a = 1` at whole-log) instead of quietly agreeing. | Integration: the rule against `MemoryProjectionStore` at all three sizes. Unit: `CommittedReadBatchStore` **and** `FirstWriteWinsBatchStore` both declared as failing this rule and confirmed by the exactness meta-test. Review: the rule body uses an **increment**, never a plain `set` — a `set` is chunk-insensitive by construction and would certify PS-13/PS-14 on nothing (`spec/SPECIFICATION.md:5086-5098`; `_decomposition.md:574-577`). |
| **AC-003** | **GIVEN** a reader who must decide whether the rows in front of them are authoritative, **WHEN** a store commits two chunks under `Authority::Rebuilding` and then one under `Authority::Live`, **THEN** `rebuilding_is_distinguishable_from_live` asserts `checkpoint(id)` reads the `Checkpoint::Rebuilding` **variant** after each of the first two and `Checkpoint::Live` after the third — comparing variants only, never the `through` value — and a store that rebuilds in place behind a single position field fails it. The rule asserts nothing immediately after `reset`, because a rebuild that has committed nothing correctly reads `NeverRun`. | Integration: the rule against `MemoryProjectionStore`. Unit: `LiveOnlyCheckpointStore`'s row, which must fail **exactly** this rule — it preserves `NeverRun` for an unseen id, so it does not also trip `fresh_projection_has_no_checkpoint` and make its own declaration false (`spec/SPECIFICATION.md:5330-5352`). |
| **AC-004** | **GIVEN** an adapter author deciding whether to trust this suite at all, and knowing that a rule no implementation can fail is decorative (`CLAUDE.md`, *The rule that matters*), **WHEN** they read the mutant registry, **THEN** each of the three new rules has at least one registered wrong implementation with an **exact** `fails` set and **non-empty `provenance` naming the real adapter mistake it models** — the round-trip batch `get`, the `entry().or_insert(…)` dedup buffer, the single-position-field rebuild — and **no pass rate is quoted anywhere over the mutant set**. | Unit: `every_rule_has_a_mutant` (`mutation_coverage.rs:2734`), `mutant_registry_is_exhaustive` (`:2754`) and `mutants_fail_exactly_their_declared_rules` (`:2889`), extended to the three new rules and four new stores. Static: review of the registry rows' `provenance` strings and of the module doc for ADR-0010's no-pass-rate warning (`.kb/decisions/0010-the-suite-must-prove-itself.md`). Discharges project **AC-003**. |
| **AC-005** | **GIVEN** an adapter author whose storage genuinely cannot answer a read from an uncommitted batch — the write-behind or buffering shape PS-12 explicitly permits — **WHEN** they declare `READS_THROUGH_BATCH = false` and run the suite, **THEN** the whole projection suite is **green**, not red: `NoBatchReadStore` is registered as a `Kind::ConformantVariant` with `fails: &[]`, exposes no read path on the open batch (`probe_read_through` is `unimplemented!()`), and passes every projection rule — so declining a capability honestly is a supported outcome rather than a failure, and the skip arm of both gated rules has a fixture behind it in the same run that `MemoryProjectionStore` holds the `true` arm. | Integration: `projection_store_conformance!` invoked against `NoBatchReadFixture`, zero failures. Unit: its `Declared` row with `fails: &[]` under `mutant_registry_is_exhaustive`, plus the conformant-variant assertion the registry already carries (`mutation_coverage.rs:3091`). |
| **AC-006** | **GIVEN** that same author reading a CI log, **WHEN** the two gated rules run against their declining fixture, **THEN** each returns `RuleOutcome::Skipped` whose `capability` field is the string **`ProjectionProbe::READS_THROUGH_BATCH`** — the constant they can actually go and change, not a fixture const that does not exist — and whose `reason` is the **fixture's own non-empty stated reason**, and the run reports **two** skips, not one and not silence. The assertion is on the `RuleOutcome` **value**, not on stdout. | Unit: a `#[test]` beside `capability_skips_are_reported` (`mutation_coverage.rs:3184`) driving both gated rules against `NoBatchReadFixture` and asserting **both** fields of `RuleOutcome::Skipped`. Static: `Capability::declined("")` is a `const fn` `assert!` that fires at codegen for an associated const, so an empty reason fails `cargo test`/`cargo build` (`crates/happenstance-testkit/src/contract.rs:374-419`). Discharges project **AC-005**'s real instance (`_storymap.md:83`). |
| **AC-007** | **GIVEN** an adapter author who invokes exactly one macro and expects the whole bar, and **GIVEN** P3, who runs that same bar on `wasm32-unknown-unknown`, **WHEN** the gate runs, **THEN** all three new rules are mounted at **all four** composition points — the projection rules module, the single enumeration `for_each_projection_store_rule!`, the three harnesses (tokio / blocking / `#![cfg(target_arch = "wasm32")]`), and the mutant registry's `for_each_mutant!` + `REGISTRY` — so each rule is emitted as a **named test on every target in one run**, with no wasm-specific subset and no new gate step. A rule present in the module but absent from the enumeration is caught by the orphan meta-test. | Unit: the projection sibling of `no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:412`). Static/E2E: `cargo xtask ci`'s mandatory wasm32 conformance-harness step type-checks the wasm emitter's expansion (`xtask/src/main.rs:231-240`); the tokio and blocking harnesses run the rules by name in the `tests` step. Contributes to project **AC-016**. |
| **AC-008** | **GIVEN** the repository owner reviewing this PR against a project whose whole point is that the port is not yet frozen, **WHEN** they read the diff, **THEN** nothing outside this story's boundary moved: `git diff` over `spec/SPECIFICATION.md` touches **nothing outside the `BEGIN/END GENERATED` region of §7.1–§7.2** — no clause text, maturity marker or rule citation changed (PS-13 and PS-14 are `[FROZEN]` and are **implemented, not amended**); no `ProjectionStore`, `ProjectionProbe`, `Checkpoint`, `Authority` or `MemoryProjectionStore` definition changed; `Capability`, `RuleOutcome`, the three emitters and the `Declared` shape are byte-identical; **no assertion anywhere compares a position or a checkpoint's `through` to a literal**; `cargo xtask spec-trace` is **green** (its generated region regenerated, never left stale); and the two other gate-compelled exits — a `CHANGELOG.md` entry per new rule (CF-29) and line-number re-pointing in `standards/rust/*.md` (`lint-constitution`) — are **taken and disclosed in the implementation report**, never taken silently. | Static: `cargo xtask spec-trace`; `git diff --stat` scoped to the PR-boundary block, reviewed against it, with the three compelled exits read against the report's disclosure. Unit: the existing event-store suite and `GappedPositionFixture`'s rules still green (`crates/happenstance-testkit/tests/mutation_coverage/variants.rs:110-125`), which is what convicts a rule that grew a `position == index + 1` assumption. |

**AC-008's instrument was amended on 2026-08-15; its premise was not.** It read
*"`cargo xtask spec-trace` is green **and unaffected**"* and listed no compelled
exit, which no story adding a rule name can satisfy — `spec-trace --write`
regenerates §7.1–§7.2, CF-29's lint demands a changelog entry, and
`lint-constitution` demands a citation re-point when a cited line moves. This
story took all three at `5be22ab` and disclosed none, which is the failure the
amendment fixes: not that the exits were taken, but that a criterion nobody could
meet produced a report that said nothing about them. The amendment is a
**tightening** — it replaces an unmeetable prohibition with a meetable disclosure
obligation, and it still rejects the thing it exists to reject: a hand-edited
clause, a moved marker, a rewritten citation, or a `standards/rust` edit that
changes a rule instead of a line number. The same settlement is written once at
project level in `_slices.md`, *Run 6's record*, so the sixth instance is not
re-litigated per story.

## Interaction quality

RFC §6.7/D6. Every invariant below is carried by an **AC-### row in the table
above** — none is stated only here, because `redkiln verify` extracts ACs from
that table and a prose bullet in this section would never be gated.

**Composition family — the honest disposition.** The project's signed-off design
declares `surfaces: []` and answers `N/A — no user-facing surface` to `## Items`,
`## Signatures`, `## The states the API must express` and `## Anti-patterns`
(`.bklg/…/projection-store-freeze/_design.md`, approved 2026-08-12 by the
repository owner). **This story renders no surface**, so the visual composition
invariants — placement, density budget, hierarchy — have no referent, and
inventing numbers for them here would contradict a signed-off design. What
`_design.md` *does* record is the one **text surface** a human meets: the single
line a conformance run prints for a declined capability, verified at
`crates/happenstance-testkit/src/contract.rs:500-507`. The composition family is
therefore held against **that** surface, and it is not vacuous — an
unstyled-render equivalent exists here and is exactly what AC-006 forbids.

| Invariant (family) | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** (composition). The skip is not a bare `println!` or a raw `Debug` dump: it is composed by the shared `RuleOutcome::skip_line`, so a projection skip is typographically identical to an event-store skip — one line, one shape, one vocabulary. Forking the skip format is the "bare markup" failure in this medium. | **AC-006** | Value assertion on `RuleOutcome::Skipped`'s fields plus reuse of `skip_line` unchanged (`contract.rs:500-507`); Architecture **AC-A06** forbids forking `Capability`/`RuleOutcome` (`_decomposition.md:317-319`). |
| **Content, not chrome** (composition). The line names the **constant the author can change** — `ProjectionProbe::READS_THROUGH_BATCH` — rather than a stringified fixture identifier. A skip that names the wrong switch is a rendered control that goes nowhere. | **AC-006** | The `capability` field is asserted for its exact string (UX **AC-U10**, `_decomposition.md:181-188`; precedent `NO_STORE_LIMITS`, `contract.rs:435-442`). |
| **Transience — persistent, not revealed on demand** (composition). The reason is emitted on the run that skipped, in the log a human already reads; it is never deferred to `--show-output`-only stdout as the *only* record. The machine-checked half is a value, which is what makes the guarantee persistent rather than incidental. | **AC-006** | The port's own doc states stdout *"makes the line reachable by a human; it does not make anyone read it"* (`contract.rs:515-523`); the assertion is on the value (UX **AC-U09**). |
| **Density budget — the real numbers.** Exactly **one** line per skipped rule, and exactly **two** skips from one declining fixture across this story's rules (PS-12's and the chunk-size rule's — D2). Not one (which would hide a gated rule), not three, not a multi-line block. | **AC-006** | The value assertion drives both gated rules against `NoBatchReadFixture` and counts two `Skipped` outcomes. |
| **Anti-pattern: silence.** The design's operative anti-pattern in this medium is a rule that *vanishes* when its capability is declined. CF-18 forbids it: the rule is still emitted as a test. | **AC-005**, **AC-007** | The gate branch lives in the **rule body**, never in an emitter — an emitter can only branch with `#[cfg]`, and a rule absent from the binary is indistinguishable from one that passed (`spec/SPECIFICATION.md:7597-7605`). |
| **Anti-pattern: a green that means nothing.** A skip arm with zero fixtures behind it is dead code wearing a pass. | **AC-005** | `NoBatchReadStore` exists precisely to keep it alive, and `MemoryProjectionStore` holds the opposite arm in the same run (D6). |
| **State — in place, no context jump.** A skip does **not** abort the run, fail the suite, or short-circuit the remaining rules: the declining fixture's suite is green end-to-end with the other rules still executing. | **AC-005** | `projection_store_conformance!` against `NoBatchReadFixture` completes with zero failures and every non-gated rule reported as run. |
| **State — non-occlusion.** A skip line never displaces or suppresses another rule's result; skipped and passing rules coexist in one run's output, and a skip is **distinguishable from a pass** rather than blending into it. | **AC-005**, **AC-006** | `RuleOutcome` is a distinct variant, `#[must_use]`, asserted by value (project AC-005, `project.md:194-197`). |
| **State — reversibility.** The declension is one `const` an author flips. Setting `READS_THROUGH_BATCH = true` restores both rules to running with no other edit — no fixture rewrite, no suite re-invocation, no registry change. | **AC-005**, **AC-006** | The gate reads `<F::Store as ProjectionProbe>::READS_THROUGH_BATCH` at the rule head and nowhere else (D1); the capability name in the message points the reader at that exact constant. |
| **State — reachability on every target** (the keyboard-reachability analogue). The reason must survive `wasm32-unknown-unknown`, where `RuleOutcome::report` is a measured no-op — otherwise P3 gets the skip with no reason, which is the one place this text surface is weakest. | **AC-007** | The wasm harness routes `skip_line` through `console_log!` the way `__emit_wasm` already does (`contract.rs:525-531`); UX **AC-U11**. If the harness landed by `projection-suite-entry-point` does not, this story is the first able to observe it and **reports it** rather than patching around it. |
| **Integrity of the composed line** (composition). A declined capability carries a **non-empty** reason, enforced by the compiler rather than by review. | **AC-006** | `Capability::declined` is a `const fn` `assert!`; on an associated const it fires at **codegen**, so `cargo build`/`cargo test` catch an empty reason and `cargo check`/`clippy` do not (`contract.rs:374-419`; UX **AC-U12**). |

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| **EC-001** | A fixture declares `READS_THROUGH_BATCH = Capability::declined("")` — an empty reason. | Not a runtime path: `Capability::declined`'s `const fn` `assert!` fires at codegen and the build fails (`contract.rs:374-419`). Do **not** add a defensive runtime check that would make the compile-time failure unreachable; do add the empty-reason case to the reason this story's fixture states a real sentence. |
| **EC-002** | The three rules cannot reach `ProjectionProbe` from `F::Store` because `ProjectionFixture` bounds only `ProjectionStore`. | Minimum repair **at the rule definitions**: a `where F::Store: ProjectionProbe` clause on these three functions. Emitters call rules by path and add no bounds, so this costs the enumeration nothing (D1). **Do not widen `ProjectionFixture`** — that trait is `projection-suite-entry-point`'s and widening it here silently re-scopes another story's deliverable. If the landed binding makes even the local clause impossible, stop and report at the slice boundary. |
| **EC-003** | A mutant fails a rule it does not declare, or passes one it does — `mutants_fail_exactly_their_declared_rules` goes red. | Narrow the defect or widen the declaration, **and write down which and why** in the row's `provenance`. Never relax or `#[ignore]` the exactness meta-test: it is the mechanism ADR-0010 exists to install (`.kb/decisions/0010-the-suite-must-prove-itself.md`). The most likely instance is `LiveOnlyCheckpointStore` also tripping `fresh_projection_has_no_checkpoint` because it lost `NeverRun`. |
| **EC-004** | `rebuild_is_chunk_size_invariant` passes against **every** store in the tree, including both mutants. | The rule is not yet a rule. Re-derive D3's arithmetic before touching the assertion: the usual causes are a plain `set` instead of an increment, or a sequence with no key written twice inside one chunk at size 3. This is Architecture Note 10 item 2 (`_decomposition.md:713-727`) and is a **finding to report**, not a reason to weaken the comparison. |
| **EC-005** | A projection rule turns out to be able to observe the read model **without** the probe. | Architecture Note 10 item 1: PS-11 is over-built and the seam should shrink — recorded as **an ADR paragraph raised at the slice boundary, not a quiet deletion** (`_decomposition.md:713-715`). Per `MEMORY`/house process, this story records the gap and does not author the ADR itself. |
| **EC-006** | A new PS-13 / PS-14 pairing defect is discovered while implementing the rule. | Report at the slice boundary. The sweep that scoped such defects is `ps-clause-pairing-sweep` and the repair lands as a decision atom in `projection-decision-atoms`; a `[FROZEN]` clause is **never** line-edited here (`CLAUDE.md`, *Open questions*). |
| **EC-007** | The chunked replay's positions regress or repeat across chunks, and the rule fails for a reason unrelated to chunk size. | Positions are chosen by the rule, deliberately **non-contiguous** and **strictly increasing** across chunks; a regression trips `commit_rejects_a_regressing_position` (PS-22, `spec/SPECIFICATION.md:5297-5316`). Fix the supplied sequence, never the position rule. |
| **EC-008** | The projection `wasm32` harness does not route `skip_line` through `console_log!`, so a skip on that target loses its reason. | This story is the first able to observe it (it supplies the first real skip). Report it as a defect against `projection-suite-entry-point`'s harness; a one-line fix inside the harness file is in bounds, a change to `RuleOutcome::report` is not (`contract.rs:525-531`). |
| **EC-009** | Every fixture in the tree ends up declaring `READS_THROUGH_BATCH = false` (e.g. if a later story flips `MemoryProjectionStore`). | Both gated rule bodies become code no test executes and their mutants never run. The structural guard is AC-005's: **one fixture on each arm, in the same run.** `memory-projection-store`'s spec already states that flipping it would be a finding for this story (`.bklg/…/memory-projection-store/spec.md:411`) — treat it as one. |

## Non-functional

| id | Requirement | Why / evidence |
| --- | --- | --- |
| **NF-001** | **No new dependency anywhere**, and in particular no `serde` and no change to any `Cargo.toml` feature table. Every store this story adds is an in-memory instrument in `happenstance-testkit`'s own `tests/`. | ADR-0003 (`.kb/decisions/0003-opaque-payloads.md`); the feature powerset `cargo hack` runs over is already widened by `projection-probe-conformance-feature` and must not grow again here. |
| **NF-002** | **No new CI step.** The three rules reach `wasm32` through the existing enumeration and the existing mandatory conformance-harness check; a new rule costs a harness nothing. | `xtask/src/main.rs:231-240`; project **AC-016**, whose whole point is that the wasm bar is not a separately maintained subset. |
| **NF-003** | **No `#[async_trait]`, no `Send` bound introduced, one flavour name per module.** Rules bind the bare `ProjectionStore` / `ProjectionProbe` — the weaker requirement, which accepts both flavours — and a module imports one of the two names, never both, or `F::Store::Batch`'s method calls are ambiguous. | `CLAUDE.md` binding constraints 1 and 4; `.kb/decisions/0001-async-port-flavours.md`. Violating this is how the `wasm32` target dies silently. |
| **NF-004** | **No borrowing GAT in any fixture** added here. `MemoryFixture`'s owned-handle pattern is what to copy. | `crates/happenstance-testkit/src/contract.rs:97-111` — the rustc ICE this repository already minimised once; `crates/happenstance-testkit/src/fixtures.rs:243-292`. |
| **NF-005** | **Rustdoc on every new `pub async fn` rule**, naming the clause it discharges, the wrong implementation it rejects, and — for the two gated rules — D2's residual sentence: *what a skip does not cover.* The chunk rule additionally documents why the end-of-run `probe_read` is not itself a PS-13 violation, so a later reader does not "fix" it. | `standards/rust/70-rustdoc-obligations.md`; `standards/rust/91-adapter-authoring-recipe.md` (RS-91-3, the declension recipe). |
| **NF-006** | **`cargo clippy --workspace --all-targets --all-features -D warnings` clean.** `unimplemented!()` is the specification's own spelling for the declining probe method and is permitted; `todo!()` is denied workspace-wide and must not be used. | `spec/SPECIFICATION.md:5011-5012`; `CLAUDE.md`, *Commands* (clippy with `-D warnings`). |
| **NF-007** | **Suite runtime stays negligible.** The chunk rule opens three isolated stores and replays a six-step sequence three times — eighteen probe writes total, all in memory. No sleep, no timeout, no wall-clock assertion anywhere; a rule that needs time to be correct is a flaky rule on `wasm32`. | `crates/happenstance-testkit/src/suite.rs:1-13` (rules take `impl AsyncFn() -> F`, so isolation is cheap by construction). |
| **NF-008** | **Determinism.** The probe sequence is a constant in the rule body, identical across all three chunk sizes and all four stores; no `HashMap` iteration order, randomness or clock feeds any assertion. | D3; the comparison is per-key, so a store's internal map ordering cannot change the verdict. |

## Implementation notes (non-prescriptive)

These are aids, not instructions; the Behavior and interfaces table is the
contract.

- **Write the failing test first, in the mutant registry.** The cheapest order is
  `CommittedReadBatchStore` → `batch_reads_reflect_pending_writes` → watch it
  fail by name → then the chunk rule, which the same mutant also fails. That
  ordering means the rule is never green before something wrong has been red.
- **The gate helper.** `suite.rs`'s `require!` stringifies its argument as the
  capability name (`crates/happenstance-testkit/src/suite.rs:36-46`), which is
  wrong for a const that lives on the store's probe impl. A sibling macro — say
  `require_probe!` — reading `<F::Store as ProjectionProbe>::READS_THROUGH_BATCH`
  and reporting a `&'static str` capability name is the smallest change. Check
  first whether `projection-capability-skips` already landed one; reusing beats
  adding.
- **Keep the sequence a `const`.** `const PROBE_SEQUENCE: &[&str] = &["a", "a",
  "b", "a", "b", "a"];` next to the rule, with the expected-state arithmetic in
  the doc comment, is what makes D3 auditable by reading rather than by running.
- **Chunking is a fold over that constant**, not three hand-written bodies:
  `for chunk in sequence.chunks(size)` with `size ∈ {1, 3, sequence.len()}`. One
  `begin`/`commit` per chunk, `Authority::Rebuilding` on every chunk (this is a
  rebuild), and a fresh `open()` before each of the three runs.
- **Positions.** Something like `10, 25, 40, …` — strictly increasing, obviously
  non-contiguous, and never compared to anything. If a reviewer can compute the
  expected checkpoint from the loop index, the sequence is too tidy.
- **`NoBatchReadStore` should be boring.** A `HashMap` staged in the batch,
  flushed at commit, and `probe_read_through` = `unimplemented!()`. Resist adding
  replay-at-commit semantics or a second batch shape — that is
  `buffering-conformant-variant`'s deliverable and claiming it here breaks
  AC-004's provenance one slice early (D6).
- **`FirstWriteWinsBatchStore` is one call site.** `entry(k).or_insert(v)` where
  the correct spelling is `insert(k, v)`. That is why it is plausible; keep the
  rest of the store honest so it fails exactly one rule.
- **`LiveOnlyCheckpointStore` must keep `NeverRun`.** Its defect is that
  `Authority` is ignored and `Rebuilding` is unrepresentable — not that
  checkpoints are broken generally (EC-003).
- **Rustdoc the residual before the code is green**, while the reason is still in
  your head: *chunk-size invariance is unverified for an adapter that declines
  `READS_THROUGH_BATCH`, because such an adapter has no read path for a
  projection's read-modify-write in the first place* (D2;
  `spec/SPECIFICATION.md:5032-5051`).
- **Mount as you go.** Adding the rule to `for_each_projection_store_rule!` in
  the same edit as the function avoids the orphan meta-test's red, and adding the
  `Declared` row in the same edit as the mutant avoids the exhaustiveness one's.

## Tests and CI (merge gate)

Tier vocabulary is the project testing brief's
(`_decomposition.md:740-786`): **Static** reads source/config without executing
the code under test; **Unit** is an in-process `#[test]`, including meta-tests
over registries; **Integration** is a conformance rule actually driving a
`ProjectionStore` through `begin` → probe-write → `commit` → read-back; **E2E**
is `cargo xtask ci` run whole.

| tier | command / path | proves |
| --- | --- | --- |
| **Unit** | `cargo test -p happenstance-testkit --test mutation_coverage every_rule_has_a_mutant` | Each of the three new rules has a registered wrong implementation — **AC-004**. |
| **Unit** | `cargo test -p happenstance-testkit --test mutation_coverage mutant_registry_is_exhaustive` | Every new store is declared, `fails` is non-empty for mutants and empty for `NoBatchReadStore`, and `provenance` is non-empty on all four rows — **AC-004**, **AC-005**. |
| **Unit** | `cargo test -p happenstance-testkit --test mutation_coverage mutants_fail_exactly_their_declared_rules` (`crates/happenstance-testkit/tests/mutation_coverage.rs:2889`) | `CommittedReadBatchStore` fails **both** read-path rules; `FirstWriteWinsBatchStore` fails only the chunk rule; `LiveOnlyCheckpointStore` fails only the rebuild-authority rule and nothing else — **AC-001**, **AC-002**, **AC-003**, **AC-004**. |
| **Unit** | `cargo test -p happenstance-testkit --lib registry::` — the projection sibling of `no_orphan_rules` (`crates/happenstance-testkit/src/registry.rs:412`) | No new rule exists in the module but outside the single enumeration — **AC-007**. |
| **Unit** | `cargo test -p happenstance-testkit --test mutation_coverage` — the new value assertion beside `capability_skips_are_reported` (`:3184`) | Both gated rules return `RuleOutcome::Skipped` against the declining fixture, with `capability == "ProjectionProbe::READS_THROUGH_BATCH"` and the fixture's non-empty reason — **AC-006**. |
| **Integration** | `cargo test -p happenstance-testkit` — the tokio and blocking projection harnesses driving `MemoryProjectionStore` | `batch_reads_reflect_pending_writes` and `rebuild_is_chunk_size_invariant` **run** (not skip) and pass against the `true` arm; `rebuilding_is_distinguishable_from_live` passes — **AC-001**, **AC-002**, **AC-003**. |
| **Integration** | the same harnesses driving `NoBatchReadFixture` through `projection_store_conformance!` | The declining store is **green across the whole projection suite** while reporting two skips — **AC-005**, **AC-006**. |
| **Static** | `cargo xtask ci`'s mandatory wasm32 conformance-harness step (`xtask/src/main.rs:231-240`) — `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | All three rules type-check under the wasm emitter, in the same run, with no new gate step — **AC-007**. |
| **Static** | `cargo clippy --workspace --all-targets --all-features -- -D warnings`; `cargo fmt --check` | NF-006; no `todo!()`, no warning-level debt introduced. |
| **Static** | `cargo xtask spec-trace` | No clause text, maturity marker or rule citation moved — **AC-008**. |
| **Static** | `git diff --stat` reviewed against the PR-boundary block | `happenstance-core`, the emitters, `Capability`, `RuleOutcome`, the `Declared` shape and every event-store rule/fixture/mutant are untouched — **AC-008**. |
| **Story grain** | `cargo xtask affected --base main` then `cargo xtask ci --fast` | The bar this non-terminal story meets during implementation (`_decomposition.md:831-845`; `CLAUDE.md`, *Commands*). **`--fast` is not evidence for AC-007**, because it omits the wasm32 steps. |
| **E2E (slice / project boundary)** | `cargo xtask ci` run whole | The merge gate. Read at the slice boundary with `reset-rules`, and again at the project boundary — the only run that is evidence for the wasm32 half of **AC-007**. |

Regression bar: the event-store suite, `GappedPositionFixture` and every
pre-existing projection rule stay green. A red anywhere outside this story's four
mount points is a signal to stop, not to adjust.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation, in this PR |
| --- | --- | --- |
| **The chunk rule is written with a plain `set` instead of an increment** and passes on a store that cannot read through its own batch — certifying two `[FROZEN]` clauses on nothing. | Medium / **high**: it is the natural spelling and the test goes green, which is the worst possible feedback. | D3 fixes the sequence and the arithmetic; `CommittedReadBatchStore` is registered as failing this rule, so a `set`-based rule fails `mutants_fail_exactly_their_declared_rules` immediately (EC-004). |
| **The gate is read from the wrong place** — a `ProjectionFixture` associated const rather than `<F::Store as ProjectionProbe>::READS_THROUGH_BATCH` — and the skip names a constant that does not exist. | Medium / medium: `require!` is right there and looks like it fits. | D1 states the correct source and the correct capability string; AC-006 asserts the exact string, so the wrong source fails a test rather than shipping a misleading log line. |
| **Both arms of the gate end up on the same side**, leaving one rule body unexecuted. | Low here, medium later / high: the coupling is to `memory-projection-store` and to any future fixture. | AC-005 requires a fixture on the declining arm and `MemoryProjectionStore` on the other, in the same run; EC-009 names the failure mode explicitly. |
| **`NoBatchReadStore` grows into the buffering variant**, silently claiming AC-004's second batch shape a slice early and leaving `buffering-conformant-variant` with nothing distinct to prove. | Medium / medium: the two are one refactor apart. | The PR boundary excludes it in words, D6 states the instrument is deliberately minimal, and the implementation notes say "should be boring". |
| **Coupling to `reset-rules` (same slice).** This story *calls* `reset` as a step; that story *interrogates* it. If `reset`'s behaviour is wrong, `rebuilding_is_distinguishable_from_live` fails for a reason it does not own. | Medium / low: both are in one context and either order is allowed (`_storymap.md:111`). | Diagnose against `reset-rules`' own rules first; a `reset` defect is that story's, and the fix belongs there, not in a workaround inside this rule. |
| **Coupling upward to the landed port.** If `Checkpoint` lacks `Rebuilding`, or `commit` lacks `Authority`, this story cannot be written. | Low / high: both are slice-2 deliverables already landed by `owned-batch-port-shape`. | Stated in the PR boundary: *a rule that cannot be written against the landed port is a **finding**, not a licence to edit the port.* Stop and report. |
| **`LiveOnlyCheckpointStore` over-fails**, tripping `fresh_projection_has_no_checkpoint` and invalidating its own declaration. | Medium / low. | AC-003 and EC-003 both require it to preserve `NeverRun`; the exactness meta-test converts the mistake into a red build in the same commit. |
| **The wasm32 skip loses its reason**, so P3 sees a skip with no explanation. | Medium / medium: `report` is a measured no-op on that target. | EC-008 makes it a reportable defect with a bounded in-harness fix; AC-007 puts the wasm harness in the gate. |
| **Scope creep into `spec/SPECIFICATION.md`** while reading four clauses closely. | Medium / high: editing a `[FROZEN]` clause is a process violation. | AC-008 asserts `spec-trace` is unaffected and the diff stays inside the boundary block; EC-006 routes any real pairing defect to a decision atom instead. |

## Dependencies

**Blocks on** (must be merged before this story starts):

- **`projection-mutant-registry`** — the projection `REGISTRY: &[Declared]`, the
  `for_each_mutant!` enumeration and the three exactness meta-tests. Without
  them this story's four rows have nowhere to go and AC-004 has no verifier
  (`.bklg/…/projection-store-freeze/projection-mutant-registry/spec.md`).
- **`projection-capability-skips`** — the projection skip machinery, the
  capability set fixed by DT-3's recorded resolution, and the `RuleOutcome`
  value-assertion pattern. This story supplies that machinery's **first real
  declension**; it does not build it (`_storymap.md:83`).

Both are in slices 3 and 4 and land ahead of slice 5 by the merge order
(`_storymap.md`, *Merge order* items 3–5). Transitively this story also needs
slice 2's port and probe (`owned-batch-port-shape`,
`projection-probe-conformance-feature`, `memory-projection-store`) and slice 3's
entry point (`projection-suite-entry-point`), all of which are already
prerequisites of the two direct dependencies.

**Slice-mate** (same slice, implemented in one context, mounted as one surface,
either order): **`reset-rules`**. Independent of this story by construction —
this one calls `reset`, that one interrogates it (`_storymap.md:111`).

**Unlocks**:

- **`buffering-conformant-variant`** — depends on this story directly; it needs
  the full read-path rule set in place before a second batch shape can be
  claimed to pass "the whole suite" (`_storymap.md`, `dependsOn` column).
- **`ps3-batch-shape-finding`**, transitively, since its evidence is read off the
  buffering variant's run.
- **`whole-gate-run-and-proof-artefact`** — the proof artefact names the rules
  this story adds among those passing under the wasm32 emitter.

## Anchors (progressive disclosure)

Open these **when the row says**, not before. Nothing here is required to
understand the Context pack — it is the depth behind it.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md:5052-5074` | PS-12's exact `MUST`, its two permitted arms, and the wrong implementation §4.11 names. The rule's assertion shape comes from `:5059-5065`. | Before writing `batch_reads_reflect_pending_writes`. | AC-001 |
| `spec/SPECIFICATION.md:5086-5098` | PS-14's prescribed rule shape — the three chunk sizes and the increment that couples it to PS-12. Read with `:5075-5085` (PS-13), whose subject is the *projection's* reads, not the suite's. | Before writing `rebuild_is_chunk_size_invariant`, and again before "fixing" the end-of-run `probe_read`. | AC-002 |
| `spec/SPECIFICATION.md:5330-5352` | PS-24's rule steps verbatim, plus the sentence that a rebuild which has committed nothing reads `NeverRun` — the assertion most likely to be added wrongly. | Before writing `rebuilding_is_distinguishable_from_live`. | AC-003 |
| `spec/SPECIFICATION.md:5658-5691` | §4.11's rule table (each rule ↔ its rejected implementation) and CF-5's conformant-variant clause, which is what makes `NoBatchReadStore` a legitimate registry entry rather than an oddity. | When writing the mutant rows' `provenance` and `NoBatchReadStore`'s. | AC-004, AC-005 |
| `crates/happenstance-testkit/src/contract.rs:355-537` | `Capability` and `RuleOutcome` in full: `declined`'s `const fn assert!`, `skip_line`'s composition, `report`'s stdout caveat and its wasm no-op. This is the whole text surface, and it is reused unchanged. | Before implementing the gate and the skip assertion. | AC-006 |
| `crates/happenstance-testkit/src/contract.rs:435-442` | `NO_STORE_LIMITS` — the precedent for a capability name as a `&'static str` that sends a reader to the constant they can change, and the sentence explaining why. | When naming the capability string. | AC-006 |
| `crates/happenstance-testkit/src/suite.rs:1-46` | The rule contract (`impl AsyncFn() -> F`, "not a fixture, but how to make one") and `require!`'s expansion — which is exactly why it cannot serve a probe const. | Before writing the first rule signature and the gate helper. | AC-001, AC-002, AC-006 |
| `crates/happenstance-testkit/src/registry.rs:346-436` | The single enumeration and `no_orphan_rules`' textual scan of the rules source — the mechanism that makes "mounted" mean something for a rule. | At mount time, and when the orphan meta-test goes red. | AC-007 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs:141-206` | `Declared`'s fields, the `expect` pin, the three-edit rule for adding a mutant, and ADR-0010's no-pass-rate warning. | Before adding the four registry rows. | AC-004, AC-005 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs:2734-2907` | The three exactness meta-tests as actually written — the failure messages you will be reading when a declaration is wrong. | When `mutants_fail_exactly_their_declared_rules` goes red (EC-003). | AC-004 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs:3131-3184` | `capability_skips_are_reported` — the exact shape of a `RuleOutcome`-value assertion, to be mirrored rather than reinvented. | When writing the two-skip assertion. | AC-006 |
| `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:110-125`, `:225-247` | `GappedPositionStore` (the store that convicts a literal-position assumption) and the conformant-variant registration pattern `NoBatchReadStore` copies. | Before choosing the position sequence, and when registering the variant. | AC-005, AC-008 |
| `crates/happenstance-testkit/src/fixtures.rs:234-292` | `MemoryFixture`'s owned-handle pattern and the precedent of building an instrument purely to keep a rule non-vacuous. | Before writing `NoBatchReadFixture` — copy this, do not invent (NF-004). | AC-005 |
| `crates/happenstance-core/src/projection.rs` | The landed port: `begin`/`commit`/`checkpoint`/`reset`, `Checkpoint`, `Authority`, and `ProjectionProbe` behind `conformance`. The signatures the rules call. | Before the first rule body; again if a rule seems to need something the port lacks (that is a finding, not an edit). | AC-001, AC-002, AC-003 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | Why the mutant registry exists and why no pass rate is ever quoted over it — the standard AC-004 is held to. | Before writing `provenance` strings or any summary of the mutant set. | AC-004 |
| `.kb/decisions/0001-async-port-flavours.md` | Why no `#[async_trait]`, why two flavours, and why generic code binds the bare trait. Explains the ambiguity NF-003 forbids. | Before adding any bound or import to the rules module. | AC-007 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:476-577` | Architecture Note 4 (the `READS_THROUGH_BATCH` gate), Note 5 (the fixture contract) and Note 6 (the rule skeleton and the increment) — the briefs this story implements rather than re-decides. | Before implementation, once. | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md:705-727` | Note 9's "never assert literal positions" and Note 10's reshape triggers, in full. | Before choosing positions; and whenever a rule passes on every store. | AC-008, EC-004, EC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The signed-off design: `surfaces: []`, the two non-visual surfaces, and the approval that makes "no visual composition invariants" a decision rather than an omission. | Before writing anything that looks like UI, or when questioning the Interaction-quality disposition. | AC-006 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/memory-projection-store/spec.md:411` | The statement that `MemoryProjectionStore` declares `READS_THROUGH_BATCH = true` and that flipping it is a finding for **this** story. | When confirming the `true` arm has a fixture (EC-009). | AC-005 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/projection-api-design-record/spec.md:301` | Where `READS_THROUGH_BATCH` is recorded as a probe const rather than a fixture const, by name. | If tempted to add it to `ProjectionFixture` (EC-002). | AC-006 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`, `:182-189` | P2's goal and the Marten objection this story answers in its exact shape; P3's `wasm32` constraint. | When judging whether a skip message is good enough. | AC-005, AC-006, AC-007 |
| `standards/rust/91-adapter-authoring-recipe.md` | The declension recipe — how an adapter author is meant to state a capability and its reason. | When writing `NoBatchReadStore`'s reason string. | AC-006 |
| `standards/rust/70-rustdoc-obligations.md` | The rustdoc bar every new `pub` item must clear, including D2's residual sentence. | Before opening a PR (NF-005). | AC-001, AC-002 |
| `xtask/src/main.rs` | The gate as defined once — in particular the mandatory wasm32 conformance-harness step at `:231-240` that carries AC-007's second half. | When claiming wasm32 evidence, and to confirm no new step was needed. | AC-007 |
| `spec/E2E-CASES.md:554-575`, `:577-596`, `:654-675` | E2E-21, E2E-22 and E2E-25 — the same three behaviours stated as observable cases, useful when a rule's assertion feels ambiguous. | If an assertion's intent is unclear from the clause alone. | AC-001, AC-002, AC-003 |
| `RUNBOOK.md:3848-3965` | Phase 6 in full — where the read-your-writes and rebuild work sits in the plan of record, and what it is a precondition for. | For orientation only; not needed to implement. | AC-008 |

## Clarifications resolved during spec

1. **Which fixture declares `READS_THROUGH_BATCH = false`?** (discover Q1) —
   Settled: a **new minimal instrument built here**, `NoBatchReadStore`,
   registered as a conformant variant. The buffering variant was the obvious
   candidate but is downstream (`buffering-conformant-variant`, slice 6), and
   waiting for it would leave the skip arm dead through two slices. If that
   variant also declares `false` when it lands, that is a second instance, **not
   a duplicate to delete** (D6, AC-005).
2. **Does `MemoryProjectionStore` declare `true`?** (discover Q2) — Yes, and this
   story **consumes** that rather than changing it; the memory store's own spec
   already records that flipping it would be a finding for this story
   (`.bklg/…/memory-projection-store/spec.md:411`). That fixes one fixture on
   each arm of the gate, which is the structural guard behind EC-009.
3. **The fixed sequence, and why 1, 3 and whole-log** (discover Q3) — Settled at
   `a, a, b, a, b, a` with an **increment** as the probe write, correct final
   state `a = 4, b = 2` at every chunking. Sizes justified in D3: 1 is the
   degenerate case a broken store gets right, 3 is the smallest size putting a
   repeated key both inside a chunk and across a boundary, whole-log is the
   scale case and the maximally wrong one for a committed-state read.
4. **Does the rebuild-authority rule need a runner?** (discover Q4) — **No.**
   Everything it needs is on the port's own signature; §4.11 places the six
   runner-dependent rules in the workspace e2e crate under CF-36
   (`spec/SPECIFICATION.md:5693-5703`), owned by a different project. A rule here
   that needed a runner would be in the wrong crate (D5).
5. **Both read-path rules gate on the same const** — decided in D2, with the
   rejected alternative recorded: running a *blind* write sequence for declining
   adapters so the chunk rule could report `Ran`. Rejected because blind writes
   are chunk-size invariant by construction, making that arm a rule no adapter
   can fail. The consequence is that a declining fixture reports **two** skips,
   and the density budget in *Interaction quality* states that number.
6. **Where the capability const is read from, and what the skip names it** —
   `<F::Store as ProjectionProbe>::READS_THROUGH_BATCH`, reported as the string
   `ProjectionProbe::READS_THROUGH_BATCH`. This is why `suite.rs`'s `require!`
   cannot be reused verbatim, and why the capability name is a string const on
   the `NO_STORE_LIMITS` model rather than a stringified identifier (D1, EC-002).
7. **AC enumeration.** The eight criteria above are exactly the ids the first
   pass fixed (AC-001 – AC-008); none was added or dropped. AC-001 – AC-003 are
   one rule each, AC-004 – AC-005 are the registry and the declining instrument,
   AC-006 is AC-005-of-the-project's real instance, AC-007 is the mount, and
   AC-008 is the "nothing else moved" bar. Project **AC-003** is discharged by
   AC-004 (with AC-001 – AC-003 supplying the mutants it counts); project
   **AC-005** is discharged by AC-006, with AC-005 supplying the fixture that
   makes it observable.
8. **Interaction quality in a medium with no screen.** The composition family is
   held against the *text surface* `_design.md` names — the skip line — rather
   than declared inapplicable. `surfaces: []` was approved on 2026-08-12, so no
   visual invariant is invented here; but "presentation exists at all" has a real
   referent in this medium (a composed `skip_line` versus a bare `println!`) and
   is gated by AC-006.
