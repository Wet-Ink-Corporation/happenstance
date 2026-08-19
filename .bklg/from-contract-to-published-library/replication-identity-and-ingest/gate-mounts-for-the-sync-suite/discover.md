---
item: HS-S0104
stage: discover
created: 2026-08-12T13:03:24.987Z
updated: 2026-08-12T13:03:24.987Z
template_sig: 86ce4036
rendered_sig: 13780d52
---

# Discover — The gate is taught about a fourth suite

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: teach every gate constant that names only `happenstance-testkit` about the fourth suite — `RULE_FILES`, `TESTKIT_SRC`, `TESTKIT_MANIFEST`, the position-literal lint and the changelog-per-rule lint — and add the two `wasm32` steps (a build of `happenstance-sync` and a check of the sync harness) beside the existing four | `_storymap.md`, *Slices* table, `sync-conformance-suite` row 2 | Five constants and two steps. Every item is a hard-coded single-crate value today |
| **Depends on `sync-testkit-crate-and-rule-registry` (HS-S0103)**, which supplies the crate the constants must name, its `version` key, and the rules module the file list must sweep | `_storymap.md`, *Slices* `depends_on` | There is nothing to point a constant at until the crate exists |
| **AC-004** — the suite discriminates; the mutant registry has provenance, and *"the suite decodes no payload byte on any path"* | `project.md`, *Acceptance criteria*, AC-004 | This story owns the `lint-position-literals` scope half; the registry itself is HS-S0105's |
| **AC-009** — the ingest path and runner bind the weaker flavour, and the whole path is *"built and exercised for `wasm32` inside the gate rather than asserted in prose"* | `project.md`, *Acceptance criteria*, AC-009 | *"this is the file that decides whether that sentence is true"* (`_decomposition.md`, *Gate mounts*) |
| **AC-014** — the clause arithmetic, with `cargo xtask spec-trace` green | `project.md`, *Acceptance criteria*, AC-014 | **AC-014 cannot pass without the `RULE_FILES` edit.** The arithmetic runs through the tool this story mounts |
| `RULE_FILES: [&str; 3]`, all three under `crates/happenstance-testkit/src/` — the set `spec-trace` sweeps so every rule is claimed by a clause and every clause resolves to a rule; it lives beside `collect_rules` so *"a list of files kept next to the function that parses them cannot drift from it"* | `xtask/src/spec_trace.rs:85-89` | A sync rules file absent from this array means every `SY` clause keeps rendering as scheduled and no sync rule can ever be orphan-checked |
| `spec-trace` treats a `Rule:` line containing `(new)`, `†`, a leading `new `, or the words *unit test / compile test / meta-test* as **scheduled**, so the rule is never resolved | `xtask/src/spec_trace.rs:1626-1634` | Every `SY` clause's `Rule:` today says `(new, happenstance-sync-testkit)`. Dropping those markers is HS-S0112's; making the resolution *possible* is this story's |
| `TESTKIT_SRC` — CF-33's no-clock scope, `src/` and deliberately not the whole crate, *"because a whole-crate grep would fire on `tests/` and teach the next person that the remedy is to widen the exclusions, which is the direction that ends with the check switched off"* | `xtask/src/lints.rs:33-42` | Extend the scope; do not widen the exclusions. The comment states the failure mode |
| `TESTKIT_MANIFEST` — CF-32's own-version check, *"the cheapest step in the gate: the testkit's version must not be `version.workspace = true`, because adding a rule is semver-MINOR for the bar and nothing at all for the contract"* | `xtask/src/lints.rs:45`; `xtask/src/main.rs:420-436` | The new manifest carries its own `version` (HS-S0103); this story is what makes the omission fail |
| `lint-position-literals` — CF-6's cheap second line, *"this catches the habit at the spelling, which is the half that names the rule rather than the store"* | `xtask/src/main.rs:375-392`; `spec/SPECIFICATION.md:7233-7249` | The lint reads `RULE_FILES`-shaped file lists. Its scope and `spec-trace`'s are the same edit twice |
| `changelog_names_every_rule` — CF-29, one entry per rule naming a defect; its first run found twenty-five of fifty-five rules with no entry | `xtask/src/lints.rs:518-530`; `xtask/src/main.rs:394-405` | *"a new suite starts at zero"* (`_decomposition.md`, *Gate mounts*). Every sync rule owes a `CHANGELOG.md` entry naming a defect |
| The four existing `wasm32` steps: a contract-crate build, a harness check, and builds of `happenstance-cloudflare` and `happenstance-neon` | `xtask/src/main.rs:203-283` | AC-009's mount is a **fifth and sixth** beside these, not a replacement |
| The harness step's own reason: `memory_conformance_wasm.rs` and `local_conformance.rs`'s wasm half are both behind `cfg(target_arch = "wasm32")`, *"so on a native run they compile to nothing at all. Type-checking them here is what stops `__emit_wasm` from rotting into a macro nobody has compiled since the day it was written — the same decorative-check failure the nightly docs step had"* | `xtask/src/main.rs:221-232` | A crate build is not a harness check. Two steps because they prove two different things |
| `xtask/src/main.rs:559-590` is the `wasm32` feature powerset, *"which is where the sync crates' feature combinations belong"* | `_decomposition.md`, *Gate mounts*, third bullet | The `memory` feature interacts with `wasm32`; the powerset is where that gets exercised |
| **SY-17 `[FROZEN]`** — the sync port and its runner MUST be written against `EventStore`, not `SendEventStore`; the rule is *"the existing CLAUDE.md rule 4, checked by `happenstance-sync-testkit` compiling its own suite against a `!Send` fixture peer"* | `spec/SPECIFICATION.md:6341-6360` | The clause's enforcement is a **compile**, on a target where the `Send` flavour is unsatisfiable. That compile is this story's step |
| The `wasm32` build of `happenstance-core` is the standing guard on binding constraint 1, and a step skips only when its probe fails to find the tool — a step that runs and finds a problem always fails the gate | `CLAUDE.md`, *Commands*; `xtask/src/main.rs:196-203` | Adding a step that can skip silently is the one thing this story must not do |
| `.redkiln/config.yaml:48` wires `cargo xtask lints && cargo xtask spec-trace` to the story grain, and `:50-55` wires `cargo xtask ci --fast` to the integration grain | `project.md`, *Definition of done* 1-2; `_decomposition.md`, testing brief *Merge-gate commands* | These run whether or not anyone types them, which is exactly why the constants they read must be right |

