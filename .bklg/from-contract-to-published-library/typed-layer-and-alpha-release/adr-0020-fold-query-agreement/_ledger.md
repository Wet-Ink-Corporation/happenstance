---
item: "HS-S0018"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0020 — fold/query agreement, and DT-2's signature answer

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two rows in this ledger are unusual and are unusual on purpose, so an implementer does not "work
around" them. **AC-008 is handoff-verified**: the atom it asserts is minted by `/redkiln:kb-ingest`
on its own worktree branch, so its evidence is that wave's commit sha plus the `redkiln validate
--kb` output — never a hand-authored `.kb/decisions/0020-*.md`, which is the anti-pattern reverted at
`0269720`. And **the substantive rows (AC-002…AC-006) are verified by human review**, because no
compiled assertion can prove a decision was recorded correctly; the testing brief says exactly this
for the whole class (`_decomposition.md`, Testing brief, AC-012 row: *"Record, not a test"*).

```yaml
- id: AC-001
  criterion: >-
    GIVEN the M2 implementer is about to write `DecisionModel` and needs the licensed shape, WHEN
    they look for ADR-0020, THEN a staged decision document exists at
    `.kb/_intake/0020-fold-query-agreement.md`, composed as a decision record — hazard/context, the
    decision, the alternatives that lost, the consequences, the residual — and carrying a proposed
    frontmatter block for the ingest run to author from (`adr_id: ADR-0020`, `kind: decision`,
    `authority_tier: decision`, `status: accepted`, `phase: 7`, a stated `reversibility`,
    `depends_on`/`related` naming the real atom ids `kb-decision-0003`/`kb-decision-0006`/`kb-decision-0007`,
    and `source_paths` that all resolve); it lands at no other path, because `_intake` is
    `/redkiln:kb-ingest`'s only input.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0020-fold-query-agreement.md"
  verifying_test: "test -f .kb/_intake/0020-fold-query-agreement.md; cargo xtask affected --base main (.redkiln/config.yaml:40); frontmatter reviewed against .kb/decisions/0029-msrv-raised-to-1-97-1.md:1-30"

- id: AC-002
  criterion: >-
    GIVEN a human approved `_design.md` on 2026-08-12 with DT-2 resolved toward explicit
    declaration, WHEN the record states the decision, THEN it states one shape and it matches that
    design row for row — `Boundary::query(&self) -> Result<Query, InvalidQuery>` on a sealed trait
    blanket-implemented for every `DecisionModel` and macro-implemented for tuples of arity 2..=8;
    `DecisionModel::scope(&self) -> &Tags`; `DecisionModel: Clone`, not `Default`; an empty
    `EVENT_TYPES` a compile error via a `const` item evaluated per-monomorphisation — and it
    contradicts no row of the signed-off design and re-decides none of them.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0020-fold-query-agreement.md"
  verifying_test: "human review at the report gate, side by side against .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md `## Shape decision` / `## Signatures` / `## Sign-off`"

- id: AC-003
  criterion: >-
    GIVEN a DCB handler names its event set twice — once in the query, once in the fold — and
    nothing checks the two agree, WHEN the maintainer asks why this trait and not a provided method,
    THEN the record states the hazard against real code in this tree, citing the two-item `Query` at
    `examples/course-subscriptions/src/main.rs:114-125` and the fold below it that re-interprets the
    same names, and states the resolution structurally: the query is derived and there is nowhere to
    put a hand-maintained one (DR-01, DR-02).
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0020-fold-query-agreement.md"
  verifying_test: "human review; every file:line citation re-opened before sign-off, reviewer opens examples/course-subscriptions/src/main.rs:114-173"

