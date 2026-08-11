# Wave `2026-08-10-intake` — retrospective

## What merged versus what was created

Nothing merged into an existing atom — the corpus had no decision atoms and only READMEs in the
layers this wave populated, so there was nothing accepted to extend by amendment. The one
merge-shaped decision in the wave was pre-integration: three intake files (`2026-08-10-phase-4-5-pressure-test.md`,
`lesson-repairing-a-frozen-clause-without-amending-it.md`,
`lesson-anchoring-citations-in-a-long-lived-document.md`) each stated the same dated measurement —
200 clauses, 139 `[FROZEN]`, the `spec-trace` summary line — and a second census atom proposed by
the extract pass was refused in favor of one, `kb-reference-phase-4-5-spec-reconciliation-001`,
that the other eleven atoms cite rather than restate (`00-corpus-match.md`, `02`'s Op 1
rationale). That refusal is the wave's clearest instance of the reference README's own rule
holding under pressure from three independent sources agreeing with each other.

Twenty-nine claims went in (`01-claims-and-classification.md`); twelve atoms came out. The
compression is concentrated in two places: the shared census (three source files, one atom) and
the six `gaps-owed-a-decision.md` findings, which the intake file itself insisted stay six atoms
rather than one (`gap-0`, honoured verbatim) and which this wave kept at six despite the closest
pair scoring 40/100 on similarity — shared provenance, not shared substance.

Four claims were labelled `aligns` against existing governance prose (`.kb/decisions/README.md`,
`CLAUDE.md`). Only one, `lesson-repairing-a-frozen-clause`'s `C1`, had a real referent — the
repair-versus-amendment test the decisions README already states. It still produced an atom,
because the atom supplies the operating procedure and worked example the README has no room for
and cites the source rather than duplicating it. The other three `aligns` labels were placement
advice superseded by fact (the layers they pointed at now exist) rather than substantive
agreement.

## Unresolved claims

**Thirteen claims labelled `requires-new-decision`, zero decision atoms authored.** This is the
wave's central and deliberate abstention, argued three ways in `02`'s Adjudications and grounded
in the ingested corpus's own content (`lesson-repairing-a-frozen-clause`'s `C3`): a pass that both
discovers a gap and decides it cannot be audited; a decision taken to unblock a checker, or to
close an ingest, is taken for the wrong reason; and `.kb/decisions/README.md` requires an `adr_id`,
recorded alternatives and a human sign-off that an ingest wave cannot supply on its own authority.
This mirrors the standing project instruction that ADR authorship stays with the runbook's own ADR
pass, never as a side effect of another task. All thirteen are now durably parked as
`open_question` atoms (seven distinct questions after dedup — Ops 6–12) rather than lost, each
naming its forcing event and, where relevant, its owning phase.

**`lesson-landing-a-stricter-gate-without-a-red-baseline`'s `C4`** ("a check and the corpus it
checks must land together iff either half alone would leave the tree unverifiable") was weighed
against authority rule 2 and placed as playbook rather than decision — the rule is conditional in
shape, binds no interface or crate, and minting a decision atom for it would itself be the
discovers-and-decides move `C3` argues against. Recorded here rather than silently promoted: a
future wave with more evidence may find this one under-called it.

**Two ADRs are named but not imported.** ADR-0009 (gap-5, es-6) and ADR-0001 (gap 6a/6b,
es-7-and-vt-9) are referenced by id in atom bodies with no corresponding `.kb/decisions/` atom and
no link, so no dangling id entered the corpus. Importing the `docs/adr/` corpus into `.kb/` is out
of scope for this wave and is its own future wave with a human in it.

## Follow-ups

- **Whoever picks up the PS-1 or PS-19 open question should check the other 35 PS clauses for the
  same shape** (a `[FROZEN]` MUST narrower than the rule table assigns it) before settling either
  one in isolation — carried into both atom bodies per `02`'s instruction, repeated here so it is
  not lost to whichever atom a reader lands on first.
- **The domain map and open-questions index are now established** (`kb-map-domain-001`,
  `kb-map-open-questions-index-001`), both first-wave. The next kb-ingest wave that touches
  specification governance should extend the existing "Specification governance & conformance"
  section rather than create a duplicate domain; a wave outside that subject should add a new `##`
  section per the maps' own "Adding a domain" / "Adding an entry" instructions.
- **`ANCHOR_SLACK` disagreement** (12 in `xtask/src/spec_trace.rs:391` vs. 10 in
  `xtask/src/lint_constitution.rs:111`, with a doc comment claiming they match) is recorded as a
  residual defect inside the census atom, not as an open question — the intake itself calls it "a
  one-line repair and not an ADR." It is a one-off dated fact, not a standing question, and stays
  discoverable through the census rather than the open-questions index.
- **No atom was routed to `.kb/product/` or `.kb/design/`.** Confirmed correct for this corpus (an
  event-sourcing library has no personas or interaction surfaces) rather than an oversight; noted
  so a future wave over this same intake style does not second-guess it without new evidence.
- **Citations inside `.kb/` atom bodies are not checked by any instrument** run in this wave —
  `redkiln validate --kb` checks frontmatter, ids and cross-atom links, not `file:line` citations
  embedded in prose. The census atom's own residual-defect citation (`xtask/src/spec_trace.rs:391`)
  was hand-verified by grep during placement, per the playbook (`kb-playbook-verify-referent-report-coverage-001`)
  this same wave just ingested — worth a future wave's attention if citation rot inside `.kb/`
  itself becomes a recurring finding.

## Instruments run

`redkiln validate --kb` — pass. `redkiln doctor` — pass (six template-drift warnings unrelated to
the KB). No repair loop was needed; both commands passed on the first run.
