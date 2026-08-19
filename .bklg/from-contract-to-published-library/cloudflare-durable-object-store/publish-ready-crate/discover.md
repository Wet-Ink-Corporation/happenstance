---
item: HS-S0059
stage: discover
created: 2026-08-12T13:02:21.106Z
updated: 2026-08-12T13:02:21.106Z
template_sig: 86ce4036
rendered_sig: 5cc509fc
---

# Discover — happenstance-cloudflare becomes a crate someone can depend on

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| The slice's one-line: make the crate depend-on-able — `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` beside `Cargo.toml`, `publish = false` removed so `cargo xtask package-check` covers it, the name reserved per `xtask/src/reserve.rs`, and `cargo xtask ci` green including every `wasm32` step | `_storymap.md`, **Slices**, `publish-ready-crate` row | Four artefacts and one green gate. The story is small and is last for a reason |
| AC-012: publish-ready and the gate green; whether the crate is actually published stays with `publication-and-positioning` | `project.md`, **Acceptance criteria**, AC-012; **Out of scope** | *Publish-ready*, not published. The boundary is explicit and is HS-P0016's |
| `dependsOn: worker-binding-layer` (there is something real to depend on), `every-rule-under-workerd` (the compliance claim has evidence behind it), `measured-store-limits` (the capability statement is honest) | `_storymap.md`, **Slices**; **Merge order** §5 | Last so the final green gate is observed against a tree whose atoms are already accepted |
| ARCH-AC-08: the three files sit alongside `Cargo.toml` and `src/`, and `publish = false` is removed so `cargo xtask package-check` would hold for this crate | `_decomposition.md`, Architecture brief, **Acceptance Criteria**; `crates/happenstance-cloudflare/Cargo.toml:12` | Removing the flag is what brings the crate *into* a check it currently sits outside |
| The gate asserts, via `cargo package --list`, that each publishable crate carries both licence files and a README | `xtask/src/main.rs:750-752`, dispatch at `:680`; `CLAUDE.md`, **Commands** | The check exists and covers three crates today. This story makes it four |
| The crate directory holds only `Cargo.toml` and `src/` today, unlike `happenstance-core`, `happenstance` and `happenstance-testkit`, each of which carries all three files | `_grounding.md` §2 | Verified against the tree rather than assumed |
| Name reservation is a standalone `0.0.0` crate sharing only metadata, never the real crate at a real version — publishing the real crate early would either cascade version pins through every dependent manifest or freeze the API before the contract-freeze phases | `xtask/src/reserve.rs` module docs; `_decomposition.md`, Deployment brief §5 | The mechanism is documented and the reasoning for it is not this story's to relitigate |
| A name is claimed when its phase starts, and `happenstance-cloudflare` was verified free on 2026-08-06 | `RUNBOOK.md:4254-4261`, `:4301`; the phase-9 crate note | The reservation is independent of the crate's content and need not wait on this story |
| The crate's own rustdoc opens with `# Status: not implemented` and "every body is `todo!()`" | `crates/happenstance-cloudflare/src/lib.rs:4-10` | That header is what docs.rs renders. Rewriting it is part of publish-ready, not cosmetics |
| The scoped `#![allow(clippy::todo)]` is documented to disappear with the last `todo!()` rather than outlive it, and DoD 3 is a grep for both | `crates/happenstance-cloudflare/src/lib.rs:121-126`; `project.md`, **Definition of done** item 3 | If it is still there at this story, an earlier story did not finish |
| `cargo xtask ci` currently runs four `wasm32` steps; AC-012's claim is only meaningful once the fifth, from `wasm-execution-seam`, is in the array too | `xtask/src/main.rs:192-283`; `_storymap.md`, **Merge order** §5 | The green-gate claim is scoped to a tree that includes the execution step |
| Publishing, the registry landing page, the MSRV promise, the semver diff and the clause-ledger audit all belong to `publication-and-positioning` (HS-P0016) | `project.md`, **Out of scope**; `_decomposition.md`, Deployment brief §5 | Positioning is theirs; accuracy of what is written here is this story's |
| The initiative's AC-08 requires the published compliance claim to name which implementations it was checked against, and this project is a blocker of `0.2.0` for that reason | `project.md`, **How this advances the initiative**; **Dependencies**, *Unlocks* | The README's claim, if it makes one, inherits that requirement |
| The rustdoc obligations and prime directives live in the Rust constitution rather than being restated per task | `standards/rust/70-rustdoc-obligations.md`, `standards/rust/00-prime-directives.md`, via `CLAUDE.md`, **House style** | The one lever that makes a documentation claim mechanically checkable is a doctest |

## Questions

**Is the name reserved in this story or at project start?** Answered:
`RUNBOOK.md` says claim the name when the phase starts, and the reservation is a
standalone `0.0.0` metadata-only crate with no dependency on this crate's content
(`xtask/src/reserve.rs`). So it can and should happen on day one; this story owns
the *removal* of `publish = false` and the three files and must not be blocked on a
registry round trip. The sequencing detail is confirmed at **spec**.

