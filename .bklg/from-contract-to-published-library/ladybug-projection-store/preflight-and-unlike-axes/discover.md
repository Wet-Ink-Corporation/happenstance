---
item: HS-S0074
stage: discover
created: 2026-08-12T13:02:43.974Z
updated: 2026-08-12T13:02:43.974Z
template_sig: 86ce4036
rendered_sig: b0f63b59
---

# Discover — Preflight the merged port, then commit the "structurally unlike" axes

## Signal Ledger

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line — assert the merged port really shipped `type Batch;`, the write seam and `projection_store_conformance!`, then commit the dated "structurally unlike" axes to `references/evaluation/` as its own merge, before any body is filled in | `_storymap.md:40` | Two obligations in one story, and the order between them is the deliverable: preflight first, axes second, bodies never. |
| **AC-001** — the definition of "structurally unlike" is on disk before the first run, naming the axes on which the Ladybug batch differs from the shapes that froze the port (no transaction handle type; a deferred, owned, `Send + 'static` write set; a synchronous driver; single-writer concurrency), at an earlier commit than the first suite invocation | `project.md:180-184` | The AC is an **ordering claim about commits**, not a content claim about a document. The check is `git log`, and it cannot be satisfied retroactively. |
| **DR-3** — the axes must be committed before the first suite run so the definition cannot be chosen afterwards to match the result; the commit order is the evidence | `project.md:146-148` | This is the project's highest-integrity requirement. Everything else in the story exists to protect it. |
| **AC-A01** — the upstream preflight is asserted, not assumed; a red preflight **halts the project** and is not worked around locally | `_decomposition.md:33-41` | Three things are checked against the merged tree: `type Batch;` with no lifetime, the write seam, and `projection_store_conformance!` resolving from `happenstance-testkit`. None of the three exists today. |
| The port today declares `type Batch<'a> where Self: 'a`, and `projection_store_conformance!` returns nothing from a repo-wide search | `_grounding.md:39-59`; `crates/happenstance-core/src/projection.rs:97-99` | The signature the skeleton is written against is **stale by design**. The axes wording depends on what actually merged, which is why the preflight rides with the axes rather than preceding them as a separate story. |
| Two shapes of write seam are in play — PS-11's `ProjectionProbe` in the contract crate behind `feature = "conformance"`, versus HS-P0010's fixture-level `write_probe(&mut Batch)` plus an out-of-band `read_probe(&Store)` — and this project **must not guess between them** | `_decomposition.md:104-114` | "Read what shipped." If neither can express a replayable parameterised Cypher statement, that is the freeze not holding, and it belongs in the verdict rather than being routed around. |
| **M9** — `references/evaluation/` is dated, commit-pinned evidence that must be *superseded rather than edited*; the axes and the verdict are two files in two commits | `_decomposition.md:211-218`; `references/evaluation/README.md:1-13` | One file amended in place cannot make an ordering claim. The lifecycle rule is what makes the artefact evidence rather than notes. |
| Testing brief, AC-001 row — proven by commit order, not by content review | `_decomposition.md:471` | The reviewable content is *what the axes say*; the falsifiable claim is *when they were said*. Both are needed and they are checked differently. |
| Slice rationale — `preflight-and-decisions` is two stories and both are foundation; the preflight rides with the axes because the axes wording depends on what actually merged | `_storymap.md:53-59` | If `type Batch;` landed, "no transaction handle type" and "no lifetime" are different axes and must be written as such. |
| Merge-order gate 1 — the preflight is red-or-green, not negotiable; the `LiveHandleProjectionStore` (T1) incompatibility is surfaced upstream from here too, because the crate will otherwise not compile | `_storymap.md:127-132`; `_decomposition.md:314-331` | This story is the project's only halt point. It is also where a problem found in HS-P0010's shipped port gets raised, before any local workaround becomes tempting. |
| `depends_on`: none | manifest; `_storymap.md:40` | Nothing upstream inside this project. The only inbound edge is the cross-project one on `projection-store-freeze` (HS-P0010), which is what the preflight reads. |

## Questions

**What counts as "structurally unlike"? — answered here; this story owns the
definition.** It is written as a *contrast*, never as a description. Each axis
names (a) the property the Ladybug batch has, (b) the property every shape that
froze the port has instead, and (c) what a conformance rule would have to *do* to
tell them apart. The four candidate axes are the ones AC-001 already enumerates —
no transaction handle type in the driver at all; a deferred, owned,
`Send + 'static` write set where nothing executes until `commit`; a synchronous
driver behind an `async` port; single-writer concurrency in which two commits
racing is routine rather than exceptional (`crates/happenstance-ladybug/src/projection_store.rs:204-213`).
Two things are deferred to `spec` and only two: the exact wording, which depends
on the preflight's finding (if `type Batch;` landed, "no lifetime" replaces or
splits "no transaction handle type"), and whether a fifth axis is warranted for
the Cypher-versus-SQL mutation vocabulary. What is **not** deferred is the rule
that the axes are fixed before the run and superseded rather than edited
afterwards.

