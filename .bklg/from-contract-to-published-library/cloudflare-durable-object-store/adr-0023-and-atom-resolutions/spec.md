---
item: HS-S0058
stage: spec
created: 2026-08-12T13:46:56.946Z
updated: 2026-08-12T13:46:56.946Z
template_sig: 87bbf1d0
rendered_sig: 3f2ddff1
---

# Spec — ADR-0023 accepted, and CF-40 and WF-11 resolved rather than deleted

## Scope lock

| Slot | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project item | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/project.md` |
| This spec | `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/adr-0023-and-atom-resolutions/spec.md` |
| Key briefs | `.bklg/…/cloudflare-durable-object-store/_decomposition.md` (Architecture brief Notes §2 — the accepted atoms and the three tensions; §6 — ADR-0023's material and the CF-40/WF-11 latitude), `_grounding.md` §1, §3, §4, `_intake-brief.md` |
| Signed-off design | `.bklg/…/cloudflare-durable-object-store/_design.md` — **N/A determination, approved 2026-08-12.** No surface, no `## Items`, no `## Signatures`. This story renders nothing and claims no design item. |
| Story map row | `.bklg/…/cloudflare-durable-object-store/_storymap.md`, **Slices**, `adr-0023-and-atom-resolutions` (milestone `evidence-and-verdicts`) |
| This story's discovery | `.bklg/…/adr-0023-and-atom-resolutions/discover.md` — the signal ledger, the four questions and the two named wrong implementations |
| Roadmap pointer | `RUNBOOK.md:302` (ADR queue row 0023), `RUNBOOK.md:4241-4307` (phase 9 goal, work, proof artefact, exit criteria) |

## One-line PR slice

Stage `.kb/_intake/` and run the ingest path so ADR-0023 lands as an accepted atom stating the
`SqlStorage` mapping and the off-tokio harness as one question with the alternatives that lost, and
so CF-40's ownership and WF-11's atom move to *resolved rather than deleted*.

## Executive summary

This PR lands **no Rust**. It converts four findings the other stories in HS-P0013 produced into the
one form this repository treats as settled knowledge, through the one mechanism it permits.

The delta against the tree as it stands: `.kb/decisions/` gains `0023-*` — one atom, one question,
the `SqlStorage` mapping and the off-tokio harness together, with the alternatives that lost and why
each lost; `references/adr/` gains the long-form record the atom summarises; the ES-6 verdict is
recorded as a **new** record that judges ADR-0009's prediction rather than as an edit to it;
`.kb/open-questions/cf-40-fixture-limits-ownership.md` and
`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` each flip from open to
resolved with their bodies untouched; `.kb/maps/decision-map.md` and
`.kb/maps/open-questions-index.md` gain the rows that make all of it findable; and `RUNBOOK.md:302`'s
queue row is struck through in the shape `RUNBOOK.md:295` already uses for ADR-0016.

The thing this story is actually *for* is the discipline, not the prose. Every one of those writes
has an easier version that passes every automated check in the repository and destroys the record —
hand-writing the atom into `.kb/decisions/`, or "clarifying" ADR-0009 in place, or deleting a
question because it has been answered. `discover.md`'s **The wrong implementation** names them and
the repository has already paid for one of them once (`CLAUDE.md`, **Where the work lives**, commit
`0269720`). This spec's job is to make the right version the path of least resistance.

## Context pack

Read this section and you can start. Everything below the fold is behind **Anchors** (second pass).

**1. Atoms are minted by the ingest path, never by hand — and no check can tell the difference.**
`/redkiln:kb-ingest` consumes `.kb/_intake/*.md`, extracts the claims, adjudicates them against the
existing corpus (biased hard toward merging into an existing atom over spawning a near-duplicate,
and toward superseding an accepted decision over editing it), writes what survives, syncs the maps,
runs `redkiln validate --kb` and clears `_intake` (`.kb/_intake/README.md`). A hand-written atom
under `.kb/decisions/` passes `validate --kb` — validation checks *conformance and immutability*,
never *provenance* — and is missing the adjudication, the map sync and the wave's audit trail. That
is the mistake `CLAUDE.md` records as reverted at `0269720`. **The only guard is the workflow.**

**2. The ingest is a human's command, so this story is a staged handoff plus a verification, not a
write.** `/redkiln:kb-ingest` carries `disable-model-invocation`; the implementer authors the intake
documents and the proposed action plan, hands off at an explicit gate, and then verifies the tree
the wave produced. Two mechanics are settled now rather than improvised at the gate: **the wave id
carries a suffix** so it does not overwrite an earlier wave's audit trail under
`.kb/_governance/integration-waves/` (`2026-08-10-intake` and `2026-08-10-intake-2` are the
precedent), and **`.kb/_intake/README.md` is dropped at the approval gate** — the default glob picks
it up and it is a README, not an atom.

**3. ADR-0023 is one atom with an "and" in its title, and that is deliberate rather than sloppy.**
`.kb/playbooks/one-decision-per-adr-title.md` is this corpus's own finding that a conjunction in a
decision title is usually a strong decision carrying a weak one — and it states its own exception:
*"It stops being worth the split when both halves are settled by the same evidence, in which case
the conjunction is describing one decision with two consequences."* That is exactly the case here.
The `SqlStorage` mapping and the off-tokio harness are settled by **one** observation — the suite
executing under `workerd` against the real bindings — so they are one decision. Project AC-006
requires the single atom; the playbook supplies the reason it is not a violation. Say so in the
record, because the next reader will check.

**4. Three accepted atoms are in scope of this story's *citations* and none of them may be edited.**

- **ADR-0001 is not lifted here.** `RUNBOOK.md:4280-4282` asks for the `provisional` marker
  "formally retired and this adapter cited". The marker was already lifted at phase 1 by
  `LocalMemoryEventStore` and ADR-0008 records the lift. `.kb/decisions/0001-async-port-flavours.md`
  is accepted and immutable and `redkiln validate --kb` checks it against `HEAD`. What phase 9
  supplies is the **real-runtime evidence behind an already-accepted decision**, and the citation
  lands *in* ADR-0023 and the long-form record. The record must say why it read the runbook's
  instruction and did not obey it, or the next reader will try again.
- **ADR-0009 is judged, not amended.** ES-6 was **settled, not deferred**: `Error` keeps
  `core::error::Error + 'static` on both flavours and the stronger property lives in a downstream
  `ThreadSafeEventStore` marker. Project AC-005's wording ("bound added, or ADR-0009's deferral
  confirmed") predates the atom's acceptance. The real work is recording what
  `caller-visible-error-verdict`'s committed artefact showed about ADR-0009's *prediction* — that a
  `!Send` error carrying a live `worker::Error` is the asymmetry it named — as a new record.
- **`references/adr/` is not validated, and that is the dangerous half.** The same "just clarify it"
  instinct applied one directory over silently moves the lines `spec/SPECIFICATION.md` cites, and
  `cargo xtask spec-trace` notices only if a citation falls out of range
  (`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`). This story **adds**
  `references/adr/0023-*.md` and edits no existing long-form record.

**5. Resolving an open question has a defined shape, and it is not deletion.**
`.kb/open-questions/README.md`, *Resolving one*: the answer is a **new atom**, the question's record
stays, the two are linked through `related`, the question's `status` moves to `withdrawn` or
`superseded`, and *"the body describing what was not known at the time"* is left alone. Do not
rewrite a question into its own answer — the value of the record is that it shows the state of
knowledge on the day the choice was made. That metadata flip is the only edit these two atoms
receive, and `.kb/governance/rewrite-the-referent-never-the-reasoning.md` is the discrimination it
rests on: ask whether the edit changes what the document asserts, not whether it changes the
document.

