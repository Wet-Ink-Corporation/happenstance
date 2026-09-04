# ES-23 is a `[FROZEN]` clause with two MUSTs on two different owners, and `FROZEN_DOC_MUSTS` records only one of them. Does the table carry a second disposition for a two-halved clause, or does ES-23's row become `Excluded`?

Short answer up front: **give the table a disposition for an obligation that
falls on an adapter, and make ES-23 the first row to carry two entries** — not
because the gate can then check the adapter half, which it largely cannot, but
because the array is read by people and it currently answers "yes, discharged"
to a question that has two answers and only one of them is yes. Held at
**medium** confidence, and the recommendation is a change to `xtask`, which this
lane was forbidden to make.

**This brief did not get the author → two-critic → revision pass the original
thirteen had.** It was written by the lane implementing `Q-02`, in the same
session as the change it describes. It carries its own strongest objection and
answers it, which is the form, but nobody independent argued the other side. Read
it with that discount applied.

---

## What happened, so the question is legible

`Q-02` of the pre-publication review
(`references/evaluation/review-pre-publication-2026-09-03.md:946-968`) found that
**ES-23** — `[FROZEN]` — carries two MUSTs:

> The port MUST document this in an explicit `# Cancellation` section, and each
> adapter MUST state which of the two it does.

The port half was discharged at `crates/happenstance-core/src/store.rs`. The
adapter half was discharged by neither shipping adapter, from phase 8 until this
lane. `grep -rn "# Cancellation"` over the tree returned exactly one source site,
and it was the port's.

This lane landed the statement in both adapters' **store-module** documentation —
beside the `append` it is about, with a pointer to it on each crate root — and a
test target per adapter that fails if the section goes missing, fails if the
pointer goes missing, and fails again if `append` acquires an `.await` and the
statement stops being true. That closes the finding.

**It does not close the reason the finding went unnoticed for two phases**, and
that is what this brief is about.

## What is true today, quoted

`xtask/src/lint_narrative.rs`'s `FROZEN_DOC_MUSTS` is the array that decides which
`[FROZEN]` documentation obligations the gate pins. Its derivation rule is
explicit:

> A candidate is **pinned** when (a) its clause is `[FROZEN]`, (b) its obligation
> falls on **the contract's own documentation** — not on an adapter's, not on a
> fixture's …

ES-23 appears in that table **once**, `Pinned` to the port site. By the rule
above that is correct: the port half falls on the contract's documentation and is
pinned; the adapter half falls on an adapter's and is out of scope by
construction. VT-21, VT-22 and VT-24 — whose obligations are wholly a store's —
are `Excluded`, with the reason written out.

So the array has three dispositions available to it and ES-23 uses one. A reader
scanning it for "which `[FROZEN]` documentation MUSTs are discharged?" reads one
`Pinned` row against a clause carrying two obligations, and the second one is
invisible in exactly the way an `Excluded` row is not. **`Excluded` is a
statement; a missing row is a silence**, and this is the case where a clause
produces both at once.

ADR-0012 records the ceiling on any check of this kind in its own words:

> The gate check proposed in §5 catches a missing `# Cancellation` section and
> cannot catch a section that lies.

and, at `references/adr/0012-append-shape-and-preconditions.md:349-364`, proposes
that very step, records that *"The step does not exist and has not been written"*,
and names the fallback: the per-adapter review at phases 8–11. **Both are
undischarged.** The step was never built; the phase 8 and phase 9 reviews did not
produce the statement. That is two named instruments failing on the same
obligation, which is what makes this a question about the instrument rather than
about one adapter's documentation.

## The question

Three options, and they differ in what a reader of the array is told.

### Option A — a third disposition, and ES-23 becomes the first two-row clause

Add a disposition meaning *this obligation falls on an adapter, here is where it
is discharged, and here is what checks it*. ES-23 gets a second entry naming the
two adapters' crate roots and their `tests/cancellation_statement.rs` targets.

- **Costs**: `xtask` work, and a row that has to be updated when a third adapter
  ships. The gate cannot verify the adapter half from `xtask` without reaching
  into adapter crates, which the derivation rule was written to avoid — so the
  entry would *point at* the per-crate test rather than perform the check.
- **Buys**: the array stops implying that a two-halved clause has one half. It is
  also the only option under which a future adapter author reading the table
  learns that they owe something.
- **Semver**: none. `xtask` is `publish = false`.

### Option B — ES-23's row becomes `Excluded`, as VT-21's is

Withdraw the `Pinned` row and record the whole clause as out of scope, with the
reason.

- **Buys**: minimal change; consistency with how the table already handles
  store-side obligations.
- **Costs**: it loses a check that currently works. The port half *is* pinned and
  *is* the contract's own documentation, and unpinning it to fix a bookkeeping
  problem trades a real instrument for tidiness. This option is listed to be
  visibly rejected.

