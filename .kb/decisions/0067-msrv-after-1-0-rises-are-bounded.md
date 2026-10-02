---
id: kb-decision-0067
title: The MSRV holds at 1.97.1 into 1.0, and after 1.0 a rise is bounded
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0067
reversibility: medium
phase: 16
supersedes: null
superseded_by: null
summary: >-
  The fourth record in the ADR-0004 → ADR-0029 → ADR-0037 amendment lineage, and like the two
  before it an amendment rather than a supersession: ADR-0037 stays accepted and byte-identical.
  Three decisions and one correction. First, the number holds: 1.0.0 publishes at rust-version
  1.97.1, the floor seven crates have carried on the registry since 0.2.0. Second, the 2026-09-06
  ratification of the msrv-premise brief, which recommended lowering the floor to 1.95, is
  withdrawn. It was never executed, it was reached in a batch of nine ratified on their own
  recommendation without individual review, and the ground its brief stood on - "0.2.0 is
  genuinely the cheap moment for this and the only one - the promise begins at the stable
  release" - passed when 0.2.0 shipped at 1.97.1. Lowering stays additive at any time and is not foreclosed; it is simply no longer owed. Third,
  the rule after 1.0: the floor rises only in a minor release, never a patch; only to a Rust stable
  whose x.y.0 release is at least six months old on the day of the happenstance release that
  raises to it; and always with a CHANGELOG.md entry naming the old floor, the new one and what
  forced it. The promise is re-based, because ADR-0037's own argument stops holding at 1.0.0:
  it rested on "below 1.0 cargo already treats a minor bump as incompatible, so a floor rise cannot
  reach a consumer without a version change they chose", and a 1.x minor is caret-compatible and
  arrives with cargo update. After 1.0 the protection is Cargo's MSRV-aware resolution - resolver
  "3", the default for an edition-2024 root and available since Cargo 1.84, sets
  resolver.incompatible-rust-versions to "fallback", which prefers a dependency version whose
  rust-version the consumer's declared package.rust-version meets (or their running rustc, when
  they declare none) and falls back to an incompatible one only when nothing else matches the
  requirement. That setting is the consumer's, not this workspace's
  - Cargo reads the resolver only from the top-level workspace - so the promise is stated no
  larger than that: a consumer on resolver 3 (or with the config key set) is held on the last
  compatible 1.x minor; a consumer on resolver 2 is protected by nothing but their lockfile, and
  the six-month bound is what makes that exposure small. The correction: ADR-0029 and ADR-0037
  both say cfg_select! is unavailable before 1.88. It is unavailable on stable before 1.95 -
  absent at 1.88.0, rejected as the unstable library feature cfg_select (E0658) at 1.91, 1.93 and
  1.94, and accepted at 1.95.0, the first stable measured to take it; 1.89, 1.90 and 1.92 were not
  measured. 1.88.0, 1.94.1 and 1.95.0 were re-measured on this machine on 2026-09-29. Neither
  body is edited. Rejected: lowering to 1.95 now, a rise as a major, a moving N-2 window, and
  per-package floors. It writes up the owner's decision kb-decision-wi-460397, and closes
  kb-open-question-msrv-ratification-conflict-001.
depends_on:
  - kb-decision-0037
related:
  - kb-decision-wi-460397
  - kb-decision-0004
  - kb-decision-0029
  - kb-open-question-msrv-ratification-conflict-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - Cargo.toml
  - rust-toolchain.toml
  - .github/workflows/ci.yml
  - CHANGELOG.md
  - .kb/decisions/0029-msrv-raised-to-1-97-1.md
  - .kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md
  - .kb/open-questions/msrv-ratification-conflicts-with-the-accepted-floor.md
  - runbook/phases/16-define-1-0.md
  - runbook/phases/21-one-point-oh.md
  - runbook/ledgers.md
last_reviewed: 2026-09-29
---

# The MSRV holds at 1.97.1 into 1.0, and after 1.0 a rise is bounded

## Decision

`1.0.0` requires Rust **1.97.1**. This is the owner's choice recorded in `kb-decision-wi-460397`
(*"Hold 1.97.1; bounded rises"*), written up here with its reasons. `Cargo.toml:26` (`rust-version = "1.97.1"`) and
`rust-toolchain.toml:2` (`channel = "1.97.1"`) do not move, and neither does the `msrv` job's
spelled-out `toolchain: "1.97.1"` (`.github/workflows/ci.yml:1032-1071`). This is the number seven
crates have been published at since `0.2.0`, through `0.3.2`.

