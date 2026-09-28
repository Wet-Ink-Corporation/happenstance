# Phase 21 — `1.0.0`

**Goal.** The crates phase 16 named, published at `1.0.0`, making exactly the
promises phase 16 wrote down.

**Why here.** Last, by definition. Its dependency row is 13, 14, 16, 17, 18 and
20: sync is inside 1.0 by D-1, so replication and retention are proved before the
promise is made.

**Decisions it settles.** None. A decision taken here is one phase 16 missed, and
the right response is to go back and record it there.

**Work**

- [ ] **The clause audit, mechanically.** Every clause on a promised surface is
      `[FROZEN]`, or carries the disposition phase 16 gave it. Checked against
      `cargo xtask spec-trace` and the [ledgers](../ledgers.md), not against prose.
- [ ] **The semver baseline.** `cargo-semver-checks` against `0.4.x` reports no
      break phase 17 did not trace.
- [ ] **The MSRV statement** in each crate's documentation matches the policy
      phase 16 recorded.
- [ ] **A reader from outside** runs the documentation's opening path against the
      release candidate, and what they stumble on is dispositioned — the method
      `HS-P0024` establishes.
- [ ] **`1.0.0-rc.1`, then a soak**, then `1.0.0`. The soak's length is phase
      16's to set.
- [ ] `SECURITY.md`'s supported-versions table and its *"Pre-1.0"* section rewritten
      for a stable line.

**Proof artefact.** The crates live at `1.0.0` and rendering on docs.rs, and the
per-clause disposition table from phase 16 with every row checked.

**Exit criteria**

- [ ] Every crate phase 16 named is published at `1.0.0`.
- [ ] The clause audit is clean.
- [ ] The semver baseline is clean.
- [ ] The status table has a `1.0.0` milestone row, and the lint holds it.

**Estimate.** 3–5 days, plus the soak.

**Session log**
