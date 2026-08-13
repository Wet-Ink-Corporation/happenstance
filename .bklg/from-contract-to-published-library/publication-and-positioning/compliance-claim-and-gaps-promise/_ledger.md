---
item: HS-S0093
stage: implement
created: 2026-08-12T13:47:31.155Z
updated: 2026-08-12T13:47:31.155Z
---

# Acceptance ledger — The compliance claim ships with its evidence, and the gaps promise is findable

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two of the verifying instruments are artefacts this story commits rather than compiled tests, because
the project testing brief assigns AC-009 and AC-010 the **human observation on the rendered page**
tier (`_decomposition.md`:472-473) and a tier with no output is not a check:
`…/compliance-claim-and-gaps-promise/_rendered-check.md` (the dated pre-publish render read, per
surface, at 1024×768 and 1440×900) and `…/compliance-claim-and-gaps-promise/_evidence-derivation.md`
(the sibling reports read, the resolved implementation set, the evidence-ladder rung and its reason).
Evidence citing either must be a `file:line` into the committed artefact, never a claim that the read
happened.

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator has landed on the rendered crates.io page for any of happenstance, happenstance-core or happenstance-testkit and read the first screen, WHEN they scroll past the fold, THEN the first `##` heading they meet is the compliance block, sited above the quick start, presented as one composed visual unit — `##` heading, the claim as the block's first sentence, the date on its own line, one link — carrying all four mandatory parts (claim, implementations, date, link) in <= 6 rendered lines, and the block is byte-identical on all three packaged READMEs because it is one claim."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (R6; same block on crates/happenstance-core/README.md and crates/happenstance-testkit/README.md)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md (heading level, part order, position vs quick start, measured rendered-line count per surface) + the slice's byte-identity assertion registered with AC-UX-011's check + `cargo xtask affected --base main`"

- id: AC-002
  criterion: "GIVEN an evaluator who knows the DCB specification defines no conformance process and therefore has nothing external to check the adjective against, WHEN they read the block's first two sentences, THEN the block concedes that \"DCB-compliant\" is self-asserted — here as everywhere — and then states what is checkable: which implementations ran happenstance-testkit's rules and passed, and why a pass from this suite discriminates (every rule carries a wrong implementation it rejects, and a declined capability still runs the rule and reports its stated reason rather than vanishing). The evaluator can restate what was checked, not merely that something was."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (R6; same block on the other two packaged READMEs)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md (restatement test, written from the block alone) cross-read against .kb/decisions/0010-the-suite-must-prove-itself.md + `cargo xtask affected --base main`"

- id: AC-003
  criterion: "GIVEN a maintainer at the publish commit with four sibling adapter projects listed as blockers, WHEN they write the implementations line, THEN every name in it is an adapter whose own sibling project report is closed and records a conformance-suite pass at or before the stated date — derived from those reports, never from project.md:315-329's Dependencies list, which records blocking and not passing; MemoryEventStore appears only if labelled the reference store and never as an implementation the claim rests on; no maturity count appears in the block (AP-3); and if no sibling has a closed passing report the block says exactly that — the suite exists, its rules discriminate, no third-party adapter has run it yet — with the date and the link still attached, rather than being padded, softened or omitted."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (R6; same block on the other two packaged READMEs)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_evidence-derivation.md (each sibling report read, its path, its closed state, the date read, the resolved set, reviewed against project.md:315-329 for non-equality) + _rendered-check.md verdict of the block against that record + `cargo xtask affected --base main`"

- id: AC-004
  criterion: "GIVEN publication is the one irreversible act here — `cargo yank` removes a version from resolution and leaves the rendered page exactly as it is — WHEN a later release passes the suite against more adapters, THEN this version's block is superseded by a later dated claim rather than needing a silent rewrite: the fact stated plainly, the implementations stated as of a date, the date attached, the authority cited by path rather than asserted, and no timeless, forward-looking or aspirational phrasing anywhere in the block."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (R6; same block on the other two packaged READMEs)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md (tense and date-attachment read, diffed in form against README.md:227-234) + _evidence-derivation.md's statement of what a later release's block would replace + `cargo xtask affected --base main`"

- id: AC-005
  criterion: "GIVEN an evaluator who cannot run the suite and will not clone the repository, WHEN they follow the block's evidence, THEN exactly one link leaves the block; it is an absolute URL; its text is meaningful standing alone (never \"here\", \"this\", or a bare URL); and it lands in one hop on a specific anchor in a reachable committed artefact — never a repository root, never a bare \"see the specification\", never \"run our CI\" or \"check out the repo\". The rung of the evidence ladder taken is recorded with its reason — (1) a committed conformance-run report naming implementations and date, (2) the single named adapter's sibling report, (3) .kb/decisions/0010-the-suite-must-prove-itself.md — taking the highest rung available, because a skip is reported and never silent."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (R6; same block on the other two packaged READMEs)"
  verifying_test: "the slice's mechanical link/anchor check (AC-UX-011, owned by landing-copy-and-status-truth) asserting the link is absolute and lands on a fragment + _evidence-derivation.md's recorded ladder rung and reason + _rendered-check.md's hop-count and link-text read + `cargo xtask affected --base main`"

- id: AC-006
  criterion: "GIVEN a reader on the happenstance-testkit crates.io page, WHEN they read the status callout and then scroll roughly ten lines to the compliance block, THEN the two agree: crates/happenstance-testkit/README.md:16-17's \"No adapter has run this suite\" is reconciled to the tree that ships, keeping the callout's existing what-changed / what-is-still-early shape — or, if AC-003's empty case holds, the sentence is true, stands unchanged, and the block agrees with it. No sentence on the page contradicts another sentence on the same page."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/README.md:9-22 (R2, the status callout) with R6 in the same screenful"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md (explicit agree/contradict verdict on callout + block in one screenful) + diff review asserting the blockquote and bold lead phrase survive + `cargo xtask affected --base main`"

