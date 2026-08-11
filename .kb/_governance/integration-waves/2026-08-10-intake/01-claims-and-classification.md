# Wave `2026-08-10-intake` — claims and classification

Every claim the extract pass raised, labelled against the accepted decision corpus, with the
destination it was routed to. Twenty-nine claims in, twelve atoms out.

## What "against the accepted decision corpus" means this wave

The corpus holds **no decision atoms at all** (`00-corpus-match.md`). The labels are therefore
used as follows, and the usage is stated so a later wave reading this table is not misled into
thinking a decision was consulted:

| Label | Meaning here |
| --- | --- |
| `aligns` | Restates a rule the corpus already carries in its own governance prose. Nothing new is committed; the atom grounds the rule rather than introducing it. |
| `extends` | Net-new knowledge with no owner and nothing to contradict. The wave's default. |
| `conflicts` | Contradicts an accepted decision atom. **Zero occurrences** — there is nothing to contradict. |
| `requires-new-decision` | The claim cannot be discharged by recording it: an ADR is needed, and this wave is not authorised to write one. Routed to `open-questions/`. |

The repository's real decision record lives in `.kb/decisions/` and is **not** mirrored into `.kb/`.
It was read for context and treated as non-authoritative for placement, because the authority
rules govern `.kb/` atoms. Where a claim names an ADR (gap 5 names ADR-0009 as accepted; the
frozen-clause lesson names ADR-0029 as the standing example), the reference is preserved in the
atom body and no `.kb/decisions/` atom is minted from it. Importing the ADR corpus is a separate
wave with a human in it.

## The claim table

### `2026-08-10-phase-4-5-pressure-test.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | Extract the pointer, not the evidence — the 50KB evaluation document must not be copied into an atom | authoring imperative | `aligns` | shapes CL-1; the reference README already states the pointer rule |
| `c2` | Placement advice: KB root, or `reference/` if one exists | "should" | `aligns` | superseded by fact — `reference/` now exists and is scaffolded |
| `c3` | What the pass was: six commits `3c704d3`…`84dcc67`, each green, no phase owned it | none | `extends` | CL-1 |
| `c4` | Census by defect class: 9 / 5 / 254-of-338 / 6 / 16 / 2, with owning commits | none | `extends` | CL-1 |
| `c5` | `spec-trace` as of 2026-08-10: 200 clauses (139 FROZEN), 95 rules, 358 citations, 69 anchored | none | `extends` | CL-1 (**merge anchor** — see `00`) |
| `c6` | `review-citation-drift.md` reached the same root cause independently; convergence is itself the finding | none | `extends` | CL-1 |
| `c7` | The pass wrote no ADR; seven findings recorded, not decided | none | `extends` | CL-1, as outbound links to CL-6…CL-12 |

### `gaps-owed-a-decision.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `gap-0` | "This file must yield six atoms, not one. Do not merge them." | "must" (process) | n/a | honoured — six atoms; see `00` for the refused merge |
| `gap-1` | `k_disjoint_boundaries_admit_exactly_k_commits` enforces DCB independence; no clause states it; ES-25 is the wrong attachment point | quoted only | `requires-new-decision` | CL-6 |
| `gap-2` | `ops_agree_with_the_model` checks the composition of seven clauses and belongs to none | none | `requires-new-decision` | CL-7 |
| `gap-3` | PS-1's `[FROZEN]` MUST is a coupling, not a progress obligation; the third assigned rule does not follow | quoted | `requires-new-decision` | CL-8 |
| `gap-4` | PS-19's MUST is scoped after a reset; `fresh_projection_has_no_checkpoint` asks about an unseen id | quoted | `requires-new-decision` | CL-9 |
| `gap-5` | ES-6 is `[FROZEN]` naming `store_error_crosses_a_join_handle`, which exists as no `fn`; check 4's `(new)`/`†` escape hatch has no expiry | "should probably" | `requires-new-decision` | CL-10 |
| `gap-6a` | ES-7's `[PROVISIONAL]` falsifier cannot be produced by its own named instrument | quoted "may" | `requires-new-decision` | CL-11 |
| `gap-6b` | VT-9's falsifier has already occurred (`wasm32` has no clock) and the clause is unharmed | quoted | `requires-new-decision` | CL-11 |

### `lesson-a-check-that-verifies-the-address-not-the-referent.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `c1` | Two obligations: verify the referent, and report your own coverage. Print the denominator; name the declined population; a narrowing filter is a silent scope reduction; suspect a check that has never failed | "must" ×2 | `extends` | CL-2 |
| `c2` | The measurement: 84 of 338 parsed, `2e4407b`'s eight green-and-wrong citations, `a843b99` → 358 checked, the summary line as the durable change | none | `extends` | CL-2 as evidence; the shared figures are owned by CL-1 |

`c1`'s two "must"s are **not** a project commitment. They prescribe how to build a checker, in
general, and the repository has already built it — nothing about the contract, the ports or the
gate changes if a future checker declines the advice. Playbook, not decision. (`.kb/playbooks/README.md`:
the test is whether a future change would need a decision to reverse it.)

