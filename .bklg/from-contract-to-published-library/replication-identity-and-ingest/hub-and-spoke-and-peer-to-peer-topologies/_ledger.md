---
item: HS-S0110
stage: implement
created: 2026-08-12T13:47:51.212Z
updated: 2026-08-12T13:47:51.212Z
---

# Acceptance ledger — Both topologies are first class, and hub-ness is an edge

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Per the project testing brief's *Merge-gate commands* (`…/_decomposition.md:775-800`) and
`.redkiln/config.yaml`'s `require_ledger: true`, each row's evidence names **which** command
produced it: a green `cargo xtask ci --fast` proves the gate passed, never on its own that AC-001's
*simultaneously* is the thing that ran, or that AC-003's rule reported `Ran` rather than `Skipped`.

Two rows carry an outcome rather than a result. **AC-008** is satisfied by SY-10's falsification test
having been run in the honest direction and its outcome recorded either way — a positive result stops
the story and routes a new decision atom to HS-S0113 (EC-003), and "we did not get to it" is not an
outcome. **AC-007** is satisfied only if the recorded finding about SY-10's `Rule:` field naming the
wrong tier exists; a differently-shaped rule wearing that clause's name does not satisfy it and makes
`spec-trace` lie.

```yaml
- id: AC-001
  criterion: "One node is a hub and an equal at the same time, and nothing in the library asks it to choose. GIVEN an application author deploying the Kestrel Rotor arrangement — one vessel store that is authoritative to nine technician tablets over ship's wifi and a symmetric peer to a shore depot over satellite — WHEN they construct **one** adapter value and attach it to both edges, THEN both edges exchange and both complete, with no `is_hub` argument, no role parameter, no `HubPeer`/`SpokePeer` split and no second adapter instance anywhere on the path. The two edges are **both in flight before either exchange completes**: a test that drives one edge to quiescence and then the other satisfies the words and not the claim (SY-9, `spec/SPECIFICATION.md:6090-6106`)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs"
  verifying_test: "crates/happenstance-sync-testkit/src/rules.rs::one_adapter_serves_both_roles, run via crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs"

- id: AC-002
  criterion: "A green run means the whole bar was applied, including the newest question. GIVEN an adapter author who invokes `sync_peer_conformance!` and reads \"all passed\", WHEN SY-9's rule exists in the `rules` module, THEN it is enumerated in `for_each_sync_peer_rule!` and therefore emitted by **all three** harnesses — tokio, blocking and `wasm32` — from one enumeration; a rule defined and not enumerated fails the meta-test in both directions and names itself. The author adds nothing per harness to receive it."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/registry.rs"
  verifying_test: "crates/happenstance-sync-testkit/src/registry.rs::no_orphan_sync_rules, plus the rule's presence in crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs, …_blocking.rs and …_wasm.rs"

- id: AC-003
  criterion: "The headline question is actually asked of the always-on peer, and declining it still costs a printed line rather than a vanished test. GIVEN the reference oracle `MemorySyncPeerFixture`, WHEN the suite runs, THEN `one_adapter_serves_both_roles` reports **Ran**, not `Skipped` — `BOTH_ROLES` is supported, because a fixture that can present one local store with two distinct far sides has no honest reason to decline. AND GIVEN a fixture that genuinely cannot, WHEN it declares `BOTH_ROLES` as `declined(\"…\")`, THEN the rule is still emitted as a test, returns `RuleOutcome::Skipped`, and the harness prints the fixture's own stated reason — so DR-8's instrument survives the flip instead of being retired by it."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/src/fixtures.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/memory_peer_conformance.rs (outcome asserted `Ran`) and crates/happenstance-sync-testkit/tests/both_roles_declined.rs::declining_both_roles_reports_the_fixtures_stated_reason"

- id: AC-004
  criterion: "A fleet is a hub and *many* spokes, and the library can tell them apart. GIVEN an application author running one authoritative store with at least **two** spokes whose progress has diverged — one caught up, one far behind, each holding facts the other has never seen — WHEN the runner drives every edge to quiescence, THEN each spoke holds the union of what the hub holds for it, the hub holds both spokes' facts, and the hub's `Watermark` advances **per origin `StoreId`** rather than as one number. Collapsing `Watermark` to a scalar must make this scenario fail: a one-spoke hub is a peer pair with a label and proves nothing (`crates/happenstance-sync/src/identity.rs:208-215`)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/hub_with_two_spokes.rs::divergent_spokes_need_a_version_vector"

- id: AC-005
  criterion: "Peers converge on what they hold, not on what they originated — even when they never meet. GIVEN the symmetric arrangement A—B—C in which A and C never communicate and each node syncs only with its neighbour, WHEN every edge is driven with the same adapter type in both directions, THEN all three converge on the union of facts keyed by `EventId`, C receives A's events by way of B, and no node's forwarding decision depends on which store originated a fact (E2E-42, `spec/E2E-CASES.md:1105-1124`). Arrival order is not a property any assertion may rely on (SY-19)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/transitive_convergence.rs::a_and_c_converge_without_ever_meeting"

- id: AC-006
  criterion: "The new rule can fail, and the thing it fails is written down and plausible. GIVEN the adapter author trusting the suite to discriminate, WHEN `HubFlaggedPeer` — `Peer::new(is_hub: bool)`, SY-9's own named rejection — is run against the suite, THEN it fails **exactly** `one_adapter_serves_both_roles` at a pinned assertion and passes every other rule; it is registered in the `Declared` table with a non-empty provenance naming the vessel that is a hub to nine tablets and a peer to a shore depot at once; and `CHANGELOG.md` carries an entry naming the **defect the rule detects**, not the rule (CF-1 – CF-4, CF-29, `spec/SPECIFICATION.md:7161-7232`, `:8141-8150`). A rule with no registered mutant is decorative and CF-1 makes it a build failure."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs"
  verifying_test: "crates/happenstance-sync-testkit/tests/mutation_coverage.rs (the `Declared` entry for `HubFlaggedPeer` with its per-rule `expect` pin), plus `cargo xtask lints` for the CHANGELOG.md entry"

- id: AC-007
  criterion: "A hub and a spoke may merge differently across one edge, and the runner permits it without knowing which end is which. GIVEN an application author whose hub adjudicates across every log while a spoke sees only its own, WHEN they attach a different merge policy to each direction of the **same** edge, THEN both directions run and the runner exposes no role parameter to make that possible — policy is per direction, per edge, and never a property of the adapter's type. The exercise lands at runner grain, in `crates/happenstance-sync/tests/`, and **not** as a suite rule: SY-8 is `[FROZEN]` that a rule needing two peer handles is testing the runner, SY-10 is `[PROVISIONAL]`, and the frozen clause wins. The finding — *SY-10's `Rule:` field names a testkit rule and points at the wrong tier* — is recorded verbatim in the implementation report and handed to `frozen-clause-repairs` and `clause-arithmetic-and-deferral-renewals`; `directional_merge_rules_compose` keeps its `(new)` marker and SY-10's marker is untouched here."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/directional_merge_rules.rs::a_different_rule_in_each_direction_of_one_edge, with `cargo xtask spec-trace` green and SY-10's marker and `Rule:` line unchanged"

- id: AC-008
  criterion: "The clause that is only provisional gets its experiment run, and an inconvenient answer stops the story instead of being absorbed by it. GIVEN SY-10's own falsification test — \"implement the hub's rule on a spoke and show the spoke's projections stay correct\" (`spec/SPECIFICATION.md:6112-6117`) — WHEN it is run in the honest direction, THEN the outcome is recorded either way. If the spoke's projections stay correct, SY-10 is falsified: **stop**, record the finding, raise a new decision atom and route the marker change to `clause-arithmetic-and-deferral-renewals` — never relax a clause in this diff. If they do not, `SymmetricWithDeclinedPush` is refuted **by name**, and what it loses is stated: the hub's adjudication (it holds every log; the spoke does not) and, in Kestrel Cold Chain, a commercial confidentiality boundary — Scottish subcontractors not holding Yorkshire's parts pricing (`spec/SPECIFICATION.md:6120-6128`). \"We did not get to it\" is not an outcome."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "crates/happenstance-sync/tests/sy10_falsification.rs::the_hubs_merge_rule_applied_on_a_spoke, with the outcome written into the story's implementation report"

- id: AC-009
  criterion: "A reader of the crate meets a replication library, not a phase-2 sketch that has quietly become one. GIVEN an application author landing on `happenstance-sync`'s front page on docs.rs, WHEN they read the module documentation, THEN both topologies are described with the reason `Watermark` is a version vector attached (verified against `crates/happenstance-sync/src/lib.rs:52-66` **before** any edit is made — AC-007's third clause may already be satisfied there), every sentence this project has made false is corrected, every new public item carries rustdoc with an `# Errors` section naming conditions rather than error types, and a compiled example shows one adapter on two edges. AND every paragraph deleted has had its **finding** moved into the ADR that consumed it first — the findings are not the same as the `todo!()`s, and this crate's honesty about what the sketch proved is the thing most easily lost in a tidy-up (`…/_decomposition.md:590-599`)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sync/src/lib.rs"
  verifying_test: "cargo test -p happenstance-sync --doc for the compiled example, plus `cargo xtask ci --fast`'s docs step with -D warnings and a reviewed diff over crates/happenstance-sync/src/lib.rs showing :3-15 and :98-133 treated and :52-66 justified"
```
