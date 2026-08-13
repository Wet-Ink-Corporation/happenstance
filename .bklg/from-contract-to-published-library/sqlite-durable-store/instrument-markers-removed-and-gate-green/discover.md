---
item: HS-S0045
stage: discover
created: 2026-08-12T13:02:07.823Z
updated: 2026-08-12T13:02:07.823Z
template_sig: 86ce4036
rendered_sig: b22a5a65
---

# Discover — The last todo!() and the scoped allow go together, and the gate is green

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: delete the last `todo!()` on a SQLite path and the scoped `#![allow(clippy::todo)]` in the same change, correct the crate's known-wrong module-doc sketch, and take `cargo xtask ci --fast` green. | `_storymap.md`, *Slices* table, row `publishable-and-reconciled` / `instrument-markers-removed-and-gate-green` | Three deliverables. The middle one — the doc correction — has no gate behind it and is the one that survives a green run. |
| **AC-014** — the instrument markers are gone and the gate is green: no `todo!()` on any SQLite path, `#![allow(clippy::todo)]` deleted from `crates/happenstance-sqlite/src/lib.rs`, and `cargo xtask ci --fast` green. | `project.md`, AC-014; `.redkiln/config.yaml:50-55` | "In the same change" is DR-01's wording, and it is the part a two-commit sequence quietly loses. |
| `dependsOn: model-family-and-mutant-pass-column` (HS-S0042), `reopen-negative-control-and-durability-verdicts` (HS-S0043), `projection-store-passes-the-borrowed-suite` (HS-S0044) — every remaining `todo!()` lives on a path one of the three completes. | `_storymap.md`, *Merge order* item 5; *Slice coherence notes* | "The scoped `#![allow(clippy::todo)]` may only be deleted in the same change as the last `todo!()` (DR-01), which means after the projection store, not before." |
| DR-01: every SQL path has a real body, "and the scoped `#![allow(clippy::todo)]` is deleted in the same change as the last `todo!()` rather than outliving it." | `project.md`, *Derived requirements*, DR-01 | The allow's *own presence* is the marker that the crate is still an instrument. |
| The allow's own comment says so, and names the phase that removes it: "the exception is scoped here rather than left open in the workspace manifest so that it is visible in review and disappears with the last `todo!()` rather than outliving it. Phase 8 removes both the bodies and this line." | `crates/happenstance-sqlite/src/lib.rs:76-80` | This story is that phase. The instruction is already written in the code it deletes. |
| `todo = "deny"` workspace-wide, and the workspace comment records why: the allow "was `allow`, workspace-wide, for stub crates… Denied now, because a grep found no `todo!()` anywhere in the workspace — the allow protected nothing and would have silently accepted the first real one." | `Cargo.toml:123-128` | The mechanism is clippy plus `-D warnings`, and nothing else. There is no `todo!()` grep anywhere in `xtask`. |
| The five file-reading lints are `no_clock` (CF-33), `testkit_version` (CF-32), `core_alloc_features`, `changelog_names_every_rule` (CF-29) and `no_position_literals` (CF-6). None of them reads `crates/happenstance-sqlite/src/`. | `xtask/src/lints.rs:231`, `:289`, `:355`, `:525`, `:628` | So "no `todo!()` on any SQLite path" is enforced by exactly one thing: `clippy::todo` with the scoped allow removed. Remove the allow and leave the `todo!()` and clippy fires; remove neither and nothing fires. |
| The crate's status banner is published documentation and is now false: "Every operation that touches SQL is `todo!()` — migration, `append`, the page query, `checkpoint` and `commit`", "It is `publish = false` until it passes… the conformance suite", plus the *Open decisions* section listing the append-condition strategy and tag storage that ADR-0022 settled. | `crates/happenstance-sqlite/src/lib.rs:1-73` | The whole module doc is an instrument marker, not just the `#![allow]` line. A green gate says nothing about it — `cargo xtask ci --fast` builds docs, it does not read them for truth. |
| The schema sketch at `event_store.rs:36-54` is known to be wrong in a way that serialises every writer, and "the module doc is *published documentation*; correcting it is part of this work, not a follow-up." | `_decomposition.md`, *Architecture brief*, §7; `RUNBOOK.md:4178-4187` | If migration 1 corrected the schema but not the sketch, the crate ships a correct implementation documented as the wrong one. |
| `cargo xtask ci --fast` is `REQUIRED` without `OPTIONAL`: it drops the two feature powersets, `cargo deny` and the nightly `--cfg docsrs` build, and keeps everything else including all four `wasm32` steps. | `.redkiln/config.yaml:50-55`; `CLAUDE.md`, *Commands* | The four wasm32 steps are kept, so DR-08's re-check of the `bench` feature's target gating happens here — but the wasm32 **feature powerset** does not run, which is the gap `benchmark-harness` was written against. |
| DR-08 is "discharged inside `benchmark-harness` … and re-checked by `instrument-markers-removed-and-gate-green`'s `--fast` gate." | `_storymap.md`, *Coverage* | Re-checked, not discharged. This story must not be relied on to catch a target-gating mistake the powerset would find. |
| The whole gate on the assembled library — full `cargo xtask ci`, feature powersets, `cargo deny`, docsrs, MSRV — belongs to `closeout-and-durable-audience` (HS-P0019). | `project.md`, *Out of scope*; `.redkiln/config.yaml:56-60` | This story's bar stops at `--fast`, deliberately, and saying so keeps the ledger honest about what green means. |

