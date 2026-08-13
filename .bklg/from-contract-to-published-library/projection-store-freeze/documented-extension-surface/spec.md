---
item: HS-S0015
stage: spec
created: 2026-08-12T13:46:09.930Z
updated: 2026-08-12T13:46:09.930Z
template_sig: 87bbf1d0
rendered_sig: 26796fc5
---

# Spec — DT-8's arm discharged — the suite's bar held for the author it was chosen for

## Scope lock

| Artefact | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — **DT-8** at *Open design tensions* (`:427`); **AC-04** and **AC-05** (`:317-322`) are the two promises this story tests against an author nobody here wrote; **DoD 7** (`:377-379`) is the scenario it moves toward |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG and scope seams; `userFacing: false`, so the designed surface here is a public API and a printed line, not a screen |
| Project charter | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` — **AC-007** (`:201-203`), **DR-08** (`:167-169`), and the two bars the outside fixture must clear: AC-002 (`:183-186`) and AC-005 (`:194-197`) |
| **This spec** | `.bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/spec.md` |
| Story discovery | `.bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/discover.md` — the signal ledger, the four deferred questions this spec answers, and the named wrong implementation |
| **Signed-off design (BINDING)** | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — `surfaces: []` and the no-screen determination are binding and not re-decided; the **two non-visual surfaces** it names (`:24-38`) — a type surface and a text surface — are exactly what this story hands to an outsider. **DT-8's arm is read out of this file**, written there by `projection-api-design-record` (HS-S0014) |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — **UX brief AC-U02** (`:97-108`, the cost-to-an-outside-author bar), **Architecture brief Note 4** (`:462-472`, the write seam and the coherence argument) and **Note 9** (`:689-693`, DT-8's blast radius), **Testing brief row AC-007** (`:777`, Integration tier, conditional on the arm) |
| Grounding | `.bklg/from-contract-to-published-library/projection-store-freeze/_grounding.md` |
| Story map | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md` — slice `outside-author-extension-surface`, merge position **7 of 8** (`:67`, `:113`) |
| Roadmap pointer | `RUNBOOK.md:3848-3965` (phase 6 in full); `RUNBOOK.md:3882-3892` (the fixture contract and the bar `CheckpointOnlyStore` sets) |
| Specification (wins on conflict) | `spec/SPECIFICATION.md:4998-5031` — `ProjectionProbe` given verbatim, and the coherence argument for its home, including the sentence this story is the only thing in the tree that can falsify (`:5019-5025`) |

## One-line PR slice

DT-8's outside-author arm, discharged as chosen: if the bar is held for an author nobody here
supervises, the documented pair `projection_store_conformance!` + `ProjectionProbe` is written up as
an extension surface and a fixture built **from that documentation alone — not copied from
`crates/happenstance-testkit/src/fixtures.rs`** — clears the mutant-registry exactness check and the
capability-skip rule; if the narrower arm was taken, the recorded scope is shown to match what the
suite actually holds implementers to.

## Executive summary

This PR lands the **first use of this project's own output by someone who did not build it** — and,
because there is no such person, the closest honest simulation of one: a projection adapter written in
a crate positioned so the orphan rule, the feature flags and the dependency graph all behave as they
would for a stranger, built by reading the rendered documentation rather than the reference fixture.

The delta over what merges before it is narrow and specific. By merge position 6 the projection suite
exists, two structurally unlike batch shapes pass it, and every rule has a registered mutant
(`_storymap.md:112`). Every fixture that has run it was written by the people who wrote the rules,
inside crates that already depend on `happenstance-testkit`. So one claim in this project is entirely
untested at that point: that the *documented* surface — `projection_store_conformance!`,
`ProjectionProbe`, the projection fixture trait and the `__private` re-export the macro expansion
names — is sufficient on its own. This story tests it, and treats **where the documentation ran out**
as the deliverable rather than as an inconvenience.

It is also the only place in the workspace that can convict Architecture brief AC-A02. The
specification's argument for putting `ProjectionProbe` in `happenstance-core` behind `conformance`
rather than in the testkit is a claim about a crate boundary that does not exist here
(`spec/SPECIFICATION.md:5019-5025`). Move the trait into the testkit and every check in this workspace
stays green, because every fixture in it already has the testkit in scope. The first thing that fails
is this story's outside crate. The same is true of the `__private` export block
(`crates/happenstance-testkit/src/lib.rs:359-364`) — omit it and every in-workspace harness still
compiles.

Why it is a `capability` and not documentation polish: the deliverable is a running, gate-green
conformance invocation from a crate that reaches the suite through nothing but its published surface,
plus a written record of every gap that reaching required. Both halves are observable in the diff and
in one `cargo xtask ci` run.

## Context pack

Everything below is a decision already made. Read this section before writing anything; the anchors in
the second half of this spec are for retrieval once you are inside the work, not for orientation.

### 1. You read the arm. You do not choose it.

DT-8 asks whether the adapter author the acceptance criteria address is only this project's own team
or an outside author too (`initiative.md:427`). **The answer is recorded in
`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md`, by
`projection-api-design-record` (HS-S0014), and it is binding here.** That file is this story's mount
of record for scope: HS-S0014's own spec states the obligation both ways and says outright that
HS-S0015 must be scopable from `_design.md` alone.

- **Arm A — outside-author.** The full exercise: the extension surface is documented, an outside
  fixture is built from that documentation, and it clears project AC-002's and AC-005's bars.
- **Arm B — narrow.** The deliverable is the **published scope statement**: the suite's bar is stated
  as internal-for-now *where an outsider meets it* — the testkit's own crate documentation — and shown
  to match what the suite enforces. Recording it only in `_design.md` is the failure DT-8 names, because
  `_design.md` is a backlog artefact and the person it protects is reading docs.rs
  (`_decomposition.md:103-107`).

If `_design.md` carries no DT-8 resolution when this story starts, **stop and report an unmet
dependency**. Choosing the arm here would make the design sign-off a document about a decision
somebody else made afterwards.

The rest of this pack is written for Arm A, because Arm A is the larger obligation and the one whose
details cannot be improvised. Arm B's obligations are stated in full in *Behavior and interfaces*.

### 2. The claim being tested is countable, so count it

The specification's cost claim is exact, and exactness is what makes it checkable: putting
`ProjectionProbe` in the contract crate behind `conformance` "costs one feature flag on a dependency
the adapter already has, and no new edge in the graph", where the alternative "forces the adapter to
take a non-dev dependency on `happenstance-testkit` and gate it behind a feature"
(`spec/SPECIFICATION.md:5019-5025`).

The claim is about the **non-dev** graph — what a consumer of the adapter pulls in. It is not falsified
by the outside crate taking `happenstance-testkit` and `tokio` as *dev*-dependencies; the testkit's own
crate doc already tells an author they need `tokio` with `macros` and `rt` in `dev-dependencies`
(`crates/happenstance-testkit/src/lib.rs:28-29`). Say this in the record, because a naive reading of
"no new edge" reports a false failure.

### 3. Where the outside crate lives, and why that location makes the exercise real

**Decision: a new workspace member at `examples/outside-projection-adapter/`, `publish = false`,
shaped after `examples/course-subscriptions/Cargo.toml`.** The workspace members glob is
`["crates/*", "examples/*", "xtask"]` (`Cargo.toml:3`), so the crate is picked up with no root-manifest
edit, and `cargo test --locked --workspace --all-features` (`xtask/src/main.rs:143-153`) runs its tests
inside the ordinary gate rather than as a separately maintained subset.

Not `crates/*`: that glob is the library's own crates, and a stranger's adapter is not one of ours. Not
`experiments/`: that tree is deliberately outside the gate (`CLAUDE.md`, *Repository map*), and this
must run in it.

The geometry that matters is inside the crate, not the workspace:

- The store type and **both** `impl ProjectionStore for …` and `impl ProjectionProbe for …` live in
  `src/lib.rs`, where the type is local.
- Only the `ProjectionFixture` impl and the `projection_store_conformance!` invocation live in
  `tests/`, where the fixture type is defined right there and so *is* local.
- Writing the probe impl in `tests/` is rejected by the orphan rule whatever crate the trait lives in,
  because the store type is foreign there. That is the whole argument: the probe impl must be reachable
  from the adapter's **library**, which means from its non-dev dependencies.

### 4. Workspace membership is a stated limit of the exercise, not a cheat

A real outsider depends on a published version; a workspace member depends by path and shares
`[workspace.dependencies]`. Nothing is published yet, so a registry dependency is impossible, and
initiative **DoD 9** — "a stranger can install it" from a scratch project outside this workspace
(`initiative.md:383-386`) — belongs to a different project and is explicitly not claimed here. Record
the limit in the findings note. The properties this position *does* reproduce faithfully are the ones
the claim is about: the orphan rule, the feature flags, and the non-dev dependency graph.

