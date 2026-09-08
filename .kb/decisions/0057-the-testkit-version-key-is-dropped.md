---
id: kb-decision-0057
title: The testkit's root version key is dropped, and a gate step keeps it dropped
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0057
reversibility: low
phase: 12
supersedes: null
superseded_by: null
summary: >-
  The root workspace.dependencies entry for happenstance-testkit loses its version key, because every consumer of the testkit in this workspace is a dev-dependency and the key is load-bearing for nothing. A reconcile step in xtask/src/package.rs then asserts that no publishable crate's testkit dev-dependency carries a resolvable version, so the defect happenstance-cloudflare still carries cannot recur silently. Neither half is urgent and neither is written yet. examples/outside-projection-adapter's version-plus-path spelling is the deliberate exception, and nobody has written that down.
depends_on: []
related:
  - kb-decision-0002
  - kb-playbook-ratchet-gate-landing-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/testkit-dev-dependency-version-requirement.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# The testkit's root version key is dropped, and a gate step keeps it dropped

## Decision

`Cargo.toml`'s `[workspace.dependencies]` entry for `happenstance-testkit` drops its
`version = "0.2.0-alpha.1"` key. Every reference to the testkit anywhere in this workspace
is a dev-dependency — `happenstance-cloudflare`, `happenstance-postgres` and
`happenstance-sqlite` via `.workspace = true`, `happenstance` and now `happenstance-sqlite`
by path only — so the root key protects nothing that would otherwise resolve
unversioned. A `cargo package -p happenstance-sqlite --no-verify` run confirms the packaged
manifest already carries no `happenstance-testkit` dependency at all once a crate switches
to a path-only per-crate spelling, which is the pattern `crates/happenstance/Cargo.toml`
established first (documented there in an eleven-line comment, NF-006) and
`happenstance-sqlite` repeated for the same reason U-2 found it: `cargo package` refuses to
publish a crate whose dev-dependency on an unpublished sibling carries a version
requirement pointing nowhere resolvable.

The root manifest's own comment calls a stale `version` key on a *real* registry dependency
"the highest-cost line in the file to get wrong" — true of `happenstance-core` and
`happenstance`, both genuine non-dev dependencies elsewhere in the tree. It does not apply
to the testkit, because nothing in this workspace depends on it non-dev, and nothing is
planned to. Dropping the key removes the mechanism that keeps reintroducing the coupling by
inheritance, rather than fixing it one crate at a time as each adapter joins the release
set — which is what happened once already: `happenstance-cloudflare` is in `PUBLISHABLE`,
is unpublished, and inherits the same version requirement `happenstance-sqlite` just moved
away from, untouched, because it was another lane's crate.

A second, independent step follows in `xtask/src/package.rs`'s `reconcile`, which already
holds the `PUBLISHABLE` set honest in both directions (naming its five members and
asserting each carries licence files and a README). A new assertion — no publishable
crate's `happenstance-testkit` dev-dependency carries a resolvable version — generalises
the per-crate check `tests/front_page.rs` already runs for one crate into a gate step that
catches the *next* adapter without anyone having to remember. That is a `xtask/`-owned
surface with its own bar (`standards/rust/80-the-gate.md`, `81-checks-that-cannot-be-types.md`),
and it is a separate, later piece of work from dropping the root key.

`examples/outside-projection-adapter` keeps `version` **and** `path` on its testkit
dependency, deliberately: that crate exists specifically to stand where a stranger to this
workspace stands, so it declines every workspace convenience it can. That exception is
correct but was, until now, unrecorded — it is the one entry that would read as a
counter-example to anyone applying this decision mechanically, and this atom is where that
gets written down.

## Alternatives rejected

**Leaving the root key and fixing each adapter's spelling as it joins the release set**
(the precedent `crates/happenstance/Cargo.toml` set) was declined once the workspace held
two independent, contemporaneous instances of the same defect: `happenstance-sqlite`'s fix
and `happenstance-cloudflare`'s outstanding case. Repeating a documented pattern once is a
deliberate choice; a second instance of the same defect, caused by the same inert
inheritance, is what a pattern looks like rather than what a policy looks like. The
strongest argument for the alternative — that changing the root key now second-guesses a
prior, explicitly-reasoned decision on no new evidence but a second instance — is accepted
as fair, and is exactly the evidence this decision treats as sufficient: the earlier
decision never established that the root key was inert, and it is.

## What this does not settle

Neither the root-key removal nor the `xtask` gate step is written yet; both are unblocked
and neither is urgent, since `RUNBOOK.md`'s phase-12 publish order (`core → testkit →
happenstance → sqlite`) already masks the defect for `0.2.0` specifically. The next
testkit-only minor bump after `0.2.0` is when `happenstance-cloudflare`'s uncorrected
spelling would surface the problem for real, which is the deadline this decision is priced
against. Who lands the gate step, and whether `happenstance-cloudflare`'s Cargo.toml moves
in the same change or a later one, are both open.
