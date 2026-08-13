---
item: HS-S0066
stage: discover
created: 2026-08-12T13:02:34.559Z
updated: 2026-08-12T13:02:34.559Z
template_sig: 86ce4036
rendered_sig: d5bf1cad
---

# Discover — The structural bill written where a consumer meets it

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: a consumer reading the Postgres crate's public docs meets the frontier `head`, the absence of read-your-own-writes and the cluster-wide staleness bound, and the choice between doc prose and a clause of its own is **recorded as a decision rather than defaulted** | `_storymap.md`, *Slices* table, `postgres-structural-bill` row | Two deliverables, and the second is a decision about the specification, not about the crate |
| **AC-009** — the consequences of the chosen mechanism are documented, and the choice between adapter documentation and a specification clause of its own is recorded as a decision rather than defaulted | `project.md`, *Acceptance criteria*, AC-009 | Sole owner. "Recorded rather than defaulted" is the whole of the second half |
| `depends_on: adr-0024-position-visibility-mechanism` (HS-S0065) | manifest; `_storymap.md`, *Merge order* item 3 | Supplies the chosen mechanism and its measured cost. There is no bill to write until there is a mechanism to bill for, and the staleness figure the docs promise is sub-question 3 of `.kb/open-questions/postgres-arm-c-structural-cost.md` |
| The three consequences, already stated normatively under ES-10: a conformant adapter buying this invariant with `xid8` + `pg_snapshot_xmin` reports a **frontier** from `head()` rather than `max(position)`, therefore does **not** satisfy read-your-own-writes, and staleness is bounded by the longest open write transaction *anywhere in the cluster* | `spec/SPECIFICATION.md:2833-2842` | The specification already says it. AC-009 asks whether that is *sufficient* — and it is deliberately silent on the answer |
| "That is a documented capability limit, **not a tuning parameter**, and it is why ES-30's `head_is_the_highest_visible_position` asserts a bound rather than an equality" | `spec/SPECIFICATION.md:2840-2842` | The bound is the promise; the measured milliseconds are illustration. A doc that promises a number promises the wrong thing |
| Measured staleness: 0.688 ms with no holder, 4010.719 ms behind an unrelated five-second write in an unrelated database | `spec/SPECIFICATION.md:2836-2840`; `experiments/position-visibility/results/staleness_pinned.txt` | Four orders of magnitude. `.kb/open-questions/postgres-arm-c-structural-cost.md` sub-question 3 asks which end a caller should plan for, and answering it is this story's |
| ADR-0013's binding caveats state the same three consequences as decision consequences | `.kb/decisions/0013-position-assignment-and-visibility.md` | The atom, the clause and the crate doc must not drift. Three copies of one requirement is what ES-10's own closing note warns against |
| The crate's `lib.rs` today carries the ES-10 question as an **open decision** in its crate-level doc, alongside tag matching and TLS | `crates/happenstance-postgres/src/lib.rs:30-36` | Prose written for a skeleton, addressed to an implementer. It must become prose addressed to a caller, and the two are not the same text in the same place |
| `head`'s own body doc already spells out the frontier and what each mechanism spells it as | `crates/happenstance-postgres/src/event_store.rs:146-161` | The right content is in the wrong register: it is a note to whoever writes the body, not a promise to whoever calls it |
| The gate's `documentation` step sets `RUSTDOCFLAGS` rather than relying on ambient `RUSTFLAGS`, because "rustdoc does not read `RUSTFLAGS`" and the step printed warnings and exited 0 for as long as it ran | `xtask/src/main.rs:284-290` | The step denies rustdoc *lints*. It has no opinion about whether a doc says the true thing, which is exactly the gap this story sits in |
| The repository's rustdoc obligations are an atom of the constitution | `standards/rust/70-rustdoc-obligations.md` | Where the bar for "a caller meets it" is set. Read it, do not restate it |
| Changing a `[FROZEN]` clause requires a new ADR, not an edit | `CLAUDE.md`, *Open questions, deliberately unresolved* | ES-10 is `[FROZEN]` (`spec/SPECIFICATION.md:8592`). If this story's verdict is "a clause of its own is owed", the clause arrives by accepted decision record |
| `cargo xtask spec-trace` regenerates §7.1–§7.2 and checks citations and markers | `_decomposition.md`, *Architecture brief* §7 | It verifies that clauses cite things that exist. It cannot detect a clause that *should* exist and does not |

## Questions

**Answered.**

1. *Is the specification's existing ES-10 prose enough?* Not decided here — that is
   the literal content of this story's second deliverable, and AC-009 forbids
   defaulting it. What *is* settled at `discover` is that both branches have a
   named consequence: "sufficient" means the crate's rustdoc carries the
   consumer-facing half and the specification is untouched; "a clause is owed"
   means a decision record, then the clause.
