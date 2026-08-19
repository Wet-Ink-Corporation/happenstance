---
item: "HS-S0099"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ADR-0027: the merge rule, the compensation contract, and the message set

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Six of these nine criteria are claims about *reasoning*, not about a build, and the spec says so in
its *Tests and CI* section. Their evidence is a `file:line` into `references/adr/0027-*.md` or
`.kb/decisions/0027-*.md` — never a green gate on its own, which proves only that the tree did not
move.

```yaml
- id: AC-001
  criterion: "The decision arrives the way this repository makes decisions. GIVEN an adapter author who wants to know how the merge rule was settled and by whom, WHEN they look under `.kb/decisions/`, THEN an accepted atom `kb-decision-0027` is there, introduced by a single `/redkiln:kb-ingest` wave commit under a wave id distinct from `2026-08-10-intake` and `-intake-2`, with the draft it consumed deleted from `.kb/_intake/` in that same commit and `.kb/_intake/README.md` surviving it. A byte-perfect hand-written atom does not satisfy this criterion."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; git log --oneline --diff-filter=A -- .kb/decisions/0027-*.md; git ls-files .kb/_intake/"
- id: AC-002
  criterion: "The reasoning survives the summary. GIVEN the same author, now needing the transcripts and the alternatives a ~100-line atom cannot hold, WHEN they follow the atom's `source_paths`, THEN `references/adr/0027-*.md` exists in the house shape — Status / Date / Settles / Corrects / Rests on and does not settle — carries every rejected alternative at full length, and is reachable from the atom rather than inferred from the number."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; references/adr/0027-*.md header block diffed against references/adr/0016-the-wire-format.md:1-25"
- id: AC-003
  criterion: "The author learns which half of the merge is theirs. GIVEN a peer author whose `ingest` has just found an incoming event conflicting with a fact the receiver already holds, WHEN they read ADR-0027, THEN they learn that the compensation is appended in the same `append` call as the losing event (SY-2), that the port supplies atomicity, identity and idempotence while the domain supplies the compensation's content through a caller-supplied closure (SY-3), and that authorship of a compensation belongs to at most one peer per fact family as runner configuration (SY-7) — each with the alternative that lost named and the reason it lost: compensate-after-commit, a `fn compensate(&self, losing: &Event) -> Event` port method, and every-peer-adjudicates."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "cargo xtask spec-trace; reviewer check of references/adr/0027-*.md against spec/SPECIFICATION.md:5871-5900, :5901-5930, :6033-6052"
- id: AC-004
  criterion: "The topology answer does not exclude the edge. GIVEN a constrained-runtime developer whose Worker must be a hub to nine tablets and a symmetric peer to a shore depot at the same time, WHEN they read ADR-0027, THEN it answers SY-10 — whether the runner expresses the two topologies as one configuration or two, given a different merge rule in each direction of one edge — names the alternative that lost, stays inside frozen SY-9 (hub-ness is a property of an edge), and hands the exercise to `hub-and-spoke-and-peer-to-peer-topologies` rather than asserting it."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "cargo xtask spec-trace; reviewer check against spec/SPECIFICATION.md:6090-6106 and :6110-6133; git diff --exit-code -- spec/SPECIFICATION.md"
- id: AC-005
  criterion: "The next implementer knows exactly what they may add. GIVEN HS-S0106's implementer opening the atom to learn which derives are permitted on the message set, WHEN they read it, THEN `PushBatch`, `EventGroup` and `ReplicatedEvent` are each named individually with an exact derive list, the record states that the message set instantiates `Envelope<T>` and never becomes an enum around it, and it states that `Envelope`'s `Deserialize` stays hand-written. A formulation such as \"the message types become serialisable\" fails this criterion even though it validates and merges."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "grep of .kb/decisions/0027-*.md and references/adr/0027-*.md for all three type names with adjacent derive lists, cross-checked against crates/happenstance-sync/src/peer.rs:191-196, :236-243, crates/happenstance-sync/src/identity.rs:171-180 and crates/happenstance-sync/src/lib.rs:78-84"
- id: AC-006
  criterion: "`FORMAT_VERSION` stops naming a vocabulary nobody chose. GIVEN a peer author writing the first message their transport will ever send, WHEN they ask whether the version travels on every message or is negotiated once per connection, THEN ADR-0027 answers it, states the cost of the side that lost (dead weight on every message versus a format break to remove the field later), says what a bump then measures, and keeps WF-9's line intact — the version bumps on a shape change and never on a capacity bound. All three ordered sub-questions of the open-question atom are answered, so HS-S0100 has a resolution to record rather than a further question."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb; reviewer check against .kb/open-questions/sync-message-set-and-format-version.md (Ordered sub-questions) and crates/happenstance-sync/src/wire.rs:46-65"
- id: AC-007
  criterion: "Nothing in this range is deferred by silence. GIVEN an evaluator deciding in one sitting whether replication is finished enough to adopt, WHEN they read the `[DEFERRED]` clauses in ADR-0027's range, THEN SY-27 and SY-28 are renewed against the whole-log-versus-scoped experiment at `references/evaluation/PRESSURE-TEST.md:687-693` by name, with the port-side reason they cannot be settled on paper recorded; and the SY-14 boundary disagreement — `RUNBOOK.md:4567-4572` places it in ADR-0027's work item, `_decomposition.md` Tension 3 places SY-8 – SY-18 in ADR-0026's range — is settled by name and written into both records so no clause is assigned twice or to nothing."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "cargo xtask spec-trace (CF-38, xtask/src/spec_trace.rs); both ADR records state the same SY-14 assignment; references/evaluation/PRESSURE-TEST.md:687-693 resolves"
- id: AC-008
  criterion: "The split adds up, and it can be added up by someone else. GIVEN HS-S0113 at project exit, whose job is to add ADR-0026's range to ADR-0027's and compare the total against this project's stated range, WHEN they read this atom, THEN the range is written as a range — SY-1 – SY-7, SY-19 – SY-31, SY-35 — with the arithmetic shown (21 + 13 + 1 = 35), it is exactly the complement of ADR-0026's, and every one of the nine `[PROVISIONAL]` clauses inside it is judged out loud: moved by this decision with its falsifier addressed, or left standing with its falsifier restated as still open."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "cargo xtask spec-trace; recomputation showing no SY- id in both ranges and none in neither once SY-32's handoff is counted (spec/SPECIFICATION.md:7000-7003); SY-7, 10, 20, 21, 22, 23, 29, 30, 31 each named with a disposition"
- id: AC-009
  criterion: "A reader who does not know the filename still finds it, and nothing else moved. GIVEN an evaluator who arrives at `.kb/maps/decision-map.md` because it is the only place the whole decision corpus is visible at once, WHEN they scan the ADR table, THEN ADR-0027 has a row carrying atom id, title, status, phase and supersession, with reciprocal `related` links that resolve in both directions; and the rest of the tree is untouched — no Rust, no clause text, no accepted atom body."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md"
  verifying_test: "redkiln validate --kb && redkiln doctor (exactly six template-drift advisories); git diff --exit-code -- spec/SPECIFICATION.md crates/ xtask/ examples/ Cargo.toml; cargo xtask ci --fast"
```
