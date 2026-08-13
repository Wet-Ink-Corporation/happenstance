---
item: HS-P0016
stage: intake
created: 2026-08-12T03:23:23.997Z
updated: 2026-08-12T03:23:23.997Z
template_sig: ab516678
rendered_sig: c2a4e9ef
---

# Intake Brief — 0.2.0 — where private opinions become promises

## Problem

Everything this library believes is currently private and revocable. The MSRV is a
build setting; ADR-0004 still carries a `provisional` marker for exactly that
reason. Forty-nine clauses are provisional, and a provisional marker with no
falsifier is indistinguishable from a decision nobody wanted to make. Semver
breakage is invisible on review — code that violates semver does not look wrong —
and publication does not stop a crate shipping in a state that looks unfinished on
its own landing page; it fails silently. Publishing converts all of this into
promises that cannot be withdrawn, and it is worth doing only if each one is true
at the moment it is made.

## Desired Outcome

`happenstance`, `happenstance-core` and `happenstance-testkit` are published at
`0.2.0`, and each promise has been checked rather than asserted. We know it worked
when: `cargo-semver-checks` has run against the `0.2.0-alpha.1` registry baseline
and the version chosen matches what the diff found (DoD 11); the clause-ledger
audit reports every clause frozen, demoted, or carrying a **non-empty** falsifier,
with the report's count reconciled against `spec/SPECIFICATION.md`'s own figure
(DoD 12); docs.rs is green under `--all-features` and the licence, description and
README have been checked **by looking at the rendered registry page** (DoD 10); and
a scratch project outside this workspace installs the published version — not a
path dependency — and completes a write-then-read cycle (DoD 9, AC-03).

## Constraints

- **Depends on all five upstream projects**: `typed-layer-and-alpha-release`,
  `sqlite-durable-store`, `cloudflare-durable-object-store`,
  `postgres-and-neon-stores`, `ladybug-projection-store`. This was decided at the
  decomposition gate: `RUNBOOK.md` makes phase 12 depend on 7 and 8 only, and the
  gate deliberately added the other three, because AC-08 requires the published
  compliance claim to name **which** implementations it was checked against, and
  publishing after SQLite alone would publish a one-adapter claim. Cost accepted:
  ~25–30 runbook-days between the alpha and `0.2.0`.
- **This project carries no `architecture` brief.** It makes promises about a
  surface it does not design.
- **`0.2.0` is the ceiling.** Under 0.x the minor bump *is* the ecosystem's
  breaking-change signal; that is the promise being made and no larger one.
- **Non-goals**, each naming its owner: creating the semver baseline →
  `typed-layer-and-alpha-release` (already done); publishing `happenstance-sync` or
  `happenstance-sync-testkit` → out of this release train entirely, per the
  charter; any `1.0` or post-1.0 commitment; answering the two silences →
  `replication-identity-and-ingest`, `retention-and-incomplete-logs`.

## Open Questions

- **DT-1 — which claim leads on first contact?** Storage-agnostic proof, the
  constrained-runtime story, or two entry points, one per audience. Leading with the
  edge story narrows the perceived audience; leading with the proof asks the reader
  to care about a claim they cannot yet check.
- **DT-4 — where is the positions-and-gaps promise stated for a newcomer?** Landing
  page, specification only, or both with the landing page pointing. Two DCB-labelled
  stores already disagree on this in public; a reader who never finds the statement
  depends on the wrong one by accident.
- **DT-5 — is the clause maturity vocabulary published?** Publishing it is unusually
  honest and unusually noisy; not publishing it hides exactly the signal an
  evaluator wants.
- **DT-6 — is an explicit comparison to the nearest live peers stated?** Silence on
  a peer that shipped the day before intake can read as an oversight; comparison
  invites maintenance and dates badly.
- **PS-3** — does the projection port ship frozen, or behind `unstable-projection`?
  Decided here, on evidence supplied by `projection-store-freeze`.
- **Which crates publish at `0.2.0`?** Working default: **three only**, since AC-03
  and DoD 9 name the installable crate in the singular, and `CLAUDE.md`'s
  `cargo package --list` assertion covers three. But the evaluator persona will
  look for a SQLite adapter on the landing page. If the answer is more than three,
  each adapter project owes a name reservation and a README — **and that changes
  their scope, so decide it early.**
- **AC-08's evidence** — where does the "DCB-compliant" claim point, which
  implementations does it name, and dated when?

## Proof artefact

**The stranger-install smoke: a scratch project outside this workspace that adds
the published crate from the registry and completes a write-then-read cycle
against the published version, not a path dependency.** This would not exist if the
design were wrong: every check inside this repository runs against a workspace that
resolves paths, features and dev-dependencies in ways a consumer never sees, so a
crate that is unusable once published would pass the entire local gate. Paired with
it: the `cargo-semver-checks` report against the alpha baseline, and the clause
ledger audit whose count reconciles against the specification's own stated figure.

## Clauses

- **Every `[PROVISIONAL]` clause in the specification (≈49)** — audited at the
  moment of publish; each must end frozen, demoted to non-normative prose, or
  carrying a **non-empty falsifier**. This is BR-06 and DoD 12.
- **The five known-short ledger rows** `RUNBOOK.md:622-635` says cannot wait past
  this point: **ES-41**, **ES-42**, **CF-39**, **CF-40**, and **ES-10**'s removal.
- **PS-3** — decided here (ship frozen, or behind `unstable-projection`).
- **The 10 `[DEFERRED]` clauses** — each re-read for whether the deferral is still
  honest at publish, and each given a stated reason a consumer can read.
- **ADR-0004 loses its `provisional` marker** — the MSRV becomes a promise with a
  recorded justification (BR-08, AC-12).
- Nothing `[FROZEN]` is amended. If the audit finds a frozen clause that is wrong,
  that is a new decision atom and a re-plan, not a line edit — and it blocks the
  release.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