## Questions

**Does ingest re-check the writer's asserted append conditions? — Not this
story's question, and this story is the reason it stays answered.** SY-1 and SY-6
are `[FROZEN]` and each names a rule in `happenstance-sync-testkit`. Until
`RULE_FILES` includes that crate, `spec-trace` cannot resolve either `Rule:`
field, so both clauses keep rendering as *scheduled* and neither can be shown to
be enforced by anything. The answer is written; this story is what stops it
rotting into decoration.

**Hub-and-spoke versus peer-to-peer — not this story's.** ADR-0027 owns it. The
gate mounts are topology-blind.

**Should `RULE_FILES` grow, or should it become a per-crate structure? —
Deferred to spec, with the constraint named.** The array is `[&str; 3]` and its
own comment ties it to `collect_rules` so the two cannot drift
(`xtask/src/spec_trace.rs:80-89`). A fourth entry is the smallest change; a
per-suite structure is the more honest one once a second suite exists and a
clause needs to say *which* suite claims its rule. Both are defensible and the
choice has a real cost either way — a flat list makes "orphaned in which suite?"
unanswerable, a structure touches every call site.

**Do `TESTKIT_SRC` and `TESTKIT_MANIFEST` become lists, or does the lint take a
crate argument? — Deferred to spec.** Same shape as above, and the same
constraint applies: whatever lands must make the *absence* of the sync crate a
failure, not a silent pass. `AC-A05` allows a written reason for not extending a
given constant; a reason is acceptable and silence is not.

**Are two `wasm32` steps enough, or does the runner need its own? — Deferred to
spec, and the answer probably depends on where the runner lands.** A build of
`happenstance-sync` covers the runner if the runner lives there
(`_decomposition.md`, *Non-prescriptive implementation notes* leaves that open).
What is fixed is that a *harness* check is a separate step from a *crate* build,
because a harness behind `cfg(target_arch = "wasm32")` compiles to nothing on a
native run.

**Does the changelog lint apply retroactively to the sync rules that do not
exist yet? — Answered: it applies as each rule lands, and this story is what
makes the omission fail.** Wiring it before the rules exist is deliberate — it is
the ratchet-gate order, land the check with a green baseline of zero rules, so
the first rule that arrives without an entry is the one that goes red
(`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md`).

## Decision