This record amends ADR-0037 and does not supersede it, on the precedent ADR-0029 set against
ADR-0004 and ADR-0037 followed against both: `kb-decision-0037` stays `accepted`, its body
byte-identical, with no `superseded_by` flip. What ADR-0037 decided — a rise is a minor named in
`CHANGELOG.md`, never a patch — is carried forward. What changes is the argument underneath it, and
two limits added on top.

## The 2026-09-06 ratification is withdrawn

`kb-open-question-msrv-ratification-conflict-001` recorded two live answers to one question.
ADR-0037 says the number does not move. The pre-publication ratification record lists
`msrv-premise` among nine briefs *"ratified **on their own recommendation**, without individual
review"* (`git show 025f300^:.kb/_intake/ratifications-2026-09-06-pre-publication.md`, lines 38-39
and 55), and that brief's recommendation is Option B, lower the floor to 1.95
(`git show 025f300^:.kb/_intake/remediation-2026-09-04-briefs/msrv-premise.md`, line 357).

The ratification is **withdrawn**, and was never executed: the discharge record of 2026-09-07
names six items and `msrv-premise` is not one, and `Cargo.toml` has read `1.97.1` at every commit
since. Three reasons, in order of weight.

1. **Its own premise expired.** The brief's case was a deadline, not a number: *"`0.2.0` is
   genuinely the cheap moment for this and the only one — the promise begins at the stable
   release"* (brief, lines 572-574, itself quoting the audit). `0.2.0` shipped at 1.97.1 on 2026-09-10. A recommendation whose stated
   reason is a window, read after the window closed, is a record of what would have been cheap.
2. **It was the weaker of two decisions.** The same ratification record reserves individual owner
   review for the briefs with a breaking arm (lines 46-47), and `msrv-premise` was not given it —
   a lower floor is additive, so the brief carried no breaking arm, and it went through as one of a
   batch. ADR-0037 is a decision record in its own right, accepted in the lineage it amends. Where
   the two disagreed, the shipped code sided with the record.
3. **Nothing forces a lower number before 1.0.** Lowering a floor is additive for ever. A lower
   floor is not refused here — it is released from being owed, and can ship in any release once
   someone makes the `msrv` job check the lower number rather than the pin.

A future reader who finds the ratification record in git history should read it as a
recommendation recorded and not adopted.

## The rule after 1.0

A rise of the `rust-version` a published crate declares:

- **ships only in a minor release** — never a patch, which is ADR-0037 carried forward;
- **only to a Rust stable at least six months old**, measured from that version's `x.y.0` release
  date to the date of the happenstance release that raises to it;
- **always with a `CHANGELOG.md` entry** naming the old floor, the new one, and what forced it — a
  crate of this workspace, or a named dependency and its version.

The six months is a bound, not a schedule. ADR-0037 rejected a moving N-2 window because it
*"breaks a build on a date rather than on an upgrade chosen"*; this rule never moves the floor on a
date, it caps how new a floor may be when a release chooses to move it. When a dependency demands a
younger compiler — the shape that forced ADR-0029, and five of the five database crates still
declare no `rust-version` — the release that would take that dependency waits until the compiler is
six months old, or does not take it.

Under the lockstep-then-independent versioning the 1.0 charter records, `rust-version` stays one
workspace number inherited through `[workspace.package]`. A rise is therefore a minor on every
crate whose declared floor it moves. It is not a major, and it is not exempt from the changelog on
any crate.

The rule does not apply backwards. 1.97.1 is the floor already promised, not a rise, so the fact
that it is younger than six months when `1.0.0` ships binds nothing.

## Why the promise needs a new footing at 1.0

ADR-0037's guarantee was a property of `0.x`, and it said so: *"below 1.0 cargo already treats a
minor bump as incompatible, so a floor rise cannot reach a consumer without a version change they
chose"* (`.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md:26-28`, and again in the body at
`:88-90`). Under `0.3`, `happenstance-core = "0.3"` cannot resolve to `0.4.0`. Under `1`,
`happenstance-core = "1"` resolves to every `1.x`, and `cargo update` takes the newest. From
`1.0.0` a floor rise **can** reach a consumer without a version bump they chose.

What stands in its place is Cargo's MSRV-aware resolver. This is stated as knowledge of Cargo,
checked against the Cargo book shipped with the 1.97.1 toolchain (`reference/resolver.html`,
*Resolver versions* and *Rust version*; `reference/config.html`,
`resolver.incompatible-rust-versions`):

- `resolver = "3"` is the default for a root whose edition is 2024, and requires Rust 1.84 or
  later. Its one change is to default `resolver.incompatible-rust-versions` from `allow` to
  `fallback`.
- Under `fallback`, the resolver prefers a dependency version whose `rust-version` is at or below
  the consumer's declared `package.rust-version`, or their running `rustc` when they declare none,
  and picks an incompatible one only when no compatible version
  satisfies the requirement. It does not error.
