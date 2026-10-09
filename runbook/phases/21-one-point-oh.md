# Phase 21 — `1.0.0`

**Goal.** The crates phase 16 named, published at `1.0.0`, making exactly the
promises phase 16 wrote down. ADR-0066 names nine: `happenstance-core`,
`happenstance`, `happenstance-testkit`, `happenstance-sqlite`,
`happenstance-cloudflare`, `happenstance-postgres`, `happenstance-neon`,
`happenstance-sync` and `happenstance-sync-testkit`. `happenstance-ladybug` is
outside 1.0, and since phase 17 retired (ADR-0078), so it has no `0.x` line to keep.

**Why here.** Last, by definition. Its dependency row is 13, 14, 16, 17, 18 and
20: sync is inside 1.0 by D-1, so replication and retention are proved before the
promise is made.

**Decisions it settles.** None. A decision taken here is one phase 16 missed, and
the right response is to go back and record it there.

**Work**

- [ ] #192 · **The clause audit, mechanically.** Every clause on a promised surface is
      `[FROZEN]`, or carries the disposition phase 16 gave it. That audit is now a
      lint, not a reading: `runbook_clause_ledgers_match_the_specification` in
      `xtask/src/lints.rs`, held since phase 16 over *The 1.0 dispositions* in
      [`ledgers.md`](../ledgers.md), so the dispositions and the specification
      cannot drift apart without the gate saying so. Its grammar already refuses
      a `renew-past-1.0` cell with no falsifier — a renewal without one is a
      deferral by another name, the shape CF-38 refuses in a marker. What is left
      for this phase is to watch it pass with no `freeze-by-*` row remaining:
      every one gone to `[FROZEN]`, or re-dispositioned by a record. Confirm, by
      reading the rows rather than the lint, that each surviving
      `renew-past-1.0` falsifier names an instrument and not just a worry.
- [ ] #195 · **Versioning after `1.0.0`** (ADR-0066). The nine crates ship `1.0.0` in
      lockstep and version independently from then on. Choose and configure the
      release tooling that makes that cheap — `release-plz` or `cargo-release` —
      and record the choice here. Each adapter declares the core it needs as
      `happenstance-core = "1.N"`, held honest by phase 17's minimal-versions job,
      and each adapter README states its conformance as *"passes
      `happenstance-testkit` X.Y"*.
- [ ] #198 · **The semver baseline.** `cargo-semver-checks` against `0.4.x` reports no
      break that is not traced to a decision — phase 17's, ADR-0070's `Chunk`, or
      phase 18's removal of `happenstance`'s `unstable-projection` (recorded in
      its session log).
- [ ] #199 · **The promise meets `PUBLISHABLE`** (ADR-0066 §1). Promised and publishable
      are different facts, reconciled here: `xtask/src/package.rs`'s
      `PUBLISHABLE` and the semver job's package list both name
      `happenstance-sync` and `happenstance-sync-testkit`, and `reconcile` passes
      with nine.
- [ ] #201 · **The MSRV statement** in each crate's documentation matches ADR-0067: the
      floor is 1.97.1; after 1.0 it rises only in a minor, only to a stable
      release at least six months old when that minor ships, and always with a
      `CHANGELOG.md` entry. The statement says what the promise rests on —
      resolver 3's MSRV-aware fallback — in one sentence a consumer can check.
- [ ] #203 · **A reader from outside** runs the documentation's opening path against the
      release candidate, and what they stumble on is dispositioned — the method
      `HS-P0024` establishes.
- [ ] #205 · **`1.0.0-rc.1`, then a soak**, then `1.0.0`. ADR-0066 sets the soak: no
      time floor, and it ends when all of these hold on the release candidate —
      all nine crates render on docs.rs from the registry;
      `examples/outside-projection-adapter` and one other example build against
      the registry rc rather than the path; the registry semver baseline is
      clean; the outside-reader pass above has run against the rc; and no defect
      the rc surfaced needs an API change. Any API change is `rc.N+1`, and the
      conditions are checked again.
- [ ] #207 · `SECURITY.md` rewritten for a stable line (ADR-0066): its scope lists the
      nine crates, not the seven it names today; its supported-versions table
      reads *the latest `1.x` minor, plus security fixes on the previous major for
      six months after the next major ships*; it states ADR-0066's licence
      promise, that the nine crates stay `MIT OR Apache-2.0` for all of 1.x; it
      carries ADR-0066 §5's semver exemption list, mirrored into `CHANGELOG.md`'s
      account of what semver covers; and its *"Pre-1.0"* section goes.
- [ ] #209 · **Confirm the dependency row still names 13 and 14.** D-1 put sync inside
      1.0, and the [status table](../README.md#status)'s row for this phase is
      what enforces it. Phase 16 confirmed it; re-read it here, because a row
      edited in passing is the failure this runbook was split to prevent.

**Proof artefact.** The crates live at `1.0.0` and rendering on docs.rs, and the
per-clause disposition table from phase 16 with every row checked.

**Exit criteria**

- [ ] Every crate phase 16 named — nine, by ADR-0066 — is published at `1.0.0`.
- [ ] The clause audit is clean, and no `renew-past-1.0` row lacks its falsifier.
- [ ] The soak's conditions all held on the last release candidate.
- [ ] The semver baseline is clean.
- [ ] The status table has a `1.0.0` milestone row, and the lint holds it.
- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Estimate.** 3–5 days, plus the soak.

**Session log**
