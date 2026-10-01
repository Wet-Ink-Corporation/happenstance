# Roadmap — from `0.3.2` to `1.0.0`

Written 2026-09-28 at `f89e184`, when the runbook was split. The status of each
phase lives in the [index](README.md#status), not here; this file says **why the
order is what it is**, which is the part a status table cannot carry.

## Where it starts

Seven crates are published at `0.3.2` — `happenstance-core`, `happenstance`,
`happenstance-testkit`, `happenstance-sqlite`, `happenstance-cloudflare`,
`happenstance-postgres` and `happenstance-neon` — with no `todo!()` left in any of
them. `EventStore` has been `[FROZEN]` since `0.2.0` and `ProjectionStore` since
`0.3.0` (ADR-0063). The specification carries 141 `[FROZEN]`, 41
`[PROVISIONAL]`, 12 `[DEFERRED]` and 7 `[NON-NORMATIVE]` clauses — `cargo xtask
spec-trace`'s figures, and the ones to trust over this sentence.

What is not finished:

- **`happenstance-sync`** is a skeleton: the port traits and `MemorySyncPeer`
  exist, no runner does, `happenstance-sync-testkit` does not, and its four
  `todo!()` bodies are blocked on `happenstance-core` having no write path that
  preserves a foreign `EventId`. Phase 13.
- **Retention** has no answer: ES-39, CF-27 and SY-32 are `[DEFERRED]`, and one
  of ADR-0028's two answers adds a method to `EventStore`. Phase 14.
- **The typed layer's runner** is still behind `unstable-projection`:
  `Projection::apply` is synchronous and nothing owns that question; there is no
  failure-policy seam (PS-27) and no fan-out runner (PS-30).
- **`happenstance-ladybug`** is finished and cannot publish until `lbug` builds
  on docs.rs. *Since phase 17 it is retired (ADR-0078): excluded from the
  workspace and kept as a frozen record.*
- **Nothing defines 1.0.** No document in the repository states what it
  promises; `HS-I0006`'s charter names it an explicit non-goal
  (`.bklg/from-contract-to-published-library/initiative.md:170`), and `SECURITY.md`
  says *"Pre-1.0, and honestly so"*.

## The ordering rule

**Anything that could break a published crate is decided before 1.0, even where
the code that uses it lands after.** In `0.x` the minor is the breaking
position and a break costs a version number; after `1.0.0` it costs a major and
every downstream `Cargo.toml`. So the roadmap runs a **decision window** — phases
16 and 17 — ahead of the implementation phases, and asks each open question one
thing first: *does its answer change a published signature?* If it does, it is
answered in that window, whether or not its implementation is.

That rule is why two decisions move earlier than the phases that own them.
**ADR-0028's decision** moves from phase 14 to phase 17, because one answer adds
a method to `EventStore`. **ADR-0026's published-surface half** — whether
`happenstance-core` needs a write path that preserves a foreign identity — moves
to phase 17 for the same reason; the rest of ADR-0026 stays phase 13's.

## The sequence

```
12 ─▶ 15 ─▶ 16 ─▶ 17 [0.4.0] ─▶ 18 ─▶ 13 ─▶ 14 ─▶ 21 [1.0.0]
                   │                             ▲
                   ├─▶ 17b ──────────────────────┤   the additive half (ADR-0072), alongside 18
                   └─▶ 19b                       │   SQLite on wasm32; a candidate spoke for 13
                                                 │
off the path, and free to run alongside it:      │
      15 ─▶ 19a                                  │   SQLite on wasm32, the skeleton
      15 ─▶ 20 ──────────────────────────────────┘   documentation that teaches
```
| # | Phase | Why here | Days |
|---|---|---|---|
| 15 | [Reconcile the record](phases/15-reconcile.md) | Every plan-of-record document stopped between 2026-09-07 and 09-11 and four releases went out after. A plan read off a stale table sends somebody to build what is built — which has happened here once already, and held a release for it. | 2–3 |
| 16 | [Define 1.0](phases/16-define-1-0.md) | Nothing else can be sequenced against a target nobody has written down. | 2 |
| 17 | [The breaking window — `0.4.0`](phases/17-breaking-window.md) | The last cheap place to break a published signature. | ~~5–8~~ 25–30 |
| 17b | [After the window](phases/17b-after-the-window.md) | Phase 17's additive half, split off at the release by ADR-0072 so `0.4.0` does not wait on work that needs no window. Before 21, because three freezes ride it. | 8–10 |
| 18 | [The typed runner leaves its gate](phases/18-typed-runner.md) | An application author's runner is the one piece of the typed layer still marked unstable. Before 13, because it builds the convergence declaration sync's SY-20 rule is written against, while 13's rule for SY-21 exercises what this phase freezes. | 5–8 |
| 13 | [`happenstance-sync`](phases/13-sync.md) | After 17, because 17 may change the core surface a peer is built against, and after 18, whose convergence declaration SY-20's rule consumes. Inside 1.0 (D-1). | 12 |
| 14 | [Retention and completeness](phases/14-retention.md) | Builds what 17 decided about forgetting. | 5 |
| 19a | [SQLite on `wasm32` — skeleton](phases/19-sqlite-on-wasm.md) | Cheap, and its three answers set 19b's cost. | 3 |
| 19b | [SQLite on `wasm32` — the adapter](phases/19-sqlite-on-wasm.md) | After 17, against the `0.4.0` surface; a candidate offline spoke for 13. | 8–10 |
| 20 | [Documentation that teaches](phases/20-docs-that-teach.md) | 1.0 is a promise to a reader who is not the author. Planned in `.bklg/docs-that-teach/`, tracked here. | to be estimated |
| 21 | [`1.0.0`](phases/21-one-point-oh.md) | Last, by definition. | 3–5, plus a soak |

**Solo, the 1.0 path is about 62–75 working days**: 15, 16, 17, 17b, 18, 13, 14 and
21. It was 34–43 until phase 17 was re-estimated at its start (ADR-0072).
Sync is inside 1.0 (D-1 below), so 13 and 14 are on the path rather than beside it;
without them it would have been 17–26. Phases 19a and 20 run alongside and are not
counted. Every figure is an estimate carried into a repository whose own history
records estimates being wrong, and the session logs are where the actuals go.

## Decisions taken

Each changed the shape of the table, and each is recorded in the Weigh-In ledger
with a decision atom in `.kb/decisions/` (staged in `.kb/_intake/` until phase 15).

- **D-1 — `happenstance-sync` is inside 1.0** (`wi-40b321`, decided 2026-09-28).
  The recommendation was the opposite — ship 1.0 on the contract, typed layer,
  testkit and four adapters, with sync on its own `0.x` line — and was overridden.
  So phase 21 depends on 13 and 14, and the path is the longer figure above. What
  the recommendation feared is worth keeping in view: sync is the least-settled
  piece, and it now sits on the critical path to the promise.
- **D-2 — `.bklg/` is frozen, and this runbook tracks the rest**
  (`wi-016abe`, decided 2026-09-28). Decided as *close `HS-I0006` and re-plan*,
  then shaped by the retirement of redkiln the same day: there is no CLI to close
  it with, so nothing in `.bklg/` is advanced or closed. It stays as the record
  redkiln left, 190 stories and none at done, and the remaining work is planned
  in `phases/`. `CLAUDE.md` states the rule. Tracking is manual until `redkiln-rs`
  is live, after 1.0.
- **D-3 — delete the merged branches** (`wi-b9b9ab`, decided 2026-09-28). Six
  `lane/*` branches squash-merged in PRs #7–#13, and
  `wip/hs-p0012-benchmark-harness`, whose story was redone on `main` and shipped
  in `v0.2.0`. Executed the same day; the tips at deletion are recorded in
  [phase 15](phases/15-reconcile.md).
- **D-4 — what 1.0 is** (decided 2026-09-29, at phase 16; recorded in ADR-0066
  and ADR-0067, and in the Weigh-In ledger). Seven calls by the owner, each of
  which the rest of this table now leans on:
  - **Nine crates, and `happenstance-ladybug` is not one** (`wi-2798d5`). The seven published
    today, plus `happenstance-sync` and `happenstance-sync-testkit` by D-1.
    Ladybug keeps its own `0.x` line: `lbug` still does not build on docs.rs, and
    the route of rendering it without its `driver` feature is recorded, not
    measured. *Overtaken at phase 17: the crate is retired (ADR-0078), and that
    line will not be cut.*
  - **`workerd` before 1.0** (`wi-d61f21`). `happenstance-cloudflare`'s 1.0 claim is
    conformance on the real runtime, not on the `node:sqlite` shim it passes
    today. So phase 17 grows a `workerd` sibling job, and the SQL-text wall and
    ADR-0052's partition widths are measured there before they are promised.
  - **Lockstep, then independent** (`wi-8e5bd4`). All nine ship `1.0.0` together and version
    on their own from then on, each adapter naming the core it needs and the
    testkit it passes. Phase 17 builds the minimal-versions job that keeps those
    bounds honest; phase 21 chooses the release tooling.
  - **The MSRV holds at 1.97.1** (`wi-460397`), and after 1.0 a rise is bounded:
    only in a minor, only to a stable at least six months old, always in the
    changelog. The 2026-09-06 ratification that would have moved it is withdrawn
    — it was never executed.
  - **The rc soak has no time floor** (`wi-1408e8`). `1.0.0` follows the release
    candidate once ADR-0066 §8's five conditions hold on it; the recommended
    14-day floor was declined.
  - **Supported versions** (`wi-cbc941`): the latest `1.x` minor, plus security
    fixes on the previous major for six months after the next major ships.
  - **The licence** (`wi-7899af`): the nine crates stay `MIT OR Apache-2.0` for
    all of 1.x. A crate added later may be licensed differently.

  What it changed: the order above moved once — phase 18 now runs before 13,
  because SY-20's rule at 13 consumes a declaration 18 builds and the two had no
  order between them — and phase 17's work list grew by the `workerd` and
  minimal-versions jobs, ADR-0022 §9's reproduction and the guard-plan assertion
  (ADR-0068), `QueryItem`'s total constructor (ADR-0069) and the mint-per-open
  call for Postgres and Neon, none of which its estimate yet counts; phases 13, 14, 17 and 18 each gained an exit criterion naming the
  clauses they freeze; and every open phase gained one requiring the
  specification reconciled against its own changes.

## What is deliberately not on this roadmap

- **`happenstance-macros`.** Out of scope by ADR-0033, on a measured 0.50:1
  ceremony ratio against a 1:1 threshold. Reopened only by the conditions that ADR
  names.
- **DCB wire interoperability (WF-1).** Deferred on a reason that got stronger at
  phase 5: DCB publishes no wire format to interoperate with.
- **Measured-not-claimed performance publication** and **the licensing and
  commercial seam** (`references/seeds/`). Both are seeds with no initiative, and
  neither changes a published signature. Phase 16 was asked to decide whether
  either is a 1.0 question, and confirmed the first is not (ADR-0066 §9): it is
  largely discharged already — `benchmarks/` exists, and the benchmark-shaped
  falsifiers live in the disposition table. The second is a 1.0 question in one
  respect only: ADR-0066 §8 promises that the nine crates stay
  `MIT OR Apache-2.0` for all of 1.x (`wi-7899af`). The commercial seam and the
  trademark question stay open on their own triggers.
