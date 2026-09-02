# Grounding — The Checked Documentation Surface (HS-P0020)

Written for `/redkiln:plan`'s briefs stage. Cites real paths only; every claim below
was verified against this worktree, not assumed from the charter.

## What this project actually is

Per `_decomposition.md`, HS-P0020 is first in merge order, depends on nothing, and
carries: the pinned narrative tree (AC-001), the mandatory compiling gate step
(AC-002), the observed-fail/observed-recover proof (AC-003), the no-silent-`ignore`
rule (AC-004), the no-orphan-page rule (AC-005), the hidden-content resolution
(AC-006, DT-7), citation resolution against `spec/SPECIFICATION.md` (AC-007), the
BR-10 clause-id pin (AC-008), clean-checkout/no-manual-step (AC-009), and the
documented blind spots (AC-010). It earns all four brief tags
(`_decomposition.md:324-331`): `architecture` for the pinned tree and gate wiring,
`ux` for DT-7, `testing` for the two falsifications (AC-003, and DoD-13's
hidden-content break), `deployment` for where the rendered surface lives — it is the
only project in the initiative that earns `deployment`.

## The load-bearing prior art: `standards/rust/` is not analogous, it is the template

This is the single most important grounding fact. The repository has already built
almost exactly this mechanism once, for a different tree, and its wiring is fully
inspectable:

- **The tree is pinned by path as constants**, not by convention:
  `xtask/src/lint_constitution.rs:55,58,61` —
  `const ATOM_DIR: &str = "standards/rust"`, `const ROUTER: &str =
  "standards/rust/README.md"`, `const HARNESS: &str =
  "xtask/src/constitution.rs"`. `docs/README.md:25-29` states the general rule this
  project must restate for its own tree: "Moving either tree means editing
  `xtask/src/` in the same change — which is the point of pinning them by path
  rather than by convention." **AC-001 is this exact pattern, applied to a new
  constant and a new directory.**
- **Every fence is compiled against the real crates, and the mechanism is a
  `#[cfg(doctest)]` module per file**, not a bespoke Markdown-to-Rust extractor:
  `xtask/src/constitution.rs:41-140` — one `mod <atom> { #![doc =
  include_str!("../../standards/rust/NN-slug.md")] }` per file, run by `cargo test
  -p xtask --doc` (wired as the REQUIRED step `"the constitution's examples
  compile"`, `xtask/src/main.rs:488`). The file's own module doc explains *why* one
  module per atom rather than one doc string concatenating all of them
  (`xtask/src/constitution.rs:11-18`): a single doc string reports a failing line
  number counted from the first atom, which maps to no file a reader can open.
  **This is the direct mechanism AC-002 should reuse or knowingly diverge from.**
  Fences in this new tree can call real workspace types the same way
  `standards/rust/91-adapter-authoring-recipe.md:37` does
  (`use happenstance_core::{...}`), because `xtask`'s `[dev-dependencies]`
  (`xtask/Cargo.toml:22-33`) already carries `happenstance`, `happenstance-core`
  and `happenstance-testkit` — added, per the manifest's own comment
  (`xtask/Cargo.toml:12-15`), specifically *because* neither the README's nor the
  constitution's examples can live in a published crate (`include_str!` resolves
  against the file tree at compile time, and that tree does not exist inside a
  packaged `.crate`). The same reasoning applies to a new narrative tree.
- **No orphan pages, already solved once**: `xtask/src/lint_constitution.rs:423-458`
  (`check_harness`) cross-checks in both directions — every atom file must have a
  matching `include_str!` and `mod` in the harness, *and* every `mod` in the harness
  must name a file that still exists in the directory. "The reverse" comment at
  `:445-446` states exactly the failure mode AC-005 names: `cfg(doctest)` hides an
  orphaned module from every step but `cargo test`, so the reverse check is what
  catches a renamed/deleted atom whose module was left behind. **This is the
  precedent implementation for AC-005**, not a hypothetical.
