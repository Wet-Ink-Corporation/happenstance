# Wave `2026-08-15-adr-0030-checkpoint-progress` — claims and classification

Every claim the extract pass raised, labelled against the accepted decision corpus, with the
destination it was routed to. **Eight claims in, five operations out** — three atoms created, two
existing `open_question` atoms resolved.

## What "against the accepted decision corpus" means this wave

Twenty accepted decision atoms exist (`.kb/decisions/0001`–`0019`, `0029`; every one
`status: accepted` except `kb-decision-0002`). The labels are the ones the previous three waves
used, with the same meanings:

| Label | Meaning here |
| --- | --- |
| `aligns` | Restates or applies a rule an accepted atom or a layer README already carries. Nothing new is committed; the claim cites rather than introduces. |
| `extends` | Net-new knowledge with no owner in `.kb/`, contradicting nothing accepted. |
| `conflicts` | Contradicts something accepted, or two sources contradict each other and the wave declines to pick a winner. **One occurrence — claim 6.** |
| `requires-new-decision` | Cannot be discharged by recording it: an ADR or a human sign-off is needed. **One occurrence — claim 6**, which is the same claim, and `02` Adjudication 3 explains why one claim carries both labels and why the wave still authors no decision for it. |

**This wave authors no decision it did not receive.** ADR-0030 was written, argued and accepted on
2026-08-15 with a 158-line record in `references/adr/`; the wave transcribes it into an atom,
exactly as the previous two waves transcribed twenty. The one act that *would* be authoring a
decision — writing the atom that supersedes ADR-0007 to correct its Context — is declined, and
declined into an atom rather than into silence.

## The claim table

### `.kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `claim-1` | One `decision` atom for ADR-0030, `phase: 6`, `reversibility: low`, `supersedes: null`. Mints **PS-38** `[PROVISIONAL]` in §4.7: a successful `commit` MUST advance `id`'s checkpoint to `position`, and a `ProjectionId` no successful `commit` has named MUST read as `Checkpoint::NeverRun`. Falsifier: a store answering `checkpoint` from a replica that may lag its own `commit`. Exposing implementation: a store whose backing state lives per **handle** rather than per store — the fixture bug CLAUDE.md exists to forbid — and it is `independent`. Four rejected alternatives, each with its reason | **MUST**, twice, in a new normative clause | `extends` | **Op 1** — `kb-decision-0030` |
| `claim-1a` | Both sentences are **one proposition** — *the checkpoint reports the commits that happened*; the first says a commit is visible in it, the second says nothing else is | atomicity argument | `aligns` | Op 1, `## Decision`. Carried as the reason the atom is not split (`00`, CL-1) |
| `claim-1b` | PS-1, PS-19, PS-21, PS-22 are **byte-identical** across the decision; each gains a recorded finding in the specification. §1.3's hand count moves 200 → 201 / 198 → 199 / 49 → 50 | negative scope + a spec edit already performed | `aligns` | Op 1, `## Consequences`. **No `spec/` edit is proposed by this wave** — the clause text already landed with the ADR |
| `claim-2` | `kb-open-question-ps-1-no-progress-obligation-001` **closes**, resolved by ADR-0030: the *"clause of its own"* candidate wins and the other two lose by name. The staged amendment stands — the obligation was **misfiled onto PS-23**, not absent | resolution event, no new modal | `extends` | **Op 2** — metadata flip + dated annotation, body verbatim |
| `claim-3` | `kb-open-question-ps-19-scope-narrower-001` **closes**, resolved by the same record. PS-19 keeps its post-reset scope; the unseen-id half becomes PS-38's second sentence. `fresh_projection_has_no_checkpoint` is listed against both clauses and stays unwritten (renders `†`). Sub-question 2's `isolated` answer stands and nothing reopens it | resolution event | `extends` | **Op 3** — same shape |
| `claim-4` | Both rows on `kb-map-open-questions-index-001` move Open → Resolved, each naming ADR-0030 | map maintenance | `aligns` | **No op of its own.** `mapsImpact.openQuestionIndex` on Ops 2 and 3; exact wording in `02` §What the Maps phase inherits. The label printed is `Superseded`, matching the atom's own `status` — `02`, Adjudication 7 |
| `claim-5` | PS-8 (S1, dependent), PS-13 (S2, dependent), PS-28 (S1, undetermined) and PS-29 (S1, independent) stay open, are recorded into the specification clauses they concern, and are **not ADR-0030's**. A future wave must not fold them into it | reports where findings already live | `aligns` | **No op.** Verified against `kb-decision-0017` and `kb-decision-0019`, both of which already state it; the scope boundary is one sentence inside Op 1's atom. `02`, Adjudication 2 |
| `claim-6` | PS-32's correction is **still owed and still not performed**: `references/adr/0007-…:37` says a callback-driven pump *"cannot be written against the port as it stands — in either crate"*; it can, and was compiled (PRESSURE-TEST §3.4). ADR-0007 is accepted and immutable, so the correction is a **superseding atom's** and never an edit. ADR-0017 records it as owed at `:356-361` | a documented factual error + a `must` about how it may be corrected | **`conflicts` / `requires-new-decision`** | **Op 4** — a new `open_question`. **No supersession is authored**; `02`, Adjudication 3 |
| `claim-7` | PS-3, PS-31 and PS-36 share one shape — a documentation obligation with no instrument — and the repository already carries the instrument that would close all three. *"Worth an atom; not opened by this story, which had no mandate to."* | explicitly deferred by the source | — | **No atom, no op.** Carried in `unresolved` so declining it stays visible — wave 3's convention for a declined candidate. `02`, Adjudication 4 |
| `claim-8` | `spec-trace`'s `has_suite` is a **per-family switch**, and a family absent from it is a family whose rule citations nothing checks. `PS-` was excluded when the projection suite did not exist and the exclusion outlived it by two slices, structurally invisible because the thing it disables is itself a check. Now held by `spec_trace::tests::the_projection_family_is_checked_against_its_suite`; `SY` still abstains, held by the test beside it | dated fact about an instrument | `extends` | **Op 5** — a new `reference` atom. `02`, Adjudication 5 |

