---
item: HS-I0002
stage: intake
created: 2026-08-10T02:55:02.156Z
updated: 2026-08-10T02:55:02.156Z
template_sig: 9b551827
rendered_sig: d27c86c7
---

# Intake Brief — 0.2.0-alpha.1 — freeze the projection port and ship the typed layer

## Problem

Two of the three ports are settled and one is a guess. `EventStore` and the value
types froze at phase 4 against six skeleton crates and a conformance suite that
proves it can fail; the wire format froze at phase 5. `ProjectionStore` did not:
`grep -rn "ProjectionStore for"` matches nothing in the workspace, so the port is
shaped like nothing at all, and §4's clauses carry `[PROVISIONAL]` accordingly.
Above it, `happenstance` — the crate most people will `cargo add` — is still a
five-line facade over the contract. So there is no consumer that exercises the
contract as an application author would, and a consumer is what discovers contract
defects. Shipping an alpha with either hole open publishes a shape nobody has used.

## Desired Outcome

An `0.2.0-alpha.1` tag in which both holes are closed and both closures are
demonstrated rather than asserted: the projection port frozen against two
structurally unlike batch shapes with a hostile store *failing* its suite, and the
typed layer real enough that adding an event variant stops the crate compiling
until the fold handles it. After this, the contract has been used by something
other than its own tests.

## Constraints

- **Phase 4 must stay frozen.** Nothing here reopens a `[FROZEN]` ES or VT clause;
  if one has to move, that is a new ADR, written first.
- **Order is not negotiable.** Phase 6 before phase 7: the typed layer is written
  against the projection port, and writing it first means writing it twice.
- **The batch shape decides the port.** Dropping the `Batch` lifetime changes every
  skeleton that spells `type Batch<'a>`, so "the skeletons compile unchanged" is
  unsatisfiable by the phase's own decision and is not the bar.
- **Non-goals.** No adapter is finished here — SQLite is phase 8, and no skeleton
  becomes an adapter until it has run the suite. Nothing is published.

## Open Questions

Carried from `.kb/open-questions/`, and each is owned by a project below rather
than by this brief:

- Does the projection batch keep a lifetime, and what writes into it? (ADR-0017)
- How is a projection reset, and what may refuse it? (ADR-0018)
- What happens when `apply` fails? (ADR-0019)
- Does the port ship at 0.1 at all, or behind an off-by-default
  `unstable-projection` feature with a documented semver exemption?
- How does a decision model guarantee its query and its fold cannot disagree?
  (ADR-0020)
- How does a payload's shape evolve, and does the read path need a hook it does
  not have? (ADR-0021)

## Proof artefact

Two, one per project, and neither would exist if the design were wrong:

- **Phase 6** — `CheckpointOnlyStore` **failing** the projection suite, plus two
  implementations at opposite ends of the batch-shape axis passing it. A store
  that commits the checkpoint and silently drops the read-model write is the
  hostile case the port exists to forbid; if it passes, the port is not frozen.
- **Phase 7** — a `trybuild` compile-fail case: add an event variant, and the
  crate stops compiling until the fold handles it. A typed layer that keeps
  compiling when the domain grows is not typed.

## Clauses

- **PS-1 – PS-37** — `[PROVISIONAL]`, discharged by phase 6. PS-32, PS-33 and
  PS-35 leave the clause space entirely; PS-15 stays provisional on purpose,
  because only a generative brand closes the foreign-batch hole and the owned
  shape does not.
- **ES / VT** — `[FROZEN]` at phase 4 and untouched here.
- ADR-0007's Context is corrected at phase 6 (PS-32): a runner that itself writes
  into the batch cannot be written today; a callback-driven one can, and was
  compiled.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` and will not leave `intake` until
every one is ticked.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
