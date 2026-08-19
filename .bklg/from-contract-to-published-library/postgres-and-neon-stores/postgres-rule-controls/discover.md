---
item: HS-S0064
stage: discover
created: 2026-08-12T13:02:32.221Z
updated: 2026-08-12T13:02:32.221Z
template_sig: 86ce4036
rendered_sig: bd87c388
---

# Discover — The naive-arm control and the Postgres mutant column

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one line: the CF-13 rule is shown to *reject* a naive `nextval()` arm, and the rule set is driven directly from the Postgres side as the mutant control, so "it passed" means the adapter had to work for it | `_storymap.md`, *Slices* table, `postgres-rule-controls` row | Two controls, one story, because both answer the same question: "would this rule have noticed?" |
| **AC-002** — the visibility rule is one the adapter had to work to pass, and ADR-0024 states what that work cost | `project.md`, *Acceptance criteria*, AC-002 | This story owns the proof it was not free: the naive arm fails, and the poll-padding decorator is built or explicitly excused |
| **AC-004** — the phase-3 mutant harness runs with `PostgresEventStore` in the pass column | `project.md`, *Acceptance criteria*, AC-004 | This story owns the mutant-control column; HS-S0062 owns the two conformance families (`_storymap.md`, *Coverage*, AC-004 row) |
| `depends_on: postgres-append-and-frontier-head` (HS-S0062) | manifest; `_storymap.md`, *Merge order* item 2 | Supplies the real `append` and frontier `head` the controls are run against, and the live job they run inside |
| The rule functions are public: `happenstance_testkit::rules::<name>` | `crates/happenstance-testkit/src/lib.rs:189` | The seam that lets a test in `crates/happenstance-postgres/tests/` drive the rule set directly, without the testkit depending on an adapter |
| Registering a live Postgres store in `mutation_coverage.rs`'s `REGISTRY` would invert the dependency direction and put a live server inside `cargo test --workspace` | `crates/happenstance-testkit/tests/mutation_coverage.rs:324`, `:2054`; `_decomposition.md`, *Architecture brief* §9.1 | CLAUDE.md forbids adapter-on-adapter and testkit-on-adapter dependencies; DR-9 forbids the live server. §9.1 is a **seam decision this story records**, not a `Cargo.toml` edit |
| The harness's correct core is in-memory, single-threaded (`Rc`/`RefCell`) and built so no `AssertUnwindSafe` appears anywhere, because the probes are `UnwindSafe` function pointers | `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:329-332` | A live async store cannot be dropped into that shape. `catch_unwind` over a non-capturing probe is the in-tree pattern for asserting a rule *rejects* an implementation |
| `PreCommitPositionStore` is the CF-13 fixture instrument, and its defect is the sequence "advanced *outside* the transaction that will publish the row" | `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:3752-3758` | The naive `nextval()` arm this story builds is the *adapter*-shaped copy of exactly that defect |
| `nothing_below_an_observed_position_appears_later` has a strength that varies with the adapter's poll shape: it polls two `append` futures A, B, B, A, `Fixture` cannot express a poll budget, and against a store whose `append` needs three polls "the interleaving window never opens where the rule looks and the rule cannot fail" | `spec/SPECIFICATION.md:2844-2848` | The single most important signal in this story. A real Postgres `append` is exactly that multi-poll shape |
| The bounding instrument — a poll-padding decorator over `PreCommitPositionStore` — is "named and owed by **phase 10** (ADR-0024); if it fires, the rule changes and this clause does not" | `spec/SPECIFICATION.md:2848-2850` | The clause pre-authorises a rule change and forecloses a clause change. ES-10 is `[FROZEN]` and stays that way whatever the decorator finds |
| The naive arm is a throwaway, not a second permanent fixture | `_decomposition.md`, *Testing brief* → *Notes* §2 | A feature-gated path exercised once and recorded in the ADR. Keeping a deliberately-broken Postgres fixture alive long-term would itself need a stated reason |
| Instrument-portfolio row: "fixture, phase 3 — `PreCommitPositionStore` fails … deterministically on one thread (CF-13). **Adapter at phase 10**" | `RUNBOOK.md:687` | This story is the adapter column that row has been waiting for |
| CF-6's linter scans only `crates/happenstance-testkit/src/{suite,model,concurrency}.rs` | `xtask/src/lints.rs:631`; `xtask/src/spec_trace.rs:85-89` | Neither this story's control in `crates/happenstance-postgres/tests/` nor a new mutant under `crates/happenstance-testkit/tests/` is scanned. CF-6 holds here by discipline |

## Questions

**Answered.**

1. *Where does the mutant control mount?* In `crates/happenstance-postgres/tests/`,
   driving `happenstance_testkit::rules::<name>` directly inside the live job — not
   as a `mutation_coverage.rs` `REGISTRY` row. Both reasons are structural rather
   than stylistic: the registry entry makes the testkit's test binary depend on an
   adapter, and it puts a live server inside `cargo test --workspace`
   (`_decomposition.md`, *Architecture brief* §9.1). If the implementer concludes
   the registry entry is right after all, §9.1 requires a **written decision**, not
   a manifest edit; `spec` carries that instruction verbatim.
2. *Is the poll-padding decorator in scope?* Yes, or a written excuse is. The clause
   names it as owed by this phase (`spec/SPECIFICATION.md:2848-2850`) and AC-002 is
   not fully evidenced until it is built or ADR-0024 records why a real multi-poll
   `append` did not need it (`_decomposition.md`, *Architecture brief* §10).
