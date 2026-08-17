---
item: HS-S0182
stage: implement
created: 2026-08-17
updated: 2026-08-17
---

# Acceptance ledger — Run the terminal gate last on the exact assembled tree

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story. **`_gate-run.md` is a working name** for the run-record companion
this story authors under its own directory (`spec.md`, "Clarifications resolved during spec", item 9);
if a different name lands, every `verifying_test` below is updated to it in the same commit. And
**no row may be flipped from a `cargo xtask ci --fast` run** — the transcript's 24 `=== step ===`
headers and its `all checks passed` closing line are what AC-002 is satisfied by, not a recalled
command string.

```yaml
- id: AC-001
  criterion: "GIVEN U2 is about to approve HS-P0025 and has been told the gate is green, WHEN they open the record, THEN it shows, one artefact per line, that commit `S` already contained every artefact this project owes — the promoted `.kb/product/` atoms, the appended `.kb/maps/domain-map.md` section, the `.kb/maps/open-questions-index.md` bullets, `links.kb` on HS-P0025, the reconciliation record, the DT-1…DT-10 audit and the fifteen-scenario DoD ledger — each with the check that was run against `S` and its result as a word, so that \"green\" is a statement about the deliverable and not about the gate (ordering 4, `_decomposition.md:627-641`)."
  satisfied: false
  evidence: ""
  mount_point: ".redkiln/config.yaml:60 — `verify.e2e: \"cargo xtask ci\"`, the terminal grain (architecture brief mount point 7, `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:485-488`)"
  verifying_test: "Tier 1 — the artefact-presence check executed at `S` (`git ls-tree -r --name-only <S>` / `test -f` per path in `.bklg/docs-that-teach/durable-audience-closeout/project.md:190-207`), transcribed into `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/_gate-run.md`, plus the Tier 2 read of that list against the same AC table"

- id: AC-002
  criterion: "GIVEN U2 cannot re-run a forty-minute gate to check a claim, WHEN they read the transcript in the record, THEN they count 24 `=== step ===` headers (20 `REQUIRED` + 4 `OPTIONAL`) and read the closing line `all checks passed`, so that they can tell the release bar ran from the transcript itself rather than from a reported command string — `cargo xtask ci --fast` emits 20 headers and the distinguishable line `all required checks passed (--fast: 4 optional step(s) not run)`."
  satisfied: false
  evidence: ""
  mount_point: ".redkiln/config.yaml:60 — `verify.e2e: \"cargo xtask ci\"`, invoked character for character as declared; the arm taken is `run_ci` (`xtask/src/main.rs:828-833`), never `run_fast` (`:853-860`)"
  verifying_test: "Tier 4 — one `cargo xtask ci` run captured whole into `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/_gate-run.md`; Tier 1 — `rg -c \"^=== \"` over that capture equals 24 and `rg -n \"all checks passed\"` matches"

- id: AC-003
  criterion: "GIVEN a skipped step prints nothing alarming and exits green, WHEN U2 reads the record, THEN every `skipped: `<probe>` did not succeed` line from the run appears verbatim with a one-sentence consequence beside it — a skip of `cargo deny` or of either feature powerset stated as a finding (CLAUDE.md says both tools resolve on this machine), a nightly `--cfg docsrs` skip stated as a narrowing and permitted — so that U2 knows exactly which of the twenty-four steps actually executed."
  satisfied: false
  evidence: ""
  mount_point: ".redkiln/config.yaml:60 — the terminal grain; the skip semantics come from `run_steps` (`xtask/src/main.rs:873-878`), which prints the line and continues"
  verifying_test: "Tier 1 — `rg -n \"^skipped:\"` over the captured transcript, counted against the skip table in `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/_gate-run.md`; Tier 2 — each row's consequence word is `finding` or `narrowing`, never blank"