- **The resolver is the consumer's.** Cargo reads it from the top-level package or virtual
  workspace only and ignores it in dependencies. This workspace's `resolver = "3"`
  (`Cargo.toml:2`, with `edition = "2024"` at `:25`) governs this repository's own lockfile and
  nothing a consumer resolves.

So the promise is exactly this wide. A consumer on resolver 3, or with
`resolver.incompatible-rust-versions = "fallback"` in their Cargo config, whose declared
`package.rust-version` (or running `rustc`, when they declare none) is below a new floor, stays on the last `1.x` minor whose floor they meet, because that version still
satisfies `"1"`. A consumer on resolver 2 — an edition-2021 root that sets nothing — is protected by
their lockfile until they run `cargo update`, and by the six-month bound after that. The bound is
what keeps the unprotected case small: a toolchain six months stale is the case this rule accepts
breaking, and nothing newer is ever asked of anyone.

## A correction to ADR-0029 and ADR-0037

Both accepted bodies say the forcing macro is unavailable before 1.88:

- `.kb/decisions/0029-msrv-raised-to-1-97-1.md:43` — *"invokes the `cfg_select!` macro, which is
  unavailable before 1.88"*;
- `.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md:21` (summary) and `:79` (body) —
  *"`cfg_select!`, unavailable before 1.88"*.

The correct threshold is **1.95**. `cfg_select!` does not exist at 1.88.0; at 1.91.1, 1.93.1 and
1.94.1 it is rejected as the unstable library feature `cfg_select`; 1.95.0 is the first stable
measured to accept it. 1.89, 1.90 and 1.92 were not measured by anyone, so the claim is about the
measured set, not every release between them. The `msrv-premise` brief bisected this against
`libsqlite3-sys`'s build script (brief, lines 186-194), and this record re-measured 1.88.0, 1.94.1
and 1.95.0 on 2026-09-29 with a one-line program under each installed toolchain. The 1.91.1 and
1.93.1 rows are the brief's:

```text
rustc 1.88.0  error: cannot find macro `cfg_select` in this scope
rustc 1.91.1  error[E0658]: use of unstable library feature `cfg_select`
rustc 1.93.1  error[E0658]: use of unstable library feature `cfg_select`
rustc 1.94.1  error[E0658]: use of unstable library feature `cfg_select`
rustc 1.95.0  compiles
```

Neither body is edited. ADR-0029 and ADR-0037 are accepted, and a wrong figure in an accepted atom
is corrected by a later one, never in place. The error changed no decision: both records chose
1.97.1, above either threshold. It changes what a reader would use the number for — the lowest
floor `happenstance-sqlite` could honestly claim is 1.95, not 1.88.

Let-chains are untouched by this: ADR-0029's *"let-chains stabilised in 1.88"* (`:71`) is correct.

## Alternatives rejected

**Lower to 1.95 before 1.0 (the ratification's Option B).** Additive and honest, and still
available. Rejected as an obligation, not as a direction: nothing forces it, it is the one change
this lineage can make at any time without cost, and making it well needs the `msrv` job pinned
below the toolchain first so the new number is checked rather than asserted.

**A rise is a major.** Untenable in a workspace whose floor has already been moved once by a
dependency's build script. Five database crates declare no `rust-version`, so a silent dependency
would force a major on every crate that inherits the floor.

**A moving N-2-stable window.** Rejected again for ADR-0037's reason: it breaks a build on a date.
The six-month rule borrows only the bound, never the motion.

**Per-package floors.** ADR-0037 rejected them on verification capacity, not merit, and independent
versioning after 1.0 weakens the argument for one number. They stay deferred because the capacity
still does not exist; whether `cargo hack --rust-version` already checks each package at its own
declared floor is unverified here, and the record that adopts per-package floors owes that check.

## Falsifier

Reopen the rule if a consumer on resolver 3 is broken by a `cargo update` across a floor rise that
followed it — that would mean `fallback` does not hold them where this record says it does. Reopen
the six-month bound if a dependency this workspace cannot drop requires a compiler younger than six
months; the choice then is between the bound and the dependency, and it belongs in its own record.

## What this leaves open

- Whether the published floor is lowered to 1.95 before or after 1.0. Not owed; available.
- Per-package floors, above.
- Phase 21 checks each crate's documented MSRV statement against this policy
  (`runbook/phases/21-one-point-oh.md:40-44`). The two README bullets that state it today
  (`crates/happenstance-core/README.md:65`, `crates/happenstance/README.md:135`) say *"checked in
  CI"*, and ADR-0037's point that the `msrv` job is vacuous while the pin equals the floor still
  stands.
