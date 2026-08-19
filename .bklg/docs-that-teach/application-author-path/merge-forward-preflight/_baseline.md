---
item: "HS-S0183"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Baseline record — the merged tree the four page stories are authored against

This is the artifact `spec.md` § Acceptance criteria fixes at this path, with the five
headings it names. It exists so that no page author in this project re-runs the archaeology
below, and so that a red gate met by the next implementer is unambiguously **their own page's
doing**.

Read it as a set of measurements, not as an argument. Every number here was taken by running
a command on the merged tree in this session; the pre-merge figures beside them are quoted
from `_design.md` and `project.md` to be **compared against**, never copied forward. Where a
measurement contradicts a binding statement in the signed-off design, it is recorded with a
disposition and routed — `_design.md` is not edited here, and its sign-off row is a human's
(`project.md:300-302`).

One thing this record deliberately does **not** do: re-decide anything. DT-1, DT-4, DT-5 and
DT-6 are certified by the slice-mate, `tension-resolutions`, against these numbers.

## § Merge

The merge is real, it is a single revertible commit, and it is observable in the working tree
rather than only in the reflog.

| fact | value | how it was re-derived |
| --- | --- | --- |
| merge commit | `a5c0f300f78b9f0289bc7d1575ce35b47dcd44f5` (`a5c0f30`) | `git rev-list --parents -n 1 a5c0f30` |
| first parent — this branch's pre-merge tip | `3fd3866e9264ed67aae821730d430f817f6ccd3c` (`3fd3866`) | same command, second field |
| second parent — the sibling tip actually merged | `4327ce33c2049de6393926946491bc4563248001` (`4327ce3`) | same command, third field |
| ancestry, not a diffstat | `initiative/from-contract-to-published-library` is an ancestor of `HEAD`; exit 0 | `git merge-base --is-ancestor initiative/from-contract-to-published-library HEAD` |
| `crates/happenstance/src/lib.rs` | 237 lines (75 before the merge) | `wc -l crates/happenstance/src/lib.rs` |
| `Tags::empty()` sites — the initiative's headline evidence | lines 38 and 55, both present | `rg -n "Tags::empty\(\)" crates/happenstance/src/lib.rs` |
| rendered selector `#main-content details.top-doc > div.docblock` | present, and composed as the design describes: `<section id="main-content" class="content" tabindex="-1">` contains `<details class="toggle top-doc" open>`, whose first element child after `<summary class="hideme">` is `<div class="docblock">` | `cargo doc -p happenstance --no-deps`, then read out of `target/doc/happenstance/index.html` |
| non-occlusion of this branch's own work | nothing under `.bklg/docs-that-teach/**`, `.redkiln/telemetry/events/**` or `references/seeds/user-documentation.md` was deleted, renamed or modified **by the merge** | `git diff --name-status --diff-filter=DRM 3fd3866 a5c0f30 -- .bklg/docs-that-teach .redkiln/telemetry/events references/seeds/user-documentation.md` → empty |
| files the merge itself resolved | seven, listed in `spec.md` § PR boundary | `git diff-tree --cc --name-only a5c0f30` |

**EC-004 fired, and this is the record of it.** `spec.md` states every pre-merge measurement
against sibling tip `3f49ec6`. The tip actually merged is `4327ce3`. Every anchor and every
disposition below was therefore re-taken against `4327ce3`, and the line numbers `spec.md`
*predicted* for the merged tree (ES-25 at `:3698`, VT-30 at `:1816`, `const REQUIRED` at
`:117`, the `"tests"` step at `:155`) are all wrong by the width of that gap. They are
superseded by § Anchors below, which is the point of having a table rather than a paragraph.

**Reversibility.** One merge commit, first parent this branch's pre-merge tip, so the whole
baseline backs out with `git revert -m 1 a5c0f30` rather than by unpicking the second parent's
commits one at a time.

## § Gate

Recorded against a sha, per NF-001. "The gate was green" without a commit is unfalsifiable,
and this story's entire value to the next four is that it is falsifiable.