**6. CF-40 must not be minted twice, and the check is a precondition of the wave, not a step inside
it.** `.kb/open-questions/cf-40-fixture-limits-ownership.md` is a contradiction between two accepted,
unedited ADRs — ADR-0015 mints CF-40 and its own Consequences section says decision 8 *"sets out both
and declines to choose"* between itself and ADR-0012. The atom names **phase 8** as what forces it,
and `sqlite-durable-store` (HS-P0012) merges one position ahead of this project. **Whichever project
reaches the answer first owns the resolution and the other cites it.** So the implementer's first
action is to look — `.kb/decisions/`, `.kb/maps/decision-map.md`, HS-P0012's merged specs — and
record the branch taken. The useful answer is the atom's own sub-question 2, which
`measured-store-limits` has now narrowed with three real numbers: whether the fixture contract has
one owning document or is amended piecemeal by whichever ADR needs the next capability.

**7. WF-11 resolves on either finding, and neither finding changes a wire format.**
`.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` is owned by phase 9 by
name. Its falsifier is broader than base64: serde's `serialize_str` and `collect_str` both
materialise the whole rendering, so *any* human-readable payload encoding buffers whole — the
question is only whether a real peer meets the condition. `wf-11-memory-ceiling-falsifier` forwarded
a payload sized against this runtime's measured memory ceiling through the adapter's real
non-streaming encode path and observed whether it bit. **Both outcomes resolve the atom**: if it
fired, the resolution is that WF-11's `MUST` is re-scoped to formats rather than peers and the
re-scoping decision is handed to `replication-identity-and-ingest` (HS-P0017); if it did not, the
resolution is *"the condition is not constructible on this runtime, and here is what would construct
it"*, which is an answer and not a shrug. **No wire format changes in this project** — the evidence
is gathered here, the decision is HS-P0017's.

**8. This story is the project's single KB writer.** Any `.kb/` change appearing in another
HS-P0013 story's diff is a defect in that story (`_storymap.md`, *Why these milestones and not
others*). Correspondingly, this story writes no Rust and touches no crate.

**9. The persona slice.** The observer here is the **gate reader** of `_storymap.md`'s backbone
(activity D, *"Know what this runtime cannot do, and why"*) one step further out: someone who opens
`.kb/` six months from now, having done none of this work, and needs to learn what phase 9 decided,
what it declined, and what it left open — without reading five story reports and a git log. The
outcome they get is one atom per decision, a supersession graph that resolves, two questions whose
records survive their own answers, and every one of them reachable from
`.kb/maps/decision-map.md` and `.kb/maps/open-questions-index.md`.

## Integration contract

- **Archetype**: `capability` — the observable slice is the state of `.kb/` after the wave, met by a
  reader who did none of the work. It is not `foundation`: nothing in this initiative compiles
  against it.
- **Slice / milestone**: `evidence-and-verdicts`. Slice-mates, implemented in one context and mounted
  as one integrated surface: **`wf-11-memory-ceiling-falsifier`** (HS-S0056) and
  **`deferral-re-reads-and-es-32-verdict`** (HS-S0057). This story merges **last** within the slice —
  its content is the other stories' findings (`_storymap.md`, **Merge order** §4).
- **Mount point**: **`.kb/maps/decision-map.md`** — the corpus's decision index, *"the one place that
  shows the whole supersession graph at a glance"*, updated by the Maps phase of every `kb-ingest`
  wave that lands a new or superseded decision atom. An atom that exists in `.kb/decisions/` and not
  on the map is an atom nobody reaches from the index, which is the KB's exact analogue of a
  component constructed but never rendered. ADR-0023's row lands there in the shape the seventeen
  existing rows use — atom id, title, status, phase 9, supersedes/superseded-by.
- **Wires into** (real sibling contracts consumed, by path):
  - `.kb/maps/open-questions-index.md` — the second mount. CF-40's and WF-11's bullets are
    **annotated in place, never removed**: *"A withdrawn or superseded question stays listed,
    annotated, rather than removed — the record that it was once open is itself worth keeping."*
  - `.kb/maps/domain-map.md` — the same atoms grouped by subject rather than by lineage.
  - `.kb/decisions/README.md` — the immutability rule, the repair-versus-amendment test, and the
    requirement to state the alternatives that lost.
  - `.kb/open-questions/README.md`, *Resolving one* — the status/`related`/body-untouched shape.
  - `.kb/_intake/README.md` — the staging contract, including that a successful run clears the
    directory and that a file still sitting there is a file the run did not ingest.
  - `.kb/_templates/` + `KbFrontmatter` as enforced by `redkiln validate --kb` — the schema every
    authored atom must satisfy; `.kb/decisions/0016-the-wire-format.md:1-30` is the reference shape
    for a decision atom's frontmatter (`adr_id`, `reversibility`, `phase`, `supersedes`,
    `superseded_by`).
  - `references/adr/0016-the-wire-format.md` — the long-form record's shape; the atom summarises, the
    record carries the transcripts, the rejected alternatives and the measurement tables.
  - `RUNBOOK.md:302` and `RUNBOOK.md:4267-4268` — the ADR queue row and phase 9's ADR-0023 work box;
    `RUNBOOK.md:295` is the struck-through shape ADR-0016 already established.
  - The five upstream stories' merged evidence — `durable-object-read-path` (HS-S0051),
    `caller-visible-error-verdict` (HS-S0052), `every-rule-under-workerd` (HS-S0054),
    `measured-store-limits` (HS-S0055), `wf-11-memory-ceiling-falsifier` (HS-S0056). Each `depends_on`
    edge is a piece of this record's *content*, not merely an ordering constraint.
- **Renders surfaces**: **none.** `_design.md` declares no surface for this project and this story
  claims no `## Items` path; there is nothing to re-decide and nothing to contradict.
- **Public items**: none. This story adds, removes and changes no `pub` item in any crate.
- **Conformance rule(s)**: **none, and deliberately.** Nothing here is adapter-observable — an atom
  is not a store, so no row belongs in `crates/happenstance-testkit/tests/mutation_coverage.rs`,
  whose rows are stores that fail named rules. The detectors that *do* observe this story already
  exist and are wired: `redkiln validate --kb` (frontmatter conformance + accepted-atom immutability
  against `HEAD`), `redkiln doctor` (the backlog, and exactly six `template-drift` advisories — a
  seventh is a template changed without deciding to), and `cargo xtask spec-trace` (clause citations,
  including the long-form line ranges).
- **Clause(s)**: this story amends **none**. CF-39, CF-40 and WF-11 are `[PROVISIONAL]` and are
  *discharged or resolved* rather than amended; CF-23, ES-6 and ES-9 are honoured by the upstream
  stories rather than changed. Nothing `[FROZEN]` is touched anywhere in HS-P0013, and this is the
  story that would have had to author the amending atom first if anything had been.
- **Advances DoD scenario**: initiative **DoD 4**'s second half — *"its error type is shown either to
  carry what the caller needs or demonstrably not to"* — which is a *record*, not a test run, and
  therefore closes here rather than in `caller-visible-error-verdict`. It also lands the project's own
  **DoD 5** (`project.md`) in full: ADR-0023 accepted under `.kb/decisions/`, `redkiln validate --kb`
  and `redkiln doctor` clean, CF-40 and WF-11 resolved rather than deleted. Structurally it is the
  initiative's first exerciser of the DoD 14 / DoD 15 pattern — *an accepted decision atom answers it,
  the corresponding open-question atom reflects the resolution, and `redkiln validate --kb` passes* —
  which HS-P0017 and HS-P0018 will each repeat.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
