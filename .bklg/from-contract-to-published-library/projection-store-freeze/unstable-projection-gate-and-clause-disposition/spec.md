---
item: HS-S0016
stage: spec
created: 2026-08-12T13:46:11.103Z
updated: 2026-08-12T13:46:11.103Z
template_sig: 87bbf1d0
rendered_sig: 52b2998a
---

# Spec — The module stops lying about its own maturity

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` |
| This spec | `.bklg/from-contract-to-published-library/projection-store-freeze/unstable-projection-gate-and-clause-disposition/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` — Architecture brief **AC-A04** (`:307-311`, the arm is already chosen), **Note 1**'s seam map (`:346-360`, what "mounted" means in a library), **Note 3**'s closing argument (`:437-460`, why PS-2's bar is *not* met in-project), **Note 8** (`:638-665`, `[FROZEN]` clauses this work will want to widen), **Note 9** first bullet (`:672-676`, rule-set scoping is what AC-014 audits); Testing brief's AC-014 row (`:784`) |
| Story map row | `.bklg/from-contract-to-published-library/projection-store-freeze/_storymap.md:68` — milestone `port-disposition-and-freeze-record`, first row; merge order item 8 (`:114`) |
| Signed-off design | `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` — **`surfaces: []`**, approved 2026-08-12 (`:48-50`, `:96-101`). No user-facing surface exists in this project, so this story renders none and re-decides none. |
| Roadmap pointer | `RUNBOOK.md:3923-3928` — "Decide the 0.1 exposure (PS-3)"; `RUNBOOK.md:3955-3957` — the exit box this story ticks; `RUNBOOK.md:3810-3819` — the standing post-phase reconciliation exit criterion, which applies to **every phase from 6 onward** and this is phase 6 |

## One-line PR slice

The module stops lying about its own maturity: `projection.rs`'s provisional
block is replaced by an `unstable-projection` gate with a stated reason (AC-014's
second arm, per AC-A04), every PS-1 – PS-37 clause carries an accurate maturity
marker and rule citation, the sweep's frozen-clause repair lands as a new
decision atom rather than a line edit, and `cargo xtask spec-trace` is green.

## Executive summary

By the time this row lands, seven slices have built the thing `projection.rs`'s
header says does not exist. The header still says it: *"the conformance suite
does not cover it yet — and a port without a conformance suite is a guess"*
(`crates/happenstance-core/src/projection.rs:3-11`). Two prose paragraphs are now
false, and the honest replacement is **not** deleting the word "provisional" —
PS-2's bar is two *adapters* at opposite ends of the batch-shape axis, and both
shapes this project ships are testkit-side instruments
(`spec/SPECIFICATION.md:4760-4775`; `_decomposition.md:437-451`). So AC-014 takes
its second arm and the module goes behind an off-by-default `unstable-projection`
feature that states why.

**Delta over what the tree already holds.** `CHANGELOG.md:19-22` already tells a
reader that *"`ProjectionStore` ships behind an off-by-default
`unstable-projection` feature"* and is exempt from semver. No such feature
exists: `rg unstable-projection crates/` returns nothing, and
`crates/happenstance-core/Cargo.toml`'s `[features]` block lists `std`, `serde`
and `memory` only. This PR makes a standing published claim true. Second delta:
`cargo xtask spec-trace` has never once resolved a `PS` rule name, because
`has_suite` returns `false` for the prefix (`xtask/src/spec_trace.rs:1735-1737`)
and `RULE_FILES` names three event-store-family files
(`xtask/src/spec_trace.rs:71-90`). Both exclusions were correct when written —
"neither crate has been written" (`:684-687`) — and both become false the moment
slice 3 lands. This PR flips them, which is what converts §7.2's seventeen `†`
marks on the `PS` block (`spec/SPECIFICATION.md:8628-8666`) from an accurate
statement into a checked one.

Nothing about the port's *behaviour* changes here. No rule is added, no signature
moves, no mutant is registered. What changes is which claims the specification
makes, whether the checker can see them, and whether a caller has to opt in.

## Context pack

**The arm is already chosen; this story executes it.** Architecture brief AC-A04
(`_decomposition.md:307-311`) states in writing that PS-2's bar is not met by
anything this project can build alone and takes AC-014's second arm. Do not
reopen it, and in particular do not reason from "two unlike batch shapes passed,
so the port is frozen" — AC-004's two shapes are `MemoryProjectionStore` and the
testkit's buffering variant, and PS-2's **Rejects** field names verbatim *"the
schedule that freezes this port against `MemoryProjectionStore` and an in-process
rusqlite transaction"* as the monoculture to refuse
(`spec/SPECIFICATION.md:4770-4775`). Two instruments we wrote are not two
adapters at opposite ends of an axis.

**The gate is not the verdict, and they are owned by different projects.** This
PR lands the feature and the reason. Whether the port *ships* behind it at 0.1 is
PS-3's exposure decision, owned by `publication-and-positioning` (HS-P0016) and
scheduled at phase 12 (`RUNBOOK.md:601` — "6, decided at 12";
`project.md:129-131`). Whether the freeze held is `ladybug-projection-store`'s
(HS-P0015) written verdict (`project.md:56-58`). A sentence in this PR that reads
as either verdict is a scope violation even if it is true.

**Moving a maturity marker is an ADR's act. This is the single sharpest
constraint on the story and the easiest one to violate while feeling
productive.** `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` records
two markers whose falsifiers can no longer falsify and states the discipline
plainly: both are *"recorded rather than moved"*, because moving one is a
decision. It also records the observation that makes AC-014 hard: *"a falsifier
that has already occurred without changing anything is a marker that has quietly
become decoration, and `spec-trace` cannot detect it — it sees that a marker
exists, not whether its condition has been met."* So this story **records**
dispositions; it moves a marker only on the authority of a named accepted
decision — ADR-0017 / 0018 / 0019 from slice 1
(`_storymap.md:54`), or the residual decision this story lands — and never on its
own.

**"Accurate" is a judgement obligation; the gate is necessary and not
sufficient.** `xtask/src/spec_trace.rs:17-18` says it in its own header: *"It
cannot tell whether a rule is the right rule for a clause. That is judgement."*
A green `spec-trace` proves every clause carries *a* marker, that every
`PROVISIONAL`/`DEFERRED` names *a* falsifier of at least twelve characters
(`:661-670`), that every rule name resolves, and that §1.3 and §7.1–§7.2 agree
with the parser. It proves nothing about whether PS-12's marker is still true.
The deliverable is therefore a **written disposition per clause**, and the gate
is the floor under it.

**The repair/gap classifier is mechanical and it decides what may be edited.**
From `.kb/decisions/README.md:20-22`, restated in
`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:56-72`: *a
correction to a `[FROZEN]` clause is a **repair** if the set of implementations
the clause admits is unchanged; otherwise it is a **gap**, and a gap is a
decision's.* The playbook also gives the one safe form for an obligation that has
now been met, which is most of this PR's specification-side work: **the MUST
stays verbatim**, **the discharge is named as a discharge** so a reader cannot
mistake a satisfied obligation for a relaxed one, and **the evidence is cited** —
the code and the test — because *"a discharge recorded without a named test is a
discharge that can silently regress"* (`:73-93`). Deleting a satisfied `MUST` is
the tempting edit and is almost always wrong.

**The frozen-clause repair lands as a decision record, and its scope is the
residual the sweep routed to nobody.** `ps-clause-pairing-sweep` (this story's
`depends_on`) produces a per-clause verdict with an ADR-owner cell drawn from
`{ADR-0017, ADR-0018, ADR-0019, new decision, routed}`. PS-19 falls inside
ADR-0018's clause group (PS-16 – PS-20, `project.md:65-71`) and is therefore
already decided in slice 1. **PS-1 falls inside no group in the queue**
(`RUNBOOK.md:296-298`), so it is the archetypal "new decision" row, and its
number is *allocated*, not invented: the queue's own precedent for an unscheduled
decision is ADR-0029's row, which reads *"(unscheduled — the queue had no number
for it)"* (`RUNBOOK.md:287`). Follow it.

**Atoms are not hand-written, and this story writes no atom by hand.**
`CLAUDE.md` ("Where the work lives") states that `.kb/` atoms are authored by
`/redkiln:kb-ingest` from `.kb/_intake/`; the first attempt at hand-writing them
was reverted (`0269720`). The full decision record goes to `references/adr/` —
the corpus CLAUDE.md describes as *"the full original records … carrying the
compiler transcripts, the rejected alternatives and the measurement tables a
summary cannot hold"* — and the atom-side content is staged in `.kb/_intake/` for
the next wave. Name the intake file so a second wave cannot overwrite the first's
audit trail.

**The feature gate's blast radius is compile-time and wide, and one arm of it has
already cost this repository a gate failure.** `pub mod projection;` is
unconditional at `crates/happenstance-core/src/lib.rs:98`, its four re-exports at
`:116`, and the crate-doc table at `:38` carries a bare intra-doc link
`[`ProjectionStore`]`. `broken_intra_doc_links = "deny"` (`Cargo.toml:133-134`),
and the gate runs `cargo doc -p happenstance-core --no-default-features` for
exactly this class of defect — its own comment records that three
`MemoryEventStore` links were broken without `memory` *"for as long as this gate
existed"* (`xtask/src/main.rs:494-513`, D13). Gate the module without repairing
that link and the gate goes red on the first run. Downstream, five crates name a
projection item today — `happenstance-sqlite`, `happenstance-ladybug`,
`happenstance-postgres`, `happenstance-neon` and `happenstance-sync` (the last
through cross-crate intra-doc links at `crates/happenstance-sync/src/lib.rs:21,49`)
— and `happenstance-testkit` joins them the moment slice 3 lands. Each names it
in a manifest that enables only `["std"]` or `["std", "memory"]`. `happenstance`
itself re-exports nothing from `projection` today, so it gains **no** feature
passthrough here: a public feature on a publishable crate promising a surface it
does not re-export is a semver promise nobody made.

**The powerset is the price, and it is already counted.** Architecture brief
Note 4 (`_decomposition.md:482-487`): `conformance` plus a possible
`unstable-projection` takes `happenstance-core` from eight feature combinations
to thirty-two, across **two** steps — the workspace powerset
(`xtask/src/main.rs:546-556`) and the wasm32 powerset, which names the crate
explicitly (`:558-591`). Every combination must compile, including `conformance`
*without* `unstable-projection`: decide and state whether `conformance` implies
the gate or the probe is gated on both.

**§1.3 is the one number in the document a human computed, and it must stay
that.** `xtask/src/spec_trace.rs:37-57` explains why generating it would destroy
the property it is being used to prove: §7.1 and §7.2 both come from
`parse_clauses`, so a parser that stops recognising a clause form shifts them
together and the equality check stays green; §1.3 is the only independent count.
If any marker moves, update `spec/SPECIFICATION.md:219-222`'s figures **by hand**
and never inside the generated markers at `:8507`/`:8753`.

**This is phase 6's exit reconciliation, and the runbook already wrote the
checklist.** `RUNBOOK.md:3810-3819`: every clause the phase's ADRs discharge is
read against the code *as it now stands*; the phase's clause range and the union
of its ADRs' clause ranges are computed and compared; `spec-trace`'s citation
count has not fallen. Whether that becomes a per-phase checkbox or a gate step is
`.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` and stays open
— **this story performs the reconciliation for its own clause family and does not
build the mechanism** (`project.md:330-332`).

**The persona slice.** The user here is the adapter author and the four sibling
projects that build against this port; a library's user-observable surface is its
public API, its feature table and what its suite prints
(`_storymap.md:26-28`). This story serves that author at the moment of deciding
whether to build on the port at all — the moment
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:146-150`
describes as the fear of *"discovering — late, expensively — that the port they
implemented against quietly assumed something their storage system cannot
provide"*. Their humane outcome is that the answer is in the feature table, where
`cargo add` shows it, rather than in a doc comment three screens down that has
been wrong for a slice and a half.