- **No silent opt-out, partially precedented but not to AC-004's bar**:
  `xtask/src/lint_constitution.rs:601-643` (`check_fences`) already forbids an
  untagged fence (compiled as Rust by rustdoc regardless) and requires any `ignore`
  fence to carry a preceding `<!-- ignore: <reason> -->` HTML comment
  (`:638-643`), reasoning explicitly that without one it is "indistinguishable from
  an example that stopped compiling." **This is weaker than what AC-004 asks for**
  — AC-004 wants an *enumerated allowance list the same check reads*, not a
  per-fence inline justification comment. That is a real design choice this
  project's `_design.md`/architecture brief must make explicitly: adopt the
  existing per-fence-comment shape, tighten it to an enumerated list (a new
  constant analogous to `ATOM_DIR`), or do both. Flag it as a deliberate divergence
  from precedent if the enumerated-list route is chosen, since it means this
  project's check will not be a drop-in reuse of `check_fences`.
- **The router-consistency mechanism** (`check_router`,
  `xtask/src/lint_constitution.rs:179`, referenced but not fully read here) and the
  "derive the fact, keep the hand-written intention" pattern generally — see
  `standards/rust/81-checks-that-cannot-be-types.md:335-341` (RS-81-5) — is the
  house pattern for AC-007's citation resolution: `cargo xtask spec-trace`
  (`xtask/src/spec_trace.rs`) already parses `spec/SPECIFICATION.md` clause IDs by
  path (`SPEC` const, `spec_trace.rs:67`) and is the natural authority a new check
  should call into or reuse, rather than re-implementing clause-ID parsing.
  `spec_trace.rs`'s own module doc (`:1-57`) is worth reading in full before
  designing AC-007's check: it draws the same distinction this project needs
  between "the generator" (convenience) and "the equality check the gate runs
  without the flag" (load-bearing), and names its own blind spot pattern
  (`[PROVISIONAL]` markers with no falsifier) that AC-010 should imitate for this
  project's own documented blind spots.

## The gate wiring pattern (AC-002, AC-009)

`Step` is a plain struct (`name`, `program`, `args`, `env`, `probe`,
`xtask/src/main.rs:99-113`) pushed into the `REQUIRED` array
(`xtask/src/main.rs:105`). `standards/rust/80-the-gate.md` is the constitution atom
that governs this file and is the primary citation for the architecture brief:

- **RS-80-1** (`80-the-gate.md:11-17`): add a check as a `Step` in `REQUIRED`, never
  as a bespoke `Command` inside the runner — a bespoke command is invisible to
  `--fast` and to the `wasm`/`lints` subsets. AC-002's "mandatory step... with
  `probe: None`" is exactly this rule; `probe: None` is what makes a step
  unconditionally mandatory rather than skippable when a tool is missing
  (RS-80-2, `80-the-gate.md:98-104`).
- **RS-80-3** (`80-the-gate.md:181-187`): if the new step touches rustdoc, flags go
  in that step's own `env` as `RUSTDOCFLAGS`, never the ambient `RUSTFLAGS` — this
  is the exact gap AC-010 must name (CLAUDE.md's own text and the seed
  `references/seeds/user-documentation.md:55-63` both cite the `RUSTDOCFLAGS` gap
  as the canonical example of a check that looked wired and silently was not: "a
  documentation step that printed warnings and exited 0" — `RUNBOOK.md:919-924`
  narrates the actual incident). The `"documentation"` step at
  `xtask/src/main.rs:290-301` is the fixed version and is the concrete anchor for
  what "wired correctly" looks like.
- **RS-80-4** (`80-the-gate.md:245-251`): any gate invocation that resolves
  dependencies carries `--locked`.
- **AC-009's "clean checkout, no manual step"** maps directly to RS-80-1's
  "anything added as a bespoke `Command`... is invisible to `--fast`" framing:
  the new step must be a `Step` in `REQUIRED`, not a separate script a README asks
  a human to run.

## BR-10 / AC-008 — the clause-id pin, verified in detail

The charter's "the seed says nine" is precise but the set is **not enumerated
anywhere by clause id today** — confirming AC-008's own premise. What exists:

- `RUNBOOK.md:3777-3845` ("Between 5 and 6 — the reconciliation nothing owned")
  records that the phase 5/6 reconciliation "discharged nine documentation MUSTs"
  (`RUNBOOK.md:3837`) but only narrates the count, not the ids, and states plainly
  that no phase from 6 onward has since re-verified the *set* — only the standing
  exit criterion checklist at `:3810-3821` was added, which checks a *phase's*
  clause range, not this cross-cutting set.
