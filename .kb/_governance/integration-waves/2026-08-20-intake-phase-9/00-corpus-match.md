# Wave `2026-08-20-intake-phase-9` — corpus match

What already exists in `.kb/`, what each incoming claim could have merged into, and the score
behind every disposition. Read this before `02-placement-and-adjudication.md`: the placements
there are only defensible because of what this file establishes about the corpus, about the
backlog, and about the five staged files.

This is the corpus's **sixth wave** and its first **phase-9** wave. Its shape is unlike the last
one's. The 2026-08-17 wave had to allocate four ADR numbers against two colliding claims; this
one has **one number that is already fixed by an artefact on disk** (`ADR-0023`, whose 30 KB
long-form record exists) and **one that nothing anywhere allocates** (the CF-40 resolution).
Its hard jobs are three:

1. **One cross-file collapse that is unambiguous.** `es-6-verdict-…` is a section of ADR-0023,
   not an atom. ADR-0023's own body forward-references it in terms — *"See the ES-6 cluster,
   staged separately"* — which is the strongest single merge signal any wave has received.
2. **One cross-file collapse that is not obvious and is the wave's real dedup.** ADR-0023's
   *"what the harness does not prove"* and WF-11's *"the runner does not model a per-isolate
   memory cap"* are **the same unowned gap, reached from two directions**, and WF-11's own text
   says so: *"a second, independent consequence of `every-rule-under-workerd`'s escalated
   blocking finding."* Ingested independently, that gap would have been described twice, in a
   decision body and in a reference body, and owned by neither.
3. **Two calls the intake hands to the wave and the wave declines to make.** The `deny.toml`
   ratify-or-refuse, and the three store-limit numbers. One becomes an open question; the other
   is not adjudicable from anything staged and is carried in `unresolved`.

The wave id differs from the one the backlog proposed (`_implementation.md:1709` suggests
`2026-08-19-intake-phase-9`). The directory is named for the day the ingest runs, which is the
20th; the suffix the backlog cared about — *"an overwritten wave directory destroys an earlier
wave's audit trail"* — is preserved either way.

## The corpus, as verified

```
ls .kb/decisions      →  27 atoms (0001–0022, 0029–0033) — 25 accepted, 2 superseded (0002, 0021)
ls .kb/open-questions →  24 atoms, all authority_tier: note — 20 accepted, 4 superseded
ls .kb/reference      →   8 · playbooks 6 · maps 3 · concepts 1 · governance 1
ls .kb/product .kb/design → READMEs only; nothing routes there this wave
                         70 atoms + 8 layer READMEs, before this wave
ls references/adr     →  25 records: 0001–0023, 0029, 0030
```

| Layer | Atoms | Bearing on this wave |
| --- | --- | --- |
| `decisions/` | **27** (25 accepted) | **Two are added** (ADR-0023, ADR-0034). **No body is edited and no `status` is flipped anywhere** — the wave performs no supersession of a decision |
| `open-questions/` | 24 (20 accepted) | **Two are created**; **two are resolved** (CF-40, WF-11 — metadata + a dated section, bodies verbatim); **two are annotated and stay Open** (ES-6, post-phase reconciliation); **one gains one dated sentence** (poll-count) |
| `reference/` | 8 | **Two are added**. **`wire-format-encoding-measurements` is deliberately not extended** — the dating rule, below |
| `maps/` | 3 | All three inherit work via `mapsImpact` — `decision-map` two rows, `domain-map` a Cloudflare-adapter section, `open-questions-index` six bullet changes |
| `playbooks/` | 6 | `one-decision-per-adr-title` is **cited by ADR-0023 in its own body**, which is new: the atom invokes the playbook's stated exception rather than the wave invoking it on the atom's behalf. **Cited, not merged** — fifth wave running |
| `governance/` | 1 | `kb-governance-referent-not-reasoning-001` is what makes ADR-0023's refusal of `RUNBOOK.md:4394-4396` a citation rather than an edit. Cited, not touched |
| `concepts/` | 1 | No bearing this wave |

For authority purposes:

- **Twenty-five accepted decision atoms exist (of twenty-seven) and the intake touches ten of
  them by name** —
  `kb-decision-0001` (the two port flavours, whose provisional marker the runbook wrongly asks
  to be lifted again), `kb-decision-0003` (the opaque-payload boundary, which WF-11 confirms and
  does not test), `kb-decision-0008` (which already recorded the ADR-0001 lift),
  `kb-decision-0009` (the unbounded `Error` and its marker — the ES-6 verdict's subject),
  `kb-decision-0011` (ceiling-and-page), `kb-decision-0012` (CF-39 / `MID_BATCH_FAULT`),
  `kb-decision-0015` (which mints CF-40), `kb-decision-0016` (WF-11 itself),
  `kb-decision-0022` (which records CF-40 as a non-verdict, and which states no clause range),
  `kb-decision-0013` (POLL_BUDGET §8, via the CF-40 resolution's sub-question 3).
  **Not one of them is edited, flipped, or superseded.** Every incoming claim about them is
  either evidence *behind* an already-accepted decision or a new decision that cites them.
- **Zero supersessions.** This is the first wave since 2026-08-13 to perform none, and it is
  worth stating because two intake files invite one: `RUNBOOK.md:4394-4396` asks for ADR-0001's
  marker to be retired (already done at phase 1 by ADR-0008 — the instruction is stale), and
  the ES-6 verdict opens by warning that `kb-decision-0009` must be *"byte-identical to `main`
  after the wave."* Both refusals are recorded inside ADR-0023's own body, which is the point:
  an instruction that invites an edit to an immutable atom will invite the same edit from the
  next reader unless the record says it was read and not obeyed.
- **Two claims ask for a verdict the wave cannot sign.** ADR-0023's *"the atom must choose one
  and say which"* on `deny.toml`, and the backlog's third travelling question (the store-limit
  numbers). The first becomes an `open_question`; the second is not adjudicable from anything
  staged and is carried in `unresolved`. `02`, Adjudications 4 and 8.

## Provenance check

Every path the five files name was tested against this worktree. The check changed nothing
material and corrected one figure, which is why it is short and is still reported.

```
references/adr/0023-the-sqlstorage-mapping-and-the-off-tokio-harness.md  ✓  exists — the number is fixed
.bklg/…/_plan.md:169                                          ✓  "ADR-0023 accepted; CF-40 and WF-11 atoms resolved"
.bklg/…/_implementation.md:1629-1635                          ✓  three questions ADR-0023 owns, named
.bklg/…/_implementation.md:1701-1707                          ✓  "CF-40 resolves to Branch B" — HS-P0012 did not mint it
.kb/decisions/0022-append-condition-strategy.md:30, :93-94    ✓  CF-40 recorded as a non-verdict, twice, verbatim
.kb/maps/open-questions-index.md:170-173                      ✓  the CF-40 bullet still reads **Open**
crates/happenstance-cloudflare/src/{sql_storage,host,lib,event_store}.rs  ✓  all four exist
crates/happenstance-cloudflare/tests/{durable_object_conformance,fixture_contract,wf11_memory_ceiling}.rs  ✓
crates/happenstance-cloudflare/tests/support/                 ✓  the fixture lives here
xtask/src/proof.rs:854-857                                    ✓  the four es6_reconstruction test names, exactly
xtask/src/proof.rs:865                                        ✓  WASM_RUNNER = "wasm-bindgen-test-runner"
deny.toml [bans]                                              ✓  wrappers = ["wasm-bindgen-test"] and nothing else
.kb/decisions/                                                ✓  no 0023-*.md, no 0034-*.md; highest taken is 0033
```

Three things the check turned up, each of which changes what an atom may say:

1. **`xtask/src/proof.rs:589-600` says "Three rows", not one.** The registry of executed
   `wasm32` targets carries three; `every-rule-under-workerd` **added one** — the Cloudflare
   conformance target — and *"inherits the runner wiring, the version check, the exhaustive
   enumeration check and both gate steps without writing any of them again."* ADR-0023's intake
   says *"driven by one row in `xtask/src/proof.rs`'s executed-target registry"*, which is true
   of the addition and false of the registry. **The atom must say "one row in a three-row
   registry", or say nothing about the count.** This is the wave's only corrected figure, and it
   is exactly the class of defect the phase-8 census in Op 10 is about — a count written beside
   a list.
2. **`deny.toml`'s own prose already frames the pending decision, and frames it as pending.**
   *"a second route into the graph — a manifest that names the crate itself, or any other crate
   that pulls it in — is a new wrapper this list does not carry, and the check fails until
   **someone decides it should**."* So Op 2 is not the wave inventing a question; it is the wave
   giving a home to one the repository wrote down in a config file, where no map can find it.
3. **ADR-0024 through ADR-0028 are all reserved** by planned projects (`_plan.md:196, 229, 230,
   257`, `_storymap.md:64`). The CF-40 resolution therefore takes **ADR-0034**, highest-taken
   plus one, and does not backfill. `02`, Adjudication 2.

## Scoring method

Unchanged from all five previous waves — a threshold that moves between waves is not a
threshold. A 0–100 judgement of **subject identity**: would a reader looking for one claim
expect to find the other in the same atom.

- **≥ 80 — same knowledge.** Collapse into one atom. Never two.
- **50–79 — same subject, different method or different settling condition.** Two atoms,
  linked, and the reason written down.
- **< 50 — related.** A `related` link and nothing else.

Rules carried forward, each with what it did this wave:

- **A README is not a merge target** (wave 1). Exercised once, on `cf40-c5`, which the extract
  routed to `.kb/decisions/README.md` at lexical 80. That README is the authority being obeyed.
  **No operation.**
- **A supersession chain is never a merge** (wave 2). Not exercised — the wave performs no
  supersession.
- **A decision naming a gap is not the same knowledge as the question that owns the gap**
  (wave 3). Exercised twice, and it is what produces Ops 1 and 2 rather than two more paragraphs
  inside ADR-0023.
- **An accepted decision atom is not a merge target, however high it scores** (wave 4).
  Exercised on `kb-decision-0009` (ES-6 verdict, 90), `kb-decision-0003` (WF-11 C7, 45),
  `kb-decision-0015` and `kb-decision-0012` (CF-40, 65 and 45), `kb-decision-0022` (0034's
  finding 2, 45). **Five atoms scored against, zero operations on any of them.**
- **An ADR number is allocated by the artefact that already carries it** (wave 5). ADR-0023's
  record exists on disk; nothing allocates a number for the CF-40 resolution, so it is
  highest-taken + 1.
- **New this wave: an intake file's numeric prefix is a staging sequence and never an ADR
  number, restated because this wave is where it would do damage.**
  `.kb/_intake/0034-what-the-phase-8-reconciliation-cost.md` mints **no ADR at all** and says so
  in its second sentence — *"nothing here should be ingested as a decision"* — while
  `.kb/decisions/0034-…` is, independently, where the **CF-40 resolution** lands. Two different
  things wearing the same four digits in one wave. `02`, Adjudication 2.
- **New this wave: a resolution may supersede a question whose sub-questions are not all
  answered, provided every unanswered residual is given a named home.** WF-11 is the case, and
  it is the wave's most contestable call. `02`, Adjudication 5.

## Merge candidates against the existing corpus

| Existing atom | Incoming claim | Score | Disposition |
| --- | --- | --- | --- |
| `kb-decision-0009` | `es-6-…` claim-1/2/3/6 — ADR-0009's prediction judged against a real `!Send` error, and holding | **90** on subject, **0 actionable** | **no operation.** Accepted, immutable, and the intake opens by demanding it stay byte-identical to `main`. The verdict is *evidence behind* an accepted decision, which is a citation, not an amendment. Folded into Op 4 |
| *(new)* `kb-decision-0023` | `es-6-…` claim-4 — `JsThrow` retained, not stringified | **95** | **merge, and it is the wave's clearest.** ADR-0023's Consequence 1 carries this bullet already and ends *"See the ES-6 cluster, staged separately."* Two atoms here would each cite the same four tests |
| `kb-open-question-es-6-unwritable-rule-001` | `es-6-…` claim-5 — `store_error_crosses_a_join_handle` is still unwritten and unowned | **92** | **merge_existing, and explicitly NOT a resolution** — Op 8. The intake says *"Do not let the wave read this document as answering that question"*, and `_implementation.md:1716-1718` makes the same refusal a gate condition |
| `kb-reference-port-traits-compiled-findings-001` | `es-6-…` claim-2/3 — the four reconstruction tests | 48 | **link only.** That atom is the corpus's register of what the **compiler** said about the two flavours. These are executed `wasm32` tests about an error's *contents*. Same crate, different instrument |
| `kb-decision-0001` | `0023` C2/C4 — `exec` is synchronous, and the bare flavour observed under execution | **70** | **link only, twice over.** Accepted and immutable; and the marker the runbook asks to retire was retired at phase 1. `depends_on` on Op 4 |
| `kb-decision-0008` | `0023` C4 — ADR-0008 records the lift | 55 | **link only.** ADR-0008 already says it. ADR-0023 cites it as the reason not to obey `RUNBOOK.md:4394-4396` |
| `kb-decision-0011` | `0023` C2 — ceiling-and-page keeps a cursor off a suspension point | **72** | **link only.** ADR-0011 decided the read shape; ADR-0023 is the first storage that made the shape load-bearing. Same subject, different settling evidence — squarely 50–79 |
| `kb-decision-0022` | `0023` C2b — `index_arms()` rejected for the same reason one adapter over | 58 | **link only.** ADR-0022 rejected it for `happenstance-sqlite`; a second adapter agreeing is a *second instance*, and ADR-0022's own named re-open trigger is two adapters **needing** it, not two declining it |
| `kb-open-question-cf-40-ownership-001` | `cf-40-…` c1/c3 — the whole resolution | **95** | **resolve, not merge-in** — Ops 5 and 6. The answer is a new atom; the question keeps its body verbatim and moves to `superseded`. `.kb/open-questions/README.md:41-45` |
| `kb-decision-0015` | `cf-40-…` c3(1) — CF-40 is ADR-0015's clause | **65** | **link only, and the ruling goes in the new atom.** ADR-0015 is where the contradiction physically is (`kb-open-question-cf-40-ownership-001` quotes both halves), and it is accepted. A wave that "clarified" ADR-0015 would be resolving its self-contradiction by editing one half out |
| `kb-decision-0012` | `cf-40-…` c3 — CF-39 owned by adjacency | 45 | **link only.** The other claimant, unedited |
| `kb-decision-0022` | `cf-40-…` c2 — the Branch B coordination result | 50 | **link only.** ADR-0022's non-verdict is *quoted* in Op 5's Context as provenance. Quoting an immutable atom is how a later atom shows it did not overwrite one |
| `kb-open-question-poll-count-rule-strength-001` | `cf-40-…` c4 — sub-question 3 stays open; phase 10's `POLL_BUDGET` is the next test | **75** | **merge_existing, one sentence, stays Open** — Op 9. That atom already quotes ADR-0013 saying the `POLL_BUDGET` call *"should be made by whoever owns the fixture contract, with an adapter in front of them."* Op 5 answers precisely that clause: **there is no such owner, and the decision needing the capability mints it.** A question carrying an unanswered premise the corpus has since answered is a question that gets re-derived |
| `kb-open-question-human-readable-encoding-limits-001` | `wf-11-…` C1/C5 — the peer condition now has a measured answer | **95** | **resolve** — Ops 3 and 7. Same shape as CF-40 |
| `kb-reference-wire-format-measurements-001` | `wf-11-…` C4 — the published cost table reproduced exactly on `wasm32` | **88** on subject, **0 actionable** | **link only, and this is the wave's one refused high scorer.** `.kb/reference/README.md:19-27` and `:35-38`: *"Every atom here is a statement about a moment"*, and *"a reference atom is a snapshot."* That atom is 2026-08-09, `experiments/wire-format/`, host toolchain. This is 2026-08-19, `wasm32-unknown-unknown`, inside the gate. **Corroboration of a snapshot is a second snapshot, not an edit to the first** — the identical refusal wave 4 made in creating `kb-reference-spec-trace-has-suite-001` and wave 5 made twice more. Op 3 carries the reproduction and cites the original by id |
| `kb-decision-0016` | `wf-11-…` — WF-11 is ADR-0016's clause | 60 | **link only.** Accepted; and the intake forbids the atom to say the marker moves |
| `kb-decision-0003` | `wf-11-…` C7 — sub-question 3 answered: no bearing | 45 | **link only, and the claim is a non-claim.** *"The probe forwards bytes it never inspects … the boundary is observed here, not tested."* An accepted decision confirmed by not being touched receives nothing |
| `kb-reference-phase-4-5-spec-reconciliation-001` | `0034-…` c1/c4 — the phase-8 census | **70** | **create_new, sibling not append** — Op 10. Same instrument, same criterion, **different pass at a different commit**, and the reference README's dating rule again. The 401/80 count is only meaningful *against* that atom's 358/69, which requires two atoms to compare |
| `kb-open-question-post-phase-reconciliation-001` | `0034-…` c2/c3 — evidence for sub-questions 1, 3 and 5 | **78** | **merge_existing, stays Open** — Op 11. One point below the collapse threshold, and the file says outright *"it settles nothing … the two arguments below are inputs to an ADR that does not exist yet"* |
| `kb-decision-0022` | `0034-…` c2 — ADR-0022 states no clause range in any form | 45 | **link only, and no operation.** Accepted. The finding is about a *missing* frontmatter key on an immutable atom, which is a schema question, not a defect the atom may be edited to fix |
| `kb-open-question-adr-status-vocabulary-001` | `0034-…` c2 — a proposed `clauses:` frontmatter key | 42 | **link only.** Same shape (the schema lacks a key the corpus wants), two different keys and two different settling conditions |
| `kb-playbook-ratchet-gate-landing-001` | `0034-…` c4 — no committed citation baseline was written | 45 | **link only** — fifth wave running. The playbook already carries the rule; a fifth instance of obeying it adds none |
| `kb-playbook-anchoring-citations-001` | `0034-…` c1 — six wrong-subject citations, none reported | **62** | **link only.** The playbook is the *method*; Op 10 is a dated count of how far the method has been applied. Two atoms, linked — and the census is what would eventually justify amending the playbook, on a second corroborating pass rather than this one |
| `kb-playbook-one-decision-per-adr-title-001` | `0023` C1 — the title's "and", and the exception | **80** on subject, **0 actionable** | **no operation, and the reason is unusual.** The claim is ADR-0023 *citing the playbook at itself*. Merging it would fold an instance of compliance into the rule being complied with — the same move `.kb/decisions/README.md` was protected from in wave 5 |
| `.kb/decisions/README.md` | `cf-40-…` c5 — "neither ADR-0015 nor ADR-0012 nor ADR-0022 may be edited" | 80 lexical | **no operation.** The authority being obeyed. Wave 1's rule |
| `.kb/open-questions/README.md` | `cf-40-…` c1, `wf-11-…` C1 — the *Resolving one* procedure, restated by two files | 95 lexical | **no operation, and the procedure is executed rather than recorded.** Ops 6 and 7 *are* this claim |

## Cross-file clusters

Five files, and **three genuine cross-file clusters**. Two would have produced duplicated
knowledge if the files had been ingested independently.

| Cluster | Files | Claims | Score | Disposition |
| --- | --- | --- | --- | --- |
| **CL-1** — the ES-6 verdict is ADR-0023's evidence | `0023-…`, `es-6-…` | C2b (JsThrow bullet) / claim-1, 2, 3, 4, 6 | **95** | **one atom.** Op 4 |
| **CL-2** — the harness cannot model the platform | `0023-…`, `wf-11-…` | C3 ("what this does not prove") / C2 ("the runner does not model it") | **85** | **one open question, sourced from both.** Op 1 |
| **CL-3** — CF-40's third data point is ADR-0023's fixture | `0023-…`, `cf-40-…` | C6 ("resolved elsewhere") / c3 (`CloudflareFixture`) | **35** | **two atoms, one `related` edge.** Ops 4 and 5 |

### CL-1 — the strongest merge signal any wave has received

`0023-…:80-84` carries the claim and names the other file:

```
- **`JsThrow` versus `StringifiedThrow`.** The thrown value is retained rather than
  stringified at the boundary. Stringifying early is what would have made ES-6's question
  unanswerable … See the ES-6 cluster, staged separately.
```

and `es-6-…:62-67` carries the reciprocal — *"The consequence for the mapping, which is why
this belongs beside ADR-0023."* Two documents, one claim, each pointing at the other. The
ES-6 file's own header leaves the layer to the adjudicator and states its bias: *"biased per
`.kb/_intake/README.md` toward folding it into ADR-0023 rather than spawning a near-duplicate,
since it is settled by the same run."*

**The one argument against, and why it loses.** `.kb/decisions/README.md:36-39` says evidence
is a `reference` atom the decision cites, and that is what split ADR-0022 from its experiment
in wave 5. It does not split here, and the difference is not size: the append-condition
experiment produced **a table of numbers with its own re-measurement schedule**, so the numbers
were expected to be superseded on a cadence the decision did not share. The ES-6 verdict
produces **four named tests, committed, pinned in a gate registry that fails if they are
renamed or emptied** (`xtask/src/proof.rs:854-857`). There is no number to go stale and no
second decision waiting to reuse it. Splitting it would produce a reference atom whose entire
content is four identifiers and a sentence, and a decision atom that could not be read without
it.

### CL-2 — the wave's real dedup, and the one that was nearly missed

Neither file proposes an atom for this. ADR-0023 states the gap as a **negative scope note** —
*"No isolate, no eviction, no hibernation, no I/O gate, no event loop re-entering the object
mid-`await`, and none of the platform's own storage ceilings"* — and separately escalates the
`workerd` runner as *"not rejected on merit … an escalated blocking finding."* WF-11 arrives
at the same gap from the far side, having **fired a falsifier at it and watched it not bite**:
the staircase asked for 2,047 pages, the host granted 2,169 pages = 142,147,584 bytes, past
Cloudflare's documented 128 MiB, and refused nothing — *"a Node isolate has no per-isolate
memory cap, so the one property this falsifier needs is exactly the property the runner does
not model. This is a second, independent consequence of `every-rule-under-workerd`'s escalated
blocking finding rather than a new problem."*

`.bklg/…/_implementation.md:1738-1740` confirms it was left unsettled on purpose: *"Whether
(c) is an acceptable answer for AC-011, or a deferral until a real runner exists, is plausibly
ADR-0023's call rather than the implementer's. It was not settled here."*

So: **one gap, two observed consequences, no owner, and a real tradeoff** (no Windows-native
story and Node-lockfile versioning against everything the harness cannot prove). That is the
open-questions layer's third bullet exactly — *"a deferral, where a decision was possible but
was consciously postponed, and what would force the choice."* Op 1 owns it; Op 3 and Op 4 both
point at it rather than each describing it.

### CL-3 — adjacency, not identity

ADR-0023's scope note says CF-40 is *"discharged or resolved elsewhere"*, and the CF-40 file
uses ADR-0023's `CloudflareFixture` as its third data point. That is two atoms referring to
one artefact, not two descriptions of one piece of knowledge — 35, well under the band. The
`related` edge is the whole of it, and Op 5 must be authored **after** Op 4 so the id it names
exists.

## The dedups that were refused, and the ones that were not offered

| Group | Score | Ruling |
| --- | --- | --- |
| WF-11's `wasm32` reproduction appended to `kb-reference-wire-format-measurements-001` | **88** | **Refused — the highest score this wave declines.** The reference README's dating rule, in terms. Op 3 reproduces the figures *as a second observation on a second runtime* and cites the first by id. If the two are ever folded, the fold destroys the only thing the reproduction proves: that two independent runtimes agree |
| Op 10 (the phase-8 census) appended to `kb-reference-phase-4-5-spec-reconciliation-001` | 70 | **Refused.** Same rule, and a stronger reason: 401 checked / 80 anchored is a number whose entire meaning is *comparison against* 358 / 69 at an earlier commit. One atom holding both counts is a mirror someone must keep current; two atoms are a series |
| Op 1 (the `workerd` gap) and Op 2 (the `deny.toml` ban) as one "the `worker` dependency's costs" atom | 40 | **Refused, and it is not close.** One is about a **runner** the gate does not have; the other is about a **transitive dependency** the gate's own ban forbids. Different settling conditions entirely: Op 1 settles when someone decides whether a `workerd`-class runner may enter `cargo xtask ci`; Op 2 settles the moment someone writes a `wrappers` line or refuses to. They share a crate name and nothing else |
| The ES-6 verdict as its own `reference` atom | 55 | **Refused** — CL-1. The intake offers this shape itself and states its own bias against it |
| The CF-40 resolution folded into ADR-0023 | 35 | **Refused, on two independent grounds.** ADR-0023's own scope note disclaims it (*"CF-39, CF-40 and WF-11 are … resolved elsewhere"*), and `kb-playbook-one-decision-per-adr-title-001`'s exception — which ADR-0023 invokes for its own title — **does not extend to CF-40**: ADR-0023's two halves rest on one conformance run, while CF-40's ownership rests on a three-ADR pattern observed over three phases. Different evidence, so different atoms. `02`, Adjudication 3 |
| The three store-limit numbers as an atom of any kind | — | **Not offered by any file, and the wave does not invent one.** `_implementation.md:1631-1632` names it as one of the three questions ADR-0023 owns — *"declare `None` with the falsification finding, hold, or ratify the 'declared refusal policy' reading"* — and **no staged document adjudicates it**. The CF-40 file reports the numbers as declared and argues nothing. Carried in `unresolved`; slice 3's owed review is where it surfaces |
| `RUNBOOK.md:4394-4396` (the stale ADR-0001 instruction) edited by this wave | — | **No atom, no op.** `RUNBOOK.md` is mutable and is not an atom. ADR-0023's body records that the instruction was read and not obeyed, which is the durable half. `02`, *Standing choices* 6 |
| Project AC-005's wrong wording corrected | — | **No `.kb/` op.** *"correct it in the record rather than answer the question it asks"* — the record is `.bklg/`, and a backlog spec is not a KB atom |