**What this story must not do**, each with its owner: pronounce the freeze
verdict (`ladybug-projection-store`, HS-P0015); pronounce the 0.1 exposure
verdict (`publication-and-positioning`, HS-P0016); line-edit any `[FROZEN]`
`MUST` (a decision record, never an edit — `CLAUDE.md`); hand-write a `.kb/` atom
(`/redkiln:kb-ingest`); add or retire a conformance rule or a mutant (slices 3–5);
change any port signature (`owned-batch-port-shape`); run the whole-gate proof
artefact (`whole-gate-run-and-proof-artefact`, the slice-mate below); settle
`ProjectionId`'s validation (`.kb/open-questions/projection-id-is-unvalidated.md`,
and AC-A09 keeps `new` infallible); or reopen ADR-0013's globally frozen
visibility invariant
(`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`).

## Integration contract

- **Archetype**: `capability` — user-observable at this library's real surface.
  An adapter author's `cargo add happenstance-core` stops silently handing them a
  port nothing has ever failed, and their build tells them so by refusing to name
  `ProjectionStore` until they opt in.
- **Slice / milestone**: `port-disposition-and-freeze-record`. Slice-mate:
  `whole-gate-run-and-proof-artefact`, implemented in the same context and
  mounted as one surface; this row lands **first** within the slice
  (`_storymap.md:114`, merge order item 8). The slice-mate's gate run is a
  *precondition* for reading DoD 1 and DoD 2, never a substitute for them.
- **Mount point**: **`crates/happenstance-core/src/lib.rs`** — the export block
  at `:98-124`, where `pub mod projection;` (`:98`) and
  `pub use projection::{ProjectionId, ProjectionStore, SendProjectionStore}`
  (`:115`) become reachable or invisible, together with its co-mount
  `crates/happenstance-core/Cargo.toml`'s `[features]` table. Architecture brief
  Note 1 fixes this as the composition root for this medium: *"a library has no
  render tree, so 'composition root' means the two places a new item is either
  reachable or invisible: the crate's `lib.rs` export block and the feature table
  that gates it. An item that compiles and is not mounted at both is an item no
  adapter can name"* (`_decomposition.md:339-349`). The documentary half mounts
  at `spec/SPECIFICATION.md`'s generated region (`:8507-8753`), which is written
  by `cargo xtask spec-trace --write` and MUST NOT be hand-edited (`:8326-8327`).
- **Wires into**:
  - `crates/happenstance-core/src/projection.rs:1-11` — the provisional block
    this story disposes of (`_decomposition.md:350`).
  - `crates/happenstance-core/src/lib.rs:38` — the crate-doc table's
    `[`ProjectionStore`]` link, the D13 hazard under the no-default-features doc
    build (`xtask/src/main.rs:494-513`).
  - The dependent manifests: `crates/happenstance-sqlite/Cargo.toml:15` (and its
    existing `projection-store` feature at `:33`),
    `crates/happenstance-ladybug/Cargo.toml:15`,
    `crates/happenstance-postgres/Cargo.toml:15`,
    `crates/happenstance-neon/Cargo.toml:15`,
    `crates/happenstance-sync/Cargo.toml:17`, and
    `crates/happenstance-testkit/Cargo.toml:29` once the projection suite is in it.
  - `xtask/src/spec_trace.rs` — `RULE_FILES` (`:71-90`), `has_suite`
    (`:1735-1737`), check 4 (`:680-712`), check 6 / `UNCLAIMED_PENDING_ADR`
    (`:725-727`, `:756-830`), `check_stated_census` (`:459-508`).
  - `xtask/src/main.rs` — the two feature-powerset steps (`:546-556`, `:558-591`)
    and the no-default-features doc step (`:494-513`).
  - `CHANGELOG.md:19-22` — the standing `unstable-projection` claim this PR makes
    true, and the `[Unreleased]` entry (`:24-`) this PR adds to (CF-29).
  - `references/evaluation/ps-clause-pairing-sweep.md` — **produced by this
    story's `depends_on`**, not present on `main`; it supplies the per-clause
    verdicts and the ADR-owner cells this story dispositions against.
  - The slice-1 atoms `.kb/decisions/0017-*`, `0018-*`, `0019-*` — the only
    authority under which a marker in PS-4 – PS-30 may move.
- **Renders surfaces**: **none.** `_design.md` records `surfaces: []`, approved
  2026-08-12 (`:48-50`, `:96-101`). No screen, no new public item. The one line
  the suite prints for a declined capability (`_design.md:33-38`) is unchanged
  here.
- **Conformance rule(s)**: **none added, and this story's own change is not
  adapter-observable — which is stated rather than assumed.** A feature flag and
  a maturity marker are packaging and documentation; no fixture can fail them,
  and inventing a rule for them would be exactly the decorative rule `CLAUDE.md`'s
  corollary forbids. It *is* compile-observable for every dependent crate, and it
  changes which rules `spec-trace` can see: after `has_suite` gains `PS-`, check 4
  resolves every `PS` rule citation and check 6 demands that every projection rule
  in `RULE_FILES` be claimed by a clause, retired by one, or listed in
  `UNCLAIMED_PENDING_ADR` (`xtask/src/spec_trace.rs:725-727`, `:756-830`). The
  projection rules themselves are observed by the rules and mutants slices 3–5
  landed.
