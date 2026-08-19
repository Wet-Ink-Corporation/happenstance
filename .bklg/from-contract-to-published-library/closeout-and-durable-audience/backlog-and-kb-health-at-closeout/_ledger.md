---
item: "HS-S0133"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Backlog and knowledge base clean, with exactly six drift advisories

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes specific to this story, all from `spec.md`. This project owns no code (`project.md`
*Out of scope*), so every `verifying_test` below is a **recorded command with its captured output
committed at a real path** rather than a test-framework id — the tier the `testing` brief assigns
(`_decomposition.md` *Test mix*, `:63-73`: static and process are load-bearing here). Nothing is
flipped on a green **exit code**: AC-002 is satisfied by the captured `doctor.json` evaluated against
the expression transcribed byte-for-byte from `.github/workflows/ci.yml:176-187`, which is the
story's named wrong implementation inverted (`discover.md` *The wrong implementation*). And a row is
not satisfied by a deviation being *absent* — where the drift set differs, AC-006 is satisfied by the
recorded **disposition** plus the routed finding, never by a repair; a template edited so the
assertion comes out at six fails this ledger and `project.md` AC-013 on `findings-disposition-register`'s
behalf.

`_evidence/` below abbreviates
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/`.

```yaml
- id: AC-001
  criterion: "The reader can name the tree and the instrument, and get the same answer themselves. GIVEN the evaluator asking \"clean on *what*, measured with *what*\", WHEN they open the Backlog and KB health section of `_closeout-record.md`, THEN they find (a) the SHA the three commands ran at and the isolation command that produced that checkout, added as this story's own Harness row rather than citing slice 1's — because slice 4 committed `.kb/product/` atoms after slice 1's pin, so slice 1's tree does not contain the atoms being validated (B-1); (b) the `redkiln --version` output captured in the same session, with an explicit statement of whether it equals CI's pinned `v0.18.0` (`ci.yml:130`) and, if not, that the comparison is against a different bundled template set stated before any verdict (B-5); and (c) `git status --porcelain --ignored` empty in that checkout, captured before any command ran. Re-running the recorded isolation command at the recorded SHA yields a tree whose `git rev-parse HEAD` equals it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/checkout-transcript.txt (clone + checkout --detach <SHA>, captured whole per clean-checkout-harness B-2), _evidence/residue-before.txt (git status --porcelain --ignored, zero bytes, command on record) and _evidence/cli-version.txt (redkiln --version), read against this story's Harness row in _closeout-record.md and compared to .github/workflows/ci.yml:130"

- id: AC-002
  criterion: "The reader can see that the verdict came from the report, not from an exit code. GIVEN the evaluator who knows `doctor` \"exits non-zero on a problem and zero on a warning\" (`ci.yml:147-149`) and therefore distrusts a green tick, WHEN they open the assertion evidence, THEN they find the three commands run in CI's order — `redkiln validate`, `redkiln validate --kb`, `redkiln doctor --json > doctor.json` with `|| true` — each invocation's stdout, stderr and exit status captured to a file; the full `doctor.json` committed as bytes; and the `jq` expression transcribed byte-for-byte from `ci.yml:176-187` (not paraphrased, not re-spelled) evaluated against those bytes with its own exit status recorded. The exit code appears in the record beside the parsed verdict and never in place of it, and a disagreement between the two is recorded as a finding rather than reconciled. This is the story's named wrong implementation (`discover.md` *The wrong implementation*) stated as its inverse."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/validate.txt, _evidence/validate-kb.txt (stdout/stderr/exit each), _evidence/doctor.json (verbatim report), _evidence/doctor-exit.txt (exit status alone) and _evidence/assertion.txt (the jq expression as pasted plus its exit status), with a byte-comparison of that expression against .github/workflows/ci.yml:176-187"

- id: AC-003
  criterion: "The reader can check the six for themselves without opening CI. GIVEN the evaluator asking \"six of *what*, and six out of how many\", WHEN they read the health section, THEN they find the six expected paths written out by name — `.redkiln/templates/_design.md`, `_intake-brief.md`, `discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md` — the sorted `template-drift` `.file` list the report actually contained, and the statement that the test is equality, not superset, with the reason on both sides: an extra entry is a template changed without a decision, a missing entry is a customisation reverted (`ci.yml:160-171`; `CLAUDE.md:116-121`). No count is written in prose beside the list — `ci.yml:168-171` records that a number beside its own list drifts the first time the list moves, having itself said \"the four files\" while the list held six."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/assertion.txt (the equality clause and its result) read against _evidence/doctor.json's template-drift entries, and the six named paths in _closeout-record.md's health section compared to .github/workflows/ci.yml:180-185 and to .redkiln/templates/ on disk"

