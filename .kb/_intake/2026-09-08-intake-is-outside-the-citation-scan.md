# The staging directory is outside the citation scan, and the ingest carries what it holds

**Date:** 2026-09-08
**Kind:** open question, with a defect already paid for
**Found by:** the `0.2.0` closeout session's baseline gate run, on the first
command it ran

## What happened

`cargo xtask ci` was red on `main` and had been since the 2026-09-07 KB intake
wave. One line:

```
.kb/decisions/0058-the-sqlite-write-path-stays-inline-and-documents-it.md:33
  — cites `crates/happenstance-sqlite/src/event_store.rs:1229-1296`, line 1229 is blank
```

`HANDOVER.md:20` records the gate green at `a0a925b`, and it was. The wave
(`025f300`, merged at `6acdf24`) landed after that, and its merge is the last thing
on `main`. Nobody ran the gate between the merge and this session.

## Why the citation was wrong

It was **stale on arrival**, not drifted afterwards. The atom inherited all three
of its ranges from `.kb/_intake/remediation-2026-09-04-briefs/sqlite-blocking-seam.md`,
authored 2026-09-04 against a tree that the merge-join read path (`b3c8d84`,
merged `8c2bfa7`) subsequently moved.

`a4616ca` — *"fix(citations): repoint the fifteen the merge moved"* — repointed
what that merge moved in code and documentation. It touched **no** file under
`.kb/`, and correctly so: nothing there was in scope.

## The hole, stated mechanically

`xtask/src/lints.rs:2003`:

```rust
const CITATION_SCAN_EXCLUDE: [&str; 2] = [".kb/_governance", ".kb/_intake"];
```

The exclusion is deliberate and its stated reason is sound — `_intake` is staging
that `/redkiln:kb-ingest` clears, so holding it to a permanent standard would fail
the gate on documents that are about to stop existing.

**What follows from it was not intended.** `/redkiln:kb-ingest` promotes a staged
document's prose, citations included, from a directory nothing checks into
`.kb/decisions/`, which *is* checked. The ingest is therefore the exact moment at
which invisible drift becomes a red gate — and it is the moment nobody is looking
at line numbers, because the work in front of them is adjudication.

`REMEDIATION-HANDOVER.md` measured the exposure before it fired: roughly 1,200
citations under `.kb/_intake`, in a directory with no citation checker at all,
alongside ~2,700 under `references/` and ~25,700 under `.bklg/`.

## Decided at review: re-anchor at promotion

**The owner chose repair 2 below.** `/redkiln:kb-ingest` resolves every `path:line`
against `HEAD` as it authors an atom, and **refuses the wave** rather than the atom
when one does not resolve.

Two things follow that this document must not leave implicit.

**It is not a change to this repository.** The ingest workflow lives in the
redkiln plugin, not in `.redkiln/` — which holds config, the pinned process pack,
templates and telemetry, and no workflow. So the repair belongs on redkiln's own
backlog, which already carries a `fix-kb-ingest-defects` project, and nothing in
happenstance can implement it.

**Until it lands, this hole is open and the next wave can redden the gate the same
way.** The interim mitigation is not a code change but a habit: run `cargo xtask
ci` immediately after an ingest wave merges, rather than trusting the pre-merge
green. The wave that caused this one was the last commit on `main` and the gate
was not re-run after it.

The alternative the owner did **not** take was scanning `_intake` as a warning.
The argument against is not that it would fail to work — it would — but that it
puts the check on the wrong side of the boundary: a warning in *this* repository
about documents *that* tool is about to promote makes happenstance responsible for
noticing redkiln's defect, on every run, forever.

## Two repairs, and they are not alternatives

1. **Bring `_intake` into scope as a warning rather than a failure.** A staged
   document with a dead citation is not a reason to fail the gate; it *is* a reason
   to say so before an ingest wave promotes it. The exclusion's own argument only
   supports "not a failure", never "not reported".
2. **Re-anchor at promotion.** `/redkiln:kb-ingest` already rewrites a document as
   it authors an atom. Resolving every `path:line` against `HEAD` at that moment —
   and refusing the wave rather than the atom when one does not resolve — closes
   the hole at the only point where the two directories meet.

The second is the stronger of the two, because it fails the wave rather than the
next person to run the gate.

## The second finding, about the guard rather than the citation

Repairing the atom means editing a body that `CLAUDE.md` calls immutable. The
repair was taken on the repository's own rule — ***rewrite the referent, never the
reasoning***, which `CLAUDE.md` states for a renamed crate inside a standing
decision — and not one word of the reasoning moved.

What the attempt revealed is worth an atom of its own: **`redkiln validate --kb`'s
immutability check compares the working tree against `HEAD`.** It refuses an
*uncommitted* edit to an accepted atom and passes the moment that edit is
committed. It is a dirty-tree guard, and it cannot tell a referent repair from a
reversal.

That is materially weaker than the surrounding prose implies, and the gap matters
in both directions: it will not stop a real reversal that arrives as a commit, and
it obstructs a repair that reverses nothing. Whether the answer is a stronger check
(hash the accepted body into the atom's own frontmatter, so history is what is
compared) or a documented carve-out for referent-only edits is the open question.

## Superseding was considered and is worse

It would mark a correct, unreversed decision `superseded` — a lie about the
decision's status to every later reader — and it **would not fix the gate**, because
the citation lint reads every atom regardless of status. The broken one would stay
red beside its replacement.
