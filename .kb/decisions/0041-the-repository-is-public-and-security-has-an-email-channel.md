---
id: kb-decision-0041
title: The repository is public, and the vulnerability channel does not depend on it
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0041
reversibility: low
phase: 12
supersedes: null
superseded_by: null
summary: >-
  The workspace's repository field pointed at a GitHub URL that 404'd for an
  anonymous reader while the organisation's repository was private, and that took
  SECURITY.md's only reporting channel down with it -- a reporter following the
  documented private-advisory link found nothing, and the document's own next
  sentence told them not to open a public issue instead. Two options landed in
  sequence rather than one being chosen over the other. Option B landed first,
  2026-09-04: security@wet-ink.net leads SECURITY.md as the channel that works
  today, the GitHub advisory link is kept as the preferred mechanism where it
  resolves, and the document says plainly that the link may 404 and that is not
  the reporter's error. Option A followed after a costed pre-publication sweep for
  credentials, leaked local paths and personal data: the repository itself was
  published, which resolves the repository field on crates.io, makes the file:line
  citations threaded through every crate's rustdoc followable, and is the only
  option that closes the citation problem rather than only the reporting one.
  .redkiln/telemetry/ shipped with the publication as the one tree in the exposure
  table that is data about a person's working pattern rather than argument about
  the software.
depends_on: []
related:
  - kb-open-question-rustdoc-citation-form-001
  - kb-open-question-stale-0-0-0-name-reservations-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/repository-url-and-security-channel.md
  - SECURITY.md
  - Cargo.toml
last_reviewed: 2026-09-07
---

# The repository is public, and the vulnerability channel does not depend on it

## Decision

The workspace repository is public, and `SECURITY.md`'s vulnerability-reporting
channel no longer depends on that fact holding. Both halves are decided and
landed, in two stages, on 2026-09-04.

## What was broken

Three crates were already on crates.io and two more were joining at `0.2.0`.
Every manifest in the workspace carried `repository =
"https://github.com/Wet-Ink-Corporation/happenstance"` through workspace
inheritance, and that URL did not resolve for an anonymous reader — the
organisation existed and had one public repository, a different project
entirely, so the 404 was a visibility answer rather than a typo.

The consequence reached further than a broken link on a crate page.
`SECURITY.md`'s only vulnerability-reporting channel was a GitHub
private-advisory link on the same host. A reporter who found a soundness bug in
the conformance suite — the exact class of problem the document calls the
library's whole purpose — would follow that link, get a 404, and read the
document's next paragraph telling them not to open a public issue instead. The
instruction the document gave when its one channel was unreachable was
silence.

The stakes were not cosmetic for a second reason: this project's documentation
is built out of citations. `README.md`, `CHANGELOG.md` and the crate rustdoc
route a reader to `spec/SPECIFICATION.md`, `.kb/decisions/`, `references/adr/`
and `standards/rust/` for the reasoning behind nearly every non-obvious choice.
Anonymously, none of that was open. A crate whose documentation cites a
repository nobody can read is a different product from the one the
documentation describes.

## Option B, landed first — an out-of-band channel

`SECURITY.md` was given `security@wet-ink.net` as the channel that works
today, with the GitHub advisory link retained as the better mechanism where it
resolves. Three sentences were added deliberately rather than left implicit,
because a reporter meets the document in a different state than its author
does:

1. The 404 is stated, not left to be discovered — the document says the link
   may not resolve and that nothing is wrong with the reporter's own setup if
   it does not.
2. "Report against the source in this repository" was corrected to allow for a
   reader who cannot reach the repository: report against the source, and
   while the repository is private, describe the finding by email without
   being asked for a link that does not resolve for them.
3. The acknowledgement paragraph gained "or by email", since its original "say
   so on the same thread" presumed the thread existed.

This closed the reporting hole and nothing else. The `repository` field still
404'd on three published crates, `homepage` and `documentation` remained
unset, and every `file:line` citation in the published rustdoc still led
nowhere.

## Option A, landed second — publishing the repository

The repository owner approved publishing the repository after a
pre-publication sweep costed the exposure rather than assuming it. Publishing
a subset was considered and rejected: the publishable surface cites the
process trees 115 times — `experiments/` 64, `references/evaluation/` 32,
`.bklg/` 16, `.redkiln/` 3 — and `cargo xtask spec-trace` cites
`references/adr/` by line range, so that tree could not be excluded at all.
Excluding a tree does not hide it; it turns a working citation into a dangling
one on precisely the pages built to show their work, the same defect as
amending a clause to match an implementation rather than the reverse.

The pre-publication sweep, run before the switch because publishing a private
repository exposes its whole history at once, found no credential patterns, no
leaked absolute local paths in any tracked file, and no occurrence of the
owner's personal email in tracked content (it appears only in git commit
metadata, ordinary for an open repository). `.redkiln/telemetry/` was
identified as the one tree that is not argument: its records are session
markers disclosing working patterns — which days and hours work happened —
rather than anything about the software. It shipped anyway: removing it costs
three citations against roughly 390,000 published lines of process artefact,
the smallest severance in the exposure table by two orders of magnitude.

## What this decides and what it leaves open

Publishing resolves the `repository` field on every crate that carries it and
makes the citation graph followable from a published crate's rustdoc, which an
email channel alone could never do. `SECURITY.md`'s "that link does not
resolve for everyone" paragraph becomes false once the repository is public
and is due one further edit to remove it — deliberately not made as part of
this decision, because writing that the repository is public before it is
would be the same class of untrue-but-green claim this remediation pass
existed to remove.

Whether the published rustdoc's `file:line` citations should be relative-path
or URL-shaped for a reader on docs.rs, and whether the `0.0.0` placeholder
crate reservations should be yanked before a real release, are both left open
and are not settled by this decision.

## Alternatives rejected

Pointing `repository` at a public mirror, or dropping the key entirely.
Dropping it removes the reader's only signal that a source exists at all;
mirroring splits the project's identity and creates a second tree to keep in
sync, which this repository's own experience with citation drift says will
fail quietly. Shipping `0.2.0` with the URL wrong and fixing it afterward was
also rejected: a published crate version's manifest is immutable on the
registry, so the wrong `repository` field would have been permanently wrong
for that version, for as long as the registry exists.
