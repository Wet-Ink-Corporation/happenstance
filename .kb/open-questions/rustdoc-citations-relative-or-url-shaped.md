---
id: kb-open-question-rustdoc-citation-form-001
title: Now the repository is public, should its file:line citations be relative-path or URL-shaped?
kind: open_question
status: accepted
authority_tier: note
summary: >-
  repository-url-and-security-channel.md named this as a live sub-question the moment the
  repository stopped being private, and ADR-0041 (the-repository-is-public-and-security-has-an-email-channel)
  settles the publication and the security-channel edit without settling this. spec/SPECIFICATION.md,
  every ADR under references/adr/, and the Rust constitution under standards/rust/ all cite by
  file:line — a form that resolves for a git checkout and for GitHub's own web view of the tree
  (which renders file:line-shaped fragments as anchors), but not for a docs.rs reader, who receives
  the rendered rustdoc and none of the surrounding source tree. cargo xtask spec-trace and cargo
  xtask lint-constitution both verify that a citation's file:line resolves inside the repository;
  neither checks, nor could check from inside the repository, whether the same citation resolves
  for a reader who arrived from crates.io. This is distinct from
  kb-open-question-docs-citation-anchor-contradiction-001 and kb-decision-0045's anchor-matches-exactly
  rule, both of which ask whether a citation still points at the content it claims to — a question
  answerable entirely inside the repository. This question is about whether the citation's spelling
  is legible to a reader standing outside it at all, which only became a live cost once Option A of
  the repository-visibility brief was ratified.
depends_on: []
related:
  - kb-decision-0041
  - kb-open-question-docs-citation-anchor-contradiction-001
  - kb-decision-0045
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/repository-url-and-security-channel.md
  - Cargo.toml
  - SECURITY.md
last_reviewed: 2026-09-07
---

# Now the repository is public, should its file:line citations be relative-path or URL-shaped?

## What is true today

The repository is public. `Cargo.toml`'s workspace package table carries
`repository = "https://github.com/Wet-Ink-Corporation/happenstance"` and
`homepage` at the same URL, both non-null, and `SECURITY.md` states that
`security@wet-ink.net` is the working channel with the GitHub advisory link
kept as "the better mechanism where it is available" — the state
`repository-url-and-security-channel.md` recommended and that was ratified in
two passes, first the security-channel fix (Option B) and then publication
itself (Option A). Neither pass touched how this project cites its own
evidence.

That evidence is cited almost everywhere as `path:line` or `path:line-range`:
`spec/SPECIFICATION.md`'s clauses, every ADR under `references/adr/`, the
twenty-seven atoms under `standards/rust/`, and this file's own siblings under
`.kb/`. Two gate steps hold that form honest from inside a checkout —
`cargo xtask spec-trace` for the specification's cross-references and
`cargo xtask lint-constitution` for the constitution's — and `kb-decision-0045`
extended the same discipline to `.kb/` itself: a citation anchor matches
exactly or the lint refuses. All three tools run against files on disk in a
git working tree. None of them, and nothing else in the gate, evaluates
whether the same string resolves for a reader who has none of that tree open —
specifically, a reader on docs.rs, who receives the rendered rustdoc for one
crate and not the surrounding `spec/`, `references/` or `.kb/` directories a
`path:line` citation assumes are sitting beside it.

GitHub's own web view softens this for a reader who followed the `repository`
link and is browsing there: it recognises a `#L123`-style fragment and a raw
`path:line` string is at least findable by search within that view. docs.rs
gives no such affordance — a citation into `spec/SPECIFICATION.md:8684` on a
docs.rs page is prose with no destination a click can reach.

## What is not decided

Whether the citation form used across `spec/`, `references/adr/`,
`standards/rust/` and `.kb/` should change to something that resolves for a
reader outside the repository — a full GitHub URL with a commit pin or branch
ref, a mixed form (relative path for in-repo readers, URL for rendered docs),
or no change at all, on the argument that a citation's job is to be checkable
by someone willing to open the repository, and docs.rs was never that
audience for `spec/SPECIFICATION.md` specifically. A URL-shaped citation also
raises its own question the brief that named this did not reach: pinned to a
commit (stable forever, silently stale the moment the cited line moves) or to
a branch (always current, breaks the moment the line moves at all).

## What forces it

Nothing yet with a deadline attached, unlike the security-channel and
publication questions this one was filed beside. What would force it is the
first time a rustdoc page's own citation — the kind `standards/rust/` atoms
and the crates' doc comments both carry into published documentation — is
read by someone who only has docs.rs open, and the citation fails them
silently rather than 404ing the way the old `repository` link did.

## Ordered sub-questions

1. Does this question even apply uniformly, or only to citations that appear
   in *published* rustdoc (crate doc comments, `README.md`s compiled via
   `include_str!`) as opposed to citations confined to `spec/`, `.kb/` and
   `references/`, which a docs.rs reader was never going to reach regardless
   of citation form?
2. If a URL-shaped form is adopted anywhere, is it pinned to a commit or to a
   branch, and does that choice differ between a specification clause (which
   `spec-trace` already re-verifies on every gate run, so staleness is caught)
   and a rustdoc citation (which no gate step re-verifies once published)?
3. Does adopting either form retroactively touch `kb-decision-0045`'s
   anchor-matches-exactly rule, or is that rule orthogonal — it verifies a
   citation still points at its subject, and this question is only about how
   the pointer is spelled for a reader standing outside the checkout?