- `references/evaluation/phase-4-5-reconciliation.md:119-142` is the actual
  evidence document and gives the closest thing to an enumeration: a table listing
  **ES-23, ES-24, VT-15, VT-17, VT-3, ES-17, ES-40**, plus two unnamed rows (`—`)
  that are explicitly *not* clause-MUST discharges ("a different animal... not an
  undischarged obligation but a comment asserting something false about the
  specification," `:139-142`), plus a ninth item named only as "ES-19's correction
  ... recorded by the pass rather than re-verified here" (`:136-137`). **Counting
  strictly by clause id from that table yields eight distinct ids** (ES-23, ES-24,
  VT-15, VT-17, VT-3, ES-17, ES-40, ES-19) against a stated total of nine — the
  arithmetic does not visibly close from this document alone. This is not a defect
  in this grounding pass; it is exactly the gap AC-008 exists to close. The
  architecture/testing briefs should treat "pin the exact nine (or reconcile the
  count) by clause id in one file" as real, unfinished work — not a formality —
  and should re-derive the set against `spec/SPECIFICATION.md` as it stands now
  (which may have shifted since `2a65d76`/`3c704d3`..`84dcc67`) rather than
  copying the table above uncritically.
- **Discharge sites**, for cross-checking against current `happenstance-core`, are
  named in that table: `crates/happenstance-core/src/store.rs:146-165` (ES-23),
  `store.rs:167-193` (ES-24), `crates/happenstance-core/src/tag.rs:29-47` (VT-15),
  `tag.rs:255-260` (VT-17), `crates/happenstance-core/src/event.rs:404-418`
  (VT-3/ES-17), `crates/happenstance-core/src/append.rs:29-47` (ES-40).
- **Where the pin should live** is itself an open design question this project
  owns (not decided by charter or decomposition): a natural candidate is a single
  new file enumerating clause ids that `cargo xtask spec-trace` can additionally
  assert against (the mechanism already parses clause ids from
  `spec/SPECIFICATION.md`, `xtask/src/spec_trace.rs:67`), or a constant list in
  the new tree's own gate-check module, following the `RULE_FILES`/`WIRE_TESTS`
  pattern of `spec_trace.rs:85-105` (an enumerated, commented `const` array is the
  house style for "the set of things a check must sweep").
- **Governance rule that binds any doc-comment rewrite touching a discharge
  site**: `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — "ask
  whether the edit changes what the document **asserts**, not whether it changes
  the document." A rename/refactor of a discharge site may be rewritten in place;
  reasoning inside a MUST-discharging comment that still holds may not be touched
  without deliberately re-discharging it. This atom is `authority_tier: guideline`
  (governance kind, status accepted) and is cited directly by BR-10's rationale.

## DT-7 — hidden content (AC-006)

Owned by this project per `_decomposition.md:156`. No existing code implements
tabs/folds/collapsed panels anywhere in this repository's documentation surfaces —
confirmed by the charter's own framing ("Filling that gap is closer to original
work than to adopting a convention" is said of diagrams, DT-5, not DT-7, but DT-7
is likewise greenfield: `.kb/design/README.md` holds zero atoms, and
`design.capture` is deliberately absent from `.redkiln/config.yaml`, so the
perceptual review is a skip — `_intake-brief.md` and `initiative.md:479-481` both
state this). There is no existing pattern to follow; the `_design.md` for this
project is the *only* record DT-7 will ever have, per the initiative's own framing.
AC-006 gives two legitimate resolutions — forbid hidden/tabbed content for
load-bearing material outright, or prove a broken claim inside a non-default panel
fails the gate — and either is compatible with the `check_fences`/`check_harness`
machinery above, since both are static-source checks over Markdown, not runtime
DOM checks.

## Precedence and non-goals this project must respect

- `standards/rust/README.md:23-29` — the five-tier precedence chain
  (**SPECIFICATION clause > ADR > constitution atom > CLAUDE.md/CONTRIBUTING.md
  summary > references/evaluation/***). This project's new tree sits inside this
  chain and does not extend it (`initiative.md:218-219`, `_intake-brief.md`
  Constraints) — its own discipline, wherever `page-need-discipline` (HS-P0021)
  lands it, is *not* a new precedence tier.
- No Accepted decision atom under `.kb/decisions/` governs CI/gate structure,
  documentation trees, or fence-compiling directly — verified by reading every
  atom's title (`.kb/decisions/000{1..9,10..16},0029`). The binding constraints for
  this project are conventions (CLAUDE.md, `standards/rust/80-the-gate.md`,
  `standards/rust/81-checks-that-cannot-be-types.md`), not ADRs. This project does
  not need to write or amend an ADR to do its work — nothing here changes a
  `[FROZEN]` clause or contradicts an accepted decision. **No tension with an
  Accepted ADR was found.**
- `standards/rust/81-checks-that-cannot-be-types.md:11-17` (RS-81-1: "Prove the
  check's blind spot in its own tests, then state it in its own documentation")
  is the direct precedent for AC-010. `xtask/src/constitution.rs:20-36` ("What this
  file does not prove") is a worked example of exactly this kind of blind-spot
  documentation — including naming the `RUSTDOCFLAGS` partial-recovery gap this
  project must also name for its own tree.
- `xtask/src/main.rs` `REQUIRED` array already has 20+ steps with names like
  `"documentation"` (`:290`), `"specification traceability"` (`:315`), `"the
  constitution's examples compile"` (`:488`) — a new step for this project's
  narrative tree slots into the same array, with a name following the same
  lower-case descriptive convention.