2. *What does the documentation promise about staleness?* The **bound** — the
   longest open write transaction anywhere in the cluster — with the measured range
   as illustration and explicitly not as a service level.
   `spec/SPECIFICATION.md:2840-2842` calls it a capability limit rather than a
   tuning parameter, and the two readings differ in whether a consumer is entitled
   to plan around 0.688 ms.
3. *Where does the consumer-facing half live?* On the items a caller meets —
   `PostgresEventStore::head`, and `append`'s return — not only in the crate-level
   doc, which today addresses an implementer deciding a mechanism
   (`crates/happenstance-postgres/src/lib.rs:30-36`).

**Deferred to `spec`.**

4. *The exact register and placement* of each of the three statements across
   `lib.rs`, `event_store.rs` and the two method docs, against
   `standards/rust/70-rustdoc-obligations.md`.
5. *Whether `contains_event_id`'s frontier disagreement is part of the bill.* It is
   a consequence of the same mechanism (`crates/happenstance-postgres/src/event_store.rs:163-174`)
   but its resolution is a replication question owned by HS-P0017; the bill can
   state the disagreement without settling it.

**Deferred to the owning stories.**

6. *How the adapter buys ES-10's visibility invariant, decided on numbers.*
   `adr-0024-position-visibility-mechanism` (HS-S0065) — this story's dependency,
   and the reason it is sequenced second in the slice.
7. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). Not part of the structural
   bill: it is a property of the append condition's reporting, not of position
   visibility, and ES-25 already answers it at the port level
   (`spec/SPECIFICATION.md:3746-3748`).

## Decision

The problem this slice solves is that the chosen mechanism bills the *caller*, and
the caller currently has no way to know. `head` returns a frontier rather than a
maximum, `append` returning `Ok(P)` does not promise the next `head()` is at or
above `P`, and how far behind the frontier runs is bounded by the longest open
write transaction anywhere in the cluster — including one held by an unrelated
application in an unrelated database. All three are stated today in the
specification and in ADR-0013, which are documents an adapter *author* reads; none
is stated where somebody calling `store.head().await` meets it. This story writes
the consumer-facing half into the crate's public rustdoc, and then answers the
question AC-009 refuses to let anyone default: whether the specification's existing
ES-10 prose is sufficient or a clause of its own is owed. The spec will cover: the
three statements, their register and the items they attach to; the staleness
*bound* as the promise with the measured range as illustration; the treatment of
`contains_event_id`'s frontier disagreement as stated-but-unsettled; and the
verdict on the clause question. **If that verdict is "a clause is owed", ES-10 is
`[FROZEN]` and the clause arrives by a new accepted decision record — written
first, then the specification edit, then `cargo xtask spec-trace --write` — and
never by an edit to the clause.** That record is either ADR-0024's own consequences
section, if this story runs close enough behind HS-S0065 to be folded into it, or a
successor atom of its own; `spec` names which.

## The wrong implementation

**A structural bill that is entirely true and lives where nobody reads it.** Add a
`# Capability limits` section to `crates/happenstance-postgres/src/lib.rs`'s
crate-level doc, beside the existing "open decisions" prose, stating the frontier,
the absence of read-your-own-writes and the cluster staleness bound. It is
accurate, it is well written, and every check passes: `cargo doc --workspace
--all-features --no-deps` is green, the gate's `documentation` step denies rustdoc
lints and has no opinion about placement (`xtask/src/main.rs:284-290`),
`cargo xtask spec-trace` is green because no clause moved, and a reviewer skimming
the diff sees three paragraphs of exactly the right content. The consumer who
autocompletes `store.head().await` in an editor is shown `head`'s own doc and
learns nothing. AC-009's wording — "written **where a consumer meets it**" — is the
only instrument that rejects this, and it is prose in a charter, not a check in the
gate.

**Its sibling: a bill that promises a number.** "`head` may lag by up to ~0.7 ms."
It is measured, it is citable, and it converts a documented capability limit into
an implicit service level that the very next paragraph of the specification denies
(`spec/SPECIFICATION.md:2840-2842`) — the same experiment recorded 4010.719 ms
behind one unrelated five-second write. A consumer who builds a read-after-write
flow on a 0.7 ms budget has been told something false by a true sentence.

**And the one AC-009 was written against: defaulting the clause question.** Write
the rustdoc, ship the story, and let "the specification already covers it at ES-10"
be the outcome nobody chose. Nothing detects it. `cargo xtask spec-trace` checks
that clauses cite things which exist and that markers have not rotted; it cannot
notice a clause that ought to exist and does not, and no `redkiln` gate reads the
specification at all. The absence of a decision is indistinguishable from a
decision to leave it — which is why the deliverable here is a *recorded verdict*,
in either direction, and why `spec` must make that verdict an acceptance criterion
of its own rather than a sentence in a rationale.

This story adds no conformance rule and defines no store, so nothing belongs in
`crates/happenstance-testkit/tests/`; the mutants above are documentation and
process shaped, and the instrument against all three is the review at `spec` plus
the explicit verdict artefact.

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
