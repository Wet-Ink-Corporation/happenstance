---
item: HS-P0016
stage: storymap
created: 2026-08-12T03:30:18.603Z
updated: 2026-08-12T03:30:18.603Z
template_sig: 1c63534a
rendered_sig: 80c1024e
---

# Story Map — 0.2.0 — where private opinions become promises

Fourteen stories in four milestones, covering `project.md`'s AC-001…AC-016. The
briefs are the source of the slicing: the `ux` brief's U1…U8 intents and IQ-1…IQ-7
invariants give the surface milestone its stories, the `testing` brief's per-AC
instrument table gives the gate milestone its two new `xtask` modules, and the
`deployment` brief's crate-set and PS-3 sections give the decisions milestone its
content (`_decomposition.md` in this directory).

Two shapes govern the whole map. **Decisions come first** — DR-1 is flagged *decide
early* at `../_decomposition.md`:302-309 because a crate-set answer larger than three
retroactively expands adapter scope, and the MSRV atom and the DT-1/4/5/6 resolutions
are inputs to copy that a later story writes. **The ledger is repaired before the
audit reads it** — DR-4 is deliberately a separate, earlier obligation than DR-3, so
that a falsifier is never chosen to make an audit pass (`RUNBOOK.md`:625-628,
*"not a thing to do in passing"*).

Nothing here is substrate owned outside this initiative. The one piece of substrate
this project consumes and does not build — the `0.2.0-alpha.1` registry baseline that
`registry-surface-diff` diffs against — is owned by the sibling project
`typed-layer-and-alpha-release`, which is rank 2 and this project is rank 3
(`../_decomposition.md`:150, 158-166).

## Backbone

The activities across the top, in the order the work is lived rather than the order it
is read.

