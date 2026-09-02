---
item: HS-S0115
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — CF-27's experiment run, and the pass list recorded as data

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
  criterion: "GIVEN the maintainer must be able to say a store that holds only a suffix of its own log was exercised against the full contract, and DA-1 requires two shapes because a suffix-only run makes a floor look correct, WHEN the instrument's test target is run, THEN the entire registered rule set has executed under `event_store_conformance!` twice — once over a suffix-retained configuration and once over a scattered hole with survivors below it — as two distinctly named modules in the one target, with neither run's rule set a subset of the other's."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `suffix_conformance` and `scattered_conformance` `event_store_conformance!` invocations, via `cargo test -p happenstance-testkit --test completeness_instrument`"

- id: AC-002
  criterion: "GIVEN the evaluator must never be able to mistake \"the suite observed nothing\" for \"the suite observed and agreed\" — a declined rule is a passing #[test] carrying a SKIP line on suppressed stdout — WHEN the census runs a rule the fixture declines by capability, or a rule whose assertions panic, THEN the first is recorded as `SKIP <rule> — <CAPABILITY>: <reason>` carrying the fixture's own declined reason verbatim and is excluded from the pass list, and the second is recorded as `FAIL <rule> — <message> @ <file>:<line>` without aborting the remaining probes."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs::census_distinguishes_pass_skip_and_fail"

- id: AC-003
  criterion: "GIVEN HS-S0116 and HS-S0117 each add a rule to `for_each_event_store_rule!` and change this census's denominator, WHEN the rule set moves, THEN the census's rule names move with it without any edit to this story's code, because the only source of names is `for_each_event_store_rule!` feeding `__emit_rule_names`, and no literal rule-name list exists anywhere in this diff's generation path."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs::census_rule_names_match_the_registry_exactly (plus `no_orphan_rules` at crates/happenstance-testkit/src/registry.rs:410 staying green)"

- id: AC-004
  criterion: "GIVEN CF-27's question is comparative — the rules that cannot tell a pruned store from a young one — so a list produced without a control is not an answer to it, WHEN the census runs, THEN it also runs unchanged against `MemoryFixture` in the same binary and the same run; the recorded pass list is the set of rules that PASS on both the control and the configuration; any rule that FAILs in the control is disqualified from every list and reported as a harness fault rather than as a finding; and any rule whose SKIP (capability, reason) pair differs between the control and a configuration is recorded as a finding rather than absorbed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs::control_census_reports_no_failure and ::skip_reasons_agree_with_the_control"

- id: AC-005
  criterion: "GIVEN DA-3's two dishonest resolutions would satisfy AC-003 by the letter while destroying AC-002 — the pass list is evidence only if every failure excluded from it is about completeness — WHEN the census is produced, THEN the four seeding-sensitive rules DA-3 names (`reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none`, `condition_against_an_empty_store_admits_the_append`, `two_fixture_instances_observe_none_of_each_others_appends`) appear as PASS in all three censuses, and the artefact states that confirmation in its own text rather than leaving it to a reviewer's inference."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs::connect_presents_an_empty_store_in_every_configuration, plus the confirmation sentence in experiments/completeness-pass-list/README.md"

- id: AC-006
  criterion: "GIVEN the next author adds an unrelated rule and must not be taught to edit the evidence until the gate goes green, WHEN the census is generated, THEN exactly one #[test] fails unless every registered rule appears exactly once in each census with exactly one verdict, and that test asserts nothing about which rules pass — no golden comparison against recorded content exists anywhere in the diff."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs::census_covers_every_registered_rule_exactly_once"

- id: AC-007
  criterion: "GIVEN the evaluator opens the evidence cold and the maintainer must tell a stale list from a wrong one, WHEN they open `experiments/completeness-pass-list/`, THEN they find exactly three same-format `results/*.txt` files (suffix, scattered, control), each opening with a header block of five labelled lines — subject, the retained predicate stated in words and never as positions, the rule count it was produced against, the one-line statement of what the list is in CF-27's wording, and the regeneration command — followed by exactly one line per registered rule in `for_each_event_store_rule!` order and nothing else; and `run.sh` regenerates all three byte-identically on the same commit."
  satisfied: false
  evidence: ""
  mount_point: "experiments/completeness-pass-list/ (README.md, results/*.txt, run.sh), generated from crates/happenstance-testkit/tests/completeness_instrument.rs"
  verifying_test: "crates/happenstance-testkit/tests/completeness_instrument.rs::census_render_is_stable_across_runs, plus a second run of experiments/completeness-pass-list/run.sh leaving `git diff --stat experiments/completeness-pass-list` empty"

- id: AC-008
  criterion: "GIVEN ADR-0028 (HS-S0121) must inherit evidence, not a conclusion, and DT-7 is HS-S0120's to settle, WHEN the writeup is read, THEN README.md names which of DA-3's three outcomes holds with the other two named as the outcomes that did not hold; records the suffix-versus-scattered comparison as data for DT-7 without drawing DT-7's conclusion; and, if outcome (1) holds, states it as CF-27's predicted result and the strongest available evidence for ES-39 rather than as an error or a reason to re-tune the instrument."
  satisfied: false
  evidence: ""
  mount_point: "experiments/completeness-pass-list/README.md"
  verifying_test: "Content review of experiments/completeness-pass-list/README.md against .bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:255-274 and :363-379, and against spec/SPECIFICATION.md:8049-8055"

- id: AC-009
  criterion: "GIVEN the maintainer must be able to say the answer came from the run and not from the run being arranged, and that the instrument is an instrument first and a target never, WHEN this PR's diff is inspected, THEN it adds no name to `for_each_event_store_rule!`, no REGISTRY row, no Defect, no marker move, no `.kb/` atom, no port surface and no `pub`/`pub use` that reaches the instrument out of `crates/happenstance-testkit/tests/`; and the story gate is green on the result."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs (the instrument stays inside this test target; nothing is exported)"
  verifying_test: "`cargo xtask affected --base main` (.redkiln/config.yaml:40) green; `no_orphan_rules` at crates/happenstance-testkit/src/registry.rs:410 green with an unchanged registered set; `cargo package --list` unchanged for the three publishable crates"
```
