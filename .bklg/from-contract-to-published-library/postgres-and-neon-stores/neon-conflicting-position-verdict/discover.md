---
item: HS-S0070
stage: discover
created: 2026-08-12T13:02:38.774Z
updated: 2026-08-12T13:02:38.774Z
template_sig: 86ce4036
rendered_sig: c62414db
---

# Discover — conflicting_position settled by evidence, and the ledger corrected

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: the in-tree CTE's claim that the collapse *keeps* `conflicting_position` is confirmed or refuted against a real endpoint, `ProbeThenWriteStore` is shown to fail a rule the real store passes, and the decision ledger's standing assumption is corrected either way | `_storymap.md`, *Slices* table, `neon-conflicting-position-verdict` row | Verification of an in-tree answer plus a ledger correction — "not the discovery of a new limitation" |
| **AC-008** — either the CTE is shown on a live endpoint to return the conflicting position, or a decision record downgrades the guarantee with the evidence that forced it; the ledger's standing assumption is corrected either way | `project.md`, *Acceptance criteria*, AC-008 | Sole owner. Both halves are required: a green run with no ledger correction leaves the known-wrong assumption on record |
| `depends_on: neon-append-and-read-over-http` (HS-S0069) | manifest; `_storymap.md`, *Merge order* item 4 | Supplies the decoder that can report a conflicting position, the fixture, and the credentialed job. There is nothing to verify until the CTE actually runs |
| The CTE, written out in full in two places, computing probe and insert on one snapshot and projecting both | `crates/happenstance-neon/src/lib.rs:54-68`; `crates/happenstance-neon/src/event_store.rs:134-148` | "One statement, one round trip, one snapshot, and a row that says both which position was assigned and — when nothing was — which position conflicted" |
| The crate's own verdict, stated as a contradiction of the ledger | `crates/happenstance-neon/src/lib.rs:72-77` | "So Neon, the case the ledger names as forcing `conflicting_position` down to a hint, does not in fact force it." Costs: one extra aggregate index scan on every append, and `IsolationLevel::Serializable` |
| **The port-level question is already settled, and settled as a hint.** ES-25: `conflicting_position` "is informational"; an adapter that detects the conflict without learning which event caused it "MUST be permitted to report `None`"; callers "MUST NOT depend on it" | `spec/SPECIFICATION.md:3746-3748`; the clause row at `:8607` marks ES-25 **FROZEN** | This reframes the whole story. The open item is not the guarantee's strength — it is the *premise* the ledger used to justify it |
| ADR-0012 settled it: "conflicting_position is a hint that may be absent and must not be parsed out of a message", and rejected demoting it to nothing "because a compiled Postgres strategy keeps the field usefully" | `.kb/decisions/0012-append-shape-and-preconditions.md:26`, `:99-101` | The **conclusion is correct and stands**. Nothing this story finds changes it |
| The ledger's standing assumption, in the runbook's own words: "open — Neon-over-HTTP is the forcing case, because it has no interactive transaction and so cannot probe and write separately" | `RUNBOOK.md:479` | This is the sentence the evidence contradicts, and the primary target of the correction |
| And its restatement in the phase-4 exit record: `happenstance-neon` "is cited for why the question was asked: it has no interactive transaction, so **the only shape it can express yields a boolean and no row**" | `RUNBOOK.md:3113-3116` | The second target. The CTE is a shape it can express that yields a row |
| `ProbeThenWriteStore` is already written, in full, as the named wrong implementation — and it "stays. Do not delete it, do not fix it, and do not leave it merely documented" | `crates/happenstance-neon/src/event_store.rs:388-503`; `_decomposition.md`, *Architecture brief* §5 | Consumed here, never rebuilt. Its four supporting `todo!()`s — `probe_request` (`:122`), `insert_request` (`:127`), `decode_probe_response` (`:497`), `decode_last_position` (`:502`) — belong to this story |
| Its defect is a **lost update**, not a slow path: "the failure is silent, produces no error, and is indistinguishable after the fact from a legitimate append" | `crates/happenstance-neon/src/event_store.rs:400-415` | The window is a full network round trip, "tens of milliseconds, not microseconds" — so it is reproducible against a live endpoint in a way it never is in-process |
| `catch_unwind` over a non-capturing probe is the in-tree pattern for asserting that a rule rejects an implementation | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:329-332`; `_decomposition.md`, *Testing brief* → *Notes* §4 | Driven through the public rule functions (`crates/happenstance-testkit/src/lib.rs:189`) from `crates/happenstance-neon/tests/`, the same seam HS-S0064 establishes |
| The isolation tension: the CTE "needs `IsolationLevel::Serializable` to be sound", while the header "applies only when `SqlRequest::statements` holds more than one statement — a single statement is its own implicit transaction and the header is ignored" | `crates/happenstance-neon/src/lib.rs:74-77`; `crates/happenstance-neon/src/transport.rs:56-60` | Surfaced by HS-S0067, and the most plausible way the live endpoint refutes the CTE. The verdict is this story's |
| An accepted decision atom is **immutable**; validation checks each against `HEAD`, so a correction is a new superseding atom, never an edited body | `CLAUDE.md`, *Where the work lives* | Bounds how the ledger may be corrected. See the second mutant |
| If the contract has to give, that is a second decision record with the suite re-run — DoD 6 explicitly contemplates it | `project.md`, DR-5 and DR-7; risk table row 5; `RUNBOOK.md:4361-4363` | Applies to a **rule Neon cannot pass**. It does not apply to this question, because ES-25 already permits `None` |

## Questions

**Answered.**

1. *Is `conflicting_position` a promise every adapter owes or a hint one may omit?*
   **A hint.** ES-25 says so and is `[FROZEN]`
   (`spec/SPECIFICATION.md:3746-3748`, `:8607`); ADR-0012 settled it and explicitly
   rejected demoting it further
   (`.kb/decisions/0012-append-shape-and-preconditions.md:26`, `:99-101`). This story
   does **not** reopen that. Recorded plainly because the story's title invites the
   opposite reading.
2. *Then what is actually open?* The ledger's **premise**. `RUNBOOK.md:479` and
   `:3113-3116` justify the hint by asserting that Neon-over-HTTP "cannot probe and
   write separately" and that "the only shape it can express yields a boolean and no
   row". The crate's CTE is a counterexample to the second, and this story tests it
   against a real endpoint.
3. *Does a refutation require amending a `[FROZEN]` clause?* No. If the CTE turns out
   unsound and Neon reports `None`, ES-25 already permits exactly that — the clause is
   untouched and the ledger correction runs the other way. The DoD-6 amendment path
   exists for a **rule Neon cannot pass**, and if the CTE's failure mode is unsound
   *append semantics* rather than a missing field, the clauses at risk are the
   condition-semantics ones and DR-7 applies: a new accepted decision atom, written
   first, with the suite re-run.
4. *May the correction edit ADR-0012?* No. Accepted atoms are immutable and
   `redkiln validate --kb` checks them against `HEAD`. And ADR-0012's *conclusion* is
   correct either way — see the second mutant for why superseding it would be worse
   than editing it is impossible.

**Deferred to `spec`.**

5. *The vehicle for the ledger correction* — a new KB atom, an evidence section in
   an existing record under authorship, or the RUNBOOK rows alone. The constraint is
   that `RUNBOOK.md:479` and `:3113-3116` must not still read as they do now once
   the evidence is in.
6. *How contention is manufactured against a live endpoint* — the count of
   concurrent clients, and how the interleaving is made reliable enough to be
   evidence rather than anecdote.
7. *Whether the `Serializable` header reaches a one-statement request*, per
   signal 13. It is the specific mechanism by which the CTE might be refuted.

**Deferred to the owning stories.**

8. *How the adapter buys ES-10's visibility invariant.*
   `adr-0024-position-visibility-mechanism` (HS-S0065). Position visibility and
   condition reporting are separate properties, and this story does not touch the
   first.

## Decision

The problem this slice solves is a contradiction the repository is carrying against
itself. The decision ledger says Neon-over-HTTP is the case that forces
`conflicting_position` down to a hint, because a store with no interactive
transaction can only express a shape that yields a boolean and no row. The Neon
crate's own documentation says that is false, quotes the CTE that disproves it, and
says so in terms — "contrary to the standing assumption in the decision ledger, the
collapse **keeps** `ConditionViolated::conflicting_position`". One of those is wrong,
and neither has met a real endpoint. This story runs the CTE against one, under
contention, and pairs that with the negative control the whole claim rests on:
`ProbeThenWriteStore` driven through the same public rule functions and required to
**fail** at least one rule the real store passes, because a single-client test cannot
tell a one-snapshot CTE from two lucky statements. Whichever way the evidence lands,
the ledger is corrected. The spec will cover: the four `todo!()` bodies
`ProbeThenWriteStore` needs; the contention harness and what counts as evidence; the
`catch_unwind` assertion naming the rule that must fail; the isolation-header
observation from HS-S0067 and what it implies; and the ledger correction's vehicle
and its exact targets at `RUNBOOK.md:479` and `RUNBOOK.md:3113-3116`. **No `[FROZEN]`
clause is changed:** ES-25 already permits `None`, so both outcomes are inside the
contract as written. If the live endpoint instead shows the CTE's *append semantics*
unsound, that is a rule Neon cannot pass, and DR-7 applies — a new accepted decision
atom, written first, with the suite re-run.

## The wrong implementation

**A confirmation taken with one client.** Append event A. Then append B with an
`after` that A already invalidated. Observe that the endpoint returns a conflicting
position. Record AC-008 as confirmed. Everything passes — the run is green, the
crate doc is vindicated, the ledger correction gets written, and the verdict is
worthless, because **`ProbeThenWriteStore` returns the conflicting position in
exactly that scenario too**: its probe runs first, finds the conflict, and reports it
(`crates/happenstance-neon/src/event_store.rs:428-503`). The property under test is
not "does a conflicting position come back" — it is "were the probe and the insert
evaluated on **one snapshot**", and a test with no second writer cannot observe the
difference. This is the same failure shape as HS-S0063's: an instrument that a wrong
implementation passes is not an instrument. The rejection is the negative control
AC-008 names, run under real contention, in `crates/happenstance-neon/tests/` through
`happenstance_testkit::rules::<name>` with `catch_unwind` over a non-capturing probe
(`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:329-332`). If no
rule in the set fails `ProbeThenWriteStore` under contention, that is a finding about
the **rule set**, not a licence to proceed: the rule is fixed and the reason given in
the same change.

**The ledger mutant: superseding ADR-0012.** The obvious way to correct a decision
record whose premise was wrong is to write a superseding atom, and that is what the
KB's own machinery is for. Here it is wrong, and it passes every check —
`redkiln validate --kb` is green, `redkiln doctor` is clean, the supersession graph
is well formed. ADR-0012's **conclusion** is correct and unchanged: the field is a
hint, `Option` spares the adapters that cannot afford it, and demoting it to nothing
was rightly rejected "because a compiled Postgres strategy keeps the field usefully"
(`.kb/decisions/0012-append-shape-and-preconditions.md:99-101`). Marking a correct,
`[FROZEN]`-backed decision as superseded because one supporting sentence was
factually wrong leaves the phase-12 audit reading a supersession chain and
re-deriving whether the outcome moved — which is precisely the work the ledger exists
to spare it. Editing the body instead is not available: accepted atoms are immutable
and validation checks them against `HEAD`. The correction belongs where the wrong
sentences are, which is the runbook.

**And the quietest one: correcting the crate doc instead of the ledger.** The Neon
crate already states the right answer. Add the live-endpoint evidence to it, ship,
and leave `RUNBOOK.md:479` reading "open — Neon-over-HTTP is the forcing case". Every
check passes; nothing in the gate reads that table. AC-008's second half — "the
decision ledger's standing assumption is corrected either way" — is the only
instrument, and `spec` must make the two line targets explicit rather than leave
"the ledger" as an unbounded noun.

**On literal positions.** No conformance rule is added, and the conflicting position
this story asserts on is the one the store assigned to the first append — never a
literal. `cargo xtask lint-position-literals` does not read
`crates/happenstance-neon/tests/` (`xtask/src/lints.rs:631`;
`xtask/src/spec_trace.rs:85-89`), so `spec` states it.

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
