# Wave `2026-08-10-intake-2` — retrospective

## What merged versus what was created

This is the wave the previous one (`2026-08-10-intake`) named and deferred: *"Importing the ADR
corpus is a separate wave with a human in it."* Its seventeen intake files are byte-identical
copies of files still canonical in `.kb/decisions/` (`00-corpus-match.md`, provenance check), so the
wave's dominant shape is transcription, not discovery — 111 claims classified, 17 decision atoms
authored, and **zero of those seventeen is a decision this wave took**. Each was written, argued
and accepted before this wave existed; `.kb/decisions/` stays the canonical record and is untouched.

`.kb/decisions/` was empty going in, so nothing merged into an existing decision — there was
nothing accepted to amend or supersede against. The two `merge_existing` operations both landed on
`open_question` atoms the previous wave left explicitly waiting for this one: ES-6
(`kb-open-question-es-6-unwritable-rule-001`) and the two provisional falsifiers
(`kb-open-question-provisional-falsifiers-001`), each amended by links plus a paragraph, with no
existing sentence deleted — an open question's value is the state of knowledge on the day it was
filed, and neither question closes: ES-6 is now writable and still unwritten, and moving a
`[PROVISIONAL]` marker is an ADR's act, not an ingest wave's.

111 claims in, 36 atoms out (`01-claims-and-classification.md`, Counts). The compression is
concentrated in three places: the largest single merge is Op 1
(`kb-reference-port-traits-compiled-findings-001`), five source files collapsed into one reference
atom at a 95 similarity score because three of them state the same compiler-observed finding in
the same words — two copies of a measurement is two things to update and one that quietly goes
stale, and here it would have been three. The four `conflicts`-labelled claims (`01`, "The four
conflicts") each resolved without a winner being picked: a duplicate correction merged into Op 1
rather than filed twice (CL-A/#4), a self-contradiction in ADR-0015 about CF-40 ownership parked
as an open question with both decision atoms linking to it rather than one silently overriding the
other (CL-H/#2), and ADR-0011's correction of ADR-0001's `dynosaur` claim left as a `related` edge
into Op 1 rather than an edit to ADR-0001's atom — authority rule 1 and the wave's own governance
atom (Op 22, "rewrite the referent, never the reasoning") both bar patching a corrected document in
place.

Five claims were labelled `aligns`. Op 22 is the one that produced a `governance` atom from an
`aligns` claim rather than skipping it: `.kb/decisions/README.md` already states the immutability
rule and the repair-versus-amendment test, so the atom does not introduce them — it adds the
referent-versus-reasoning discrimination the README has no room for, three worked instances from
the ADR chain itself, and cites the README rather than restating it. The README was not edited,
consistent with the previous wave's finding that a layer README is a contract, not a merge target.

## Unresolved claims

**Twelve claims labelled `requires-new-decision`, zero decision atoms newly authored for them.**
Same abstention as the previous wave, for the same three reasons restated in `02`'s Adjudications
and confirmed to still hold: a pass that both discovers a gap and decides it cannot be audited; a
decision taken to unblock an ingest is taken for the wrong reason; and an ADR here has a described
human-sign-off process an ingest cannot supply on its own authority. This mirrors the standing
project instruction that ADR authorship stays with the runbook's own ADR pass. All twelve are now
durably parked as `open_question` atoms in `.kb/open-questions/`, each naming its forcing event and
owning phase where one exists — `adr-status-vocabulary-exceeds-the-schema`,
`projection-store-batch-has-no-apply-seam`, `query-union-rule-is-owed-and-unowned`,
`global-versus-per-boundary-visibility-invariant`, `postgres-arm-c-structural-cost`,
`poll-count-bounds-the-visibility-rule`, `es-38-and-gap-read-rules-are-unowned`,
`projection-id-is-unvalidated`, `cf-40-fixture-limits-ownership`,
`dcb-reference-publishes-no-wire-format`, `human-readable-payload-encoding-on-a-constrained-peer`,
`sync-message-set-and-format-version`.

**`kb-open-question-adr-status-vocabulary-001` is the wave's own instrument turned on itself.**
Nine imported ADRs use `status` values (`accepted — provisional`, `partly superseded by`) that
`KbFrontmatter.status` cannot express (`01`, CL-D). The wave applied and *stated* a working
convention — provisional-ness moves into the summary's first clause, partial supersession becomes
prose plus a `related` edge, never the frontmatter pair — rather than silently normalising the
gap, and filed the residual as this open question. A future schema change should check this atom
before widening `status`'s enum.

**`kb-open-question-cf-40-ownership-001` records a live self-contradiction, not a settled fact.**
ADR-0015 and ADR-0012 each state, in terms, that the other owns the fixture's numeric-limit
capability. Neither decision atom was edited to pick a winner — authority rule 4 bars it — so both
link to the open question and the contradiction is preserved rather than resolved by an ingest
wave's unilateral judgement.

**One forward reference, declared rather than accidental.** `kb-decision-0002` (ADR-0002) carries
`superseded_by: kb-decision-0005`, an atom created three operations later in the same wave. This is
the only forward link in the plan; it was authored at create time because a `superseded` record
with an empty `superseded_by` is a broken statement for as long as it exists, and the id is
deterministic and lands in the same wave before validation runs (`02`, Op 6).

## Follow-ups

- **A human importing ADR-0017 onward should use the real `supersede` operation type**, not
  `create_new` with `status: superseded` — this wave's ADR-0002 is a special case (it arrives
  already superseded, by a document in the same batch) and the `supersede` op exists precisely for
  the ordinary case of a new atom superseding one already accepted in the corpus.
- **The decision map's partial-supersession chain (0002 → 0005 → 0006 → 0007) is annotation prose,
  not frontmatter**, by deliberate choice (`02`, Standing choice 2): marking ADR-0006
  `status: superseded` would strip `accepted` from the crate-allocation rule `CLAUDE.md` still
  enforces today, since ADR-0007 explicitly carries only half of ADR-0006 forward. A reader
  relying on `status` alone to reconstruct the chain will get it wrong; the decision map is the
  source of truth for the full picture.
- **`ANCHOR_SLACK` and the amendment ledgers stay outside `.kb/`.** Seven ADRs list
  amendment-ledger items against `spec/SPECIFICATION.md` (thirteen in ADR-0012, eighteen in
  ADR-0015, nine in ADR-0014); they are named-and-counted in the decision atom bodies with a
  `related` edge to the repair-frozen-clause playbook rather than copied in, per the
  mirror-nobody-maintains rule the reference README already states.
- **Nothing was routed to `.kb/product/`, `.kb/design/`, `.kb/narratives/` or `.kb/roadmaps/`.**
  Confirmed correct for this corpus rather than an oversight (`02`, Adjudications) — an
  event-sourcing library has no personas or interaction surfaces, and the ADR chain's own
  narrative shape is already carried by the two extracted atoms (Ops 22, 23) rather than
  duplicated into a `narrative` atom; `RUNBOOK.md` stays a living document outside `.kb/` rather
  than a mirrored `roadmap` atom.
- **Two new layers now exist and need the same discipline the previous wave gave `reference/`.**
  `.kb/concepts/` and `.kb/governance/` were created this wave with READMEs mirroring
  `reference/README.md`'s voice. The next wave placing a concept or a corpus-operating rule should
  read those READMEs before inventing a third shape for either kind.
- **Citations inside `.kb/` atom bodies are still not checked by any instrument.**
  `redkiln validate --kb` checks frontmatter, ids and cross-atom links, not `file:line` citations
  embedded in prose — same gap the previous wave's retrospective noted, still open, now with
  substantially more prose citing `.kb/decisions/`, `crates/`, `spec/` and `experiments/` paths to rot.

## Instruments run

`redkiln validate --kb` — pass (exit 0). `redkiln doctor` — pass (exit 0; six template-drift
warnings on `.redkiln/templates/{discover.md, gates/discover.md, gates/intake.md, spec.md,
_design.md, _intake-brief.md}`, all reporting the templates were pinned at init and differ from the
installed redkiln's bundled default — unrelated to `.kb/` content and not KB findings). No repair
loop was needed; both commands passed on the first run.