## Questions

Open questions to resolve before specifying.

1. **Is the whole module-doc rewrite in this story or spread across the
   implementation stories?** Answered: the *correction of the schema sketch* is
   `schema-migration-and-identity`'s, because it is wrong about what that story
   builds; the **status banner, the `publish = false` sentence and the *Open
   decisions* section** are this story's, because they are wrong only once the
   last `todo!()` is gone. The spec should verify the first happened rather than
   assume it — a doc correction is exactly the kind of item a green run absorbs.
2. **Does `cargo xtask ci --fast` prove anything about `todo!()`?** Answered: only
   through `clippy::todo` and only once the scoped allow is deleted. There is no
   grep-based lint, and adding one is not proposed here — `clippy::todo` is the
   right mechanism and the workspace comment at `Cargo.toml:123-128` explains why
   the *scoped allow* is the visible artefact.
3. **What about the `todo!()`s in the crate that are not on a SQLite path?**
   Answered: there are none by the time this story runs — `begin` and `rollback`
   already have real bodies (`crates/happenstance-sqlite/src/lib.rs:5-13`) and the
   remaining bodies are all SQL. If any survives, the allow cannot be deleted, and
   that is the correct outcome rather than a reason to scope the allow more
   narrowly.
4. **Does this story touch `publish = false`?** Answered: **no**. That belongs to
   `crates-io-name-and-packaging-facts` (HS-S0046) and, ultimately, to
   `publication-and-positioning`. `RUNBOOK.md:4230`'s exit checkbox says otherwise
   and is stale; AC-015 and the initiative decomposition are the current authority.
5. **Does the `--fast` gate suffice as AC-014's evidence?** Answered: yes for
   AC-014 as written, and the ledger must say what it does **not** cover — the two
   feature powersets, `cargo deny`, docsrs and MSRV are `closeout-and-durable-audience`'s.
   A green `--fast` presented as a green gate is the misreading to avoid.
6. **The append-condition SQL strategy.** Settled by ADR-0022 and, by this story,
   also *removed from the crate's "Open decisions" prose*, which is a
   documentation change rather than a decision. If the ADR left anything genuinely
   open, the prose says so and cites the open question rather than deleting the
   section wholesale.

## Decision

