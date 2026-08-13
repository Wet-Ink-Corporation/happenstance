---
item: HS-S0066
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The structural bill written where a consumer meets it

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, both from `spec.md`:

- The tier is **Static** (`_decomposition.md:519`), which here means two compiler-checked instruments
  (the `-D warnings` rustdoc build and doctest compilation) **plus a recorded human read**. Evidence
  for a read is a `file:line` into the source of the rendered text, not the sentence "I read it".
- AC-003 and AC-005 each have a legitimate two-branch outcome (planning figure vs. reasoned
  still-open; declined clause vs. new clause). The evidence must say **which branch** was taken and
  cite the record that says so. Silence is the one outcome AC-005 forbids.

```yaml
- id: AC-001
  criterion: >-
    GIVEN an evaluator making an adopt-or-decline call in one sitting, WHEN they land on
    happenstance-postgres's docs.rs page and read the crate root before opening a single method,
    THEN the crate root states — in the persistent chrome of the crate-level rustdoc, above the
    module and item listings — that head() reports the visibility frontier rather than
    max(position), that the frontier trails the maximum, and that this is the shipped behaviour of
    the mechanism ADR-0024 chose, naming the arm that lost exactly once (RS-70-5) rather than
    presenting three candidates.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/lib.rs"
  verifying_test: "gate step `documentation` — `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with RUSTDOCFLAGS=-D warnings (xtask/src/main.rs:290-300); plus the recorded read of the rendered happenstance_postgres crate root"

- id: AC-002
  criterion: >-
    GIVEN an application author who arrived by deep link to a method — #method.append or
    #method.head — rather than through the crate root, WHEN they read that method's own
    documentation, THEN the consequence they are about to hit is stated there and not merely
    deferred upward: append's docs say that returning Ok(P) does not promise the next head() is at
    or above P and that a read issued immediately afterwards may not contain the event, and head's
    docs say what a frontier is. It is named a capability limit, in the caller's terms rather than
    the mechanism's, so a reader knows it is not a bug to report.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/event_store.rs (append :136-144, head :146-161)"
  verifying_test: "gate step `documentation` (xtask/src/main.rs:290-300); plus the recorded read of the rendered PostgresEventStore item page against spec/SPECIFICATION.md:2833-2836 and crates/happenstance-testkit/src/suite.rs:1798"

- id: AC-003
  criterion: >-
    GIVEN an application author choosing a contract before a database, who needs to know whether a
    read-after-write workflow is servable, WHEN they read the staleness statement, THEN they learn
    its scope (bounded by the longest open write transaction anywhere in the cluster, including an
    unrelated transaction in an unrelated database), both measured endpoints with their conditions
    (0.688 ms with no holder; 4010.719 ms behind an unrelated five-second write), and the single
    magnitude they should budget for — which closes sub-question 3 of
    .kb/open-questions/postgres-arm-c-structural-cost.md, or states explicitly that it remains open
    and what measurement would close it. The wording admits no reading in which an operator can tune
    the bound down: it is a documented capability limit, not a knob.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/lib.rs (crate-root bill); disposition staged under .kb/_intake/"
  verifying_test: "gate step `documentation` (xtask/src/main.rs:290-300) with both figures checked verbatim against experiments/position-visibility/results/staleness_pinned.txt; `redkiln validate --kb` over the staged open-question disposition"

- id: AC-004
  criterion: >-
    GIVEN the slice-mate adr-0024-position-visibility-mechanism has just chosen the adapter's
    mechanism, WHEN a reader compares the ADR's verdict with the crate's documentation, THEN they
    cannot be made to disagree: the bill describes the mechanism actually chosen. If ADR-0024
    selected either lock arm, the frontier collapses to max(position), read-your-own-writes does
    hold, and AC-001–AC-003's arm-C prose must not ship — a documented limit the adapter does not
    have is as false as an undocumented one it does.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/lib.rs, against the ADR-0024 record at references/adr/0024-*.md and its .kb/_intake/ staging"
  verifying_test: "recorded line-for-line correspondence read between the ADR-0024 verdict and the rendered crate root, taken in the shared slice context; backstop read of crates/happenstance-postgres/src/event_store.rs:43-77"

- id: AC-005
  criterion: >-
    GIVEN a phase-12 publication auditor reading the clause ledger, WHEN they ask whether the three
    consequences are stated normatively or only as prose inside ES-10, THEN they find a written
    verdict with its loser named — exactly one of (a) no new clause, recorded with VT-12's
    within-one-editing-pass drift as the stated reason and a pointer to what the auditor should read
    instead; or (b) a new clause in spec/SPECIFICATION.md naming its conformance rule and the wrong
    implementation it forbids. Silence, or a clause that names no rule, fails this criterion. The
    verdict is written into ADR-0024's staged intake document and references/adr/ long form before
    ingest, or — if that atom is already accepted — staged as a superseding record; never an edit to
    an accepted body, and no ADR number invented here.
  satisfied: false
  evidence: ""
  mount_point: "references/adr/0024-*.md and .kb/_intake/ (the decision record); spec/SPECIFICATION.md on branch (b) only"
  verifying_test: "`cargo xtask spec-trace` (xtask/src/main.rs:315) on branch (b); `redkiln validate --kb` (accepted-decision immutability against HEAD) on both branches; the verdict text itself cited by file:line"

- id: AC-006
  criterion: >-
    GIVEN the same evaluator, WHEN they scan the crate root for what is unresolved, THEN the crate no
    longer advertises a question it has answered: the "Open decisions — how ES-10 is bought" bullet
    (lib.rs:28-36) is gone, no path this slice made real still says "Status: not implemented"
    (lib.rs:3-9), and the module-level ES-10 note (event_store.rs:26-77) reads as one shipped
    mechanism rather than three candidates and no measurement. The crate root ends with no more than
    five # sections — it carries four today — so the bill lands by replacing stale chrome, not by
    appending a fifth and sixth section beneath it.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/lib.rs:1-63; crates/happenstance-postgres/src/event_store.rs:26-77"
  verifying_test: "rendered crate-root read with the heading count recorded; diff review against lib.rs:1-63 and event_store.rs:26-77; `cargo xtask ci --fast` (.redkiln/config.yaml:55) green over the change"

- id: AC-007
  criterion: >-
    GIVEN any reader on any docs.rs build of this crate, WHEN the page renders, THEN the bill exists
    as composed rustdoc and survives the gate: it is //! / /// documentation with real headings and
    not a // body comment (which renders nowhere), every fenced code block in it uses a checked fence
    (no_run, or rust where it can run, never text), every intra-doc link resolves in every feature
    configuration the crate can be built in (RS-70-2), and nothing load-bearing sits behind
    #[doc(hidden)], a non-default feature, or --document-private-items. No new public item is added
    and no custom HTML, CSS or JavaScript is introduced into the rustdoc.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/src/lib.rs and crates/happenstance-postgres/src/event_store.rs"
  verifying_test: "`cargo test -p happenstance-postgres --doc`, reached by the gate's `tests` step (xtask/src/main.rs:143); gate step `documentation` with rustdoc::broken_intra_doc_links denied (xtask/src/main.rs:290-300); `semver compatibility` job (.github/workflows/ci.yml:279)"
```