## Conflicts

**One, and it is the wave's most consequential classification.**

| # | Claim | Why it is a conflict, and what the wave does with it |
| --- | --- | --- |
| 1 | `claim-6` — PS-32 | A sentence in an accepted decision's long-form record is **false**, and the falsification is by compilation rather than by argument (`PRESSURE-TEST.md:203-232`: the pump builds). `spec/SPECIFICATION.md:5679` states the correction as a `[FROZEN]` `MUST`, and `:5699` names the sentence in its *Rejects* field. Nothing in `.kb/` owns it. The resolution — a decision atom superseding ADR-0007 — is exactly what this wave has no mandate to author, so it is **routed to `open-questions/`** rather than forced, per the ingest rule and the layer's own first bullet. |

Two near-misses, both of which read as conflicts on first pass and are not:

| # | Near-miss | Why it is not a conflict |
| --- | --- | --- |
| 1 | ADR-0030 mints a `MUST` about `commit` while `kb-decision-0017`, `-0018` and `-0019` already own the projection port | PS-38 is a **new** clause in §4.7; PS-1, PS-19, PS-21 and PS-22 are byte-identical before and after, and the record says so at `:7-9`. No accepted decision says a commit need not advance anything — the whole finding is that **nothing said either way**. `extends`, and `depends_on` carries the edge |
| 2 | ADR-0030 answers two questions the 2026-08-13 wave had just amended rather than resolved | An `open_question` at `authority_tier: note` binds nothing, so answering one contradicts nothing. The 2026-08-13 amendments **stand and are cited** by the resolutions; both bodies are left verbatim, and the earlier annotations are what make the new one legible |

## Counts

| | |
| --- | --- |
| claims classified | **8** |
| `aligns` | 4 |
| `extends` | 4 |
| `conflicts` | **1** (also the one `requires-new-decision`; counted once above under its primary label) |
| accepted decision atoms **edited** | **0** |
| accepted decision atoms superseded (frontmatter flip) | **0** |
| decision atoms authored | **1** — transcribed from `references/adr/0030-…`, not newly decided |
| `open_question` atoms created | **1** |
| `open_question` atoms resolved (flip + annotation, body verbatim) | **2** |
| `reference` atoms created | **1** |
| existing atoms otherwise amended | **0** |
| map atoms inheriting work | **3**, via `mapsImpact` rather than ops |
| atoms out | **3 new + 2 resolved**, across 5 operations |
| claims producing no operation, deliberately | **3** (`claim-4` folded into `mapsImpact`; `claim-5`; `claim-7`) |