| command | run at | result |
| --- | --- | --- |
| `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, the non-terminal project bar) | `HEAD`, whose source tree is byte-identical to merge commit `a5c0f30` — see the note below | **green** |
| `cargo xtask affected --base main` (`.redkiln/config.yaml:41`, the story grain) | this story's checkpoint | **green** |
| `cargo xtask spec-trace` (REQUIRED step) | `HEAD` = `a5c0f30`'s source tree | **green** — 201 clauses, 112 rules, 401 citations checked, "no problems found" |
| `cargo xtask lints` (`reachability_static`, with `spec-trace`) | `HEAD` = `a5c0f30`'s source tree | **green** — 27 constitution atoms, 2 narrative pages, 2 page-need checks, all consistent |
| `git revert -m 1 a5c0f30` | not run; recorded as the one-move backout the parent order buys | n/a |

**Why "`HEAD`, whose source tree is identical to `a5c0f30`" is the honest phrasing.** Two
commits land on the branch after the merge (`6d44316`, `7c62e4d`) and both touch `.bklg/` only.
Verified, rather than assumed:

```console
$ git diff --stat a5c0f30 HEAD -- . ':!.bklg' ':!.redkiln'
$                                    # empty: no source file differs
```

So a gate run at `HEAD` is a gate run on the merge commit's source, and recording it that way
is stronger than re-checking-out a detached `a5c0f30` and running it there — it is the tree the
next implementer actually inherits.

**EC-002 did not fire.** No step of `--fast` was red on this tree. The one red thing the merge
*did* produce — `crates/happenstance/tests/doc_budget.rs`, whose CRLF-vs-LF failure is a latent
defect on the sibling branch that a fresh Windows clone reproduces — was resolved in the merge
commit itself and is recorded in `spec.md` § PR boundary.

## § Anchors

Id first, line second, and every row carries the command that re-derives it (NF-003). Clause
**ids** are stable and are never renumbered (`spec/SPECIFICATION.md:300`); clause **lines** are
not, which is the whole reason this table exists. Run any command in the last column and the
line it prints is the line in the second.

Every command below is `git grep`, deliberately and not by preference. `rg` is this
repository's convention in prose and **is not on this machine's `PATH`** — a re-derivation
command that only runs in one agent's sandbox is a claim, not a check. `git grep -nF` needs
nothing but the repository it is standing in, takes a fixed string rather than a pattern, and
prints `path:line:text`.

| anchor (id first) | merged location | pre-merge / corpus claim (stale) | re-derivation command |
| --- | --- | --- | --- |
| ES-25 — condition semantics, `[FROZEN]`, the taught refusal | `spec/SPECIFICATION.md:3756` | `:3693` (`_design.md:276`), `:3698` (`spec.md`) | `git grep -nF "#### ES-25" -- spec/SPECIFICATION.md` |
| ES-26 — `after` is exclusive, `from` is inclusive, `[FROZEN]` | `spec/SPECIFICATION.md:3814` | not cited by line in the corpus | `git grep -nF "#### ES-26" -- spec/SPECIFICATION.md` |
| ES-27 — a condition matches on tags, not only types, `[FROZEN]` | `spec/SPECIFICATION.md:3855` | not cited by line in the corpus | `git grep -nF "#### ES-27" -- spec/SPECIFICATION.md` |
| VT-30 — an `AppendCondition` is one or more guards, `[PROVISIONAL]` | `spec/SPECIFICATION.md:1861` | `:1813` (`_design.md:400`), `:1816` (`spec.md`) | `git grep -nF "#### VT-30" -- spec/SPECIFICATION.md` |
| CF-7 — the tag-join obligation, `[FROZEN]`; DT-6 ships its **mirror** | `spec/SPECIFICATION.md:7670` | `:7266` (`_design.md:219`) | `git grep -nF "**CF-7.** At least one rule MUST" -- spec/SPECIFICATION.md` |
| CF-7's cautionary sentence — "rejects every command touching any course" | `spec/SPECIFICATION.md:7690` | `:7286-7288` (`_design.md:222`) | `git grep -nF "rejects every command touching any course" -- spec/SPECIFICATION.md` |
| the clause-id stability sentence — the sentence the whole anchor strategy rests on | `spec/SPECIFICATION.md:300` | `:280` (`spec.md`, `_design.md`) | `git grep -nF "Clause IDs are stable and are never renumbered" -- spec/SPECIFICATION.md` |
| `const REQUIRED` — the gate, defined once | `xtask/src/main.rs:120` | `:105` pre-merge, `:117` predicted (`spec.md`) | `git grep -nF "const REQUIRED: &[Step]" -- xtask/src/main.rs` |
| the `"tests"` REQUIRED step — the step tier 3 rides | `xtask/src/main.rs:158` | `:143` pre-merge, `:155` predicted (`spec.md`) | `git grep -nF 'name: "tests",' -- xtask/src/main.rs` |
| the `"documentation"` REQUIRED step — `RUSTDOCFLAGS=-D warnings` | `xtask/src/main.rs:344` | not cited by line in the corpus | `git grep -nF 'name: "documentation",' -- xtask/src/main.rs` |
| `crates/happenstance/Cargo.toml`'s `[dev-dependencies]` table | `crates/happenstance/Cargo.toml:45` | "declares **no** dev-dependencies at all today" (`_design.md:299-300`) | `git grep -nF "[dev-dependencies]" -- crates/happenstance/Cargo.toml` |
| the `tokio` dev-dependency, `macros` + `rt` + `rt-multi-thread` | `crates/happenstance/Cargo.toml:55` | recorded as `change: added` (`_design.md:284-288`) | `git grep -nF 'tokio = { workspace = true, features = ["macros", "rt", "rt-multi-thread"] }' -- crates/happenstance/Cargo.toml` |
| the worked example's module doc — last `//!` line | `examples/course-subscriptions/src/main.rs:28` | `:1-19` (`_design.md:499`, `project.md`, AC-010) | `git grep -nF '//! taken on it.' -- examples/course-subscriptions/src/main.rs` |
| the worked example's import of the taught crate | `examples/course-subscriptions/src/main.rs:34` | `use happenstance_core` at `:24-27` (`_design.md:499`) | `git grep -nF 'use happenstance::{' -- examples/course-subscriptions/src/main.rs` |
| the worked example's `trybuild` dev-dependency | `examples/course-subscriptions/Cargo.toml:34` | "no test target" (`project.md:75-77`) | `git grep -nF 'trybuild.workspace = true' -- examples/course-subscriptions/Cargo.toml` |
| the pinned narrative tree — `docs/`, included as doctests | `xtask/src/narrative.rs:124` | "the pinned narrative tree does not exist" (`_design.md:891-894`) | `git grep -nF 'include_str!("../../docs/append-conditions.md")' -- xtask/src/narrative.rs` |
| the answered-need notation, in its own standard | `standards/pages/00-one-need.md:62` | "HS-P0021's answered-need notation does not exist" (`_design.md:900-903`) | ``git grep -nF '> **Answers:** `token` — <question>?' -- standards/pages/00-one-need.md`` |

**EC-003 did not fire.** All five clause ids resolve on the merged text, with their maturity
markers intact: ES-25, ES-26, ES-27 and CF-7 `[FROZEN]`, VT-30 `[PROVISIONAL]`. Nothing was
repointed at a nearby clause; the ids are the same names they always were and only their
offsets moved.

## § Dispositions

Three fields each — what the design says, what the merged tree says, and one of `carried`,
`routed to <item>` or `fixed here` — because anything longer starts re-arguing a signed-off
decision. `_design.md` is **byte-identical** to its signed-off state
(`git diff --stat 3fd3866 HEAD -- .bklg/docs-that-teach/application-author-path/_design.md`
returns empty).

The four `spec.md` measured are rows 1–4. Rows 5–8 were found by taking the measurements this
story owes; project DoD item 9 forbids absorbing a finding silently, so they are recorded here
rather than left for a page author to meet mid-authoring.

| # | what the design says | what the merged tree says | disposition |
| --- | --- | --- | --- |
| 1 | `happenstance [dev-dependencies] tokio`, `change: added`, because "`crates/happenstance/Cargo.toml` declares **no** dev-dependencies at all today" (`_design.md:284-306`) | the table is at `Cargo.toml:45` and already carries `tokio` with `macros`, `rt`, `rt-multi-thread` (`:55`), beside `serde_json`, `futures-core` and a path-only `happenstance-testkit` | carried — the `## Items` row's status is `unchanged`, not `added`; the *reason* the design gave (a fence must **execute**, not merely type-check — AC-004, IQ-7) is untouched and still binding on `boundary-refusal-encounter` |
| 2 | the vocabulary-seam sentence exists because "the example imports `happenstance_core` directly (`examples/course-subscriptions/src/main.rs:24-27`) and that is correct for its own purpose" (`_design.md:497-502`) | the example imports `happenstance` at `:34`, and its manifest depends on `happenstance` with a comment recording the phase-7 move (ADR-0006). The sentence's premise is deleted by the merge | routed to `surface-course-subscriptions` (HS-S0188) — that story renders the handoff surface and owns the composition slot; anti-pattern 13's "one permitted appearance of the other crate's name" now has no occasion on that page, and deciding what replaces it is authoring, not preflight |
| 3 | AC-010 surfaces the worked example's module doc `:1-19`, by extraction into `overview.md` (`_design.md:728-741`) | the module doc runs `:1-28` and has gained a `# What is not in this file, and used to be` section | carried — the span moved, the mechanism did not; `surface-course-subscriptions` extracts `:1-28` and the extraction is still what makes "surfaced, not paraphrased" true by construction |
| 4 | the worked example carries "no test target" (`project.md:75-77`), which is why `cargo run -p course-subscriptions` proves nothing to the gate (`_decomposition.md:568-573`) | `examples/course-subscriptions/tests/runs.rs`, `tests/ui.rs` and a `tests/ui/` fixture directory exist, under a `trybuild` dev-dependency (`Cargo.toml:34`), still `publish = false` | carried — the premise of AC-005's drill improves rather than breaks: the example is now swept by the `"tests"` REQUIRED step (`xtask/src/main.rs:158`), so `boundary-falsification-drill` must build its check on the merged position and not on `project.md`'s |
| 5 | `## Composition` item 3 names `## Status: a facade over happenstance_core` as a heading to preserve and retitle (`_design.md:431-432`) | that heading does not exist on the merged crate root. ADR-0006's reasoning survives, moved inside `# What arrives here, and what stays below` (`crates/happenstance/src/lib.rs:72-77`) | routed to `boundary-refusal-encounter` (HS-S0185) — it owns the crate-root rewrite and the composition order; the design's *intent* (ADR-0006's reasoning is preserved verbatim, `.kb/governance/rewrite-the-referent-never-the-reasoning.md`) is met by the merged file already, and only the heading the design named is gone |
| 6 | four substrate gaps: the pinned narrative tree does not exist, the render shape is undecided, HS-P0021's answered-need notation does not exist, and the compiled-fence REQUIRED step does not exist (`_design.md:891-908`) | all four have landed on the sibling. The tree is `docs/` (`xtask/src/narrative.rs:124`); the render is markdown; the notation is `> **Answers:** \`<token>\` — <question>?` with its own standard (`standards/pages/00-one-need.md`); the compiled-fence step is `narrative-doctests` and the file-reading one is `narrative`, both in `REQUIRED` and both green | carried — this closes gaps 1–4 in this project's favour and makes `{tree}` resolvable to `docs/`. The consequence for the 22-character heading budget's *scope* is the slice-mate's to state, not this story's: `tension-resolutions` AC-004 owns the reconciliation against sign-off condition 2 |
| 7 | anti-pattern 2: "four exist on the crate root today; the target is zero on every page this project touches" (`_design.md:839-841`, gap 6 at `:916-919`) | **zero** literal `[bracket]` pairs render on the merged crate root. Both `happenstance_core` references now resolve to real intra-doc links (`<a href="../happenstance_core/index.html">`) | fixed here — by parentage, not by an authored edit. The number `boundary-refusal-encounter` must hold at zero is now a floor it inherits rather than a debt it repays, and § Composition baseline records it so the story is held against a real starting number |
| 8 | `spec.md` § PR boundary names six conflict-resolution files | `git diff-tree --cc --name-only a5c0f30` reports **seven**: the six named, plus `xtask/src/main.rs`, a genuine union resolution (both branches added a dispatch arm and a `lint_steps()` entry). The amendment's own prose already describes main.rs's conflict; the fence block omitted it | fixed here — the seventh path is added to `spec.md`'s fence with the same "one commit late" honesty the amendment already carries. Correcting a factual omission in a post-hoc record is not a scope decision: the file was resolved either way, and a list known to be incomplete is worse than one that admits its ordering |
| 9 | anti-pattern 2 is stated as a property of the page — "four exist on the crate root today" (`_design.md:839-841`) — and gap 6 records that `cargo doc` did not warn about them (`:916-919`) | the count is a property of the **invocation**, not of the page: 0 under `cargo doc -p happenstance --no-deps`, 2 under the gate's `documentation` step (`xtask/src/main.rs:344`), which exited 0 with `RUSTDOCFLAGS=-D warnings` set. Both measured in this session; the transcript is in § Composition baseline | routed to `boundary-refusal-encounter` (HS-S0185) for the page half — it must state which invocation its zero is measured under — and to the `support` initiative (`.redkiln/config.yaml:5`) for the gate half, that the `documentation` step renders two unresolved intra-doc links and exits 0 |

## § Composition baseline

Every number below was read off `target/doc/happenstance/index.html` after
`cargo doc -p happenstance --no-deps`, which `_design.md:1025-1027` names as the cheapest
instrument for exactly these measurements. The design's figures beside them were measured on a
75-line `lib.rs` that no longer exists.

The purpose of this section is **not** to relax a budget — the design gate rejected this
project once for that (`_design.md:1041-1048`). It is to hand `boundary-refusal-encounter` a
real starting number for each invariant it will be held to.

| invariant | design's pre-merge figure | measured on the merged tree | budget | verdict |
| --- | --- | --- | --- | --- |
| literal `[bracket]` pairs rendered on the crate root (anti-pattern 2) | 4 (`_design.md:697`, `:839-841`) | 0 under `cargo doc -p happenstance --no-deps`; **2** under the gate's own `documentation` step, `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps --document-private-items`, which exited **0** with both unresolved | zero, on every page this project touches | **improved, and invocation-dependent — see the note below.** Under the canonical invocation both `happenstance_core` references resolve to `<a href="../happenstance_core/index.html">`; under the gate's they render as `[<code>happenstance_core</code>]` |
| hidden (`#`-prefixed) lines in the crate-root fence, and what they carry | 5 of 9 source lines hidden; only 4 lines rendered (`_design.md:524`) | 2 of 37 source fence lines: `lib.rs:29` (`# #[tokio::main] async fn main() -> …`) and `lib.rs:61` (`# Ok::<(), Box<dyn Error>>(()) }`) | forbidden for any `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call, or any assertion (`_design.md:524`) | **holds, with one observation.** Neither hidden line carries a forbidden construct — the `commit` call and `assert_eq!` are both visible (rendered lines 34 and 35). The `#[tokio::main]` wrapper is on neither the permitted nor the forbidden list; recorded so `boundary-refusal-encounter` decides it deliberately rather than inheriting it |
| widest rendered fence line, in columns | budget re-derived to 68 from finding F-1 (`_design.md:552-562`) | 70 — two lines, rendered lines 12 (`fn event_type…`) and 33 (`\|_: &Seats\| Ok::<_, core::convert::Infallible>(vec![Seat::Taken]);`) | 68 hard; `overflow-x` engages at 72 on the 696px fence at 1024×768 | **exceeded by 2 columns, on 2 lines.** Below the 72 at which a scrollbar actually appears, so anti-pattern 4 does not fire today — but the budget does. Routed with the row below |
| rendered fence height, in lines | the crate-root program was projected at 31 rendered lines, and the ceiling exempted to 32 for that reason (finding F-4, `_design.md:563-574`) | 35 | 32 on `crate-root-encounter` alone; 24 on a step page | **exceeded by 3 lines.** The merged fence is a *different program* from the one F-4 measured — it teaches `commit`, not the refusal — so this is not F-4's exemption being overspent; it is a new starting point |
| authored `##` headings over 22 characters | eleven of the sixteen headings the design *names* exceed it (`_design.md:583-584`) | 2 of the 4 authored h2 on the merged crate root: `What arrives here, and what stays below` (39) and `Testing without a database` (26). Within budget: `The vocabulary` (14), `Features` (8) | 22 characters, before the 200px sidebar TOC truncates with an ellipsis | **exceeded on 2 of 4.** Both are inherited, not authored by this project |
| answered-need line on the crate root | "exactly one, above everything" (`_design.md:429-430`, IQ-8/UX-008) | none. The merged crate root carries a one-line summary and goes straight to the fence | exactly one, above the first fence | **absent, as expected.** This project authors it; the notation now exists (`standards/pages/00-one-need.md`) where the design recorded it as a gap |

**The bracket count depends on the `cargo doc` invocation, and that was measured rather than
inferred.** Both builds were run in this session, one after the other, and the rendered
`index.html` counted each time:

```console
$ cargo doc -p happenstance --no-deps            # the invocation _design.md:1025-1027 names
$ # count of "[<code>" in target/doc/happenstance/index.html -> 0
$ RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features \
      --no-deps --document-private-items         # xtask/src/main.rs:344, the REQUIRED step
$ echo $?                                        # -> 0
$ # count of "[<code>" in target/doc/happenstance/index.html -> 2
```

Two consequences, and neither is this story's to fix. First, **any composition number read
off a render must name the invocation that produced it**, or two honest people measuring the
same tree get different answers — every figure in this section was therefore taken under the
canonical `-p happenstance --no-deps` build. Second, `-D warnings` did not fail on two
unresolved intra-doc links, which is `_design.md` gap 6's observation ("`cargo doc` did not
warn") reproduced on the merged tree under the *stricter* of the two invocations. The
page-level half is a condition on `boundary-refusal-encounter`, which owns anti-pattern 2 on
that surface; the gate-level half — that the `documentation` step can render a broken link and
exit 0 — is an incidental substrate defect and routes to the `support` initiative
(`.redkiln/config.yaml:5`).

**EC-005 applies to the two exceeded density rows, and this is the routing.** Both are
conditions on **`boundary-refusal-encounter`** (HS-S0185), which is the story that rewrites
`crates/happenstance/src/lib.rs` and is therefore the only one that can act on them. Neither
number is relaxed here and `_design.md` is not edited. The finding, stated so that story can
act on it without re-measuring: *the merged crate-root fence is 35 rendered lines and 70
columns wide; the design's ceiling for that surface is 32 lines and 68 columns; the rewrite
replaces the program, so meeting both is a property of the program that story writes, not a
budget this story must renegotiate.*

**Zero teaching content shipped.** This story's authored diff is this folder. No Rust fence, no
answered-need line, no mapping table, no output block, and no control outside rustdoc's own
chrome (anti-pattern 8) — the two `console` transcripts above are command output, not teaching.
The merge's own diff against its first parent is 223 commits of the sibling's source arriving
by **parentage**; `git diff --name-only a5c0f30^1 a5c0f30 -- .bklg/docs-that-teach/application-author-path`
is empty, which is the check that says the merge authored nothing in this project's scope.

**EC-007 fired, and this is the recorded output rather than a widened fence.**
`redkiln verify --grain story --item HS-S0183` was run at this story's checkpoint. Two of its
four checks pass and two do not, both for reasons the spec forecast:

```console
redkiln: verify HS-S0183 (story): FAIL
redkiln:   [ok]   affected-gate
redkiln:   [FAIL] boundary — changed outside declared boundary: <~230 paths>
redkiln:   [ok]   ledger
redkiln:   [FAIL] provenance — 13 file(s) changed inside this story's declared
                  boundary and links.commits is empty
```

The `boundary` failure is EC-007 exactly: the tool diffs the branch against its base and
cannot express "arrived by parentage", so all ~230 files the merge's **second parent** brings
are reported as changed outside the fence. They are not authored: `git diff --name-only
a5c0f30^1 a5c0f30 -- .bklg/docs-that-teach/application-author-path` is empty, and the seven
files the merge genuinely *resolved* are each named in the fence. The response is to record
this, not to widen the fence to `**` — a fence widened to quiet a tool stops being a statement
about scope, and every future merge-bearing story would inherit the precedent.

The `provenance` failure is a sequencing artifact, not a defect: `links.commits` is written by
`redkiln record-links` **after** the checkpoint commit exists, and the CLI is the only writer
of an item's system frontmatter (CLAUDE.md). The commit is cut in the same session; recording
it is the orchestrating command's step, not this record's.

`[ok] ledger` is the check that matters here, and it is green: five criteria, each present,
satisfied and carrying non-placeholder evidence.

## Re-deriving this record

Every row above names its own command. To re-take the whole record at once:

```console
$ git rev-list --parents -n 1 a5c0f30
$ git merge-base --is-ancestor initiative/from-contract-to-published-library HEAD
$ git diff-tree --cc --name-only a5c0f30
$ cargo doc -p happenstance --no-deps
$ cargo xtask spec-trace && cargo xtask lints
$ cargo xtask ci --fast
```

The § Anchors table is machine-checkable as it stands: each row's last column is a command
whose printed line number must equal the line in its second column. A row that has rotted says
so the first time anyone runs it, which is the only property that makes this table worth more
than the corpus of line citations it supersedes.

## § Carried forward to HS-P0025

Two process observations from this story that are **not** rework here and must not be
rediscovered at closeout. HS-P0025 (`durable-audience-closeout`) owns the whole-initiative
merge-forward re-observation in `merge-forward-baseline/`, which is the story that meets both
of these next; project DoD item 9's routing rule (`project.md:300-302`) is why they are written
down rather than remembered.

| # | the observation | why it is HS-P0025's | what would close it |
| --- | --- | --- | --- |
| P1 | This story's PR-boundary fence was completed **after** its conflict resolutions rather than before, twice (`spec.md:213-264`), against its own EC-001. The disclosure is in the fence itself and the slice review verified all seven resolved files independently — union preserved, no test dropped, all ten recomputed constitution citations resolving, the `doc_budget.rs` CRLF normalisation strengthening the test — so nothing is hidden by it, and the ordering is the whole finding | The next merge-bearing story is HS-P0025's, and it merges a wider tree than this one did. The point of naming a resolution path *first* is that nobody gets to decide a file was in scope by having already edited it — a property that survives only if the rule is met before the merge, not audited after it | The closeout's merge story names every path it expects to resolve **before** running the merge, and records the delta between that list and `git diff-tree --cc --name-only <merge>` as a finding either way |
| P2 | `redkiln verify --grain story` cannot express "arrived by parentage". Both this story's and `tension-resolutions`' `boundary` checks failed on ~230 paths the merge commit's **second parent** brings, none of them authored — `git diff HEAD^1 HEAD` is empty of anything either story wrote. Neither was answered by widening the fence, and both recorded the raw output instead (EC-007, `_ledger.md`; `tension-resolutions/_resolutions.md § Conditions and dispositions`) | A glob cannot describe a merge, so every merge-bearing story on this initiative pays the same false failure and answers it in prose. Whether the fix is a `verify` flag, a first-parent diff, or a documented exemption is a tooling decision, not a story decision — and the closeout is where tooling observations for this initiative are collected | A recorded disposition at closeout: either an issue on the `support` initiative (`.redkiln/config.yaml:5`) naming the desired semantics, or an explicit statement that recording the raw output is the accepted answer and merge stories will keep doing it |
