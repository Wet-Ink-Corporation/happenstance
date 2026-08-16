---
item: "HS-S0019"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — ADR-0021 — payload evolution and the codec tag's home

> **STATUS: seven of eight.** AC-001 … AC-007 are satisfied by the two artefacts
> this story delivers. **AC-008 is not**, and cannot be from inside this PR: it
> asserts an accepted `.kb/decisions/0021-payload-evolution-and-codec-tag.md`, which
> only a **human-invoked `/redkiln:kb-ingest` wave** may author. This story's spec
> anticipates that outcome and names it correct (`spec.md`, *Clarifications resolved
> during spec*, item 4).

**This story differs structurally from its slice-mate, and that changed the work.**
`adr-0020-fold-query-agreement` *records* a decision a human took at the design
gate. This one **takes** three, because `_design.md:674-679` hands them over in
terms — the public surface is invariant under all three, so the design did not wait
and no later stage will take them if this one does not. M3
(`codec-and-feature-forwarding`) *consumes* the siting; it does not choose it.

**The three answers, and the rule they imply:**

1. **The codec tag lives in `Event::metadata`**, inside a versioned framing region
   the typed layer owns and no store parses.
2. **`EventType` carries no version suffix.** An event type is a stable identity;
   the payload evolves.
3. **No read-path hook is needed**, earned by naming **decode-time tolerance** as
   the strategy and an out-of-event upcast as the falsifier.
4. **An event with no framing region decodes with the codec in hand**, not
   `CodecError::UnknownTag`.

## TDD Evidence

The deliverable is two markdown artefacts, so the tests that encode the ACs are the
existence-and-reachability checks the spec's merge-gate table names, plus the
mechanical negatives that AC-003, AC-005 and NF-001 turn on. Each was run **before**
the change and observed to fail for the right reason — the artefact absent, not a
typo or a broken command — and again after.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `test -f .kb/_intake/0021-payload-evolution-and-codec-tag.md` | **Red** `FAIL (missing)`. **Green** `PASS` at 254 lines, composed as a decision record with the proposed frontmatter at `:24-88` |
| AC-001 | `cargo xtask affected --base main` (`.redkiln/config.yaml`, `affected_gate`) | **Green** — `affected gate passed`. **Correction to the spec's expectation of this step:** `--base main` diffs the *whole initiative branch* (923 files, seven packages named), not this story's checkpoint, so it is not the "maps to no package" proof the gate table predicts. It is the configured story grain and it is green; the story-scoped negative is the `git diff --name-only` row below |
| AC-002 | Review against `_decomposition.md:548-570` and `_design.md:674-679`; one-home claim against the contract crate's own words | **Red** — no record, and the design explicitly declines to answer. **Green** — one home, its cost discharged with five framing rules and a stated price |
| AC-003 | Re-open `spec/SPECIFICATION.md:629-646` and confirm **both** halves of VT-3 are used | **Red** — nothing to check. **Green** — the second half is named as the deciding half, via the prior question *does anything below the port need to see the codec tag?*, and the clause's own `Rejects:` line is handled as the mirror case rather than ignored |
| AC-003, AC-005 | `git diff --name-only` shows no path under `crates/`, `examples/`, `xtask/`, `spec/` | **Green.** This is the *whole* mechanical enforcement available for "VT-3 obeyed, `EventStore` untouched" |
| AC-004 | Review with `examples/course-subscriptions/src/main.rs:114-125` open | **Red** — the question was open. **Green** — "no suffix", with the query is quoted verbatim and the consequence traced to `main.rs:158-160` |
| AC-006 | The untagged-event rule readable as one implementable sentence; read back against `codec-and-feature-forwarding/discover.md:41-51` | **Red** — M3's discovery lists the tag's home as an input it does not have. **Green** — a blockquoted rule at `references/adr/0021-…:275-278`, plus an explicit *not* `UnknownTag` |
| AC-007 | `test -f references/adr/0021-payload-evolution-and-codec-tag.md` | **Red** `FAIL (missing)`. **Green** `PASS` at 454 lines |
| AC-007 | proposed `source_paths` resolves and lists both artefacts | **Green** — six entries at `.kb/_intake/0021-…:82-88`, each re-checked to exist |
| — | `redkiln validate --kb` | **Green** before and after: `redkiln: validate passed.` No atom was added by this PR, which is the state AC-008 measures the wave against |
| AC-008 | `git log --diff-filter=A -- .kb/decisions/0021-payload-evolution-and-codec-tag.md` | **Red, and still red.** No such path. Discharged by the wave, not by this PR |