- **Clause(s)**: **PS-1 – PS-37, every one dispositioned.** Discharged: **PS-3**
  (the feature now exists, off by default, with the documented semver exemption).
  Repaired under the playbook's safe form, MUST verbatim: every `PS` clause whose
  supporting prose or rule citation describes a suite that did not exist.
  Re-marked **only** on a named accepted decision's authority (slice 1's
  ADR-0017 / 0018 / 0019, or this story's residual record). Recorded as gaps and
  routed, never edited: any clause the sweep verdicted `defective` that no
  accepted decision claims. **PS-1** is the frozen-clause repair this story lands
  as a new decision record. **PS-31** and **PS-32** are documentation MUSTs owned
  by this phase (`RUNBOOK.md:3929-3933`) and are dispositioned here — PS-32's
  correction to ADR-0007's Context is a *superseding record*, never an edit, per
  `.kb/governance/rewrite-the-referent-never-the-reasoning.md:57-59` ("Reasoning
  inside a decision that still stands is never touched").
- **Advances DoD scenario**: initiative **DoD 12** — *"The clause ledger is
  audited at publish… no clause is provisional with an empty falsifier; the count
  in the report matches `spec/SPECIFICATION.md`'s own stated figure"*
  (`initiative.md:393-395`) — for this port's clause family, at the phase exit
  where the audit is cheap rather than at publish where it is not. It is also the
  precondition half of **DoD 8** ("The freeze verdict is written",
  `initiative.md:380-382`): a verdict written against a module whose stated
  maturity contradicts its own evidence is a verdict about nothing. It feeds
  **DoD 13** (`spec-trace` green on the assembled whole) and project DoD items 4
  and 5 (`project.md:247-250`).

## PR boundary

**In this PR**

1. `crates/happenstance-core/Cargo.toml` — a new `unstable-projection` feature,
   **not** in `default`, with a comment stating the reason and what would retire
   it, and its stated relation to `conformance`.
2. `crates/happenstance-core/src/lib.rs` — the module and its re-exports gated
   with `#[cfg(feature = "unstable-projection")]` +
   `#[cfg_attr(docsrs, doc(cfg(feature = "unstable-projection")))]`, copying the
   `memory` pattern at `:101-103` / `:120-122`; and the crate-doc table link at
   `:38` repaired so `cargo doc --no-default-features` stays green.
3. `crates/happenstance-core/src/projection.rs` — the `# Status: provisional`
   block (`:1-11`) replaced by the gate's stated reason: what PS-2 requires, why
   this project's two shapes do not meet it, what would, and who decides the 0.1
   exposure. The invariant section (`:13-30`) is untouched.
4. The dependent manifests named in the Integration contract — each enabling
   `unstable-projection` explicitly, and `happenstance-sqlite`'s existing
   `projection-store` feature mapping onto it rather than shadowing it.
5. `xtask/src/spec_trace.rs` — `RULE_FILES` gains the projection rules file;
   `has_suite` gains the `PS-` prefix; both comments updated to say why the old
   exclusion was right and is no longer.
6. `spec/SPECIFICATION.md` — the dispositions: maturity markers and rule
   citations for PS-1 – PS-37, §1.3's hand count (`:219-222`) if any marker
   moved, and the regenerated `§7.1–§7.2` region via
   `cargo xtask spec-trace --write` (`:8507-8753`, never hand-edited).
7. `CHANGELOG.md` — the `[Unreleased]` entry naming the feature, the semver
   exemption and what retires it (CF-29).
8. `references/adr/<allocated-number>-<slug>.md` — the residual frozen-clause
   repair as a full decision record, plus a dated
   `.kb/_intake/<date>-<slug>.md` staging its atom for the next
   `/redkiln:kb-ingest` wave.
9. `RUNBOOK.md` — one ADR-queue row allocating that number, on ADR-0029's
   `"(unscheduled — the queue had no number for it)"` precedent (`:287`).
10. `.bklg/from-contract-to-published-library/projection-store-freeze/unstable-projection-gate-and-clause-disposition/**`
    — this story's ledger and implementation report.

**Explicitly not in this PR**

- Any new conformance rule, mutant, fixture or `Capability`. Owners: slices 3–5.
- Any change to a port signature, associated type or method body. Owner:
  `owned-batch-port-shape`.
- The whole-gate proof artefact — the named failing test, both passing batch
  shapes, the clean-checkout run. Owner: `whole-gate-run-and-proof-artefact`
  (`_storymap.md:69`).
- The freeze verdict (HS-P0015) and the 0.1 exposure verdict (HS-P0016)
  (`project.md:120-131`).
- Any hand-edit under `.kb/` outside `_intake/`, and any edit to an accepted
  decision atom's body (`CLAUDE.md`; `redkiln validate --kb` enforces it).
- A feature passthrough on `crates/happenstance/` — it re-exports no projection
  item today.
- The post-phase-reconciliation *mechanism* (checkbox versus gate step). Routed
  to `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`, per
  `project.md:330-332`.
- The six integration-tier projection rules CF-36 moves out of the adapter set
  (`spec/SPECIFICATION.md:5658-5703`). Owner: `typed-layer-and-alpha-release`
  (HS-P0011); this story records their clauses' markers, it does not write them.

The implementer **may** touch the composition-root and wiring files named in the
Integration contract — `lib.rs`'s export block, the six manifests, the two xtask
files — to mount this slice. That is the mount, not scope drift.

**Merge DoD one-liner.** Merged when `cargo add happenstance-core` gives a caller
no `ProjectionStore` until they name `unstable-projection`; when the module's own
header states why in terms of PS-2's unmet bar rather than in terms of a missing
suite; when every clause PS-1 – PS-37 carries a marker and a rule citation
someone has read against today's tree and recorded; when the frozen-clause repair
exists as a decision record with the `MUST` byte-identical; and when
`cargo xtask ci` — including `spec-trace`, both feature powersets and the
no-default-features doc build — is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **`unstable-projection` exists, is off by default, and gates the module** | A new feature in `happenstance-core`'s `[features]`, absent from `default = ["std", "memory"]`. `pub mod projection;` and the `pub use projection::{…}` line carry `#[cfg(feature = "unstable-projection")]` and the matching `#[cfg_attr(docsrs, doc(cfg(…)))]`, copying the `memory` precedent verbatim. This is PS-3's `tokio_unstable` idiom, not an invention. | `crates/happenstance-core/Cargo.toml` `[features]`; `crates/happenstance-core/src/lib.rs:98`, `:101-103`, `:115`, `:120-122`; `spec/SPECIFICATION.md:4776-4790` |
| **The module header states the reason, and the reason is PS-2's** | `:1-11`'s "the conformance suite does not cover it yet" is false at merge and is replaced — not deleted. The replacement names what PS-2 requires (two adapters at opposite ends of the batch-shape axis), what this project shipped instead (two testkit-side shapes), what would clear it (a Workers / Neon / Ladybug projection adapter), and who decides the 0.1 exposure (HS-P0016, phase 12). | `crates/happenstance-core/src/projection.rs:1-11`; `spec/SPECIFICATION.md:4760-4775`; `_decomposition.md:437-456`; `RUNBOOK.md:601` |
| **The gate does not break the documentation build** | `crates/happenstance-core/src/lib.rs:38`'s crate-doc table links `[`ProjectionStore`]` unqualified. With the item behind an off-by-default feature, `cargo doc -p happenstance-core --no-default-features` resolves nothing and rustdoc treats it as a hard error under `broken_intra_doc_links = "deny"`. Repair the link the way the `memory` hazard was repaired, not by widening the doc build. | `Cargo.toml:133-134`; `xtask/src/main.rs:494-513`; `crates/happenstance-testkit/src/lib.rs:112-132` (the same hazard, already paid for once) |
| **Every crate that names a projection item opts in explicitly** | Five today — sqlite, ladybug, postgres, neon and sync (the last through cross-crate intra-doc links) — plus testkit once the suite is in it. Each manifest names the feature rather than relying on workspace feature unification, because unification is per-build and the `-p` doc and powerset steps are not. | `crates/happenstance-{sqlite,ladybug,postgres,neon,sync,testkit}/Cargo.toml`; `crates/happenstance-sync/src/lib.rs:21,49` |
| **Every feature combination still compiles, on both targets** | Adding the feature multiplies both powersets. State and honour the relation between `conformance` and `unstable-projection` — `ProjectionProbe` names projection types, so either `conformance` implies the gate or the probe is gated on both; a combination that fails to compile is a broken feature table, not a `cargo hack` quirk. | `xtask/src/main.rs:546-556`, `:558-591`; `_decomposition.md:482-487` |
| **The changelog's standing claim becomes true** | `CHANGELOG.md:19-22` already promises this feature and its semver exemption to a reader. The `[Unreleased]` entry records the landing, names what retires the exemption (PS-2's bar), and does not quote a pass rate. | `CHANGELOG.md:13-22`; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| **`spec-trace` starts resolving `PS` rule names** | `has_suite` gains the `PS-` prefix and `RULE_FILES` gains the projection rules file. Consequence to plan for, not to discover: check 6 then demands every projection rule be claimed by a `PS` clause, retired by one, or listed in `UNCLAIMED_PENDING_ADR`. An unclaimed rule is a finding to record, never an attachment to a nearby clause "to close it" — that would assert a `[FROZEN]` clause contains a proposition it does not. | `xtask/src/spec_trace.rs:71-90`, `:680-712`, `:725-727`, `:1735-1737`; `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:110-119` |
| **`†` means what the legend says** | §7.2's legend defines `†` as "does not exist yet". Seventeen `PS` rows carry it. After the switch, the regenerated region computes each mark from `resolvable`, so a rule that now exists loses its dagger by construction. The region is regenerated with `--write`, never hand-edited. | `spec/SPECIFICATION.md:8628-8666`, `:8326-8327`, `:8507`, `:8753`; `xtask/src/spec_trace.rs:1055-1060`, `:1147-1185` |
| **Every PS-1 – PS-37 clause carries a disposition someone wrote** | One recorded verdict per clause: marker accurate as it stands; marker moved on a named accepted decision's authority; obligation discharged and recorded as a discharge; or gap, recorded inside the clause it is about and routed. `spec-trace` cannot check accuracy (`:17-18`), so the written disposition is the deliverable and the gate is the floor. | `.kb/open-questions/es-7-and-vt-9-provisional-markers.md`; `xtask/src/spec_trace.rs:11-13`, `:17-18`, `:653-678`; `_decomposition.md:784` |
| **A discharged MUST keeps its sentence** | The playbook's three required components on every discharge: the `MUST` stays verbatim (so no implementation gains or loses conformance — that is what makes it a repair), the discharge is named *as* a discharge, and the code and the test are cited so the discharge is guarded rather than a claim about a moment in time. | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:73-93` |
| **PS-3 is discharged by this PR, and says so** | PS-3's `[PROVISIONAL]` falsifier is "PS-2's bar met before 0.1". That has not happened, so the marker does not move; what changes is that its SHOULD is now satisfied by a feature that exists, cited to the manifest and the module. | `spec/SPECIFICATION.md:4776-4790`; `crates/happenstance-core/Cargo.toml` |
| **The frozen-clause repair is a decision record, never an edit** | Scope is the sweep's residual: rows verdicted `defective` that slice 1's three ADRs do not claim — PS-1 archetypally, since the queue has no group containing it. The `MUST` is byte-identical before and after; the finding sits inside the clause it is about; the record goes to `references/adr/` with the atom staged in `.kb/_intake/`; the number is allocated in the RUNBOOK queue on ADR-0029's precedent. | `references/evaluation/ps-clause-pairing-sweep.md` (from `depends_on`); `RUNBOOK.md:287`, `:296-298`; `project.md:65-71`; `CLAUDE.md`; `.kb/open-questions/ps-1-states-no-progress-obligation.md` |
| **PS-31 and PS-32 are dispositioned, and PS-32 does not edit ADR-0007** | Both are documentation MUSTs the runbook assigns to this phase. PS-31's exclusion is verified against §2's identity decisions as they now stand. PS-32 requires ADR-0007's Context to be corrected — and reasoning inside a decision that still stands is never touched, so the correction is a superseding record or a routed finding, never a rewrite of the atom or of the record's argument. | `spec/SPECIFICATION.md:5480-5528`; `RUNBOOK.md:3929-3933`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59`; `.kb/decisions/0007-projection-runner-decodes.md` |
| **§1.3's hand count is updated by hand, or not at all** | If any marker moved, the four figures at `:219-222` and the document's own subtraction are corrected by a human and left outside the generated markers. Generating it would destroy the only independent check that the parser reads the document the way a person does. | `spec/SPECIFICATION.md:219-222`; `xtask/src/spec_trace.rs:37-57`, `:459-508` |
| **Phase 6's exit reconciliation is performed and recorded** | The three standing boxes: every clause the phase's ADRs discharge read against the code *as it now stands*; the phase's clause range and the union of ADR-0017 / 0018 / 0019's clause ranges computed and compared; `spec-trace`'s citation count not fallen. A disagreement between the two clause ranges is reported, not reconciled by widening an ADR. | `RUNBOOK.md:3810-3819`; `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` |
| **No verdict is pronounced** | Nothing in the diff states that the freeze held, that the port is frozen, or that the port will ship behind the feature at 0.1. The first is HS-P0015's, the second is PS-2's own bar, the third is HS-P0016's at phase 12. | `project.md:56-58`, `:120-131`; `RUNBOOK.md:601` |

**Interfaces.** One new compile-time interface and no runtime one: the
`unstable-projection` Cargo feature on `happenstance-core`, off by default,
gating `projection` and its three re-exports. No type, trait, function or method
signature is added, removed or changed by this story. Its other interface is
documentary — a per-clause disposition for PS-1 – PS-37 that
`whole-gate-run-and-proof-artefact` and, later, HS-P0015 and HS-P0016 cite by
clause id rather than read end to end.

## Data and migrations

**N/A for stored data.** This story touches no schema, no persisted state, no
serialised format and no on-disk representation. It adds no table and no column;
the wire format (`WF-*`) and the `serde` feature are untouched, and
`happenstance-core`'s payloads remain opaque `Bytes` (ADR-0003).

**One migration-shaped concern, and it is a compile-time one worth naming.**
Gating an already-exported module is a breaking change to every consumer that
names it — six crates inside this workspace, and zero outside it, because
nothing is published (`crates/happenstance-*/Cargo.toml` carry
`publish = false` on every adapter, and `happenstance-core` has no released
version). So the migration is executed in-tree, in this PR, by adding the feature
to each dependent manifest; there is no deprecation window to design and none is
owed. The *forward* migration — retiring the feature when PS-2's bar is met — is
recorded in the changelog entry and in the module header as the condition that
retires it, and it is HS-P0016's to execute, not this story's to schedule.

`happenstance-sqlite` is the one manifest with a pre-existing feature in the same
space: its `projection-store` feature (`crates/happenstance-sqlite/Cargo.toml:33`,
on by default) must *enable* `happenstance-core/unstable-projection` rather than
sit beside it, or the crate has two switches for one capability and the weaker
one silently wins.

## Acceptance criteria

The persona throughout is the **adapter author** of
`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md:130-165`
— the engineer deciding whether to implement a port for their storage system,
whose stated fear is *"discovering — late, expensively — that the port they
implemented against quietly assumed something their storage system cannot
provide"* (`:145-150`). Every criterion below is that author's journey crossing
the whole stack of this medium: manifest → export block → module doc →
specification clause → gate. The second reader is the **verdict author**
(HS-P0015 and HS-P0016), who arrives after this story and needs a per-clause
record they can cite by id rather than a document they must re-derive.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author runs `cargo add happenstance-core` and reads the crate's feature table, **WHEN** they look for the projection port, **THEN** `unstable-projection` is listed there, is absent from `default = ["std", "memory"]`, and gates `pub mod projection;` and its re-exports — so a build that names `ProjectionStore` without opting in fails to compile rather than silently handing them a port nothing has ever failed. The opt-in is one documented flag and is reversible by removing it; nothing else in the crate changes shape when it is off. | **Static.** `crates/happenstance-core/Cargo.toml` `[features]` contains `unstable-projection` and `default` does not. `crates/happenstance-core/src/lib.rs:98`/`:115` carry `#[cfg(feature = "unstable-projection")]` + `#[cfg_attr(docsrs, doc(cfg(…)))]`, matching the `memory` precedent at `:101-103`/`:120-122`. Proven by the workspace feature-powerset step (`xtask/src/main.rs:546-556`): the combination *without* the feature compiles, and a probe file naming `happenstance_core::ProjectionStore` under that combination does not. |
| AC-002 | **GIVEN** that same author opens `projection.rs` to learn what "provisional" costs them, **WHEN** they read the first screen, **THEN** the module tells them the truth as of this merge: what PS-2 actually requires (two adapters at opposite ends of the batch-shape axis), what this project shipped instead (two testkit-side shapes), what would clear the bar, and who decides the 0.1 exposure — and the false sentence *"the conformance suite does not cover it yet"* is **replaced**, not deleted. The reason sits at the module header where the gate sent them, not three screens down. | **Static (review).** `crates/happenstance-core/src/projection.rs:1-11` no longer contains the "conformance suite does not cover it yet" claim; the replacement block cites `spec/SPECIFICATION.md` §4's PS-2 and names HS-P0016 / phase 12 (`RUNBOOK.md:601`). The invariant section (`:13-30`) is byte-identical. Checked by diff review against the recorded before/after in the implementation report — a compiler cannot tell an honest reason from a plausible one. |
| AC-003 | **GIVEN** the whole gate runs after the module is gated, **WHEN** `cargo doc -p happenstance-core --no-default-features` and both feature powersets execute, **THEN** every one is green: the crate-doc table's `[`ProjectionStore`]` link at `lib.rs:38` is repaired the way the `memory` hazard was (not by widening the doc build), and the stated relation between `conformance` and `unstable-projection` holds for **every** combination including `conformance` without the gate. An author on any feature selection gets documentation that builds. | **Static + E2E.** `cargo xtask ci` steps: the no-default-features doc build (`xtask/src/main.rs:494-513`), the workspace powerset (`:546-556`) and the wasm32 powerset naming `happenstance-core` explicitly (`:558-591`). All three must pass with the widened combination set (Architecture brief Note 4, `_decomposition.md:482-487`). A failing combination is a broken feature table, recorded as such. |
| AC-004 | **GIVEN** an author reading any sibling crate to learn the house pattern, **WHEN** they inspect its manifest, **THEN** every crate that names a projection item enables `unstable-projection` **explicitly** — sqlite, ladybug, postgres, neon, sync, and testkit once the suite is in it — rather than relying on workspace feature unification, which is per-build and does not hold under the `-p` doc and powerset steps. `happenstance-sqlite`'s existing `projection-store` feature *enables* the gate rather than shadowing it, and `happenstance` gains no passthrough because it re-exports no projection item. The one text surface the design signs off — the suite's declined-capability line — is unchanged. | **Static.** Each of `crates/happenstance-{sqlite,ladybug,postgres,neon,sync,testkit}/Cargo.toml` names the feature; `crates/happenstance/Cargo.toml` does not. Proven by the same two powerset steps plus `cargo doc` per crate under `-p`. The text surface is checked against `_design.md:33-38` and `crates/happenstance-testkit/src/contract.rs:500-507` — no diff to `skip_line` or its call site. |
| AC-005 | **GIVEN** the verdict author later asks the checker whether the projection clauses cite rules that exist, **WHEN** `cargo xtask spec-trace` runs, **THEN** it answers instead of abstaining: `has_suite` recognises the `PS-` prefix (`xtask/src/spec_trace.rs:1735-1737`), `RULE_FILES` names the projection rules file (`:71-90`), check 4 resolves every `PS` rule citation, and §7.2's regenerated region computes each `†` from `resolvable` — so the seventeen daggers on the `PS` block become a checked statement rather than an accurate one. Both comments say why the old exclusion was right and is no longer. | **Static + E2E.** `cargo xtask spec-trace` (mandatory gate step) exits zero. The generated region `spec/SPECIFICATION.md:8507-8753` is produced by `cargo xtask spec-trace --write` and never hand-edited (`:8326-8327`); the `PS` rows at `:8628-8666` show daggers only where a rule genuinely does not exist. Check 6's demand (every projection rule claimed, retired, or in `UNCLAIMED_PENDING_ADR`, `:725-727`, `:756-830`) is satisfied by a recorded route, not by attaching a rule to a nearby clause. |
| AC-006 | **GIVEN** the verdict author needs to cite one clause rather than read thirty-seven, **WHEN** they open PS-1 – PS-37, **THEN** each carries a maturity marker and a rule citation that someone read against **today's** tree and recorded a verdict for: accurate as it stands / moved on a named accepted decision's authority / discharged / gap-and-routed. PS-31 and PS-32 are dispositioned as this phase's documentation MUSTs, with PS-32's correction to ADR-0007's Context taken as a superseding record — reasoning inside a decision that still stands is never touched. If any marker moved, §1.3's four figures are corrected **by hand** and left outside the generated markers. | **Static (review) + E2E.** The per-clause disposition table in the implementation report covers PS-1 – PS-37 with no gaps. `cargo xtask spec-trace` enforces the floor: every clause carries a marker, every `PROVISIONAL`/`DEFERRED` names a falsifier of ≥12 characters (`xtask/src/spec_trace.rs:653-678`), and `check_stated_census` (`:459-508`) agrees with `spec/SPECIFICATION.md:219-222`. Accuracy itself is judgement, not gateable (`:17-18`) — the written verdict is the artefact. Cross-checked against `.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59` and `.kb/decisions/0007-projection-runner-decodes.md`. |
| AC-007 | **GIVEN** an author who read a `MUST` last month and built against it, **WHEN** they re-read it after this merge, **THEN** the sentence they built against is byte-identical: every obligation this project met is recorded in the playbook's safe form — the `MUST` stays verbatim, the discharge is named *as* a discharge so a satisfied obligation cannot be mistaken for a relaxed one, and the code and the test are cited so it is guarded rather than a claim about a moment in time. PS-3 is discharged this way: its `[PROVISIONAL]` falsifier (PS-2's bar met before 0.1) has **not** occurred, so its marker does not move; what changed is that its SHOULD now points at a feature that exists. No satisfied `MUST` is deleted. | **Static (review).** `git diff` over `spec/SPECIFICATION.md` shows zero byte changes inside any `[FROZEN]` MUST sentence. Each discharge cites a real `file:line` and a real test id, per `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:73-93`. PS-3's marker at `spec/SPECIFICATION.md:4776-4790` is unchanged and its discharge cites `crates/happenstance-core/Cargo.toml` and `crates/happenstance-core/src/projection.rs`. |
| AC-008 | **GIVEN** the sweep routed a residual defect to nobody — PS-1 archetypally, since the ADR queue has no group containing it (`RUNBOOK.md:296-298`) — **WHEN** this story repairs it, **THEN** the repair arrives as a **decision record**, never a line edit: the full record in `references/adr/`, its atom staged in a dated `.kb/_intake/` file whose name cannot overwrite a prior wave's audit trail, and its number *allocated* in the RUNBOOK queue on ADR-0029's `"(unscheduled — the queue had no number for it)"` precedent (`RUNBOOK.md:287`). No `.kb/` atom is hand-written and no accepted atom's body is edited. | **Static.** `redkiln validate --kb` passes (KbFrontmatter conformance + accepted-decision immutability against `HEAD`); `redkiln doctor` reports only the six standing `template-drift` advisories. `git diff --stat` shows no modification to any file under `.kb/decisions/` and exactly one addition under `.kb/_intake/`. The record exists at `references/adr/<allocated>-<slug>.md` and the RUNBOOK queue row names the same number. |
| AC-009 | **GIVEN** phase 6 is exiting and the next reader must trust the phase, **WHEN** the reconciliation runs, **THEN** its three standing boxes are recorded for this clause family: every clause the phase's ADRs discharge read against the code *as it now stands*; the phase's clause range compared against the union of ADR-0017 / 0018 / 0019's ranges, with any disagreement **reported** rather than reconciled by widening an ADR; and `spec-trace`'s citation count not fallen. The `[Unreleased]` changelog entry makes `CHANGELOG.md:19-22`'s standing promise true, names what retires the exemption, and quotes no pass rate. Nothing in the diff pronounces the freeze verdict (HS-P0015) or the 0.1 exposure verdict (HS-P0016). | **Static (review) + E2E.** The reconciliation section of the implementation report answers `RUNBOOK.md:3810-3819`'s three boxes with figures, and the citation count is compared against the pre-merge `cargo xtask spec-trace` output. `CHANGELOG.md`'s `[Unreleased]` entry is reviewed against `.kb/decisions/0010-the-suite-must-prove-itself.md` for the no-pass-rate rule. A verdict-shaped sentence anywhere in the diff fails this AC even if it is true. The mechanism question stays routed to `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`. |