- id: AC-004
  criterion: >-
    GIVEN `Boundary::query` builds a `QueryItem` out of values that are already validated and still
    meets a `Result`, WHEN the M2 implementer reaches for the obvious fix, THEN the record has
    already answered where the validation went and what was done about the shortfall: paid once in
    the model's constructor because `Tags::from_pairs` is the only (fallible) way in; `Err` kept and
    re-read as `InvalidQuery::UnconstrainedItem` = this boundary constrains nothing; the other route
    to it turned into a compile error; `commit`/`commit_with` absorbing the `Result` so the first
    program writes no extra `?`; no `unwrap`, no edit to `happenstance-core`; and the shortfall
    logged as defect candidate D-1 with its routing to a decision record and `VT-18` named as its
    nearest clause subject.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0020-fold-query-agreement.md"
  verifying_test: "human review against crates/happenstance-core/src/query.rs:56, crates/happenstance-core/src/tag.rs:304, crates/happenstance-core/src/error.rs:99-102; plus `git diff --name-only` showing no path under crates/, examples/, xtask/ or spec/"

- id: AC-005
  criterion: >-
    GIVEN the maintainer six months out meets this fork again, WHEN they read the record, THEN every
    alternative that lost is named with the wrong implementation it admits — provided method on
    `DecisionModel` (overridable ⇒ a hand-maintained query); free `derive_query::<M>()` (ignorable);
    infallible `query()` (unreachable without an `unwrap`); fallible-without-the-const-assertion
    (defers a compile-time-decidable mistake to the first read); `scope() -> Tags` by value (`unwrap`
    inside an infallible signature); `&[(&str, &str)]` (revalidates, moves the error to the read); a
    `Default` supertrait (re-opens the invalid-`Tags` hole); an exported `compose!` macro
    (caller-visible ceremony) — and the record says explicitly that exactly one path is licensed, in
    the contract crate's own words about the same defect class.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0020-fold-query-agreement.md"
  verifying_test: "human review; enumeration checked complete against _design.md `## Shape decision` Rejected column, and the one-shape claim against crates/happenstance-core/src/projection.rs:47-61"

- id: AC-006
  criterion: >-
    GIVEN the audience is fluent in the domain and new to idiomatic Rust (`CLAUDE.md`, Who you are
    working with), so ceremony is the first-hour cost, WHEN the record states what this shape costs,
    THEN it carries DT-2's price honestly: the measured 2.4:1 ceremony-to-domain ratio in the first
    program is recorded as a consequence with a falsifiable prediction — that AC-013's verdict lands
    "`happenstance-macros` is in scope for 0.1" — and not as a second decision the record is taking
    on the derive's behalf.
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0020-fold-query-agreement.md"
  verifying_test: "human review against _design.md:1105-1109 and RUNBOOK.md:524; record checked to contain no commitment beyond DT-2's resolution and to name AC-013 as project-closeout-owned (project.md DoD 8)"

- id: AC-007
  criterion: >-
    GIVEN a successful ingest clears `_intake`, so the staged document is not a durable home for the
    reasoning, WHEN the wave runs, THEN the long-form record already exists at
    `references/adr/0020-fold-query-agreement.md` carrying the compiler-facing detail, the rejected
    alternatives and the trade tables a ~100-line atom cannot hold, and the staged document names it
    in the `source_paths` it proposes — exactly the two-places-on-purpose pairing ADR-0029 already
    demonstrates.
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0020-fold-query-agreement.md"
  verifying_test: "test -f references/adr/0020-fold-query-agreement.md; proposed source_paths lists both the intake and references paths, matching .kb/decisions/0029-msrv-raised-to-1-97-1.md's own source_paths; wc -l reviewed against CLAUDE.md's ~100-line atom budget"

- id: AC-008
  criterion: >-
    GIVEN AC-016 is a sequencing obligation — the record must exist before the code it governs — and
    atoms are authored by the ingest path and never by hand, WHEN a human runs `/redkiln:kb-ingest`
    over this document and its slice-mate in one wave on its own worktree branch, THEN
    `.kb/decisions/0020-fold-query-agreement.md` exists as an accepted atom,
    `.kb/maps/decision-map.md` carries its row, `redkiln validate --kb` and `redkiln doctor` are
    clean, and `git log --diff-filter=A` shows that atom added by the wave's commit and not by this
    story's PR.
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0020-fold-query-agreement.md"
  verifying_test: "redkiln validate --kb && redkiln doctor on the ingest branch; git log --diff-filter=A -- .kb/decisions/0020-fold-query-agreement.md names the wave commit (evidence = that sha)"
```
