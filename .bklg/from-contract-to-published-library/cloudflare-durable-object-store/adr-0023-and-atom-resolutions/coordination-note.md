# CF-40 coordination note, and the wave's proposed action plan

This is AC-001's artefact and the staged handoff AC-002 requires. It was written
**before a single intake document**, which is the order AC-001 fixes: the forbidden
outcome is two independent mintings of CF-40's answer across HS-P0012 and HS-P0013,
and the only way to avoid it is to look first.

---

## Part 1 — CF-40: the branch, and the evidence for it

**Branch B. No resolution exists, so this story stages one and `sqlite-durable-store`
(HS-P0012) cites it.**

### What was run, at implement time, and what it returned

```console
$ ls .kb/decisions/
0001-async-port-flavours.md            0017-what-a-projection-batch-owns.md
0002-crate-naming.md                   0018-returning-a-projection-to-never-run.md
0003-opaque-payloads.md                0019-what-happens-when-apply-fails.md
0004-edition-and-msrv.md               0020-fold-query-agreement.md
0005-rename-to-happenstance.md         0021-payload-evolution-and-codec-tag.md
0006-bare-name-to-the-typed-layer.md   0022-append-condition-strategy.md
0007-projection-runner-decodes.md      0029-msrv-raised-to-1-97-1.md
0008-one-derivation-for-both-ports.md  0030-the-checkpoint-reports-the-commits-that-happened.md
0009-error-send-sync.md                0031-the-runner-collapses-upward.md
0010-the-suite-must-prove-itself.md    0032-adr-0021-serde-attribution-correction.md
0011-read-laziness-and-isolation.md    0033-happenstance-macros-out-of-scope-for-0-1.md
0012-append-shape-and-preconditions.md README.md
0013-position-assignment-and-visibility.md
0014-event-identity-and-recorded-time.md
0015-validated-identifiers-and-store-limits.md
0016-the-wire-format.md
```

No `0023-*`. No atom minting CF-40's resolution.

```console
$ rg -n "CF-40" .kb/decisions .kb/maps
.kb/decisions/0015-validated-identifiers-and-store-limits.md:35:  CF-40 is minted so a fixture can declare its store's actual numeric limits as Option<usize>
.kb/decisions/0015-validated-identifiers-and-store-limits.md:103:existing fixture contract could do — so CF-40 is minted, letting a `Fixture` declare
.kb/decisions/0022-append-condition-strategy.md:30:  lifted, and CF-40's clause home stays open. rusqlite without a pool is ratified rather than
.kb/decisions/0022-append-condition-strategy.md:93:ownership, do not supply the two-build evidence ADR-0012's falsifier requires. CF-40's clause home —
.kb/maps/open-questions-index.md:172:  disclaims ownership of CF-40 in its own text; ADR-0012 is the other
```

### The reading

**HS-P0012 merged and deliberately did not mint it.** ADR-0022 is phase 8's — the
`sqlite-durable-store` project's own decision — and it records CF-40 as a
*non-verdict with a named owner*, twice:

> Two subjects are recorded as non-verdicts with named owners rather than settled
> here. … **CF-40's clause home — which document owns a fixture-constant clause —
> stays open at `kb-open-question-cf-40-ownership-001`.**
> — `.kb/decisions/0022-append-condition-strategy.md:93-94`

and in its own frontmatter summary: *"CF-40's clause home stays open"* (`:30`).

`.kb/maps/open-questions-index.md:171-173` still carries the bullet as **Open**.

So there is nothing to cite, and Branch A does not apply. **This story stages the
resolution; HS-P0012 cites it** — which is also the direction ADR-0022 already
points, since it named the open question by id rather than answering it.

### What this project newly knows, and why it can answer sub-question 2

The atom's sub-question 2 is the substantive one — *does the fixture contract have
one owning document, or is it amended piecemeal by whichever ADR needs the next
capability?* Phase 9 supplies the **third** data point and it is the one that turns
an argument into an observation:

| decision | what it did to the fixture contract | did it claim ownership? |
| --- | --- | --- |
| ADR-0015 | minted CF-40 — three numeric `Option<usize>` ceilings | claimed **and** disclaimed, in the same document |
| ADR-0012 | owns CF-39 and `MID_BATCH_FAULT` in the same neighbourhood | by adjacency, never asserted |
| ADR-0022 (phase 8) | needed the *capability* to state numeric limits | **explicitly declined** |
| **ADR-0023 (phase 9)** | first fixture to declare **all three** ceilings *and* claim CF-39 | declines, and records why the pattern works |

`CloudflareFixture` is the first fixture in the workspace to declare all of
`MAX_EVENT_DATA_LEN = 1 MiB`, `MAX_TAGS_PER_EVENT = 1024` and
`MAX_EVENTS_PER_BATCH = 1024` *and* claim `MID_BATCH_FAULT` with a real SQLite
trigger inside the store's own write path. Before it, every fixture in the tree
left all three constants at `None`, so `append_reports_exceeded_store_limits`
reported a skip everywhere and certified nothing.

