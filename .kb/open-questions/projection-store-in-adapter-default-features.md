---
id: kb-open-question-adapter-default-projection-feature-001
title: Whether the projection port stays in happenstance-neon's and happenstance-postgres's default features
kind: open_question
status: superseded
authority_tier: note
summary: >-
  ADR-0036 ships ProjectionStore behind an off-by-default unstable-projection gate, and
  happenstance-sqlite left projection-store out of its default set on 2026-09-02 so that cargo add
  would stop walking around it. happenstance-neon and happenstance-postgres still carry
  default = ["event-store", "projection-store"], and what is not decided is whether they should.
  The exposure is not the one happenstance-sqlite had, and the difference runs the wrong way: both
  crates name happenstance-core = { features = ["std", "unstable-projection"] } in their
  dependency table, outside any feature, so unlike happenstance-sqlite's gated forward, changing
  default closes nothing on its own — --no-default-features still enables the core gate in that
  build. Latent rather than live today, because neither crate is published; spec/SPECIFICATION.md:392
  is the impl census and this atom does not restate it. Owner: the postgres-and-neon-stores
  project, whose deskeleton work already contemplates shipping happenstance-neon with
  projection-store off by default. Forced before either crate is published, which is when a
  default nobody re-read becomes a consumer's problem.
  Resolved 2026-09-03 — no, they do not stay. Both crates now read
  happenstance-core = { workspace = true, features = ["std"] }, default = ["event-store"] and
  projection-store = ["happenstance-core/unstable-projection"], the shape happenstance-sqlite took
  on 2026-09-02, verified across all four feature combinations for each crate, eight checks in
  total. This atom's central observation held exactly as written and is the reason the fix took two
  edits per crate rather than one: trimming default alone would have produced a crate that looks
  gated and is not, so the flag had to leave the unconditional dependency table and become the
  feature's forward at the same time. Closed here rather than by the owning postgres-and-neon-stores
  project, whose stories are all still at plan, because the change is three manifest lines per crate
  with no Rust touched — so the owning project inherits it as done rather than as owed. ADR-0036
  stands exactly as written, its [PROVISIONAL] marker on PS-3 does not move, and
  spec/SPECIFICATION.md's impl census is untouched. Neither crate is published and both remain
  publish = false skeletons, so the exposure closed was latent rather than live.
depends_on: []
related:
  - kb-decision-0017
  - kb-decision-0036
source_paths:
  - .kb/_intake/ps-3-projection-port-ships-gated.md
  - .kb/_intake/adapter-default-projection-feature-resolved.md
  - crates/happenstance-neon/Cargo.toml
  - crates/happenstance-postgres/Cargo.toml
  - crates/happenstance-sqlite/Cargo.toml
  - crates/happenstance-core/Cargo.toml
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
---

# Whether the projection port stays in happenstance-neon's and happenstance-postgres's default features

## What is true today

`kb-decision-0036` ships `ProjectionStore` behind the off-by-default
`unstable-projection` feature in `happenstance-core` and `happenstance`, with the
surface making no semver promise. `happenstance-sqlite` forwarded that gate through
its own `projection-store` feature and, until 2026-09-02, also listed
`projection-store` in `default`, which meant `cargo add happenstance-sqlite` turned
the unfrozen port on without the consumer naming it. That was resolved the same day:
`happenstance-sqlite`'s `default` is now `["event-store"]` alone, and the family of
tests gated on `all(feature = "projection-store", feature = "conformance")` was
confirmed to run 0 tests bare and 24 under `--all-features` either way, so nothing
was lost by moving it.

`happenstance-neon` and `happenstance-postgres` were not touched by that fix and
still carry `default = ["event-store", "projection-store"]`. What is open is whether
they should follow `happenstance-sqlite`'s move.

## Why the exposure is not the sqlite exposure, and runs the wrong way

For `happenstance-sqlite`, `projection-store` forwards
`happenstance-core/unstable-projection` — a genuinely gated feature edge — so
removing the forward from `default` closes the walk-around. For
`happenstance-neon` and `happenstance-postgres`, the dependency declaration is
different in a way that changes what a fix would even do: both crates' `Cargo.toml`
name `happenstance-core = { features = ["std", "unstable-projection"] }`
unconditionally, in `[dependencies]`, outside any `[features]` table entry. That
means `unstable-projection` on `happenstance-core` is already enabled by simply
depending on either adapter, with or without `default-features`, with or without
`projection-store` — `--no-default-features` on `happenstance-neon` or
`happenstance-postgres` still enables the core gate in that build. Moving
`projection-store` out of `default` here would change which adapter-level surface a
consumer must ask for; it would not, by itself, restore `unstable-projection` to
off-by-default the way it did for `happenstance-sqlite`.

