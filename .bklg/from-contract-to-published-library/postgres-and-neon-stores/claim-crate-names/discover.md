---
item: HS-S0060
stage: discover
created: 2026-08-12T13:02:28.569Z
updated: 2026-08-12T13:02:28.569Z
template_sig: 86ce4036
rendered_sig: 230203b5
---

# Discover — Claim both crate names on crates.io

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: run `cargo xtask reserve` so both names are held on crates.io **before a line of this project's work lands** | `_storymap.md`, *Slices* table, `crate-name-claims` row | The deliverable is an executed command and two held names, not code. Ordering is the whole content of the story |
| **AC-012** — "both names are claimed on crates.io" | `project.md`, *Acceptance criteria*, AC-012 | This story owns only that clause of AC-012. The `todo!()` removal, the scoped `#![allow(clippy::todo)]`, `publish = false` and `PUBLISHABLE` are `deskeleton-and-package-readiness`'s half (`_storymap.md`, *Coverage*, AC-012 row) |
| Both names, their descriptions, their blurbs and their phase attribution are **already** in the reserver's table | `xtask/src/reserve.rs:99`, `xtask/src/reserve.rs:105` | Nothing is invented here. The story runs a command that already knows the answer; writing a new one would be the drift the command exists to prevent |
| The placeholder is a standalone `0.0.0` crate, never the real crate | `xtask/src/reserve.rs:1-30` | Publishing the real crates at the workspace version makes the API semver-binding, which "the whole plan in `RUNBOOK.md` is built around freezing *deliberately*". Reserving a name is not a reason to freeze an API |
| crates.io's parked-name policy, and why one-per-phase answers it | `xtask/src/reserve.rs:56-66` | "a placeholder claimed at the start of the phase that builds it always has an answer to that". Claiming all ten at once would not, and prefix reservation does not exist, so waiting costs nothing |
| Phase 0's rule, restated as phase 10's first work item | `RUNBOOK.md:4346` | "a name is reserved when its phase starts, not before — the point being that by now there is a crate to justify it with" |
| The deployment brief's AC-012 row | `_decomposition.md`, *Deployment brief* → *Acceptance Criteria*, AC-012 | `cargo xtask reserve` "must be **run**, not merely available, at the start of this project's work" |
| Outbound edge → `deskeleton-and-package-readiness` (HS-S0072) | `_storymap.md`, *Slices* table, `deskeleton-and-package-readiness` row; and *What each story is*, `claim-crate-names` | That story "cannot honestly remove `publish = false` from a name nobody holds". This story supplies exactly that precondition and nothing else |
| No inbound `depends_on` | manifest (`dependsOn: []`); `_storymap.md`, *Merge order* item 1 | Nothing gates it. It is first by rule, not by dependency, and it can be done on day one alongside `neon-sql-transport` |
| The licence trap the command was written to close | `xtask/src/reserve.rs:3-10` | "the licence files get forgotten once, and the crate that carries the project's first impression ships without them, **permanently**, because a version can be yanked and never removed" |

## Questions

**Answered.**

1. *Does reserving bind the API?* No — and this is the reason the command exists in
   the shape it does. The placeholder "shares nothing with the workspace but its
   metadata" (`xtask/src/reserve.rs:26-30`), so `0.0.0` on crates.io says nothing
   about `happenstance-postgres`'s eventual surface.
2. *Do the licence texts and README go on the placeholder?*
   Yes; `xtask/src/reserve.rs:3-10` names forgetting them as the failure the
   command was written against. `spec` restates the three files by the same names
   `xtask/src/package.rs:94` uses, so the two lists cannot drift apart silently.
3. *Does this change `PUBLISHABLE`?* No. `xtask/src/package.rs:86` still names three
   crates and `reconcile` (`xtask/src/package.rs:172-212`) compares it against what
   `cargo metadata` says the **workspace** will publish. The placeholder is not a
   workspace member, so the default gate does not notice this story at all — which
   is the point flagged under *The wrong implementation*.

**Deferred to the owning stories, deliberately not touched here.**

4. *How the adapter buys ES-10's visibility invariant when `nextval()` allocates
   outside the transaction.* ADR-0024's, and it is owned by
   `adr-0024-position-visibility-mechanism` (HS-S0065). No signal in this story
   bears on it, and a name claim must not become the place a mechanism gets
   assumed.
5. *Whether `conflicting_position` is a promise every adapter owes or a hint one
   may omit.* `neon-conflicting-position-verdict` (HS-S0070) owns it. Noted here
   only so the deferral is on the record: ES-25 already answers it as a hint
   (`spec/SPECIFICATION.md:3746-3748`), and what is open is the *ledger's* standing
   assumption about Neon, not the clause.
6. *Whether the publish step is run by a human or by a credentialed CI job.* Left to
   `spec`. It touches the same "a credential the default `gate` job does not have"
   question the deployment brief raises for the Neon job, and answering it here
   would pre-empt `neon-fixture-and-live-job`.

## Decision

The problem this slice solves is a race with strangers: two crate names this
project is about to make real are unheld on a first-come registry, and the moment
`deskeleton-and-package-readiness` deletes `publish = false` the workspace starts
asserting a claim it may not own. Phase 0's rule — reserve when the phase starts —
exists because a reservation made earlier is a parked name and one made later is a
gamble, and `cargo xtask reserve` exists because a procedure run once every few
weeks from memory is a procedure that drifts. The spec will cover: running the
command for both entries already present at `xtask/src/reserve.rs:99` and
`xtask/src/reserve.rs:105`; the `0.0.0` standalone-placeholder shape and why it is
not the real crate; the three required files travelling with each placeholder,
named identically to `xtask/src/package.rs:94`'s `REQUIRED_FILES`; evidence that
both names resolve on crates.io afterwards; and an explicit statement that
`PUBLISHABLE`, the manifests and the skeleton markers are **not** touched here. No
`[FROZEN]` clause is read or changed, so no ADR is owed.

## The wrong implementation

**A reservation performed by publishing the real crates instead of the
placeholders.** Drop `publish = false` from `crates/happenstance-postgres/Cargo.toml`
and `crates/happenstance-neon/Cargo.toml`, add both names to `PUBLISHABLE`, copy the
three files in, and `cargo publish` at the workspace version. It satisfies every
existing check: `cargo xtask package-check`'s `reconcile` (`xtask/src/package.rs:172-212`)
is *happier* than before because both directions agree, `cargo xtask ci` is green,
and both names are indisputably held. It is also irreversible in the one direction
that matters — `0.2.0` of a crate whose every body is `todo!()` is now a published
API surface, and crates.io yanks but never removes, so the deliberate phase-4-to-6
freeze is pre-empted by a name claim (`xtask/src/reserve.rs:20-25`).

The nastier sibling is quieter: **a hand-rolled placeholder** — `cargo new`, a
description typed from memory, published without `LICENSE-APACHE`. Nothing in this
repository can catch it. `xtask/src/package.rs`'s `REQUIRED_FILES` check walks the
**workspace's** publishable crates, and a standalone placeholder is not a workspace
member, so the one instrument that knows the rule never looks at the artifact the
rule is about. That gap is why the deliverable is *the command was run*, not *the
name is held*, and `spec` must state the evidence in those terms.

Neither mutant is a store, so neither belongs in `crates/happenstance-testkit/tests/`;
no conformance rule is added, changed, or driven by this story.

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
