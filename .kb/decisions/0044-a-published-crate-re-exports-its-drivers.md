---
id: kb-decision-0044
title: A published crate re-exports any crate whose type appears in one of its public signatures
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0044
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  pub use futures_core in happenstance-core, rusqlite in happenstance-sqlite, worker in
  happenstance-cloudflare, and pub use happenstance_core where the contract is in a public
  signature; the four publish = false crates owe nothing until that key is removed. tokio is
  excluded from happenstance-sqlite's set by owner ruling even though two of its types appear
  in public error variants, because the crate takes tokio only at features = ["rt"], so the
  re-export would hand a reader a partial tokio and a second, farther-from-cause E0433 rather
  than the one it exists to prevent. RS-40-4 is amended in the same change to state the
  signature-arithmetic as a necessary condition on the re-export set, not a sufficient one.
depends_on: []
related:
  - kb-decision-0006
  - kb-decision-0003
  - kb-open-question-adapter-version-lockstep-001
  - kb-open-question-facade-does-not-match-adr-0006-001
  - kb-open-question-cloudflare-feature-gate-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - standards/rust/40-public-surface-and-evolution.md
last_reviewed: 2026-09-07
---

# A published crate re-exports any crate whose type appears in one of its public signatures

## Decision

A **published** crate re-exports any crate whose type appears in one of its own public
signatures — additive, `pub use`, with the doc sentence `happenstance-core` and
`happenstance-postgres` had already written independently of each other. Landed: `pub use
futures_core;` beside `pub use bytes;` in `happenstance-core` (`Stream` is in `read`'s return
type); `pub use rusqlite;` in `happenstance-sqlite`; `pub use worker;` in
`happenstance-cloudflare`; and `pub use happenstance_core;` — the crate itself, not the
existing item-glob — in every crate whose signatures put the contract in front of a caller.
The four crates still carrying `publish = false` (`happenstance-neon`, `happenstance-postgres`,
`happenstance-ladybug`, `happenstance-sync`) owe nothing while that key stands; the rule
attaches the moment `xtask/src/package.rs`'s `reconcile` check would otherwise let the crate
onto the registry without it.

**The publication test is the whole reason this is not simply Option A.** Applied without it,
the rule would re-export `serde_json` from `happenstance-neon`, a `publish = false` skeleton
whose bodies are `todo!()` — a semver promise a crate that makes none can never be observed
keeping. `happenstance-postgres`'s own `pub use sqlx;`, the audit's cited exemplar, has the
identical defect: it has never been observed by a consumer and cannot be, because the crate has
never shipped. The discriminator that predicts harm is not "is it a driver" but "will a
consumer of a *published* crate hold this type, and does the dependency take majors often" —
which is why the rule is scoped to crates that actually ship.

## Why `tokio` is out

`JoinError` and `TryCurrentError` are variants of `happenstance-sqlite`'s public error enums,
so the signature arithmetic says `tokio` qualifies. The owner ruled the caveat does not rescue
the path: the crate takes `tokio` at `features = ["rt"]`
(`crates/happenstance-sqlite/Cargo.toml:37`), so `happenstance_sqlite::tokio` would be a
*partial* `tokio`, and a consumer who reached it that way and then wrote `#[tokio::main]` would
meet an `error[E0433]` farther from its cause than the `error[E0308]` the whole policy exists to
prevent. A narrower alternative — re-exporting only the two named types — was offered and
declined; the owner took the whole removal, recorded as available again at `0.3.0` if this
proves wrong. The omission is compiled, not merely documented: a `compile_fail,E0433` doctest
in `happenstance-sqlite`'s lib sits on the exact path a reader following the old README would
take, proven non-vacuous by restoring `pub use tokio;` and watching the doctest fail to fail.

## What this does to RS-40-4

The constitution atom said the re-export set is *exactly* the crates whose types appear in the
crate's own public signatures. `happenstance-sqlite` is now a standing counter-example to that
sentence, so it is amended rather than left standing: the signature arithmetic is **necessary**
and not **sufficient** — a crate whose types appear in your signatures is a *candidate*, and it
earns the re-export only if the path it hands a reader is shorter than the one they would have
walked anyway. A crate taken at a partial feature set fails that second test. The amendment
carries the worked case, the requirement to state the omission at the item, and the requirement
to fence it with a compiled negative control.

## What this does not decide

Whether the five publishable crates' version numbers move in lockstep is a different decision
and is not settled here: CF-32 `[FROZEN]` already requires `happenstance-testkit` to carry its
own independent `version` key, and a lockstep promise across the family would contradict a
frozen clause in the one medium its manifest check cannot see. It needs its own ADR against
CF-32, not a scoping clause riding on a re-export brief. Also open: whether
`happenstance-cloudflare` gains a `[features]` table before publishing, and whether the two
crates' live `0.0.0` registry reservations should be yanked before a real `0.2.0` release.
