# Wave `2026-08-17-adr-0022-append-condition` — integration summary

Fourteen operations from **six** intake documents — the largest wave in this corpus, and the first
to carry two unrelated ADR queues plus a defect log in one pass. Eleven atom files created (ten
counted in `newAtoms`, plus `kb-decision-0032` authored under Op 6's two-sided supersede), four
atoms mutated, three maps synced, twelve backlinks wired, zero deferred.

`redkiln validate --kb` **passes**. `redkiln doctor` **fails**, on nine pre-existing `.bklg` errors
that touch none of this wave's operations — see `04`.

## Atoms created

| # | destPath | id | kind | status |
| --- | --- | --- | --- | --- |
| 3 | `.kb/decisions/0022-append-condition-strategy.md` | `kb-decision-0022` | decision | accepted |
| 4 | `.kb/decisions/0031-the-runner-collapses-upward.md` | `kb-decision-0031` | decision | accepted |
| 6 | `.kb/decisions/0032-adr-0021-serde-attribution-correction.md` | `kb-decision-0032` | decision | accepted |
| 10 | `.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md` | `kb-decision-0033` | decision | accepted |
| 1 | `.kb/reference/append-condition-experiment-2026-08.md` | `kb-reference-append-condition-experiment-001` | reference | accepted |
| 9 | `.kb/reference/phase-7-macros-ceremony-measurement.md` | `kb-reference-macros-ceremony-measurement-001` | reference | accepted |
| 13 | `.kb/reference/projection-fan-out-costs-n-reads.md` | `kb-reference-projection-fan-out-cost-001` | reference | accepted |
| 2 | `.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` | `kb-open-question-es-17-two-adapter-measurement-001` | open_question | accepted |
| 8 | `.kb/open-questions/d-1-the-validated-type-has-no-total-path.md` | `kb-open-question-d-1-no-total-path-001` | open_question | accepted |
| 11 | `.kb/open-questions/cf-36-names-a-cross-reference-nothing-performs.md` | `kb-open-question-cf-36-unperformed-cross-reference-001` | open_question | accepted |
| 12 | `.kb/open-questions/no-ps-rule-name-is-resolved.md` | `kb-open-question-no-ps-rule-name-resolved-001` | open_question | accepted |

**Op 3 — ADR-0022, the headline.** The atom project DoD 3 of `sqlite-durable-store` (HS-P0012) has
been blocked on since run 2. `supersedes: null` and `superseded_by: null`, because it discharges no
clause. The intake's `related` carried a **dangling** id
(`kb-open-question-cf-40-fixture-limits-ownership-001`); the wave resolved it against the actual
sibling atom and wrote `kb-open-question-cf-40-ownership-001` instead. Its evidence deliberately
lives in a **separate** reference atom (Op 1) so the decision can later be superseded without
invalidating the measurements — which is what the intake's own two-file staging asked for, and what
`crates-io-name-and-packaging-facts`' AC-008 inverted-dependency test was written against.

**Op 4 — ADR-0031, and the correction that four waves carried.** The runner collapses upward into
`happenstance`; ADR-0007's falsifier at `references/adr/0007-projection-runner-decodes.md:118-121`
fired exactly as that record specified — no independent caller ever appeared, because
`happenstance::run_projection` drives the port directly and `happenstance-core` publishes no
module-level function that runs. The atom carries PS-32's corrected Context in its own body rather
than as a separate editorial artifact.

**The supersession is spelled narrowly, and against the intake's explicit instruction.** The intake
(`0032-…:104-106`) and `_implementation.md:704` both asked for `kb-decision-0007` flipped to
`superseded` with `superseded_by: kb-decision-0031`. The wave wrote `supersedes: null` and left
`kb-decision-0007` **wholly untouched**, on the corpus's own partial-supersession convention
(`.kb/maps/decision-map.md`, *"Reading the partial-supersession chain"*; `kb-decision-0007` itself
carries `supersedes: null` while partly superseding `kb-decision-0006`). Flipping ADR-0007 would
retire three shape decisions that are implemented and in force — `Query` as the only nomination
vocabulary, `Projection::Store` as an associated type, checkpoints per `(store, ProjectionId)` —
one of which `kb-decision-0030` `depends_on`. **Confirmed by the human on 2026-08-17**: the narrow
reading stands, on the ground that it is reversible in a single frontmatter edit and the wide one
is not.

**Ops 9, 10 — the macros verdict, and why it is not a supersession.** `kb-decision-0033` records
`happenstance-macros` out of scope for 0.1 as a *new* decision (`reversibility: high`, `phase: 7`),
not as a supersession of ADR-0020, because ADR-0020 asserts no `must` about the derive — nothing
accepted is overturned. The 29-range, 532-line ceremony/domain classification behind it went to its
own reference atom (Op 9) rather than into the decision, on the same evidence-separation rule as
Ops 1/3.

**Op 8 — one open question from two defects, in two documents.** The defect log's C1
(`QueryItem::new` fallible over already-validated inputs) and C2 (`DomainEvent::tags` total over
fallible `Tags`, an 81-line workaround in the worked example) were merged into a single atom,
because the intake itself names C2 *"D-1's other face"*. The dedup ran **across** intake files, not
only within them.

## Atoms mutated

| destPath | action | shape of the edit |
| --- | --- | --- |
| `.kb/open-questions/ps-32-adr-0007-context-correction-is-owed.md` | amended | 49 insertions, 3 deletions; body above the new section byte-for-byte unchanged |
| `.kb/decisions/0021-payload-evolution-and-codec-tag.md` | superseded | **3 frontmatter lines, 0 body lines** |
| `.kb/decisions/0032-…serde-attribution-correction.md` | superseded (authored) | the superseding atom, `supersedes: [kb-decision-0021]` |
| `.kb/reference/port-traits-compiled-findings.md` | amended | integrated, not pasted; all 8 existing `related` and 13 existing `source_paths` kept |

**The ADR-0021 supersession is the contract working as designed.** `kb-decision-0021` is `accepted`
and therefore immutable, so it received **only** the metadata flip — `status: accepted →
superseded`, `superseded_by: null → kb-decision-0032`, `last_reviewed` refreshed. `git diff --stat`
on that file is three insertions and three deletions, zero body lines. The correction itself lives
in a new atom, which is the only place it may live.

Worth recording because it is the kind of thing that silently double-writes: two of those three
lines were **already applied** when the serial mutate op opened the file — Op 6 had flipped them as
part of its own two-sided write. The op verified the pre-existing pair rather than re-applying it,
and contributed only the `last_reviewed` refresh. Serial ordering of mutating operations is what
made that observable instead of a conflict.

## Maps and backlinks

Three map atoms synced — `decision-map.md`, `domain-map.md`, `open-questions-index.md` — and twelve
reciprocal backlinks wired, so every new outbound edge has its inbound counterpart. New-atom
integration wrote **outbound-only** links by design; the inbound half is the Maps phase's, which is
what lets the eleven creations run in parallel without racing on shared files.

**One map decision was deliberately not taken.** `.kb/maps/domain-map.md` has sections for
specification governance, contract ports/conformance and the typed layer, and none for phase 8 /
adapter storage. The wave declined to choose between opening a new domain and widening *"Contract
ports, conformance, and the ADR corpus"* for `kb-decision-0022`,
`kb-reference-append-condition-experiment-001` and `kb-reference-projection-fan-out-cost-001` — a
domain section is the map's own vocabulary, and inventing one silently is how a map stops matching
the corpus. Flagged rather than guessed. **Owed: one placement decision.**

## The long-form amendment, performed in this wave

`references/adr/0007-projection-runner-decodes.md:36-38` carried the sentence PS-32 rejects, and
three prior waves closed the *record* of the obligation while the sentence stood. This wave
performed it, by human decision at the finish-by-hand gate.

The form matters and was forced by the corpus. `spec/SPECIFICATION.md` cites that record at
`:44-50`, `:62-67` and `:76-81`, and two backlog specs cite `:62-67` and `:118-121`; `cargo xtask
spec-trace` is a gate step. **Any insertion above those anchors would silently re-point every one of
them.** So the marker at `:38` was written *into an existing line* — line count preserved — and the
substance was appended below `:121`, past the last citation in the file. All five anchors were
verified byte-identical afterwards.

The correction **annotates and does not rewrite**: the 2026-06 paragraph stands exactly as authored,
per `.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59`, and a new `## Correction`
section states what is now known. That is the same discipline as `ce933d8`, *"keep the record the
atom is not"*.

`RUNBOOK.md` carried the derived claim in one place — `:4025`, *"layered over the checkpoint pump
that stays in the contract crate, per ADR-0007"* — now corrected in place, line count preserved.
`RUNBOOK.md:3885` was **already** correct (*"A callback-driven pump can be written against the port
as it stands — that was compiled"*), so only one of the two sites needed the edit.
