---
id: kb-open-question-cloudflare-feature-gate-001
title: happenstance-cloudflare carries no [features] table, and worker reaches every construction path
kind: open_question
status: superseded
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
  Resolved 2026-09-29 by kb-decision-0066, the 1.0 charter: no features table, with the reason on the
  record. worker reaches every construction and error path, so a feature that excluded it would
  exclude the adapter. Classified additive, which takes it off phase 17's list. After 1.0, a feature
  that gates only new items is a minor. The only breaking shape would be putting an existing item
  behind a new default feature, and nothing asks for that.
depends_on: []
related:
  - kb-decision-0023
  - kb-decision-0009
  - kb-open-question-worker-async-trait-ban-001
  - kb-open-question-workerd-runner-absent-001
  - kb-decision-0066
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - .kb/_intake/remediation-2026-09-04-briefs/stringified-throw-visibility.md
  - crates/happenstance-cloudflare/Cargo.toml
  - crates/happenstance-cloudflare/src/sql_storage.rs
  - crates/happenstance-cloudflare/src/js.rs
  - crates/happenstance-cloudflare/src/lib.rs
last_reviewed: 2026-09-29
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

## Closed — 2026-09-29

`kb-decision-0066`, the 1.0 charter, answers this the way sub-question 2 proposed: a decision
saying "no features table, and why", rather than letting the question retire unanswered. The facts
it rests on still hold at HEAD. `crates/happenstance-cloudflare/Cargo.toml` has no `[features]`
table, while every other adapter has one (`happenstance-sqlite`'s at `:100`, `happenstance-postgres`'s
at `:82`, `happenstance-neon`'s at `:115`). `worker.workspace = true` is unconditional at `:102`.
The crate re-exports `worker` next to `happenstance_core` (`crates/happenstance-cloudflare/src/lib.rs:587`),
so `worker`'s types sit on the public surface as well as in the signatures this atom listed.

The sub-questions:

1. **No such path exists, and none is planned.** Nothing in the tree or the runbook proposes a
   construction path that does not link `worker`. The adapter is worker-shaped from end to end.
2. **A decision, taken in the charter** rather than in a record of its own, because it is one row
   of the crate set's shape and not a design with alternatives worth a separate record.
3. **Moot until an axis appears.** If one ever does, a feature that gates only *new* items is
   additive after 1.0. Gating an *existing* item behind a new default feature would break every
   consumer on `default-features = false`, and that is the only shape that would need a major.

Phase 16's classification list named this as a candidate breaking question. It is additive,
because having no features table breaks nobody, so it leaves phase 17's list with this closure.
Closed by hand in phase 16. No accepted decision was edited.