.kb/_intake/**
.kb/decisions/**
.kb/open-questions/**
.kb/reference/**
.kb/maps/**
.kb/_governance/integration-waves/**
references/adr/0023-*.md
RUNBOOK.md
.bklg/from-contract-to-published-library/cloudflare-durable-object-store/adr-0023-and-atom-resolutions/**
```

Wider than it looks and narrower than it reads: `.kb/maps/**`, `.kb/reference/**` and
`.kb/_governance/integration-waves/**` are written by the ingest wave itself, not typed by the
implementer, and they are inside the boundary because the wave commits on this branch.

**In this PR**

- The staged intake documents under `.kb/_intake/`, one per claim cluster, plus the proposed action
  plan and the HS-P0012 coordination result, authored before the gate.
- The handoff to `/redkiln:kb-ingest` and the post-wave verification of what it produced.
- `references/adr/0023-*.md`, the long-form record, **new**.
- `RUNBOOK.md:302`'s queue row struck through and pointed at the atom, and phase 9's ADR-0023 work
  box (`RUNBOOK.md:4267-4268`) ticked.
- Companion notes in this story's own backlog folder.

**Explicitly not in this PR**

- **Any Rust.** No crate under `crates/` is touched, and no `pub` item changes.
- **Any edit to an accepted decision atom's body** — ADR-0001, ADR-0008, ADR-0009, ADR-0011,
  ADR-0012, ADR-0015 and ADR-0016 are read and cited, never rewritten.
- **Any edit to an existing file under `references/adr/`.** New file only.
- **`RUNBOOK.md`'s phase 9 ledger paragraph and the CF-14 / CF-27 re-reads** → slice-mate
  `deferral-re-reads-and-es-32-verdict` (HS-S0057). This story's `RUNBOOK.md` edits are confined to
  the ADR queue row and the ADR-0023 work box; the two stories share the file and not the lines.
- **Any wire-format change** → `replication-identity-and-ingest` (HS-P0017), even if WF-11's
  falsifier fired.
- **Minting CF-40's answer when HS-P0012 already has.** In that branch this story *cites*.
- **`.kb/product/` persona and journey atoms** → `closeout-and-durable-audience` (HS-P0019).
- **Deleting `.kb/_intake/README.md` from the repository.** It is dropped *from the wave*, at the
  approval gate; the file stays on disk.

**Merge DoD one-liner.** ADR-0023 is an accepted atom under `.kb/decisions/` reachable from
`.kb/maps/decision-map.md`, CF-40's and WF-11's atoms are resolved with their bodies unchanged, no
accepted atom's body moved, and `redkiln validate --kb`, `redkiln doctor` (six `template-drift`
advisories, no seventh) and `cargo xtask spec-trace` are all green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Coordinate CF-40 before authoring anything** | Look for an already-minted CF-40 resolution from HS-P0012 (`sqlite-durable-store`, merge position 3). Record the branch and the evidence for it in this story's folder. Branch A — it exists: this story stages a *citation*, not a minting, and says which atom owns it. Branch B — it does not: this story stages the resolution and records that HS-P0012 cites it. The forbidden outcome is two independent mintings. | `.kb/open-questions/cf-40-fixture-limits-ownership.md` (*What is not decided*, *What forces it*, sub-question 2); `.bklg/…/cloudflare-durable-object-store/_decomposition.md` Architecture brief §6; `.bklg/from-contract-to-published-library/_decomposition.md:44`; `.kb/maps/decision-map.md` |
| **Stage the wave, do not write the atoms** | Author `.kb/_intake/*.md` — raw material, not held to `KbFrontmatter` — one document per claim cluster: the `SqlStorage` mapping + harness (ADR-0023), the ES-6 verdict against ADR-0009's prediction, CF-40's resolution or citation, WF-11's resolution. Then hand off. Nothing is written into `.kb/decisions/` by hand. | `.kb/_intake/README.md`; `CLAUDE.md` **Where the work lives** (commit `0269720`); `discover.md` **The wrong implementation** |
| **ADR-0023 states one question with two consequences** | The `SqlStorage` mapping and the off-tokio harness in a single atom, justified explicitly against the "and"-in-a-title playbook by naming the single body of evidence that settles both — the suite executing under `workerd` against the real bindings. | `.kb/playbooks/one-decision-per-adr-title.md` (*It stops being worth the split when both halves are settled by the same evidence*); `project.md` AC-006; `RUNBOOK.md:302` |
| **ADR-0023 names the alternatives that lost, and why each lost** | At minimum: `vitest-pool-workers` as its own CI job (`RUNBOOK.md:4267-4268`) — loses to AC-004's *same run as the rest of the gate* if an in-gate step can exist; `wasm-bindgen-test-runner` under node with no Durable Object — loses if the `SqlStorage` binding cannot be satisfied without `workerd`; a probe-gated step with no compensating mandatory assertion — loses to AC-004's own wording. Which one *won* is `every-rule-under-workerd`'s finding; this story records it, it does not choose it. On the mapping side, the same treatment for the ceiling capture, the DCB query rendering, and `JsThrow` versus `StringifiedThrow`. | `.kb/decisions/README.md` (*State the alternatives that lost*); `discover.md` **Questions**, *The off-tokio harness shape*; `_decomposition.md` Architecture brief §6 |
| **ADR-0001 is cited, not lifted** | ADR-0023 records that phase 9 furnishes the real-runtime evidence behind an already-accepted decision, that the `provisional` marker was lifted at phase 1 and recorded by ADR-0008, and therefore that `RUNBOOK.md:4280-4282`'s "formally retired" is discharged by citation. The reason is written down so the instruction does not invite the same wrong move next time. | `.kb/decisions/0001-async-port-flavours.md`; `.kb/decisions/0008-one-derivation-for-both-ports.md`; `RUNBOOK.md:4280-4282`; `_decomposition.md` Architecture brief §2 |
| **ADR-0009's prediction is judged in a new record** | The ES-6 outcome — what `caller-visible-error-verdict`'s committed reconstruction test showed about a caller-visible `CloudflareEventStoreError` carrying a real `worker::Error` — is recorded as new content. Zero bytes change in `.kb/decisions/0009-error-send-sync.md`. If the finding contradicts ADR-0009, the mechanism is a superseding atom, never a clarifying line. | `.kb/decisions/0009-error-send-sync.md`; `.kb/decisions/README.md` (*The immutability rule*); `project.md` DR-7 and AC-005 |
| **Two open questions resolve without losing their bodies** | For each of CF-40 and WF-11: `status` → `withdrawn` or `superseded`, `related` gains the answering atom's id, `superseded_by` set where the shape calls for it, **body unchanged**. The bullet on `.kb/maps/open-questions-index.md` is annotated in place, never removed. | `.kb/open-questions/README.md` (*Resolving one*); `.kb/maps/open-questions-index.md` frontmatter summary; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **WF-11 resolves on either finding** | Fired → the resolution states that WF-11's `MUST` re-scopes to formats rather than peers, and hands the re-scoping decision to HS-P0017. Did not fire → the resolution states the condition is not constructible on this runtime and names what would construct it. Either way: no wire format changes here. | `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` (*Ordered sub-questions*); `.kb/decisions/0016-the-wire-format.md`; `project.md` **Out of scope** |
| **The long-form record lands beside the atom** | `references/adr/0023-*.md` is new and carries what the ~100-line atom cannot: the transcripts, the measurement tables from `measured-store-limits`, and the rejected alternatives in full. The atom is linked; the record is cited by `file:line`. No existing file under `references/adr/` is edited — that directory is unvalidated and `spec/SPECIFICATION.md` cites line ranges inside it. | `CLAUDE.md` **Where the work lives**; `references/adr/0016-the-wire-format.md` as the shape; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` |
| **The wave is hygienic** | The wave id is suffixed so it does not overwrite an earlier wave's audit trail under `.kb/_governance/integration-waves/`; `.kb/_intake/README.md` is dropped at the approval gate rather than ingested; a successful run **clears** `.kb/_intake/`, so a file still sitting there afterwards is a file the wave did not ingest. | `.kb/_intake/README.md` (*A successful ingest clears this directory*); `.kb/_governance/integration-waves/2026-08-10-intake`, `…-intake-2` |
| **The runbook's queue row closes** | `RUNBOOK.md:302` is struck through and rewritten in the shape `RUNBOOK.md:295` uses for ADR-0016 — *"**Written**, as [ADR-0023](.kb/decisions/0023-….md), and accepted"* plus the one-sentence statement of what it settled — and phase 9's ADR-0023 work box is ticked. Nothing else in `RUNBOOK.md` is this story's. | `RUNBOOK.md:295`, `:302`, `:4267-4268` |
| **Post-wave verification is part of the story, not a follow-up** | `redkiln validate --kb` clean; `redkiln doctor` reporting exactly six `template-drift` advisories and no seventh; `cargo xtask spec-trace` green. `redkiln adopt --templates` is never run. | `CLAUDE.md` **Commands** and the six-template rule; `.kb/decisions/README.md`; `xtask` `spec-trace` step |

## Data and migrations

**N/A for runtime data** — this story adds no schema, no column, no serialised format and no stored
value. It touches nothing under `crates/`, so the Durable Object's SQL schema (`origin_store`,
`origin_position` and the rest) is entirely `durable-object-write-path`'s.

There is one true state transition, and it is metadata on knowledge-base atoms. It is recorded here
because it is the only place in this story where an existing tracked file's frontmatter changes, and
because the wrong version of it is `git rm`:

| Atom | Field | From | To | Body |
| --- | --- | --- | --- | --- |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | `status` | `accepted` | `withdrawn` or `superseded` | unchanged, byte for byte |
| " | `related` | `kb-decision-0015`, `kb-decision-0012`, `kb-decision-0010` | + the answering atom's id | — |
| `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` | `status` | `accepted` | `withdrawn` or `superseded` | unchanged, byte for byte |
| " | `related` | `kb-decision-0016`, `kb-decision-0003`, `kb-reference-wire-format-measurements-001` | + the answering atom's id | — |
| `.kb/decisions/0023-*.md` | — | *(does not exist)* | new atom: `kind: decision`, `authority_tier: decision`, `adr_id: ADR-0023`, `phase: 9`, `reversibility`, `supersedes: null`, `superseded_by: null` | authored by the wave |
| `.kb/maps/decision-map.md`, `.kb/maps/open-questions-index.md`, `.kb/maps/domain-map.md` | rows / bullets | seventeen decision rows; CF-40 and WF-11 listed as open | + ADR-0023's row; CF-40 and WF-11 annotated as resolved, still listed | synced by the wave's Maps phase |

**Rollback.** There is none in the KB's own model, and that is the point: an accepted atom is never
un-accepted by an edit. If ADR-0023 turns out to be wrong, the remedy is a superseding atom carrying
`supersedes: [kb-decision-0023]` and a metadata flip on this one — which is why the record must state
the alternatives that lost while the reasons are still known. The wave itself is a single commit on
this branch, so reverting *before* merge is an ordinary `git revert`; after merge it is a new
decision.

## Acceptance criteria

Twelve criteria. Each is framed from the intent of a person who did none of this work — the **gate
reader** of `_storymap.md`'s backbone activity D (*"Know what this runtime cannot do, and why"*), the
**adapter author** (persona 2), the **edge developer** (persona 3) and the **evaluator** (persona 4)
of `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` —
because a knowledge-base write whose only beneficiary is its author is indistinguishable from not
writing it. `redkiln verify --grain story` extracts these rows and `_ledger.md` carries one row per id.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN `sqlite-durable-store` (HS-P0012) merges one position ahead of this project and may already have answered CF-40, and GIVEN the forbidden outcome is two independent mintings, WHEN the implementer takes their **first** action on this story — before authoring a single intake document — THEN they look in `.kb/decisions/`, `.kb/maps/decision-map.md` and HS-P0012's merged specs, and record in this story's own folder which branch holds (A: a resolution exists, so this story *cites* it and names the owning atom; B: none exists, so this story stages the resolution and records that HS-P0012 cites it) together with the evidence for that reading — so that exactly one minting of CF-40's answer exists across both projects. | Review gate on a committed coordination note under `.bklg/…/adr-0023-and-atom-resolutions/` naming branch + evidence path; `rg -n "CF-40" .kb/decisions .kb/maps` re-run at implement time and its output quoted in the note; diff review confirms no second minting. |
| AC-002 | GIVEN that **no automated check in this repository can distinguish an ingested atom from a hand-written one** — `redkiln validate --kb` checks conformance and immutability, never provenance — and GIVEN that hand-authoring `.kb/decisions/` was already done once here and reverted (`CLAUDE.md`, **Where the work lives**, commit `0269720`), WHEN this story produces its knowledge-base content, THEN every new atom and every open-question metadata flip arrives through a `/redkiln:kb-ingest` wave staged from `.kb/_intake/*.md` and gated by a human, and the branch's history shows the wave's own commit introducing them — never a file typed directly into `.kb/decisions/`. | `git log --oneline -- .kb/decisions/` on the branch shows the atom introduced by the ingest wave's commit, not by a hand-edit commit; `.kb/_governance/integration-waves/<wave-id>/03-integration-summary.md` exists and lists the atom; review gate against `discover.md` **The wrong implementation**. |
| AC-003 | GIVEN a gate reader who opens `.kb/maps/decision-map.md` six months from now having done none of this work, WHEN they ask what phase 9 settled about the Cloudflare adapter, THEN one row on that map points at a single accepted atom `.kb/decisions/0023-*.md` that states the `SqlStorage` mapping **and** the off-tokio harness as ONE question, the atom names the single body of evidence that makes the conjunction one decision (the suite executing under `workerd` against the real bindings) and cites `.kb/playbooks/one-decision-per-adr-title.md`'s own stated exception, and **no second atom splits the pair** — an atom in `.kb/decisions/` that is absent from the map is the KB's analogue of a component constructed but never rendered and fails this criterion. | `redkiln validate --kb`; `ls .kb/decisions/0023-*.md` returns exactly one path; `rg -n "0023" .kb/maps/decision-map.md` returns the row; review gate on the "one question, two consequences" paragraph. |
| AC-004 | GIVEN the adapter author who later has to stand a fourth conformance harness up on a runtime nobody has tried, and whose stated fear is discovering late that the ground was already walked, WHEN they read ADR-0023 and its long-form record, THEN each rejected alternative is named **with the reason it lost** — at minimum `vitest-pool-workers` as its own CI job, `wasm-bindgen-test-runner` under node with no Durable Object, and a probe-gated step carrying no compensating mandatory assertion — the same treatment covers the mapping side (the ceiling capture, the DCB query rendering, and `JsThrow` versus `StringifiedThrow`), and the *winning* harness is recorded as `every-rule-under-workerd`'s finding rather than chosen here. | Review gate against `.kb/decisions/README.md` (*state the alternatives that lost*); each named alternative and its losing reason present in `.kb/decisions/0023-*.md` or `references/adr/0023-*.md`; the winning shape traced by citation to `every-rule-under-workerd`'s merged artefact. |
| AC-005 | GIVEN `RUNBOOK.md:4280-4282` instructing that ADR-0001's `provisional` marker be "formally retired and this adapter cited", and GIVEN the marker was already lifted at phase 1 with ADR-0008 recording the lift, WHEN this story discharges that instruction, THEN `.kb/decisions/0001-async-port-flavours.md` is byte-identical to `main`, the discharge takes the form of a citation *inside* ADR-0023 and the long-form record, and the record states **why the runbook's wording was read and not obeyed** — so the next reader does not attempt the edit the instruction invites. | `git diff main -- .kb/decisions/0001-async-port-flavours.md` produces no output; `redkiln validate --kb` (accepted-atom immutability against `HEAD`); review gate on the "cited, not lifted" paragraph and its ADR-0008 citation. |
| AC-006 | GIVEN the evaluator deciding in a bounded sitting whether this library tells the truth about the runtime it claims, WHEN they ask whether ADR-0009's ES-6 prediction survived contact with a real `worker::Error`, THEN the answer exists as **new content** — the verdict recorded in ADR-0023 or its own atom, citing `caller-visible-error-verdict`'s committed reconstruction test by path — `.kb/decisions/0009-error-send-sync.md` is byte-identical to `main`, and any contradiction of ADR-0009 is expressed as a superseding atom rather than a clarifying line; project AC-005's older wording ("bound added, or ADR-0009's deferral confirmed") is corrected in the record, because ES-6 was settled rather than deferred. | `git diff main -- .kb/decisions/0009-error-send-sync.md` produces no output; `redkiln validate --kb`; the verdict paragraph cites the reconstruction test's real path in `crates/happenstance-cloudflare/`. |
| AC-007 | GIVEN a reader whose value from the corpus is knowing not only what was decided but **what was unknown on the day it was decided**, WHEN CF-40's ownership question is answered, THEN `.kb/open-questions/cf-40-fixture-limits-ownership.md` still exists with its body byte-identical, its `status` moved to `withdrawn` or `superseded`, its `related` extended with the answering atom's id, its answer covering sub-question 2 (whether the fixture contract has one owning document or is amended piecemeal by whichever ADR needs the next capability), and its bullet on `.kb/maps/open-questions-index.md` **annotated in place rather than removed** — the file is never `git rm`ed and the question is never rewritten into its own answer. | `git diff main -- .kb/open-questions/cf-40-fixture-limits-ownership.md` shows frontmatter hunks only and zero body hunks; `git diff --diff-filter=D --name-only main -- .kb/open-questions/` is empty; the index bullet is still present and carries the annotation; `redkiln validate --kb`. |
| AC-008 | GIVEN the edge developer who needs to know whether this runtime's memory ceiling forces a peer to buffer a payload it cannot hold, WHEN `wf-11-memory-ceiling-falsifier`'s finding is recorded, THEN `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` reaches a resolved state on **either** outcome — fired: WF-11's `MUST` re-scopes to formats rather than peers and the re-scoping decision is handed to `replication-identity-and-ingest` (HS-P0017); did not fire: the condition is not constructible on this runtime and the record names what *would* construct it — with the body byte-identical, the index bullet annotated in place, and **no wire-format change anywhere in this PR**. | `git diff main -- .kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` shows frontmatter hunks only; `git diff --name-only main -- crates/ spec/` is empty for this story's commits; `redkiln validate --kb`; review gate on which of the two resolutions was written and the evidence it cites. |
| AC-009 | GIVEN that `spec/SPECIFICATION.md` cites line ranges existing only in `references/adr/`, and that directory is validated by nothing, WHEN ADR-0023's long-form record lands, THEN `references/adr/0023-*.md` is a **new** file carrying what a ~100-line atom cannot — the compiler transcripts, `measured-store-limits`' measurement tables, and the rejected alternatives in full — **no existing file under `references/adr/` is modified**, and `cargo xtask spec-trace` is green. | `git diff --name-status main -- references/adr/` shows exactly one `A` row and zero `M` rows; `cargo xtask spec-trace`; review gate that the atom links the record and the record is citable by `file:line`. |
| AC-010 | GIVEN `.kb/_governance/integration-waves/` already holds `2026-08-10-intake` and `2026-08-10-intake-2`, and GIVEN `.kb/_intake/README.md` is a README rather than an atom that the default glob nevertheless picks up, WHEN the wave runs, THEN its id carries a suffix so it writes a **new** audit-trail directory instead of overwriting an earlier wave's, `.kb/_intake/README.md` is dropped at the approval gate and is still on disk afterwards, and `.kb/_intake/` is otherwise empty — a file still sitting there after a successful run is a file the wave did not ingest. | `ls .kb/_governance/integration-waves/` shows a third, distinctly-named directory with `main`'s two intact; `ls .kb/_intake/` returns `README.md` and nothing else; `git diff --diff-filter=D --name-only main -- .kb/_governance/` is empty. |
| AC-011 | GIVEN a reader following `RUNBOOK.md`'s ADR queue to find out whether 0023 was ever written, WHEN they reach the queue row at `RUNBOOK.md:302`, THEN it is struck through and rewritten in the shape `RUNBOOK.md:295` already uses for ADR-0016 — pointing at the atom by path and stating in one sentence what it settled — phase 9's ADR-0023 work box (`RUNBOOK.md:4267-4268`) is ticked, and **no other `RUNBOOK.md` line is touched by this story**, because the phase 9 ledger paragraph and the CF-14/CF-27 re-reads belong to slice-mate `deferral-re-reads-and-es-32-verdict`. | `git diff main -- RUNBOOK.md` for this story's commits confines every hunk to the queue row and the ADR-0023 work box; `cargo xtask spec-trace`; review gate against `RUNBOOK.md:295`'s established shape. |
| AC-012 | GIVEN this PR lands no Rust and therefore has **no compiler standing behind it**, WHEN the wave is complete and the story is proposed as done, THEN `redkiln validate --kb` is clean, `redkiln doctor` reports exactly **six** `template-drift` advisories and no seventh, `cargo xtask spec-trace` is green, `cargo xtask affected --base main` is green, and `redkiln adopt --templates` has not been run at any point in the story — the four commands are the whole of the automation available here and all four are part of the story, not of a follow-up. | `redkiln validate --kb`; `redkiln doctor` with the advisory count asserted at exactly six; `cargo xtask spec-trace`; `cargo xtask affected --base main`; review gate that no `adopt --templates` invocation appears in the story's log or history. |

**Coverage of the traced project ACs.** Project AC-005 → story AC-006. Project AC-006 → story AC-002,
AC-003, AC-004, AC-005, AC-009, AC-010, AC-011, AC-012. Project AC-008 → story AC-001, AC-007.
Project AC-011 → story AC-008. Nothing is traced twice by accident: AC-002 and AC-012 are the
provenance and verification halves that every other criterion depends on and are therefore stated
once rather than repeated per atom.

## Interaction quality

**Composition family — a stated N/A, not a skip.** `_design.md` for HS-P0013 is an approved **N/A
determination** (2026-08-12): no `## Items`, no `## Signatures`, no surface. This story renders
nothing, so *presentation exists at all*, *composition and placement*, *transience*, *density budget*
and *hierarchy* have no referent here and no AC row is owed for them. There is no design to
contradict and none is re-decided. Recording this explicitly matters because the section's usual
failure mode is a story that renders a surface and silently declines the composition bar; that is not
what is happening here, and a reviewer should be able to tell the two apart in one read.

**State family — real, and translated to the surface this story actually has.** The medium is a
long-lived document corpus rather than a screen, and every one of the state invariants has a precise
analogue in it, because the failure modes are the same failure modes. Each is carried by an AC row in
the table above; none is left as a prose bullet here.

| Invariant | Its analogue on this surface | Carried by | How it is verified |
| --- | --- | --- | --- |
| **In place, not a context jump** | An answered question is annotated where it already lives — status, `related`, an index annotation — rather than removed and re-created elsewhere. Deletion is the context jump: the reader who followed a link arrives nowhere. | AC-007, AC-008 | `git diff --diff-filter=D` over `.kb/open-questions/` empty; the index bullet still present. |
| **Non-occlusion** | The new record never covers the old one. ADR-0009's and ADR-0001's bodies stay legible and unmoved; a verdict is added beside them, never over them. | AC-005, AC-006 | `git diff main -- <atom>` produces no output for either accepted atom. |
| **Preserved focus / scroll / selection** | The reader's *position* is what survives here: `spec/SPECIFICATION.md`'s `file:line` citations into `references/adr/` must still land on the paragraph they were written against. Adding a new record preserves them; editing an existing one silently moves them. | AC-009, AC-011 | `cargo xtask spec-trace`; `git diff --name-status main -- references/adr/` shows `A` only. |
| **Reversibility** | An accepted atom is never un-accepted by an edit; the reverse of a wrong decision is a superseding atom plus a metadata flip. Before merge, the wave is one commit and an ordinary `git revert`. | AC-002, AC-006 | The wave is a single commit; `redkiln validate --kb` rejects a body change to an accepted atom. |
| **Reachability** (the keyboard-reachability analogue) | Every atom this story lands is reachable from an index without prior knowledge of its filename: `.kb/maps/decision-map.md` for the decision, `.kb/maps/open-questions-index.md` for the two questions, `.kb/maps/domain-map.md` for subject. An atom reachable only by knowing its path is unreachable. | AC-003, AC-007, AC-008 | The row and both bullets present after the wave's Maps phase; `redkiln validate --kb`. |
| **Provenance visibility** (the "real composed presentation, not bare markup" analogue) | A conformant-looking atom that no wave produced is this surface's unstyled render: it satisfies every schema assertion and is missing everything the process exists to add — adjudication, map sync, audit trail. Nothing automated catches it. | AC-002, AC-010 | The wave directory under `.kb/_governance/integration-waves/` lists the atom; `git log` attributes it to the wave's commit. |

**The named anti-patterns**, taken from `discover.md` **The wrong implementation** and blocked by the
rows above rather than by prose: the atom hand-written into `.kb/decisions/` (AC-002); the correction
applied as an edit to an accepted atom (AC-005, AC-006); the same instinct applied one directory over
to the unvalidated `references/adr/` (AC-009); and the question deleted because it has been answered
(AC-007, AC-008).

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | HS-P0012 has **already** minted CF-40's resolution by the time this story runs. | Branch A of AC-001: this story cites the existing atom and stages **no** competing resolution. The coordination note records the owning atom's id. A second minting is a defect even if both mintings agree. |
| EC-002 | The wave's adjudication phase proposes **merging** an ADR-0023 claim into an existing accepted decision atom, or **amending** one. | Refuse the merge into an accepted decision body at the approval gate. The only two legal shapes are a new atom or a superseding atom (`.kb/decisions/README.md`). Amendment is legal for non-decision atoms; for an accepted decision it is not, whatever the adjudicator scores. |
| EC-003 | `redkiln validate --kb` reports an accepted-atom body change after the wave. | Stop. Do not "fix" the atom forward. Restore the atom to its `HEAD` content, re-stage the claim as a superseding atom, and re-run the wave. The failure is the guard working, not an obstacle to route around. |
| EC-004 | `.kb/_intake/` still contains a document (other than `README.md`) after a "successful" wave. | Treat the wave as partial: that file was not ingested. Do not delete it to make the directory look clean — either re-run the wave for it or record in the coordination note why it was deliberately withheld. |
| EC-005 | `redkiln doctor` reports a **seventh** `template-drift` advisory. | A template changed without anyone deciding to. Halt and identify it. Never resolve it with `redkiln adopt --templates`, which would overwrite all six deliberate customisations and then fail CI on the absence it created (`CLAUDE.md`, **Where the work lives**). |
| EC-006 | `cargo xtask spec-trace` fails after the long-form record lands. | A clause citation no longer resolves. Since this story only *adds* a file under `references/adr/`, the likely cause is a citation authored into the new record against a range that does not exist. Fix the new record; do not adjust the clause. |
| EC-007 | An upstream story's finding is missing, inconclusive, or contradicts what this spec assumed (e.g. the WF-11 falsifier was never run, or `every-rule-under-workerd` declined a family this record was going to cite). | Record the gap as a gap. The record states what was observed and what was not, and — for WF-11 — that "the condition is not constructible on this runtime, and here is what would construct it" is a resolution, not a shrug. Do not invent a finding to complete a sentence. |
| EC-008 | The chosen wave id collides with an existing directory under `.kb/_governance/integration-waves/`. | Suffix it (`…-intake-3`, following `2026-08-10-intake-2`'s precedent) before running. An overwritten wave directory destroys an earlier wave's audit trail and is not recoverable from the atoms it produced. |
| EC-009 | ADR-0023 exists under `.kb/decisions/` but no row appears on `.kb/maps/decision-map.md`. | The Maps phase did not run or did not see the atom. The story is not done: an unindexed atom is unreachable. Re-run the maps sync rather than hand-adding the row, for the same provenance reason as AC-002. |
| EC-010 | The CF-40 answer this story would mint contradicts what `measured-store-limits` actually measured. | The measurement wins over the plan. Re-open the coordination note, state the contradiction, and stage the resolution the evidence supports — including "the fixture contract gets one named owner" being the wrong answer if the evidence says otherwise. |

## Non-functional

| id | Requirement | Why it binds here |
| --- | --- | --- |
| NF-001 | The atom stays at the corpus's ~100-line grain; everything longer goes into `references/adr/0023-*.md`. | The split is the whole reason both directories exist. An atom that grows to hold transcripts stops being summarisable and starts competing with its own record (`CLAUDE.md`, **Where the work lives**). |
| NF-002 | This PR compiles nothing: no file under `crates/`, `examples/`, `xtask/` or `spec/` changes. | `cargo xtask affected --base main` should select no package. If it selects one, the PR boundary has been crossed and the story has taken work belonging to a slice-mate. |
| NF-003 | Links are outbound-only from the atoms this wave authors; reciprocal backlinks are wired by the wave's Maps phase, not by hand. | Hand-wiring a backlink is the same provenance defect as hand-writing the atom, and is the specific detail `discover.md`'s wrong implementation calls out as "added by hand for good measure". |
| NF-004 | Every state change in this story is forward-only: new atom, superseding atom, or metadata flip. No content is destroyed. | The corpus's value is that it shows the state of knowledge on the day each choice was made. Destroying that to tidy up is the failure this whole story exists to avoid. |
| NF-005 | The wave lands as a single commit on this branch, so pre-merge rollback is one `git revert`. | Post-merge there is no rollback in the KB's model — only a superseding decision. The cheap window is before merge and it is worth keeping cheap. |
| NF-006 | A reader starting from `.kb/maps/decision-map.md` or `.kb/maps/open-questions-index.md` reaches every artefact this story produced in at most two hops, without reading a story report or the git log. | This is the persona outcome restated as a measurable property, and it is the difference between a knowledge base and a directory of files. |
| NF-007 | Every normative sentence in ADR-0023 cites the upstream artefact that produced it, by path. | Five stories' findings are being compressed into one record. Uncited, the record becomes an assertion rather than evidence, and the next reader cannot check it — which is exactly the state phase 9 was supposed to end. |

## Implementation notes (non-prescriptive)

The order below is the one the constraints imply; the content of each step is the implementer's.

1. **Coordinate before authoring.** AC-001's check is a *precondition of the wave*, not a step inside
   it. Doing it after the intake documents are written means writing one that may have to be thrown
   away — and worse, being tempted not to throw it away.
2. **Read the five upstream artefacts as content, not as ordering.** Each `depends_on` edge exists
   because a paragraph of this record is that story's finding. If an upstream story's merged evidence
   does not actually say what this spec assumed, EC-007 governs: record what it says.
3. **Stage one intake document per claim cluster**, not one per output atom. The wave's adjudication
   is what decides how claims map to atoms; pre-deciding that by naming the files after the atoms
   biases the adjudicator toward the shape you already chose. Intake documents are raw material and
   are not held to `KbFrontmatter` (`.kb/_intake/README.md`).
4. **Write the proposed action plan alongside them**, including the CF-40 branch, the wave id (with
   its suffix), and the explicit instruction that `.kb/_intake/README.md` is dropped at the approval
   gate. That plan is what the human reads at the gate; anything left implicit gets improvised there.
5. **Hand off.** `/redkiln:kb-ingest` is user-invoked. The story pauses here by design.
6. **Verify what the wave produced**, against AC-003, AC-007, AC-008, AC-009, AC-010 and AC-012 —
   including reading the wave's own `03-integration-summary.md`. A wave that succeeded and produced
   the wrong shape is the case these criteria exist for.
7. **Close the runbook row last**, after the atom's real filename is known, so the citation points at
   a path that exists.

Deliberately left open: the atom's exact filename beyond `0023-*`; whether the ES-6 verdict lives
inside ADR-0023 or as its own atom (both satisfy AC-006 — the adjudicator may legitimately prefer
either, and the constraint is only that it is *new content*); the precise `withdrawn`-versus-
`superseded` choice per question, which follows from whether an answering atom supersedes the
question or merely retires it; and the wave id's exact suffix.

## Tests and CI (merge gate)

There is no test tier here in the compiler's sense, and the testing brief says so directly: AC-006,
AC-008's ownership half and AC-010 sit in its **process-gate tier** — *"No test runner is the right
tool for 'is this decision atom accepted and immutable'"* (`_decomposition.md`, Testing brief Notes
§1). What follows is the whole of the automation that stands behind this story, plus the two places a
human is the instrument.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Process gate | `redkiln validate --kb` | `KbFrontmatter` conformance on every atom the wave wrote, **and** accepted-decision immutability checked against `HEAD` — the mechanical half of AC-003, AC-005, AC-006, AC-007, AC-008. |
| Process gate | `redkiln doctor` | The backlog is coherent and exactly six `template-drift` advisories are reported; a seventh fails AC-012 and triggers EC-005. |
| Static / traceability | `cargo xtask spec-trace` | Every `spec/SPECIFICATION.md` citation still resolves, including the `file:line` ranges into `references/adr/` that nothing else checks — AC-009 and AC-011. |
| Story grain | `cargo xtask affected --base main` | The story-grain verification `.redkiln/config.yaml`'s `verify:` block runs at the `advance` seam. For this story it should select **no package** (NF-002); a green run over an empty selection is the expected shape, and a non-empty selection is a PR-boundary breach. |
| Project grain | `cargo xtask ci --fast` | The `REQUIRED`-only bar this non-terminal project is held to (`CLAUDE.md`, **Commands**). Not this story's own gate, but it must still be green with this story merged — nothing here may break it, and nothing here can fix it. |
| Diff assertion | `git diff main -- .kb/decisions/0001-async-port-flavours.md .kb/decisions/0009-error-send-sync.md` (empty) | AC-005 and AC-006's byte-identity requirement, checked directly rather than trusted. |
| Diff assertion | `git diff --name-status main -- references/adr/` (exactly one `A`, zero `M`) | AC-009 — the unvalidated directory is added to and never edited. |
| Diff assertion | `git diff --diff-filter=D --name-only main -- .kb/` (empty) | AC-007 and AC-008 — resolved rather than deleted, stated as a check rather than as an intention. |
| Review gate | The diff, read against `.kb/decisions/README.md` and `.kb/open-questions/README.md` | AC-001, AC-004 and the prose halves of AC-003, AC-006, AC-011 — "does this record name the alternatives that lost, and why each lost" is not a machine question. |
| Review gate | `.kb/_governance/integration-waves/<wave-id>/03-integration-summary.md` | AC-002 and AC-010 — the wave ran, the atoms are attributed to it, and the audit trail is new rather than overwritten. |

No row belongs in `crates/happenstance-testkit/tests/mutation_coverage.rs`: its rows are stores that
fail named conformance rules, and an atom is not a store (`discover.md`, *Where the detectors live*).

## Risks and coupling (PR-scoped)

- **Five upstream dependencies, and every one of them is content.** This story cannot be written
  early, and if any of the five merges with a weaker finding than planned, this record shrinks to
  match rather than asserting more than the evidence carries (EC-007). The mitigation is that it
  merges last within `evidence-and-verdicts` by design (`_storymap.md`, **Merge order** §4).
- **The HS-P0012 coupling is cross-project and only loosely enforced.** Nothing mechanical prevents
  two mintings of CF-40's answer; only AC-001's check does. This is the highest-likelihood defect in
  the story and the one with the least automation behind it.
- **The human gate is a real pause.** `/redkiln:kb-ingest` carries `disable-model-invocation`, so
  implementation genuinely stops between step 5 and step 6. A plan that assumes an unbroken run will
  either stall or — worse — route around the gate by hand-writing the atoms, which is AC-002's exact
  prohibition.
- **The wave's adjudicator has latitude this story does not control.** It is biased toward merging
  and superseding, which is usually right and is wrong for an accepted decision body. EC-002 is the
  standing instruction at the gate.
- **`RUNBOOK.md` is shared with a slice-mate.** `deferral-re-reads-and-es-32-verdict` (HS-S0057) edits
  the phase 9 ledger paragraph in the same file. The two stories share the file and not the lines
  (AC-011); a merge conflict here means one of them took the other's hunk.
- **The unvalidated half is the dangerous half.** `references/adr/` is checked by nothing except
  `spec-trace`'s range test, which only notices a citation that falls *out* of range. A record edited
  in place can leave every clause still resolving and pointing at the wrong paragraph — a silent
  failure with no detector, mitigated only by AC-009's add-only rule.
- **No compiler stands behind this PR.** NF-002 is a benefit for CI cost and a risk for correctness:
  everything true about this change is true because a person or a schema said so, which is why AC-012
  makes all four commands part of the story rather than of a follow-up.

## Dependencies

**Blocks on** (all five must be merged; each supplies content, not merely ordering):

| Story slug | What this record takes from it | Serves |
| --- | --- | --- |
| `durable-object-read-path` | ADR-0011's ceiling-and-page resolution as actually built, or the compiled reason it did not fit — the mapping half of ADR-0023. | AC-003, AC-004 |
| `caller-visible-error-verdict` | The ES-6 artefact and what it showed about a caller-visible `CloudflareEventStoreError` carrying a real `worker::Error`. | AC-006 |
| `every-rule-under-workerd` | The harness shape actually built (the alternative that *won*) and the concurrency family's stated non-invocation. | AC-003, AC-004 |
| `measured-store-limits` | The three measured numbers and how each was measured — the evidence that narrows CF-40 to its sub-question 2. | AC-001, AC-007 |
| `wf-11-memory-ceiling-falsifier` | Whether the falsifier bit, and under what payload size. | AC-008 |

**Unlocks:** no story slug inside HS-P0013 — this story merges last in its slice and nothing in the
project consumes it. What it unlocks is outside the project: `replication-identity-and-ingest`
(HS-P0017) inherits WF-11's re-scoping decision if the falsifier fired, and
`closeout-and-durable-audience` (HS-P0019) inherits the DoD 14 / DoD 15 pattern this story is the
initiative's first exerciser of.

## Anchors (progressive disclosure)

Everything above is enough to start. Open these at the moment named, not before — and link them from
the record rather than pasting them into it.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | Carries the actual contradiction (ADR-0015's header versus its own Consequences section, quoted verbatim) and the ordered sub-questions — sub-question 2 is the answer worth writing. | Before step 1, during the HS-P0012 coordination check. | AC-001 |
| `.kb/open-questions/README.md` | The *Resolving one* section is the exact shape a resolution takes: new atom, `related` link, `status` to `withdrawn` or `superseded`, body left alone — plus the `.passthrough()` note that an imported corpus's own `tracks`/`resolution_ref` keys must not be stripped. | Before staging either question's resolution. | AC-007 |
| `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` | Its three ordered sub-questions decide which resolution wording is legitimate; sub-question 1 is why "the falsifier never fired" is an answer rather than a failure. | When writing WF-11's resolution, after reading the falsifier's finding. | AC-008 |
| `.kb/decisions/README.md` | States the immutability rule, the repair-versus-amendment test, and the requirement to name the alternatives that lost — the three rules this story is most likely to violate by good intentions. | Before authoring any intake document, and again at the approval gate. | AC-004 |
| `.kb/_intake/README.md` | The staging contract: what an intake document is, that it is not held to `KbFrontmatter`, and that a successful run clears the directory. | At step 3, before writing the first intake file. | AC-002 |
| `.kb/playbooks/one-decision-per-adr-title.md` | Supplies the *stated exception* that makes ADR-0023's conjunction legitimate; without citing it, the atom looks like a violation of this corpus's own finding. | While drafting ADR-0023's opening. | AC-003 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The discrimination the metadata-only flip rests on: ask whether the edit changes what the document *asserts*, not whether it changes the document. | Before touching either open question's frontmatter. | AC-007 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | Why an edit inside `references/adr/` is more dangerous than an edit inside `.kb/`, and how citations are anchored so they survive. | Before creating `references/adr/0023-*.md`. | AC-009 |
| `.kb/decisions/0016-the-wire-format.md` | The reference shape for a decision atom's frontmatter — `adr_id`, `reversibility`, `phase`, `supersedes`, `superseded_by` — and the ~100-line grain. | When reviewing what the wave produced against AC-003. | AC-003 |
| `references/adr/0016-the-wire-format.md` | The long-form record's shape: what a record carries that its atom cannot, and how the two point at each other. | When drafting the long-form record. | AC-009 |
| `.kb/decisions/0009-error-send-sync.md` | The ES-6 prediction being judged, in its own words — the verdict must engage what it actually claimed, and this is the file that must end the PR byte-identical. | When writing the ES-6 verdict. | AC-006 |
| `.kb/decisions/0008-one-derivation-for-both-ports.md` | Records that ADR-0001's `provisional` marker was lifted at phase 1, which is the evidence for reading `RUNBOOK.md:4280-4282` as discharged by citation. | When writing the "cited, not lifted" paragraph. | AC-005 |
| `.kb/maps/decision-map.md` | The mount point. Shows the seventeen existing rows' shape and the whole supersession graph ADR-0023's row joins. | At step 6, verifying the wave's Maps phase. | AC-003 |
| `.kb/maps/open-questions-index.md` | The second mount, and the source of the rule that a withdrawn question stays listed and annotated rather than removed. | At step 6, verifying both questions' bullets. | AC-007 |
| `.kb/_governance/integration-waves/2026-08-10-intake-2/03-integration-summary.md` | A completed wave's audit trail — the precedent for the suffixed id, and the shape of the summary this story's wave must also produce. | When choosing the wave id, and again when verifying the wave. | AC-010 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/_decomposition.md` | Architecture brief Notes §2 (the accepted atoms and the three tensions) and §6 (ADR-0023's material and the CF-40/WF-11 latitude); Testing brief Notes §1 names the process-gate tier this story lives in. | Before step 2, when reading the upstream findings as content. | AC-004, AC-012 |
| `.bklg/from-contract-to-published-library/cloudflare-durable-object-store/adr-0023-and-atom-resolutions/discover.md` | The signal ledger and, more importantly, **The wrong implementation** — the two named failure modes and the statement that provenance has no detector. | Before staging anything, and at the approval gate. | AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The four personas whose intent frames every criterion above, with the qualification that all four rest on secondary evidence — the framing this record inherits and must not overstate. | When writing the record's audience-facing framing. | AC-006 |
| `RUNBOOK.md` | `:295` is the struck-through shape ADR-0016 established, `:302` is the row to close, `:4267-4268` is the work box, and `:4280-4282` is the instruction that must be read rather than obeyed. | At step 7, last. | AC-005, AC-011 |
| `.kb/reference/wire-format-encoding-measurements.md` | The existing measurement reference WF-11's atom already relates to; the falsifier's result is read against it rather than in isolation. | When writing WF-11's resolution. | AC-008 |

## Clarifications resolved during spec

1. **AC count.** Twelve criteria, AC-001 through AC-012, exactly as the first pass decided. None were
   added and none dropped; the ledger carries the same twelve.
2. **Project AC-005's wording is corrected, not satisfied literally.** It says "bound added, or
   ADR-0009's deferral confirmed", which predates ADR-0009's acceptance — ES-6 was **settled**, not
   deferred. Story AC-006 therefore requires a *verdict on ADR-0009's prediction* recorded as new
   content, and requires the record to state the correction, so the next reader does not go looking
   for a deferral to confirm.
3. **`RUNBOOK.md:4280-4282` is discharged by citation, not obeyed.** ADR-0001's `provisional` marker
   was lifted at phase 1 and ADR-0008 records the lift; the atom is accepted and immutable. AC-005
   makes the byte-identity of `0001-async-port-flavours.md` a checked criterion precisely because the
   runbook's wording invites the opposite action.
4. **Whether the ES-6 verdict is its own atom or a section of ADR-0023 is left to the wave.** Both
   satisfy AC-006. The binding constraint is that it is *new content* and that ADR-0009 is untouched;
   pre-deciding the atom count here would pre-empt the adjudication AC-002 exists to preserve.
5. **CF-40's resolution is branch-dependent and both branches are specified.** AC-001 accepts either
   outcome and rejects only the third: two independent mintings. The spec does not guess which branch
   holds, because HS-P0012 merges between this spec and its implementation.
6. **WF-11 resolves on either finding.** "The condition is not constructible on this runtime, and here
   is what would construct it" is a resolution under AC-008, not a deferral. No wire-format change is
   in scope on either branch; that decision belongs to HS-P0017.
7. **The interaction-quality composition family is N/A and is stated as such.** `_design.md` is an
   approved N/A determination for HS-P0013 and this story renders no surface. The state family is
   *not* waived — it is translated to the document corpus, and every translated invariant is carried
   by an AC row rather than by a bullet, so each one is gated.
8. **`cargo xtask affected --base main` is expected to select nothing.** That is stated as NF-002 and
   verified as part of AC-012 rather than treated as a vacuous pass: a non-empty selection means this
   story touched a crate, which is a PR-boundary breach, not a bonus.