## Why it is latent rather than live

Neither crate is published, and `spec/SPECIFICATION.md`:392 already states the impl
census this atom does not restate: `NeonProjectionStore` carries real bodies in all
four port methods and runs against nothing, and `PostgresProjectionStore` is
`todo!()` throughout. So today the mis-scoped default costs no real consumer
anything. It becomes a consumer's problem at first publish, which is the forcing
event.

## What is not decided

Whether `projection-store` should leave `default` on these two crates, whether the
`happenstance-core` dependency declaration should itself move `unstable-projection`
behind a forwarded feature (mirroring the pattern `happenstance-sqlite` uses) rather
than enabling it unconditionally, and whether the two changes should land together
or separately given that the second is the one that actually restores
off-by-default.

## Who owns it, and when it is forced

The `postgres-and-neon-stores` project
(`.bklg/from-contract-to-published-library/postgres-and-neon-stores/`), whose
`deskeleton-and-package-readiness/discover.md`:45 already contemplates shipping
`happenstance-neon` with its `projection-store` feature off by default — so the
question has a named owner rather than an orphaned finding. It is forced before
either crate's first publish, which is when an unexamined default becomes a promise
made to a consumer rather than a property of an unpublished skeleton.

## Resolved 2026-09-03 — status `superseded`; the flag left the dependency table

Everything above is the state of knowledge on 2026-09-02 and is left exactly as it was
written. Unlike `kb-open-question-cf-40-ownership-001` and
`kb-open-question-worker-async-trait-ban-001`, **there is no answering decision atom, and
deliberately so.** `kb-decision-0036` already commits `ProjectionStore` to an off-by-default
gate; two more adapters honouring that gate add no obligation, name no rejected alternative
and settle no fork, and `.kb/decisions/README.md`:32 calls a decision recorded without
rejected options "indistinguishable from an accident". Both earlier resolutions in this
layer produced a new decision atom because a decision already existed for reasons of its
own. This one is ADR-0036 being enforced in the two places that had not yet enforced it, so
the record is this amendment and the manifests it cites.

**The answer is no — they do not stay.** `happenstance-neon` and `happenstance-postgres`
now both read `happenstance-core = { workspace = true, features = ["std"] }` in
`[dependencies]`, `default = ["event-store"]`, and
`projection-store = ["happenstance-core/unstable-projection"]` — the shape
`happenstance-sqlite` took on 2026-09-02, with the reasoning written inline beside each of
the three changed lines in both manifests.

**The diagnosis in "Why the exposure is not the sqlite exposure" held exactly as written,
and is why the fix took two edits per crate rather than one.** Because `projection-store`
was `[]` while the dependency table carried `unstable-projection` unconditionally, the
feature named the capability while doing none of the gating: trimming `default` alone would
have produced a crate that *looks* gated and is not — a worse state than the one it started
in, because it would read as fixed. Both halves had to move together, the flag leaving the
unconditional dependency and becoming the feature's forward, because that is the only
arrangement in which `--no-default-features` yields a build without the unfrozen port. The
third of the three things this atom left undecided — whether the two changes land together
or separately — is therefore answered by the mechanism rather than by preference: together,
or not at all.

**Verified across all four feature combinations for each crate** —
`--no-default-features`, default, `--features projection-store`, `--all-features` — eight
checks in total. No Rust was touched: both crates' projection modules were already gated on
`feature = "projection-store"` in `src/lib.rs`, which is what let the forward move through
the manifest alone.

**What this does not change.** `kb-decision-0036` stands exactly as written and its
`[PROVISIONAL]` marker on PS-3 does not move — nothing here changes what has run the
projection suite. `spec/SPECIFICATION.md`'s impl census is untouched, and is still not
restated here. Neither crate is published and both remain `publish = false` skeletons whose
projection bodies are `todo!()`, so the exposure that closed was latent rather than live;
closing it now was cheap precisely because nothing depended on the old shape.

**Closed here rather than by the named owner.** The `postgres-and-neon-stores` project has
not started — its stories are all at `plan` — and the deadline this atom set was *before
either crate is published*. Three manifest lines per crate with a mechanical verification
did not need a project, and leaving a known walk-around in place until one starts is how a
default nobody re-read becomes a consumer's problem. The owning project inherits the change
as done rather than as owed: its `deskeleton-and-package-readiness` story already
contemplated shipping `happenstance-neon` with `projection-store` off by default, and now
finds it that way.
