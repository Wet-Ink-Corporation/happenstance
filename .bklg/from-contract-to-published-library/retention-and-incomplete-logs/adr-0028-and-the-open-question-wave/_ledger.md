---
item: "HS-S0121"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0028, and the open question resolved rather than deleted

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator doing a bounded look at public evidence, WHEN they open references/adr/0028-<slug>.md, THEN it is a single long-form record that states one question verbatim — what is a store permitted to forget, and how does it say so — scoped to ES-39, CF-27 and SY-32 and to nothing else, and it carries the depth (the transcripts, the tables, the losers argued out) that a summary cannot hold. Hierarchy invariant: the long form is where the depth lives; the atom links to it and never duplicates it."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the long-form half of the two-file convention (CLAUDE.md, Where the work lives)"
  verifying_test: "static (content review): references/adr/0028-<slug>.md read against RUNBOOK.md:307 and RUNBOOK.md:264-268; shape compared to references/adr/0029-msrv-raised-to-1-97-1.md"
- id: AC-002
  criterion: "GIVEN a maintainer who trusts redkiln validate --kb to tell them the knowledge base is well-formed, WHEN ADR-0028 lands, THEN it reached .kb/decisions/0028-<slug>.md only by being staged as .kb/_intake/0028-<slug>.md and consumed by /redkiln:kb-ingest — never hand-written into .kb/decisions/ — and the resulting atom carries real composed KbFrontmatter (kind, status, authority_tier, adr_id, reversibility, phase, supersedes/superseded_by, summary, depends_on, related, source_paths, last_reviewed) and a real body, not a stub that only points at the long form. Presentation invariant: the atom is a composed artifact in its own right."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ — consumed by /redkiln:kb-ingest (architecture brief composition root 8), landing at .kb/decisions/0028-<slug>.md"
  verifying_test: "static (process): `redkiln validate --kb` && `redkiln doctor`; plus `git log --diff-filter=A -- .kb/decisions/0028-*.md` showing first appearance in the kb-ingest wave commit; frontmatter compared to .kb/decisions/0029-msrv-raised-to-1-97-1.md"
- id: AC-003
  criterion: "GIVEN an adapter author who needs to know whether they are finished, WHEN they read ADR-0028's decision section, THEN they find either a decision or a refusal stated in terms — and if it refuses, the second half of the sentence is present too: deletion is out of scope for EventStore, and here is what a deleted-from store looks like to a reader, with the completeness instrument and the three recorded reader observations cited as that illustration. A refusal by omission fails this criterion."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md and .kb/decisions/0028-<slug>.md — the decision section of both halves"
  verifying_test: "static (content review): the decision section read against RUNBOOK.md:4626-4634 and project DR-9; refusal branch checked for citations to the committed instrument and all three reader observations"
- id: AC-004
  criterion: "GIVEN an application author choosing a contract before a database, WHEN they ask why the library does not simply expose a floor, THEN ADR-0028 names earliest_position() as a rejected alternative on ES-39's own grounds — a regulated purge is scattered, not a prefix; the low-position survivor that must outlive its neighbours; it ships looking correct until a claim runs long — rather than omitting it."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the alternatives-that-lost section"
  verifying_test: "static (content review): alternatives section read against spec/SPECIFICATION.md:4336-4349, checking ES-39's Rejects: line is answered"
- id: AC-005
  criterion: "GIVEN an evaluator who wants to know what the alternatives cost, WHEN they read the alternatives section, THEN the remaining DA-7/DA-8 losers are each named with their surface and version class: a set of retained ranges (new method + new public type), a third outcome on condition evaluation (crates/happenstance-core/src/error.rs, reaching every caller's match), and the tri-state contains_event_id on a surface that already exists (crates/happenstance-core/src/store.rs:268) — all four 0.3.0-class under 0.x."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the alternatives-that-lost section"
  verifying_test: "static (content review): alternatives section read against _decomposition.md:363-379 (DA-7) and :381-397 (DA-8); cross-checked that no row recommends a change"
- id: AC-006
  criterion: "GIVEN an adapter author who wants to know which conformance rules would have caught a forgetting store, WHEN they read ADR-0028's evidence section, THEN the enumerated pass list is cited per configuration — suffix and scattered, per DA-1 — stated as the set of rules that cannot tell a pruned store from a young one, with the DA-3 outcome that actually held recorded and the other two named; and a null result (all of them passed) is recorded as CF-27's predicted outcome and the strongest evidence for ES-39, never softened."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the evidence section, citing cf-27-experiment-and-recorded-pass-list's committed artefact"
  verifying_test: "static (content review): evidence section read against spec/SPECIFICATION.md:8034-8060 and _decomposition.md:234-274; rule names matched exactly against the upstream committed pass-list artefact"
- id: AC-007
  criterion: "GIVEN a maintainer who arrives at the claim a reader fails loudly and wants to know whether anyone checked, WHEN they read ADR-0028, THEN all three reader observations appear as observed output, not predicted prose: read_decision_model's last-retained match feeding AppendCondition::after_opt into an admitted append; IngestStore::holds answering false for an event this store minted; and the projection runner's actual state across the hole."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the evidence section, citing decision-model-and-ingest-observed and projection-runner-across-the-hole"
  verifying_test: "static (content review): evidence section read against crates/happenstance-core/src/store.rs:321-331, crates/happenstance-sync/src/ingest.rs:164 and RUNBOOK.md:4658-4668; actual values quoted from the upstream artefacts"
