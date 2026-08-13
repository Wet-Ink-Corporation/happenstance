---
item: HS-S0030
stage: discover
created: 2026-08-12T13:01:53.114Z
updated: 2026-08-12T13:01:53.114Z
template_sig: 86ce4036
rendered_sig: 388fe1a3
---

# Discover — The compile-fail case, its negative control, and its gate row

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: land the compile-fail case that adds a variant to the example's domain enum **and its negative control**, with the diagnostic landing on the user's `match` arm and not inside a macro body, registered as a row in `xtask/src/proof.rs`'s `ARTEFACTS` so the gate runs `cargo xtask proof-artefact` — which a deleted *or* emptied case fails — and **verify the negative control fails rather than assuming it** | `_storymap.md:61` (M6 row) | Four obligations, and the fourth is an act rather than an artefact |
| AC-002 — a `trybuild` compile-fail case adds a variant to the worked example's domain enum and the crate fails to compile until the fold handles it; **the negative control — removing the protection makes the case fail — is present and runs in the gate**. *"This is the project's proof artefact"* | `project.md:165-168`; DoD 2 at `project.md:226-227` | The negative control is not a nicety. It is what distinguishes an instrument from a decoration |
| `depends_on: worked-example-on-typed-layer` — supplies the domain enum whose `match` arm the diagnostic must point at. The case *"points its diagnostic at the example's own `match` arm and therefore cannot precede it"* | `_storymap.md:61`, `:127-129` | The dependency is not organisational; the fixture's `-->` span literally names a file the sibling story creates |
| AC-U07 — the diagnostic lands on the **caller's** code: the `.stderr` fixture's span points into `examples/course-subscriptions/`, not into a macro body. *"A guarantee whose diagnostic names a file the user did not write is a guarantee they cannot act on"* | `_decomposition.md:132-138`; `_design.md:210-213`, `:999-1001` (anti-pattern 13) | A passing fixture that points at `crates/happenstance/src/` is a passing fixture and a failed guarantee |
| **AC-A03 — registered in the gate, not merely present in a `tests/` directory.** The row goes in `xtask/src/proof.rs`'s `ARTEFACTS`, and the gate step runs `cargo xtask proof-artefact` rather than `cargo test` for a stated reason: `cargo test` exits 0 on `running 0 tests`, *"so an emptied file passes a step that a deleted one fails"* | `_decomposition.md:372-380`; `xtask/src/proof.rs:57-70` (the `Artefact` row shape: `package`, `target`, `tests`), `:132-149` (`ARTEFACTS`, three rows today) | The row names the *tests* that must be present by name, so a rename has to be noticed |
| The existing rows are the pattern and the precedent: `mutation_coverage`'s meta-tests, `happenstance-core`'s `wire` negative controls — *"the only tests in the workspace whose deletion reads as tidying up dead code"* — and `happenstance-sync`'s two envelope tests | `xtask/src/proof.rs:104-149` | This story adds a fourth row in the same shape, and must justify the names it lists the way the existing rows do |
| **A `compile_fail` doctest is not an equivalent instrument, and this is settled rather than re-argued.** rustdoc collects doctests from the lib target only, so one placed in `tests/` never runs; and rustdoc on 1.97.1 *silently ignores* an unmatched error-code annotation, so the bare form passes on any compile error including a typo — which means the negative control **cannot discriminate** | `_design.md:215-220`; `_decomposition.md:674-688` (Tensions 2); `RUNBOOK.md:3614-3627`; `spec/SPECIFICATION.md:8772` | Phase 4's artefact was decorative for exactly this reason. The repository has paid for this once |
| The in-tree confirmation, in the contract crate's own doc comment: a `compile_fail` doctest *"Spelled bare `compile_fail` rather than `compile_fail,E0080`: rustdoc on 1.97.1 silently ignores an error-code annotation it cannot match, so the stricter-looking spelling is the weaker check. It is also deliberately weaker than a `trybuild` snapshot — it does not pin the diagnostic — and ADR-0015 records that phase 6 owns the `trybuild` dependency decision"* | `crates/happenstance-core/src/event.rs:95-106` | Read directly. The contract crate states both the limitation and the ownership of the fix |
| **`trybuild` is absent from the workspace and its adoption is HS-P0010's decision.** Verified in `_grounding.md`: no `trybuild` reference anywhere in `Cargo.toml` or `Cargo.lock`. The design marks this surface *"specified but unbuildable"* until the input arrives, and says **escalate rather than substitute** | `_grounding.md:23-27`, `:177-187`; `_design.md:222-225`, `:1257-1259`; `project.md:266` (risk row 2) | The instrument does not exist and this project may not create it |
| The design's own `compile_fail` companion — the `DomainEvent` with an empty `EVENT_TYPES` on the trait's item page — is explicitly **not** AC-002's instrument | `_design.md:1113-1131` | Two compile-fail artefacts exist in this project and only one of them is the proof artefact. Conflating them is how AC-002 gets quietly downgraded |
| If `trybuild` needs a CI provision: none. It is a pure-Rust dev-dependency exercised by `cargo test`, unlike `cargo-hack`/`cargo-deny`/nightly rustdoc | `_decomposition.md:1084-1090` | The blocker is a dependency decision, not an infrastructure one |

