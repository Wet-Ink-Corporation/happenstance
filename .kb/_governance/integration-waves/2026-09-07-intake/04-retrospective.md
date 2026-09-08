# Wave `2026-09-07-intake` — retrospective

## What merged vs. created

74 atoms created against 18 mutated (17 merges, 1 supersession) — a roughly 4:1 create-to-merge
ratio, the corpus's most create-heavy wave to date. That skew is a property of the intake, not a
placement failure: 48 of the 56 staged files are the `remediation-2026-09-04-briefs/` batch, each
brief written to answer exactly one adjudication from an earlier review with no existing atom
narrow enough to absorb it, so the placement pass correctly minted a new decision or open question
rather than forcing a merge for the sake of a lower atom count (`02`'s closing "why 74 creates is
right, not high" is not a section title in `02` but the reasoning runs through Adjudications
6–23). Where a real second instance of an existing question or defect showed up — poll-count,
postgres-arm-c, the event-metadata floor, query-plan chunking, the citation-anchor playbook — this
wave merged, exactly as prior waves did.

**What this wave does to the accepted decision corpus:** twenty-three atoms added, three of them
amendments (ADR-0038 → ADR-0001 and ADR-0035; ADR-0043 → ADR-0015), zero supersessions of an
accepted decision's body or `status`. The one supersession this wave performs is at the
open-question layer (`no-ps-rule-name-is-resolved.md` → `is-the-dagger-convention-superseded-by-
maturity-markers.md`), not the decision layer — the streak of never flipping an accepted decision
in place continues into a fourth wave.

Two structural notes: `.kb/decisions/` took twenty-three atoms in one wave, more than doubling its
prior largest single-wave addition, and lands ADR-0024 in the number phase 10 reserved for it
rather than at the sequence's tail (`02`, Adjudication 1) — a deliberate refusal to let numbering
reflect ingestion order instead of decision order. `.kb/open-questions/` took thirty-eight new
atoms, more than six times its previous record (six, set 2026-09-04) — the direct consequence of
staging a 48-file remediation batch in one wave rather than across several.

## Unresolved claims

- **The MSRV ratification conflicts with the accepted floor, and this wave declines to settle
  it.** `.kb/open-questions/msrv-ratification-conflicts-with-the-accepted-floor.md` is a
  `defer_open_question` disposition: a ratification brief proposed moving the MSRV again, and
  ADR-0029 (phase 2, raising it to 1.97.1) stays `status: accepted` and untouched. Settling which
  floor governs needs its own ADR, not a same-wave resolution smuggled through an unrelated
  intake.
- **Twenty-two `remediation-2026-09-04-briefs/` findings surface as open questions with no
  assignee named**, matching this corpus's established convention (`es-17`,
  `testkit-contention-tolerance`, and others from prior waves) of recording a live fork without
  forcing a premature owner onto it. The full list is `03`'s thirty-eight-atom enumeration.
- **The dagger-convention supersession answers two of its predecessor's three sub-questions and
  leaves the third open on purpose.** `is-the-dagger-convention-superseded-by-maturity-markers.md`
  states plainly that whether the dagger and the maturity marker are redundant or a division of
  labour "is an ADR's to say, not a lane's" — this wave records the surviving question rather than
  answering it.

## Post-integration repairs made at this stage

`redkiln validate --kb`'s first run failed with 24 dangling `related` references, all introduced
by this wave's own atoms (never a pre-existing corpus defect) — a hand-typed candidate id in a new
decision, playbook or reference atom that did not match the id the target atom actually carries.
Every one was repaired in place by correcting the reference to the real id; no atom's body,
`title`, or any other frontmatter key was touched, and no new atom was minted to paper over a
mismatch. The corrections, grouped by the file that carried the stale reference:

| File | Stale id → correct id |
| --- | --- |
| `decisions/0040-f2-5-holds-the-release-for-phase-10.md` | `…cf-5-per-rule-or-per-branch-001` → `…cf-5-per-rule-or-branch-001` |
| `decisions/0041-…email-channel.md` | `…rustdoc-citation-shape-001` → `…rustdoc-citation-form-001`; `…stale-name-reservations-001` → `…stale-0-0-0-name-reservations-001` |
| `decisions/0042-every-fixture-capability-lands-defaulted.md` | `…cf-40-fixture-limits-001` → `…cf-40-ownership-001`; `…reset-refusal-no-clause-001` → `…reset-refusal-declension-001`; `…read-fault-no-clause-001` → `…read-fault-rule-no-clause-001`; `…testkit-projection-exemption-001` → `…projection-module-exemption-scope-001` |
| `decisions/0043-event-metadata-…no-floor.md` | `…event-metadata-floor-001` → `…event-metadata-no-floor-001`; `…cf-40-fixture-limits-001` → `…cf-40-ownership-001` |
| `decisions/0044-a-published-crate-re-exports-its-drivers.md` | `…happenstance-facade-vs-adr-0006-001` → `…facade-does-not-match-adr-0006-001`; `…cloudflare-worker-feature-gate-001` → `…cloudflare-feature-gate-001` |
| `decisions/0050-stringified-throw-is-crate-private.md` | `…cloudflare-worker-feature-gate-001` → `…cloudflare-feature-gate-001` |
| `decisions/0045-a-citation-anchor-…-refuses.md` | `…exact-anchor-residuals-001` → `…exact-anchor-residue-001` |
| `decisions/0046-commit-reports-a-nothing-to-do-outcome.md` | `…then-empty-and-nothing-to-do-001` → `…then-empty-emission-idiom-001` |
| `decisions/0047-tags-and-scope-must-agree-at-commit.md` | `…scope-coverage-helper-001` → `…scope-coverage-helper-projection-gap-001` |
| `decisions/0048-op-read-is-non-exhaustive.md` | `…model-family-no-clause-001` → `…model-family-rule-no-clause-001` |
| `reference/model-family-case-count-cliff-2026-09.md` | `…model-family-no-clause-001` → `…model-family-rule-no-clause-001` |
| `decisions/0051-declension-by-inheritance-discharges-cf-18.md` | `…cf-18-residuals-001` → `…cf-18-residuals-after-declension-001`; `…postgres-fixture-read-fault-001` → `…postgres-read-fault-declension-001` |
| `decisions/0052-the-query-partition-constants-stay-public.md` | `…query-plan-chunking-001` → `…query-plan-parameter-chunking-001`; `…no-workerd-runner-001` → `…workerd-runner-absent-001` |
| `decisions/0054-after-opt-applies-…-says-so.md` | `…vt-30-marker-stale-001` → `…vt-30-marker-stale-unscheduled-001` |
| `decisions/0057-the-testkit-version-key-is-dropped.md` | `kb-playbook-landing-a-stricter-gate-001` → `kb-playbook-ratchet-gate-landing-001` |
| `decisions/0058-the-sqlite-write-path-stays-inline-and-documents-it.md` | `…es-11-ceiling-sample-cost-001` → `…es-11-sqlite-ceiling-sample-cost-001` |
| `playbooks/a-count-or-an-index-nobody-re-derives.md` | `kb-playbook-verify-the-referent-001` → `kb-playbook-verify-referent-report-coverage-001` |
| `playbooks/require-the-property-not-the-mechanism.md` | `kb-playbook-verify-the-referent-001` → `kb-playbook-verify-referent-report-coverage-001`; `kb-playbook-assert-a-tests-execution-001` → `kb-playbook-assert-execution-not-discovery-001` |

`redkiln validate --kb` exited 0 with no problems after the repair; the second run needed no
further correction, so the two-attempt repair budget this stage is held to was not exhausted.

## Follow-ups

- **Twenty-three new decisions become promises at whatever phase the initiative's release table
  next names** — the same `0.2.0` publication boundary ADR-0029 and ADR-0037 already established
  applies to every accepted decision minted here; none is provisional.
- **`kb-open-question-msrv-ratification-conflicts-…-001`** is forced by whoever next proposes
  moving the MSRV — the conflict this atom records must be resolved by a new ADR before that
  proposal can land, per the binding-constraints section of `CLAUDE.md`.
- **The dagger-convention question's surviving third** (redundancy vs. division of labour between
  the dagger and the maturity markers) is forced by whoever next regenerates spec/SPECIFICATION.md
  §7.2, per the atom's own closing line.
- **The 38 new open questions carry no assignee**, matching corpus convention; each names in its
  own body what would force it, and no follow-up beyond that is owed here.

## Doctor problems set aside as out of scope

`redkiln doctor --json` reported four problems, none of them under this wave's `.kb` path set —
this wave's writes are entirely under `.kb/decisions/`, `.kb/open-questions/`, `.kb/concepts/`,
`.kb/playbooks/`, `.kb/reference/`, `.kb/maps/`, `.kb/governance/` (one merge) and
`.kb/_governance/integration-waves/`, and it never opened `.bklg/` or `.redkiln/telemetry/` at
all. Set aside rather than repaired, because fixing either would smuggle an unrelated backlog or
telemetry change into a KB-intake commit:

- `unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/adr-0003-provisional-lift/story.md` (foundation story `HS-S0108`, consumed by no capability slice)
- `unconsumed-foundation: .bklg/from-contract-to-published-library/replication-identity-and-ingest/open-questions-resolved-and-indexed/story.md` (foundation story `HS-S0100`, consumed by no capability slice)
- `ledger-chain-broken: .redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl` (1 event does not hash to its own id)
- `ledger-chain-broken: ../../../.redkiln/telemetry/events/ryan-britton@happenstance@runs.jsonl` (the same partition, reported again under a relative path)

The two `unconsumed-foundation` problems are the identical pair every prior wave since
`2026-09-02-intake` has recorded as out of scope — nothing in
`.bklg/from-contract-to-published-library/` was touched by this wave either. The two
`ledger-chain-broken` problems are new to this wave's report and belong to `.redkiln/telemetry/`,
a tree this wave never wrote to; they are recorded here for visibility and left for whoever owns
the telemetry ledger to recover from git history, per the tool's own guidance, rather than
repaired by hand from inside a KB-intake wave.

`redkiln validate --kb` exits 0 after this stage's repair, with no problems of any kind. The
combined, scoped verdict for this wave is **pass**.