3. *Where would the decorator live?* Beside the store it decorates, in
   `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs`. It needs no
   server, stays single-threaded, and preserves the `UnwindSafe` probe shape the
   harness depends on (`crates/happenstance-testkit/tests/mutation_coverage/harness.rs:329-332`).
4. *Does a rule change need an ADR?* No — `spec/SPECIFICATION.md:2848-2850` settles
   it in advance: "if it fires, **the rule changes and this clause does not**." ES-10
   stays `[FROZEN]` and untouched; the change is to
   `nothing_below_an_observed_position_appears_later`, with its reason given in the
   same change and a CHANGELOG entry, which CF-29's lint enforces
   (`xtask/src/lints.rs:525`).

**Deferred to the owning stories.**

5. *How the adapter buys ES-10's visibility invariant, decided on numbers.*
   `adr-0024-position-visibility-mechanism` (HS-S0065). This story supplies the
   control that decision cites; it does not choose.
6. *Whether `conflicting_position` is a promise every adapter owes or a hint one may
   omit.* `neon-conflicting-position-verdict` (HS-S0070). Neon's negative control,
   `ProbeThenWriteStore`, uses the same public-rule-function seam this story
   establishes, which is the only coupling.

**Deferred to `spec`.**

7. *How the naive arm is spelled* — a `nextval()`-based feature-gated path in
   `happenstance-postgres`, or a second `impl` behind a `cfg`. Either satisfies
   "exercised once and recorded"; the constraint is that it does not survive as a
   maintained fixture.

## Decision

The problem this slice solves is that a rule which no implementation can fail is
decorative, and this project is about to claim that a Postgres adapter *had to
work* to pass CF-13. Two things have to be true for that claim to mean anything,
and neither is established by a green suite. First, the same rule must be shown to
**reject** a naive `nextval()` build of the same adapter — the adapter-shaped copy
of `PreCommitPositionStore`'s defect, which the instrument-portfolio table has been
listing as "adapter at phase 10" since phase 3. Second, the rule must be capable of
failing against a store with a real, multi-poll `append` at all: the specification
already records that the rule polls A, B, B, A and that against a three-poll
`append` the interleaving window never opens where it looks, and names the
poll-padding decorator over `PreCommitPositionStore` as the bounding instrument
this phase owes. The spec will cover: the naive-arm control and how its failure is
recorded; the seam decision that the control drives public rule functions from
`crates/happenstance-postgres/tests/` rather than through the mutation registry,
written down as a decision; the poll-padding decorator, its home beside
`PreCommitPositionStore`, and what happens if it fires; and the ADR-0024 text this
story owes if the decorator is excused instead of built. No `[FROZEN]` clause
changes — ES-10's own clause says the rule gives and the clause does not — so no
ADR is owed by this story.

## The wrong implementation

**A naive-`nextval()` `PostgresEventStore` that *passes*
`nothing_below_an_observed_position_appears_later`.** This is the mutant, and its
danger is that it is the *expected* outcome, not an unlikely one. Strip the
visibility machinery: `position bigserial`, plain `INSERT … RETURNING position`, no
`xid8` column, `head` as `SELECT max(position)`. Run the rule. The rule drives two
`append` futures A, B, B, A, but a real `append` needs several polls — acquire a
connection, `BEGIN`, execute, `COMMIT` — so the second future never reaches its
allocation inside the window the rule opens, and the rule reports green
(`spec/SPECIFICATION.md:2844-2848`). Everything else is green too. The control is
then recorded backwards: the naive arm "passed", the implementer concludes the
mechanism was unnecessary or that the rule is satisfied by any Postgres adapter,
and AC-002's "hard-won" claim is written on a rule that could not have failed
either way. The workspace ends up with a `[FROZEN]` ES-10 whose only adapter-level
evidence is a rule with no teeth against adapters.

Two instruments reject it, and both are this story's:

- **The poll-padding decorator over `PreCommitPositionStore`**, in
  `crates/happenstance-testkit/tests/mutation_coverage/mutants.rs` beside the store
  it wraps (`:3752`). It pads `append` to the poll count a real adapter needs while
  keeping the known defect, so it bounds the rule's strength directly: if the
  padded store passes, the rule cannot detect the defect at that poll shape, and
  the rule is fixed in the same change with the reason given —
  `spec/SPECIFICATION.md:2848-2850` authorises exactly that and forecloses touching
  the clause.
- **The naive arm run through the public rule functions** from
  `crates/happenstance-postgres/tests/` (`crates/happenstance-testkit/src/lib.rs:189`),
  asserting failure by name via `catch_unwind` over a non-capturing probe, the
  in-tree pattern at
  `crates/happenstance-testkit/tests/mutation_coverage/harness.rs:329-332`. Asserting
  "the arm failed" without pinning *which* rule failed and why is the weaker control
  that lets a decode panic or a missing column masquerade as a visibility finding;
  `spec` requires the assertion to name the rule.

The tempting shortcut — adding `PostgresEventStore` to `mutation_coverage.rs`'s
`REGISTRY` (`:324`, `:2054`) — is the third wrong implementation. It reads as the
literal discharge of AC-004's wording, it would compile, and it would make the
testkit's test binary depend on `happenstance-postgres` and drag a live server into
`cargo test --workspace`, breaking CLAUDE.md's dependency rule and DR-9 in one move.

**On literal positions.** Nothing here asserts one. The decorator is a mutant, not a
rule, and it inherits `PreCommitPositionStore`'s shape rather than any position
literal; the control compares against the positions the store actually assigned. As
in HS-S0062, `cargo xtask lint-position-literals` does not scan either location
(`xtask/src/lints.rs:631`; `xtask/src/spec_trace.rs:85-89`), so `spec` states the
constraint rather than relying on the gate to enforce it.

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
