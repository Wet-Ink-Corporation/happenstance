---
item: HS-S0008
stage: discover
created: 2026-08-12T13:01:15.582Z
updated: 2026-08-12T13:01:15.582Z
template_sig: 86ce4036
rendered_sig: b32dfdde
---

# Discover — A declined capability is a reported skip, never a silent absence

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: a rule whose projection capability the fixture declines is still emitted as a test, returns `RuleOutcome::Skipped` carrying the fixture's stated reason, and is distinguishable from a pass in the harness output — asserted on `RuleOutcome` values rather than on stdout, with the projection capability set fixed by DT-3's recorded resolution rather than invented here | `../_storymap.md:60` | Two halves: the machinery, and the *value* assertion that proves it. The capability set arrives from upstream |
| **AC-005** — a declined capability is still emitted as a test, returns a skip carrying the fixture's stated reason, and is distinguishable from a pass in CI output; **asserted on `RuleOutcome` values rather than on stdout** | `../project.md:194-197` | This story owns the machinery and the value assertion; `read-through-and-rebuild-rules` owns the real `READS_THROUGH_BATCH = false` instance (`../_storymap.md:83`) |
| `dependsOn: projection-suite-entry-point, projection-api-design-record` — the first supplies the fixture trait, the enumeration and the emitters; the second supplies the capability set and DT-3's authoritative reason source | `../_storymap.md:55,59-60,109` | The second edge is what stops this story minting a declension policy of its own |
| The initiative-level promise this discharges: BR-13 / AC-05 — the adapter author is "told, with a reason, where a guarantee does not apply to them"; a declined capability "reports the fixture's stated reason and still appears in the output; it never vanishes from the binary" | `../_decomposition.md:38-43` (UX brief, quoting the charter); `../project.md:152-154` (DR-03) | "Never vanishes from the binary" is the operative clause and is what rules out `#[cfg]` |
| The machinery already exists and is reused **unchanged**: `Capability` (with `declined(reason)` as a `const fn` carrying an `assert!`) and `RuleOutcome` (`#[must_use]`, no `Failed` variant — a failing rule panics) | `crates/happenstance-testkit/src/contract.rs:372,412,473`; `../_decomposition.md:317-319` (AC-A06) | Nothing is invented. A fork would give an adapter author two skip vocabularies to learn |
| The line shape, reused unchanged: `` SKIP {rule}: fixture declines `{capability}` — {reason} `` | `crates/happenstance-testkit/src/contract.rs:500-507`; `../_decomposition.md:160-169` (AC-U08) | One skip vocabulary, one line shape. "An author reading one CI log must not have to learn two shapes" |
| UX brief **AC-U09**: `RuleOutcome::report` writes to stdout, which libtest suppresses for a *passing* test unless `--show-output` — which is why the gate passes it — and the port's own doc is blunt that this "makes the line reachable by a human; it does not make anyone read it" | `../_decomposition.md:170-180`; `crates/happenstance-testkit/src/contract.rs:515-523,532` | "A promise checked only by stdout is a promise checked by nobody." The value assertion is the load-bearing half |
| The precedent to sibling: `capability_skips_are_reported` in the event-store mutant registry | `crates/happenstance-testkit/tests/mutation_coverage.rs:3184`; `../_decomposition.md:176-178` | A projection sibling of this test is exactly the instrument AC-005 names |
| UX brief **AC-U10**: the skip must name the switch the author actually set. `NO_STORE_LIMITS` names all three constants precisely because "a skip naming only one of them would send an adapter author to look for the constant they did set" | `crates/happenstance-testkit/src/contract.rs:435-442`; `../_decomposition.md:181-188` | The `capability` field's content is an acceptance criterion, not a formatting detail |
| UX brief **AC-U11**: on `wasm32-unknown-unknown` `RuleOutcome::report` is a **no-op**, measured under `wasm-bindgen-test-runner` rather than assumed, so the projection wasm harness must route `skip_line` to `console_log!` the way `__emit_wasm` already does | `crates/happenstance-testkit/src/contract.rs:525-531`; `../_decomposition.md:189-195` | AC-016's run is otherwise the one where the stated reason silently disappears — and it is the run the constrained-target persona depends on |
| UX brief **AC-U12**: `Capability::declined("")` is a `const fn` `assert!`, but on an **associated** const it is evaluated lazily and fails at codegen — `cargo build` and `cargo test` catch it, `cargo check` and `cargo clippy` do not. Any `compile_fail` doctest is spelled **bare**, never `compile_fail,E0080` | `crates/happenstance-testkit/src/contract.rs:400-419`; `../_decomposition.md:196-208` | A reasonless declension must fail the build, and the fixture trait's rustdoc must say where that fires |
| UX brief, *What would make this brief wrong*: if the projection fixture ends with **no declinable capability at all**, this machinery is decorative on this port — "that outcome is not a licence to quietly drop the reporting discipline: it is a finding" | `../_decomposition.md:252-259` | The reshape trigger for this story, and it belongs in DT-3's resolution rather than in a silent deletion |
| Testing brief AC-005: **Unit** (assert directly on the returned `RuleOutcome` value, `#[must_use]`) + **Integration** (the same rule inside a real harness invocation prints `skip_line`'s text) | `../_decomposition.md:775` | Both tiers, and the Unit one is the one that can actually fail |

## Questions

Open questions to resolve before specifying.

1. **Which projection capabilities exist, and is any a MUST in `SECOND_HANDLE`'s
   sense?** Answered upstream and deliberately not here: it is `_design.md`'s
   call via Architecture brief Note 5 (`../_decomposition.md:526-534`), delivered
   by `projection-api-design-record`. This story constrains how a declension
   *reads*, never which ones exist — the UX brief says so in those words
   (`../_decomposition.md:230-232`).
2. **Does a declined MUST panic rather than skip?** Deferred to `spec`, with the
   event-store precedent recorded: `Fixture` **panics** on a declined
   `SECOND_HANDLE` rather than reporting a skip
   (`crates/happenstance-testkit/src/contract.rs:137-161`). Whether the
   projection fixture has an equivalent MUST is question 1's answer.
3. **Does the wasm harness need a projection-specific `skip_line` route?**
   Deferred to `spec`. `__emit_wasm` already routes to `console_log!`
   (`crates/happenstance-testkit/src/registry.rs:276`); whether the projection
   emitter reuses it unchanged depends on which of Note 4's three options
   `projection-suite-entry-point` recorded.
4. **What if nothing is declinable?** Answered as a *process* answer: it is a
   finding recorded in DT-3's resolution, not a reason to drop the machinery
   (`../_decomposition.md:252-259`).
5. **None beyond what the project brief already carries** on CF-40's ownership —
   that atom is cited as authoritative by `projection-api-design-record` and is
   not re-litigated here (`../project.md:322-325`).

## Decision

The initiative promises an adapter author that where a guarantee does not apply
to them, they are told so with a reason — and the projection suite currently has
no way to say it, because it has no way to say anything. This slice wires the
projection family into the reporting discipline the event-store family already
runs: `Capability` and `RuleOutcome` reused unchanged, one skip line shape, the
`capability` field naming the constant the author can actually change, the reason
surviving the wasm32 target where `report` is a no-op, and — the part that makes
it real — a projection sibling of `capability_skips_are_reported` asserting on
`RuleOutcome::Skipped` **values** rather than on stdout. The spec will fix: the
fixture instrument that declines a capability, so the skip path is exercised by
something rather than being a branch nobody takes; the exact assertion shape
(variant, `capability` field content, `reason` non-empty); the wasm routing for
`skip_line`; the fixture trait's rustdoc statement that a reasonless declension
fails at codegen rather than at `cargo check`, with any `compile_fail` doctest
spelled bare; and the finding to record if the capability set turns out to have
nothing declinable in it. No `[FROZEN]` clause is touched — this is testkit
reporting machinery, not `PS-*` text.

## The wrong implementation

**A rule gated with `#[cfg]`, or with an early `return` when the capability is
declined.** Both compile. Both make `cargo xtask ci` green. Both make the rule
count in the CI output *smaller*, which reads as a faster suite rather than a
missing guarantee, and neither tells the adapter author anything at all. The
`#[cfg]` version is the worse of the two because the rule is not merely quiet, it
is **absent from the binary** — the exact phrase DR-03 and the charter's AC-05 use
(`../project.md:152-154`). Nothing in this repository catches it: there is no test
that counts emitted tests, and a rule that was never emitted cannot report
anything to assert against.

The instrument that convicts it must be built here, and it has a named precedent:
a projection sibling of `capability_skips_are_reported`
(`crates/happenstance-testkit/tests/mutation_coverage.rs:3184`), run against a
**projection fixture instrument that declines a capability**, asserting that the
call returns `RuleOutcome::Skipped` with the fixture's own reason string in it.
The event-store side's own arrangement is the model: `MemoryFixture` states no
CF-40 limit and `GappedPositionFixture` is what keeps the limits rule non-vacuous
(`crates/happenstance-testkit/src/fixtures.rs:234-241`;
`../_decomposition.md:420-426`). Without a declining fixture in the tree, the skip
branch is code no test ever enters, and this whole story is decorative — which is
`CLAUDE.md`'s first corollary applied to reporting machinery rather than to rules.

**The subtler one, and it passes today's checks by construction:** a skip that is
reported only through `RuleOutcome::report`'s stdout line. It looks complete — the
line is well-formed, it names the capability and the reason, and a human running
the suite locally sees it. libtest suppresses stdout for a **passing** test unless
`--show-output` is passed, and the port's own documentation is blunt that being
reachable by a human "does not make anyone read it"
(`crates/happenstance-testkit/src/contract.rs:515-523`). A regression that turns
the skip into a silent pass changes nothing any assertion can see. AC-005 names
the value assertion for exactly this reason, and the spec must make the stdout
line the *secondary* evidence rather than the check.

**And the one that disappears on the target that needs it most:** a skip whose
reason is emitted only via `report`, on `wasm32-unknown-unknown`, where `report`
is a measured no-op (`contract.rs:525-531`). AC-016 runs the projection rules on
that target in the same gate run; without the `console_log!` routing `__emit_wasm`
already uses, the constrained-target author is the one person who gets silence.

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

The two judgement boxes. **Literal positions**: this story adds reporting
machinery, a fixture instrument and a meta-test over `RuleOutcome` values — no
conformance rule, and nothing in it asserts a position at all. Vacuously true.
**`[FROZEN]` clauses**: none. The surface changed here is
`crates/happenstance-testkit/src/contract.rs`'s reporting types and the projection
fixture trait; no `PS-*` clause text moves, and CF-18's reported-skip requirement
(`spec/SPECIFICATION.md:5059-5065`) is implemented rather than amended.
