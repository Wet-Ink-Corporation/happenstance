---
item: HS-S0055
stage: discover
created: 2026-08-12T13:02:17.163Z
updated: 2026-08-12T13:02:17.163Z
template_sig: 86ce4036
rendered_sig: 5c27de18
---

# Discover — The fixture's numeric limits are measurements, not guesses

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: measure rather than guess `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and `MAX_EVENTS_PER_BATCH`, and decide `REOPEN` and `MID_BATCH_FAULT` honestly against the real runtime, so CF-39/CF-40's rules pass at the boundary **in both directions** | `_storymap.md`, **Slices**, `measured-store-limits` row | "Both directions" is the whole content: accepted at the number, refused at one more |
| It is a story rather than a checkbox because the numbers are facts, not trades: stating one promises exactly that many bytes accepted and one more refused as `ExceedsStoreLimit`, and a guessed number fails in one direction or the other | `_storymap.md`, **Why these milestones and not others** | A guess is caught by the rule. What is not caught is not guessing at all |
| AC-007 (the two non-type-error limits have stated resolutions) and AC-008 (CF-39 and CF-40 discharged, CF-40's ownership resolved) | `project.md`, **Acceptance criteria** | This story owns the measured half of both; the atom is `adr-0023-and-atom-resolutions`' |
| `dependsOn: durable-object-host-and-fixture` — the constants' home; `dependsOn: every-rule-under-workerd` — the only place the numbers can be measured *and* the only place `append_reports_exceeded_store_limits` can be observed against them | `_storymap.md`, **Slices** and **Merge order** §3 | The measurement and its verification are the same run. That is why this story sits last in its milestone |
| The three ceilings are `Option<usize>` **facts, not trades**, defaulting to `None` | `crates/happenstance-testkit/src/contract.rs:214-279`, defaults at `:253`, `:262`, `:279` | The default is a *statement* — "this store has no ceiling" — and for this runtime it is a false one |
| `NO_STORE_LIMITS` is the reason reported when all three are `None`, and the rule returns `Skipped` through the same path a declined `Capability` uses | `crates/happenstance-testkit/src/contract.rs:230`, `:442` | The rule is emitted and answers honestly. It is still testing nothing |
| The testkit's own `MUST_SKIP` list names `append_reports_exceeded_store_limits` and says its gate is CF-40's three `Option<usize>` ceilings rather than a `Capability` — "the reporting obligation is identical either way" | `crates/happenstance-testkit/tests/mutation_coverage.rs:3144-3164` | The skip is legitimate by construction, which is exactly what makes leaving it on dangerous here |
| Four mutants are already registered against that rule: `PayloadCeilingStore` (refuses through `AppendError::Store`), `TruncatingPayloadStore` (does not refuse at all), `BatchParameterCeilingStore` (the driver's parameter ceiling met at write time), `ChunkLosingBatchStore` (`&events[..CEILING]` where `chunks` was meant) | `crates/happenstance-testkit/tests/mutation_coverage.rs:1919-2040` | The declared-but-wrong failures are covered. The undeclared one is not, and cannot be by a store |
| CF-40 requires a stated ceiling to be refused as `ExceedsStoreLimit` naming the corresponding `StoreLimit`, never as `AppendError::Store` and never by truncating | `spec/SPECIFICATION.md:7661-7674`; `crates/happenstance-testkit/src/contract.rs:239-247` | The channel is `durable-object-write-path`'s; the numbers that make it reachable are this story's |
| CF-39 covers `MID_BATCH_FAULT`, and no adapter in the workspace has exercised it for real; a Durable Object can arm one with a `CHECK` constraint or trigger | `spec/SPECIFICATION.md:7621-7660`, ledger row `:8750`; `_decomposition.md`, Architecture brief §4d | New *conformance suite* coverage, not just new adapter coverage — which is why it carries a changelog obligation |
| CF-29's lint requires a changelog entry for a rule a fixture newly exercises for real | `xtask/src/main.rs:20-24`; `_decomposition.md`, Testing brief Notes §3 | The one mechanical hook this story has, and it is a review hook rather than a test |
| The suite's four guaranteed minima — 65,536 bytes of payload, 64 tags, a 128-item query, a 128-event batch | `crates/happenstance-testkit/src/lib.rs:145` (Value edges row) | A measured ceiling *below* a minimum is a conformance failure, not a declaration. A fixture cannot declare its way under the floor |
| CF-40's ownership is contested between two accepted, unedited ADRs by one of them contradicting itself, and the atom names phase 8 (`happenstance-sqlite`) as what forces it while HS-P0012 merges one position ahead of this project | `.kb/open-questions/cf-40-fixture-limits-ownership.md`, *What is not decided* and *What forces it*; `_decomposition.md`, Architecture brief §6 | Neither project may mint an answer independently. Whichever reaches it first owns it and the other cites it |

## Questions

**CF-40's fixture-limits ownership — the live one, answered as far as this story
can reach.** This story does **not** mint it. The contradiction is between two
accepted, immutable ADRs, so resolving it is a knowledge-base write, and every KB
write in this project belongs to `adr-0023-and-atom-resolutions`, coordinated with
`sqlite-durable-store` (HS-P0012). What this story *owes* that story is the
evidence: three measured numbers, and a statement of whether declaring them was
actually blocked by not knowing which document owns the clause. It will not have
been — CF-40's text is identical under either reading, as the atom itself observes
— and that is itself the finding, because it narrows the atom's question from
"which ADR owns CF-40" to its own second sub-question: whether the fixture contract
has one owning document at all.

**Measured, or read off a documentation page?** Answered: **measured**, against
the real runtime, under the wasm32 conformance run. A published platform figure is
a starting point and never the declaration, because the declaration is a promise
about *this adapter's* behaviour at the boundary — exactly that many bytes
accepted, one more refused — and an adapter can be narrower than its platform for
reasons of its own schema (`crates/happenstance-testkit/src/contract.rs:214-279`).

**What if a measured ceiling falls below a guaranteed minimum?** Deferred to
**spec** in detail, but the shape is settled now: that is a conformance
**failure**, not a declaration. The four minima are the suite's floor and a fixture
cannot declare its way under them; the response is a schema or batching change in
the adapter, or a blocking finding — never a lower number reported as a fact.

**`MID_BATCH_FAULT`: claim `SUPPORTED` and arm a real seam, or decline?** Deferred
to **spec**, jointly with `durable-object-host-and-fixture`. Claiming it would make
this the first adapter in the workspace to exercise CF-39 for real and would carry
a CF-29 changelog entry; claiming it with an empty or fake arm is worse than
declining, because it turns `append_is_atomic_under_a_mid_batch_fault` into a
vacuous green, which the rule's own documentation spells out
(`crates/happenstance-testkit/src/suite.rs:2820-2831`).

**`REOPEN`.** Declared by `durable-object-host-and-fixture`; re-read for CF-14 by
`deferral-re-reads-and-es-32-verdict`. This story only confirms the declaration
survived contact with the run.

**The off-tokio harness shape.** Not this story's; inherited from
`every-rule-under-workerd` and recorded in ADR-0023.

## Decision

`Fixture`'s three numeric ceilings are the only place in the conformance contract
where a fixture makes a *quantitative* promise, and the promise is symmetric:
declare `N` and the suite will check that `N` is accepted and `N + 1` is refused as
`ExceedsStoreLimit` naming a `StoreLimit`. That symmetry is what makes a guess
useless — it fails in one direction or the other — and it is why the numbers cannot
be lifted from a documentation page and must come out of the runtime under the run
that will then check them. This slice takes those three measurements against a real
Durable Object, settles `REOPEN` and `MID_BATCH_FAULT` against what the host
actually permits, and thereby turns `append_reports_exceeded_store_limits` from a
rule that reports `NO_STORE_LIMITS` into a rule that reaches the store. The spec
will cover: how each of the three numbers is measured and what the measurement
harness is; the `StoreLimit` each maps to; the check that no measured ceiling falls
below the suite's guaranteed minima; the `MID_BATCH_FAULT` decision and, if it is
`SUPPORTED`, the armed seam and the CF-29 changelog entry it owes; and the evidence
package handed to `adr-0023-and-atom-resolutions` for CF-40, together with the
coordination check against `sqlite-durable-store` so the answer is minted once.
Nothing `[FROZEN]` is amended: CF-39 and CF-40 are `[PROVISIONAL]` and are
discharged rather than changed.

## The wrong implementation

**Not the guess — the rule catches that, and catching it is the rule doing its
job.** Declare `MAX_EVENT_DATA_LEN = 1_000_000` because a blog post said Durable
Object SQL rows cap near a megabyte, and
`append_reports_exceeded_store_limits` fails at one boundary or the other:
too high and the store refuses at the declared maximum; too low and it accepts one
past it. That is a red test, which is the good outcome.

**The mutant is the one that does not fail: leave all three at `None`.** The
default is `None` on each of the three constants
(`crates/happenstance-testkit/src/contract.rs:253`, `:262`, `:279`), the rule
returns `Skipped` carrying `NO_STORE_LIMITS` (`contract.rs:230`, `:442`), and every
existing check is satisfied. The gate is green. The rule is emitted, answers,
prints the fixture's stated reason, and appears in the run's output exactly as
CF-18 requires. `capability_skips_are_reported` is content, because it checks that
every rule produces an outcome and not that any rule reached a store. And the one
runtime in this workspace with a hard, documented SQL storage cap
(`SqlError::StorageLimitExceeded`, `crates/happenstance-cloudflare/src/sql_storage.rs:119-121`)
has contributed nothing at all to CF-40, the clause it was brought in to discharge.
This is the project's own named risk — AC-003 satisfied by silence — reappearing one
level down, in a *numeric* declaration rather than a boolean one, where the skip
reason is not even suspicious because it is the honest default for a store that
genuinely has no ceiling.

**Where the detectors live, and the one that must not be written.** The
declared-but-wrong shapes are already registered and this story's job is to make
them reachable rather than to add to them: `TruncatingPayloadStore` clamps and
raises a warning into a log nobody reads; `ChunkLosingBatchStore` is `&events[..CEILING]`
where `events.chunks(CEILING)` was meant, returning `Ok` with a real position so
nothing downstream notices; `PayloadCeilingStore` refuses through the channel CF-40
replaced; `BatchParameterCeilingStore` meets its driver's ceiling after the caller
has taken its side effects — all at
`crates/happenstance-testkit/tests/mutation_coverage.rs:1919-2040`. The `None` case
has **no** registered mutant and must not be given one: no *store* can fail that
rule, because the defect is in the fixture's declaration rather than in the store's
behaviour, and `CLAUDE.md`'s corollary forbids a rule no adapter can fail. Its
detector is therefore a named review obligation rather than a test — the CF-29
changelog entry recording that this adapter newly exercises
`append_reports_exceeded_store_limits` for real (`xtask/src/main.rs:20-24`) — plus
the ADR-0023 record of the three numbers and how each was measured, so a reader can
tell a measurement from an omission.

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

**Box 6.** No conformance rule is added here. The rule this story turns on,
`append_reports_exceeded_store_limits`, asserts over payload bytes, tag counts and
batch sizes at the fixture's own declared boundary — never over positions. The
measurement harness likewise compares acceptance and refusal against the numbers
the store was given, not against positions the store assigned, so the
specification's permission for gaps is untouched.

**Box 7.** Nothing `[FROZEN]` is changed. CF-39 and CF-40 are both
`[PROVISIONAL]` and are discharged, not amended. If a measured ceiling fell below
one of the suite's guaranteed minima — which *are* frozen value-edge guarantees —
the answer is a change to this adapter or a blocking finding, and if it genuinely
could not be met, a new decision atom and a re-plan written first. A fixture is
never excused from a minimum by declaring a smaller number.

**Box 8.** No conformance rule here seems wrong. The one that looks wrong on first
reading — a rule a fixture can switch off by leaving three constants at their
defaults — is correctly gated, because a store with no ceiling has nothing to
promise; what this story does about it is supply the numbers rather than change the
gate, and the CF-18 reporting obligation is what keeps the off state visible in the
meantime.