## Anchors this project's briefs should cite directly

- `xtask/src/lint_constitution.rs:55,58,61` — pinned-path constants (`ATOM_DIR`,
  `ROUTER`, `HARNESS`) — the template for AC-001's constant.
- `xtask/src/lint_constitution.rs:423-458` (`check_harness`) — the bidirectional
  orphan-page check — the template for AC-005.
- `xtask/src/lint_constitution.rs:601-643` (`check_fences`) — the existing
  `ignore`-fence rule — the baseline AC-004 must meet or knowingly exceed.
- `xtask/src/constitution.rs:1-140` — the `#[cfg(doctest)]`-per-file compiling
  mechanism — the template for AC-002.
- `xtask/src/main.rs:99-113` (`Step`), `:105` (`REQUIRED`), `:290-301`
  (`"documentation"` step, the `RUSTDOCFLAGS` fix in context) — the gate-wiring
  template for AC-002/AC-009/AC-010.
- `xtask/src/spec_trace.rs:1-105` — clause-id parsing and the
  written-vs-checked distinction — reusable for AC-007 and the pattern for
  AC-008's pin.
- `xtask/Cargo.toml:9-36` — why `xtask` carries the workspace crates as
  dev-dependencies, and why that is necessary for any fence to compile against
  real types.
- `docs/README.md:1-36` — the existing near-empty user-documentation tree; this
  project's pinned narrative tree is either this directory repurposed or a sibling
  to it, and `docs/README.md:25-29` already states the pin-by-path rule in prose.
- `RUNBOOK.md:3777-3845` and `references/evaluation/phase-4-5-reconciliation.md:79-142`
  — BR-10's evidentiary base, with the counting gap noted above.
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — binds any
  doc-comment touch at a MUST-discharge site.
- `standards/rust/80-the-gate.md` (RS-80-1 through RS-80-5) and
  `standards/rust/81-checks-that-cannot-be-types.md` (RS-81-1, RS-81-5) — the
  constitution atoms this project's architecture brief should load per
  `standards/rust/README.md`'s own trigger table ("adding a gate step" → `80`,
  then `81`).

## Tensions / risks to flag in the briefs

1. **AC-004 vs. existing `check_fences` precedent.** The existing `ignore`-fence
   rule in `standards/rust/` uses an inline `<!-- ignore: reason -->` comment, not
   an enumerated allowance list. AC-004 explicitly asks for the latter shape. Not a
   conflict with an Accepted decision — `standards/rust/` conventions are not ADRs
   — but a real architectural choice the brief must make and justify, since it
   means this project's check is not a mechanical copy of `check_fences`.
2. **BR-10's "nine" does not visibly reconcile from the cited evidence
   document alone** (eight named clause ids across the table plus one item named
   only in prose). AC-008 should be scoped as "enumerate and verify the exact set
   against `spec/SPECIFICATION.md` as it stands," not "copy the table from
   `phase-4-5-reconciliation.md`" — the two unnamed `—` rows in that table are
   explicitly *not* MUST-discharges and must not be miscounted into the nine.
3. **Where the pinned tree physically lives is undecided by the charter or
   decomposition** (`docs/` repurposed vs. a new sibling tree) and is called out in
   the decomposition as this project's to decide ("Hosting shape... is owned by
   `checked-documentation-surface`," `_decomposition.md:71-73`). No existing code
   presupposes an answer either way — `docs/README.md` is currently near-empty and
   available either way.
