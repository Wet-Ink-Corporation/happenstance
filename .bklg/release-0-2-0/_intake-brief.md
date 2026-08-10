---
item: HS-I0003
stage: intake
created: 2026-08-10T02:55:02.525Z
updated: 2026-08-10T02:55:02.525Z
template_sig: 9b551827
rendered_sig: b5546637
---

# Intake Brief — 0.2.0: the flagship adapter and first publish

## Problem

After the alpha, the contract has one consumer and no real store. Everything that
implements `EventStore` today either lives in memory or is a skeleton whose bodies
are `todo!()`, and a `todo!()` body type-checks against any signature — so the
freeze has never met a database. Publication compounds it: nothing is on crates.io,
so every decision so far has been reversible at no cost to anyone, and the MSRV,
the public surface and the semver promise are all still private opinions.

## Desired Outcome

`0.2.0` on crates.io, behind a real SQLite adapter that has run the conformance
suite and passed it — including the concurrency family at 64 contenders and a
write that survives a process reopen. After this the contract is a promise to
somebody else, and ADR-0004 loses its provisional marker because the MSRV becomes
one too.

## Constraints

- **Phase 7 before phase 8.** The typed layer is the consumer that discovers
  contract defects, and discovering them after the flagship adapter is written is
  the sequence the whole plan exists to avoid.
- **`EventId` and `recorded_at` are columns in migration 1.** They were settled at
  phase 4 for exactly this reason; a schema written before them would need a
  migration before its first release.
- **A crates.io release can be yanked but never edited.** The `--cfg docsrs`
  rustdoc build is the one class of breakage that cannot be fixed after the fact.
- **Non-goals.** No Cloudflare, no Postgres, no Neon, no replication. Those are
  the next initiative and none of them is on the path to 0.2.0.

## Open Questions

- SQLite's schema, tag storage, and the **append-condition SQL strategy** — how a
  condition becomes SQL, and what that costs under contention. (ADR-0022;
  `.kb/open-questions/append-condition-sql-strategy.md`)
- Whether `cargo-semver-checks` against a registry baseline reports anything the
  branch-baseline run has been missing. It cannot until a real version exists,
  which is itself part of what this initiative buys.

## Proof artefact

- **Phase 8** — the concurrency macro green at 64 contenders, and an acknowledged
  write surviving a process reopen. Both are properties an in-memory store gets
  for free and a real one does not; a wrong durability story fails the second and
  a wrong locking story fails the first.
- **Phase 12** — docs.rs green under `--all-features` and the `docsrs` cfg, and
  `cargo-semver-checks` reporting against a **registry** baseline rather than a
  branch one. The branch baseline catches a break introduced by one pull request;
  it is blind to a break merged two pull requests ago, which is exactly the class
  a first publish exposes.

## Clauses

- **ES-\*** and **VT-\*** — `[FROZEN]`. This initiative is the first real test of
  that freeze, and a clause that has to move needs a new ADR, not an edit.
- **ADR-0004** loses its `provisional` marker at phase 12, when first publish turns
  the MSRV into a promise.
- **ADR-0003** stays provisional past this initiative — it loses the marker at
  phase 13, when a payload round-trips between two stores.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` and will not leave `intake` until
every one is ticked.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.

On the fifth box: this initiative freezes no new port, but it is the first time
the `EventStore` freeze meets a real database rather than a skeleton. The axis is
durability and concurrency — the two things memory gives away free — and the thing
at the other end of it is `happenstance-sqlite` under the 64-contender macro and a
process restart.
