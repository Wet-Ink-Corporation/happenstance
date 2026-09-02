---
item: HS-S0089
stage: discover
created: 2026-08-12T13:02:58.330Z
updated: 2026-08-12T13:02:58.330Z
template_sig: 86ce4036
rendered_sig: 882bc8f6
---

# Discover — A publish-time clause audit that reconciles and cannot be talked out of a frozen clause

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:61 | A mandatory `cargo xtask` step composing with `spec_trace.rs`'s existing parser: reports every clause's maturity, reconciles totals against §1.3's 200/198/139/49/10/2, asserts the repaired ledger's clause-ID set equals the parsed `[PROVISIONAL]` list, fingerprints every `[FROZEN]` clause against the tree the project received, and carries `#[cfg(test)] mod tests` with a seeded §1.3 disagreement and a seeded frozen-clause edit it must fail on. |
| AC-003 | `project.md`:233-237 | Audit reports every clause's maturity; no `[PROVISIONAL]` clause has an empty falsifier; totals equal §1.3's stated figures; "a seeded disagreement between the two fails the check." |
| AC-004 (this story's half) | `project.md`:238-242 | "The ledger's clause set equals `spec-trace`'s list of `[PROVISIONAL]` IDs, checked rather than counted by hand" — the equality check itself, as distinct from `falsifier-ledger-repair`'s content edits. |
| AC-015 | `project.md`:281-283 | "No `[FROZEN]` clause differs between the tree this project received and the tree it published. If the audit found one wrong, the release is blocked and a decision atom records it." |
| `dependsOn: falsifier-ledger-repair` | manifest, `_storymap.md`:163, 180-182 | Must land after the ledger repair, "so the audit never reads a short ledger" — a strict, explicitly-protected ordering. |
| `xtask/src/spec_trace.rs`'s existing contract | `xtask/src/spec_trace.rs`:1-57 | Already parses every clause's maturity marker, writes §7.1/§7.2, and **checks but never writes** §1.3 — "moving it inside the generated markers would destroy the very property it is being used to prove." This story composes with that split rather than duplicating the parser (`_grounding.md`:166-170, "code patterns to follow"). |
| CF-38, already live | `spec/SPECIFICATION.md`:213-217 | A `[PROVISIONAL]`/`[DEFERRED]` marker with an empty falisfier is already a build failure — but this check is on each clause's *own* inline marker text in `SPECIFICATION.md`, not on `RUNBOOK.md`'s separate ledger table. The two are different instruments checking different documents; this story's set-equality obligation (AC-004) is the one CF-38 does not cover. |
| Testing brief's named wrong implementations | `_decomposition.md`, Testing brief AC table, AC-003/AC-004/AC-015 rows | AC-003: "a seeded disagreement between the parsed count and §1.3's stated figure that the check does not fail." AC-004: "a ledger row added or removed without updating the check." AC-015: "a frozen clause edited to match a wrong implementation rather than blocking release." |
| `#[cfg(test)]` precedent | `xtask/src/package.rs`:408-457, `xtask/src/spec_trace.rs`:1809-1825 | Both existing gate instruments carry a fixture string, a named wrong shape, and a failing assertion — the shape this story's two new modules must match (`_decomposition.md`, Testing brief Intent, lines 449-458). |
| CI implication, already decided | `_decomposition.md`, Deployment brief "CI implication" (lines 682-720) | This instrument joins `xtask/src/main.rs`'s **Mandatory** list, never behind a tool probe (there is no external tool to probe for), and the module's own doc comment (`xtask/src/main.rs`:8-24) is updated in the same change. |

## Questions

- **How does the audit "fingerprint" a `[FROZEN]` clause against the tree the project received?** Not fully specified by any brief — `_storymap.md`:61 uses the word "fingerprints" without defining the mechanism (a hash of clause text? a diff against a pinned commit?). Deferred to spec, which must pick a concrete comparison (most likely: a stored digest of each `[FROZEN]` clause's text at the commit this project started from, diffed against the current text at audit time) and justify it against AC-015's actual requirement — "differs between the tree this project received and the tree it published." 
- **What happens procedurally if the audit finds a wrong `[FROZEN]` clause?** DR-15 and AC-015 already answer *that it blocks*, but the task brief's own framing asks this story to say so explicitly: **this is not a silent-pass condition.** If the clause-maturity audit finds a `[FROZEN]` clause that should not be frozen as written, the correct response is a new decision atom and a re-plan of this project's release date — the audit's job is to make that discoverable, not to fix it. This is not deferred; it is stated in Decision below because DR-15 requires this project to say so rather than assume it.
- **Does composing with `spec_trace.rs` mean adding a new module, or extending `spec_trace.rs` itself?** Deferred to spec. The grounding's "code patterns to follow" (`_grounding.md`:166-170) says compose rather than duplicate the parser, but doesn't specify whether that means a new `xtask` module calling into `spec_trace`'s public parsing functions, or new functions added directly to `spec_trace.rs`. Either satisfies "compose, don't duplicate"; spec picks based on `spec_trace.rs`'s existing module boundary.

## Decision

A new, mandatory `cargo xtask` step audits every clause's maturity at the publish commit by composing with — not duplicating — `spec_trace.rs`'s existing parser. It reconciles its totals against §1.3's hand-computed 200/198/139/49/10/2 census (failing on a seeded disagreement), asserts `falsifier-ledger-repair`'s repaired ledger's clause-ID set equals `spec-trace`'s own parsed `[PROVISIONAL]` list (failing on a seeded ledger drift), and fingerprints every `[FROZEN]` clause's text against the tree this project received (failing if any differs). **If the fingerprint check finds a `[FROZEN]` clause whose text is wrong** — not merely changed, but wrong as written — that is not this story's to fix: it blocks the release, and the correction is a new decision atom and a re-plan, per DR-15 and AC-015. Spec will decide the fingerprint mechanism and whether the new logic lives in `spec_trace.rs` or a new module, and will add the step to `xtask/src/main.rs`'s Mandatory list with the module doc updated in the same change, per the deployment brief's CI-implication section.

## The wrong implementation

An audit that faithfully reconciles the 200/198/139/49/10/2 totals against §1.3, and even re-checks CF-38's non-empty-falsifier requirement on each clause's own inline marker in `SPECIFICATION.md` — both of which already exist or are straightforward to compose — but never performs AC-004's distinct obligation: the set-equality check between `RUNBOOK.md`'s repaired ledger and `spec-trace`'s parsed `[PROVISIONAL]` list. Because CF-38 checks each clause's *own* falsifier text inside `SPECIFICATION.md`, a clause can carry a perfectly valid inline falsifier and still be entirely missing from the human-facing `RUNBOOK.md` ledger (exactly the historical defect this project starts by repairing) — CF-38 would never notice, because it doesn't read `RUNBOOK.md` at all. An audit that stops at reconciling totals and re-running CF-38 would report green even if `falsifier-ledger-repair`'s next edit silently dropped a row again, because a *count* of 49 provisional clauses can be correct while the *set* of clause IDs behind that count has drifted from the ledger. AC-004's own text names exactly this: "checked rather than counted by hand." What catches it: the set-equality assertion itself, seeded in `#[cfg(test)] mod tests` with a fixture where the ledger's clause-ID set and `spec-trace`'s parsed list disagree by exactly one ID, following the `package.rs:408-457` / `spec_trace.rs:1809-1825` shape.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
