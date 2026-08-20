# CF-40 — the fixture contract has no single owner, and phase 9 is the third decision to amend it

**Staged for `/redkiln:kb-ingest`. Not an atom.** This resolves an **open
question**, and the shape of that is fixed by `.kb/open-questions/README.md`,
*Resolving one*: the answer is a **new atom**; the question's record **stays**;
the two are linked through `related`; the question's `status` moves to `withdrawn`
or `superseded`; and *"the body describing what was not known at the time"* is left
alone. The question is **never** `git rm`ed and is **never** rewritten into its own
answer.

**Target question atom:** `.kb/open-questions/cf-40-fixture-limits-ownership.md`
(`kb-open-question-cf-40-ownership-001`). After the wave its `git diff` against
`main` must show **frontmatter hunks only and zero body hunks**, and its bullet on
`.kb/maps/open-questions-index.md` must be **annotated in place, never removed**.

---

## Branch B — the coordination result, checked at implement time

`sqlite-durable-store` (HS-P0012) merges one position ahead of this project and
could have minted this answer first. **It did not, and it said so.** ADR-0022 records
CF-40 as a *non-verdict with a named owner*:

> Two subjects are recorded as non-verdicts with named owners rather than settled
> here. … **CF-40's clause home — which document owns a fixture-constant clause —
> stays open at `kb-open-question-cf-40-ownership-001`.**
> — `.kb/decisions/0022-append-condition-strategy.md:93-94`

and its frontmatter summary repeats it: *"CF-40's clause home stays open"*
(`:30`). `ls .kb/decisions/` shows no `0023-*` and no atom minting CF-40's
resolution; `.kb/maps/open-questions-index.md:171-173` still lists the question as
**Open**. So this story **stages the resolution** and HS-P0012 cites it. There is
exactly one minting across both projects.

## What this project newly knows, and why it answers sub-question 2

The atom's own sub-question 2 is the useful one: *"should the fixture contract
(`CF-` clauses) get a single named owner going forward … rather than being amended
by whichever ADR happens to need the next capability?"*

Phase 9 supplies the third data point, and it is the one that settles the shape:

- **ADR-0015** minted CF-40 (three numeric `Option<usize>` ceilings) and, in the
  same document, declined to choose whether it owned it.
- **ADR-0012** owns CF-39 and `MID_BATCH_FAULT` in the same neighbourhood, by
  adjacency rather than by counter-assertion.
- **ADR-0023** — this phase — is the **third** decision to amend the fixture
  contract without owning it. `CloudflareFixture` is the first fixture in the
  workspace to declare **all three** of CF-40's numeric ceilings *and* claim
  CF-39's `MID_BATCH_FAULT` (`crates/happenstance-cloudflare/tests/support/mod.rs`):
  `MAX_EVENT_DATA_LEN = 1 MiB`, `MAX_TAGS_PER_EVENT = 1024`,
  `MAX_EVENTS_PER_BATCH = 1024`, and a fault that is a real SQLite trigger inside
  the store's own write path. Before it, every fixture in the tree left all three
  constants at `None` and `append_reports_exceeded_store_limits` reported a skip
  everywhere and certified nothing.

So the pattern is now observed three times rather than argued about twice, and the
evidence points one way: **the fixture contract is amended piecemeal by whichever
decision needs the next capability, and that has worked** — three ADRs, no
collision, and each capability's *reason* recorded where the adapter that needed it
lives. What it costs is exactly what this question is: nobody can answer *which
document owns CF-40* without reading three ADRs, and a fourth capability arrives
with no default home.

## The proposed resolution, for the adjudicator to write into an atom

1. **CF-40 is ADR-0015's clause**, as its header and decision-8 prose assert; the
   hedge in its own Consequences section is superseded by use rather than by
   argument — two later decisions (ADR-0022, ADR-0023) both cite CF-40 to
   ADR-0015 and neither claims it.
2. **The fixture contract has no single owning document, and that is now a
   recorded position rather than an unanswered one.** A `CF-` clause is minted by
   the decision that first needs the capability, and it carries the reason there.
   What the position *costs* is stated with it: a reader locating a fixture-contract
   clause reads the specification's §6 by subject, not one ADR.
3. **Sub-question 3 stays open on its own terms.** Phase 10's `POLL_BUDGET`-shaped
   question (ADR-0013 §8) is the next test of (2) and is not settled here; if it
   collides, that is the evidence that would supersede this resolution.

**Neither ADR-0015 nor ADR-0012 nor ADR-0022 may be edited to record any of this.**
All three are accepted. The resolution is a new atom that cites them.