**How does `lbug`'s blocking API meet a non-blocking port (ADR-0025)? — deferred,
and deliberately not pre-empted here.** It is `adr-0025-three-answers`
(HS-S0075)'s Q3, framed at `_decomposition.md:293-310`. This story records the
synchronous driver as an *axis of unlikeness* — a fact about the shape — and says
nothing about the bridge. Writing the answer here would let the axes document
double as a design decision, and an axis that encodes a preferred remedy is no
longer a neutral yardstick for the verdict.

**What happens if the preflight is red? — answered.** The project halts and the
finding is surfaced to `projection-store-freeze` (HS-P0010). Specifically: if
`crates/happenstance-core/src/projection.rs` has not shipped `type Batch;`, or
the write seam is absent, or `projection_store_conformance!` does not resolve,
none of it is stubbed, vendored or worked around locally
(`_decomposition.md:33-41`; `_storymap.md:127-132`). T1's separate finding — that
`LiveHandleProjectionStore` binds `type Batch<'a> = GraphWriteHandle<'a>`
(`crates/happenstance-ladybug/src/live_handle.rs:177-180`) and has no
lifetime-free spelling — is raised in the same breath, because it is HS-P0010's
own "compiles with no change other than the lifetime parameter's removal" claim
being false for one of the two impls in this crate.

**Does the preflight belong in `spec` as an AC, or is it a precondition? —
answered: an AC.** It produces a written, dated finding naming the three
observations against a commit SHA of the merged tree. A precondition that leaves
no artefact cannot be shown to have been checked, and "we looked" is exactly the
class of claim this project exists to stop accepting.

**Deferred to `spec`:** where under `references/evaluation/` the file sits and
what it is called (the genre precedent is `phase-4-reconciliation.md` and
`phase-4-5-reconciliation.md`), and whether the preflight finding is a section of
the axes document or its own file. Either is compatible with AC-001 as long as
the axes land in a commit of their own, before any suite run.

## Decision

The project cannot be judged until it says what it is measuring, and it cannot be
trusted to say it afterwards — so this story fixes both ends before any code
moves: it reads the merged `ProjectionStore` port and records what actually
shipped, and it commits a dated statement of the axes on which the Ladybug batch
is structurally unlike every shape that froze the port. The spec will cover the
preflight's three observations as a red-or-green finding with a commit SHA and an
explicit halt-and-escalate path to HS-P0010 (including T1's
`LiveHandleProjectionStore` incompatibility), and the axes document as a
contrastive table — one row per axis, each naming the Ladybug property, the
property the freezing shapes had instead, and what a rule would have to do to
distinguish them — landed in `references/evaluation/` in a commit of its own that
precedes every commit touching a `projection_store_conformance!` invocation
against a Ladybug fixture. Nothing `[FROZEN]` is touched: PS-2
(`spec/SPECIFICATION.md:4760-4774`) is read and cited as the bar the axes are
written against, and if the port that merged contradicts what this story expects,
the response is a finding raised upstream, never a local edit to a clause.

## The wrong implementation

**The axes document written as a description of Ladybug rather than a contrast
against what froze the port.** A file dated correctly, committed in its own merge,
ahead of every suite run, saying "Ladybug is an embedded property-graph database
with a Cypher interface and no SQL." It satisfies AC-001's ordering claim
perfectly, passes every content skim, and names no axis that a conformance rule
could be *blind to* — so any verdict written against it is unfalsifiable, because
nothing in it could ever come out the other way. The tell is that no row answers
"what would a rule have to do to notice this?"; the fix is that every row must,
and a row that cannot is not an axis.

**The mutant this document exists to make detectable, named here because it is
unrecognisable later: a "graph" batch that is a SQL batch wearing different type
names.** A `GraphWriteSet` whose statements are all
`MERGE (r:Row {k: $k}) SET r.v = $v` — one label, scalar properties, no
relationships, no traversal — is a key-value row store with a Cypher accent. It
will pass the projection suite, satisfy AC-004's rule count, and re-test the
freeze against precisely the shape that froze it, producing a verdict worth
nothing. Downstream, `ladybug-fixture-and-conformance-run` (HS-S0078) is where the
negative control for it lives, in `crates/happenstance-ladybug/tests/` rather than
the testkit's, because `crates/happenstance-testkit/**` is explicitly not this
project's to edit (`_decomposition.md:86-88`) and an adapter-specific mutant does
not belong in a shared suite. But it is only *detectable* if this story writes the
axis it violates — "the batch expresses relationships and traversal, not rows" —
into the document first. That is the dependency: the mutant is caught by a
document, not by a test.

**The ordering mutant: axes and first run in the same merge.** The document exists,
is dated, names good axes, and lands in the same PR (or is amended in place after
the run "for clarity"). Every content review passes; the one claim AC-001 makes is
gone. The guard is structural rather than a matter of care — `references/evaluation/`
documents are superseded, never edited (`references/evaluation/README.md:1-13`), and
the verdict in `freeze-verdict-document` (HS-S0082) must cite the axes commit SHA
alongside its own, so an inverted order is visible in the artefact rather than only
in `git log`.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
