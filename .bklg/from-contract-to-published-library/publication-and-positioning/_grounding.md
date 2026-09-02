# Grounding — 0.2.0: where private opinions become promises

Companion to `_intake-brief.md`, read before authoring this project's briefs.
Every claim below cites a real path; verify with `test -f` before relying on it.

## Accepted decision atoms that constrain this project

- **`.kb/decisions/0004-edition-and-msrv.md`** (ADR-0004, accepted, phase 0).
  States the 1.85 floor is `provisional` and its own **Policy** section says the
  promise/preference line is drawn at "first publish (phase 12)". Its own
  **Status** section says it is amended, not superseded, by ADR-0029, and that
  "the body stays verbatim" — this project MUST NOT edit it (AC-006 says the
  same). It is the reasoning this project's new MSRV atom inherits, not a file
  to touch.
- **`.kb/decisions/0029-msrv-raised-to-1-97-1.md`** (ADR-0029, accepted, phase 2).
  Raises the floor to 1.97.1, `depends_on: [kb-decision-0004]`, and its own
  closing line is the direct charge for this project: *"Phase 12 must revisit
  the floor at first publish, when it stops being a preference and starts being
  a promise to a consumer who may not even build `happenstance-sqlite`."* Also
  unmodified per AC-006 — the new atom supersedes neither, it is a sibling that
  the promise-conversion produces.
- **`.kb/decisions/0006-bare-name-to-the-typed-layer.md`** (ADR-0006, accepted).
  Establishes `happenstance` as the typed layer over `happenstance-core`; this
  is why `happenstance` (not `happenstance-core`) is the crate a `cargo add`
  evaluator meets, which is load-bearing for DT-1 and the README's "Quick
  start" section.
- **`.kb/decisions/0010-the-suite-must-prove-itself.md`** (ADR-0010, accepted,
  phase 3). Every conformance rule carries a mutant with stated provenance and
  a skip is always reported, never silent. This is the evidentiary backbone
  AC-009's compliance claim points to — "DCB-compliant" is checkable because
  this ADR is what makes a passing rule mean something.
- No other accepted decision atom names publication, MSRV promotion, or the
  clause ledger directly. `.kb/decisions/` currently holds exactly
  0001–0016 and 0029 (`ls .kb/decisions/`) — nothing in the 0017–0028 range
  exists yet, confirming AC-013's numbering constraint: this project's new
  atoms belong at **0030 and up**.

## The instruments this project extends, not invents

- **`xtask/src/package.rs`** — `PUBLISHABLE` (line 86) is the **intention**
  list: `["happenstance-core", "happenstance", "happenstance-testkit"]`. AC-001
  asks for the derived-vs-intention reconciliation this file already performs
  (`publishable_members` derives from `cargo metadata`, `reconcile` fails on
  disagreement) — the deployment brief should point at this file rather than
  design a new mechanism. Six adapter crates currently declare
  `publish = false` (`crates/happenstance-{sqlite,cloudflare,ladybug,postgres,neon,sync}/Cargo.toml:12`).
- **`xtask/src/spec_trace.rs`** — already parses every clause's maturity marker,
  writes §7.1/§7.2, and *checks but never writes* §1.3 (the prose census),
  specifically because generating the one count a human verifies by reading it
  would remove the independent check. AC-003's "audit reconciles... a seeded
  disagreement fails the check" is this file's existing contract, not new
  scope — read its module doc (`spec_trace.rs:1-50`) before proposing a new
  clause-ledger tool.
- **`CONTRIBUTING.md:291-296`** already runs `cargo-semver-checks` on every PR
  with `--baseline-rev` pointed at the branch point, and is explicit about what
  that job does *not* prove ("nothing about the last [release]"). AC-002's
  registry-baseline diff is a second, distinct comparison this project adds —
  ground the testing brief in the distinction CONTRIBUTING.md already draws.
- **`RUNBOOK.md:4448-4512`** ("Phase 12 — Publish `0.2.0`") is the closest
  thing to an existing plan for this project, but it predates the decomposition
  gate's decision and is stale in two concrete, citable ways (see Tensions).