- id: AC-004
  criterion: "GIVEN U2 has just read scenario 2's failing-by-name transcript and needs to know the break is gone, WHEN they read this record, THEN it shows `git status --porcelain` producing no output in the fresh checkout at the moment of the run, and the pinned narrative page identical at `S` to its pre-break content (by `git diff <pre-break-ref> <S> -- <page>` producing no output), so that \"reverted\" is checkable rather than asserted and the deliberate mutation is provably neither a committed nor an uncommitted state of the tree the green run was taken on."
  satisfied: false
  evidence: ""
  mount_point: ".redkiln/config.yaml:60 — merge-gate step 5 (`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:757-762`), which AC-TB-05 (`:837-839`) requires the broken edit to be absent at"
  verifying_test: "Tier 1 — `git status --porcelain` and `git diff <pre-break-ref> <S> -- <pinned page>` in the slice's fresh checkout, both empty, transcribed into `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/_gate-run.md` immediately above the gate capture; the page and ref come from `.bklg/docs-that-teach/durable-audience-closeout/scenario-two-fault-injection/spec.md`"

- id: AC-005
  criterion: "GIVEN the project's Definition of Done fixes an order and a reason for it, WHEN U2 reads the record, THEN `redkiln validate --kb` is shown exiting zero first and `cargo xtask ci` green second, both against the same sha `S`, each with its own transcribed exit line, so that the gate being green is legible as a precondition for reading the evidence and never as a substitute for it (`project.md:213-215`)."
  satisfied: false
  evidence: ""
  mount_point: ".redkiln/config.yaml:60 — the terminal grain, run as merge-gate step 5 after step 1's `redkiln validate --kb` (`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:740-743`, `:757-762`)"
  verifying_test: "Tier 4 — `redkiln validate --kb` then `cargo xtask ci`, both at `S` in the fresh checkout, both exit lines transcribed into `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/_gate-run.md`; Tier 1 — one sha appears in both blocks (AC-TB-07, `_decomposition.md:843-845`)"

- id: AC-006
  criterion: "GIVEN U2 reads one row at a time and may quote a single line into a review comment, WHEN they read any line of the record, THEN the verdict is the word `green` (never a tick, colour, emoji, strikethrough or empty cell), and the record names all seven provenance facts — checkout path, branch, commit `S`, the exact command string, the toolchain (`rust-toolchain.toml`'s pin), the observer, and the wall-clock date — each stated in its own labelled field, so that a later reader can re-run the observation from the record alone without asking anyone what was meant."
  satisfied: false
  evidence: ""
  mount_point: ".redkiln/config.yaml:60 — the record is the human-readable face of the terminal grain's result; its shape is fixed by AC-UX-09/10/12 (`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:266-281`) and the a11y floor (`:125-145`)"
  verifying_test: "Tier 2 — a human read of `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/_gate-run.md` against AC-UX-09, AC-UX-10, AC-UX-12 and NF-004; Tier 1 — `rg -n \"✅|❌|🟢|~~\"` over `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/` returns nothing and each of the seven field labels matches exactly once"

- id: AC-007
  criterion: "GIVEN U2 arrives from the fifteen-scenario ledger's DoD-1 row rather than from this folder, WHEN they follow that row's evidence pointer, THEN they land on this record in one hop, this story's `_ledger.md` cites the record back by `file:line`, the work commit is recorded via `redkiln record-links HS-S0182 --sha <sha>`, and the record states the closure rule with its proof — `git diff --name-only <S> HEAD` listing only paths under `.bklg/docs-that-teach/durable-audience-closeout/**` — so that the run is mounted rather than filed, and the commit that records the gate is shown to be one the gate could not have read."
  satisfied: false
  evidence: ""
  mount_point: "DoD-1's evidence cell in the slice's fifteen-scenario re-observation ledger under `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/` (working name `_dod-reobservation.md`, `dod-scenario-ledger/spec.md:166-173`), pointing at this story's run record; plus `links.commits` on HS-S0182 via `redkiln record-links --sha` (`.redkiln/config.yaml:73`)"
  verifying_test: "Tier 3 — the mount walk (open the fifteen-scenario ledger, follow DoD-1's pointer, arrive at `.bklg/docs-that-teach/durable-audience-closeout/terminal-gate-run/_gate-run.md` in one hop) and `redkiln verify --grain story` for `require_ledger` + `require_commit_provenance`; Tier 1 — `git diff --name-only <S> HEAD` transcribed, every line inside the spec's PR-boundary globs (argument: `xtask/src/affected.rs:248-266`)"
```
