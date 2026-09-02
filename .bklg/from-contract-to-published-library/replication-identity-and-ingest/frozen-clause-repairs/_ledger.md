---
item: HS-S0112
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Every clause still rejects something that exists

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
  criterion: "**GIVEN** an evaluator who wants to know whether the specification still frames ingest as re-checking a writer's conditions — the charter's own named worry — **WHEN** the implementer re-runs `re-check\\|recheck\\|re-evaluat` over `spec/SPECIFICATION.md` at the tree this story starts from, **THEN** the implementation report carries **every** hit with its line number and a one-line verdict against SY-1/SY-6-as-frozen, and states the outcome as a finding in its own words — including *\"null finding: no clause describes ingest as re-checking conditions\"* if that is what the run says — never as a quotation of the HEAD run recorded in `discover.md`"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — the gate this story mounts into; the sweep's record lands in .bklg/from-contract-to-published-library/replication-identity-and-ingest/frozen-clause-repairs/implementation-report.md, re-derivable against spec/SPECIFICATION.md at the merge commit"
  verifying_test: "Review of the recorded hit list in the story's implementation report against .bklg/from-contract-to-published-library/replication-identity-and-ingest/frozen-clause-repairs/discover.md (Signal Ledger row 5), plus re-running the sweep over spec/SPECIFICATION.md on the merge commit"

- id: AC-002
  criterion: "**GIVEN** a maintainer who removes a `(new)` marker from an `SY` clause, **WHEN** `cargo xtask spec-trace` runs, **THEN** that clause's `Rule:` names are actually resolved — because `has_suite` (`xtask/src/spec_trace.rs:1735-1737`) admits `SY-` alongside `ES-`, `VT-` and `WF-` (three prefixes to four), still refuses `PS-`, and its rustdoc states why the asymmetry is deliberate: the replication suite is now in `RULE_FILES` and the projection suite is not, so admitting `PS-` would report every `PS` rule as missing — *\"noise indistinguishable from a real typo\"* (`:684-688`)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — `has_suite` (:1735-1737), the predicate check 4's loop (:694-711) is guarded by"
  verifying_test: "xtask/src/spec_trace.rs::tests::has_suite_admits_sy_and_still_refuses_ps, green under `cargo test -p xtask`"

- id: AC-003
  criterion: "**GIVEN** the same maintainer, but they have de-marked a clause whose named rule does **not** exist — a typo, or a rule the slice never landed — **WHEN** `cargo xtask spec-trace` runs, **THEN** the gate is **red**, naming the clause id and the missing rule in the existing check-4 message form (`… — SY-n names rule \\`x\\`, which is not in … and the clause does not declare it new`, `xtask/src/spec_trace.rs:694-711`), and the implementation report carries the transcript of that failure demonstrated against the real tree and then reverted"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — check 4's loop (:694-711), reached for `SY` clauses only after AC-002's predicate change"
  verifying_test: "xtask/src/spec_trace.rs::tests::de_marked_sy_clause_reports_its_missing_rule_by_name under `cargo test -p xtask`, PLUS the recorded transient-violation transcript of `cargo xtask spec-trace` against spec/SPECIFICATION.md (demonstrate, capture, revert)"

- id: AC-004
  criterion: "**GIVEN** an adapter author who follows SY-1's `Rule:` field expecting to find `ingest_never_rejects`, **WHEN** they read `spec/SPECIFICATION.md` after this PR, **THEN** every `SY`/`WF` clause whose `Rule:` names rules that **all** now resolve against `RULE_FILES` has lost its `(new)`/`†` scheduling marker, every clause with an unresolved name has **kept** its marker with the unresolved name recorded, and the set was **computed from the tree** — the ledger names each drop together with the file the rule resolved in, and the enumeration exists nowhere in this spec because slices 3–6 landed rules a plan written earlier cannot list without going stale"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — `RULE_FILES` (:85-89) and the marker logic in the clause parser (:1626-1634); the drops themselves land in spec/SPECIFICATION.md's `Rule:` fields"
  verifying_test: "`cargo xtask spec-trace` green (load-bearing only after AC-002), plus the per-drop table in the implementation report cross-checked against xtask/src/spec_trace.rs::collect_rules over RULE_FILES"

