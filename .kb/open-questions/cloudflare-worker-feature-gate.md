---
id: kb-open-question-cloudflare-feature-gate-001
title: happenstance-cloudflare carries no [features] table, and worker reaches every construction path
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Two independent remediation briefs each raised the same item and routed it elsewhere rather
  than deciding it: happenstance-cloudflare declares no [features] table at all, worker is an
  unconditional dependency, and worker's types (SqlStorage, State, Error) reach every public
  construction path the adapter offers — SqlStorage::new, SqlStorage::from_state, JsThrow::from_error
  and JsThrow::error. So a feature that excluded worker would exclude the adapter itself, which is
  the reason neither brief tried to design one. What is not decided is whether the crate should gain
  a features table before it first ships a real release — for parity with happenstance-sqlite's
  event-store/projection-store split, to gate a future non-worker construction path, or for some
  other reason neither brief names — or whether "no features table" is simply the correct shape for
  a crate with exactly one storage backend and one platform. This is distinct from
  kb-open-question-worker-async-trait-ban-001, which is settled (kb-decision-0035) and is about
  whether cargo deny's async-trait ban tolerates worker's own dependency graph, not about whether
  happenstance-cloudflare exposes a Cargo feature at all.
depends_on: []
related:
  - kb-decision-0023
  - kb-decision-0009
  - kb-open-question-worker-async-trait-ban-001
  - kb-open-question-workerd-runner-absent-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - .kb/_intake/remediation-2026-09-04-briefs/stringified-throw-visibility.md
  - crates/happenstance-cloudflare/Cargo.toml
  - crates/happenstance-cloudflare/src/sql_storage.rs
  - crates/happenstance-cloudflare/src/js.rs
last_reviewed: 2026-09-07
---

# happenstance-cloudflare carries no [features] table, and worker reaches every construction path

## What is true today

`crates/happenstance-cloudflare/Cargo.toml` declares no `[features]` table — `grep -n
"^\[features\]"` over the file returns nothing, and the file is 187 lines long, so this
is not an oversight of scale. `worker.workspace = true` is an ordinary, unconditional
dependency (`Cargo.toml:102`); there is no build of the crate that does not link it.

`worker`'s own types are not confined to internals a feature gate could hide behind:
they sit on the adapter's public construction and error paths. `SqlStorage::new(sql:
worker::SqlStorage)` and `SqlStorage::from_state(state: &worker::State)`
(`crates/happenstance-cloudflare/src/sql_storage.rs:248, 263`) are how a caller builds
the event store at all, and `JsThrow::from_error(error: worker::Error)` /
`JsThrow::error(&self) -> &worker::Error` (`crates/happenstance-cloudflare/src/js.rs:188,
197`) are how a caller inspects what went wrong. Compare `happenstance-sqlite`, the
crate's nearest sibling, which does carry a features table splitting `event-store` from
`projection-store` — `happenstance-cloudflare` has never needed the same split because it
has one storage backend, one platform, and one driver reaching every path.

Two independent remediation briefs noticed the same fact from different angles —
`adapter-driver-reexport-policy.md` while cataloguing which adapters re-export their
driver, `stringified-throw-visibility.md` while deciding whether an internal error type
should stay public — and both filed it under "what this does not settle" rather than
proposing a shape, because a feature that excludes `worker` excludes the adapter, and
neither brief had a use case for a feature that keeps it.

## What is not decided

Whether `happenstance-cloudflare` should gain a `[features]` table before it first
ships a release that is more than a `0.0.0` name reservation, and if so, what it would
gate — there is no second storage backend, no second platform, and no construction path
that does not need `worker` today. The live alternative is that "no features table" is
simply correct for a single-backend, single-platform adapter, and the item exists in
this corpus only because the audit's own template asks every crate the same question
regardless of whether it has an axis to split on.

## What forces it

The crate's own path to publication. `happenstance-cloudflare` is `PUBLISHABLE`
(`xtask/src/package.rs`) and gate-held to publication standards already, even though the
audit deferred this and the `StringifiedThrow` visibility question past `0.2.0`
(`.github/workflows/ci.yml`). The moment either brief's author, or a reviewer of the
crate's manifest, is asked "why does this crate have no features and every other
adapter does," this question needs an answer on record rather than a re-derivation.

## Ordered sub-questions

1. Is there any construction path, present or planned, that a caller would want without
   linking `worker` — a pure-Rust test double, a non-Workers deployment target — or is
   the adapter correctly worker-shaped end to end?
2. If no such path exists, is the right resolution a decision atom stating "no features
   table, and here is why" (closing the question rather than deferring it again), or
   does the question simply retire unanswered once `0.2.0` ships without one?
3. If a features table is ever added, does it follow `happenstance-sqlite`'s precedent
   of gating *store roles* (event store vs. projection store), or does Cloudflare's
   single-role shape mean the axis, if one ever appears, is something else entirely?
