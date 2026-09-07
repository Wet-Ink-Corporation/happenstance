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
  satisfied: true
  evidence: >-
    Branch: arm C shipped, so EC-001 did not fire. The crate root is rewritten at
    crates/happenstance-postgres/src/lib.rs:32-105 — "# What this store costs a caller", with the
    first of the three consequences at :45-52 ("head() reports a visibility frontier, not
    max(position)", and the frontier "legitimately sits below the highest position this store has
    already assigned"), stated at :38-44 as the shipped mechanism rather than as three candidates.
    The arms that lost are named exactly once, in a single sentence at :98-105, priced by what they
    buy before what they cost (RS-70-5). Rendered and read:
    target/doc/happenstance_postgres/index.html carries #what-this-store-costs-a-caller as
    crate-level chrome above the Re-exports and Modules listings, and its rendered text contains
    both "visibility frontier" and "frontier trails the maximum". Gate: the `documentation` step
    green under RUSTDOCFLAGS=-D warnings (xtask/src/main.rs:491-502), inside `cargo xtask ci
    --fast` ("all required checks passed").
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
  satisfied: true
  evidence: >-
    The item-grain half landed on the trait impl, so both deep-link anchors carry it. `append` at
    crates/happenstance-postgres/src/event_store.rs:440-469 — the section "# What Ok(P) does and
    does not promise" says it does NOT promise the next head() is at or above P and that a read
    issued immediately afterwards may not contain the event, named a "documented capability limit
    ... rather than a bug to report" at :456-462, with an "# Errors" section beneath it. `head` at
    :517-547 — "# What a frontier is, and why it is not max(position)" at :521, the frontier's
    definition, that it TRAILS what append returned, and ES-30's
    head_is_the_highest_visible_position cited as asserting a bound rather than an equality at
    :535-537 (the rule in crates/happenstance-testkit/src/suite.rs; the clause at
    spec/SPECIFICATION.md:4044-4047). Consistency with spec/SPECIFICATION.md:2883-2886 was checked
    in the same read. Rendered:
    target/doc/happenstance_postgres/event_store/struct.PostgresEventStore.html carries
    #method.append and #method.head, and both statements are on that page without scrolling to the
    crate root; the struct's own doc adds "# What this store does not promise" at
    event_store.rs:123-131. Gate: the `documentation` step green.
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
  satisfied: true
  evidence: >-
    Branch taken — the planning magnitude is STATED, not left open. EC-004 did not fire, because
    HS-S0065's re-measurement against the built adapter exists. Scope at
    crates/happenstance-postgres/src/lib.rs:61-67 ("anywhere on the cluster", "not on this table and
    not in this database", with the migration / batch job / idle-in-transaction / unrelated-tenant
    list); the budget at :68-79 — "Sub-millisecond when nothing else is holding a write transaction
    open, and the whole remaining duration of the longest one that is otherwise. The remainder of
    the holder, not a fraction of it." Both endpoints travel with their conditions — a median of
    0.593 ms with no holder; 4,799 ms behind a five-second held write transaction writing to its own
    unrelated table; and the unguarded control arm unaffected by the identical hold at 0.595 ms,
    nine samples per cell — checked verbatim against
    experiments/position-visibility/results/adapter-remeasurement.md:91-114 and
    references/adr/0024-position-visibility-mechanism.md:123-151. These are the ADAPTER figures and
    deliberately not staleness_pinned.txt's phase-2 pair (0.688 / 4010.719 ms): AC-004 requires the
    bill to describe the mechanism as shipped, and phase 2 measured four SQL scripts with no
    unguarded control, which ADR-0024 §4 states in terms. Limit-not-knob wording at :33-35 ("not a
    defect to report, not a setting, and not something an operator can tune down") and at :61.
    Sub-question 3 of .kb/open-questions/postgres-arm-c-structural-cost.md is closed through the
    staged disposition at .kb/_intake/2026-09-07-adr-0024-position-visibility-mechanism.md:70-79,
    which now records that the figure sits on the crate root where a consumer meets it. The atom
    itself is not hand-edited; `redkiln validate --kb` is owed at /redkiln:kb-ingest.
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
  satisfied: true
  evidence: >-
    EC-001 did NOT fire — ADR-0024 chose arm C, so the arm-C bill is the one that ships.
    Correspondence, line for line. Mechanism:
    references/adr/0024-position-visibility-mechanism.md:38-52 (xid8 + pg_snapshot_xmin, the guard
    moved to the read side, writers unserialised) against
    crates/happenstance-postgres/src/lib.rs:38-44. Frontier: ADR :146-151 and :232-233 against
    lib.rs:45-52 and event_store.rs:517-547. No read-your-own-writes: ADR :132-134 and :232-233
    against lib.rs:54-59 and event_store.rs:456-462. Staleness and the planning figure: ADR
    :127-151 against lib.rs:61-79. Losing arms: ADR :153-169 (A at 0.062 and B-const at 0.033 on
    cost; B-tag at 0.935 on the invariant) against lib.rs:98-105 and event_store.rs:85-92.
    Backstop: crates/happenstance-postgres/src/event_store.rs:35-101 is now one shipped mechanism
    rather than the three-candidate enumeration it carried, so a mismatch would be visible in the
    same file. The ADR and the bill were written in one context, the ADR first.
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
  satisfied: true
  evidence: >-
    Branch (a) — NO NEW CLAUSE — decided and recorded, with its reason and its loser named. Written
    into ADR-0024's long form at references/adr/0024-position-visibility-mechanism.md:228-283 (§9,
    "Where the bill is stated normatively: no new clause, and its loser named") and into its
    Consequences at :298-301, and into the staged intake document at
    .kb/_intake/2026-09-07-adr-0024-position-visibility-mechanism.md:88-111 so the ingested atom
    carries it. EC-002 did NOT fire: ADR-0024 is not yet an accepted atom — .kb/decisions/ runs 0023
    then 0029 and the record is still staged — so the verdict rides ADR-0024's own record and no ADR
    number is minted here. The decisive reason is EC-005's: the three consequences are PERMISSIONS
    granted to an adapter rather than requirements binding one, so no conformance rule can fail an
    adapter over them, and a clause naming no rule is decorative (CF-4). The only testable sentence
    in the vicinity is already head_is_the_highest_visible_position under ES-30
    (spec/SPECIFICATION.md:4044-4047). VT-12's within-one-editing-pass drift is the second reason
    (spec/SPECIFICATION.md:2902-2906; VT-12 retained as a cross-reference at :1086), and that drift
    is demonstrated rather than predicted — ES-10's frozen prose still quotes phase 2's 0.688 /
    4010.719 ms pair that ADR-0024 §4 supersedes. The phase-12 auditor's reading order is written
    out at references/adr/0024-position-visibility-mechanism.md:270-283.
    spec/SPECIFICATION.md is unedited, so branch (b)'s spec-trace obligation does not arise;
    `cargo xtask spec-trace` was run anyway and reports "traceability: no problems found; §7.1–§7.2
    matches the checker". `redkiln validate --kb` is owed at /redkiln:kb-ingest, a human-driven
    command and not this story's to run.
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
  satisfied: true
  evidence: >-
    Both stale advertisements are gone. "# Status: not implemented" and "every body that would touch
    a server is todo!()" are replaced by crates/happenstance-postgres/src/lib.rs:3-15, which states
    the event store implemented at 101/101 against live PostgreSQL 17.10 and scopes the remaining
    skeleton to the projection store — the one path this slice did not make real. The "# Open
    decisions — how ES-10 is bought" bullet is deleted; "# Still open" at :107-117 carries only TLS,
    tag matching having also been settled by the shipped migration's text[] column and GIN index
    (migrations/0001_event_log.sql:43,80). The struct-level "# Status: not implemented" at the old
    event_store.rs:121 is gone, replaced by "# What this store does not promise" at
    event_store.rs:123-131. The module ES-10 note at event_store.rs:35-101 now reads as one shipped
    mechanism with one naming of the arms that lost, in place of the three-candidate comparison that
    ended "Nothing above is a measurement, and the choice is owed one." Heading count read off the
    rendered page: target/doc/happenstance_postgres/index.html has exactly FIVE authored sections —
    #status, #why-this-crate-exists, #what-this-store-costs-a-caller, #still-open,
    #not-the-neon-adapter — against a budget of at most five and four before; the rendered text
    contains neither "Open decisions" nor "not implemented". `cargo xtask ci --fast` green over the
    change ("all required checks passed"), and `cargo xtask affected --base main` green ("affected
    gate passed").
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
  satisfied: true
  evidence: >-
    Composed rustdoc throughout — the bill is //! and /// with real # headings, and it renders:
    target/doc/happenstance_postgres/index.html and
    target/doc/happenstance_postgres/event_store/struct.PostgresEventStore.html both carry it, the
    latter under #method.append and #method.head. The frontier explanation that previously rendered
    nowhere (a // comment inside head's body at the old event_store.rs:483-499) is promoted to ///
    at :517-547 rather than rewritten from scratch. One fenced code block in the bill,
    crates/happenstance-postgres/src/lib.rs:80-96, and it is a checked no_run fence — `cargo test -p
    happenstance-postgres --all-features --doc` compiles it, reporting "lib.rs - (line 82) - compile
    ... ok", and the gate reaches it through its `tests` step (xtask/src/main.rs:179-190). No
    feature-gated intra-doc link was added: the crate root names PostgresEventStore and event_store
    in prose and inside the fence rather than linking them, because both sit behind the event-store
    feature, and the only crate-root link — [EventStore](happenstance_core::EventStore) — is to an
    unconditional dependency, which is the house spelling RS-70-2 prescribes at
    standards/rust/70-rustdoc-obligations.md:93-148. rustdoc::broken_intra_doc_links is deny
    (Cargo.toml:224) and the `documentation` step ran green under RUSTDOCFLAGS=-D warnings. Nothing
    load-bearing sits behind #[doc(hidden)], a non-default feature or --document-private-items: the
    bill renders on the default-feature page. No public item added, no signature or visibility
    changed, and no custom HTML, CSS or JavaScript introduced — the only doc attribute in play is
    the pre-existing #![doc(html_no_source)].
  mount_point: "crates/happenstance-postgres/src/lib.rs and crates/happenstance-postgres/src/event_store.rs"
  verifying_test: "`cargo test -p happenstance-postgres --doc`, reached by the gate's `tests` step (xtask/src/main.rs:143); gate step `documentation` with rustdoc::broken_intra_doc_links denied (xtask/src/main.rs:290-300); `semver compatibility` job (.github/workflows/ci.yml:279)"
```