**Does removing `publish = false` change what the gate checks?** Answered: yes, and
that is the point. `cargo xtask package-check` asserts both licence files and a
README are inside the packaged artifact for each publishable crate
(`xtask/src/main.rs:750-752`), and this crate is currently outside that set. The
three files and the flag removal therefore land in one change, or the gate goes red
on the crate's first step into the set.

**Does the README make a compliance claim, and how narrow must it be?** Answered in
principle, settled at **spec**: it may, and if it does it names what the claim was
checked against — the initiative's AC-08 requires the published claim to name its
implementations, and this adapter's claim has real qualifiers, notably the
concurrency family's non-invocation and whatever capabilities the fixture declined.
The *positioning* of that claim across the workspace is HS-P0016's; its *accuracy*
here is this story's.

**MSRV promise, semver diff, registry landing page.** Explicitly HS-P0016's
(`project.md`, **Out of scope**). Named here so they are not drafted in passing
into a README.

**Which `wasm32` steps must be green?** Answered: all of them, including the
execution step this project adds — AC-012's wording predates that step and the
higher bar wins, as the story map says in its merge-order note.

**CF-39 / CF-40's fixture-limits ownership, and the off-tokio harness shape.**
Neither is this story's; both are settled upstream and this story only inherits
their results into what the crate says about itself.

## Decision

`happenstance-cloudflare` currently cannot be depended on and cannot even be
checked as though it could: `publish = false` keeps it outside `cargo xtask
package-check`, its directory carries neither licence file nor a README, and its
front-page rustdoc opens with `# Status: not implemented`. This slice closes that
gap — the two licence files, a README, the flag removed so the packaging assertion
starts covering this crate, the name reserved through the documented `0.0.0`
mechanism, and a green `cargo xtask ci` including the wasm32 execution step this
project added. It stops firmly short of publishing: the version, the registry
landing page, the MSRV promise and the semver diff are `publication-and-positioning`'s.
Its one substantive risk is not mechanical but editorial — this is the first
surface a consumer meets, and it is the only artefact in the project that no test
reads. The spec will cover: the two licence files and the README's contents; the
rewritten crate-level rustdoc replacing the not-implemented header; the exact
capability qualifiers the compliance claim carries and where each is sourced from
in the run's own output; the flag removal and the `package-check` consequence; the
reservation and its timing; and the confirmation that a `todo!()` grep and an
`#![allow(clippy::todo)]` grep both come back empty. Nothing `[FROZEN]` is amended.

## The wrong implementation

**The crate that is publishable and lies.** `publish = false` removed, both licence
files added, `README.md` added, `cargo xtask package-check` green,
`cargo xtask ci` green — and the front page still opens with "Status: not
implemented … every body is a `todo!()`"
(`crates/happenstance-cloudflare/src/lib.rs:4-10`), because that text lives in the
crate rustdoc rather than in any file the packaging assertion names. Or the inverse and more damaging version: a README that says
"passes the DCB conformance suite" flat, on a crate whose concurrency family is not
invoked at all, whose fixture may decline `REOPEN`, and whose positions are bounded
by 2^53 rather than 2^64. `cargo package --list` asserts three *filenames*
(`xtask/src/main.rs:750-752`); nothing in the gate reads a word of what is inside
them. Every check in the tree is green and the first thing a consumer meets is
false — the same defect class as a decorative gate, relocated to the one surface a
library user actually sees, and the one the initiative's AC-08 exists to constrain.

**Where the detector lives, and the honest limits of it.** Not in
`crates/happenstance-testkit/tests/mutation_coverage.rs` and not as a conformance
rule: no store fails anything here, and a rule asserting on prose would be exactly
the decorative rule `CLAUDE.md`'s corollary forbids. Two mechanisms are available
and both should be used. First, the README's usage example is written as a
**doctest** rather than as a fenced block, so a claim that stops compiling stops the
gate — which is the whole argument the Rust constitution's rustdoc obligations make
(`standards/rust/70-rustdoc-obligations.md`), and the reason the documentation step
is in the gate at all. Second, the capability statement is written in **the same
words the run prints** — the fixture's own declined-capability reasons and the
stated non-invocation of the concurrency family — so it can be diffed against the
conformance run's output rather than trusted. What neither mechanism can catch is a
true sentence that is narrower than it sounds; that is a review obligation, and the
standing requirement it answers to is the initiative's AC-08: the published claim
names which implementations it was checked against.

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

**Box 6.** No conformance rule is added here. The only executable artefact this
story adds is a README doctest, and it demonstrates construction and use rather
than asserting on any position value — which matters, because a doctest that
printed `[1, 2, 3]` as expected output would be publishing the literal-position
assumption `CLAUDE.md` forbids, on the most-read page the crate has.

**Box 7.** Nothing `[FROZEN]` is touched. This story adds files, removes a
manifest flag and rewrites documentation; it changes no clause, no marker and no
behaviour. The MSRV promise — the one thing here that *would* turn a provisional
marker into a commitment — is explicitly `publication-and-positioning`'s and is not
made in this project.

**Box 8.** No conformance rule here seems wrong. The one check that looks
stronger than it is — `cargo package --list` asserting three filenames while
reading none of their contents — is correctly scoped for what it guards, and this
story's answer is to add a doctest and a diffable capability statement rather than
to ask a packaging assertion to review prose.