**What a green gate proves here.** Nothing compiled changed, so every compiled step
is a negative proof. That no VT-3 amendment and no `EventStore` edit is in the diff
is mechanically true; that the *decisions* are the right ones has no compiled
assertion and never will — *"Record, not a test"* (`_decomposition.md:790`).

## Commits

One story checkpoint. Its own sha cannot be written into a file the commit
contains; it is recorded on the item through `redkiln record-links`.

| SHA | Subject |
| --- | ------- |
| `c011143` | `feat(typed-layer-and-alpha-release): ADR-0020: fold/query agreement` — the slice-mate, and this checkpoint's parent |
| *on the item's `links.commits`* | `feat(typed-layer-and-alpha-release): ADR-0021: payload evolution and codec tag` |
| *pending* | the `/redkiln:kb-ingest` wave that mints both M1 atoms — AC-008's evidence for this story and for its slice-mate, and a human's to run |

## Changes

| Path | Shape of the change |
| --- | --- |
| `references/adr/0021-payload-evolution-and-codec-tag.md` | **New, 454 lines.** The long-form record: `RUNBOOK.md:301`'s four sub-questions and the three-plus-one answers; VT-3 read in both directions with its `Rejects:` line handled as the mirror case; a six-row falsifier table over both candidate homes; the migration row expanded into the silent boundary-shape change; the accepted cost of `Event::metadata` discharged as a two-region split with five numbered rules and a stated price; the version-suffix answer traced line by line through the worked example; the hook answer with its strategy, its falsifier and its AC-012 route; the untagged-event rule; a ten-row rejected table; the ADR-0016 seam in one sentence; the `serde`-direction paragraph; consequences with a `low` reversibility argued rather than asserted; and one citation correction |
| `.kb/_intake/0021-payload-evolution-and-codec-tag.md` | **New, 254 lines, staged.** The wave's input: the op, a proposed frontmatter block (`adr_id: ADR-0021`, `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 7`, `reversibility: low`, `depends_on`/`related` naming five real atom ids including `kb-decision-0016`, six resolving `source_paths`), the problem, VT-3 in both directions, the four claims, the falsifier table, the accepted cost, ten rejected alternatives, the ADR-0016 seam, the `serde` direction, the consequences, the citation correction, the open-questions statement, the map rows, and an explicit *Not proposed* list |
| `.bklg/.../adr-0021-payload-evolution-and-codec-tag/_ledger.md` | Seven rows flipped `false → true` with cited evidence. **AC-008 left `false`.** No criterion, `mount_point` or `verifying_test` touched |
| `.bklg/.../adr-0021-payload-evolution-and-codec-tag/implementation-report.md`, `report.md` | **New.** This report and the story's findings ledger |

**Not touched, and asserted by the gate:** `crates/**`, `examples/**`, `xtask/**`,
`spec/SPECIFICATION.md` (VT-3 and every `ES-*` clause are byte-identical),
`.kb/decisions/**`, `.kb/maps/**`, `.kb/open-questions/**` — including
`human-readable-payload-encoding-on-a-constrained-peer.md`, which is cited as
`related` and left exactly as it is (EC-009).

## Gates

| Command | Result |
| --- | --- |
| `test -f` × 2 (red baseline) | both `FAIL (missing)` before the change |
| `test -f` × 2 (after) | both `PASS` |
| `cargo xtask affected --base main` | **passed** — `affected gate passed` |
| `redkiln validate --kb` | **passed** — `redkiln: validate passed.`, before and after |
| `redkiln doctor` | unchanged: the **six** expected `template-drift` advisories and the pre-existing foundation-story advisories, none of them this story's |
| `git diff --name-only` | only `.kb/_intake/0021-…`, `references/adr/0021-…` and this story's own `.bklg` folder |
| `cargo xtask ci --fast` | the project's declared integration bar, run once for the slice after both stories — see *Gates* in the slice digest. Green **and unchanged**, because nothing compiled moved (NF-001) |