**Traceability.** Project **AC-014** (`project.md:225-229`) — *"`projection.rs` no
longer describes itself as provisional, **or** the module is behind
`unstable-projection` and says why; every PS-1 – PS-37 clause carries an accurate
maturity marker and `cargo xtask spec-trace` is green"* — is covered in three
parts: its second arm by AC-001 + AC-002 + AC-003 + AC-004, its clause-accuracy
half by AC-005 + AC-006 + AC-007 + AC-008, and its `spec-trace` half by AC-005
with AC-009 as the phase-exit record. No other project AC is claimed here.

## Interaction quality

**Composition family: does not apply, and that is signed off rather than
assumed.** `_design.md` declares `surfaces: []`, approved 2026-08-12 (`:48-50`,
`:96-101`), on two independent grounds: the initiative charter's
`userFacing: false` with `interaction-patterns.md` *"deliberately not
commissioned"* (`_decomposition.md:411-412`), and this project's UX brief saying
*"There is no screen"* (`_decomposition.md:25`). There is no route, no DOM, no
control, no density budget and no named visual anti-pattern to honour, so no
composition invariant is written as an AC row. Inventing one would be a story
re-deciding a design a human already settled.

What the design **does** sign off in this medium is two non-visual surfaces
(`_design.md:20-38`), and both are load-bearing here:

| Design-signed surface | Obligation in this story | Carried by |
| --- | --- | --- |
| **The type surface** — `ProjectionStore`, `ProjectionId` and the batch types as a caller meets them by writing Rust (`_design.md:20-27`) | Its *reachability* changes and nothing else does. No signature, no visibility beyond the gate, no item added or removed. | AC-001, AC-004 |
| **The text surface** — the one line a conformance run prints for a declined capability, `SKIP {rule}: fixture declines …` (`_design.md:29-38`, verified at `crates/happenstance-testkit/src/contract.rs:500-507`) | Unchanged by this story, byte for byte. A packaging change that alters what the suite prints has changed a signed-off surface. | AC-004 |

**State family, translated to this medium.** The state invariants have real
analogues for a library, and each is carried by a table row above rather than
left as prose here:

| Invariant | How it reads for a crate | AC row that carries it | Verified by |
| --- | --- | --- | --- |
| **In place, not a context jump** | The reason the module is gated lives at the module header the compiler error sends the reader to — not in a changelog, an ADR, or three screens below the invariant section. | **AC-002** | diff review of `projection.rs:1-11` |
| **Non-occlusion** | Gating the module must not hide anything else. The crate-doc table, the invariant section and the `memory` items stay exactly as reachable as before. | **AC-003** | no-default-features doc build (`xtask/src/main.rs:494-513`) |
| **Discoverability at the point of decision** | The maturity signal appears in the feature table, where `cargo add` shows it, rather than only in prose a reader has to already be inside the crate to find. | **AC-001** | `Cargo.toml` `[features]` review + powerset |
| **Reversibility** | Opting in is one flag and opting back out is removing it; no state is written, no migration is owed, nothing else breaks in either direction. | **AC-001**, **AC-003** | both feature powersets |
| **Preserved selection / no silent loss** | The specification analogue of preserved selection: a `MUST` a reader already built against reads identically after the merge, and a satisfied obligation is not deleted. | **AC-007** | `git diff` over `[FROZEN]` MUST sentences |
| **Reachable without special knowledge** | The keyboard-reachability analogue: an outside author needs no undocumented flag, no private item and no dependency edge — the gate is a public, named, documented feature. | **AC-001**, **AC-004** | `cargo package --list` / doc build |

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | `cargo doc -p happenstance-core --no-default-features` fails with a broken intra-doc link once `projection` is gated — `lib.rs:38`'s bare `[`ProjectionStore`]` under `broken_intra_doc_links = "deny"` (`Cargo.toml:133-134`). This is D13 repeating: three `MemoryEventStore` links were broken *"for as long as this gate existed"* (`xtask/src/main.rs:494-513`). | Repair the link the way the `memory`/testkit hazard was repaired (`crates/happenstance-testkit/src/lib.rs:112-132`) — spell it defensively. **Never** by widening the doc build's feature set, which would delete the check that found it. |
| **EC-002** | A feature combination fails to compile in either powerset — most likely `conformance` **without** `unstable-projection`, since `ProjectionProbe` names projection types. | Fix the feature table, not the gate step: either `conformance` implies `unstable-projection`, or the probe is gated on both. State which, in the manifest comment. Never narrow `--feature-powerset` to make a combination disappear. |
| **EC-003** | After `has_suite` gains `PS-`, check 6 fails: a projection rule is claimed by no `PS` clause (`xtask/src/spec_trace.rs:725-727`, `:756-830`). | Record it as a finding and route it — either retire the rule under a clause that says so, or list it in `UNCLAIMED_PENDING_ADR` with what it owes. **Never** attach it to a nearby `[FROZEN]` clause "to close it": that asserts the clause contains a proposition it does not (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:110-119`). |
| **EC-004** | `references/evaluation/ps-clause-pairing-sweep.md` is absent — the `depends_on` story has not merged. Dispositioning PS-1 – PS-37 without it means re-deriving the verdicts this story is supposed to consume. | **Halt loudly.** Do not reconstruct the sweep inside this story; report the missing dependency and stop. The sweep also carries the systematic-vs-isolated finding that decides whether this repair is one record or a re-plan (`_decomposition.md:660-662`). |
| **EC-005** | `redkiln validate --kb` fails on accepted-decision immutability, or a `.kb/` atom appears outside `_intake/`. | Revert the atom edit; the correction is a *new* atom that supersedes, staged in `.kb/_intake/` for `/redkiln:kb-ingest`. Hand-writing atoms produces the directory layout of the process without the process — the first attempt was reverted (`0269720`). |
| **EC-006** | `check_stated_census` (`xtask/src/spec_trace.rs:459-508`) fails after a marker moves: §1.3's stated figures (`spec/SPECIFICATION.md:219-222`) disagree with the parser. | Correct §1.3 **by hand**, outside the generated markers at `:8507`/`:8753`. Never generate it: §7.1 and §7.2 both come from `parse_clauses`, so §1.3 is the only count that can disagree with the parser (`:37-57`). |
| **EC-007** | A dependent crate compiles in the workspace build but fails under `cargo doc -p …` or in a powerset combination — feature unification masked a missing opt-in. | Name the feature in that crate's own manifest. Unification is per-build; the `-p` and powerset steps are the ones that prove the manifest is honest. |
| **EC-008** | The sweep's residual turns out to be a **gap**, not a repair — the set of implementations the clause admits would change (`.kb/decisions/README.md:20-22`). | Stop and route it as a decision this story does not own. A gap widened under cover of a "repair" is the failure mode `_decomposition.md:660-662` names as a re-plan the phase-6 budget does not assume. |

## Non-functional

| id | requirement | how it is honoured |
| --- | --- | --- |
| **NF-001** | **Zero runtime cost and zero behaviour change.** No type, trait, function or method signature is added, removed or changed; no code path executes differently at run time. | The diff touches manifests, `cfg` attributes, doc comments, specification prose and xtask constants only. Any change under `crates/*/src/**` other than attributes and doc comments is out of boundary (owner: `owned-batch-port-shape`). |
| **NF-002** | **The gate's widened cost is paid, counted and stated.** `happenstance-core` goes from eight feature combinations to thirty-two across two powerset steps (`_decomposition.md:482-487`). | The implementation report records the measured wall-clock delta for both powersets so `whole-gate-run-and-proof-artefact` inherits a number rather than a surprise. A slower gate is acceptable; an uncounted one is not. |
| **NF-003** | **No new dependency and no MSRV movement.** The floor stays 1.97.1 (ADR-0029). | Nothing here needs a crate. Moving the floor would require an ADR (`CLAUDE.md`, binding constraint 5) and this story has no cause to. |
| **NF-004** | **The compile-time break is executed in-tree, in one PR.** Nothing is published, so no deprecation window is owed and none is designed. | Every dependent manifest lands in the same PR; the *forward* migration (retiring the feature when PS-2's bar is met) is stated as a condition in the module header and the changelog, and is HS-P0016's to execute. |
| **NF-005** | **Documentation renders the gate.** The `docs.rs` build shows the feature requirement on the gated items. | `#[cfg_attr(docsrs, doc(cfg(feature = "unstable-projection")))]` matching the `memory` precedent (`lib.rs:101-103`, `:120-122`), with `all-features = true` already set (`crates/happenstance-core/Cargo.toml:54-56`); proven by the nightly `--cfg docsrs` rustdoc step. |
| **NF-006** | **`no_std` is preserved.** `unstable-projection` does not pull `std` in on its own. | Its feature entry declares only what the module genuinely needs; the powerset compiles the combination without `std` wherever `default` is off. |
| **NF-007** | **The specification's citation count does not fall.** A disposition pass that quietly drops citations would make the document less checkable while looking tidier. | Pre- and post-merge `cargo xtask spec-trace` counts compared and recorded (`RUNBOOK.md:3810-3819`, third box) — AC-009. |

## Implementation notes (non-prescriptive)

Left to the implementer, recorded so each is chosen rather than defaulted.

- **Order the edits so the gate can tell you which change broke it.** The two
  halves — the feature gate and the `spec-trace` prefix flip — fail in completely
  different ways, and landing them in one commit turns two clear diagnostics into
  one confusing one. A defensible sequence is: gate + repair the doc link + fix
  the six manifests (gate green) → flip `has_suite`/`RULE_FILES` and absorb check
  4 and check 6 (gate green) → disposition the clauses and regenerate with
  `--write` (gate green) → the decision record and the changelog. Checkpoint
  between them.
- **Copy the `memory` precedent literally.** `lib.rs:101-103` and `:120-122`
  already show the exact attribute pair this crate uses for an off-by-default
  gated module. Deviating buys nothing and costs a reviewer a comparison.
- **Decide `conformance` × `unstable-projection` before writing either.** The two
  legible options are `conformance = ["unstable-projection", …]` (fewer valid
  combinations, one switch for one capability) or gating `ProjectionProbe` on
  both (`conformance` stays orthogonal, the powerset stays wider). Either is
  defensible; state which in the manifest comment, because the powerset will find
  out regardless (EC-002).
- **The disposition table wants to be written once, in the report, and cited from
  the spec.** Thirty-seven rows of "why this marker is still true" do not belong
  inside `SPECIFICATION.md` — the clause carries the marker and the citation; the
  reasoning lives in the implementation report where HS-P0015 and HS-P0016 can
  cite it by clause id.
- **Allocate the ADR number, do not invent it.** Read the queue in `RUNBOOK.md`
  and follow ADR-0029's row precedent — *"(unscheduled — the queue had no number
  for it)"* (`:287`). A number chosen without a queue row is a number two stories
  can pick.
- **Name the intake file so a second wave cannot overwrite the first's audit
  trail** — date-prefixed and slug-suffixed, matching the wave-id convention the
  prior ingests used.
- **`happenstance-sqlite`'s `projection-store` is the only pre-existing switch in
  this space** (`Cargo.toml:33`, on by default). Making it *enable*
  `happenstance-core/unstable-projection` keeps one capability behind one switch;
  leaving it beside gives the crate two, and the weaker one wins silently.
- **If the sweep returned "systematic" rather than "isolated"**, say so and stop
  at the boundary this story owns. Widening ten frozen clauses under cover of a
  packaging story is exactly the move `_decomposition.md:660-662` names as a
  re-plan.

## Tests and CI (merge gate)

Tier vocabulary is the project testing brief's (`_decomposition.md:755-770`):
**Static** reads source/config without executing the code under test; **Unit** is
`cargo test` in-process; **Integration** is a rule driving a real store; **E2E**
is `cargo xtask ci` run whole. This story is almost entirely Static and E2E by
construction — its subject is packaging and documentation, and the brief's AC-014
row says so in one word: **Static** (`_decomposition.md:784`).

| tier | command / path | proves |
| --- | --- | --- |
| **Static** | `cargo xtask spec-trace` — `xtask/src/spec_trace.rs`, run standalone before the whole gate | AC-005, AC-006. Every `PS` rule citation resolves once `has_suite` knows the prefix; every clause carries a marker; every `PROVISIONAL`/`DEFERRED` names a falsifier ≥12 chars (`:653-678`); §1.3 agrees with the parser (`check_stated_census`, `:459-508`); §7.1/§7.2 regenerate cleanly. |
| **Static** | `cargo doc -p happenstance-core --no-default-features` — the gate step at `xtask/src/main.rs:494-513` | AC-003, EC-001. The D13 hazard: an intra-doc link into a module absent on a documented configuration is a hard rustdoc error under `broken_intra_doc_links = "deny"`. |
| **Static** | `cargo hack check --workspace --feature-powerset --no-dev-deps` — `xtask/src/main.rs:546-556` | AC-001, AC-003, AC-004, EC-002, EC-007, NF-006. All thirty-two `happenstance-core` combinations compile, including `conformance` without the gate and every combination without `std`. |
| **Static** | `cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` — `xtask/src/main.rs:558-591` | AC-003. The same widening on the target the whole two-trait design exists for; `happenstance-core` is named explicitly in this step. |
| **Static** | Nightly `--cfg docsrs` rustdoc build (mandatory-if-present step in `cargo xtask ci`) | NF-005. The `doc(cfg(…))` badge renders on the gated items rather than the feature being invisible on docs.rs. |
| **Static** | `cargo package --list` assertion over the three publishable crates (in `cargo xtask ci`) | AC-004. The gate does not disturb packaging — licences and README still present, and `happenstance` gains no promise it cannot keep. |
| **Static** | `redkiln validate --kb && redkiln doctor` | AC-008, EC-005. KbFrontmatter conformance and accepted-decision immutability against `HEAD`; `doctor` reports exactly the six standing `template-drift` advisories and no seventh. |
| **Static (review)** | Diff review recorded in the implementation report: `crates/happenstance-core/src/projection.rs:1-11` before/after; `git diff` over `[FROZEN]` MUST sentences in `spec/SPECIFICATION.md` | AC-002, AC-007, AC-009. Honesty of a stated reason, byte-identity of a `MUST`, and the absence of a verdict-shaped sentence are all judgement — `spec_trace.rs:17-18` says so in its own header. A compiler cannot run this row and the story does not pretend otherwise. |
| **Static (review)** | The per-clause disposition table in the implementation report, PS-1 – PS-37, one row each with verdict + evidence | AC-006. The deliverable the gate is the floor under. Consumed by HS-P0015 and HS-P0016 by clause id. |
| **Unit** | `cargo test -p xtask` — the checker's own tests, run under the workspace `tests` step | AC-005. `has_suite` and `RULE_FILES` are constants the checker's tests exercise; a prefix flip that breaks an existing expectation surfaces here rather than in a 200-clause diff. |
| **Integration** | *(none added by this story.)* The projection rules that drive a real store are slices 3–5's; this story adds no rule and no mutant, per `CLAUDE.md`'s "a rule that no adapter can fail is decorative". | Recorded so no implementer invents a decorative rule for a feature flag. |
| **E2E** | `cargo xtask ci` — the whole gate, on this branch | AC-003, AC-005, AC-009, and project DoD 4 (`project.md:243-244`). Everything above in one run, plus the tests, wasm32 and `cargo deny` steps this story could break only by accident. |
| **E2E** | `cargo xtask affected --base main` + `cargo xtask ci --fast` | The story grain during implementation (`_storymap.md:118-121`). Note that `--fast` **omits** `spec-trace`, which is the step AC-005 and AC-006 rest on — so `--fast` is the working loop, never the merge bar for this row. |

**Merge bar.** All Static and E2E rows green, both review rows recorded with their
artefacts, and the ledger's nine rows flipped with cited evidence. The slice-mate
`whole-gate-run-and-proof-artefact` re-runs the whole gate on a clean checkout
afterwards; that run is a *precondition* for reading DoD 1 and DoD 2 and is never
a substitute for this row's own green gate (`project.md:255-256`).

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **The doc-link break lands on the first run** (EC-001) | `lib.rs:38` links `[`ProjectionStore`]` unqualified today, and the no-default-features doc build is a mandatory step. This is the single most likely red first gate. | Repair the link **in the same commit** as the gate, not after. The precedent is already in the tree (`crates/happenstance-testkit/src/lib.rs:112-132`) and cost this repository the D13 lesson once. |
| **Check 6 opens a queue nobody budgeted** | Flipping `has_suite` makes `spec-trace` demand that *every* projection rule be claimed, retired or listed. Slices 3–5 wrote those rules against clauses; any mismatch surfaces here, at the end of the project, in a story whose budget assumed packaging. | Run `cargo xtask spec-trace` with the prefix flipped **early**, before writing any disposition prose, so the size of the queue is known while there is still room to route it (EC-003). Route, never attach. |
| **The tempting edit: deleting a satisfied `MUST`** | Most of this story's specification work is recording obligations that have now been met, and the tidy-looking move is to delete the sentence. It silently changes which implementations conform. | The playbook's three components are non-negotiable and are AC-007's whole content: MUST verbatim, discharge named as a discharge, evidence cited (`.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md:73-93`). |
| **Scope creep into a verdict** | Writing thirty-seven honest dispositions puts the author one sentence away from "so the port is frozen" — which is HS-P0015's, and from "so it ships behind the feature at 0.1" — which is HS-P0016's at phase 12. Both would be *plausible* and both are out of bounds. | AC-009 makes the absence of a verdict a checked criterion. Reviewer reads the diff for verdict-shaped sentences explicitly. |
| **Marker drift disguised as accuracy** | `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` records the exact failure: a falsifier that has already occurred without changing anything is a marker that has quietly become decoration, *and `spec-trace` cannot detect it*. An author with thirty-seven clauses and a green gate will feel finished. | A marker moves only on a named accepted decision's authority (ADR-0017/0018/0019 or this story's residual record). Every other case is **recorded**, not moved. AC-006. |
| **Feature unification masks a missing opt-in** (EC-007) | The workspace build unifies features, so a dependent crate that never names `unstable-projection` still compiles — until the `-p` doc step or a powerset combination runs it alone. | AC-004 requires the explicit name in each manifest, and the two powerset steps are what prove it rather than the workspace build. |
| **The dependency is a real artefact, not a formality** (EC-004) | `references/evaluation/ps-clause-pairing-sweep.md` does not exist on `main`; it is produced by `ps-clause-pairing-sweep`. Without it there are no per-clause verdicts and no ADR-owner cells, and the disposition becomes invention. | Halt loudly. Do not reconstruct the sweep here. |
| **Coupling to the slice-mate's ordering** | `whole-gate-run-and-proof-artefact` runs the clean-checkout gate and reads DoD 1 and DoD 2. If this row lands second, that run proves a tree this story then changes. | Merge order fixes this row **first** within the slice (`_storymap.md:114`, item 8). Do not reorder for convenience. |
| **Powerset time doubles quietly** | Thirty-two combinations across two steps, on top of whatever `conformance` already added in slice 2. A gate nobody wants to run stops being a gate. | NF-002: measure and record the delta so the slice-mate and HS-P0016 inherit a number. |

## Dependencies

**Blocks on** (must be merged before this story starts; both are in this
project):

| story slug | what this story consumes from it | consequence if absent |
| --- | --- | --- |
| `ps-clause-pairing-sweep` | The per-clause verdict table for PS-1 – PS-37 with its ADR-owner cell drawn from `{ADR-0017, ADR-0018, ADR-0019, new decision, routed}`, landing at `references/evaluation/ps-clause-pairing-sweep.md`; and the systematic-vs-isolated finding that decides whether the residual repair is one record or a re-plan (`_decomposition.md:660-662`). | AC-006 and AC-008 have no input. Halt (EC-004). |
| `ps3-batch-shape-finding` | The written PS-3 evidence — *did the two batch shapes disagree, and where?* — including the "they agreed everywhere" outcome, which is itself the finding. It is what lets AC-002's module header state what this project shipped instead of PS-2's bar, without overclaiming. | AC-002's reason cannot be written honestly, and AC-009 risks absorbing HS-P0016's verdict by default. |

Transitively, everything those two rest on: slices 1–7 in
`_storymap.md:104-117`'s merge order. In particular the projection **rules file**
must exist in `happenstance-testkit` before `RULE_FILES` can name it (AC-005) —
landed by `projection-suite-entry-point` and extended by slices 4–5 — and the
slice-1 atoms `.kb/decisions/0017-*`, `0018-*`, `0019-*` must be **accepted**
before any marker in PS-4 – PS-30 may move (AC-006).

**Unlocks:**

| consumer | what it takes from this story |
| --- | --- |
| `whole-gate-run-and-proof-artefact` (slice-mate, merges second) | A tree whose feature table, clause markers and `spec-trace` configuration are final, so its clean-checkout `cargo xtask ci` proves something durable rather than a snapshot about to change (`_storymap.md:114`). |
| `ladybug-projection-store` (HS-P0015) | The freeze verdict it writes is about a module whose stated maturity no longer contradicts its own evidence — the precondition half of initiative DoD 8 (`initiative.md:380-382`). |
| `publication-and-positioning` (HS-P0016) | The `unstable-projection` feature exists to *decide about* at phase 12, and PS-3's disposition tells it exactly what would retire the semver exemption (`RUNBOOK.md:601`; `project.md:129-131`). |
| `typed-layer-and-alpha-release` (HS-P0011) | The recorded markers for the six integration-tier rules CF-36 moves out of the adapter set (`spec/SPECIFICATION.md:5658-5703`) — this story records their clauses' markers; HS-P0011 writes the rules. |

## Anchors (progressive disclosure)

Everything load-bearing that is **not** distilled into the Context pack. Open
each at the moment named, not before — the Context pack above is the must-read.

| anchor (real path) | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `crates/happenstance-core/src/lib.rs` | The mount point. `:98`/`:115` are the two lines that gain `cfg`; `:101-103`/`:120-122` are the `memory` precedent to copy verbatim; `:38` is the crate-doc link that breaks the no-default-features doc build if left bare. | First edit of the story, before writing any `cfg`. | AC-001, AC-003 |
| `crates/happenstance-core/Cargo.toml` | The co-mount. `[features]` at `:34-52` shows how this crate comments a feature (why it exists, what it costs, the D12 lesson about stating what you need); `:54-56` already sets `all-features` + `--cfg docsrs`. Your new entry must read like its neighbours. | With the `lib.rs` edit. | AC-001, NF-005, NF-006 |
| `crates/happenstance-core/src/projection.rs` | `:1-11` is the false paragraph being replaced and `:13-30` is the invariant section that must stay untouched. Read both before editing so the boundary between them is obvious. | Before writing the replacement reason. | AC-002 |
| `xtask/src/main.rs` | The gate, defined once. `:494-513` is the no-default-features doc step **with the D13 comment explaining the exact failure you are about to risk**; `:546-556` and `:558-591` are the two powersets your feature multiplies. | Before the first `cargo xtask ci`, and again when a combination fails. | AC-003, EC-001, EC-002, NF-002 |
| `xtask/src/spec_trace.rs` | The checker you are reconfiguring. `:17-18` states what it cannot check (accuracy — which is why AC-006's deliverable is prose); `:37-57` explains why §1.3 stays hand-computed; `:459-508`, `:653-678`, `:680-712`, `:725-727`, `:756-830`, `:1735-1737` are the census check, the falsifier check, check 4, check 6 and `has_suite`. | Before flipping the `PS-` prefix — and read check 6 first, so its queue is a plan rather than a surprise. | AC-005, AC-006, EC-003, EC-006 |
| `spec/SPECIFICATION.md` | The document being dispositioned. `:219-222` is §1.3's hand count; `:4760-4775` is PS-2's bar and its verbatim **Rejects** field; `:4776-4790` is PS-3; `:5480-5528` is PS-31/PS-32; `:5658-5703` is CF-36's six integration rules; `:8326-8327` is the do-not-hand-edit banner; `:8507-8753` is the generated region and `:8628-8666` the seventeen `PS` daggers. | Continuously through AC-005 – AC-007; open PS-2 **before** writing AC-002's reason. | AC-002, AC-005, AC-006, AC-007 |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | The mechanical repair-vs-gap classifier (`:56-72`), the three required components of a discharge (`:73-93`), and the explicit warning against attaching an unclaimed rule to a nearby clause (`:110-119`). This is the procedure AC-007 and EC-003 are made of. | Before touching any `[FROZEN]` clause — every time, not once. | AC-007, AC-006, EC-003, EC-008 |
| `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` | The precedent for a marker whose falsifier can no longer falsify: both were *recorded rather than moved*, because moving one is a decision. It also states the defect `spec-trace` structurally cannot see. | Before deciding any marker is "obviously stale". | AC-006 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | `:50-59` — reasoning inside a decision that still stands is never touched. PS-32 asks for a correction to ADR-0007's Context, and this is the rule that makes it a superseding record instead of an edit. | Before dispositioning PS-32. | AC-006 |
| `.kb/decisions/0007-projection-runner-decodes.md` | The atom PS-32's correction is *about*. Read it to see what the Context actually says before deciding whether a superseding record or a routed finding is proportionate. | With the previous anchor, at PS-32. | AC-006 |
| `.kb/decisions/README.md` | `:20-22` — the repair/gap definition in its original home, and the corpus conventions the residual decision record must match. | Before drafting the residual decision record. | AC-008, EC-008 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The no-pass-rate rule the changelog entry must respect. A `[Unreleased]` line quoting "n of m rules pass" is the exact thing this atom forbids. | While writing the `CHANGELOG.md` entry. | AC-009 |
| `.kb/open-questions/ps-1-states-no-progress-obligation.md` | The archetypal residual: PS-1's defect, already written up, with no ADR group in the queue containing it. This is the most likely subject of the decision record AC-008 lands. | When the sweep routes PS-1 to "new decision". | AC-008 |
| `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` | The open question this story performs but does **not** close. Read it so the reconciliation is done for this clause family without accidentally building the mechanism. | Before writing the AC-009 reconciliation section. | AC-009 |
| `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` | The adjacent trap the Architecture brief names (`_decomposition.md:664-666`): a boundary-scoped projection checkpoint reopens ADR-0013's globally frozen invariant. If a disposition drifts here, stop and file the decision. | If any PS clause's disposition touches checkpoint scope. | AC-006 |
| `RUNBOOK.md` | `:287` is ADR-0029's unscheduled-number precedent; `:296-298` shows PS-1 falling inside no ADR group; `:601` is the "6, decided at 12" exposure schedule; `:3810-3819` is the phase-exit reconciliation checklist; `:3923-3933` is this phase's PS-3/PS-31/PS-32 assignment and `:3955-3957` the exit box this story ticks. | At AC-008 (number allocation) and AC-009 (reconciliation). | AC-008, AC-009 |
| `CHANGELOG.md` | `:13-22` carries the standing promise about `unstable-projection` and the semver exemption that this PR makes true, plus the testkit versioning note that sets the tone for how this crate talks about breakage. | While writing the `[Unreleased]` entry. | AC-009 |
| `crates/happenstance-testkit/src/lib.rs` | `:112-132` — the same rustdoc hazard, already paid for once in this repository, with the defensive spelling that fixed it. The cheapest possible source for EC-001's repair. | The moment the no-default-features doc build goes red. | AC-003, EC-001 |
| `crates/happenstance-testkit/src/contract.rs` | `:500-507` — `skip_line`, the one text surface `_design.md` signs off. Read it to confirm this story leaves it byte-identical. | At AC-004's review. | AC-004 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_design.md` | `:20-38` names the two non-visual surfaces and `:48-50`/`:96-101` record `surfaces: []` as approved. It is the authority for why no composition invariant appears in this spec's AC table. | If anyone asks for a surface, a screenshot or a composition AC. | AC-004 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/_decomposition.md` | The briefs. AC-A04 (`:307-311`) is the already-chosen arm; Note 1 (`:339-360`) defines "mounted" for a library; Note 3 (`:437-460`) is why PS-2's bar is unmet in-project; Note 4 (`:482-487`) counts the powerset price; Note 8 (`:638-665`) lists the `[FROZEN]` clauses this work will want to widen and forbids it; Note 9 (`:672-676`) is the rule-set scoping AC-014 audits; the testing brief's AC-014 row is `:784`. | Note 3 before AC-002; Note 4 before the first powerset run; Note 8 before any clause edit. | AC-002, AC-003, AC-006 |
| `.bklg/from-contract-to-published-library/projection-store-freeze/project.md` | `:225-229` is project AC-014 verbatim; `:56-58` and `:120-131` are the two verdicts this story must not pronounce; `:240-256` is the project DoD this story feeds items 4 and 5 of; `:330-332` routes the reconciliation mechanism. | Before writing anything that sounds like a conclusion. | AC-009 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | `:130-165` — the adapter author, their stated fear, and the four-step journey whose step 4 (*"has no way to tell, from inside their own effort"*) is precisely what a truthful feature table addresses. The grounding for every GIVEN above. | When an acceptance criterion starts sounding like a capability rather than a person's goal. | AC-001, AC-002 |
| `references/evaluation/ps-clause-pairing-sweep.md` | **Does not exist on `main`** — produced by `ps-clause-pairing-sweep`, this story's `depends_on`. It carries the per-clause verdicts and ADR-owner cells the disposition consumes and the systematic-vs-isolated finding that bounds the repair. | First action of implementation: confirm it exists. If it does not, halt (EC-004). | AC-006, AC-008 |

## Clarifications resolved during spec

1. **Which arm of AC-014 this story takes was not left open.** Project AC-014
   offers "drops provisional **or** gates behind `unstable-projection` and says
   why" (`project.md:225-229`). Architecture brief AC-A04 (`_decomposition.md:307-311`)
   already took the second arm in writing, and PS-2's **Rejects** field names the
   `MemoryProjectionStore`-plus-one-in-process-store schedule as the monoculture
   to refuse (`spec/SPECIFICATION.md:4770-4775`). This spec therefore treats the
   arm as settled input, not as a decision to make. AC-001 – AC-004 execute it.
2. **The nine AC ids are exactly the front half's.** No id was added or dropped.
   The mapping is: AC-001 – AC-004 discharge AC-014's gating arm (feature,
   reason, gate-stays-green, dependents opt in); AC-005 – AC-008 discharge its
   clause-accuracy half (`spec-trace` sees `PS`, every clause dispositioned,
   discharges keep their MUST, the residual lands as a decision record); AC-009
   carries the phase-exit reconciliation, the changelog and the
   no-verdict constraint.
3. **No composition AC exists, and that is a citation rather than an omission.**
   `_design.md` records `surfaces: []` as approved (`:48-50`, `:96-101`). The
   Interaction quality section translates the *state* family into this medium and
   binds each invariant to an existing AC row, because `redkiln verify` extracts
   ACs from table cells and a prose bullet would never be gated. The design's two
   non-visual surfaces are bound to AC-001/AC-004.
4. **No conformance rule is added, and this is stated rather than assumed.** A
   feature flag and a maturity marker are packaging and documentation; no fixture
   can fail them. `CLAUDE.md`'s corollary — *"a rule that no adapter can fail is
   decorative"* — makes inventing one a defect, so the Tests table carries an
   explicit empty Integration row so nobody reads the absence as an oversight.
5. **`happenstance` gets no feature passthrough.** It re-exports no projection
   item today (`crates/happenstance/`), so a public feature there would be a
   semver promise about a surface the crate does not expose. Recorded in the PR
   boundary as explicitly out.
6. **The residual decision record's number is allocated, not chosen.** The queue
   in `RUNBOOK.md` has no group containing PS-1 (`:296-298`); ADR-0029's row
   supplies the precedent for an unscheduled allocation (`:287`). The record goes
   to `references/adr/` and its atom is staged in `.kb/_intake/` for
   `/redkiln:kb-ingest` — this story hand-writes no `.kb/` atom (`CLAUDE.md`;
   commit `0269720`).
7. **`--fast` is not this story's merge bar.** `cargo xtask ci --fast` omits
   `spec-trace`, which is the one step AC-005 and AC-006 rest on
   (`_storymap.md:118-121`). It stays the working loop; the merge bar is the full
   `cargo xtask ci`.
8. **The `conformance` × `unstable-projection` relation is deliberately left to
   the implementer, with the obligation to *state* it.** Both arms are
   defensible; what is not defensible is discovering the relation from a red
   powerset. Recorded as an implementation note with EC-002 as its failure mode.