### Option C — leave the table and document the limit beside it

Add a paragraph to the derivation rule saying that a clause may carry obligations
the table cannot see, and that a `Pinned` row therefore does not mean the clause
is fully discharged.

- **Buys**: cheapest, and it is honest.
- **Costs**: it is a comment about a silence, which is what the rule already is.
  Nothing changes for the reader scanning rows, and nothing points at where the
  adapter half went.

## Recommendation

**Option A**, at medium confidence, owned by whoever owns `xtask`'s narrative pin
(`.bklg/docs-that-teach/checked-documentation-surface/frozen-documentation-must-pin/`)
and sequenced before phase 12 rather than now.

The argument is that this table's purpose is to be the answer to *"which frozen
documentation obligations are met?"*, and it is currently a correct answer to a
narrower question that nobody asked. The cost of the narrower question was
measured rather than imagined: two phases, one audit finding, and two named
instruments — ADR-0012's proposed step and its own recorded fallback — that both
failed to notice a `[FROZEN]` MUST going undischarged on a crate about to
publish.

## The strongest argument against, in its own words

> *The table did exactly what it says it does, and the derivation rule that scopes
> it was written deliberately: a check in `xtask` that reaches into adapter crates
> is a check that grows a row per adapter and rots the first time one is renamed.
> ES-23's adapter half was not missed because the table was silent about it; it
> was missed because two per-adapter reviews did not do their job, and the fix for
> a review that did not happen is not a wider array. Adding a disposition that
> cannot check anything — that only points at a test in another crate — is how a
> gate acquires rows that are documentation wearing a gate's clothes, which is the
> decorative-check failure `xtask/src/main.rs`'s own module documentation warns
> about.*

That is a strong argument and it is why this is medium. Two concessions. First,
it is right that the entry would not *check* the adapter half — the checking is
now done, per adapter, by the two test targets this lane landed, and that is the
correct place for it. What the entry buys is that the array stops being readable
as a completeness claim it does not make; if the owner decides that is worth less
than a non-checking row costs, Option C is the honest fallback and this brief
would not argue hard against it. Second, the objection understates the review
failure by treating it as an accident: ADR-0012 named the per-adapter review as
the *fallback for a step it knew it was not building*, so the review was carrying
a load it was never resourced for, and "do the review properly" is the answer that
was already tried.

## Cost of delay

**Low for the clause, moderate for the pattern.** ES-23 is now discharged and
checked in both adapters, so nothing is undocumented today. What delay costs is
the third adapter: `happenstance-postgres` and `happenstance-neon` will each owe
the same statement when they stop being skeletons, and nothing in the tree will
tell their author so. The two test targets are the transferable part — a third
adapter can copy one — but only if someone knows to look.

## What this does not settle

- **Whether the adapter half should have a conformance rule instead.** It cannot
  have one as stated: no rule can observe an adapter's documentation. But a rule
  *could* observe the behaviour — poll `append` once, drop it, and assert what the
  store holds — and ES-22 already does exactly that. Whether ES-23's adapter
  obligation is better served by strengthening ES-22's rule than by a
  documentation check is a question for the rule set's owner, and it is adjacent
  to `F2-5`, which found that ES-22's second arm has never executed against
  anything.
- **Where the statement should live.** This lane put it in each adapter's **store
  module** — `src/event_store.rs` — with a pointer to it on each crate root. Two
  reasons, and the first was not a choice. `happenstance-sqlite`'s front page is
  held to a density budget by `tests/front_page.rs`: five `#` headings and 54–94
  `//!` lines, *"measured from the 76 it carried as an instrument"*. The section
  is a sixth heading and thirty-odd lines, so the crate root could not carry it,
  and the first cut of this change failed that test rather than discovering the
  constraint in review. The second reason is the better one: a statement about
  what `append` does belongs on the page `append` is on. The rejected third option
  was a doc comment on the `impl EventStore` method, which would have *replaced*
  the trait's inherited documentation on the impl page rather than adding to it.
  A crate that later grows a dedicated `# Cancellation` page in `docs/` may want
  it moved; the tests read the store module and the crate root, and would have to
  move with it.

- **Whether `happenstance-cloudflare`'s front page wants a budget of its own.**
  It has none, and it is by far the longest crate root in the workspace — twenty
  `#`/`##` headings. That is a question for whoever owns that page, and this lane
  noticed it only because the sibling crate's budget stopped an edit.
- **The wording of the statement itself.** Both adapters say the same thing
  because the answer is the same, and the two paragraphs were written together.
  A reviewer who thinks either overstates — in particular the Cloudflare crate's
  sentence about an eviction discarding the turn, which is a claim about the
  runtime rather than about this code — should say so; nothing in the tree checks
  it.
