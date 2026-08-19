---
item: HS-P0011
stage: intake
created: 2026-08-12T03:23:07.071Z
updated: 2026-08-12T03:23:07.071Z
template_sig: ab516678
rendered_sig: e3d2d97f
---

# Intake Brief — The typed layer, the worked example, and 0.2.0-alpha.1

## Problem

A contract is discovered by use, and this one has never been used. `happenstance`
— the crate holding the bare name, the one most people will `cargo add` — is a
glob re-export five lines long (`crates/happenstance/src/lib.rs:75`). Nothing has
played the role of consumer, so no contract defect has been found by anything
other than a type checker looking at a skeleton. `RUNBOOK.md:244-258` is explicit
that doing this *after* the flagship adapter is the sequence the plan exists to
avoid.

## Desired Outcome

A real typed layer sits above the facade and a worked example exercises it the way
an application would. We know it worked when `cargo run -p course-subscriptions`
completes the canonical DCB cycle with no `todo!()` reached (DoD 1), and when the
`trybuild` compile-fail case on an unhandled event variant is green **and** fails
if the protection is removed (DoD 2 — the negative control is the point). The
contract defects this use discovers are recorded as they are found (BR-01), not
quietly absorbed. `0.2.0-alpha.1` is published, which is the first registry
artefact this workspace has ever had and therefore the semver baseline that does
not currently exist.

## Constraints

- **Depends on** `projection-store-freeze` (HS-P0010) — this project consumes the
  frozen port; it does not freeze it.
- **Blocks** `sqlite-durable-store`. This is `RUNBOOK.md`'s phase 7 → phase 8 edge
  and it is not negotiable.
- **ADR-0003 constrains `happenstance-core`, not this crate.** Read the crate name
  carefully: after ADR-0006's rename, `happenstance` is the *typed* layer, whose
  entire job is encoding. It is the crate that will depend on `serde`, and
  forbidding it here would forbid the thing the split exists to allow.
- **Never introduce `#[async_trait]`** (ADR-0001); **`EventStore::read` returns the
  stream at the top level and is not `async`** (ADR-0001, ADR-0008) — two tests in
  `memory.rs` assert this and it takes both.
- **Bind `EventStore`, not `SendEventStore`**, in generic code, and import only one
  of the two names per module.
- **Non-goals**, each naming its owner: the MSRV promise, the semver diff verdict,
  the clause-ledger audit and registry presentation → `publication-and-positioning`
  (this project publishes an alpha to *create* the baseline; it does not make the
  promises); any storage adapter → `sqlite-durable-store`,
  `cloudflare-durable-object-store`, `postgres-and-neon-stores`; freezing
  `ProjectionStore` → `projection-store-freeze`.

## Open Questions

- **DT-2 — how much must a caller state before their consistency boundary is
  checked?** Minimal ceremony with more caught later, or explicit declaration with
  more caught at build time. This is error timing versus first-hour cost, and the
  audience is new to Rust's idioms as often as not.
- **ADR-0020** — how a decision model guarantees its query and its fold cannot
  disagree.
- **ADR-0021** — payload evolution: the codec tag, versioned event types,
  upcasting, and whether the read path needs a hook it currently lacks.
- **PS-33** — ADR-0007's falsifier, evaluated at this project's exit and nowhere
  else.
- Whether `happenstance-macros` meets its own in-scope-for-0.1 criterion. Evaluated
  and recorded either way.
- The cost of N views × N reads in the polling runner — recorded as a measurement,
  not an estimate.

## Proof artefact

**The `trybuild` compile-fail case on an unhandled event variant, together with its
negative control** — the demonstration that removing the protection makes the case
fail. This would not exist if the design were wrong: a typed layer that claims the
compiler protects the domain, but whose protection nothing checks, is a convention
rather than a guarantee, and only a case that fails when the guard is removed can
tell the two apart. Secondary and equally required: `cargo run -p
course-subscriptions` completing the full cycle with no `todo!()` reached.

## Clauses

- **PS-33** — ADR-0007's falsifier, evaluated here.
- **ES-*** `[FROZEN]` — the `EventStore` contract is consumed, never amended. Any
  defect this use discovers is recorded as a contract defect and routed to a
  decision record; it is not fixed by a line edit.
- **WF-11** `[PROVISIONAL]` — the human-readable encoding question is touched by the
  codec-tag work in ADR-0021 but its falsifier is
  `cloudflare-durable-object-store`'s.
- No clause is amended by this project. ADR-0020 and ADR-0021 add records; they do
  not move a maturity marker on a frozen clause.

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