## Questions

**Answered here.**

- *Where is the artefact registered?* A new row in `xtask/src/proof.rs`'s `ARTEFACTS`
  (`:132-149`), naming the package, the test target and the tests by name — the same shape as
  the three rows already there. Not a `cargo test` step, for the reason the file itself gives.
- *Where must the diagnostic point?* At the example's own `match` arm, in
  `examples/course-subscriptions/`. A span into `crates/` fails anti-pattern 13
  (`_design.md:999-1001`).
- *Is a `compile_fail` doctest an acceptable fallback?* No. It is measurably weaker in two
  independent ways and the negative control — the entire point of AC-002 — cannot
  discriminate under it. The design records this as settled and instructs escalation rather
  than substitution (`_design.md:215-225`).
- *Is the negative control verified or assumed?* Verified. *"Verify it fails, do not assume
  it"* is written into the risk table (`project.md:268`) and into the slice
  (`_storymap.md:61`).

**Deferred, and the first one is a hard block.**

- **BLOCKED on HS-P0010: `trybuild`.** AC-002's only instrument is absent from the workspace,
  and its adoption is `projection-store-freeze`'s (HS-P0010) decision, recorded in the
  specification's own disposition for PS-36 — pinning a compile-fail diagnostic *"needs a
  `trybuild`-style stderr snapshot, which is a dependency decision phase 6 owns"*
  (`spec/SPECIFICATION.md:8772`, cited at `_decomposition.md:674-688`). This story is
  **specified and unbuildable** until that input arrives. It must be raised as an explicit
  input to HS-P0010 before this project's implementation reaches M6; if HS-P0010 declines it,
  **escalate — do not substitute a `compile_fail` doctest** (`_design.md:1257-1259`).
- *The `.stderr` fixture's exact text.* Cannot be authored until the instrument exists and the
  rewritten example's enum is final; the *requirement* on it — first error block's `-->` span
  into `examples/course-subscriptions/` — is fixed now.
- *Which test names the `ARTEFACTS` row lists.* Spec's, once the target exists. Discovery's
  constraint: the names must be the ones a reviewer would paste into `cargo test`, and the
  row must be defensible in the way the existing rows are — a deletion that would read as
  tidying up dead code.

**Not blocked on `MemoryProjectionStore`.**

## Decision

The problem this slice solves is that this project's central claim — *the compiler protects
your domain, so adding an event variant your fold does not handle is a build failure rather
than a production incident* — is exactly the kind of claim that is trivially satisfiable in
form and worthless in substance. A file that is *supposed* not to compile passes any check
that never compiles it, and this repository has already shipped one such artefact: phase 4's
was decorative because rustdoc collects doctests from the lib target only, so a
`compile_fail` case in `tests/` was never run at all. This story lands the real instrument —
a `trybuild` case that adds a variant to the rewritten example's domain enum and pins the
resulting diagnostic to the user's own `match` arm — plus the negative control that proves
the instrument can fail, plus the `ARTEFACTS` row that makes an emptied case fail a step that
today only a deleted one would. The spec for this story covers the compile-fail case's
contents, the `.stderr` fixture and the span requirement, the negative control's form and the
explicit act of *running* it to confirm it fails, the new `xtask/src/proof.rs` `ARTEFACTS`
row with the test names it pins and the justification for each, and the statement — carried
forward, not resolved — that the whole story is blocked on `trybuild` arriving from HS-P0010
and that the fallback is escalation rather than a doctest. No `[FROZEN]` clause is amended;
the case exercises `happenstance`'s own types, which no clause namespace covers.

