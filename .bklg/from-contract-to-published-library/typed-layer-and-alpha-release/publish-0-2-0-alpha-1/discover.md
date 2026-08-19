---
item: HS-S0033
stage: discover
created: 2026-08-12T13:01:55.491Z
updated: 2026-08-12T13:01:55.491Z
template_sig: 86ce4036
rendered_sig: bd8fa581
---

# Discover — 0.2.0-alpha.1 on the registry, with its churn mitigations

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice: cut `0.2.0-alpha.1` for `happenstance-core` **then** `happenstance` (`happenstance-testkit` on its own CF-32 number), after **one full `cargo xtask ci` — not `--fast`**, with the README `## Stability` section, `CHANGELOG.md`'s `## [Unreleased]` becoming the alpha's section *and its `unstable-projection` claim reconciled with the manifest*, the yank policy stated, and post-publish resolution verified with an explicit pre-release requirement | `_storymap.md:64` (M7 row) | Six obligations, one of them irrevocable, and one of them a discrepancy that exists in the tree right now |
| AC-011 — the published version resolves, carries a README `## Stability` section naming the phase at which the API stops moving, a `CHANGELOG.md`, and the yank policy for superseded alphas; a later `cargo-semver-checks` run has a baseline to compare against | `project.md:200-203`; DoD 7 at `project.md:239-240` | The mitigations are acceptance criteria, not intentions |
| `depends_on`: `compile-fail-proof-artefact`, `projection-clause-verdicts`, `edge-flavour-and-wasm-claim`, `defect-log-and-macros-verdict` — the four M7-and-before stories whose outputs the release either ships or asserts. *"The publish is the one irrevocable act in the project"* | `_storymap.md:64`, `:130-132` | Everything else in the project can be revised. This cannot |
| DEP-001 — the publishable set is the three crates carrying no `publish = false`: `happenstance`, `happenstance-core`, `happenstance-testkit`. The other six skeletons are `publish = false` and untouched | `_decomposition.md:909-923` | Confirmed against all nine manifests by the deployment brief |
| DEP-004 — publish order matches the dependency graph: `happenstance-core` **before** `happenstance`, because crates.io rejects a manifest whose dependency cannot resolve against the registry yet. `happenstance-testkit` has no ordering dependency on `happenstance` | `_decomposition.md:945-952` | The order is a hard constraint, not a preference |
| DEP-002 — the pre-release is unreachable by accident: Cargo will not resolve a pre-release without an explicit pre-release requirement in the consumer's manifest. No feature flag is needed; **the version string is the gate** | `_decomposition.md:924-930` | This is Cargo's mechanism, not something this project builds |
| DEP-006/DEP-007 — a crates.io publish is **irrevocable**: `cargo yank` hides a version from *new* resolution but does not delete it, and a published version cannot be edited. Rollback means yank-the-mistake and publish the fix at a new version, with a `CHANGELOG.md` entry saying what broke between alphas | `_decomposition.md:964-990` | There is no server to roll back and no traffic to shift. Verification is a resolution check |
| **Why the full gate, not `--fast`.** `run_fast`'s own doc comment says it unprompted: *"`REQUIRED` without `OPTIONAL` — the project-scoped bar, **not the release bar**"*. `--fast` drops the feature powerset, `cargo deny` and the nightly `--cfg docsrs` build; two of those matter directly to publish-readiness | `_decomposition.md:1025-1054`, citing `xtask/src/main.rs:835`; `_storymap.md:64` | The powerset is the only instrument proving the codec features compose; a licence violation caught post-publish is a legal problem, not a follow-up commit |
| **A discrepancy that exists in the tree today and must not ship.** `CHANGELOG.md` states *"**`ProjectionStore` ships behind an off-by-default `unstable-projection` feature** and is exempt from semver until two adapters at opposite ends of the batch-shape axis have passed its conformance suite"* — and `crates/happenstance-core/Cargo.toml`'s `[features]` block carries `default`, `std`, `serde` and `memory` only | `CHANGELOG.md:19-22` (read directly); `_decomposition.md:701-710` (Tensions 4), `:1003-1023` | *"Today that is a claim in an unpublished file; at this project's exit it becomes a claim a stranger can check."* Either the feature lands or the line is corrected — **before** the cut |
| The README mount, settled by the design against two briefs: `## Stability` goes **above the first code block**, replacing the status blockquote — not between `## Guarantees` and `## Design`. AC-U18 wins on ordering, and the blockquote asserting *"this crate is currently a facade… adds nothing yet"* is **false the moment this project ships** | `_design.md:124-145`, `:1249-1251`; `crates/happenstance/README.md` headings (`# happenstance`, `## Which crate do I want?` `:13`, `## What DCB buys you` `:22`, `## Guarantees` `:41`, `## Design` `:51`, `## Licence` `:58`) | Two stability claims on one page, one of them wrong, is worse than either |
| `## Stability` carries three things and nothing else: the phase at which the API stops moving in the reader's terms, a pointer to `CHANGELOG.md`, and the yank policy — *"only one pre-release resolves at a time; each alpha is yanked when the next lands"* | `_design.md:789-791` | A stated policy that is never executed is the mutant below |
| The README is **compiled as a doctest** through `crates/happenstance/src/lib.rs:10`, so it must not carry an uncompilable fence | `_decomposition.md:426`; `crates/happenstance/src/lib.rs:1-10` | A `## Stability` section with a stray fence turns the release into a broken build |
| **`happenstance-testkit` publishes `0.2.0-alpha.1`**, not a stable `0.2.0` — CF-32 gives it an independent *number*, not an independent *maturity*, and a stable conformance suite over a port still moving is the promise this project refuses to make. Signed off explicitly | `_design.md:649`, `:1252`; `_decomposition.md:1056-1072` (the deployment brief flagged it; the design settled it) | Its in-tree `0.2.0` moves down |
| DT-1 — which claim leads on first contact — is HS-P0016's. The README's opening two lines are **not touched**; `## Stability` is this project's only claim on that page | `_design.md:142-145`, `:989` (anti-pattern 8) | Positioning is out of scope and easy to change by accident here |
| The packaging gate already exists: `cargo xtask ci`'s *"packaged artifacts carry their licences and README"* step asserts via `cargo package --list` that all three publishable crates ship correctly | `_decomposition.md:953-963`; `CLAUDE.md`, *Commands* | Require it green; do not re-describe it |
| `cargo publish` is a **human handoff**, not an implementation step | `_storymap.md:163-165`; `_decomposition.md:1074-1083` (no workflow references `publish` or `crates.io` today) | Plan it as a handoff at the point the material is ready |
| The names were reserved at phase 0 and verified free on 2026-08-06 — exactly the three this release touches | `_decomposition.md:900-905`, citing `RUNBOOK.md:798-803` | Name reservation is not this story's concern |
| The failure mode the mitigations guard against is the **authors**, not the users: *"a published alpha makes 'we can't change that now' available"* | `project.md:265` (risk row 1); `_decomposition.md:931-944` | The whole point of publishing here is feedback about decisions still worth challenging |

