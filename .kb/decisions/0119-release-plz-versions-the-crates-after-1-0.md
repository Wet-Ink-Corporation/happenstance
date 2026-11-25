---
id: kb-decision-0119
title: After 1.0.0 the nine crates version independently, prepared by release-plz and published by the tag
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0119
reversibility: medium
phase: 21
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-25 by the v1 AFK run for the owner (#195). ADR-0066 ships the nine crates at
  1.0.0 in lockstep and versions them independently afterwards, and leaves the tooling to phase 21.
  Recommended: release-plz in its release-PR mode only, which bumps each changed crate by
  cargo-semver-checks and conventional commits and writes the CHANGELOG, while publishing stays
  where it is: a v* tag on the release PR's merge commit, which release.yml publishes by trusted
  publishing. cargo-release is the alternative, and loses on per-crate bump inference. Adapters
  declare happenstance-core = "1.N"; each adapter README states "passes happenstance-testkit X.Y".
  The flip is the owner wanting to keep lockstep after 1.0.
depends_on:
  - kb-decision-0066
related:
  - kb-decision-0067
  - kb-decision-0057
source_paths:
  - .github/workflows/release.yml
  - Cargo.toml
  - runbook/phases/21-one-point-oh.md
last_reviewed: 2026-11-25
---

# After `1.0.0` the nine crates version independently, prepared by `release-plz` and published by the tag

**Proposed 2026-11-25** by the unattended v1 run, for the owner (#195). Nothing below has been
applied, and no tool has been installed or configured.

## Context

ADR-0066 §4: the nine crates ship `1.0.0` in lockstep, then version independently; each adapter
declares `happenstance-core = "1.N"`, held honest by phase 17's minimal-versions job; each adapter
README states its conformance as *"passes `happenstance-testkit` X.Y"*; release tooling is chosen
at phase 21. Today one workspace version (`Cargo.toml`'s `[workspace.package] version`) moves every
crate, the release PR is written by hand, and `release.yml` publishes all nine on a `v*` tag with
crates.io trusted publishing and a check that each crate's version matches the tag.

Independent versions break two things that are hand-held today: the workspace-wide version, and the
single tag that names one version for all nine.

## Options

- **A. `release-plz`, release-PR mode only (recommended).** On `main` it opens a release PR that
  bumps each crate that changed, inferring minor or patch from `cargo-semver-checks` (already
  configured here) and conventional commits, and writes each crate's CHANGELOG section. The owner
  reviews and merges it. Publishing stays in `release.yml`, triggered by tags; with independent
  versions the tags become per crate (`happenstance-core-v1.2.0`), and `release.yml`'s version check
  reads the crate from the tag. `release-plz`'s own publish step is not used, so trusted publishing
  and the `crates-io` environment are unchanged.
- B. `cargo-release`. Mature and scriptable, but the bump level is the human's per crate, and the
  CHANGELOG is ours to write; it adds tagging and publishing we already have.
- C. No tool: keep lockstep after 1.0. Simple, and contradicts ADR-0066 §4, which needs its own
  record.

## What the manifests and READMEs would look like

- Each crate's `[package]` carries its own `version`; `[workspace.package] version` goes.
- An adapter's dependency: `happenstance-core = { version = "1.2", path = "../happenstance-core" }`,
  the lowest minor it works with, and the minimal-versions job resolves exactly that bound.
- Each adapter README, under its conformance heading: *"Passes `happenstance-testkit` 1.3 (event
  store and projection families), `happenstance-sync-testkit` 1.1 (ingest and write-group
  families)."*, the testkit versions it is pinned to exactly in its `dev-dependencies` (CF-32).
- `release.yml`'s matrix check compares the tag's crate and version, not one version for nine.

## Decision (proposed)

A, to be configured in phase 21 before `1.0.0-rc.1`: a `release-plz.toml` with `publish = false`
and `git_tag_enable = false` (the tag stays the owner's act), the per-crate tag scheme in
`release.yml`, and a dry run recorded in the phase file.

## What would flip it

The owner deciding to keep lockstep after 1.0 (a record superseding ADR-0066 §4), or `release-plz`
being unable to leave publishing and tagging to `release.yml`.

## Door

One-way: it shapes every publish after `1.0.0`.