## Notes

**The three decisions, and why each came out the way it did.** These are the
deviations that matter, because nothing upstream decided them.

1. **`Event::metadata` over `Tags`, on the falsifier and not on taste.** The
   discriminator turned out to be VT-3's *second* half, not its first. *"Any value
   that a store, a peer, a conformance rule or a query must be able to see MUST be
   carried in the `EventType` or in `Tags`"* (`spec/SPECIFICATION.md:632-633`) makes
   the siting turn on a prior question — *does anything below the port need to see
   the codec tag?* — and the answer is no, because decoding is strictly above the
   port (ADR-0007) and `QueryItem::matches` reads `event_type` and `tags` only
   (`crates/happenstance-core/src/query.rs:112-115`). Had the answer been yes, VT-3
   would have **required** `Tags` and the binding constraint would already have been
   violated. The clause's `Rejects:` line — origin identity in `metadata` — is the
   mirror case, and is handled rather than stepped around.
2. **The accepted cost is discharged as a rule, not a hope.** `metadata` is split
   into a versioned framing region owned by `happenstance` and everything after it,
   owned by the application and carried through unparsed. Five rules bound it, and
   the two that will bite an implementer are: the framing region must be readable
   *without* knowing the payload codec (it is how the codec is discovered), and it
   must add no dependency, so it is not `serde`-encoded and not JSON — a
   `postcard`-only build must read a `json`-only build's tag. **The exact bytes are
   deliberately not decided here**: the PR boundary assigns *"the tag's on-the-wire
   encoding"* to M3, so this record sites and bounds, and M3 formats.
3. **"No hook" was made falsifiable rather than merely asserted.** Decode-time
   tolerance sees one event's bytes; if an upcast ever needs information from
   *outside* the event being decoded, this answer is wrong, and the route is an
   AC-012 defect entry with a clause ID — not a hook. Naming the shape that would
   break it is what separates this from the third mutant (`discover.md:121-128`).

**AC-006's rule chose the fallback over the refusal, and the reason is mechanical
rather than aesthetic.** `CodecError::UnknownTag` for untagged events would make
every event written before the typed layer existed unreadable, including the ones
`examples/course-subscriptions` writes today — and there is no legal repair, because
no adapter may rewrite stored events (VT-3) and no backfill may run through a hook
`EventStore` does not have. Refusal is not the strict-and-safe option here; it is
the unimplementable one. The fallback also turns out to be load-bearing for
reversibility: it is what makes any future re-siting *additive*.

**`reversibility: low`, and it is not inherited.** NF-006 warns against defaulting
to `0016`'s `high`, and the warning is right: `0016` is `high` because its wire
format is private and unpublished. This decision is inherited by `0.2.0-alpha.1` and
by every event a user writes with it, and re-siting a tag on stored events is
exactly what VT-3 forbids an adapter doing on their behalf. The only legal reversal
is application-level re-emission, which mints new identities (VT-5) and leaves the
old events where they are, forever.

**One citation checked and found imprecise.** `_design.md`, `_decomposition.md` and
this story's spec all describe `EventStore` as offering *"`read`, `append`, `head`
and `read_decision_model`"* at `crates/happenstance-core/src/store.rs:110-159`. The
trait is at `:93-269` and requires `read` `:119`, `append` `:213`, `head` `:248` and
`contains_event_id` `:268`; `read_decision_model` is a **free function** at `:321`.
AC-005's conclusion is unaffected — neither spelling contains a hook — and the
record uses the correct enumeration and says so. A **repair** by
`.kb/decisions/README.md`'s mechanical test, not an amendment.

**Scope held.** No sentence of this record decides ADR-0020's subject (EC-008), and
none resolves or edits `kb-open-question-human-readable-encoding-limits-001`
(EC-009) — it is cited as `related` to mark the seam and nothing more. The `serde`
paragraph is written in the ADR-0003-constrains-`happenstance-core` direction
(EC-004, NF-004), which is the one thing here that is unfixable after the wave.