## Questions

**Answered here.**

- *Which gate before the cut?* The **full** `cargo xtask ci`, once, immediately before
  publishing — in addition to the `--fast` runs every story already passed, not as a
  replacement for the project's DoD-6 bar (`_decomposition.md:1050-1054`).
- *What order?* `happenstance-core`, then `happenstance`; `happenstance-testkit` any time
  after `happenstance-core` (`_decomposition.md:945-952`).
- *What version does the testkit carry?* `0.2.0-alpha.1`. Settled at design sign-off
  (`_design.md:1252`), reversing nothing — the deployment brief flagged it as open and the
  design closed it.
- *Where does `## Stability` sit?* Above the first code block, replacing the status
  blockquote. The design explicitly overrides the architecture and deployment briefs'
  `## Guarantees`/`## Design` placement (`_design.md:128-140`, `:1249-1251`) — a rule that
  seemed wrong, fixed, with the reason given in the same document.
- *Is there a migration or backfill?* No, explicitly: nothing persisted is touched, and every
  durable adapter carries `publish = false` (`_decomposition.md:994-1001`).

**Deferred to `spec`, each with a decider.**

- *How the `CHANGELOG.md` / manifest discrepancy is resolved.* Two admissible resolutions —
  the `unstable-projection` feature lands in the manifest, or the changelog line is corrected
  — and the choice follows `projection-trait-and-runner`'s gating decision, which already
  settles this crate's side (`_design.md:647`). **Release-blocking either way**
  (`_decomposition.md:1020-1023`).