### 5. "From the documentation alone" is procedural, and the procedure is the deliverable

No tool distinguishes a fixture written from documentation from the same fixture copied out of
`fixtures.rs` — the resulting code is the same code (`discover.md:79-91`). The constraint therefore
takes its strongest available form, which is a named allowlist, a named denylist, and a written record.

**Allowlist — read as *rendered* rustdoc (`cargo doc --all-features --open`), not as source:**
the testkit crate documentation (`crates/happenstance-testkit/src/lib.rs`'s `//!` block), the
`projection_store_conformance!` macro docs, the projection fixture trait's rustdoc in
`crates/happenstance-testkit/src/contract.rs`, `Capability` and `RuleOutcome`'s rustdoc
(`contract.rs:355-433`, `:458-537`), and `ProjectionProbe`'s rustdoc on
`crates/happenstance-core/src/projection.rs`.

**Denylist — not opened while the fixture is being written:**
`crates/happenstance-testkit/src/fixtures.rs` foremost (the owned-handle reference implementation at
`:243-292` is precisely what an author would copy), `MemoryProjectionStore`'s source, the in-tree
buffering conformant variant landed by `buffering-conformant-variant` (HS-S0013), and the testkit's own
`tests/*conformance*.rs` harnesses.

**Every point at which the allowlist was insufficient is a gap**, recorded with: what was missing, which
denylisted file or which source read answered it, and the documentation change that would have
prevented it. Gaps are the exercise's primary output. A run that finds no gaps is recorded as such,
explicitly — "a fixture built with no gaps found is either a triumph or a copy, and only the record
tells you which" (`discover.md:89-91`).

### 6. What the outside fixture must actually demonstrate, and the one thing it probably cannot

Project AC-007 requires AC-002's and AC-005's bars demonstrated against this fixture
(`project.md:201-203`). Concretely, three demonstrations, all inside the outside crate:

1. **Conformant.** The outside store passes the whole projection suite through
   `projection_store_conformance!` — initiative AC-04, "told when they are finished".
2. **Discriminating.** A second, deliberately wrong store in the same crate — an outside author's own
   checkpoint-only analogue — **fails by name**, and the failing test's name is recorded. This is
   AC-002's discipline reached from the documented surface rather than from the testkit's private
   harness.
3. **Declining.** The outside fixture declines one projection capability with its own stated reason,
   and the skip is asserted on the **`RuleOutcome` value** returned by the rule function, never on
   stdout — AC-005's instrument, and the reason the projection rules module must be publicly reachable
   the way `pub use suite::rules;` (`crates/happenstance-testkit/src/lib.rs:189`) makes the event-store
   family reachable today.

**The honest limit, and it is expected to become a finding.** The exactness meta-tests themselves —
`mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive` — live in the testkit's
own test binary (`crates/happenstance-testkit/tests/mutation_coverage.rs:2889`, `:2754`), over a
`Declared`/`Kind` registry that is private to it (`:120-186`). An outside author cannot invoke that
machinery. If that proves true, it is a **gap of the first kind**: the mutant-registry discipline is
not part of the documented extension surface, and saying so is a better outcome than quietly
re-implementing 3,500 lines in an example crate. Do not register the outside fixture in the testkit's
own registry to work around it — a `happenstance-testkit` dev-dependency on the example crate inverts
the very edge this story exists to count.

### 7. Gaps are fixed in the same change where the fix is publishing; otherwise reported

`CLAUDE.md`'s rule is that if a rule seems wrong you fix it and say why in the same change. Applied
here: a gap whose fix is **documentation** (a missing sentence, an unexplained constant, an example
that does not compile) or a **missing public re-export** the macro expansion needs is fixed in this PR,
with the gap preserved in the record. A gap whose fix is a **port change, a new conformance rule, or a
`[FROZEN]` clause amendment** is reported at the story boundary and not absorbed
(`_decomposition.md:709-727`, *What would make this brief wrong*).

Note the semver consequence before touching the testkit's public surface: `happenstance-testkit`
carries its own `version` key precisely because its number is a promise about the bar rather than about
types (`spec/SPECIFICATION.md:8200-8222`, CF-32 `[FROZEN]`; `crates/happenstance-testkit/Cargo.toml`,
the `version` comment). A newly `pub` item is a MINOR event and belongs in `CHANGELOG.md`.

### 8. The persona slice this realises

**P2, the adapter author** (`_discovery/distillation/personas-and-journeys.md:114-153`) wants an
executable definition of "correct" they can run against their own storage, and fears "that the port
quietly assumed something their storage cannot provide, discovered late". The persona's own text is
explicit that today this is exclusively the project's own team, and that the constraint is general:
"whoever eventually writes a seventh adapter needs the same thing the first six needed"
(`:118-121`). This story is the only place in the project where that fear is tested rather than
assumed away (`discover.md:29`).

### 9. Anti-patterns, carried in from the binding constraints

No `#[async_trait]` anywhere in the outside crate (ADR-0001; it injects `+ Send` and forecloses
wasm32). Bind the bare `ProjectionStore` flavour, not the `Send` one, and import only one of the two
names per module (`CLAUDE.md`, binding constraint 4). No borrowing GAT on the fixture — `type
Store<'a> where Self: 'a` is one of five ingredients of a rustc ICE this repository has already
minimised and which still reproduces on 1.97.1 (`crates/happenstance-testkit/src/contract.rs:97-111`).
No literal position assertions anywhere (`CLAUDE.md`, *The rule that matters*), which here is a
constraint on the outside crate's own tests as much as on any rule. And no `_design.md` edit: this
story reads that file, it does not write it.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice through every layer, where the user is P2 and the observable is a green conformance run plus a written gap record (`_storymap.md:67`) |
| **Slice / milestone** | `outside-author-extension-surface`, merge position 7 of 8 (`_storymap.md:113`). **Slice-mates: none** — this story is the sole story in its slice, so "implemented together in one context" is satisfied trivially and there is no sibling to integrate against |
| **Mount point** | `crates/happenstance-testkit/src/lib.rs` — the crate-level `//!` documentation block (`:8-135`), which is the page an outsider meets on docs.rs and where the extension surface is either written up (Arm A) or scope-limited (Arm B), together with the `__private` re-export block at `:359-364` that the macro expansion must be able to name from a foreign crate. Both arms mount here; neither mounts in `_design.md` alone |
| **Second mount (Arm A only)** | `examples/outside-projection-adapter/Cargo.toml` and `examples/outside-projection-adapter/tests/`, picked up by the workspace members glob at `Cargo.toml:3` and run by the gate's `tests` step (`xtask/src/main.rs:143-153`). Not constructed-but-unmounted: the crate is in the workspace, its tests run in `cargo xtask ci`, and its manifest is the evidence for the cost claim |
| **Wires into** | `crates/happenstance-core/src/projection.rs` — `ProjectionProbe` behind `feature = "conformance"` and the `ProjectionStore` port itself (landed by HS-S0004 / HS-S0005); `crates/happenstance-core/src/lib.rs:96-124` — the export block and its `#[cfg_attr(docsrs, doc(cfg(…)))]` pattern; `crates/happenstance-testkit/src/contract.rs` — the projection fixture trait plus `Capability` (`:355-433`) and `RuleOutcome` (`:458-537`), both reused unchanged (Architecture brief AC-A06); `projection_store_conformance!` and the emitter it defaults to (`crates/happenstance-testkit/src/registry.rs`); `examples/course-subscriptions/Cargo.toml` as the `publish = false` manifest precedent |
| **Renders surfaces** | **None in the `surfaces:` manifest sense** — `_design.md:48-50` declares `surfaces: []` and that determination is binding. This story *consumes* the two non-visual surfaces that file names (`:24-38`): the **type surface** (`ProjectionStore`, `ProjectionProbe`, the batch vocabulary) and the **text surface** (``SKIP {rule}: fixture declines `{capability}` — {reason}``, `contract.rs:500-507`). It changes neither; it is the first reader to meet both from outside |
| **Conformance rule(s)** | **Adds none.** It runs the projection rule set enumerated by `for_each_projection_store_rule!` (landed by `projection-suite-entry-point`, HS-S0007) against a new fixture, and asserts on `RuleOutcome` values for the declined capability. If the exercise shows a rule to be wrong, that is reported at the story boundary, not patched here (§7 above) |
| **Clause(s)** | Discharges no clause and amends none. It is the first **in-tree falsifier** for `spec/SPECIFICATION.md:5019-5025` — the coherence argument for `ProjectionProbe`'s home — which is a claim about a crate boundary nothing else in this workspace can test. No `[FROZEN]` clause is touched; PS-1 – PS-37 disposition belongs to `unstable-projection-gate-and-clause-disposition` (HS-S0016) |
| **Advances DoD scenario** | Initiative **DoD 7** — "The projection suite discriminates" (`initiative.md:377-379`) — by showing the discrimination is reachable by the author it is stated for, which is the public half of initiative **AC-04** and **AC-05** (`:317-322`). It completes project **AC-007** (`project.md:201-203`) and is the second half of project DoD 6, whose first half is HS-S0014's recorded resolution. It explicitly does **not** advance DoD 9 (a stranger installs from the registry) — see Context pack §4 |

## PR boundary

The narrowest set that is honestly true. `redkiln verify --grain story` reads the first fenced block
below and fails on any file changed outside it. It is deliberately wider than the files this story
*expects* to touch, because the mount points are shared and a gap fixed in the same change is the
documented policy (§7) — not because the boundary is a place to hide scope.

```
examples/outside-projection-adapter/**
crates/happenstance-testkit/src/lib.rs
crates/happenstance-testkit/src/contract.rs
crates/happenstance-core/src/projection.rs
CHANGELOG.md
CLAUDE.md
.bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/**
```

**In this PR**

- **Arm A** — a new workspace member `examples/outside-projection-adapter/`: manifest (`publish = false`,
  `happenstance-core` with `conformance` under `[dependencies]`, `happenstance-testkit` and `tokio`
  under `[dev-dependencies]` only), a store implementing `ProjectionStore` + `ProjectionProbe` in
  `src/lib.rs`, a second deliberately wrong store beside it, and `tests/` carrying the fixture impl,
  the `projection_store_conformance!` invocation, the named-failure demonstration and the
  `RuleOutcome`-value assertion for the declined capability.
- **Arm A** — the extension-surface write-up in `crates/happenstance-testkit/src/lib.rs`'s crate doc:
  what an outside author needs, in what order, with the feature flag named and the `__private` export's
  role stated.
- **Arm B** — the published scope statement in the same crate doc, plus the correspondence record
  showing it matches the enumerated rule set and capability set.
- **Both arms** — `_extension-surface-gaps.md` in this story's own folder: the gap record, the named
  allowlist and denylist, the workspace-membership limit, the manifest-diff evidence, and the
  `error[E0117]` transcript from the counterfactual probe placement.
- **Both arms** — documentation-shaped gap fixes in the mounted files, each cited from the gap record;
  a `CHANGELOG.md` entry if any testkit public item became `pub`; one line in `CLAUDE.md`'s repository
  map if Arm A adds the example crate, because a map that omits a workspace member is a map that lies.
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- **Any edit to `_design.md`.** The arm is read from it. Amending it here would make the design
  sign-off retroactive (`project.md` DoD 6; HS-S0014 owns that file).
- **Any new conformance rule, any change to the projection rule set, and any change to the mutant
  registry** — `projection-mutant-registry` (HS-S0009) and the rule stories own those, and a rule added
  here would arrive without the mutant CF-1 requires.
- **`spec/SPECIFICATION.md`** — no clause edited, no maturity marker moved; HS-S0016 owns clause
  disposition, and the `unstable-projection` exposure verdict is HS-P0016's (`project.md:129-131`).
- **Any `.kb/**` atom.** A finding that deserves an ADR is recorded as a named gap for the runbook's
  ADR pass, never minted as a side effect of a story.
- **Any port change to `ProjectionStore` or `ProjectionProbe`'s shape.** HS-S0004 and HS-S0005 own the
  signatures; a gap requiring a signature change is reported, not absorbed.
- **A registry-install demonstration.** Initiative DoD 9 is `publication-and-positioning`'s.
- **Registering the outside fixture in `crates/happenstance-testkit/tests/mutation_coverage.rs`** —
  it would invert the dependency edge this story exists to count (§6).

**Merge DoD (one line):** the arm recorded in `_design.md` is discharged where an outsider meets it —
Arm A with a workspace-member fixture that reaches the suite through nothing but the published surface,
passes it, demonstrates a named failure and a reasoned skip asserted on `RuleOutcome` values, and
carries manifest evidence of zero new non-dev edges; Arm B with a published scope statement shown to
match the enforced rule set — and in both cases with the gap record written, `cargo xtask ci` green,
and `redkiln verify --grain story` green on the boundary above.

## Behavior and interfaces

Rows marked **A** are live only on the outside-author arm, **B** only on the narrow arm, and **A+B** on
both. The arm-conditional rows are not optional work: on the arm that is not in force, each is
discharged by citing `_design.md`'s recorded resolution as the evidence that it does not apply, and the
ledger says so. Nothing is left blank.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **A+B · The arm is read, and the story halts if it is absent** | The implementer's first act is to read `_design.md`'s DT-8 resolution and record which arm is in force in this story's `_ledger.md`. No resolution present ⇒ stop and report an unmet dependency on HS-S0014; do not choose. | `.bklg/…/projection-store-freeze/_design.md`; `project.md:201-203`; `_storymap.md:113` ("scope depends on which DT-8 arm slice 1 recorded") |
| **A+B · Whatever is published lands where an outsider meets it** | The deliverable's home is `crates/happenstance-testkit/src/lib.rs`'s crate documentation — the docs.rs page — not a `.bklg/` artefact. Arm A publishes the extension surface there; Arm B publishes the scope limit there. Recording either only in `_design.md` is the failure DT-8 names. | UX brief **AC-U02** (`_decomposition.md:103-107`); `discover.md:108-115`; `crates/happenstance-testkit/src/lib.rs:8-135` |
| **A · The outside crate exists in a position where the orphan rule behaves as it would for a stranger** | New workspace member `examples/outside-projection-adapter/`, `publish = false`, picked up by the members glob with no root-manifest edit. Store type and **both** trait impls in `src/lib.rs`; fixture impl and suite invocation in `tests/`. | `Cargo.toml:3`; `examples/course-subscriptions/Cargo.toml` (the `publish = false` precedent); `spec/SPECIFICATION.md:5019-5025` |
| **A · The suite is reached through the published surface and passes whole** | `projection_store_conformance!(OutsideFixture::new())` expands in a foreign crate — which is what exercises the `__private` re-export — and every rule passes, inside the gate's own `tests` step rather than a bespoke command. | `crates/happenstance-testkit/src/lib.rs:333`, `:359-364`; `xtask/src/main.rs:143-153`; initiative **AC-04** (`initiative.md:317-319`) |
| **A · A wrong store written by the same outside author fails by name** | A second store in the crate commits the checkpoint and drops the read-model write; the suite fails it, and the **name of the failing test** is recorded in the gap note. This is project AC-002's discipline reached from documentation, not from the testkit's private harness. | `project.md:183-186`; `RUNBOOK.md:3890-3892` ("if it passes, the port is not frozen"); `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| **A · A declined capability is asserted on the value, never on stdout** | The outside fixture declines one projection capability with its own reason; a rule requiring it is still emitted as a test and returns `RuleOutcome::Skipped { capability, reason }`, asserted directly. Stdout is *additionally* legible under the gate's `--show-output`, and that is not the check. | `crates/happenstance-testkit/src/contract.rs:458-537`, `:500-507`; `xtask/src/main.rs:132-153`; `project.md:194-197`; initiative **AC-05** (`initiative.md:320-322`) |
| **A · The projection rules module is publicly reachable, or that is gap #1** | The value assertion above requires calling a rule function directly from a foreign crate, exactly as `pub use suite::rules;` permits for the event-store family. If the projection family is not re-exported equivalently, the missing `pub use` is a documented-surface gap and is fixed here (a MINOR event on the testkit's own version, with a `CHANGELOG.md` entry). | `crates/happenstance-testkit/src/lib.rs:189`; `spec/SPECIFICATION.md:8200-8222` (CF-32); `spec/SPECIFICATION.md:8141-8143` (CF-29, for the changelog discipline) |
| **A · The cost claim is counted, not asserted** | Evidence is the manifest plus `cargo tree -p outside-projection-adapter --edges normal`: `happenstance-core` gains **one feature** (`conformance`) on a dependency the crate already has; `happenstance-testkit` appears **only** under `[dev-dependencies]`; the normal-edge tree contains no `happenstance-testkit` node. The claim is about the non-dev graph and the record says so, so a dev-only `tokio` edge is not read as a falsification. | `spec/SPECIFICATION.md:5019-5025`; UX brief **AC-U02**; `crates/happenstance-testkit/src/lib.rs:28-29` (the documented `tokio` dev-dep cost) |
| **A · AC-A02's placement is convicted by transcript, not by assertion** | The probe impl is written in `tests/` once, deliberately; the resulting `error[E0117]` is captured verbatim and quoted in the gap note; the impl is then moved to `src/lib.rs` where it belongs. The permanent artefact is the transcript and the prose, never a broken build — the same discipline `references/adapter-shapes.md` uses for the six skeletons. | `references/adapter-shapes.md`; `spec/SPECIFICATION.md:5019-5025`; `discover.md:93-106` |
| **A+B · The reading discipline is named before the work, not after** | The allowlist (rendered rustdoc only) and denylist (`crates/happenstance-testkit/src/fixtures.rs` foremost, `MemoryProjectionStore`'s source, HS-S0013's buffering variant, the testkit's own conformance harnesses) are written into the gap note **before** the fixture is written. A denylist produced afterwards describes what happened; one produced first constrains it. | `_storymap.md:67` ("not copied from `fixtures.rs`"); Testing brief row **AC-007** (`_decomposition.md:777`); `crates/happenstance-testkit/src/fixtures.rs:243-292` |
| **A+B · Gaps are the primary output, and "none" is a finding** | Each gap records what was missing, what answered it, and the documentation change that would have prevented it. A no-gap run is stated explicitly together with the discipline that makes it credible, because "a fixture built with no gaps found is either a triumph or a copy, and only the record tells you which". | `discover.md:77-91`; `_decomposition.md:709-727` |
| **A+B · Fix what publishing can fix; report what it cannot** | Documentation gaps and missing public re-exports are fixed in this PR with the gap preserved in the record. A gap requiring a port change, a new conformance rule, or a `[FROZEN]` clause amendment is reported at the story boundary as a finding for HS-S0016 / the runbook's ADR pass. | `CLAUDE.md` (*The rule that matters*, and *Where the work lives*); `_decomposition.md:709-727`; `project.md:129-138` |
| **A · The exactness meta-tests' unreachability is expected and is recorded, not routed around** | `mutants_fail_exactly_their_declared_rules` and `mutant_registry_is_exhaustive` run over a `Declared`/`Kind` registry private to the testkit's test binary. If an outside author cannot invoke them, that is a first-class gap. Registering the outside fixture in that binary is forbidden: it inverts the edge this story counts. | `crates/happenstance-testkit/tests/mutation_coverage.rs:120-186`, `:2754`, `:2889`; `project.md:183-186` |
| **B · The published scope statement says what the bar is, and for whom** | One paragraph in the testkit crate doc: the suite's bar is currently held for adapters written in this workspace; what an outside author may and may not conclude from a green run; and where the boundary would move. It reads as a qualification, not an apology. | `discover.md:108-115`; `initiative.md:427`; `crates/happenstance-testkit/src/lib.rs:8-135` |
| **B · The recorded scope is shown to match what the suite enforces** | A correspondence record: the enumerated projection rule set and the fixture capability set, each mapped to the sentence in the scope statement that covers it, with any rule the statement does not cover named. The Testing brief calls this row **Static**; the instrument is the design review, since nothing compiles a crate doc's honesty. | Testing brief row **AC-007** (`_decomposition.md:777`); `discover.md:113-115` |
| **A+B · Nothing about the port, the rules or the clause ledger moves** | No signature change, no rule added or edited, no mutant registered, no clause marker touched, no `.kb/` atom written, no `_design.md` edit. The diff is an example crate, crate documentation, a changelog line, a repository-map line, and this story's own folder. | The PR boundary block above; `project.md:115-141` (project-level out of scope) |
| **A+B · The gate is the whole check, run whole** | `cargo xtask ci` green — including the `tests` step that runs the new crate, `cargo hack check --workspace --feature-powerset --no-dev-deps`, which now includes the example crate's own features, and the doc build. `--fast` is not sufficient for this story's merge, because the widened powerset is one of the steps it omits. | `xtask/src/main.rs:143-153`, `:546-556`; `CLAUDE.md` (*Commands*) |

## Data and migrations

**N/A.** This story persists nothing and migrates nothing.

Three things could be mistaken for data and are not. The outside store's read model is in-process
state owned by the example crate for the duration of one test — no file, no schema, no serialisation
format; `happenstance-core` carries no `serde` in its default features by binding constraint
(ADR-0003) and nothing here changes that. The manifest and lock-file movement from adding a workspace
member is a build-graph change, evidence for the cost claim, and covered by the PR boundary rather
than by a migration plan — note only that the gate's `cargo test` step runs `--locked`
(`xtask/src/main.rs:147`), so `Cargo.lock` must be committed with the new member in it. And the gap
record is a markdown note in this story's own folder, an artefact rather than state.

The one thing with a migration *shape* is the crate documentation this story amends: it is edited
additively, and any newly `pub` testkit item is a semver-MINOR event recorded in `CHANGELOG.md` under
CF-32's reasoning that the testkit's number is a promise about the bar
(`spec/SPECIFICATION.md:8200-8222`).

## Acceptance criteria

The persona is **P2, the adapter author**
(`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:114-153`):
they want an executable definition of "correct" they can run against their own storage, and they fear
finding out late that the port assumed something their storage cannot give. Every criterion below is
written from that crossing — what P2 reaches for, from outside, and what they get back.

Rows tagged **A** are live on the outside-author arm and rows tagged **B** on the narrow arm; a row on
the arm not in force is discharged by citing `_design.md`'s recorded resolution as evidence that it
does not apply, and the ledger row says exactly that. No row is deleted and none is left blank.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **A+B ·** GIVEN P2's protection rests on a scope decision a human signed off, WHEN the implementer's *first* act is to open `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` before writing any code, THEN the DT-8 arm in force is recorded — verbatim, with a `file:line` citation into that file — in this story's `_ledger.md` and at the head of `_extension-surface-gaps.md`, and every arm-conditional row below is discharged on that arm's terms; and IF `_design.md` carries no DT-8 resolution, THEN the story halts and reports an unmet dependency on `projection-api-design-record` (HS-S0014) rather than choosing an arm, because a sign-off that ratifies a decision the implementer already made is not a sign-off. | Reviewed diff over `.bklg/…/documented-extension-surface/_extension-surface-gaps.md` and the arm citation in `_ledger.md`; `redkiln verify --grain story` (the ledger must carry the citation before any other row may flip). No code test — the check is that the decision was *read*, and only a diff can show that. |
| **AC-002** | **A+B ·** GIVEN P2 meets this library on `docs.rs` and never opens a `.bklg/` artefact, WHEN they land on `happenstance-testkit`'s crate page, THEN the arm's deliverable is *there* as a composed section of the crate documentation — its own `#` heading taking its place in the existing six-heading hierarchy (`crates/happenstance-testkit/src/lib.rs:15`, `:31`, `:55`, `:84`, `:134`, `:154`), the extension surface written up in ordered steps that name the `conformance` feature flag and the `__private` export's role (Arm A) or the scope limit stated as a qualification rather than an apology (Arm B) — and it renders under `cargo doc --all-features` with every intra-doc link resolving and every code block compiling; a resolution recorded only in `_design.md` **fails** this criterion, which is precisely the failure DT-8 names. | The gate's `docs` step plus `cargo test --doc -p happenstance-testkit` (links resolve, examples compile); the nightly `--cfg docsrs` rustdoc step for the rendered page; and the design-stage review reading the rendered page against `_design.md:24-38`, since nothing compiles a crate doc's honesty (`_decomposition.md:777`, the Static half of the Testing brief's AC-007 row). |
| **AC-003** | **A ·** GIVEN P2 has a projection store and wants to know when they are finished, WHEN they add `happenstance-core` with `conformance` to `[dependencies]`, `happenstance-testkit` and `tokio` to `[dev-dependencies]` only, and write the single line `projection_store_conformance!(OutsideFixture::new())` in their own `tests/`, THEN the macro expands inside a foreign crate — which is the only thing in this workspace that exercises the `__private` re-export (`crates/happenstance-testkit/src/lib.rs:359-364`) — one test per registered projection rule is emitted, and **every one passes**, inside the gate's ordinary `tests` step (`xtask/src/main.rs:143-153`) rather than a bespoke command, with no fork of the harness, no vendored copy of a rule and no edit to the rule set. | `examples/outside-projection-adapter/tests/outside_projection_conformance.rs`, run by `cargo test --locked --workspace --all-features` inside `cargo xtask ci`; the crate's presence in the run is evidence in itself, because `Cargo.toml:3`'s members glob picks it up with no root-manifest edit. **B:** discharged by citing `_design.md`'s narrow-arm resolution. |
| **AC-004** | **A ·** GIVEN P2's real fear is that "it compiles" gets mistaken for "it is correct" (`personas-and-journeys.md:129-135`), WHEN a second store *in the same outside crate* commits the checkpoint and silently drops the read-model write — an outside author's own analogue of `CheckpointOnlyStore` — and is run through the same documented entry point, THEN the suite fails it **by a named rule**, that failing test's name is written into `_extension-surface-gaps.md`, and the failure is reached from the published surface rather than from the testkit's private mutant machinery; and IF the exactness meta-tests (`mutants_fail_exactly_their_declared_rules`, `mutant_registry_is_exhaustive`) prove unreachable from outside, THEN that unreachability is recorded as a first-class gap and **not** worked around by registering the outside fixture in `crates/happenstance-testkit/tests/mutation_coverage.rs`, which would invert the dependency edge this story exists to count. | `examples/outside-projection-adapter/tests/outside_projection_discrimination.rs` — a `#[should_panic]`-shaped or explicitly-caught run naming the rule — plus the recorded rule name in the gap note; cross-checked against `project.md:183-186` (AC-002's bar) and `.kb/decisions/0010-the-suite-must-prove-itself.md`. **B:** discharged by citation. |
| **AC-005** | **A ·** GIVEN P2's storage genuinely cannot provide one guarantee and they need to say so without opting out of the bar, WHEN their fixture declines exactly one projection capability with its own stated reason and the suite runs, THEN the rule requiring it is **still emitted as a test**, returns `RuleOutcome::Skipped` carrying the fixture's own words, and the outside crate asserts on that **value** (`crates/happenstance-testkit/src/contract.rs:458-537`) — never on captured stdout — which requires the projection rules module to be publicly reachable from a foreign crate the way `pub use suite::rules;` (`lib.rs:189`) makes the event-store family reachable; the rendered `SKIP {rule}: fixture declines …` line (`contract.rs:500-507`) additionally appears under the gate's `--show-output` (`xtask/src/main.rs:132-153`) and is legibility, not the check; and IF that re-export is missing, THEN it is recorded as gap #1 and added in this PR with a `CHANGELOG.md` entry under CF-32's MINOR reasoning. | `examples/outside-projection-adapter/tests/outside_projection_capability_skip.rs` asserting the returned `RuleOutcome` value, with the reason compared against the fixture's own `const` rather than a literal repeated in the test; the printed line observed in the same `cargo xtask ci` run's output. Bar: `project.md:194-197`, initiative **AC-05** (`initiative.md:320-322`). **B:** discharged by citation. |
| **AC-006** | **A ·** GIVEN P2 must decide whether conforming is affordable before they start, WHEN they read the outside crate's manifest and run `cargo tree -p outside-projection-adapter --edges normal`, THEN what they find is exactly what the specification promised — **one feature** (`conformance`) added to a dependency the adapter already has, **no `happenstance-testkit` node anywhere in the normal-edge graph**, and the testkit present only under `[dev-dependencies]` — with the gap note stating plainly that the claim is about the *non-dev* graph, so the documented `tokio` dev-dependency cost (`crates/happenstance-testkit/src/lib.rs:28-29`) is not misreported as a falsification; and the counterfactual is convicted by transcript rather than assertion: the probe impl is written in `tests/` once, deliberately, the resulting `error[E0117]` is captured verbatim into the note, and the impl then lives in `src/lib.rs` where the store type is local. | The committed `examples/outside-projection-adapter/Cargo.toml`, the `cargo tree --edges normal` output pasted into `_extension-surface-gaps.md`, and the quoted `error[E0117]` transcript — the same evidence discipline `references/adapter-shapes.md` uses for the six skeletons. Claim under test: `spec/SPECIFICATION.md:5019-5025`; bar: UX brief **AC-U02** (`_decomposition.md:97-108`). **B:** discharged by citation. |
| **AC-007** | **A+B ·** GIVEN no tool can distinguish a fixture written from documentation from the same fixture copied out of `crates/happenstance-testkit/src/fixtures.rs`, WHEN the work begins, THEN `_extension-surface-gaps.md` **already** carries the named allowlist (rendered rustdoc only) and denylist (`fixtures.rs` foremost, `MemoryProjectionStore`'s source, HS-S0013's buffering variant, the testkit's own conformance harnesses) — a denylist written afterwards describes what happened, one written first constrains it — and WHEN the work ends, THEN every point at which the allowlist was insufficient is recorded with what was missing, which denylisted file or source read answered it, and the documentation change that would have prevented it; including the workspace-membership limit and the explicit non-claim on initiative DoD 9 (`initiative.md:383-386`), and including the standing tension that the crate doc today points an author straight at the reference fixture (`crates/happenstance-testkit/src/lib.rs:52-53`), which is the file the denylist forbids; and a run that found **no** gaps says so in those words, because "a fixture built with no gaps found is either a triumph or a copy, and only the record tells you which" (`discover.md:89-91`). | Reviewed diff over `_extension-surface-gaps.md`, checked against the commit order (the allowlist/denylist commit precedes the fixture commit — the ordering *is* the check, exactly as AC-008's ADR ordering is in `_decomposition.md:778`); the Testing brief's AC-007 row (`_decomposition.md:777`) names the negation "not copied from `fixtures.rs`" as the criterion. |
| **AC-008** | **A+B ·** GIVEN this repository's rule that a wrong rule is fixed with its reason in the same change, WHEN a gap is found, THEN a **documentation** gap or a **missing public re-export** is fixed in this PR and cited from the gap record — with `CHANGELOG.md` updated if any testkit item became `pub` (`spec/SPECIFICATION.md:8200-8222`, CF-32) and one line added to `CLAUDE.md`'s repository map if Arm A adds the example crate, because a map that omits a workspace member is a map that lies — while a gap requiring a **port signature change, a new conformance rule, or a `[FROZEN]` clause amendment** is reported at the story boundary for HS-S0016 and the runbook's ADR pass and is **not** absorbed; and the whole gate is green: `cargo xtask ci` run whole (not `--fast`, which omits the widened feature powerset this story enlarges), `Cargo.lock` committed with the new member so `--locked` holds, `redkiln verify --grain story` green on the PR boundary, and no `_design.md` edit, no `.kb/**` atom and no `spec/SPECIFICATION.md` edit in the diff. | `cargo xtask ci` (whole) — `tests` at `xtask/src/main.rs:143-153`, feature powerset at `:546-556`, docs and `spec-trace`; `redkiln verify --grain story` over the PR boundary block above; reviewed diff for the fixed-vs-reported split and the `CHANGELOG.md` / `CLAUDE.md` lines. |

Coverage of the traced project AC: **AC-007** (`project.md:201-203`) is satisfied jointly by AC-001
(the arm is read from `_design.md`, not re-decided), AC-003 + AC-004 (project AC-002's bar reached
from the documentation), AC-005 (project AC-005's bar, asserted on `RuleOutcome` values), and AC-002 +
AC-007 (the surface is published where an outsider meets it, and the record says where it ran out).
No other project AC is claimed here.

## Interaction quality

`_design.md` declares `surfaces: []` and that determination is **binding** (`_design.md:48-50`); its
`## Anti-patterns` section is `N/A — no user-facing surface` (`:84-86`), and the sign-off approved the
no-surface determination itself (`:96-101`). So there is no screen, and the screen-shaped invariants
below are not waved away — they are translated into the two non-visual surfaces the same file names
(`:24-38`): a **type surface** (the trait pair an author writes Rust against) and a **text surface**
(the one line a run prints for a declined capability). Every invariant that applies is carried by an
AC row above; this section only says which row carries it and how it is checked. Nothing here is a
free-floating bullet, by design — a bullet in this section gets no ledger row and is never gated.

**State invariants.**

- *In-place, not a context jump* — **AC-003**. P2 conforms from inside their own crate, with one line
  in their own `tests/`. Forking a harness, vendoring a rule, or requiring a PR into this workspace
  would be the context jump, and each is excluded by the criterion's "no fork, no vendored copy, no
  edit to the rule set".
- *Non-occlusion* — **AC-005**. A declined capability must not occlude the rule: the test is still
  emitted, the outcome is still counted, and the reason is present both in the returned value and in
  the printed line. `#[cfg]`-ing the rule out is the occlusion this forbids.
- *Preserved selection / preserved context on failure* — **AC-004**. The failure names the rule that
  broke rather than reporting "conformance failed" (`crates/happenstance-testkit/src/lib.rs:24-26`),
  and the name is written into the record so it survives the run that produced it.
- *Reversibility* — **AC-006**. The deliberate orphan-rule violation is performed, captured and then
  reverted; the permanent artefact is a transcript and prose, never a broken build.
- *Reachability without privilege* — **AC-003** and **AC-005**. Everything P2 needs is reachable with
  `cargo` alone through public items: no `#[doc(hidden)]` item named by hand, no path-only trick, no
  private module reached by a sibling-crate `#[path]` include. This is the medium's honest analogue of
  keyboard reachability, and AC-005 is where it bites, because the value assertion needs the
  projection rules module to be re-exported.

**Composition invariants** (from the design's two non-visual surfaces).

- *Presentation exists at all* — **AC-002**. The deliverable is a **composed** crate-doc section with a
  heading, ordered steps and a compiled example, not a paragraph appended to the end of a `//!` block
  and not a `.bklg/` note. The equivalent of "bare markup" here is a scope sentence buried in a
  backlog artefact, which is exactly the failure DT-8 names (`_decomposition.md:105-107`).
- *Composition and placement* — **AC-002**. It lands in
  `crates/happenstance-testkit/src/lib.rs`'s crate documentation, above the fold of the docs.rs page an
  outsider actually meets, and takes its place in the existing heading hierarchy rather than sitting
  outside it.
- *Transience* — **AC-002** and **AC-005**. Three tiers, and the story must not move anything between
  them. *Persistent chrome*: the crate documentation, present on every visit. *Revealed on condition*:
  the `SKIP {rule}: …` line, printed only when a capability is declined and only under the gate's
  `--show-output` (`xtask/src/main.rs:132-153`). *Opened on demand*: `_extension-surface-gaps.md` and
  the `error[E0117]` transcript, which are records for a reviewer, not chrome for an author.
- *Density budget, with numbers* — **AC-002**. The crate doc today carries **six** `#` sections
  (`lib.rs:15`, `:31`, `:55`, `:84`, `:134`, `:154`) and opens with a **one-line** invocation
  (`:19-22`). Arm A adds **exactly one** new `#` section, at most **70** doc lines, at most **6**
  ordered steps and **one** compiled example, and the invocation P2 copies stays **one line**. Arm B
  adds at most **one paragraph of 12 doc lines**. A write-up that needs more than that is reporting
  that the surface is too large to be an extension surface, and that finding belongs in the gap record
  (AC-007) rather than in a longer page.
- *Hierarchy* — **AC-002**. First the one-line invocation and the feature flag P2 must set; then the
  fixture they must write; only then the `__private` mechanics and the orphan-rule reasoning. A page
  that opens with the coherence argument has inverted what P2 needs first.
- *Named anti-patterns* — carried by the AC rows shown, since `_design.md` records none of its own and
  the binding ones come from `CLAUDE.md` and the accepted decision atoms: `#[async_trait]` anywhere in
  the outside crate (**AC-003**, ADR-0001 / `.kb/decisions/0001-async-port-flavours.md`); importing both
  `ProjectionStore` flavours into one module or binding the `Send` one (**AC-003**, `CLAUDE.md`
  constraint 4); a borrowing GAT on the fixture (**AC-003**,
  `crates/happenstance-testkit/src/contract.rs:97-111`); literal position assertions (**AC-003**,
  **AC-004**, `CLAUDE.md`, *The rule that matters*); a second skip format or a projection-local skip
  type (**AC-005**); a `.bklg/`-only record (**AC-002**); a fixture copied from `fixtures.rs`
  (**AC-007**); registering the outside fixture in the testkit's mutation binary (**AC-004**); and any
  edit to `_design.md` (**AC-001**, **AC-008**).

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `_design.md` carries no DT-8 resolution when this story starts. | **Halt and report** an unmet dependency on `projection-api-design-record` (HS-S0014). Do not choose an arm, do not proceed on Arm B "because it is cheaper", do not write the resolution into `_design.md`. The ledger records the halt. (AC-001) |
| **EC-002** | The projection rule set is incomplete — `buffering-conformant-variant` (HS-S0013) or an earlier slice has not merged — so the outside fixture would run a partial suite. | Halt and report. If a partial run is nonetheless performed for information, the rule set actually enumerated by `for_each_projection_store_rule!` at that commit is recorded in the gap note by name, and AC-003 stays unsatisfied — a green run over half a suite is not the claim. (AC-003) |
| **EC-003** | The macro expansion cannot name an item from the foreign crate: `error[E0433]` / `error[E0603]` against `__private` or a rules path. | This is a **documented-surface gap of the fixable kind**. Record it (what was missing, what the expansion needed), add the `pub use` / `__private` entry in this PR, and add a `CHANGELOG.md` line — a newly `pub` testkit item is a MINOR event (`spec/SPECIFICATION.md:8200-8222`). Do not work around it by importing a private path. (AC-005, AC-008) |
| **EC-004** | `error[E0117]` when the `ProjectionProbe` impl is placed in `tests/`. | **Expected, and a deliverable.** Capture the transcript verbatim, quote it in the gap note, move the impl into `src/lib.rs`. It is not a build failure to be avoided; it is the only in-tree conviction of AC-A02's placement argument (`spec/SPECIFICATION.md:5019-5025`). The committed tree must of course compile. (AC-006) |
| **EC-005** | The exactness meta-tests are unreachable from the outside crate because `Declared`/`Kind` are private to the testkit's test binary (`crates/happenstance-testkit/tests/mutation_coverage.rs:120-186`). | Record as a **gap of the first kind**: the mutant-registry discipline is not part of the documented extension surface. Forbidden remedies: registering the outside fixture in that binary (inverts the edge this story counts), or re-implementing the registry inside the example crate. AC-004 is then satisfied by the named-failure demonstration plus the recorded gap, and the ledger's evidence says which. (AC-004, AC-007) |
| **EC-006** | Adding the workspace member widens `cargo hack check --workspace --feature-powerset --no-dev-deps` (`xtask/src/main.rs:546-556`) and a combination fails. | The example crate declares **no features of its own**, so the powerset grows by one crate at one combination rather than multiplicatively. If a feature turns out to be required, its cost to the powerset is recorded in the gap note before it is added. (AC-008, NF-001) |
| **EC-007** | `Cargo.lock` is not committed with the new member, so the gate's `cargo test --locked` (`xtask/src/main.rs:147`) fails. | Commit the lock file in the same change. Never relax `--locked` to make the step pass. (AC-008) |
| **EC-008** | A gap's only fix is a port signature change, a new conformance rule, a mutant registration, or a `[FROZEN]` clause amendment. | **Report, do not absorb.** Write it as a named finding at the story boundary for `unstable-projection-gate-and-clause-disposition` (HS-S0016) and the runbook's ADR pass. Minting a `.kb/` atom here is forbidden. (AC-008) |
| **EC-009** | The write-up cannot be made to fit AC-002's density budget. | That is itself a finding — the extension surface is larger than "documented pair" claims — recorded in the gap note rather than resolved by a longer page. Escalate at the story boundary if it implies a surface change. (AC-002, AC-007) |

## Non-functional

| id | requirement | why, and where it is checked |
| --- | --- | --- |
| **NF-001** | The new workspace member costs the gate one crate in `cargo test --workspace --all-features` and **one** additional feature-powerset combination, not a multiplier. | `xtask/src/main.rs:143-153`, `:546-556`. The crate declares no features of its own (EC-006). |
| **NF-002** | The example crate compiles at the MSRV, inheriting `rust-version` from `[workspace.package]` (`Cargo.toml:8`, 1.97.1). | ADR-0029 (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). CI's `msrv` job runs `--no-dev-deps`, which hides `tokio` and the testkit — so the crate's *library* is what that job covers, and its tests are covered by the full 1.97.1 `cargo test` pass the same job runs. |
| **NF-003** | No `#[async_trait]` anywhere in the outside crate, and the bare `ProjectionStore` flavour is bound, not the `Send` one, with only one of the two names imported per module. | ADR-0001 (`.kb/decisions/0001-async-port-flavours.md`); `CLAUDE.md` binding constraints 1 and 4. Enforced by `cargo clippy -D warnings` and by review; a `Send` bound here would silently model an author who cannot target wasm32. |
| **NF-004** | The example crate is `publish = false` and never becomes publishable. | `examples/course-subscriptions/Cargo.toml` is the precedent; the gate's `cargo package --list` step asserts licence files and a README on the **three** publishable crates, and a fourth publishable crate would change that assertion silently. |
| **NF-005** | Any testkit item made `pub` by this story is a semver-MINOR event with a `CHANGELOG.md` entry, and the testkit's version number remains a promise about *the bar*, not about types. | `spec/SPECIFICATION.md:8200-8222` (CF-32 `[FROZEN]`); `crates/happenstance-testkit/Cargo.toml`'s own `version` comment. |
| **NF-006** | The rendered documentation must survive the rustdoc hazards this crate has already paid for once — no intra-doc link that resolves only under a feature the reader's build does not have. | `crates/happenstance-testkit/src/lib.rs:112-132` records the exact failure (`no item named concurrency in scope`) and the defensive spelling; `standards/rust/70-rustdoc-obligations.md` is the rule. Checked by the gate's `docs` step and the nightly `--cfg docsrs` build. |
| **NF-007** | The gap record must be legible a year later without its author: each gap cites the allowlist item that failed, the artefact that answered it, and the exact documentation change that would have prevented it. | `_decomposition.md:709-727` (*What would make this brief wrong*); this is the artefact the next adapter author, and the runbook's ADR pass, actually reads. |
| **NF-008** | The exercise's limits are stated, not implied: a workspace member depends by path, shares `[workspace.dependencies]`, and therefore does **not** discharge initiative DoD 9. | `initiative.md:383-386`; Context pack §4. An unstated limit is how a simulation gets quoted later as a proof. |

## Implementation notes (non-prescriptive)

An order that keeps the evidence honest, offered because the *sequence* is part of what several
criteria check — not because the design is prescribed.

1. **Read the arm first, write nothing else until it is recorded.** Open `_design.md`, copy the DT-8
   resolution and its `file:line` into `_extension-surface-gaps.md` and `_ledger.md`. If it is absent,
   stop (EC-001).
2. **Commit the allowlist and denylist before the fixture exists.** AC-007's check is partly the commit
   order; a denylist that lands in the same commit as the fixture is indistinguishable from one written
   to describe it.
3. **Read only the rendered documentation.** `cargo doc --all-features --open`, then work from the
   page. Expect friction at line 52-53 of the crate doc, which today tells an author that
   `fixtures::MemoryFixture` "is the reference implementation and the one to read before writing your
   own" — the single most useful sentence on the page and the one the denylist forbids following. Under
   Arm A, whether the write-up can stand without that pointer is itself a finding worth recording.
4. **Scaffold from `examples/course-subscriptions/Cargo.toml`**, which is the `publish = false`
   workspace-member precedent. No root-manifest edit is needed; `Cargo.toml:3`'s glob does it.
5. **Write the probe impl in `tests/` once, on purpose**, capture `error[E0117]` verbatim, then move it
   into `src/lib.rs` beside the store type (EC-004). Do this before the fixture is finished, while the
   crate is small enough that the transcript is short enough to quote.
6. **Build the conformant store, then the wrong one, then the declining fixture** — in that order, so
   the discriminating demonstration (AC-004) is written against a suite already known to pass, and a
   red result is unambiguous.
7. **Assert on `RuleOutcome` values, compare reasons against the fixture's own `const`.** A literal
   string repeated in the test would let the report carry someone else's words — the same discipline
   the capability-skip story applies in-tree (`crates/happenstance-testkit/src/contract.rs:458-537`).
8. **Count the graph last, when the manifest has stopped moving**: `cargo tree -p
   outside-projection-adapter --edges normal`, pasted into the note with the non-dev caveat spelled out
   (AC-006).
9. **Then write the crate-doc section**, from the gap record rather than from memory — the write-up's
   job is to close the gaps the exercise found, which means it is written after them, not before.
10. **Run the whole gate, not `--fast`.** The widened feature powerset is one of the steps `--fast`
    omits, and this story is the change that widens it.

Two things to resist. Resist making the outside crate elegant: it models an author's *first* adapter,
and a fixture that needs three helper modules is reporting something about the surface. And resist
fixing a rule that looks wrong — record it (EC-008); at merge position 7 of 8 a rule change arrives
with only one story left to absorb it.

## Tests and CI (merge gate)

Tier vocabulary is the Testing brief's (`_decomposition.md:753-767`): **Static** reads source or
config without executing the code under test; **Unit** is an in-process `#[test]`; **Integration** is a
conformance rule actually driving a `ProjectionStore` through `begin` → write → `commit` → read-back;
**E2E** is `cargo xtask ci` run whole, or a claim provable only from that run's combined output.

| tier | command / path | proves |
| --- | --- | --- |
| **Static** | Reviewed diff over `.bklg/…/documented-extension-surface/_extension-surface-gaps.md` and its commit order relative to the fixture commit | AC-001 (the arm was read and cited), AC-007 (allowlist/denylist precede the fixture; gaps recorded with cause and fix; "none" stated explicitly) |
| **Static** | `cargo doc --workspace --all-features` (gate `docs` step) + the nightly `--cfg docsrs` rustdoc step | AC-002 (the section renders; no unresolved intra-doc link), NF-006 |
| **Static** | `cargo tree -p outside-projection-adapter --edges normal` + the committed `examples/outside-projection-adapter/Cargo.toml`, both quoted in the gap note | AC-006 (one feature flag; no `happenstance-testkit` node on a normal edge) |
| **Static** | `cargo clippy --workspace --all-targets --all-features -- -D warnings` (gate step) over the new crate | NF-003 (no `#[async_trait]`, no `Send`-flavour binding, no unused-import ambiguity) |
| **Static** | `cargo hack check --workspace --feature-powerset --no-dev-deps` (`xtask/src/main.rs:546-556`) | AC-008, NF-001 (the widened powerset still passes; the example crate adds one combination) |
| **Static** | `redkiln verify --grain story` over the PR boundary block | AC-001 (ledger carries the arm citation), AC-008 (no file changed outside the boundary — no `_design.md`, no `.kb/**`, no `spec/SPECIFICATION.md`) |
| **Unit** | `cargo test --doc -p happenstance-testkit` | AC-002 (every code block in the new crate-doc section compiles — the doctest that stands in for a mock, per `CLAUDE.md`'s design-stage note and `standards/rust/70-rustdoc-obligations.md`) |
| **Unit** | `examples/outside-projection-adapter/tests/outside_projection_capability_skip.rs` | AC-005 (the returned `RuleOutcome::Skipped` value carries the fixture's own reason; the assertion reads a value, never stdout) |
| **Integration** | `examples/outside-projection-adapter/tests/outside_projection_conformance.rs` — `projection_store_conformance!(OutsideFixture::new())` | AC-003 (the macro expands in a foreign crate through `__private`; every projection rule passes against a store built from the documentation) |
| **Integration** | `examples/outside-projection-adapter/tests/outside_projection_discrimination.rs` | AC-004 (the outside author's checkpoint-only analogue fails by a named rule, reached from the published surface) |
| **E2E** | `cargo xtask ci` run whole (**not** `--fast`), with `--show-output` on the `tests` step (`xtask/src/main.rs:132-153`) | AC-003, AC-004, AC-005 and AC-008 in one artefact: the outside crate's tests run inside the ordinary gate, the `SKIP …` line is legible in the same output, and the widened powerset and docs steps are green in the same run |
| **E2E (process)** | The merged diff read against the PR boundary and the gap record | AC-008's fixed-vs-reported split: every fix cited from a gap, every unfixable gap named as a finding for HS-S0016 |

Story-grain iteration is `cargo xtask affected --base main` plus `cargo xtask ci --fast`
(`_decomposition.md:832-853`), but **`--fast` is not sufficient for this story's merge**: it omits the
feature powerset this change widens and the docs steps AC-002 leans on. The project boundary takes the
full run regardless (`project.md` DoD 4).

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The arm is not yet recorded.** | This story's entire scope is a read of `_design.md`; HS-S0014 owns that file, and slice 1 is four slices upstream. | EC-001: halt and report. The spec is written so the halt is a *legitimate outcome*, not a failure — the ledger carries the unmet dependency and the story does not guess. |
| **The suite is incomplete at merge position 7.** | The outside fixture is only meaningful against the whole rule set; `buffering-conformant-variant` (HS-S0013) closes slice 6 immediately before. | EC-002; the dependency is declared, and the enumerated rule set is recorded by name if a partial run is done for information. |
| **"Written from the documentation" is unenforceable.** | The wrong implementation named in `discover.md:79-91` compiles, passes, and reads as a success. | Procedure as the control: allowlist, denylist, commit ordering (AC-007), and gaps as the primary output. Accepted residual risk, stated in the record rather than hidden. |
| **A workspace member is not a stranger.** | Path dependencies and `[workspace.dependencies]` are conveniences no outside author has. | NF-008: the limit is written into the record and DoD 9 is explicitly not claimed. What the position *does* reproduce — orphan rule, feature flags, non-dev graph — is exactly what the claim under test is about. |
| **Shared mount points invite scope creep.** | `crates/happenstance-testkit/src/lib.rs` and `contract.rs` are inside the boundary so that documentation gaps can be fixed in place (§7), and a boundary wide enough to fix is wide enough to overreach. | AC-008's fixed-vs-reported split plus `redkiln verify --grain story`; every fix must be cited from a numbered gap in the record, which makes an uncited edit visible in review. |
| **A finding lands too late to absorb.** | This is the story most likely to convict a rule or a port shape, and only `port-disposition-and-freeze-record` remains after it. | EC-008: report at the boundary to HS-S0016 and the runbook's ADR pass. Do not mint an ADR here (`CLAUDE.md`, *Where the work lives*). |
| **A newly `pub` item is published carelessly.** | The testkit's version is a promise about the bar (CF-32), and this story is the one with a motive to widen its surface. | NF-005: `CHANGELOG.md` entry, MINOR, named in the gap record with the expansion that required it. |
| **Gate time and powerset growth.** | Every workspace member is multiplied by the feature powerset. | NF-001 / EC-006: the example crate declares no features. |

## Dependencies

**Blocks on**

- `projection-api-design-record` (HS-S0014) — records DT-8's arm in `_design.md`. Without it this
  story has no scope and must halt (EC-001). This is the hard one: the dependency is on a *decision*,
  not on code, so it cannot be worked around by stubbing.
- `buffering-conformant-variant` (HS-S0013) — closes slice 6, at which point the whole projection rule
  set exists and two structurally unlike batch shapes already pass it. Running an outside fixture
  against a partial suite proves less than it appears to (EC-002).

Transitively, everything those two depend on: the port and probe (`owned-batch-port-shape`,
`projection-probe-conformance-feature`, `memory-projection-store`), the suite entry point and skips
(`projection-suite-entry-point`, `projection-capability-skips`), the mutant registry and the rule
stories (`projection-mutant-registry`, `commit-rollback-and-drop-rules`, `reset-rules`,
`read-through-and-rebuild-rules`). None of those is re-derived here.

**Unlocks**

- `whole-gate-run-and-proof-artefact` (HS-S0018) — names this story among its `dependsOn`
  (`_storymap.md`, slice 8), because the proof artefact's single full `cargo xtask ci` must include the
  outside crate's tests if Arm A landed.

**Deliberately not a dependency**

- `unstable-projection-gate-and-clause-disposition` (HS-S0016) runs in slice 8 and *receives* this
  story's findings; it is not upstream of it. Nothing here waits on the clause disposition, and nothing
  here makes it.

## Anchors (progressive disclosure)

Links, not paste. Open each at the moment named, not before — the Context pack above is what this story
needs on first load.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | The binding design record. `surfaces: []` (`:48-50`) is why there is no screen; `:24-38` names the two non-visual surfaces this story hands to an outsider; and it is where HS-S0014 writes DT-8's arm — the single input that decides this story's scope. | **First, before any code.** Nothing else may be written until the arm is read and cited. | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/discover.md` | Carries the signal ledger, the four deferred questions this spec answers, and the named wrong implementation — "a fixture written from the documentation with `fixtures.rs` open in the next tab" (`:79-91`) — in the words the gap record must live up to. | Before writing the allowlist and denylist. | AC-007 |
| `spec/SPECIFICATION.md` | `:5019-5025` is the coherence argument for `ProjectionProbe`'s home — the exact claim this story is the only thing in the tree that can falsify; `:8200-8222` (CF-32) is why a newly `pub` testkit item is a MINOR event. | `:5019-5025` before designing the outside crate's layout; `:8200-8222` only if an item becomes `pub`. | AC-006, AC-005, AC-008 |
| `crates/happenstance-testkit/src/lib.rs` | The mount point. `:8-135` is the crate doc an outsider meets; `:15`/`:31`/`:55`/`:84`/`:134`/`:154` are the heading hierarchy the new section joins; `:52-53` is the pointer at the reference fixture that the denylist forbids following; `:189` is the `pub use suite::rules;` precedent AC-005 needs a projection sibling of; `:359-364` is the `__private` block the foreign expansion must name. | `:8-135` at step 3 (reading the rendered docs); `:189` and `:359-364` when the value assertion or the expansion first fails to resolve. | AC-002, AC-003, AC-005 |
| `crates/happenstance-testkit/src/contract.rs` | `Capability` (`:355-433`) and `RuleOutcome` (`:458-537`, with `skip_line` at `:500-507`) are reused unchanged and are what AC-005 asserts on; `:97-111` records the borrowing-GAT rustc ICE the fixture must not reintroduce. | When writing the declining fixture and its assertion. | AC-005, AC-003 |
| `crates/happenstance-testkit/src/fixtures.rs` | **The denylist's first entry.** `:243-292` is the owned-handle reference implementation an author would copy. It is listed here so it is unmistakable *which* file must stay closed, not so it can be opened. | **Only after** the outside fixture is written and the gap record is closed — as a review-time comparison, if at all. | AC-007 |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `:120-186` is the `Declared`/`Kind` registry private to the test binary; `:2754` and `:2889` are the two exactness meta-tests AC-004's bar names. Reading it is how the implementer establishes whether they are reachable from outside — which is expected to be the story's first-kind gap. | When AC-004's "clears the mutant-registry exactness check" has to be either demonstrated or recorded as unreachable (EC-005). | AC-004 |
| `crates/happenstance-core/src/projection.rs` | Where `ProjectionStore` and (after HS-S0005) `ProjectionProbe` behind `feature = "conformance"` live. The outside crate's `[dependencies]` line and its `impl` placement both follow from what is here. | When scaffolding the outside crate's manifest and `src/lib.rs`. | AC-003, AC-006 |
| `examples/course-subscriptions/Cargo.toml` | The `publish = false` workspace-member precedent, already picked up by `Cargo.toml:3`'s glob — the shape the new manifest copies, so the crate joins the gate without a root-manifest edit. | At scaffolding, step 4. | AC-003, NF-004 |
| `xtask/src/main.rs` | `:132-153` is the `tests` step and its `--show-output` rationale (why a `SKIP` line is reachable at all); `:546-556` is the feature powerset this change widens; the file *is* the gate's definition, so "green" means what this file says. | Before claiming AC-008; earlier if a gate step behaves unexpectedly. | AC-003, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | UX brief **AC-U02** (`:97-108`) states the cost bar in the words AC-006 counts against; Architecture brief Note 4 (`:462-472`) and Note 9 (`:689-693`) give the write seam and DT-8's blast radius; the Testing brief's AC-007 row (`:777`) fixes the tier and the "not copied" negation; `:709-727` is the fix-versus-report boundary. | Note 9 and AC-U02 before the manifest work; the Testing brief row before writing the tests table's real paths; `:709-727` the moment a gap is found. | AC-006, AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | P2's own text (`:114-153`) — the goal, the fear, and the sentence that makes this exercise general ("whoever eventually writes a seventh adapter needs the same thing the first six needed"). Every AC above is written from this. | When the crate-doc write-up is drafted (step 9), so it addresses the reader it is for. | AC-002, AC-004 |
| `references/adapter-shapes.md` | The house discipline for convicting a placement by *transcript* rather than by a broken build — the pattern AC-006's `error[E0117]` evidence copies. | Before performing the deliberate orphan-rule violation (step 5). | AC-006 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision behind "a rule no adapter can fail is decorative" and the no-pass-rate discipline — the standard the named-failure demonstration is held to. | Before writing the deliberately wrong outside store. | AC-004 |
| `.kb/decisions/0001-async-port-flavours.md` | The `#[async_trait]` prohibition and the two-flavour design, in the accepted-atom form that binds. | Before writing any trait impl in the outside crate. | AC-003 |
| `RUNBOOK.md` | `:3848-3965` is phase 6 in full; `:3882-3892` states the fixture contract and the bar `CheckpointOnlyStore` sets ("if it passes, the port is not frozen") — the sentence AC-004's demonstration is the outside-author instance of. | Before the discrimination test, for the bar's original wording. | AC-004 |
| `CHANGELOG.md` | Where a newly `pub` testkit item is recorded, under CF-32's reasoning that the testkit's version is a promise about the bar. | Only if EC-003 fires and something becomes `pub`. | AC-005, AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the first pass enumerated** — AC-001 … AC-008 — and none was
   added or dropped. Each maps onto a *Behavior and interfaces* row family rather than one-to-one:
   AC-001 to the arm-reading row, AC-002 to the "lands where an outsider meets it" row (both arms),
   AC-003 – AC-006 to the four Arm A demonstrations, AC-007 to the reading-discipline and gaps rows,
   AC-008 to the fix-or-report and whole-gate rows.
2. **Arm-conditional criteria are discharged, never skipped.** A ledger row for an Arm A criterion on
   the narrow arm is satisfied by citing `_design.md`'s recorded resolution as the evidence that it
   does not apply. This keeps `redkiln verify --grain story` meaningful in both worlds without two
   different ledgers, and makes the arm visible in the ledger itself.
3. **"Interaction quality" is translated, not waived.** `_design.md` binds `surfaces: []`, so the
   state and composition families are applied to the two non-visual surfaces that file names. The
   density budget is stated in real numbers taken from the crate doc as it stands today (six `#`
   sections, a one-line invocation), because a budget without numbers cannot fail.
4. **Where the outside crate lives** — `discover.md`'s deferred question 2 — is settled in Context pack
   §3: a `publish = false` workspace member under `examples/`, chosen because the members glob
   (`Cargo.toml:3`) puts it inside the ordinary gate while `experiments/` is deliberately outside it.
5. **"Written from the documentation alone"** — deferred question 3 — is settled as a procedure with a
   commit-order check (AC-007), and the residual risk is recorded rather than claimed away. No tool
   can close it; the record is the control.
6. **The exactness meta-tests are expected to be unreachable from outside**, and that outcome is
   pre-authorised as a finding (EC-005) rather than treated as a failure of the story. What is
   forbidden is the workaround that would make it look reachable.
7. **This story writes no ADR and no `.kb/` atom.** A finding that deserves one is recorded as a named
   gap for the runbook's ADR pass — the repository's standing rule that a decision record is never a
   side effect of a story.
8. **Not settled here, deliberately:** whether the extension surface should ship as a `docs/` page or a
   `book` chapter rather than crate documentation. `publication-and-positioning` owns the documentation
   estate; AC-002 requires only that it reaches P2 where they meet the crate today, which is the
   docs.rs page.
