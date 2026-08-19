---
item: HS-S0046
stage: discover
created: 2026-08-12T13:02:08.621Z
updated: 2026-08-12T13:02:08.621Z
template_sig: 86ce4036
rendered_sig: b1678dba
---

# Discover — The name reserved and the manifest publishable, with publishing left to someone else

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-line: reserve `happenstance-sqlite` on crates.io and give the crate a real description, README and both licence files so `cargo package --list --allow-dirty` shows them — while `publish = false` and `PUBLISHABLE` stay untouched. | `_storymap.md`, *Slices* table, row `publishable-and-reconciled` / `crates-io-name-and-packaging-facts` | Two things that look like one: a **reservation** (a separate placeholder crate) and **packaging facts** (files in this crate's directory). They are unrelated artefacts with a shared motive. |
| **AC-015** — the crate is publishable and publishing stays someone else's decision: the name reserved, and the manifest carrying a description, README and both licence files "such that `CLAUDE.md`'s `cargo package --list` assertion **would** hold if `publication-and-positioning` adds it to the publishable set. Whether it does is that project's." | `project.md`, AC-015 | The subjunctive is the whole criterion. This story makes a conditional true; it does not make the condition true. |
| `dependsOn: instrument-markers-removed-and-gate-green` (HS-S0045) — the README cannot state the adapter's real durability settings and enforced ceiling until they exist and the crate is no longer a skeleton. | `_storymap.md`, *Merge order* item 5 | The merge order also notes the *name reservation* half "has no code dependency and should be done at the project's start per `RUNBOOK.md:4191-4193`". Two halves, two timings. |
| `cargo xtask reserve <name>` already exists and already knows this crate: `Reservable { name: "happenstance-sqlite", description: "SQLite event store and projection store adapters for happenstance.", blurb: …, phase: 8 }`. | `xtask/src/reserve.rs:86-91`; `xtask/src/main.rs:670` | The reservation is a **command run**, not a procedure to invent. Its `description` is also the text this crate's manifest should converge on. |
| Why the command exists at all: "Names are reserved one at a time, as each phase starts… A procedure run that rarely, from memory, is a procedure that drifts: the licence files get forgotten once, and the crate that carries the project's first impression ships without them, permanently, because a version can be yanked and never removed." | `xtask/src/reserve.rs:1-10` | The failure mode this story is guarding against is already documented, by name, in the tool that prevents it. |
| The placeholder is standalone rather than the real crate, for two stated reasons — `0.0.0` cascades through workspace dependency requirements resolved against crates.io, and publishing at the real version "makes the API semver-binding, and the whole plan… is built around freezing the contract *deliberately*". | `xtask/src/reserve.rs:12-30` | Reserving must not be done by publishing this crate at any version. That is the pre-refuted shortcut. |
| `publish = false` **stays**. `RUNBOOK.md:4230`'s exit checkbox reads "publish = false removed" and is **stale**; AC-015 and `.bklg/from-contract-to-published-library/_decomposition.md:43` give that decision to `publication-and-positioning`. | `_decomposition.md`, *Architecture brief*, §8; `_grounding.md`, *Tensions*; `crates/happenstance-sqlite/Cargo.toml:12` | "Record the runbook checkbox as superseded by the AC; do not silently diverge from either." |
| `reconcile()` fails loudly in **both** directions, and its message names the remedy: "Cargo will publish {…} but this step does not check it. If the crate was promoted on purpose, copy {LICENSE-MIT, LICENSE-APACHE, README.md} into its directory and add its name to PUBLISHABLE…". | `xtask/src/package.rs:172`, `:195-208` | Deleting `publish = false` here fails CI the moment the crate is promoted while `PUBLISHABLE` has not been updated — the "promoted-but-unreconciled" failure the module's own doc names. |
| `PUBLISHABLE` is exactly `["happenstance-core", "happenstance", "happenstance-testkit"]`; `REQUIRED_FILES` is `["LICENSE-MIT", "LICENSE-APACHE", "README.md"]`. AC-A05 requires both unchanged at the end of this project. | `xtask/src/package.rs:86`, `:94`; `_decomposition.md`, *Architecture brief*, AC-A05 | Adding this crate to `PUBLISHABLE` would be this project deciding that the crate publishes. |
| The packaging module names **this crate** as the wrong implementation it exists to reject: "`cargo package -p happenstance-sqlite --list` exits 0, lists seven files, and none of them is one of the three" — true today; `crates/happenstance-sqlite/` has no `README.md` and no `LICENSE-*`. | `xtask/src/package.rs:20-22`; `_grounding.md` | The named wrong implementation for AC-015 is the current state of the tree. |
| **Consequence to carry into the ledger:** with `publish = false` standing, the packaging gate **does not** check this crate. AC-015's evidence is a recorded `cargo package -p happenstance-sqlite --list --allow-dirty` run showing the three files, not a green gate step. | `_decomposition.md`, *Architecture brief*, §8; *Testing brief*, §2, AC-015 row | The gate step is `publication-and-positioning`'s to earn. Presenting a green `--fast` as evidence here would be evidence for the wrong proposition. |
| The stale `description = "… Not yet implemented."` is this project's to replace; `publish` is not. | `crates/happenstance-sqlite/Cargo.toml:3`; `_decomposition.md`, *Architecture brief*, §8 | And the replacement text already exists, in `reserve.rs:88`, so the placeholder and the eventual release describe the same thing. |
| The README must state the adapter's real durability settings and enforced ceiling — the `synchronous` value and journal mode are "a documented property of the adapter, in ADR-0022 and in the crate's README", and the payload ceiling is "a **fact about this adapter** that must be documented in the README and enforced in `append`". | `_decomposition.md`, *Architecture brief*, §6 and §11 | The README is not boilerplate; two conformance-relevant facts have no other public home. |

## Questions

Open questions to resolve before specifying.

1. **Is the reservation run now or at the project's start?** Answered: **at the
   project's start**, per `RUNBOOK.md:4191-4193` ("a name is reserved when its
   phase starts") and the merge-order note that the reservation half has no code
   dependency. The *packaging facts* half is what waits on HS-S0045. The story
   carries both, and the spec must not let the code dependency delay the
   reservation.
2. **Are the licence files copied or symlinked?** Deferred to `spec`, with the
   constraint fixed: `cargo package --list` must show `LICENSE-MIT`,
   `LICENSE-APACHE` and `README.md` **inside this crate's package**
   (`xtask/src/package.rs:94`, `:131-141`). Whatever mechanism produces that is
   acceptable; whatever does not, is not — and the check is running the command,
   not reasoning about it.
3. **Does `publish = false` change?** Answered: **no**, and the reasoning is
   recorded rather than assumed, because the runbook's own exit checkbox says the
   opposite. The spec states `RUNBOOK.md:4230` as superseded by AC-015 and cites
   `_decomposition.md:43` for who owns the decision instead.
4. **Does `PUBLISHABLE` change?** Answered: **no** (AC-A05). Adding the crate
   would silently transfer `publication-and-positioning`'s decision into this
   project, and `reconcile()` would then be satisfied by a claim nobody made.
5. **What does the README say about durability?** Deferred to `spec` as text,
   fixed here as content: the journal mode, the `synchronous` setting, the busy
   timeout and the three enforced ceilings — the facts CF-14 and CF-40 make
   publicly relevant and which exist in no other user-visible place.
6. **Does this story publish anything?** Answered: it publishes the **placeholder**
   `0.0.0` crate that `cargo xtask reserve` generates, and nothing else. It does
   not publish `happenstance-sqlite` itself at any version — `reserve.rs:12-30`
   pre-refutes both spellings of that shortcut.
7. **The append-condition SQL strategy.** Not touched. It is *documented* in the
   README as a consequence of ADR-0022, not re-decided.

## Decision

A name on crates.io is claimed once and a first impression is shipped once — a
version can be yanked and never removed — so the two irreversible packaging acts
in this project are grouped into one slice and separated from the decision about
whether the crate publishes at all, which belongs to
`publication-and-positioning`. The spec will cover: running
`cargo xtask reserve happenstance-sqlite` to generate and publish the standalone
`0.0.0` placeholder the tool already describes at `xtask/src/reserve.rs:86-91`;
adding `README.md`, `LICENSE-MIT` and `LICENSE-APACHE` to
`crates/happenstance-sqlite/`, with the README stating the adapter's real journal
mode, `synchronous` setting, busy timeout and three enforced ceilings; replacing
the stale `description = "… Not yet implemented."` with the text the placeholder
already uses; recording a `cargo package -p happenstance-sqlite --list
--allow-dirty` run showing the three files as AC-015's evidence, with an explicit
note that the packaging **gate** does not check this crate while `publish = false`
stands; and leaving `publish = false` (`Cargo.toml:12`) and `PUBLISHABLE`
(`xtask/src/package.rs:86`) untouched, with `RUNBOOK.md:4230`'s contrary checkbox
recorded as superseded rather than silently ignored. This story adds no
conformance rule, so the literal-position bar is vacuous; it amends no `[FROZEN]`
clause.

## The wrong implementation

**The mutant: deleting `publish = false` from
`crates/happenstance-sqlite/Cargo.toml:12`.** It is not a slip — it is what
`RUNBOOK.md:4230`, the phase's own written exit criterion, instructs: "`[ ] publish
= false removed`". An implementer working from the runbook does this on purpose.

And on the day it lands it is **green**, which is the trap. `cargo xtask ci
--fast` runs the packaging step, `reconcile()` derives the publishable set from
`cargo metadata`, sees `happenstance-sqlite` newly present, sees it absent from
`PUBLISHABLE`, and fails — so actually this one is caught *loudly*, immediately,
with a message that names the remedy: "Cargo will publish happenstance-sqlite but
this step does not check it… add its name to PUBLISHABLE"
(`xtask/src/package.rs:197-203`). The dangerous move is the **fix the message
suggests**: adding `happenstance-sqlite` to `PUBLISHABLE`. That is green forever,
satisfies every check, produces a README and two licence files, and quietly
transfers a decision this project does not own — whether adapter crates are
published at all — from `publication-and-positioning` into a packaging tweak
(AC-A05; `.bklg/from-contract-to-published-library/_decomposition.md:43`). The
error message is correct for the case it was written for and wrong for this one,
and nothing distinguishes them mechanically. The control is this story's spec
saying `publish` and `PUBLISHABLE` are both out of scope, and the ledger citing
`RUNBOOK.md:4230` as superseded rather than as followed.

**The second mutant, and it is the one that satisfies every check the repository
can run: a README that exists.** `xtask/src/package.rs` asserts the three files by
**name** — `REQUIRED_FILES` is a list of filenames, and `reconcile()` checks
presence in the `cargo package --list` output (`:94`, `:131-141`). A one-line
`# happenstance-sqlite` README passes it exactly as well as a real one, and this
crate is not even checked while `publish = false` stands. What is lost is
specific rather than aesthetic: the `synchronous` setting, the journal mode, the
busy timeout and the three enforced ceilings have **no other user-visible home**.
CF-14 names `PRAGMA synchronous = OFF` as a wrong implementation the reopen rule
exists to reject (`spec/SPECIFICATION.md:7471-7490`), and CF-40 makes a store's
ceilings a stated fact rather than a trade — but a consumer reading docs.rs cannot
see a `Fixture` associated constant. An adapter that enforces a 1 MiB payload
ceiling and documents nothing is, from the caller's side, indistinguishable from
one with no ceiling until an append fails in production. The control is a named
review item — the README states all four facts — because no gate can assert the
*content* of a file whose presence is all it checks.

**The third mutant: reserving the name by publishing this crate rather than the
placeholder.** At `0.0.0` it cascades — `happenstance-testkit` depends on
`happenstance-core` at the workspace version, and a registry publish resolves those
requirements against crates.io rather than against the path. At the real version it
is worse: `0.1.0` makes the API semver-binding, and "the whole plan in `RUNBOOK.md`
is built around freezing the contract *deliberately*, at phases 4 through 6,
against evidence. Reserving a name is not a reason to freeze an API, and a
reservation that quietly does so has cost more than it bought"
(`xtask/src/reserve.rs:12-30`). Both spellings are already refuted in the tool's
own documentation, which is why the control is "run `cargo xtask reserve`" rather
than "reserve the name" — the command is the argument, encoded.

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