`crates/*` is a workspace glob, so `happenstance-sync-testkit` becomes a
workspace member the moment its directory exists — and that is exactly how it
escapes the checks that name crates explicitly. Five gate constants hard-code
`happenstance-testkit`: `RULE_FILES`, which is the set `spec-trace` sweeps in both
directions; `TESTKIT_SRC` and `TESTKIT_MANIFEST`, which are CF-33's and CF-32's
scopes; and the file lists behind `lint-position-literals` and
`changelog_names_every_rule`. A second suite that none of them knows about
compiles, runs, and is invisible to every one of them: its rules can be orphaned,
its clauses stay *scheduled* forever, it may assert literal positions, it may
declare `version.workspace = true`, and it owes no changelog entry. Separately,
the four `wasm32` steps name the contract crate, the existing harness, Cloudflare
and Neon, and none of them compiles the sync path — so AC-009's *"built and
exercised for `wasm32` inside the gate rather than asserted in prose"* is
currently false by construction, and SY-17's `[FROZEN]` `Rule:` field, which is
literally *"`happenstance-sync-testkit` compiling its own suite against a `!Send`
fixture peer"*, has nothing compiling it. This story mounts the fourth suite into
the machinery. The spec stage will cover: each of the five constants, the shape
chosen for each (extended list versus per-suite structure) and the written reason
for any left alone; the fifth and sixth `wasm32` steps — a build of
`happenstance-sync` and a **check of the sync conformance harness** as two
separate steps; the sync crates' entries in the `wasm32` feature powerset; and
the assertion that no new step carries a probe that can make it skip silently.

## The wrong implementation

**The mutant is the diff that is not written, and it passes everything.** Land
`happenstance-sync-testkit` with a full rule set and change nothing in `xtask/`.
`cargo fmt`, `cargo clippy -D warnings`, `cargo test --workspace --all-features`
and `cargo xtask ci --fast` are all green — the crate is a workspace member and
its tests run. `cargo xtask spec-trace` is green, because it sweeps three files
under `crates/happenstance-testkit/src/` and finds nothing wrong in them.
`cargo xtask lints` is green for the same reason. Every `SY` clause continues to
render as *scheduled*, because its `Rule:` line says `(new,
happenstance-sync-testkit)` and `(new)` is the token that means unresolved
(`xtask/src/spec_trace.rs:1626-1634`) — so the specification reports thirty-five
replication clauses awaiting rules while thirty-five rules sit in the tree
enforcing them, and nothing anywhere notices the mismatch. This is the
**decorative-suite** mutant, and its distinguishing feature is that it looks like
*more* work done, not less: a suite exists, it is green, the clauses say the work
is scheduled. It must be refuted by an assertion inside `xtask`'s own tests that
the sweep set is non-empty for the sync crate — a check that the file list
actually resolves to files containing rules, rather than that the list has four
entries.

**Its `wasm32` twin is sharper, because a partial fix reads as a fix.** Add one
step, `cargo check -p happenstance-sync --target wasm32-unknown-unknown`, and
stop. The gate goes from four steps to five, `cargo xtask wasm` runs longer, and
the AC-009 box gets ticked. It proves the *library* compiles and says nothing at
all about the *suite*, because a conformance harness lives behind
`cfg(target_arch = "wasm32")` and compiles to nothing on any other target — which
is precisely why the existing harness step exists as its own step and says so in
its comment (`xtask/src/main.rs:221-232`). SY-17's frozen `Rule:` field is
discharged by the harness compile and by nothing else, so a build-only step
leaves a `[FROZEN]` clause enforced by a step that cannot fail for the reason the
clause names.

**And the quietest one: extend the lint scopes to the crate but not to the file
the rules live in.** Point `TESTKIT_SRC`-equivalent at
`crates/happenstance-sync-testkit/src` while the rules are written in
`crates/happenstance-sync-testkit/src/rules/` behind a module the sweep's glob
does not reach, or point `lint-position-literals` at a file list that omits the
one file with assertions in it. The lint runs, finds nothing, and prints a
success line. A check that runs and cannot fail is worse than an absent check,
because the absent one is at least visible as a gap; this one is a green line in
CI output. The guard is the same in both cases: the sweep must be asserted
non-empty, and the counts it reports must be visible.

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

**Box 6, and this story is the box's own enforcement.** It adds no conformance
rule; it extends `lint-position-literals` — CF-6's mechanism
(`spec/SPECIFICATION.md:7233-7249`) — to cover the file every sync rule will be
written in. The story's obligation is therefore stronger than a promise about its
own diff: it must leave the lint able to *fail* on a sync rule, verified by
temporarily writing a literal-position assertion into the sync rules file and
confirming the lint names it. CF-5's `GappedPositionStore` remains the
behavioural enforcement and the lint the cheap second line, in that order
(`spec/SPECIFICATION.md:7233-7241`).

**Box 7.** This story edits no `[FROZEN]` clause and no clause text at all — its
whole diff is `xtask/` plus `CHANGELOG.md`. It is the mechanism by which two
frozen clauses (SY-17's compile-against-a-`!Send`-fixture rule, and every `SY`
clause whose `Rule:` must eventually resolve) become checkable. ADR-0026 is
written first and reaches this story through
`sync-testkit-crate-and-rule-registry`.