## The falsifier ledger and clause totals (AC-003, AC-004) — verified against the primary sources

- `spec/SPECIFICATION.md:220` states **"139 `[FROZEN]`, 49 `[PROVISIONAL]`, 10
  `[DEFERRED]` and two"** (the "two" being the non-normative-demoted count) —
  this is the exact 200/198/139/49/10/2 figure AC-003 cites; it is real and at
  that line.
- `RUNBOOK.md:588-635` ("The 49 `[PROVISIONAL]` clauses") is the falsifier
  ledger AC-004 names. It is currently short by exactly the four rows AC-004
  lists, in the runbook's own words (`RUNBOOK.md:628-631`): *"the groups above
  are short by **ES-41, ES-42, CF-39 and CF-40**, and the last row still
  carries **ES-10**, which is no longer provisional. That is five edits."*
  AC-004 is verbatim this repair, already scoped and self-diagnosed — the
  testing/deployment briefs should cite `RUNBOOK.md:622-635` directly rather
  than re-deriving the count.
- The ledger's own methodology note (`RUNBOOK.md:618-620`) is the precedent for
  AC-004's "by check rather than by hand count" clause: it was already checked
  once against `spec-trace`'s own `[PROVISIONAL]` list and agreed only "at that
  commit" — i.e. hand-reconciliation rots, which is the argument for making the
  reconciliation itself a `cargo xtask` step rather than a one-time count.

## PS-3 and the projection-port evidence (AC-012)

- `RUNBOOK.md:601` names PS-3 in the ledger: *"Ship behind `unstable-projection`
  | PS-3 | the two batch shapes disagreeing at phase 6 | 6, decided at 12."*
  Phase 6 is `projection-store-freeze` (HS-P0010); its evidence plus
  `ladybug-projection-store`'s (HS-P0015) verdict on whether a structurally
  unlike batch shape held the freeze is what AC-012 asks this project to cite,
  not re-derive. `spec/SPECIFICATION.md:4777-4784` already writes the two
  candidate outcomes ("ships behind `unstable-projection` ... with a documented
  exemption from semver" vs. "Rejects: publishing a 0.1 whose most defective
  surface is semver-binding") — ground the PS-3 write-up here rather than
  inventing new language for the same fork.

## The registry crate-set tension (flag explicitly, do not silently pick three)

`RUNBOOK.md:4450-4451` ("Phase 12 — Publish `0.2.0`") states the goal as
*"`happenstance-core`, `happenstance`, `happenstance-testkit` **and
`happenstance-sqlite`** on crates.io"* — **four** crates. This directly
disagrees with:
- `CLAUDE.md`'s own commands section: *"a `cargo package --list` assertion that
  each of the **three** publishable crates carries both licence files and a
  README."*
- `xtask/src/package.rs:86`'s `PUBLISHABLE` constant — three names, matching
  CLAUDE.md.
- The decomposition's own carried-forward flag
  (`_decomposition.md:302-309`, "Which crates publish at `0.2.0`"): *"Working
  default: three crates only, since AC-03 and DoD 9 name the installable crate
  in the singular... But the charter never says so, and the evaluator persona
  will look for a SQLite adapter on the landing page... if the answer is more
  than three, each adapter project owes a name reservation and a README."*

This is not a new tension this grounding pass discovered — it is the same one
the decomposition already named and explicitly deferred to this project's
deployment brief. `RUNBOOK.md`'s phase-12 section is the stale artifact (it
also cites a superseded `#the-46-provisional-clauses` anchor at line 4495,
consistent with predating the phase-4 recount to 49 documented at
`RUNBOOK.md:622-635`). **The deployment brief must decide the crate set
explicitly and record which of the two conflicting sources it is overriding**
— AC-001 requires the decision to name "the rejected alternative", and this is
exactly that fork. Given `sqlite-durable-store` (HS-P0012) is upstream in merge
order and CLAUDE.md's package-check already encodes three, the lower-risk
default remains three unless the deployment brief finds new information — but
it must say so, not inherit the runbook's stale four silently.