The proposed resolution is staged in
`.kb/_intake/cf-40-fixture-contract-ownership-resolution.md`. It is a **new atom**;
neither ADR-0015 nor ADR-0012 nor ADR-0022 is edited, because all three are
accepted.

### Diff check

`git diff --name-only main -- .kb/decisions/` for this story is **empty**: nothing
was typed into `.kb/decisions/` by hand, which is AC-002's whole subject and the
failure `CLAUDE.md` records as reverted at `0269720`.

---

## Part 2 — the proposed action plan for the wave

The ingest is a human's command — `/redkiln:kb-ingest` carries
`disable-model-invocation: true` and creates its **own** dedicated worktree with a
human gate before the workflow runs. This section is the plan that gate is asked to
approve, settled now rather than improvised there.

### The intake set

Five documents will be in `.kb/_intake/` when the wave runs. **Four are this
story's**; the fifth is already tracked and predates it.

| file | claim cluster | intended outcome |
| --- | --- | --- |
| `0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` | ADR-0023 proper | **new** decision atom `.kb/decisions/0023-*.md`, linking `references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md` |
| `es-6-verdict-against-adr-0009s-prediction.md` | the ES-6 verdict | folded into ADR-0023 (preferred) or its own `.kb/reference/` note — **never** merged into `.kb/decisions/0009-error-send-sync.md` |
| `cf-40-fixture-contract-ownership-resolution.md` | CF-40's resolution | **new** atom; `.kb/open-questions/cf-40-fixture-limits-ownership.md` flips `status` and gains `related`, body untouched |
| `wf-11-human-readable-encoding-measured-on-this-runtime.md` | WF-11's resolution | **new** atom; `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` flips `status` and gains `related`, body untouched |
| `0034-what-the-phase-8-reconciliation-cost.md` | phase-8 reconciliation evidence | **not this story's.** Tracked since `4ad58d0` and still in `_intake`, which by the directory's own contract means no wave has ingested it. It will be swept up by the same run; that is correct and should be expected rather than treated as a surprise. |

`.kb/_intake/README.md` is **dropped at the approval gate**. The default glob picks
it up and it is a README, not an atom. It stays on disk afterwards.

### The wave id

Suffix it. `.kb/_governance/integration-waves/` already holds `2026-08-10-intake`
and `2026-08-10-intake-2`; an overwritten wave directory destroys an earlier wave's
audit trail and is not recoverable from the atoms it produced. Proposed:
**`2026-08-19-intake-phase-9`**.

### Two refusals the gate must be ready to make

1. **Merging any ADR-0023 claim into an existing accepted decision atom, or
   amending one.** The only two legal shapes for decision content are a new atom or
   a superseding atom. Amendment is legal for non-decision atoms; for an accepted
   decision it is not, whatever the adjudicator scores. The atoms most at risk are
   ADR-0009 (the ES-6 cluster reads like a clarification of it and is not) and
   ADR-0015 (the CF-40 cluster reads like a correction of its hedge and is not).
2. **Reading the ES-6 cluster as resolving
   `.kb/open-questions/es-6-names-an-unwritable-rule.md`.** It does not.
   `store_error_crosses_a_join_handle` is still unwritten and unowned; that question
   stays open and its document says so.

### After the wave — the verification that is part of this story, not a follow-up

- `git log --oneline -- .kb/decisions/` shows `0023-*` introduced by the **wave's**
  commit, never by a hand-edit commit.
- `.kb/_governance/integration-waves/2026-08-19-intake-phase-9/03-integration-summary.md`
  exists and lists the atom; `main`'s two wave directories are intact.
- `ls .kb/decisions/0023-*.md` returns exactly one path;
  `rg -n "0023" .kb/maps/decision-map.md` returns its row.
- `git diff main -- .kb/decisions/0001-async-port-flavours.md` and
  `… 0009-error-send-sync.md` both produce **no output**.
- `git diff main -- .kb/open-questions/cf-40-fixture-limits-ownership.md` and
  `… human-readable-payload-encoding-on-a-constrained-peer.md` show **frontmatter
  hunks only**; `git diff --diff-filter=D --name-only main -- .kb/open-questions/`
  is empty; both index bullets are still present and annotated.
- `ls .kb/_intake/` returns `README.md` and nothing else.
- `redkiln validate --kb`; `redkiln doctor` reporting **exactly six**
  `template-drift` advisories and no seventh; `cargo xtask spec-trace`;
  `cargo xtask affected --base main`. **`redkiln adopt --templates` is never run.**

### The last two edits, which belong after the wave and not before it

`RUNBOOK.md:302`'s ADR queue row is struck through and rewritten in the shape
`RUNBOOK.md:295` uses for ADR-0016 — pointing at the atom **by path** and stating in
one sentence what it settled — and phase 9's ADR-0023 work box is ticked.

**Neither is done in this commit, deliberately.** Striking the queue row before the
atom exists would point a reader at a path that is not there and would claim an
acceptance that has not happened, which is the same inversion the phase's own
proof-artefact section was rewritten to prevent: a box ticked because the work is
"done" rather than because the artefact is there.
