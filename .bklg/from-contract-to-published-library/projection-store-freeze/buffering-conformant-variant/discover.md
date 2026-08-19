---
item: HS-S0013
stage: discover
created: 2026-08-12T13:01:20.556Z
updated: 2026-08-12T13:01:20.556Z
template_sig: 86ce4036
rendered_sig: 4dae53d0
---

# Discover — A second, structurally unlike batch shape passes the whole suite

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: the CF-5 conformant variant — a buffering, replay-at-commit store legally unlike apply-on-write `MemoryProjectionStore`, built in the testkit's own `tests/` per AC-A03 — passes the whole suite, so two structurally unlike batch shapes are green inside one `cargo xtask ci` run with both fixtures named | `../_storymap.md:65` | One store, one fixture, zero new rules. Its whole job is to be *legally unlike* the oracle |
| **AC-004** — two structurally unlike batch shapes pass the whole suite, each named in the proof artefact, and the second shape's provenance is **a decision recorded in the architecture brief rather than an assumption** | `../project.md:191-193` | Sole owner (`../_storymap.md:82`); `MemoryProjectionStore` is the first shape, landed at `memory-projection-store` |
| `dependsOn: commit-rollback-and-drop-rules, reset-rules, read-through-and-rebuild-rules` — all three rule slices, because "passes the whole suite" is only meaningful once the suite is whole | `../_storymap.md:62-65,112` | This story is where DoD 2 becomes observable (`../_storymap.md:112`) |
| Architecture brief **AC-A03**, settled: the second batch shape is built **inside `happenstance-testkit`'s own `tests/`** as §4.11's CF-5 conformant variant. "No edge is added from this project to `sqlite-durable-store`, and none is needed" | `../_decomposition.md:303-306,406-435` | The provenance AC-004 demands is this decision, with three grounds in order of authority. It is not re-decided here |
| The specification already assigns it there: the projection suite owes "CF-5's conformant variant — a store legally different from `MemoryProjectionStore` that passes everything — and the obvious one is the buffering adapter PS-4 permits, **which doubles as the far end of §6's batch-shape axis**" | `spec/SPECIFICATION.md:5686-5692` | The far end is named, and it is testkit-internal |
| The precedent is in the tree: `GappedPositionStore` and `PagedStreamStore` are conformant variants in `tests/mutation_coverage/variants.rs` whose entire job is to be legally unlike `MemoryEventStore` | `crates/happenstance-testkit/tests/mutation_coverage/variants.rs:10-17,116,379`; `../_decomposition.md:420-426` | "Building an instrument to keep a rule non-vacuous is this repository's established move" |
| The alternative inverts the DAG: `sqlite-durable-store` is rank 2 and lists this project in its `blocked_by`; borrowing its rusqlite skeleton would also add a `[target.'cfg(not(target_arch = "wasm32"))'.dev-dependencies]` entry and a new licence/advisory surface for `cargo deny` — "to buy a shape the buffering variant already provides" | `../_decomposition.md:427-435`; `../project.md:284-291` | The RUNBOOK's own phase-6 proof artefact assumed the rusqlite route (`RUNBOOK.md:3935-3940`); AC-A03 is the correction and this story executes it |
| PS-4 permits it: a `Batch` MUST NOT be required to be a live transaction; an adapter MAY back one with a live transaction, but the port's obligation is PS-1 | `spec/SPECIFICATION.md:4849-4853` | The buffering shape is conformant *by clause*, not by tolerance |
| `mutant_registry_is_exhaustive` requires **at least one conformant variant** to exist | `crates/happenstance-testkit/tests/mutation_coverage.rs:2754`; `../_grounding.md:115-118` | The variant is registry data, not a loose test fixture, and the meta-test is what keeps it registered |
| **What this decision does not buy**, stated so nobody claims it later: PS-2's bar is "two *adapters* at opposite ends of the batch-shape axis", and its **Rejects** clause names verbatim "the schedule that freezes this port against `MemoryProjectionStore` and an in-process rusqlite transaction". Two testkit instruments do not clear that bar either | `spec/SPECIFICATION.md:4760-4775`; `../_decomposition.md:436-456` | AC-004 is satisfied in-project; **PS-2 is not, and must not be reported as satisfied**. That is why AC-014 takes its second arm |
| Testing brief AC-004: **Integration** (each fixture drives `projection_store_conformance!` to completion with zero failures) + **E2E** (both runs inside the **same** `cargo xtask ci` invocation, "so the proof artefact is one gate run naming both fixtures, not two separate `cargo test` invocations a reviewer has to reconcile by hand") | `../_decomposition.md:774` | Same-run is an acceptance property, not a convenience |
| The reshape trigger: "the buffering variant passing every rule trivially. If the two shapes never disagree anywhere, either the rules are shape-blind in a way that hides the axis, or the axis is not where §4.2 says it is" — that is PS-3's evidence and belongs to AC-015 | `../_decomposition.md:716-719`; `../_storymap.md:129` | Whatever this story observes is `ps3-batch-shape-finding`'s input. Nothing is swallowed |
| `CLAUDE.md`'s spread rule: "a port is only as well-designed as the *spread* of what implements it… four adapters, one storage shape, and any port frozen against them is frozen against SQLite wearing four hats" | `CLAUDE.md`, *The rule that matters* | The standard this story's variant is judged against, and the one the wrong implementation below fails |

## Questions

Open questions to resolve before specifying.