- id: AC-004
  criterion: "The reader can tell what was deliberately not claimed. GIVEN the evaluator asking \"you asserted on two warning kinds — what else was in that report\", WHEN they read the health section, THEN they find every other `warnings[].kind` the report contained listed unasserted, with the inherited scope stated in the repository's own terms: `harvest-unrecorded` \"and friends fire in normal operation, and a `warnings.length > 0` guard would red this branch the day it landed\" (`ci.yml:173-175`). The difference the criterion protects is between \"there were no other warnings\" and \"no other warnings were asserted on\" — the first is a claim this run cannot support and must not appear. Inventing a stricter local check than CI's also fails this: the local run and CI must be capable of disagreeing about the tree, never about the question."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/warning-kinds.txt (an enumeration of [.warnings[].kind] over the same _evidence/doctor.json), reproduced as an unasserted list in _closeout-record.md's health section with the .github/workflows/ci.yml:173-175 reason quoted"

- id: AC-005
  criterion: "The reader is not offered a vacuous check as evidence. GIVEN the evaluator who reads that accepted decision atoms are immutable and that \"validation checks each one against `HEAD`\" (`CLAUDE.md:87-91`), and who then notices the run happened in a pristine checkout where the working tree is `HEAD`, WHEN they look for what actually backs the immutability claim, THEN they find the record stating which half `validate --kb` can carry there (frontmatter/status conformance, including on slice 4's new `.kb/product/` atoms) and which half it cannot, plus the non-vacuous complement over the commit range — `git log --diff-filter=M <initiative-start-sha>..HEAD -- .kb/decisions/`, expected to name no commit modifying the body of an atom whose status was `accepted`, with a supersession (a new atom plus a status flip on the old one) recorded as the permitted shape when one appears. A green `validate --kb` offered alone as evidence for immutability fails this criterion — that is a rule no run in this checkout could fail, which is decorative in exactly the sense `CLAUDE.md` *The rule that matters* forbids."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/validate-kb.txt (the conformance half, exit zero) and _evidence/decisions-range.txt (git log --diff-filter=M <initiative-start-sha>..HEAD -- .kb/decisions/ with its full command line and every hit classified supersession vs. body edit), read against .kb/decisions/README.md and .kb/governance/rewrite-the-referent-never-the-reasoning.md"

- id: AC-006
  criterion: "The reader can verify the prohibition held, and see any deviation dispositioned rather than tidied. GIVEN the evaluator who has read `CLAUDE.md:123-128` (\"Never run `redkiln adopt --templates`\" — `redkiln upgrade` recommends it and it would overwrite all six customisations with the bundled defaults, silently), and who knows that \"we did not run it\" is an assertion rather than evidence, WHEN they check, THEN they find `git log --oneline <initiative-start-sha>..HEAD -- .redkiln/templates/` naming no commit modifying any of the six — the exact footprint an adopt would have left — which together with AC-003's equality result lets them conclude the prohibition held without taking anyone's word for it. AND, where the sorted set is not the six, they find the deviation dispositioned in the record: which file is extra or missing, which commit changed it (from the same range check), and the decision — revert the accidental change, or adopt deliberately with a stated reason — with confirmation that `adopt --templates` was not run either way, and the finding handed to `findings-disposition-register` with a destination. The change itself is not made here; a template edited so the assertion comes out at six fails this criterion and AC-013 on the project's behalf."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/templates-range.txt (git log --oneline <initiative-start-sha>..HEAD -- .redkiln/templates/, each hit classified adopt-footprint vs. deliberate customisation), plus the disposition rows in _closeout-record.md's health and Findings sections (destination column, no fix column) checked against project.md:233-236, project.md:179-183 and .redkiln/config.yaml:5"

- id: AC-007
  criterion: "The reader can tell exactly how far the verdict reaches. GIVEN the evaluator who notices that three artefacts in this project land after the SHA this story measured — this story's own `_ledger.md` and the folders of `findings-disposition-register` and `initiative-closeout-readiness` (`_storymap.md:152-155`) — WHEN they read the health verdict, THEN it carries the SHA on the row rather than the document carrying one, names those artefacts as arriving later, and names the `backlog` CI job on the merge commit (`ci.yml:142-187`) as the standing re-check that covers them; every evidence cell names the artefact path plus the subject string a reader should find there plus the SHA it was observed on; the section states rows filled against rows owed; and whatever footprint the run itself left in the checkout (a telemetry file under the tracked `.redkiln/telemetry/events/` path, given `auto_stage_telemetry: true` at `.redkiln/config.yaml:12`, or a rebuilt `.redkiln/index/` that `.redkiln/.gitignore` ignores) is recorded, not deleted. A verdict written as though it covered the finished tree fails this; so does a post-run status check made clean by tidying, which manufactures the evidence the residue check exists to collect."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md"
  verifying_test: ".bklg/from-contract-to-published-library/closeout-and-durable-audience/backlog-and-kb-health-at-closeout/_evidence/residue-after.txt (git status --porcelain in the checkout after the three commands, every entry named and explained rather than removed), read together with the per-row SHA, the coverage line and the scope caveat in _closeout-record.md's health section; contract checked against .kb/playbooks/verify-the-referent-and-report-coverage.md and .kb/playbooks/anchoring-citations-in-a-long-lived-document.md"
```
