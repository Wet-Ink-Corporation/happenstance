---
item: "HS-S0065"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0024 — the position-visibility mechanism, decided on numbers

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Two things about this story's ledger specifically.** First, `mount_point` carries the placeholder
`0024-<slug>` because the slug names the mechanism the measurement selects and is not decided until
the number is in hand (spec *Clarifications* item 7); the implementer replaces `<slug>` with the real
one when citing evidence. Second, this story's mount is completed by a **human-invoked**
`/redkiln:kb-ingest` wave, so an intake document under `.kb/_intake/` is never evidence for AC-008 —
only a `file:line` under `.kb/decisions/` plus the `.kb/maps/decision-map.md` row is (spec EC-002).

```yaml
- id: AC-001
  criterion: "GIVEN an evaluator deciding in one sitting whether `happenstance-postgres` is affordable, WHEN they open the accepted decision atom, THEN they find the steady-state cost of the chosen mechanism taken against the **built** `PostgresEventStore` — connection pool, `append`'s transaction lifetime tied to the trait method, cursor read, error mapping all present — stated as a ratio against a baseline re-measured between arms with its residual drift reported, under `fsync=on`, and explicitly **not** the phase-2 SQL-script figure and not a preference."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md — Root D (the knowledge base), reached through .kb/_intake/ and /redkiln:kb-ingest"
  verifying_test: "process: `redkiln validate --kb` over .kb/decisions/0024-<slug>.md; artefact: the HS_TAG-suffixed ratio series committed under experiments/position-visibility/results/ (method: experiments/position-visibility/README.md:209-259)"

- id: AC-002
  criterion: "GIVEN an application author who must size a read-after-write expectation before choosing a database, WHEN they read the record, THEN the held-transaction scenario is reported as its **own** number — append → commit → poll a fresh snapshot until admitted, with a write transaction deliberately held open on the same cluster — placed against its no-holder control, with a stated magnitude the caller should plan for out of the phase-2 range (0.688 ms → 4010.719 ms), and with the bound named as a property of the *cluster*, not of this store's workload."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md — Root D (the knowledge base), reached through .kb/_intake/ and /redkiln:kb-ingest"
  verifying_test: "process: `redkiln validate --kb`; artefacts: the HS_TAG-suffixed control and held transcripts committed side by side under experiments/position-visibility/results/, compared against experiments/position-visibility/results/staleness_pinned.txt"

- id: AC-003
  criterion: "GIVEN an adapter author about to build the seventh store and wanting to know what this mechanism costs to *express*, WHEN they read the record's structural-cost section, THEN each of the four gaps the phase-2 harness never had — connection pooling, a transaction whose lifetime is tied to `append`'s async boundary, the cursor, error mapping — is named with what it actually cost, and the read-side half (the visibility predicate composed into the cursor's `DECLARE` inside one `REPEATABLE READ` transaction rather than evaluated per chunk) is priced as part of that bill rather than omitted."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md — Root D (the knowledge base), reached through .kb/_intake/ and /redkiln:kb-ingest"
  verifying_test: "process: `redkiln validate --kb`; human read of the record's structural-cost section against .kb/open-questions/postgres-arm-c-structural-cost.md:64-70 and the shipped read path crates/happenstance-postgres/src/read_stream.rs:37-77"

- id: AC-004
  criterion: "GIVEN a future reader who must be able to re-derive the choice without re-running an experiment, WHEN they read the record, THEN both losing arms are named **and priced** — the serialised sequence table and the constant advisory lock losing on cost (16× and 30× at 64 writers, throughput flat from 8 clients up: they buy ES-10 by deleting the reason to reach for Postgres) and the tag-keyed advisory lock losing on **the correctness of a different invariant** (nearly free, reproduces the baseline inversion on disjoint keys, because it implements a per-boundary property where ES-10 states a global one) — so that the arms are not ranked by tps."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md — Root D, with its long form retained at references/adr/0024-<slug>.md"
  verifying_test: "process: `redkiln validate --kb`; human read against experiments/position-visibility/README.md:128-184, :263-296, :398-412 and crates/happenstance-postgres/src/event_store.rs:43-75"

- id: AC-005
  criterion: "GIVEN an adapter author who will inherit this decision after phase 6 has shaped the projection checkpoint, WHEN they read the record, THEN it says in its own words that ES-10's **global** framing rests on the checkpoint being global, that this is unsettled and owned by phase 6, and that a boundary-scoped checkpoint reopens ADR-0013 and this measurement with it — stated as an inherited premise this record does not own, never as a permanently settled invariant."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md — Root D, linked outbound to .kb/open-questions/global-versus-per-boundary-visibility-invariant.md"
  verifying_test: "process: `redkiln validate --kb` (link resolution plus accepted-decision immutability against HEAD, which is what proves .kb/decisions/0013-position-assignment-and-visibility.md was not edited)"

- id: AC-006
  criterion: "GIVEN an adapter author asking \"did this rule actually cost the implementer anything, or is it decorative?\", WHEN they read the record's evidence, THEN the claim that `nothing_below_an_observed_position_appears_later` was hard-won is carried by **citation of the two predecessor runs** — the naive-`nextval()` control observed *failing* that rule, and the concurrency family green at `CONTENDERS = 8` under a multi-thread runtime with writers unserialised — each attributed to the run that produced it, with what the passing mechanism cost stated."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md — Root D; the cited rule is crates/happenstance-testkit/src/suite.rs:5880"
  verifying_test: "cited, not re-run: `happenstance_testkit::rules::nothing_below_an_observed_position_appears_later` (crates/happenstance-testkit/src/suite.rs:5880) as run by postgres-rule-controls (HS-S0064), and `event_store_concurrency_conformance!` at CONTENDERS = 8 as run by postgres-concurrency-family (HS-S0063), both in the live Postgres CI job; plus `redkiln validate --kb`"

- id: AC-007
  criterion: "GIVEN the specification's standing debt at `spec/SPECIFICATION.md:2844-2850` — a poll-padding decorator over `PreCommitPositionStore`, owed by phase 10 — WHEN a reader arrives at ES-10 wanting to know whether the rule's strength was ever bounded, THEN this record either reports the decorator's result (does the rule still reject an *n*-poll implementation?) or states in writing why a real multi-poll `append` made it unnecessary. Silence is a failure of this AC, not a neutral outcome."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md — Root D; reports on spec/SPECIFICATION.md:2844-2850 without editing it"
  verifying_test: "`cargo xtask spec-trace` green with ES-10's clause text and markers unchanged; `redkiln validate --kb`; human read against .kb/open-questions/poll-count-bounds-the-visibility-rule.md:74-99"

- id: AC-008
  criterion: "GIVEN a reader who meets the question in the crate before they meet the answer in the knowledge base, WHEN the merge lands, THEN the decision has **mounted at Root D** — an accepted atom at `.kb/decisions/0024-<slug>.md` with valid `KbFrontmatter` (`kind: decision`, `status: accepted`), authored by `/redkiln:kb-ingest` and never by hand, linking its long form at `references/adr/0024-<slug>.md` which is retained rather than folded into the atom, carrying its row in `.kb/maps/decision-map.md`, with `.kb/_intake/` emptied by the wave — **and** `crates/happenstance-postgres/src/event_store.rs:77`'s now-false sentence \"Nothing above is a measurement, and the choice is owed one\" replaced by a pointer to that atom, so the reader is answered where they asked."
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0024-<slug>.md plus its row in .kb/maps/decision-map.md — Root D; and the in-crate pointer at crates/happenstance-postgres/src/event_store.rs:77"
  verifying_test: "`redkiln validate --kb` and `redkiln doctor` (clean, exactly six template-drift advisories); `.kb/_intake/` empty after the wave; `cargo xtask affected --base main` and `cargo xtask ci --fast` green over the rustdoc edit; `test -f references/adr/0024-<slug>.md`"
```