1. **What makes the variant "structurally unlike", concretely?** Deferred to
   `spec`, and it is the story's central question rather than a detail. The axis
   is named — apply-on-write versus replay-at-commit
   (`../_decomposition.md:446-448`) — and the spec must state the property that
   makes the difference real: the variant's writes are held in a buffer and
   applied to the read model only during `commit`, so nothing outside the batch
   can observe them beforehand.
2. **Does the variant declare `READS_THROUGH_BATCH` true or false?** Deferred to
   `spec`, and it interacts with `read-through-and-rebuild-rules`' requirement
   that both arms of that constant have a fixture. A buffering store can
   legitimately answer either way — it can read its own buffer, or decline — and
   whichever it chooses, the other arm needs a home.
3. **Where does the variant live — `tests/mutation_coverage/variants.rs` or a
   projection sibling?** Deferred to `spec`, following whichever
   one-binary-or-two decision `projection-mutant-registry` recorded
   (`../_decomposition.md:677-683`).
4. **Is this story allowed to change a rule?** Answered: only under
   `CLAUDE.md`'s own instruction — if a rule turns out to be wrong, it is fixed
   with the reason given in the same change. A rule *weakened* to let the variant
   pass is the failure; see the wrong implementation.
5. **None beyond these.** The freeze verdict is not this project's
   (`../project.md:56-58`) and PS-2's bar is not claimed here.

## Decision

A port frozen against two implementations that share a storage shape is a port
frozen against one implementation wearing two hats, and `MemoryProjectionStore`
alone is exactly that. This slice builds the second shape at the other end of the
axis §4.2 names: a buffering, replay-at-commit store that holds its writes until
`commit` and applies them in one unit, legally permitted by PS-4, structurally
unlike the apply-on-write oracle, and living in the testkit's own `tests/` rather
than borrowed from a downstream project. It passes the whole suite in the same
`cargo xtask ci` run as the first shape, with both fixtures named. The spec will
fix: the variant's identifier and internal shape; the property that makes it
unlike the oracle, stated as something a reviewer can check rather than as an
adjective; its registration as CF-5's conformant variant so
`mutant_registry_is_exhaustive` holds it in place; its `READS_THROUGH_BATCH`
declaration and the consequences for the other arm; and the same-run requirement
for the proof artefact. It also fixes what is **not** claimed: PS-2's bar is
`[FROZEN]` and explicitly rejects a two-instrument monoculture, so nothing here
reports it met. No `[FROZEN]` clause is edited.

## The wrong implementation

**`MemoryProjectionStore` with a `Vec` in front of it.** A "buffering" variant
whose `probe_write` pushes onto a vector and whose `commit` drains the vector into
the same `HashMap` the oracle uses, under the same borrow, in the same process,
with the same visibility semantics. It passes the whole suite — of course it
does; it *is* the oracle — it gets registered as CF-5's conformant variant, it
gets named in the proof artefact beside `MemoryProjectionStore`, AC-004 reads as
satisfied and DoD 2 reads as met. And the port has been frozen against one
storage shape wearing two hats, which is the failure `CLAUDE.md`'s spread rule
names in those words and which PS-2's **Rejects** clause already convicts for the
rusqlite version of the same mistake (`spec/SPECIFICATION.md:4771-4775`). Nothing
mechanical catches it: a conformant variant is *supposed* to pass everything, so
"it passed" is not evidence either way, and `mutant_registry_is_exhaustive` only
requires that some conformant variant exists.

The guard has to be a stated, checkable property rather than a claim of
unlikeness. The one the specification and the architecture brief both point at is
**visibility of pending writes**: in the buffering shape, nothing outside the
batch can observe a write until `commit` replays it, and the batch is not a live
transaction at all (PS-4, `spec/SPECIFICATION.md:4849-4853`). A variant that
shares the oracle's map and applies on write does not have that property, and the
spec must require it be demonstrable — for instance by the variant answering
`READS_THROUGH_BATCH` from its own buffer rather than from committed state, which
is precisely the axis `batch_reads_reflect_pending_writes` sits on.

**The variant that passes because a rule was softened.** The likelier failure in
practice: the buffering shape fails one rule, the rule looks over-strict from the
buffering side, and the rule is relaxed so both shapes go green. Every check
passes — the rule still exists, still has a registered mutant, still runs on
three runtimes — and the suite has just been calibrated to the two
implementations it was supposed to test. `CLAUDE.md` permits fixing a rule that
is genuinely wrong, and requires the reason in the same change; what it forbids is
the silent version. The spec should require that any rule text changed in this
story carries its reason inline and is reported at the story boundary, because
this is the one story with a structural incentive to weaken the suite.

**And the finding that must not become a shrug:** the variant passing every rule
*trivially*, with the two shapes never disagreeing anywhere. That is not success
and it is not failure — it is evidence that either the rules are shape-blind or
the axis is not where §4.2 says it is (`../_decomposition.md:716-719`). It is
`ps3-batch-shape-finding`'s content, and this story's obligation is to record what
it saw rather than to report green.

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

The two judgement boxes. **Literal positions**: this story adds a conformant
variant and a fixture, not a rule — vacuously true. If a rule *is* touched here,
it is under the eighth box's discipline and the no-literal-positions requirement
recorded in `commit-rollback-and-drop-rules` continues to bind it. **`[FROZEN]`
clauses**: none is edited. PS-2 is frozen and is the clause this story is most
likely to be misread as discharging — it is not, and the record above says so
explicitly (`spec/SPECIFICATION.md:4760-4775`;
`../_decomposition.md:436-456`). The eighth box is ticked with intent rather than
by default: this is the one story with an incentive to soften a rule, so any rule
change here carries its reason in the same change and is reported at the story
boundary.
