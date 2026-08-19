---
item: HS-S0115
stage: discover
created: 2026-08-12T13:03:37.975Z
updated: 2026-08-12T13:03:37.975Z
template_sig: 86ce4036
rendered_sig: cc40d337
---

# Discover — CF-27's experiment run, and the pass list recorded as data

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: run the full suite against the instrument in **both** configurations, commit the enumerated pass list per configuration as recorded data, state the DA-3 finding, and confirm no recorded failure is a seeding artefact | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:69` | The deliverable is *data*, not a green tick. "It passed" is the failure this story exists to prevent |
| **AC-002** — the full suite is run and the **enumerated list of rules that pass** is written into this project's artifacts, stated as what it is: the list of rules that cannot tell a pruned store from a young one | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/project.md:198-201` | The pass list is the evidence base for ADR-0028; DR-2 (`project.md:147-149`) names *"it passed"* as the specific defect |
| `dependsOn: retained-set-instrument-and-conformance-mount` (HS-S0114) supplies the instrument, its `Fixture`, its retained-predicate parameterisation and its `event_store_conformance!` mount | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_storymap.md:68`, `:130-132` | This story writes no store and no rule. It runs what HS-S0114 landed and records the result — twice, once per configuration |
| CF-27, in the clause's own words: *"the outcome that matters is the list of rules that pass, which is the list of rules that cannot tell a pruned store from a young one"* | `spec/SPECIFICATION.md:8036-8041` | This story **is** CF-27's experiment. The wording of the recorded artefact should be the clause's wording, so the clause and the evidence are legibly the same claim |
| CF-27's `Rejects:` predicts the opposite of AC-003: after a 90-day prune *"the store passes every rule unchanged, including `query_all_matches_every_event`, whose contract is store-relative by wording and therefore accidentally correct"* | `spec/SPECIFICATION.md:8049-8055` | A pass list reading "all of them" is CF-27's **predicted** result and the strongest available evidence for ES-39 — not a failed experiment. This story must be able to report that without it looking like an error |
| **DA-1** — two configurations are required: a suffix (the prune case) and a scattered hole with survivors below it (the regulated-purge case, E2E-46/E2E-47) | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:150-171` | One pass list per configuration. A single suffix run cannot falsify a floor, which is the outcome ES-39's `Rejects:` names as the path of least resistance (`spec/SPECIFICATION.md:4347-4349`) |
| **DA-3** — the two dishonest resolutions, named so they are recognisable: pre-seeding, and an eager sliding window | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:234-274` | Pre-seeding fails `reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none` and two more — satisfying AC-003 by the letter while destroying AC-002 entirely, because the pass list is evidence only if every excluded failure is about completeness |
| AC-A02 and AC-A04 — a pass list per configuration, and no recorded failure that fails for a seeding reason | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:508-513` | The "no seeding failure" confirmation is part of the artefact, not a reviewer's inference |
| The testing brief's tier for AC-002: the suite run itself in two configurations, plus **the recorded pass list as a committed artifact, not a test assertion** | `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:604` | The pass list is committed data. It is not an `assert_eq!` against a hard-coded list of rule names, which would go red on every unrelated rule addition and teach the next author to edit the evidence |
| Rule names are enumerable at compile time — `for_each_event_store_rule!` feeds `__emit_rule_names` | `crates/happenstance-testkit/src/registry.rs:86-94`; `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:98-106` | The pass list can be produced *from the registry* rather than transcribed by hand, which is what keeps it honest when the rule set moves |
| Gate decision 4 — no published-surface change | `.bklg/from-contract-to-published-library/_decomposition.md:239-249` | Trivially satisfied: this story runs code and writes markdown. Its real contribution to the constraint is evidential — the pass list is what tells HS-S0121 whether the honest answer needs a surface at all |

## Questions