## `!Send` / wasm32 surface (AC-014)

- `CLAUDE.md` constraint 1 and `.kb/decisions/0001-async-port-flavours.md`
  govern the two-flavour design; `README.md:159-164` already documents that CI
  builds `happenstance-core` for `wasm32-unknown-unknown` on every commit.
  AC-014 extends this to the **published tree** and the **published feature
  set** admitting the `!Send` flavour — ground the deployment brief in the
  existing `cargo xtask wasm` / `cargo xtask ci`'s four wasm32 steps (CLAUDE.md,
  "Commands" section) rather than proposing a new check.

## The registry-facing landing surface (AC-007, AC-009, AC-010, DT-1/4/5/6)

- `README.md` (repo root) is the current landing page and is written for
  `0.1.0` (`README.md:11-16`, the status callout; `README.md:79-98`, the status
  table listing four 🔲 stubs). It has **no existing "DCB-compliant" badge or
  claim string** to redirect (verified: no match for that phrase anywhere in
  `README.md` or any crate README) — AC-009's evidence link is new copy, not an
  edit to existing copy.
  `README.md:227-234` is the precedent for how this project should *phrase* a
  claim with evidence attached: the former-name section cites `ADR-0005` by
  path and states the fact plainly rather than asserting authority.
- `README.md:100-137` already establishes the doctest-compiled-by-`xtask`
  pattern ("This block ... [is a] doctest of `xtask`, not of a published
  crate") and explains why each crate's own README is what the packaged
  artifact carries. Any new landing copy for DT-1/DT-4 should follow this
  crate-README-is-the-package-surface precedent, not add prose only to the
  workspace root README.
- No `.kb` atom or open-question resolves DT-1, DT-4, DT-5, DT-6, or the
  evaluator-vs-application-author persona question
  (`_decomposition.md:310-314`) — grepped `.kb` for each DT id and found no
  hits. These are this project's own `_design.md` work (AC-011), not
  inheritable from anywhere in the tree.

## Code patterns to follow

- **Derive-and-reconcile, not derive-only or list-only**
  (`xtask/src/package.rs:23-46`'s module doc spells out why both the derived
  fact and the hand-written intention are kept). AC-001's "reconciles with its
  intention list" is this exact pattern; reuse it rather than choosing one side.
- **Check but do not generate the human-verified count**
  (`xtask/src/spec_trace.rs`'s module doc, the "§1.3" section). AC-003's clause
  audit should follow this precedent: `spec-trace` already owns generating
  §7.1/§7.2 and *checking* (not writing) §1.3 — a new publish-time audit step
  should compose with this existing split rather than duplicate the parser.
- **A skip is reported, never silent** (`.kb/decisions/0010-the-suite-must-prove-itself.md`,
  itself echoing `CLAUDE.md`'s testkit section on declined `Capability`
  constants). AC-005's "consumer-readable stated reason" for each `[DEFERRED]`
  clause is the same discipline applied to documentation instead of test
  output.

## Tensions to flag in the briefs

1. **Crate-set disagreement** — `RUNBOOK.md:4450-4451` (four crates, including
   `happenstance-sqlite`) vs. `CLAUDE.md` / `xtask/src/package.rs:86` (three).
   Detailed above; the deployment brief owns resolving it explicitly.
2. **`RUNBOOK.md`'s phase-12 section is written against a stale ledger** — it
   references "[the provisional ledger](#the-46-provisional-clauses)"
   (`RUNBOOK.md:4495`), but the ledger heading itself was corrected to 49 at
   `RUNBOOK.md:622-628` following the phase-4 recount. Cite the corrected
   count (`spec/SPECIFICATION.md:220`, `RUNBOOK.md:623-624`), not the phase-12
   section's anchor text, when the brief states the total.
3. **No brief in this project is `architecture`** (`_decomposition.md:261`,
   the warranted-briefs table) — confirmed against the table; this project
   carries `ux`, `testing`, `deployment` only, consistent with
   `_intake-brief.md`'s constraint *"This project carries no `architecture`
   brief. It makes promises about a surface it does not design."* Do not
   introduce one.
