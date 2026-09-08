---
id: kb-open-question-stale-0-0-0-name-reservations-001
title: Whether the live 0.0.0 name-reservation releases should be yanked before a real 0.2.0 ships
kind: open_question
status: accepted
authority_tier: note
summary: >-
  happenstance-core, happenstance and happenstance-testkit each carry both a 0.0.0 and a
  0.2.0-alpha.1 release on crates.io, and happenstance-sqlite and happenstance-cloudflare are live
  at 0.0.0 only, published 2026-08-18 and 2026-08-20 as pure name reservations - the sqlite release
  carries zero dependencies, so it never linked rusqlite or exposed a driver type to anyone. Two
  independent briefs read the registry for unrelated reasons and both had to argue, rather than
  assert, that a live 0.0.0 does not settle the question each was actually asking - one about
  driver re-export policy, the other about the repository URL 0.0.0 already publishes to three
  crate pages. Neither brief needed to resolve whether the reservations themselves are the right
  thing to have live, and neither did; both simply worked around the ambiguity in front of them.
  What is not decided is whether the 0.0.0 releases should be yanked before 0.2.0 makes them
  redundant, so that a later decision does not have to re-derive, each time, why a 0.0.0 in its own
  disjoint compatibility range confers nothing on 0.2.0. Forced by the next registry-dependent
  question this ambiguity recurs in, or by phase 12's publish of 0.2.0, after which the reservations
  are either yanked as superseded or left standing as a permanent, slightly confusing prefix to
  every crate's release history.
depends_on: []
related:
  - kb-open-question-adapter-version-lockstep-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adapter-driver-reexport-policy.md
  - .kb/_intake/remediation-2026-09-04-briefs/repository-url-and-security-channel.md
last_reviewed: 2026-09-07
---

# Whether the live `0.0.0` name-reservation releases should be yanked before a real `0.2.0` ships

## What is true today

Registry state, verified 2026-09-03 and 2026-09-04 across two independent
briefs querying `crates.io`'s API directly rather than reading it off a
repository file:

- `happenstance-core`, `happenstance`, `happenstance-testkit`: **both** `0.0.0`
  and `0.2.0-alpha.1` are live and unyanked.
- `happenstance-sqlite`: **`0.0.0` only**, published 2026-08-18, not yanked.
  `/api/v1/crates/happenstance-sqlite/0.0.0/dependencies` returns an empty
  list — the release links nothing.
- `happenstance-cloudflare`: **`0.0.0` only**, published 2026-08-20, not
  yanked.

All five `0.0.0` publications land in the same August window as a set, ahead
of any of the crates having a real body a consumer could depend on for
behaviour. They bought the name across the family and nothing else: under
Cargo's semver rules every `0.0.z` is its own compatibility range, incompatible
with every other version, so a `0.0.0` release confers no obligation on
`0.2.0` and no consumer could have written a version requirement against it
that resolves to anything else.

**Two unrelated briefs met this same fact and both had to argue past it rather
than use it.** The driver re-export brief
(`adapter-driver-reexport-policy.md`) was investigating whether
`happenstance-sqlite` and `happenstance-cloudflare` should re-export their
driver crates, and needed to know whether a live `0.0.0` foreclosed sealing
the surface later (Option C in that brief). It could not simply say "the
crates are unpublished" — that premise was checked against the registry and
found false, and the brief records correcting an earlier draft that had relied
on it, the same mistake ADR-0029 made once already with "nothing is
published." The argument that survived is narrower: `0.0.0` exposed no driver
type because it linked no dependencies, so nothing a later seal would break
has actually shipped. The repository-URL brief
(`repository-url-and-security-channel.md`) read the same registry rows for an
unrelated question — whether `0.2.0` may ship with a `repository` URL that
404s anonymously — and used the version table only as evidence that three
crates are "publishing this URL to the public today, not at some future
release," without needing to ask whether the `0.0.0` rows themselves should
still be there.

Neither brief needed to settle whether the reservations are the right thing to
leave live, and neither did. Both simply built an argument narrow enough to
route around the ambiguity in front of them.

## What is not decided

Whether the `0.0.0` releases across all five crates should be yanked before
`0.2.0` ships as the real release, so that the next question this same
ambiguity touches does not have to re-derive the "a `0.0.0` in a disjoint
range confers nothing" argument from scratch. Yanking costs nothing a consumer
could be relying on — no version requirement resolves to `0.0.z` from outside
it — but it is also not obviously worth doing: the reservations are already
inert, and a yank is itself a registry action with its own small, permanent
footprint on each crate's release history.

## What forces it

The next registry-dependent decision this ambiguity recurs in — a third brief
having to write the same "why a live `0.0.0` doesn't count" argument a third
time is this repository's own named symptom of a missing check, per
`xtask/src/main.rs:625-635`'s argument about a finding raised three times. Or
phase 12's publication of `0.2.0` (`RUNBOOK.md:4681`), which is the natural
moment to either yank the reservations as superseded by the real release or
decide deliberately to leave them standing.

## Ordered sub-questions

1. Is there any reason to prefer leaving the `0.0.0` reservations live once
   `0.2.0` exists, or is yanking them strictly cleanup with no downside?
2. Does yanking all five happen together, or only the two — `happenstance-sqlite`,
   `happenstance-cloudflare` — that have no `0.2.0-alpha.1` companion yet?
3. Who owns running the yank, and is it a `0.2.0` release-checklist item or a
   follow-up after?