- id: AC-008
  criterion: "GIVEN a maintainer comparing CF-27's clause text (holds only a suffix) to the instrument that was actually built, WHEN they read ADR-0028, THEN DA-1's widening to an arbitrary retained set is recorded with its reason — a suffix-only instrument structurally cannot falsify the floor — and with the ground that made the widening available: CF-27 is [DEFERRED — owned by the pass that settles retention and deletion] and this project is that pass."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the widening note in the context/evidence section"
  verifying_test: "static (content review): read against spec/SPECIFICATION.md:8034-8036 versus :4309-4311 and :4326-4328, and _decomposition.md:150-171 (DA-1)"
- id: AC-009
  criterion: "GIVEN an evaluator deciding in one sitting whether the completeness axis is covered, WHEN they read ADR-0028, THEN the CF-25/CF-26 residual is recorded open: falsifiability is discharged by a fixture instrument, implementability is not, and the device adapter second half of the completeness axis stays outstanding and out of this project's scope. Reporting the axis as covered fails this criterion."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md and .kb/decisions/0028-<slug>.md — the residual-exposure statement"
  verifying_test: "static (content review): read against spec/SPECIFICATION.md:8003-8032 (CF-26) and :8090-8098 (the completeness axis), and _decomposition.md:205-213 (DA-2); an explicit still-open sentence naming the adapter half must be present"
- id: AC-010
  criterion: "GIVEN a maintainer who owns replication and must not have it re-decided under them, WHEN they read ADR-0028 and the PR diff, THEN SY-32 is cited by id as answered by replication-identity-and-ingest and never re-derived, any change to what a peer must report is recorded against SY-32, and no file under .kb/decisions/0026-* or 0027-* is touched by this diff."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the SY-32 citation; and the PR diff over .kb/decisions/"
  verifying_test: "static (citation check): SY-32 cited by id against spec/SPECIFICATION.md:6771-6799; `git diff --name-only main...HEAD` lists no .kb/decisions/0026-* or 0027-* path (_decomposition.md:618)"
- id: AC-011
  criterion: "GIVEN an application author who asked whether a Tag can be redacted, WHEN they read ADR-0028, THEN they get an answer: either the decision itself — grounded in the code DA-9 read, where Tag is Tag(Cow<'static, str>) with hand-written PartialEq/Eq/Ord/Hash all delegating to as_str(), no digest field and no store-side update path — or an explicit deferral naming a real path under experiments/. A deferral with no named experiment is a gate failure under CF-38 and fails this criterion."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md — the redaction paragraph; if deferred, a real path under experiments/"
  verifying_test: "static (content review) + spec-trace's CF-38 empty-falsifier check: read against spec/E2E-CASES.md:1288-1311, crates/happenstance-core/src/tag.rs:79 and :170-193, _decomposition.md:446-466 (DA-9); named experiment path confirmed to exist"
- id: AC-012
  criterion: "GIVEN a maintainer following the ES-38 thread, WHEN they read ADR-0028, THEN sub-question 3 — does positions_are_not_reused_after_removal get written against whatever removal-capable fixture arrives with CF-27, or does phase 14 need its own instrument decision first? — is answered on the record as what actually happened in this project, and sub-questions 1 and 2 are visibly not answered anywhere in the diff."
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0028-<slug>.md and .kb/decisions/0028-<slug>.md — the sub-question 3 answer"
  verifying_test: "static (content review): answer read against .kb/open-questions/es-38-and-gap-read-rules-are-unowned.md:87-89; negative check that read_from_a_gap_position's ownership and the [FROZEN]-clause-owner policy are not settled anywhere in the diff"
- id: AC-013
  criterion: "GIVEN a maintainer who follows the gap-read thread six months from now, WHEN they open .kb/maps/open-questions-index.md and click through, THEN they find the question still open rather than closed while they were not looking: a narrowed successor open_question atom, status: accepted, carrying only the read_from_a_gap_position ownership thread with related back to the original; the original at status: superseded, not deleted, its body left describing what was not known at the time, with related naming both ADR-0028 and the successor; and the index bullet at .kb/maps/open-questions-index.md:109-113 re-annotated rather than removed. State invariants: resolution happens in place, the unanswered thread is preserved across the transition, and the whole wave is reversible as one commit."
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/ → .kb/open-questions/ (narrowed successor + superseded original) and .kb/maps/open-questions-index.md, through /redkiln:kb-ingest"
  verifying_test: "static (process + atom read): `redkiln validate --kb` clean; the three atoms read directly against .kb/open-questions/README.md:40-45 and .kb/README.md:21; index bullet at .kb/maps/open-questions-index.md:109-113 present and re-annotated"
- id: AC-014
  criterion: "GIVEN an adapter author already building against 0.2.0, WHEN this PR merges, THEN nothing they compile against moved: no file under crates/ is touched, no pub item, signature or rustdoc changes, and spec/SPECIFICATION.md is untouched (every marker move is marker-moves-and-spec-trace-green's) — and the wave itself is hygienic: it lands as one kb-ingest run under a suffixed wave id that does not overwrite .kb/_governance/integration-waves/2026-08-10-intake or -2, with .kb/_intake/ cleared on success and .kb/_intake/README.md dropped at the approval gate. Non-occlusion invariant: resolving this question disturbs no unrelated part of the tree."
  satisfied: false
  evidence: ""
  mount_point: "the PR diff itself, plus .kb/_governance/integration-waves/<wave-id>/ and a cleared .kb/_intake/"
  verifying_test: "story grain: `cargo xtask affected --base main` green (.redkiln/config.yaml:40); plus `git diff --name-only main...HEAD` listing no crates/ path and not spec/SPECIFICATION.md; `ls .kb/_intake/` shows only README.md (.kb/_intake/README.md:13-19); `ls .kb/_governance/integration-waves/` shows a non-colliding wave id"
```