- id: AC-007
  criterion: "GIVEN a newcomer of any of the four personas asking U3 — what happens when a read is independent of the write that produced it — and none of them currently has anywhere to look, WHEN they scroll to the Guarantees block of the rendered happenstance or happenstance-core page (0 hops from the landing entry), THEN one bullet of <= 3 rendered lines tells them, in plain language and without reading source, that positions are unique and strictly increasing within one store, that gaps are permitted, that a position is not a count, that code assuming the next event is at `n + 1` is wrong against a conformant store, and that resume is `checkpoint.next()` — stated as the consequence for the caller, in the caller's register, and not as a paraphrase of the clause. happenstance-testkit gains no Guarantees section."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md:41-49 (R9, Guarantees) and crates/happenstance-core/README.md:43-54"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md (the bullet read against U3's question, hop count, register check against spec/SPECIFICATION.md:1020-1027, rendered-line count vs the <= 3 budget) + `cargo xtask affected --base main`"

- id: AC-008
  criterion: "GIVEN the citation is the only thing separating this promise from what two DCB-labelled stores already disagreeing in public can each write, WHEN the reader takes the one hop, THEN the bullet carries exactly one link, absolute, into VT-11's clause ID in spec/SPECIFICATION.md (not a heading, not the file root), and it resolves to a live rendered anchor — and that anchor's slug is registered with the slice's mechanical link check, because `cargo xtask spec-trace` guards the clause ID while the rendered anchor is slugged from the whole heading text and can die while the gate stays green."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md:41-49 (R9, Guarantees) and crates/happenstance-core/README.md:43-54"
  verifying_test: "the slice's mechanical link/anchor check (AC-UX-011) resolving the fragment against the rendered spec/SPECIFICATION.md, with the ID-versus-slug reason recorded + `cargo xtask spec-trace` (xtask/src/spec_trace.rs) green + `cargo xtask affected --base main`, which runs spec-trace unconditionally"

- id: AC-009
  criterion: "GIVEN `read_from_a_gap_position` is named by ADR-0011 and ADR-0013 and taken by neither — \"two accepted decisions have now looked directly at this rule and neither took it\" — WHEN a reader reads the promise, THEN nothing in it can be read as claiming the library owns that rule; a single nested line — the only nested bullet permitted anywhere in Guarantees — states the absence and its reason, attached to the thing it qualifies, is structurally subordinate so it cannot be read as the promise, and carries its own link to .kb/open-questions/es-38-and-gap-read-rules-are-unowned.md. The open question stays open: its status, its body and .kb/maps/open-questions-index.md are unchanged, and the diff contains no .kb/ write at all."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md:41-49 (R9, the nested line under the DT-4 bullet) and crates/happenstance-core/README.md:43-54"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md (explicit over-claim verdict listing the phrases considered) + the slice's link check resolving the open-question link + `git diff --stat` asserting zero paths under .kb/ + `cargo xtask affected --base main`"

- id: AC-010
  criterion: "GIVEN the owner signed off a first screen measured at ~343 px against a 340 px budget and declared it full at 0.2.0, WHEN the page is read at 1024x768 (14 rendered lines above the fold — the design case, not the degraded one), THEN both of this story's blocks are revealed on scroll, below the fold, 0 hops, and neither is promoted onto the first screen to make it more prominent; the compliance block is <= 6 rendered lines (claim 1, implementations 1-2, date 1, link 1) and is the only primary-ranked element in its screenful; and the Guarantees list holds <= 7 bullets, each <= 3 rendered lines, exactly 1 link per top-level bullet, of which this story spends exactly one, plus its single nested exception."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md (R6 and R9), crates/happenstance-core/README.md, crates/happenstance-testkit/README.md (R6 only)"
  verifying_test: ".bklg/from-contract-to-published-library/publication-and-positioning/compliance-claim-and-gaps-promise/_rendered-check.md (per surface, at 1024x768 and 1440x900: fold position, measured rendered-line count per block, Guarantees bullet and link tally, primary-ranked element count in the block's screenful) + slice-level reconciliation with guarantees-and-docs-rs-presentation confirming 3 + 1 + 3 = 7 + `cargo xtask affected --base main`"

- id: AC-011
  criterion: "GIVEN crates/happenstance/src/lib.rs:10 compiles this README as the crate's own doctest and two live intra-document anchors are already held, WHEN the story-grain gate runs on the diff, THEN the packaged surface is still a working artefact: the README doctests compile, `cargo package -p happenstance --list` still contains the README and both licence files, README.md:9 -> #licence and README.md:16 -> #status still resolve and no heading this story adds collides with or shadows either, and no raw HTML, inline style, JavaScript, colour, meaning-bearing image or animated media enters any packaged surface."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/README.md, compiled via crates/happenstance/src/lib.rs:10 and contained per xtask/src/package.rs:88-94"
  verifying_test: "`cargo test -p happenstance --doc` + `cargo package -p happenstance --list` against xtask/src/package.rs:88-94's REQUIRED_FILES + the slice's mechanical anchor check (AC-UX-011) resolving README.md:9 -> #licence and README.md:16 -> #status + `cargo xtask affected --base main` (.redkiln/config.yaml:40)"
```