- *The exact wording of `## Stability`'s "phase at which the API stops moving".* It must be in
  the reader's terms rather than in runbook phase numbers, which is a writing problem the spec
  owns.
- *Whether `Cbor` ships.* Conditional on the licence verdict (`_design.md:739`); if refused,
  the alpha ships `json` + `postcard` and the changelog records the reason. Decided by
  `codec-and-feature-forwarding`, consumed here.
- *The post-publish verification script.* Manual by nature — `cargo xtask ci` cannot reach the
  live registry (`_decomposition.md:964-977`), so DEP-006 is an explicit acceptance step, not
  an automated assertion (`_storymap.md:100-102`).

**Not blocked** on `MemoryProjectionStore`. **Transitively gated on `trybuild`** through
`compile-fail-proof-artefact`, which is a `depends_on` edge: AC-002 is this project's proof
artefact and DoD 2 requires the case and its negative control in the gate, so a release cut
while that story is still blocked would ship a project whose central claim has no instrument.
That is HS-P0010's input to supply and this story's reason to wait.

## Decision

The problem this slice solves is that everything this project has built is currently a claim
nobody outside the repository can check, and the initiative's later work — the semver diff,
the surface audit, the positioning — all needs a baseline that does not exist until something
is published. This story cuts that baseline: `0.2.0-alpha.1` for the contract crate and the
typed layer, with the testkit on its own number, behind a full gate rather than the
project-scoped one, and wrapped in the three mitigations that keep a pre-release from
hardening into a commitment nobody agreed to make — a `## Stability` section a reader meets
before the first code block they would copy, a changelog that says what changed between
alphas, and a yank policy that keeps exactly one pre-release resolvable. The spec for this
story covers the pre-publish full `cargo xtask ci` run and why `--fast` is not the release
bar, the `CHANGELOG.md`/manifest reconciliation as a release blocker, the README edit
(blockquote deleted, `## Stability` in its place, opening two lines untouched, no
uncompilable fence), the three version strings and the publish order, the yank policy stated
and its execution rule for the next alpha, and the post-publish verification — resolving each
crate from a clean environment with an explicit pre-release requirement, plus a best-effort
docs.rs check. The publish itself is a human handoff. No `[FROZEN]` clause is amended.

## The wrong implementation

**The mutant: cutting the release after `cargo xtask ci --fast`.**

It is green. It is *this project's own integration bar*, named in DoD 6 and wired into
`.redkiln/config.yaml`, so using it is defensible on paper and nobody in review objects.
Every story passed it. All four `wasm32` steps ran. The packaging step ran. `spec-trace`
ran. The three crates publish cleanly, in the right order, and resolve.

It is wrong because `--fast` is `REQUIRED` without `OPTIONAL`, and `run_fast`'s own doc
comment says what that means without being asked: *"the project-scoped bar, **not the release
bar**"* (`xtask/src/main.rs:835`, cited at `_decomposition.md:1030-1033`). Two of the three
dropped steps matter at exactly this boundary and nowhere else:

- **The feature powerset** is the only instrument that proves AC-005's CBOR/postcard-behind-features
  claim and AC-A04's forwarding claim hold *in combination*. The alpha therefore ships a
  `cargo add happenstance --no-default-features --features cbor` configuration that nothing
  has ever compiled — and the first person to try it is a stranger reading the registry page.
- **`cargo deny check`** is the licence/advisory gate, and a crate cannot un-publish its way
  out of a licence violation. `cargo yank` hides a version from new resolution; it does not
  delete it (`_decomposition.md:964-977`). A licence problem found after the cut is a legal
  problem, not a follow-up commit.

The mutant is not caught by anything, because the check that would catch it is the check that
was skipped. The discriminator is procedural and belongs in the spec as an explicit
pre-publish step: one full `cargo xtask ci`, recorded, immediately before the cut.

**A second mutant: publishing the changelog's `unstable-projection` claim as it stands.**

`CHANGELOG.md:19-22` asserts that *"`ProjectionStore` ships behind an off-by-default
`unstable-projection` feature"*. `crates/happenstance-core/Cargo.toml`'s `[features]` block
does not contain it. Nothing checks a changelog against a manifest — not clippy, not
`spec-trace`, not the packaging step, not `cargo deny` — so this ships silently in a full,
green gate. And it converts an internal inaccuracy into a public one: a reader follows the
changelog, runs `cargo add happenstance-core --features unstable-projection`, and gets an
error naming a feature the project's own release notes told them to use. Both admissible
fixes are cheap; neither happens unless someone looks, which is why it is written into this
story as a release blocker rather than left as a tension in a brief.

**A third mutant: publishing `happenstance-testkit` as a stable `0.2.0`.** Its manifest
already says `version = "0.2.0"`, so this is the mutant of *doing nothing* — `cargo publish
-p happenstance-testkit` with the number that is already there. It succeeds. CF-32 even
supplies a rationale-shaped justification: the testkit's version is independent. But CF-32
gives it an independent *number*, not an independent *maturity*, and a stable conformance
suite makes a semver promise about a rule set whose ports are still moving underneath it —
`happenstance-core` is pre-1.0 and this release is explicitly a pre-release. The design
settled it at `0.2.0-alpha.1` and the approver signed that specific item
(`_design.md:649`, `:1252`), so shipping the in-tree number is a decision taken by inertia
against a decision taken deliberately.

**A fourth mutant: a yank policy stated and never executed.** `## Stability` says *"each
alpha is yanked when the next lands"*, and when `0.2.0-alpha.2` ships nobody runs `cargo
yank --version 0.2.0-alpha.1`. Everything passes; two pre-releases resolve; a consumer's
`--pre` resolution picks one of them at a time nobody predicted, and the churn the policy
existed to bound is now unbounded *and* documented as bounded. DEP-003 states the policy as
an observable artefact with an execution rule (`_decomposition.md:931-944`), and the spec
must carry the execution rule forward as an obligation on the *next* alpha rather than as
prose in this one.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

The two judgement boxes. **Literal positions:** this story adds no conformance rule and no
test — its work is a gate run, three manifest/document edits, a human-invoked `cargo publish`
and a resolution check. Vacuously true, ticked on that basis. **Frozen clauses:** none is
amended; the release ships the specification as the preceding stories left it. The two
documents this story edits — `crates/happenstance/README.md` and `CHANGELOG.md` — carry no
clause IDs, and the changelog reconciliation corrects a *claim about a feature*, not a clause.

On the last box, which is not vacuous here: a rule did seem wrong and has been fixed with the
reason given. The architecture brief (*Composition roots* 7) and DEP-003 both mount
`## Stability` between `## Guarantees` and `## Design`; that position is below the first code
block, which AC-U18 forbids. `_design.md:128-140` resolves it in favour of AC-U18 and records
why — a mount point is a convenience, ordering is a composition decision — and this story
follows the design, not the briefs.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