### `lesson-anchoring-citations-in-a-long-lived-document.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | P1/P2, content hash rejected, windowed subject search, explicit versus derived anchors, four measured attempts (118/316 → 2/262), the decline-don't-guess discriminator | "must" ×2 (design properties) | `extends` | CL-3 |
| `C2` | Line-numbered citations across documents are a standing tax; three breakages in one pass | "should be priced in" | `extends` | CL-3, as the "stops holding" clause |
| `C3` | `ANCHOR_SLACK` is 12 in `spec_trace.rs` and 10 in `lint_constitution.rs`, and a doc comment says they match | explicitly "not an ADR" | `extends` | **CL-1**, not CL-3 — a one-off dated fact belongs in `reference/` |

### `lesson-landing-a-stricter-gate-without-a-red-baseline.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | Widening check 6 surfaced six unclaimed rules; options A (wait) and B (report-only) refused, with reasons | none | `extends` | CL-4 |
| `C2` | The ratchet's four properties: fatal from day one, printed on every green run, discharge-is-a-failure, evidence per entry | none | `extends` | CL-4 |
| `C3` | Never write a count of a list beside the list — a self-referential number is a citation and rots like one | generalised imperative | `extends` | CL-4 |
| `C4` | A check and the corpus it checks must land together iff either half alone leaves the tree unverifiable | **"must" ×2** | `extends` — flagged | CL-4; see below and `unresolved` |
| `C5` | A ratchet suits single-digit, individually-argued exceptions; a legacy corpus wants a decreasing threshold with a deadline | none | `extends` | CL-4, as the "stops holding" clause |

**`C4` is the wave's one genuine modal question.** Authority rule 2 sends new commitments to a
decision atom. Three things argue it is not one, and they were weighed rather than assumed:
the rule is conditional ("if and only if either half alone would leave the tree unverifiable"),
which is the shape of a method and not of a commitment; it binds no interface, clause or crate,
so no ADR is needed to reverse it; and the decisions README requires an `adr_id`, the rejected
alternatives, and a human sign-off that an ingest wave cannot supply. Minting a decision atom
here would also be the exact move the frozen-clause lesson's own `C3` argues against — deciding
in the pass that discovered. Placed as playbook; the stricter reading is carried in `unresolved`
so the runbook's ADR pass can promote it deliberately rather than inherit it silently.

### `lesson-repairing-a-frozen-clause-without-amending-it.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `C1` | The mechanical test: a correction to a `[FROZEN]` clause is a repair iff the admitted implementation set is unchanged; otherwise a gap, and a gap is an ADR's. Plus both taxonomies | restates CLAUDE.md | **`aligns`** | CL-5 |
| `C2` | The safe form for a discharged MUST: keep it verbatim, name the discharge as a discharge, cite the code and the test | none | `extends` | CL-5 |
| `C3` | Why ADR authorship stayed outside the pass: a pass that discovers and decides cannot be audited; a decision taken to unblock a checker is taken for the wrong reason; ADRs have a described process | none | `extends` | CL-5 |
| `C4` | 200 clauses, 139 `[FROZEN]` | none | `extends` | **CL-1** — same measurement as pressure-test `c5` |
| `C5` | Pointer to the seven unscheduled findings | none | n/a | links only, no atom |

`C1` is the only `aligns` in the wave with a real referent: `.kb/decisions/README.md` already
states the repair-versus-amendment test in the corpus's own voice. The playbook does not
introduce the rule — it supplies the operating procedure and the worked examples the README has
no room for, and cites it.

### `open-question-nothing-owns-the-post-phase-reconciliation.md`

| Claim | Substance | Modal | Label | Destination |
| --- | --- | --- | --- | --- |
| `claim-1` | Self-placement: `open_question`, `authority_tier: note` | none | n/a | honoured |
| `claim-2` | What is true today: 16 false clauses, 9 undischarged doc MUSTs, 5 missing tests, 254 of 338 citations unparsed; no phase item obliged the read-back | none | `requires-new-decision` | CL-12 (figures cited from CL-1, not restated) |
| `claim-3` | The rule was already written three times in `RUNBOOK.md` and nothing implements it; the pass added a standing exit criterion at `:3810-3820` | quoted | `requires-new-decision` | CL-12 |
| `claim-4` | Three separable unknowns: per-phase checkbox, who computes the two numbers, what is mechanisable | none | `requires-new-decision` | CL-12 |
| `claim-5` | Forced by phase 6's exit (PS-1 – PS-37), secondarily by first publish at phase 12 | none | `requires-new-decision` | CL-12 |
| `claim-6` | Five ordered sub-questions | none | `requires-new-decision` | CL-12 |
| `claim-7` | Why a question and not a task: a real tradeoff would be smuggled into whoever picks up the ticket | none | n/a | CL-12, confirms the layer |
| `claim-8` | Add a bullet on the specification/governance map "once one exists" — no map atom exists | conditional | n/a | `mapsImpact`; see `unresolved` |

## Counts

| | |
| --- | --- |
| claims classified | 29 |
| `aligns` | 4 |
| `extends` | 16 |
| `conflicts` | 0 |
| `requires-new-decision` | 13 |
| accepted decisions touched | 0 |
| accepted decisions superseded | 0 |
| decision atoms authored | **0** — by design; ADR authorship belongs to the runbook's ADR pass |
| atoms out | 12 (1 reference, 4 playbook, 7 open_question) |