| | Activity | Who is on the other side | Milestone that delivers it |
| --- | --- | --- | --- |
| **A1** | **Decide what ships, and what is claimed about it** — the crate set, the projection port's ship shape, the MSRV promise, and which claim leads | the maintainer, deciding once; the evaluator, downstream of every one of these | `release-decisions` |
| **A2** | **Prove the promises before making them** — the specification's clause maturities audited at publish, and the public surface diffed against what is already on the registry | the maintainer at the release gate; the consumer whose build AC-11 says must not break by surprise | `publish-time-gate-instruments` |
| **A3** | **Meet the crate on first contact** — the crates.io front page, the docs.rs page and the Guarantees slot say what is true of the tree that shipped | Persona 4, the evaluator, one-shot and time-boxed (`../_discovery/distillation/personas-and-journeys.md`:249-313, 333-338) | `published-surface-copy` |
| **A4** | **Take the one irreversible act, and check it from outside** — read the rendered pages, run the whole gate, publish, then install as a stranger | the stranger who did not build it (DoD 2 of this project's Definition of done) | `the-release-event` |

The `ux` brief's user intents map onto the backbone directly: U1, U4, U8 → A3
(`landing-copy-and-status-truth`); U2, U3 → A3 (`compliance-claim-and-gaps-promise`);
U5, U6 → A3 (`guarantees-and-docs-rs-presentation`); U7 → A4
(`stranger-install-smoke`). A1 and A2 have no user intent of their own, which is
exactly why every story in them is either a foundation or a gate instrument consumed
by A3 and A4.

## Slices

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `release-decisions` | `crate-set-decision` | foundation | Record, as an ingested decision atom, that `0.2.0` ships exactly `happenstance-core`, `happenstance`, `happenstance-testkit`, naming the four-crate alternative at `RUNBOOK.md`:4450-4451 as rejected and its cost, and assert `xtask/src/package.rs`'s derived set and its `PUBLISHABLE` intention list agree with no reconciliation failure | — | AC-001, AC-013 |
| `release-decisions` | `projection-port-ship-shape` | capability | Settle PS-3 — the projection port ships frozen or behind `unstable-projection` — on `projection-store-freeze`'s and `ladybug-projection-store`'s reports rather than re-derivation, and if it is gated, land the flag with RS-51-5 `doc(cfg)` treatment so the port is visible *with* its gate on docs.rs rather than absent | `crate-set-decision` | AC-012, AC-013 |
| `release-decisions` | `msrv-promise-atom` | foundation | Author, through `.kb/_intake/` and `/redkiln:kb-ingest`, a new decision atom at 0030-or-above stating the 1.97.1 floor as a promise, why it is where it is, and what an MSRV bump costs a consumer under 0.x — leaving `.kb/decisions/0004-edition-and-msrv.md` and `0029-msrv-raised-to-1-97-1.md` byte-identical and `redkiln validate --kb` clean | — | AC-006, AC-013 |
| `release-decisions` | `first-contact-design-resolutions` | foundation | Resolve DT-1, DT-4, DT-5 and DT-6 in this project's `_design.md`, each naming the option that lost, plus the evaluator-vs-application-author persona question the resolutions turn on (`../_decomposition.md`:310-314), and carry the settled answers into an ingested decision atom | — | AC-011, AC-013 |
| `publish-time-gate-instruments` | `falsifier-ledger-repair` | foundation | Repair the five known-short rows in `RUNBOOK.md`'s provisional ledger — add ES-41, ES-42, CF-39, CF-40 each with a deliberately chosen falsifier and an owning phase, remove ES-10's stale row — as its own change, landed before anything reads the table | — | AC-004 |
| `publish-time-gate-instruments` | `clause-maturity-audit` | capability | Add a mandatory `cargo xtask` clause-audit step that composes with `xtask/src/spec_trace.rs`'s existing parser: it reports every clause's maturity at the commit, reconciles its totals against §1.3's hand-computed 200/198/139/49/10/2, asserts the repaired ledger's clause-ID set equals the parsed `[PROVISIONAL]` list, fingerprints every `[FROZEN]` clause against the tree the project received, and carries `#[cfg(test)] mod tests` in the `xtask/src/package.rs`:408-457 shape whose fixtures include a seeded §1.3 disagreement and a seeded frozen-clause edit that it must fail on | `falsifier-ledger-repair` | AC-003, AC-004, AC-015 |
| `publish-time-gate-instruments` | `deferred-clause-reread` | capability | Re-read all ten `[DEFERRED]` clauses for whether the deferral is still honest at publish, give each a dated, consumer-readable reason, and extend the audit in the same change so an inherited (undated, unchanged) reason fails the gate — the *"a skip is reported, never silent"* discipline of `.kb/decisions/0010-the-suite-must-prove-itself.md` applied to prose | `clause-maturity-audit` | AC-005 |
| `publish-time-gate-instruments` | `registry-surface-diff` | capability | Add a mandatory `cargo xtask` step that diffs the public surface of the decided crate set against the `0.2.0-alpha.1` registry baseline (distinct from `CONTRIBUTING.md`:291-296's branch-point `cargo-semver-checks` job, which proves nothing about the last release), commit its dated report, derive the published version number from what it found, fail closed with a stated reason when the baseline is unreachable, and prove it rejects a seeded breaking change | `crate-set-decision` | AC-002 |
| `published-surface-copy` | `landing-copy-and-status-truth` | capability | Carry DT-1's lead claim onto the packaged `happenstance` README's first screen and DT-5's maturity treatment and DT-6's peer positioning into the slots that already exist (`README.md`:218-225 Prior art, `README.md`:79-98 status table), correct the four stale strings (`README.md`:11-16, :104-109, :85-90, `crates/happenstance/README.md`:6-11), spell *passes the conformance suite* differently from *compiles*, never suppress the 49 provisional clauses while listing the frozen ones, and check every link and held anchor mechanically rather than by reading | `first-contact-design-resolutions`, `crate-set-decision`, `projection-port-ship-shape` | AC-007, AC-011 |
| `published-surface-copy` | `compliance-claim-and-gaps-promise` | capability | Put the "DCB-compliant" claim on the packaged READMEs in `README.md`:227-234's claim-with-evidence form — the implementations the suite was actually run against, the date it was run, and a reachable link — and state, at DT-4's chosen site and within one hop of the landing entry, what the library promises about sequence positions and gaps in plain language, without implying ownership of `read_from_a_gap_position` that `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md` records as owned by nobody | `first-contact-design-resolutions` | AC-009, AC-010 |
| `published-surface-copy` | `guarantees-and-docs-rs-presentation` | capability | State the cost of depending in `crates/happenstance/README.md`:41-49's Guarantees slot — the MSRV as a promise linking the new atom, minor-bump-is-breaking under 0.x, and which feature set a `wasm32` reader takes — add the missing `[package.metadata.docs.rs]` block to `crates/happenstance/Cargo.toml` so all three crates build docs under all features with `doc(cfg)` on every gated item, and make the quick-start snippet the crate's own doctest via `crates/happenstance/src/lib.rs`:10 so it is the same text the stranger-install smoke runs | `msrv-promise-atom`, `projection-port-ship-shape` | AC-006, AC-007, AC-008, AC-014 |
| `the-release-event` | `rendered-page-preflight` | capability | Read the *rendered* crates.io and docs.rs pages for all three crates against the accessibility floor — one H1, no skipped levels, header rows, languaged fences, meaningful link text, alt text, no raw HTML/CSS/JS, no meaning in colour or glyph alone — plus licence, description and README as they read rather than as `cargo package --list` says they are contained, and commit the result as a dated check before the irreversible act | `landing-copy-and-status-truth`, `compliance-claim-and-gaps-promise`, `guarantees-and-docs-rs-presentation` | AC-007 |
| `the-release-event` | `publish-0-2-0` | capability | Publish the decided three crates at the version `registry-surface-diff` implies, having run the *whole* `cargo xtask ci` — not `--fast` — on the literal publish commit and committed its dated output, with the four mandatory `wasm32` steps asserted against the published tree and published feature set so the `!Send` flavour is proven rather than assumed | `rendered-page-preflight`, `registry-surface-diff`, `clause-maturity-audit`, `deferred-clause-reread`, `crate-set-decision` | AC-001, AC-014, AC-016 |
| `the-release-event` | `stranger-install-smoke` | capability | The proof artefact: from a scratch project outside this workspace, `cargo add` the published crate from the registry — no path dependency, no workspace feature unification — run the README's own quick-start write-then-read cycle against the resolved version, and record the run | `publish-0-2-0` | AC-008 |

### Why the milestones are cut where they are

- **`release-decisions` is one slice, not four PRs of unrelated paperwork.** Every
  story in it produces an ingested decision atom under AC-013's numbering constraint
  (0030 and up — `.kb/decisions/` holds 0001–0016 and 0029, and 0017–0028 are
  allocated to siblings by `../_decomposition.md`:119-121), and three of the four are
  read by copy that a later milestone writes. Splitting them would put four
  `/redkiln:kb-ingest` passes where one belongs.
- **`publish-time-gate-instruments` is one slice because both new steps mount in the
  same place.** The deployment brief's AC-DEP-005 requires each to join
  `xtask/src/main.rs`'s **Mandatory** list — never behind a tool probe, since neither
  has an external tool to probe for — and to update the module doc at
  `xtask/src/main.rs`:8-24 in the same change, because that doc comment is the one
  place in the repository that states in prose what the gate proves. Two stories
  editing that list and that paragraph in separate contexts is a merge conflict by
  construction.
- **`published-surface-copy` is one slice because it is one page.** A reader does not
  experience the compliance claim, the gaps promise and the Guarantees slot as three
  deliveries; IQ-1 budgets **0 hops to read a claim, ≤ 1 hop to its evidence**, and
  that budget can only be held by whoever sees the whole surface at once.
- **`the-release-event` is one slice because reversibility has to be bought before
  the act.** `cargo yank` removes a version from resolution and leaves every rendered
  page exactly as it was (`standards/rust/51-features-and-no-std.md`:226-231). The
  preflight read, the gate run and the publish are therefore one context; the smoke
  is in the same slice because it is the only check that can only run *after*, and it
  is this project's proof artefact.

### Archetype notes

Ten capabilities and four foundations. Each foundation is consumed and demonstrated
inside this project, so none is an unproven island:

| Foundation | Consumed and demonstrated by |
| --- | --- |
| `crate-set-decision` | `registry-surface-diff` (what it diffs), `landing-copy-and-status-truth` (the status table and the *"Which crate do I want?"* block), `publish-0-2-0` (what it publishes) |
| `msrv-promise-atom` | `guarantees-and-docs-rs-presentation` (the Guarantees slot links the atom and states what a bump costs) |
| `first-contact-design-resolutions` | all three `published-surface-copy` stories — DT-1 the lead claim, DT-4 the gaps site, DT-5 the maturity treatment, DT-6 the peer statement |
| `falsifier-ledger-repair` | `clause-maturity-audit`'s set-equality check between the ledger's clause IDs and the parsed `[PROVISIONAL]` list |

`projection-port-ship-shape` is typed **capability** rather than foundation because
its outcome is visible to a consumer either way: a frozen port, or an
`unstable-projection` feature that RS-51-5's `doc(cfg)` treatment renders *with its
gate* on docs.rs. IQ-2 forbids the third outcome — an unstable item that simply
vanishes from the page, which reads as "not supported".

## Coverage

Every project AC-001…AC-016 is covered by at least one story, and no two stories own
the same responsibility for the same AC. Where an AC appears twice, the two stories
own different halves of it and the split is named.

| Project AC | Stories | Split, where an AC is shared |
| --- | --- | --- |
| AC-001 — published, for a decided crate set | `crate-set-decision`, `publish-0-2-0` | *decide and reconcile* vs *publish the decided set* |
| AC-002 — the release is diffed, not asserted | `registry-surface-diff` | — |
| AC-003 — clause ledger audited and reconciles | `clause-maturity-audit` | — |
| AC-004 — the falsifier ledger is no longer short | `falsifier-ledger-repair`, `clause-maturity-audit` | *the five edits* vs *the set-equality check that keeps them true* |
| AC-005 — every deferral is still honest, in writing | `deferred-clause-reread` | — |
| AC-006 — MSRV is a promise with a justification | `msrv-promise-atom`, `guarantees-and-docs-rs-presentation` | *the atom of record* vs *the sentence a consumer actually reads* |
| AC-007 — the published crate looks finished | `landing-copy-and-status-truth`, `guarantees-and-docs-rs-presentation`, `rendered-page-preflight` | *the copy*, *the docs.rs configuration*, *the dated read of the rendered page* |
| AC-008 — a stranger can install it | `stranger-install-smoke`, `guarantees-and-docs-rs-presentation` | *the run* vs *the snippet being the same text the page shows* |
| AC-009 — the compliance claim can be checked | `compliance-claim-and-gaps-promise` | — |
| AC-010 — positions-and-gaps promise findable | `compliance-claim-and-gaps-promise` | — |
| AC-011 — first-contact decisions recorded | `first-contact-design-resolutions`, `landing-copy-and-status-truth` | *the resolution naming what lost* vs *the surface carrying it* |
| AC-012 — projection port's ship shape decided | `projection-port-ship-shape` | — |
| AC-013 — every settled answer is a decision atom | `crate-set-decision`, `projection-port-ship-shape`, `msrv-promise-atom`, `first-contact-design-resolutions` | one atom per answer settled; each names what lost |
| AC-014 — constrained-runtime flavour survives | `publish-0-2-0`, `guarantees-and-docs-rs-presentation` | *asserted by the four `wasm32` steps on the published tree* vs *stated so a `wasm32` reader can self-identify* |
| AC-015 — nothing frozen was amended | `clause-maturity-audit` | — |
| AC-016 — the full gate is green on the publish commit | `publish-0-2-0` | — |

**No AC is orphaned and no story is untraced**: all fourteen stories carry at least
one AC, satisfying Definition-of-done item 6. The briefs' own criteria fold in
without a gap — AC-UX-001…012 land in `published-surface-copy` and
`rendered-page-preflight`; AC-TEST-002's `#[cfg(test)] mod tests` obligation is
written into both `clause-maturity-audit` and `registry-surface-diff`; AC-TEST-004's
"a committed full-gate artefact distinct from any `--fast` run" is written into
`publish-0-2-0`; AC-DEP-001, AC-DEP-002 and AC-DEP-005 land in `crate-set-decision`,
`projection-port-ship-shape` and the two instrument stories respectively; AC-DEP-004's
N/A-with-a-reason posture is recorded by `publish-0-2-0`.

## Merge order

Foundations before the capability slices that consume them; the milestone order is a
topological order of the cross-milestone edges, which are acyclic.

1. **`release-decisions`**
   1. `crate-set-decision` *(foundation)*
   2. `projection-port-ship-shape` — after 1, because the ship shape is a property of a crate in the decided set
   3. `msrv-promise-atom` *(foundation)* — unordered with 1 and 2
   4. `first-contact-design-resolutions` *(foundation)* — unordered with 1–3
2. **`publish-time-gate-instruments`** — may start as soon as `crate-set-decision` has landed; it does not wait on 1.2–1.4
   1. `falsifier-ledger-repair` *(foundation)* — no blockers at all; can run alongside milestone 1
   2. `clause-maturity-audit` — after 2.1, so the audit never reads a short ledger
   3. `deferred-clause-reread` — after 2.2, so the ten reasons and the check that requires them land together and the gate is never red between them
   4. `registry-surface-diff` — after 1.1; unordered with 2.1–2.3
3. **`published-surface-copy`** — after milestone 1 in full
   1. `landing-copy-and-status-truth`
   2. `compliance-claim-and-gaps-promise` — unordered with 3.1
   3. `guarantees-and-docs-rs-presentation` — unordered with 3.1–3.2
4. **`the-release-event`** — after milestones 2 and 3 in full
   1. `rendered-page-preflight`
   2. `publish-0-2-0` — the irreversible act
   3. `stranger-install-smoke` — the proof artefact, and the only step that can only run after

Milestones 2 and 3 are mutually unordered once `crate-set-decision` and
`first-contact-design-resolutions` have landed, and may be interleaved. The serial
trunk is 1.1 → {2.x, 3.x} → 4.1 → 4.2 → 4.3.

**Two ordering constraints that must not be reordered by a later re-plan.**
`falsifier-ledger-repair` strictly precedes `clause-maturity-audit` (DR-4 before
DR-3), because the risk register's first row is precisely a falsifier chosen during
an audit to make the audit pass. And `rendered-page-preflight` strictly precedes
`publish-0-2-0`: IQ-4 puts the whole of this project's reversibility before the act,
and `xtask/src/package.rs`:4-18 is explicit that containment is not presentation.

**Where the release blocks.** If `clause-maturity-audit` finds a `[FROZEN]` clause
that is wrong, DR-15 and AC-015 make that a stop: it becomes a new decision atom and
a re-plan, not an edit inside this project. The same applies if
`registry-surface-diff` reports a break the crate set was not meant to make — the
version follows the report, and if the report implies something outside `0.2.0`, that
is a re-plan rather than a rounded-down version number.
