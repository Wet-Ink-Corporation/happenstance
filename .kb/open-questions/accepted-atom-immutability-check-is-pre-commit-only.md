---
id: kb-open-question-immutability-check-pre-commit-001
title: The accepted-atom immutability check is a dirty-tree guard, and it cannot tell a referent repair from a reversal
kind: open_question
status: accepted
authority_tier: note
summary: >-
  redkiln validate --kb's immutability check compares each status: accepted
  decision atom in the working tree against the same file at HEAD. Verified
  (CLAUDE.md:216): it reports "accepted decision 'kb-decision-0042' was
  edited in place" on an uncommitted edit and exits 0 the moment that edit is
  committed. It is therefore a dirty-tree guard whose honest reach is
  pre-commit, not pre-merge, and it is weaker than the surrounding prose
  implies in both directions: it will not stop a real reversal that arrives
  as a commit, and it obstructs a repair that reverses nothing — the
  2026-09-08 repointing of kb-decision-0058's three stale line ranges
  (4e13ee2), taken under rewrite-the-referent-never-the-reasoning with no
  word of reasoning moved, was refused locally and then not noticed. The
  backlog CI job that would run the check is if: false
  (.github/workflows/ci.yml:243) and, enabled, would run it against a
  checkout whose working tree is HEAD, so this half has never been able to
  fire there. What is not decided: which remedy — hash the accepted body
  into the atom's own frontmatter so history is what is compared; a
  documented carve-out for referent-only edits (a line number, a renamed
  identifier); or a diff of every accepted body against the merge base in
  CI — and whether a referent-only carve-out can be stated mechanically at
  all, or only as the governance atom's judgement test (does the edit change
  what the document asserts). What forces it: the next referent repair to an
  accepted atom, or the next attempt to restore the backlog job, which would
  restore nothing on this half. Ordered sub-questions: does the corpus want
  the instrument to catch a committed reversal, given the rule it enforces
  is a judgement and not a byte comparison; if so, is the base the merge
  base or a carried hash; and does a carve-out require the lint to know what
  a citation is.
depends_on: []
related:
  - kb-governance-referent-not-reasoning-001
  - kb-open-question-references-adr-correction-policy-001
  - kb-open-question-docs-citation-anchor-contradiction-001
  - kb-decision-0058
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/2026-09-08-intake-is-outside-the-citation-scan.md
  - CLAUDE.md
  - .github/workflows/ci.yml
  - .kb/decisions/README.md
last_reviewed: 2026-09-11
---

# The accepted-atom immutability check is a dirty-tree guard, and it cannot tell a referent repair from a reversal

## What was verified, and what it actually checks

`redkiln validate --kb`'s immutability rule compares each `status:
accepted` decision atom's body in the working tree against the same file as
`HEAD` has it. `CLAUDE.md:216` records the transcript that proves the
comparison's base: editing an accepted atom and running the check reports
*"accepted decision 'kb-decision-0042' was edited in place"* on the
**uncommitted** change, and the report clears the instant that edit is
committed — because at that point the working tree and `HEAD` agree again.
The honest name for that is a dirty-tree guard, and its honest reach is
**pre-commit**, not pre-merge. A CI checkout's working tree *is* `HEAD` by
construction, so the check could never have fired in the disabled `backlog`
job (`.github/workflows/ci.yml:243`, `if: false` since 2026-09-04) and would
not fire there if the job were switched back on tomorrow — a correction
`CLAUDE.md` itself now carries, having previously listed this rule among
what the disabled job would have caught.

## The gap in the other direction, found by using it

The 2026-09-07 KB intake wave landed `kb-decision-0058` carrying three line
citations into `crates/happenstance-sqlite/src/event_store.rs`, all three
stale on arrival — inherited from a 2026-09-04 intake brief written against
a tree a later merge-join refactor moved. `cargo xtask ci` had been red on
`main` since the wave merged, undetected because `.kb/_intake` is outside
the citation scanner's scope
(`kb-open-question-docs-citation-anchor-contradiction-001`'s sibling finding
in the same intake file names the exclusion) and nobody re-ran the gate
after the merge. The repair, commit `4e13ee2`, repointed the three ranges
and changed no other word of the atom's reasoning — exactly what
`kb-governance-referent-not-reasoning-001` prescribes for a renamed or moved
referent inside a standing decision. Run against that repair, the
immutability check reported the same *"edited in place"* refusal it reports
against a reversal: the check has no way to distinguish a citation
repointed from a conclusion reversed, because it compares bytes, not
claims. The distinction the corpus's own governance rule draws — reasoning
versus referent — is not a distinction the mechanical check can see.

## Three remedies named, none chosen

**Hash the accepted body into the atom's own frontmatter.** Comparing a
carried hash against the current body separates "this file changed" from
"this file changed since it was accepted," which is closer to what
immutability is meant to mean, at the cost of a field every acceptance must
maintain.

**A documented carve-out for referent-only edits** — a line-number range, a
renamed identifier, a moved file path — accepted without triggering the
guard. Cheapest to state, hardest to state *mechanically*: telling a
citation repointing apart from a substantive rewrite by pattern alone is
close to asking the lint to know what a citation is, which is the same gap
`kb-open-question-docs-citation-anchor-contradiction-001` names from the
opposite side (a citation that resolves but says the wrong thing).

**A CI instrument that diffs every accepted body against the merge base**,
rather than against a working tree that is already `HEAD`. This is the one
remedy that would actually run in CI and catch a committed reversal; it
does not by itself solve the referent-versus-reasoning problem, so it would
need to ship with one of the other two or accept the same false refusal
`4e13ee2` hit.

## What forces this, and what is not decided

The next referent-only repair to an accepted atom will hit the same false
refusal `4e13ee2` did, silently, unless a carve-out or hash scheme exists
first. Separately, any future attempt to restore the disabled `backlog`
job on the strength of this check needs to know it restores nothing on the
pre-commit half — that correction is already written into `CLAUDE.md`, but
no `.kb` atom carried it until now. Not decided: which of the three
remedies the corpus wants, whether a mechanical carve-out is possible at
all or only expressible as the governance atom's judgement test, and — if a
CI diff against the merge base is built — whether it subsumes the
carve-out question or still needs one beside it.