## The wrong implementation

**The mutant: a `compile_fail` doctest under `crates/happenstance/tests/`.**

```rust
// crates/happenstance/tests/protection.rs
/// ```compile_fail,E0004
/// # use happenstance::*;
/// // a domain enum with an unhandled variant
/// ```
fn _protection() {}
```

It looks like the artefact. It is in a `tests/` directory, it is annotated with the error
code the protection produces, it is committed, and it is visible in review. `cargo test
--workspace --all-features` passes. `cargo xtask ci` is green. AC-002 reads as satisfied: a
compile-fail case exists and it names the protection.

It is wrong twice over, and each way is independently fatal. First, **rustdoc collects
doctests from the lib target only**, so a doctest in `tests/` is never executed — the case
does not pass, it is not run, and the negative control is not run either, so removing the
protection changes nothing and the "control" reports success. That is not a hypothetical:
phase 4's proof artefact was decorative for precisely this reason
(`RUNBOOK.md:3614-3627`). Second, even placed correctly in the lib target, **rustdoc on
1.97.1 silently ignores an error-code annotation it cannot match**, so `compile_fail,E0004`
passes on *any* compile failure — a typo in a type name, a missing import, a renamed
method. The `happenstance-core` doc comment at
`crates/happenstance-core/src/event.rs:95-106` says exactly this about the same construct,
in the tree, today. Under that spelling the negative control cannot discriminate: with the
protection removed the case still fails to compile for some other reason, and the control
"passes" while proving nothing.

The instrument that rejects it is `trybuild` with a pinned `.stderr` snapshot, and its
absence is the story's blocker rather than a licence to accept the weaker thing.

**A second mutant: registering the artefact as a `cargo test` step instead of an `ARTEFACTS`
row.** Adding `Step { name: "compile-fail protection", program: "cargo", args: &["test",
"-p", "happenstance", "--test", "ui"], … }` to `REQUIRED` is one line, runs in the gate, and
goes green. `cargo test` **exits 0 on `running 0 tests`**, so emptying `tests/ui/` — deleting
the cases while leaving the file — passes a step that deleting the file would fail. That
asymmetry is the entire reason the gate runs `cargo xtask proof-artefact` instead, and
`xtask/src/proof.rs`'s own module doc and row shape (`:57-70`) exist to name the tests that
must be present rather than trusting a target to contain any.

**A third mutant: a fixture whose span points into the library.** The `.stderr` snapshot is
generated by running `trybuild` and accepting whatever it produced — and what it produced
points at a macro expansion inside `crates/happenstance/src/`, because the protection fires
through a `const` assertion or a trait bound emitted there. The test passes: the snapshot
matches itself, forever. And the guarantee it certifies is one the user cannot act on,
because the error names a file they did not write (AC-U07; anti-pattern 13,
`_design.md:999-1001`). Accepting a generated snapshot without reading its `-->` line is how
this one ships, and it is why the span requirement is an acceptance criterion rather than a
review preference.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule; its
artefact is a compile-fail case and a `.stderr` snapshot, and nothing in it constructs,
observes or asserts a `SequencePosition` — the protection it pins is about event-type
exhaustiveness, not about the log. Ticked as vacuously true, checked rather than assumed.
**Frozen clauses:** nothing under `crates/happenstance-core/src/**` and nothing in
`spec/SPECIFICATION.md` is edited. The one clause adjacent to this work is PS-36, whose
disposition already records that the `trybuild` dependency decision belongs to phase 6 — that
is a constraint this story obeys by staying blocked rather than one it amends.

Note on the *"clear starting point"* box, ticked deliberately and not by reflex: this story
is blocked on an **input**, not on a decision. Everything a spec needs — the case's content,
the span requirement, the negative control's form, the `ARTEFACTS` row's shape and the
prohibition on substituting a doctest — is fixed and citable now, and the spec's job includes
recording the block and its escalation path. A story that cannot be *implemented* yet can
still be *specified* completely, and pretending otherwise would hide the dependency instead
of naming it.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