The crate stops being an instrument in one commit or it does not stop being one at
all: the scoped `#![allow(clippy::todo)]` is the *only* thing standing between a
`todo!()` on a SQLite path and a denied lint, so deleting the allow while a
`todo!()` survives fails the gate, and deleting the last `todo!()` while the allow
survives leaves a permanent marker asserting the opposite of what is true. This
slice does both together and takes `cargo xtask ci --fast` green. The spec will
cover: the last SQL body replaced and `crates/happenstance-sqlite/src/lib.rs:76-80`
deleted in the same change (DR-01); the crate's published module doc rewritten —
the "Status: an instrument, not yet an adapter" banner, the "`publish = false`
until it passes the conformance suite" sentence, and the *Open decisions* section
ADR-0022 closed — with the schema sketch's correction verified rather than assumed;
and `cargo xtask ci --fast` green end to end, recorded with an explicit note of
what `--fast` does not run. This story adds no conformance rule, so the
literal-position bar is vacuous, and it amends no `[FROZEN]` clause; `publish =
false` and `PUBLISHABLE` are untouched.

## The wrong implementation

**The mutant: `todo!()` replaced by `unimplemented!()`.** It is one word, it reads
as more honest ("this is genuinely not implemented" rather than "I will get to
it"), and it makes the scoped allow deletable immediately.

Every check passes. `clippy::todo` is satisfied — the macro is gone.
`clippy::unimplemented` is **not** in the workspace lint table at all: `Cargo.toml`
denies `unwrap_used` and `todo` and says nothing about `unimplemented`
(`Cargo.toml:118-128`), so `-D warnings` has nothing to fire on. None of the five
file-reading lints reads `crates/happenstance-sqlite/src/` — `no_clock` is scoped
to the testkit's `src/` (`xtask/src/lints.rs:33-41`) and `no_position_literals` to
rule files. `cargo xtask ci --fast` is green. The `#![allow(clippy::todo)]` line is
gone, so AC-014's second clause literally reads as satisfied, and a `rg 'todo!'`
over the crate — the testing brief's own proposed check
(`_decomposition.md`, *Testing brief*, §2, AC-014 row) — returns nothing. The crate
ships as an adapter and panics on the first call to whatever body was left behind.
The same mutant wears other clothes: `unreachable!()`, `panic!("not implemented")`,
or a body returning `Err(SqliteEventStoreError::…)` fabricated to stand in for
work not done.

The control is not a new lint; it is the **conformance run**, which is why AC-014
depends on three stories that each take a suite green rather than on a grep. A
body replaced by `unimplemented!()` fails whichever rule reaches it, loudly. The
spec's obligation is therefore to state that AC-014's evidence is *the green
suites plus the deleted allow*, never the grep alone — the grep is a convenience
that this mutant defeats in one character.

**The second mutant, and it is the one that survives every possible green run: the
crate's module documentation left as written.** `crates/happenstance-sqlite/src/lib.rs:1-24`
tells every reader on docs.rs that "Every operation that touches SQL is `todo!()`"
and that the crate is "`publish = false` until it passes the conformance suite";
`event_store.rs:36-54` shows a schema that `RUNBOOK.md:4178-4187` establishes will
serialise every writer; the *Open decisions* section still lists the
append-condition strategy and tag storage that ADR-0022 settled. `cargo xtask ci
--fast` builds the docs and checks their intra-doc links — it does not read them
for truth, and no lint anywhere in this repository does. So the crate can be a
correct, conformant, published-ready adapter whose own front page describes the
skeleton it used to be, and the next implementer who reads the schema sketch
implements the version that holds a join under the write lock. This is the second
place in the project where documentation is load-bearing and unenforced (the first
being ADR-0022's substrate), and the control in both cases is a named review item
in the ledger rather than a machine.

**The third mutant, quiet and structural: deleting the last `todo!()` in one
commit and the allow in the next.** Both commits are green in isolation, the story
closes with both changes present, and DR-01's actual requirement — that the allow
cannot outlive the bodies — is satisfied by coincidence rather than by
construction. It matters because the allow's whole design rationale is that it is
"visible in review" (`Cargo.toml:126-127`), and a two-commit split is exactly the
shape in which the second commit does not happen. The control is
`require_commit_provenance: true` (`.redkiln/config.yaml:68-73`) plus a review
reading of the diff, not a test.

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