- id: AC-005
  criterion: "**GIVEN** a maintainer tempted to make a clause resolvable by tidying its prose, **WHEN** the diff is reviewed, **THEN** only `(new)` and `†` scheduling markers were removed: the words *unit test*, *compile test* and *meta-test* — the `elsewhere` family at `xtask/src/spec_trace.rs:1626-1631` — are untouched in every clause, WF-12 in particular still reading *\"a const-evaluation assertion … **not** a compile test\"* (`:1617-1624`), because removing those two words would change what the clause says its rule **is** and turn WF-12 into a `†` no test can ever clear"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the `Rule:` fields of every `SY`/`WF` clause; xtask/src/spec_trace.rs:1626-1634 is the mechanism that defines what a marker is"
  verifying_test: "Review of `git diff main -- spec/SPECIFICATION.md` for any removal of `unit test`/`compile test`/`meta-test`, plus `cargo xtask spec-trace` green (a WF-12 tidied into resolvability goes red on the missing `read_options_is_not_serialisable`)"

- id: AC-006
  criterion: "**GIVEN** an evaluator auditing whether the specification was checked or merely edited, **WHEN** they open the implementation report, **THEN** **every** id in the pre-committed audit list — SY-1, SY-2, SY-6, SY-9, SY-10, SY-12, SY-15, SY-17, plus every clause whose scheduled rule this project landed (`…/frozen-clause-repairs/discover.md`, *Questions* 3) — carries exactly one verdict from {**keeps a live target**, **repaired**, **gap**} with its evidence, no id is absent, and where the verdict is **gap** the recorded finding sits **inside the clause it is about** with nothing normative changed and a blocker raised naming what an ADR would have to decide"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the audit-list clauses' `Rejects:` fields and in-clause findings; the verdict table lands in the story's implementation report"
  verifying_test: "Review of the per-id verdict table against .bklg/from-contract-to-published-library/replication-identity-and-ingest/frozen-clause-repairs/discover.md (Questions 3); for any `gap`, the in-clause finding visible in `git diff main -- spec/SPECIFICATION.md` and the blocker on the story item"

- id: AC-007
  criterion: "**GIVEN** an evaluator who opens SY-12 — the clause that has already survived one withdrawal and records it in its own text (`spec/SPECIFICATION.md:6173-6183`) — after `memory-store-ingest-seam` and `ingest-store-and-memory-peer-round-trip` made `impl IngestStore for MemoryEventStore` truthful, **THEN** its `Rejects:` either names a **new wrong implementation someone would actually ship** or records a **discharge in the three-part form** — the MUST **verbatim**, the discharge named as a discharge, and the code **and** the test that now assert it cited — and in neither case is the field deleted or replaced by a generic restatement such as *\"a peer that carries identity somewhere unreachable\"*"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:6153-6207 — SY-12's `Rejects:` field and citations; the rule/citation it names is resolved by xtask/src/spec_trace.rs"
  verifying_test: "Review of the SY-12 hunk against .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md (The safe form for a discharged MUST) and spec/SPECIFICATION.md:7208-7215 (CF-4's saboteur bar); `cargo xtask spec-trace` resolving the cited rule and citation; the cited test confirmed to exist and to assert what the discharge claims"

- id: AC-008
  criterion: "**GIVEN** an evaluator reading SY-1 and SY-6, whose `Rejects:` fields both name the **public** `guard: Option<AppendCondition>` field on `EventGroup` as the reason re-evaluation is reachable at all (`spec/SPECIFICATION.md:5856-5867`, `:5998-6006` → `crates/happenstance-sync/src/peer.rs:236-243`), **THEN** the disposition of that field at the merge tree decides the verdict and the verdict is recorded: field still public → **keeps a live target**, verdict recorded and the field left alone; field made private, renamed or removed by ADR-0027's slice → **repaired in this same commit**, in the three-part form, with the mechanical test's answer written down"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md:5841-5867 and :5973-6029 — SY-1's and SY-6's `Rejects:` fields, whose target is crates/happenstance-sync/src/peer.rs:236-243"
  verifying_test: "Read crates/happenstance-sync/src/peer.rs at the merge tree and record the field's visibility; review the recorded verdict against .bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md AC-A11 (a repair in the same commit)"

