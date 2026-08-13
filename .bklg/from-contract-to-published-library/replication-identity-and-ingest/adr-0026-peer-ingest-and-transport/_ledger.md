---
item: HS-S0098
stage: implement
created: 2026-08-12T13:47:36.518Z
updated: 2026-08-12T13:47:36.518Z
---

# Acceptance ledger — ADR-0026: what a peer is, what the port may assume, what ingest promises

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

This story ships no `.rs` file, so `verifying_test` names the deterministic command and the real path
it inspects rather than a Rust test id — the testing brief's Static tier
(`.bklg/from-contract-to-published-library/replication-identity-and-ingest/_decomposition.md:625-663`).
The single Rust command below proves a *citation* is still live, not that this story wrote code.

```yaml
- id: AC-001
  criterion: "GIVEN the intake brief's clause ledger disagrees with `spec/SPECIFICATION.md` — wrong ranges, and `VT-*` where the file says `WF-*` — WHEN a maintainer opens ADR-0026 to find out which SY and WF clauses phase 13 actually discharges, THEN they find a ledger **re-derived from `spec/SPECIFICATION.md` at HEAD**: every SY and WF clause listed with its maturity marker and its line, the per-marker counts stated, and every disagreement with the brief resolved in the specification's favour and written down as a recorded difference rather than silently absorbed — and the re-derivation is stated as having preceded the drafting, not reconciled after it."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md, whose source_paths resolve to references/adr/0026-*.md (clause-ledger section)"
  verifying_test: "rg -n '^\\*\\*(SY|WF)-[0-9]+' spec/SPECIFICATION.md and rg -n '\\[FROZEN\\]|\\[PROVISIONAL\\]|\\[DEFERRED\\]' spec/SPECIFICATION.md reproduce the record's table clause-for-clause; cargo xtask spec-trace green (xtask/src/spec_trace.rs)"
- id: AC-002
  criterion: "GIVEN AC-014's arithmetic is computed six slices later by `clause-arithmetic-and-deferral-renewals`, WHEN that story computes the union of the two ADRs' stated ranges against the project's stated range, THEN ADR-0026 has already written its own range down in a form that can be computed against rather than argued with — **SY-8 – SY-18, SY-33, SY-34 plus WF-1's interoperability half**, with ADR-0027's complement named (SY-1 – SY-7, SY-19 – SY-31, SY-35) and SY-32 subtracted as a named handoff — and clauses it merely cites (SY-1, SY-2, SY-6) are marked as cited, not claimed, so the union counts each clause exactly once."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (stated clause range), long form at references/adr/0026-*.md"
  verifying_test: "rg -n 'SY-33|SY-34|SY-32' references/adr/0026-*.md shows all three dispositions; the union stated-range ∪ named-complement ∪ {SY-32 handoff} = SY-1 – SY-35 with no clause twice, cross-checked against RUNBOOK.md:305-306 and :4535"
- id: AC-003
  criterion: "GIVEN a maintainer asks the question the whole phase is named for — does ingest re-check the writer's asserted append conditions — WHEN they read ADR-0026, THEN they get the answer (**it does not**) **reconciled against the frozen clauses rather than re-deliberated**: SY-1 and SY-6 cited by line as the binding statements, the three apparent counter-examples the specification already dismisses accounted for, and the alternative that lost — receiver re-evaluates `origin_condition` — refuted with **both** of SY-6's independent defects reproduced, that a position-relative `after` checks an arbitrary tail and passes vacuously, and that even `after: None` inverts on re-delivery."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (the central question), long form at references/adr/0026-*.md"
  verifying_test: "the record cites spec/SPECIFICATION.md:5841-5867, :5973-6029 and :5798-5836; the re-check|recheck|re-evaluat sweep over spec/SPECIFICATION.md is recorded with its result; git diff --name-only main -- spec/SPECIFICATION.md is empty"
- id: AC-004
  criterion: "GIVEN a maintainer wiring a second peer wants to know whether that is a configuration change or a breaking one, WHEN they read the record's answer to *what a peer is*, THEN it states that one `SyncPeer` is exactly one peer relationship, that fan-out, ordering and merge policy live in a runner above the port, and that hub-ness is a property of the **edge** and never of the port's type or constructor (SY-9, `[FROZEN]`) — so adding a peer stays runner configuration — with SY-8's refusal to grow `EventStore` cited as the same decision seen from the other side."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (what a peer is)"
  verifying_test: "record cites crates/happenstance-sync/src/peer.rs:59-80, spec/SPECIFICATION.md:6070-6086 and :6090-6106; git diff --stat main -- crates/ is empty"
- id: AC-005
  criterion: "GIVEN a maintainer asks why `pull` hands back a bounded batch and an owned resume token instead of a stream, WHEN they read the record's answer to *what the port may assume about a transport it cannot see*, THEN they find the reason stated as evidence rather than taste — the cursor shape was attempted, compiled against both peer shapes, and admitted the one-shot-HTTP peer only by buffering a whole response into a `Vec` and replaying it — **the type checker did not force the choice and the record says so** — the still-compiling probe is cited by path, and the property the port cannot express (one round trip holding no state) is handed to `sync-testkit-crate-and-rule-registry` as a fixture-owned round-trip-counter obligation rather than pushed back into the port."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (transport assumptions), long form at references/adr/0026-*.md"
  verifying_test: "cargo test -p happenstance-sync --test cursor_shape_probe (crates/happenstance-sync/tests/cursor_shape_probe.rs) proves the cited probe is live; record cites crates/happenstance-sync/src/peer.rs:30-50, src/lib.rs:125-132 and tests/real_peer_shapes.rs, and names the recipient story of the round-trip obligation"
- id: AC-006
  criterion: "GIVEN a maintainer needs to know what arriving at a receiver guarantees, WHEN they read *what ingest promises*, THEN they find four promises stated and each tied to a clause — atomicity per group, the foreign `EventId` preserved, idempotence on re-delivery, and a local position assigned at the arrival-order tail — together with the reason `IngestStore` is a separate trait at all: an append is a decision taken now against a condition checked now by the store that assigns identity, while an ingest records a decision somebody else already took and already made durable, so a `SequencePosition` is meaningful only inside one store."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (what ingest promises)"
  verifying_test: "record cites crates/happenstance-sync/src/ingest.rs:87-93 and spec/SPECIFICATION.md:5954, :6135, :6409; rg -n 'position' references/adr/0026-*.md shows the position-is-not-an-identity finding from crates/happenstance-sync/src/lib.rs:98-133 carried across"
- id: AC-007
  criterion: "GIVEN `memory-store-ingest-seam` cannot land a change to a published crate that nobody decided, WHEN its implementer opens ADR-0026 for the authorisation it is told to cite, THEN the atom authorises exactly one additive **inherent** `&self` operation on `MemoryEventStore` accepting an already-identified `SequencedEvent`, **states its semver class** (minor, on a concrete type in a published crate; no trait change; `EventStore`'s signature byte-identical before and after), and refuses growing `EventStore` by name — with the sketch's own finding carried across, that the trait seam is discharged and the write path is not, so a foreign identity today has a place to sit and no door to come in through."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (write-path seam authorisation), quoted by memory-store-ingest-seam's ledger"
  verifying_test: "record cites crates/happenstance-sync/src/ingest.rs:50-71, RUNBOOK.md:450-455 and spec/SPECIFICATION.md:6070-6086; git diff --name-only main -- crates/happenstance-core is empty in this PR"
- id: AC-008
  criterion: "GIVEN `RUNBOOK.md:4593-4595` forbids silence on DCB wire interoperability, WHEN a maintainer reads ADR-0026's **envelope section**, THEN WF-1's interoperability half is there by name — deferred, with the experiment stated as a *specific external implementation to interoperate with*, and with the ground recorded that the DCB specification and its reference TypeScript library publish no wire format at all, which makes the deferral stronger rather than weaker — so `open-questions-resolved-and-indexed` can resolve `dcb-reference-publishes-no-wire-format` by citation instead of re-arguing it."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (envelope section), cited by .kb/open-questions/dcb-reference-publishes-no-wire-format.md"
  verifying_test: "rg -n 'WF-1' references/adr/0026-*.md non-empty and names the external implementation; record cites spec/SPECIFICATION.md:1892-1922; cargo xtask spec-trace green over WF-1's deferral (CF-38, spec/SPECIFICATION.md:213-217)"
- id: AC-009
  criterion: "GIVEN a `[DEFERRED]` clause with no named experiment is a build failure under CF-38, WHEN `clause-arithmetic-and-deferral-renewals` sweeps at project exit, THEN ADR-0026 has already supplied its own range's half: **SY-14** renewed against the whole-log-versus-scoped experiment and **SY-18** against the Turnstile KV-backed peer shape, each experiment named as an observation someone could actually make, and the exit-wide sweep left to the story that owns it."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (in-range deferral renewals)"
  verifying_test: "cargo xtask spec-trace green with CF-38 satisfied for SY-14 (spec/SPECIFICATION.md:6234) and SY-18 (:6362); record cites references/evaluation/PRESSURE-TEST.md:685-693"
- id: AC-010
  criterion: "GIVEN HS-P0018 is unblocked only when SY-32's disposition is on disk, WHEN a maintainer asks whether replication settles retention or waits on it, THEN ADR-0026 records the ES-39 dependency **by name**, states plainly that it cannot discharge it, names ADR-0028 and `retention-and-incomplete-logs` as the owner, notes that `PeerLimits::retention_floor` already exists on the port while the store-side primitive does not, and confirms — rather than flips — the `replication → retention` edge at ranks 4 → 5 of the initiative DAG."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (SY-32 handoff)"
  verifying_test: "record cites spec/SPECIFICATION.md:7000-7003 and RUNBOOK.md:307; the confirmation checks out against .bklg/from-contract-to-published-library/_decomposition.md's Dependency DAG with no edit to that file"
- id: AC-011
  criterion: "GIVEN project DoD 7 requires that any change to `spec/SPECIFICATION.md` be one a decision record authorised, WHEN `frozen-clause-repairs` runs in slice 7, THEN it finds both live repairs already authorised in this atom in the playbook's three-part form — the MUST kept verbatim, the discharge named as a discharge, the code and test that assert it cited — namely dropping `(new)` from SY-1's and SY-2's `Rule:` lines once those rules exist, and repairing any `Rejects:` whose named symbol a later story changes, chiefly `EventGroup::guard`; each carries the mechanical justification that the admitted implementation set is unchanged, and **this story performs neither repair**."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md → .kb/decisions/0026-*.md (authorised repairs), consumed by frozen-clause-repairs"
  verifying_test: "git diff --name-only main -- spec/SPECIFICATION.md '*.rs' is empty; cargo xtask spec-trace still reports SY-1/SY-2's rules as scheduled (xtask/src/spec_trace.rs:1626); each authorisation applies .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md's mechanical test"
- id: AC-012
  criterion: "GIVEN a maintainer opening `.kb/` to find what a `SequencePosition` means once it has crossed a store boundary, WHEN they navigate from `.kb/maps/decision-map.md`, THEN they reach `kb-decision-0026` — an atom **authored by the ingest wave, never by hand** (`.kb/_intake/` → `/redkiln:kb-ingest`), carrying valid `KbFrontmatter` with `adr_id: ADR-0026`, `status: accepted`, `phase: 13`, a prose `summary`, `depends_on` and `source_paths` resolving into `references/adr/0026-*.md` — whose title states **one** decision, with any half resting on code not yet written marked provisional and carrying the observation that would refute it; `.kb/_intake/` is back to `README.md` only and no accepted atom's body was edited."
  satisfied: false
  evidence: ""
  mount_point: ".kb/maps/decision-map.md (row for kb-decision-0026) plus .kb/maps/domain-map.md subject grouping"
  verifying_test: "redkiln validate --kb && redkiln doctor green; rg -n 'kb-decision-0026' .kb/maps/decision-map.md .kb/maps/domain-map.md non-empty; test -f references/adr/0026-*.md; ls .kb/_intake is README.md alone; git diff --name-only main -- .kb/decisions lists only the new atom; title checked against .kb/playbooks/one-decision-per-adr-title.md:34-55"
```
