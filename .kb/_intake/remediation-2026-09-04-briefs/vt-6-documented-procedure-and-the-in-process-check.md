# Does an adapter taking VT-6's documented-procedure branch owe an in-process incarnation check as part of the permission?

Record: **R-3-vt6-documented-procedure**. Source:
`references/evaluation/review-pre-publication-2026-09-03.md:1453-1482`. Repository read in
the `lane/sqlite-ceilings` worktree at `bd11598`.

**This brief did not get the author → two-critic → revision pass the original thirteen
had.** It was written by the lane implementing R-3, in the same session as the change it
describes. Read it with that discount applied.

---

## Why this is owed

VT-6 `[PROVISIONAL]` (`spec/SPECIFICATION.md:813-819`) permits mint-once

> **only if** it can detect that its state was restored or cloned, **or** the deployment
> is documented to invoke the re-mint

(`spec/SPECIFICATION.md:859-863`). `happenstance-sqlite` takes the **second** branch, and
its `remint_identity` doc is the documented procedure the permission rests on. That
procedure carried a precondition nothing enforced — *"Run it with nothing else holding the
database open"* — and VT-6's own `Rejects:` paragraph names what a violation costs:
*"the one failure mode in the replication design with no error path and no observable
symptom"* (`spec/SPECIFICATION.md:836-837`).

The lane closed the **narrow** case: an append through a handle whose file has been
re-minted is now refused with `SqliteEventStoreError::IdentityMoved`, checked inside the
append transaction against the persisted row. That is adapter-local, additive, free while
unpublished, and it needs no clause.

**The question it raises is not adapter-local.** VT-6 offers two branches and says nothing
about what the second one *costs*. If a documented procedure with an enforceable
precondition is now expected to enforce it, that is a sentence VT-6 does not have, and
every adapter taking that branch acquires an obligation it was not told about. If it is
not expected, `happenstance-sqlite` has done more than the clause asks and the next
adapter will reasonably do less.

## What is true today, after this lane

- `append` re-reads the file's incarnation inside `BEGIN IMMEDIATE` and refuses if it has
  moved, naming both incarnations. One indexed read on a four-row `WITHOUT ROWID` table,
  under a lock the writer already holds.
- `read`, `head` and `contains_event_id` do **not** check. They mint nothing, so a stale
  handle reading a re-minted file returns events that are still true.
- Nothing checks the **cross-process** case at the moment of the re-mint itself, and
  nothing can: SQLite has no notion of "who else has this file open".
- `references/adapter-shapes.md` — which VT-6 requires every adapter to record its
  mechanism in — has not been updated. It is `references/`, and this lane was not
  permitted to edit it. **That is an outstanding item and it is named here rather than
  left.**

## Options

### Option A — leave VT-6 as it is; the check is one adapter's choice

**Cost.** The next adapter taking the documented-procedure branch —
`happenstance-neon`, `happenstance-postgres` and `happenstance-cloudflare` are all
candidates and none has run the suite — reads VT-6, sees no obligation, and ships without
the check. `happenstance-sqlite`'s behaviour then differs from theirs in a way no clause
explains, which is exactly the fork a specification exists to prevent.

**Buys.** No clause change, no marker moved, no adapter obliged to do work that on some
storages is genuinely impossible.

### Option B — widen VT-6: the documented-procedure branch owes a check where the platform can make one

The sentence would have to be conditional, because on some storages there is nothing to
check — a Durable Object's incarnation is not a row anyone can re-read cheaply — and a
MUST that some conformant adapter cannot satisfy is a MUST that gets waived.

**Cost.** Conditional MUSTs are hard to conformance-test, and CF-24's discipline is that a
clause without a rule is as much a problem as a rule without a clause. What rule would
check this? A fixture would have to re-mint underneath a live handle, which is an
operation no port method exposes.

**Buys.** The fork above cannot open.

### Option C — a capability rather than a clause

Say nothing normative; add an entry to `references/adapter-shapes.md`'s per-adapter record
— *does this adapter detect an incarnation moving under a live handle?* — and let the
answer be a fact each adapter states rather than a bar each must clear. That is what VT-6
already asks adapter-shapes to hold.

**Cost.** Nothing enforces a fact stated in a reference document, which is this
repository's own recurring complaint about prose.

**Buys.** It records the axis without pretending it is checkable, and it is the one option
that costs nothing to reverse.

## Recommendation

**Option C now, and Option B only if a second adapter reaches the same branch and answers
differently.**

VT-6's marker is `[PROVISIONAL]` and its named falsifier is about Durable Object eviction
rather than about this, so its next review already has an owner and an agenda; this
question belongs on that agenda rather than ahead of it. Meanwhile the axis is real and
undocumented, and `adapter-shapes.md` is where VT-6 already sends adapters to record their
mechanism — so recording it there costs one row and forecloses nothing.

**The strongest argument against, stated in its own words.** *Option C is how a fork
starts. `happenstance-sqlite` now refuses an append that every other adapter in this
workspace would accept, and the only place that difference is written down is a reference
document nothing reads and no gate checks. A behavioural difference between conformant
adapters is precisely what CLAUDE.md means by "a port is only as well-designed as the
spread of what implements it" — and here the spread has produced a disagreement, which is
evidence for a clause rather than for a note.* That is the better argument if a second
adapter has actually taken the branch. **None has**: no adapter besides
`happenstance-sqlite` has run the event-store conformance suite at all, so there is no
disagreement yet — only one adapter that did more than it was asked. Minting a clause
from one implementor is the thing ADR-0022 §10 refused to do for the query decomposition,
one file over, for the same reason.

## Cost of delay

Low, and it does not compound. The check is landed and the behaviour is the safe one; what
is undecided is whether it is required. The cost is paid by the second adapter to reach
this branch, whenever that is.

## What this does not settle

- **`references/adapter-shapes.md` is not updated.** VT-6 requires every adapter to record
  its mechanism there, and this lane could not write to `references/`. Whoever applies
  Option C owns that row.
- Whether `read`, `head` and `contains_event_id` should check too. They mint nothing, so
  the argument for them is weaker; but a caller that reads through a stale handle and then
  appends through a *fresh* one has derived its condition's boundary from a store it no
  longer speaks for, and nobody has thought that through.
- What a **cross-process** re-mint should do at the moment it happens. It is undetectable
  from inside SQLite, VT-6 accepts that, and the check landed here catches it on the next
  append rather than at the re-mint — which is later than ideal and earlier than never.