- id: AC-009
  criterion: "**GIVEN** an evaluator who follows a clause's `file:line` citation into `crates/happenstance-sync/` — module prose this project rewrites wholesale — **THEN** every citation this story re-anchored was **opened and read first**, the implementation report records what the new referent actually says in the implementer's own words, and where the sentence the clause needs has moved out of the code and into a decision record the citation points at the record rather than at whatever now occupies the old lines. Specifically covered: SY-9/SY-10 at `spec/SPECIFICATION.md:6120-6121` → `crates/happenstance-sync/src/lib.rs:57-63`; SY-6's citation of `crates/happenstance-sync/src/lib.rs:100-104`; SY-15/SY-17's neighbourhood at `spec/SPECIFICATION.md:5775` naming the `tests/real_peer_shapes.rs` stand-ins"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the citations of SY-6, SY-9, SY-10, SY-15 and SY-17 into crates/happenstance-sync/src/lib.rs and its tests; the citation check in xtask/src/spec_trace.rs resolves them but cannot read them"
  verifying_test: "The per-citation \"what it now says\" record in the implementation report, spot-checked by review against crates/happenstance-sync/src/lib.rs at the merge tree; `cargo xtask spec-trace` green on citation resolution as the necessary-but-insufficient half"

- id: AC-010
  criterion: "**GIVEN** two findings handed forward to this story by earlier slices, **WHEN** the implementation report is read, **THEN** each is closed **explicitly**: (a) `SyncPeer::push`'s rustdoc — *\"Transport- or authorisation-level refusals only. A peer may not refuse a push because it disagrees with the events in it\"* (`crates/happenstance-sync/src/peer.rs:140-148`) — either gains one clarifying sentence distinguishing refusal of a structurally unreplicable **condition** from disagreement with an **event**, or carries the evidence that a slice-2–5 rewrite already says it; (b) `ingest-store-and-memory-peer-round-trip`'s recorded list of `spec/SPECIFICATION.md` and test citations its diff invalidated is consumed entry by entry, each ending *discharged* or *no longer applicable, because …*"
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/peer.rs:140-148 — the `SyncPeer::push` doc comment (doc comments only, no item or signature); the closeout table lands in the story's implementation report"
  verifying_test: "`cargo test -p happenstance-sync --doc` green; review of the doc comment against standards/rust/70-rustdoc-obligations.md; review of the entry-by-entry closeout table whose input list was located by grepping earlier implementation reports for both HS-S0112 and HS-S0114"

- id: AC-011
  criterion: "**GIVEN** the repository owner reviewing this PR against project DoD 7 — *\"No `[FROZEN]` clause was edited, and `git diff` over `spec/SPECIFICATION.md` shows only additions a decision record authorises\"* — **WHEN** they read `git diff main -- spec/SPECIFICATION.md`, **THEN** every hunk is a `Rule:` field, a `Rejects:` field, a citation, an in-clause recorded finding, or a `--write`-regenerated §7.1/§7.2 region: **no MUST, MUST NOT, SHOULD or MAY changes by a character**, **no `[FROZEN]`/`[PROVISIONAL]`/`[DEFERRED]` marker moves**, no clause id changes, no `Cases:` field changes, and each repair hunk is traceable to the ADR-0026 authorisation that permits it"
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the whole diff, including the generated §7.1/§7.2 regions held equal to xtask/src/spec_trace.rs's computation (:735-744, marker at spec/SPECIFICATION.md:8717)"
  verifying_test: "The DoD-7 review pass over `git diff main -- spec/SPECIFICATION.md`, hunk by hunk; `cargo xtask spec-trace` green after `cargo xtask spec-trace --write` regeneration; `cargo xtask ci --fast` green"
```