- **DT-7 — one signal or three?** **Deferred to HS-S0120.** But this story supplies its evidence: the pass list, read per configuration, is the direct measurement of how much a reader is *not* told. If the suffix and scattered pass lists are identical, that is an argument that a reader cannot distinguish kinds of hole through the port and therefore that a three-way distinction has nowhere to be reported from — a finding DT-7 should weigh. Record the comparison; do not draw DT-7's conclusion here.
- **Is "fails loudly" expressible in the existing error vocabulary?** **Answered, inherited from HS-S0114 and confirmed by this story's own instrument: no**, not as a typed signal — `Guard::is_violated_by` has two arms and no third (`crates/happenstance-core/src/append.rs:249-252`), `read_decision_model` has no completeness channel (`crates/happenstance-core/src/store.rs:321-331`), and `contains_event_id`'s `false` conflates *"never had it"* with *"minted it and forgot it"* (`crates/happenstance-core/src/store.rs:268`). This story is where that stops being an argument and becomes a measurement: **the pass list is the enumerated proof of it.** Every rule on that list is a rule the conformance suite runs and cannot use to tell the two stores apart. No surface change is proposed here; the option set is DA-7's (`_decomposition.md:363-379`) and the escalation is HS-S0124's.
- **What if the pass list is "all of them"?** **Answered: that is a result, and it is recorded as one.** DA-3's three-way outcome is decided by this run, not before it, and outcome (1) — CF-27's predicted result — is the strongest possible evidence for ES-39. AC-003's two named failures are owed by the two registered mutants (HS-S0116 and HS-S0117), not by the honest instrument.
- **What is the artefact's form and where does it live?** **Deferred to `spec`, with the constraint stated:** it is committed data enumerating rule names per configuration, it is not an assertion in a test, and it must be reproducible from a command a reader can run. Whether it lives under this project's backlog folder, under `experiments/` (the repository's home for reproducible measurements not in the gate, per `CLAUDE.md`), or both is the spec stage's call.
- **Does anything need re-running when the two new rules land?** **Answered: yes, and it is scheduled.** HS-S0116 and HS-S0117 each add a rule to `for_each_event_store_rule!`, which changes the denominator of the pass list. The recorded artefact states the rule-set revision it was produced against, so a later reader can tell a stale list from a wrong one.

## Decision

CF-27 asks for an experiment, not a test, and the reason is that a pre-decided result is not evidence: the specification's own prediction (`spec/SPECIFICATION.md:8049-8055`) is that a pruned store passes every rule unchanged, and the project's AC-003 asks for at least two named failures — both cannot be true, and which one is resolves this project's central question. This story runs the full conformance suite against HS-S0114's instrument in both the suffix and the scattered configuration and commits the enumerated list of rule names that pass, per configuration, stated in CF-27's own words as the set of rules that cannot tell a pruned store from a young one. The spec will cover: the two retained-predicate configurations and why each exists; the mechanism that produces the list from the registry rather than by transcription, so it cannot silently go stale; the recorded artefact's form, location and the command that reproduces it; the rule-set revision it is stamped against; the explicit confirmation that no rule in any recorded failure set fails for a seeding reason (AC-A04); and the DA-3 finding — which of the three outcomes holds — stated as a finding with the other two named, so that ADR-0028 inherits evidence rather than a conclusion.

## The wrong implementation

**The mutant this story rejects is not a store — it is an artefact: a pass list produced by tuning the instrument until the number came out right.** It satisfies every existing check, because every existing check runs against whatever fixture it is handed. Two concrete forms, both named in DA-3 (`_decomposition.md:241-253`) and both of which would leave `cargo xtask ci --fast` green:

- **The pre-seeded fixture.** A `connect()` that hands back a store already holding history fails `reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none`, `condition_against_an_empty_store_admits_the_append` and `two_fixture_instances_observe_none_of_each_others_appends`. Four red rules, AC-003 satisfied by the letter, and **not one of them is about forgetting** — they are about seeding. The pass list is then worthless: it is evidence only if every failure excluded from it is about completeness.
- **The eager sliding window.** A store that forgets continuously fails dozens of rules for the boring reason that a rule writes three events and reads them back. Same defect, more noise, and it reads as a *more* thorough experiment.

There is a third form specific to this story, and it is the one to guard hardest: **a pass list that is a hand-written prose summary** — *"the suite passed apart from a couple of rules"* — which is precisely DR-2's stated failure (`project.md:147-149`) and which cannot be diffed, cannot be re-derived, and cannot be cited by ADR-0028 as evidence. The defence is mechanical: derive the list from `for_each_event_store_rule!`'s own emission (`crates/happenstance-testkit/src/registry.rs:86-94`), stamp it with the rule-set revision, and commit the command that regenerates it.

Note for the record, because it is easy to lose between stories: **the instrument is an instrument first and a target never** (`project.md:141-145`). Nothing in this story may make it easier to reach — no `pub use`, no promotion out of `crates/happenstance-testkit/tests/`, no convenience export to make the experiment runnable from elsewhere. A reproducible experiment does not require a published fixture.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.

**The two ticks that needed judgement.** *Literal position values*: this story adds no conformance rule — it runs the existing set and records the outcome. It is load-bearing anyway, one level up: the recorded artefact must enumerate **rule names**, never positions, and must not record the instrument's first retained position as though it were a property of the design. That position is chosen by the retained predicate and differs between the suffix and scattered configurations; anything downstream that reads the pass list and infers a floor from it has read a parameter as a fact, which is ES-39's argument against `earliest_position()` reproduced in miniature (`spec/SPECIFICATION.md:4336-4341`). *`[FROZEN]` clauses*: none is changed. This story executes CF-27, which is `[DEFERRED]`, and produces evidence about ES-38 (`[FROZEN]`) and CF-25/CF-26 (`[FROZEN]`) without editing any of them; the marker moves are HS-S0123's and the residual CF-26 exposure is recorded open in ADR-0028 (HS-S0121) rather than implied closed here.
