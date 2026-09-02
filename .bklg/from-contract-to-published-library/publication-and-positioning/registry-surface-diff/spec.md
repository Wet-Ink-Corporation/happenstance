---
item: HS-S0091
stage: spec
created: 2026-08-12T13:47:29.262Z
updated: 2026-08-12T13:47:29.262Z
template_sig: 87bbf1d0
rendered_sig: 52aa59a3
---

# Spec — The public surface is diffed against what is already on the registry

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — **DoD 11**, *"The release is diffed, not asserted"* (`:390-392`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the DAG that makes the alpha baseline substrate for this project, not evidence |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — **AC-002** (`:229-232`), **DR-2** (`:162-164`), and the coupling note that without the baseline *"the project's central instrument does not exist"* (`:355-358`) |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — **testing brief** (the AC-002 row at `:465`, the string-fixture seam at `:507-523`, AC-TEST-002 at `:530-533`) and **deployment brief** (`### CI implication` `:682-721`, `### Release path if a published version changes` `:722-747`, AC-DEP-005 `:766-769`). No `architecture` brief exists for this project, deliberately (`:788-792`). |
| Signed-off design | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — binding, and it names this story: *"Nothing is removed and no signature changes. If `registry-surface-diff` (AC-002) reports otherwise against the `0.2.0-alpha.1` baseline, that is a finding this file did not anticipate and it blocks the release for a re-plan — it does not get absorbed here"* (`:76-79`). This story renders none of its seven surfaces (`:104-176`). |
| Grounding | `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md` — *"AC-002's registry-baseline diff is a second, distinct comparison"* (`:54-58`); the derive-and-reconcile pattern to reuse (`:160-175`) |
| Story map / merge order | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` — milestone `publish-time-gate-instruments`, position 2.4, after `crate-set-decision`, unordered with 2.1–2.3 (`:161-165`); *"the version follows the report"* (`:186-191`) |
| Baseline owner (upstream substrate) | `.bklg/from-contract-to-published-library/typed-layer-and-alpha-release/publish-0-2-0-alpha-1/spec.md` — the story that puts `0.2.0-alpha.1` on the registry and moves `Cargo.toml:6`, the three requirement strings and `crates/happenstance-testkit/Cargo.toml:14` to it |

## One-line PR slice

Add a mandatory `cargo xtask` step that diffs the public surface of the decided crate set against
the `0.2.0-alpha.1` **registry** baseline — distinct from `CONTRIBUTING.md`:291-296's branch-point
`cargo-semver-checks` job, which proves nothing about the last release — commit its dated report,
derive the published version number from what it found, fail closed with a stated reason when the
baseline is unreachable, and prove it rejects a seeded breaking change.

## Executive summary

**What lands.** A new `xtask` module, `surface_diff`, in **two modes**: a mandatory, network-free
*check* mode wired into `xtask/src/main.rs`'s `REQUIRED` list and `xtask/src/affected.rs`'s
unconditional block, and a release-grain *capture* mode (`--run`) that shells `cargo semver-checks`
at the published registry version and writes a dated report under `spec/audits/`. Plus the first
report, one corrected paragraph in `CONTRIBUTING.md`, and a `CHANGELOG.md` entry.

**The delta against what the tree already does.** `cargo-semver-checks` already runs on every pull
request — `.github/workflows/ci.yml:295-316`, with `baseline-rev` pointed at the branch point, over
all three publishable crates. That job is not what AC-002 asks for, and `CONTRIBUTING.md:295-301`
says so in its own words: *"It proves nothing about the last released version: a break merged two
pull requests ago is part of the baseline and so is invisible."* Three things are genuinely absent
from this tree, and they are what this PR is for:

1. **Nothing compares this tree against the registry.** Until `typed-layer-and-alpha-release` ships
   `0.2.0-alpha.1` there was nothing to compare against — the three reserved names sit at `0.0.0`
   and Cargo treats every `0.0.x` as incompatible with every other, so the existing job's own
   comment records the registry baseline as deliberately deferred to this phase
   (`.github/workflows/ci.yml:298-308`).
2. **Nothing makes the release's version number a derived fact.** AC-002 requires the published
   number to be *"the one that report's findings imply"*. Today it is whatever someone types into
   `Cargo.toml:6`, and nothing in the gate would notice a disagreement.
3. **There is no committed surface-diff artefact.** Project DoD 3 requires one with a date; the
   signed-off design (`_design.md`:76-79) has already staked a claim on what it must say and made
   the opposite finding a release blocker.

**What this PR is not.** It does not publish, does not move any crate version, does not edit
`.github/workflows/ci.yml`'s `semver` job, and does not change one line of public API. If the diff
reports a break, this story **records** it — deciding what to do about it is `publish-0-2-0`'s and,
where the design did not anticipate it, a re-plan's (`_design.md`:76-79; `_storymap.md`:186-191).

## Context pack

Everything an implementer needs to start. Deeper material sits behind the anchors, never pasted here.

**Two instruments, two questions, and conflating them is the named wrong implementation.** The
existing PR-grain job proves *this pull request* did not break what it branched from. This step
proves *this tree* has not broken what is on the registry. A step that runs `cargo semver-checks`
with `--baseline-rev` and is relabelled "the registry surface diff" passes every mechanical property
a shallow review checks and still leaves AC-002 unmet (`discover.md`:40-42). The mechanical
consequence: the invocation this module builds carries `--baseline-version <exact published
version>` and **must never carry `--baseline-rev`**, and that is asserted on the constructed argv in
a unit test — argv is constructible without running anything or touching the network.

**The mandatory step is network-free by construction, and that is how AC-DEP-005 is honoured
honestly.** The deployment brief requires both new instruments in the **Mandatory** list, never
behind a tool probe, on the stated basis that optional means *skipped when the probed tool is
absent* and never *skipped when the check would fail* (`_decomposition.md`:682-701;
`xtask/src/main.rs`:89-102). Its premise — *"there is no external tool to probe for"* — is true of
the clause audit and **would be false** of a step that shelled to `cargo-semver-checks` on every
`cargo xtask ci`. So the step is split rather than the brief contradicted: the mandatory half reads
two files in the tree plus `git`, exactly like `spec-trace` and `affected`, and the half that needs
the tool and the network is a release-grain command the gate never invokes. Anyone can run the gate
offline; nobody can reach the release without the real run.

**Fail closed, and say why — never find nothing and pass.** ADR-0010's discipline is that a skip is
reported, never silent (`.kb/decisions/0010-the-suite-must-prove-itself.md`), and the deployment
brief applies it to this instrument by name for the case that mattered when it was written: the
baseline not existing yet (`_decomposition.md`:701-707). Concretely, in check mode: no released
section in `CHANGELOG.md` (nothing has been published, so there is no baseline), no report at all, a
report whose header is missing a field, a report file whose name is not an ISO date, a report whose
crate set disagrees with `PUBLISHABLE`, a `captured-at` that is not an ancestor of `HEAD`. In
capture mode: the tool absent, the registry unreachable, the named baseline version not on the
registry. Each fails naming the path, the value it read and what the reader must do. **A locator
that finds zero reports and reports zero problems is the single most likely wrong implementation of
this story.**

**The version number is derived and reconciled, never typed.** This is `xtask/src/package.rs`'s
pattern one level up (`:24-42` — the derivation is the fact, the hand-written list is the intention,
and holding both is what lets a failure say which side moved). The report declares a
`release-version` per crate; check mode **recomputes** it from the baseline and the findings and
fails on disagreement. The rule, both branches, because the second is what every release after this
one uses (`_decomposition.md`:722-747):

- **Baseline is a pre-release of the candidate** (`0.2.0-alpha.1` → `0.2.0`): the implied version is
  that release, whatever the findings — *provided* every breaking finding is enumerated in the
  report's `accepted-breaking` block with a one-line reason. This is not a formality. Cargo's caret
  requirement `^0.2.0-alpha.1` **does** match `0.2.0`, so a consumer who took the alpha is carried
  onto the release by `cargo update`; the alpha's published `## Stability` section is what makes
  that acceptable, and enumerating each break is what keeps "acceptable" from meaning "unread".
- **Baseline is a stable `0.Y.Z`**: any breaking finding implies `0.(Y+1).0`; otherwise
  `0.Y.(Z+1)`. Under 0.x the minor bump *is* the breaking boundary and that is the promise this
  release makes (`_decomposition.md`:722-732; `project.md`:141-142).

**Three crates, three baselines, and one of them moves on its own key.** CF-32 is `[FROZEN]`
(`spec/SPECIFICATION.md`:8200-8220) precisely because the contract's number and the testkit's number
mean different things — *"the contract's is a promise about types, the testkit's is a promise about
the bar"* — so the diff runs **per crate against that crate's own published version**, and the
report carries three rows, not one verdict. The reconciliation that follows from it: `happenstance`
and `happenstance-core` inherit `[workspace.package] version` (`Cargo.toml`:5-6), so their implied
versions must agree with each other or the report is internally inconsistent and fails;
`happenstance-testkit` carries its own key and may legitimately differ. A single workspace-wide
verdict is the shape a naive implementation reaches for and it is wrong against a frozen clause.

**State what the tool does not check — a check whose limits are undocumented is read as a
guarantee.** `xtask/src/main.rs`:37-41 makes this the house rule for gate steps, and the
constitution already carries the measurement: `cargo-semver-checks` 0.50 *"does not detect breaking
type changes, generic/lifetime changes, or breakage visible only under a feature subset (checked
2026-08-09, rustc 1.97.1)"* (`standards/rust/40-public-surface-and-evolution.md`:69-71). The report
therefore carries the tool's version, the feature set each run used, and a *"what this did not
check"* section naming those three blind spots — and the last of them is why the feature powerset
step is named there as the nearest neighbouring check rather than left implied.

**The crate set is an input, not a guess.** This story blocks on `crate-set-decision`
(`story.md` `blocked_by: HS-S0084`) because the set it diffs is that story's recorded answer: three
crates, with the four-crate alternative rejected (`_decomposition.md`:574-608). The wiring is
mechanical rather than editorial — check mode reads `PUBLISHABLE` (`xtask/src/package.rs`:86) and
fails when the report's crate set differs, so a crate-set change invalidates the report instead of
silently narrowing it.

**The persona slice.** Backbone activity **A2**, *"prove the promises before making them"*
(`_storymap.md`:41): the maintainer at the release gate, and — one hop downstream — *"the consumer
whose build AC-11 says must not break by surprise"*. Like its slice-mate, this story has no user
intent of its own, which is exactly why it is a gate instrument rather than copy. Its reach onto a
published surface is indirect and specific: the enumerated `accepted-breaking` list is the material
`publish-0-2-0`'s changelog section is written from, and IQ-5 forbids *"run our CI"* as a claim's
only evidence (`_decomposition.md`:289-296) — a committed report is a reachable artefact, a green
badge is not.

**What this story must not settle in passing.** Whether any finding is *acceptable* is not this
instrument's call. `_design.md`:76-79 has already ruled that an unanticipated removal or signature
change **blocks the release for a re-plan**, and `_storymap.md`:186-191 says the same about a report
implying a version outside `0.2.0`. The step's job is to make the finding unmissable and the number
underivable-by-hand; it is not to choose the remedy.

## Integration contract

| | |
| --- | --- |
| **Archetype** | `capability` — the maintainer runs one command and gets an answer that can block a release, and a consumer's build is what is on the other side of it. |
| **Slice / milestone** | `publish-time-gate-instruments`. Slice-mates, implemented in one context and mounted as one integrated surface: `falsifier-ledger-repair`, `clause-maturity-audit` (the sibling instrument mounting into the same four places), `deferred-clause-reread`. This story is unordered with all three (`_storymap.md`:161-165) but shares their mount, so the four edits below must be made once, not four times. |
| **Mount point** | **`xtask/src/main.rs`** — the gate's composition root, and the same four edits `clause-maturity-audit` makes: the `REQUIRED` array (a `Step` with `probe: None`, following the `spec-trace` entry at `:315-328` and sitting beside `package-check` at `:519-532`); the `main` dispatch (`:639-706`, a `Some("surface-diff")` arm with flag handling modelled on `spec-trace`'s at `:670-678`); `print_help` (`:718-772`); and the module doc's *"What the gate proves"* paragraph (`:8-24`), which is the one place in this repository that states in prose what the gate proves and which AC-DEP-005 requires updated in the same change (`_decomposition.md`:716-721). |
| **Second mount** | **`xtask/src/affected.rs`:114-125** — the unconditional file-reading block, so the story-grain gate (`cargo xtask affected --base main`, wired at `.redkiln/config.yaml`:40) runs it for a diff that touches no package. A story whose whole diff is the report, the changelog or `Cargo.toml`'s version line maps to no package's sources, and that is exactly the diff this check exists to read. |
| **Wires into** | `xtask/src/package.rs`:86 `PUBLISHABLE` and `:226-241` `publishable_members` — the decided crate set, consumed read-only (visibility widened to `pub(crate)` where needed, no behaviour change); `xtask/src/lints.rs`:463-481 `entries` — the established `CHANGELOG.md` parser precedent for reading the newest released heading; `Cargo.toml`:5-6 `[workspace.package] version` and `crates/happenstance-testkit/Cargo.toml`:14 (CF-32's independent key), read-only; `git` via `Command`, as `affected.rs` already does, for the `captured-at` ancestry test; `cargo semver-checks` — **capture mode only**, never from the gate. |
| **Renders surfaces** | **None.** `_design.md`'s seven surfaces (`:104-176`) are rendered registry and docs.rs pages owned by `published-surface-copy` and `rendered-page-preflight`. This story is upstream of them by *constraint* rather than by copy: `_design.md`:76-79 states that nothing is removed and no signature changes, and this instrument is what can falsify that sentence. |
| **Public items** | **None.** `_design.md`'s `## Items` block (`:49-79`) carries two entries and this story implements neither; `xtask` is `publish = false` and adds no public API. That is the correct reading of the Items block, not a gap in it. |
| **Conformance rule(s)** | **None, and it is not adapter-observable.** No rule in `crates/happenstance-testkit/src/suite.rs` can observe whether a published surface changed — the subject is a registry artefact, not a store. The equivalent obligation is discharged in the module's own `#[cfg(test)] mod tests` (testing brief AC-TEST-002, `_decomposition.md`:530-533), where each fixture names the wrong implementation it rejects. |
| **Clause(s)** | **Reads CF-32 (`spec/SPECIFICATION.md`:8200-8220), amends nothing.** CF-32 is `[FROZEN]`; this story honours it by diffing per crate against per-crate baselines rather than assuming one workspace number, and touching a frozen clause would take a new ADR, not an edit (DR-15, `project.md`:217-219). `§1.2`'s statement that a `[FROZEN]` `ES` clause is semver-binding (`:182-184`) is the reason a break in `happenstance-core` is a specification-level event and not merely a packaging one. |
| **Advances DoD scenario** | Initiative **DoD 11** — *"The release is diffed, not asserted. The public-surface comparison runs against the prior published baseline and its report is recorded; the version chosen matches what it found"* (`initiative.md`:390-392). It also supplies the second of the three dated artefacts project DoD 3 requires, and is a precondition of DoD 13's *"green on the exact tree that was published"* being worth anything about the surface. |

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
xtask/src/surface_diff.rs
xtask/src/main.rs
xtask/src/affected.rs
xtask/src/package.rs
spec/audits/surface-diff-*.md
CONTRIBUTING.md
CHANGELOG.md
.bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/**
```

**In this PR**

- `xtask/src/surface_diff.rs` — the module: the report reader and its header parser, the four check-
  mode assertions, the implied-version rule with both branches, the capture-mode invocation builder
  and transcript scraper, the report writer, and `#[cfg(test)] mod tests`.
- `xtask/src/main.rs` — the four mount edits named in the Integration contract.
- `xtask/src/affected.rs` — one call appended to the unconditional block, and the module doc
  extended to name it.
- `xtask/src/package.rs` — **visibility only.** `pub(crate)` on `PUBLISHABLE` (and
  `publishable_members` if the check reconciles against the derived set as well as the intention
  list), plus a doc line naming the new consumer. `cargo xtask package-check`'s output is identical
  before and after.
- `spec/audits/surface-diff-<YYYY-MM-DD>.md` — the first real report, produced by a real `--run`
  against the registry, committed. The directory is shared with `clause-maturity-audit`; whichever
  slice-mate lands first creates it, and the glob above is narrowed to this story's files so the two
  cannot collide when interleaved.
- `CONTRIBUTING.md`:295-301 — the paragraph that currently ends *"The registry baseline that would
  catch it is not available yet… Phase 12 keeps both baselines once a real `0.1.0` exists."* It is
  now available and this step is the second baseline; the paragraph is corrected in place to name
  `cargo xtask surface-diff` and to keep the distinction between the two jobs explicit. Leaving it
  is the documentation form of IQ-7 (`_decomposition.md`:305-310): a sentence that is false on the
  tree that shipped.
- `CHANGELOG.md` — one entry naming the defect the step detects, matching the discipline
  `lint-changelog` already enforces for conformance rules (CF-29).

**Explicitly not in this PR**

- **`.github/workflows/ci.yml`'s `semver` job.** It stays exactly as it is, `baseline-rev` included.
  The two answer different questions and neither substitutes for the other
  (`CONTRIBUTING.md`:295-301); deleting or repointing it would trade a per-PR check for a per-
  release one and call it a simplification.
- **Any version bump.** `Cargo.toml`:6, the three requirement strings at `:24-26` and
  `crates/happenstance-testkit/Cargo.toml`:14 are untouched. The report *implies* a number; moving
  the manifests to it is `publish-0-2-0`'s act, and asserting the manifests carry the implied number
  is that story's check, not this one's.
- **Any public API change**, in either direction — including "fixing" a finding. A break the design
  did not anticipate blocks the release for a re-plan (`_design.md`:76-79); it is not repaired
  inside a gate-instrument PR.
- **The clause audit, the ledger repair and the deferred-clause reasons** — the three slice-mates.
  They share the mount point; they do not share this module.
- **Publishing anything, or running the whole `cargo xtask ci` on a publish commit** —
  `publish-0-2-0`.
- **Any packaged README or docs.rs copy.** `published-surface-copy` owns every consumer-facing
  sentence, including any that quotes this report.

**Merge DoD.** `cargo xtask ci --fast` is green with the new step in the Mandatory list; the first
dated report is in the tree and check mode passes against it; check mode fails, naming the path and
the value it read, on each seeded fixture; and a real `--run` against the registry — including one
against a seeded breaking change, reverted — is captured under this story's folder.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B1 — `cargo xtask surface-diff`, the gate mode** | No flags, no network, no external tool. Reads the newest report under `spec/audits/`, the `CHANGELOG.md` baseline, `PUBLISHABLE` and `git`; prints one line per crate naming baseline, verdict and implied version; exits non-zero on any problem. Writes nothing. This is the form `REQUIRED` and `affected` invoke. | `xtask/src/main.rs`:315-328 (the `spec-trace` step to copy), `:639-706` (dispatch), `:89-102` (`probe: None` means mandatory) |
| **B2 — `cargo xtask surface-diff --run --date <YYYY-MM-DD>`, the capture mode** | Runs `cargo semver-checks` once per crate in the set against that crate's published baseline, captures each invocation's argv, exit status and full transcript, and writes `spec/audits/surface-diff-<date>.md`. Never invoked by `ci`, `--fast` or `affected`. The date is an **argument, not a clock read** — the release date is a decision, and an explicit date keeps the writer deterministic and its tests hermetic. Refuses to overwrite an existing report for that date unless the content is identical. | `xtask/src/spec_trace.rs` (`Mode::Check` / `Mode::Write` precedent); `xtask/src/lints.rs` (CF-33: nothing in this repository's gate reads a clock without saying so) |
| **B3 — the baseline is a published version, per crate, never a rev** | The invocation carries `--baseline-version <exact published version>` and `-p <crate>`, once per crate, and **never `--baseline-rev`**. Three crates means three invocations and three baselines, because `happenstance-testkit` moves on its own key (CF-32). The constructed argv is asserted in a unit test — this is the direct rejection of the story's named wrong implementation and it needs neither network nor tool to check. | `discover.md`:40-42; `spec/SPECIFICATION.md`:8200-8220 (CF-32, `[FROZEN]`); `.github/workflows/ci.yml`:298-316 (the job this must not be confused with) |
| **B4 — verdicts come from exit status, not from parsed prose** | Each crate's verdict is the exit status of its own invocation: zero means no breaking change found, non-zero means one was. Lint identities are scraped from the transcript only to *name* findings for the acceptance list, and the scraper is unit-tested against a synthetic transcript. A non-zero exit that yields an empty scraped set is **schema drift and must fail**, never be rounded to "no findings" — the same call `package.rs` makes about an unrecognised `publish` value. | `xtask/src/package.rs`:44-68, `:449-456` (schema drift stops the gate rather than being guessed at); `_decomposition.md`:507-519 (synthetic `cargo-semver-checks` output, not a registry round trip) |
| **B5 — the report is the artefact, and its header is machine-read** | `spec/audits/surface-diff-<YYYY-MM-DD>.md` carries: `captured-on` (the date), `captured-at` (the commit, `git rev-parse HEAD`), the `cargo semver-checks` version, and per crate a row of *baseline version · feature set · verdict · findings · release-version*. Then the `accepted-breaking` block, the verbatim transcripts, and the *"what this did not check"* section. Every field the gate reads is in the header; the prose below it is for a human and no check depends on it. | `project.md` DoD 3 (`:295-296`, a dated committed artefact); `_design.md`:76-79 |
| **B6 — the implied version is recomputed and reconciled** | Check mode derives each crate's implied version from its baseline and findings by the two-branch rule in the Context pack, and fails when the report's declared `release-version` differs — naming which side moved. It additionally fails when `happenstance` and `happenstance-core` imply different numbers, because they share one `[workspace.package] version` key and a shared key cannot carry two numbers. | `xtask/src/package.rs`:24-42 (derive-and-reconcile, and why both sides are kept); `Cargo.toml`:5-6; `_decomposition.md`:722-732 |
| **B7 — a breaking finding is enumerated and accepted, never waved through** | Every finding the transcripts carry must appear in the report's `accepted-breaking` block with a one-line reason, and every entry in that block must correspond to a finding. Both directions fail, because they mean different things: an unlisted finding is a break nobody read, and a listed non-finding is an acceptance carried over from a report that no longer exists. | `_storymap.md`:186-191; `_design.md`:76-79; `.kb/decisions/0010-the-suite-must-prove-itself.md` |
| **B8 — the crate set comes from the decision, not from the report** | The report's crate set must equal `PUBLISHABLE` (`xtask/src/package.rs`:86). A crate added to or removed from the published set invalidates the report rather than narrowing it silently — which is what makes the `crate-set-decision` dependency mechanical instead of editorial. | `xtask/src/package.rs`:81-94; `_decomposition.md`:574-583; `story.md` `blocked_by: HS-S0084` |
| **B9 — staleness is bound by what can be checked without a network** | Check mode asserts the report's `captured-at` is an ancestor of `HEAD` (a report captured on an abandoned branch is not evidence) and that its baselines match the newest released version in `CHANGELOG.md`. It states plainly what it does **not** verify: that the surface has not moved since `captured-at`. That gap is covered by the PR-grain job on every pull request and closed at the release by `publish-0-2-0` re-running capture mode on the literal publish commit. | `xtask/src/affected.rs`:100-125 (shelling to `git`, and the unconditional block); `xtask/src/lints.rs`:463-481 (the changelog parser precedent); `xtask/src/main.rs`:37-41 (a check states its own limits) |
| **B10 — report selection is deterministic and cannot skip what it fails to parse** | Reports are named by ISO date, so lexical order is chronological, and check mode reads the lexically greatest match. A file under `spec/audits/` matching the surface-diff prefix whose date is not a well-formed ISO date is a **failure**, not a file to skip — a glob that silently ignores what it cannot parse is how the newest report stops being the one that is read. | `xtask/src/package.rs`:44-68 (narrow, documented parsing over a guessed schema) |
| **B11 — the fail-closed catalogue** | Check mode: `CHANGELOG.md` has no released section (nothing is published — name `typed-layer-and-alpha-release` as the owner); no report; a malformed report name; a missing or empty header field; a crate-set disagreement; a `captured-at` that is not an ancestor; an unreconciled finding or acceptance; an implied/declared version disagreement. Capture mode: `cargo semver-checks` absent (name the install command); the registry unreachable; the named baseline version not on the registry. Each names the path, the value read and the remedy. **Never an empty set compared against an empty set.** | `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_decomposition.md`:701-707; `xtask/src/main.rs`:89-102 |
| **B12 — the report states what it did not check** | Named, not implied: `cargo-semver-checks` does not detect breaking type changes, generic or lifetime changes, or breakage visible only under a feature subset. The report carries the tool version, the feature set each invocation used, and the date, and points at the feature-powerset step as the nearest neighbouring check. A check whose limits are undocumented is read as a guarantee. | `standards/rust/40-public-surface-and-evolution.md`:69-71 (*checked 2026-08-09, rustc 1.97.1*); `xtask/src/main.rs`:37-41 |
| **B13 — unit tests, in the established shape** | `#[cfg(test)] mod tests` with fixture strings and a named wrong implementation per test, following `xtask/src/package.rs`:408-457. Fixtures required: an argv assertion that the invocation carries `--baseline-version` and not `--baseline-rev`; a transcript with a breaking finding and an empty acceptance list (must fail); an acceptance entry with no matching finding (must fail); a non-zero exit with zero scraped lints (schema drift, must fail); both branches of the implied-version rule; `happenstance` and `happenstance-core` implying different numbers (must fail); a crate set of two; a report file named `surface-diff-2026-1-5.md` (must fail, not skip); a `CHANGELOG.md` with only `## [Unreleased]` (must fail with the baseline-owner message). Fixtures are synthetic strings, never a real registry round trip. | `_decomposition.md`:507-519, `:530-533`; `xtask/src/package.rs`:408-457; `xtask/src/spec_trace.rs` (`#[cfg(test)]` blocks, the same discipline applied to a parser) |
| **B14 — runtime posture** | Check mode: three file reads and two `git` invocations. No network, no registry, no clock, no external tool. It runs in `--fast` and in `affected`, on every story in this project. Capture mode: three network builds, run deliberately, at the release. Every cargo invocation that resolves dependencies passes `--locked`. | `xtask/src/main.rs`:52-54; `_decomposition.md`:682-701 |

## Data and migrations

**No database, no schema, no runtime data.** `project.md`'s Out of scope reserves any API surface
change for an upstream project, and the deployment brief records migration and backfill as N/A with
the reason stated: this is the first real publish of these three crates, the reserved names at
`0.0.0` have no compatible predecessor a consumer could have depended on, and this library defines
no storage schema of its own (`_decomposition.md`:643-655; `CONTRIBUTING.md`:298-301).

What this story does add is one **committed text artefact** whose format and lifecycle are specified
here rather than discovered during implementation, because a gate step reads it.

| Artefact | Path | Lifecycle |
| --- | --- | --- |
| Surface-diff report | `spec/audits/surface-diff-<YYYY-MM-DD>.md` | Written by `--run`, one per capture, never edited by hand. Header fields — `captured-on`, `captured-at`, tool version, and per crate `baseline-version` / `feature-set` / `verdict` / `release-version` — are machine-read; the `accepted-breaking` block is hand-written and reconciled against the transcripts; the transcripts are verbatim. It is a **snapshot, not a mirror**: check mode reads it to verify it is current, complete and self-consistent, never to learn whether the surface is safe. `publish-0-2-0` captures the next one on the publish commit; that one, not this one, is the release's artefact of record. |

**The one lifecycle event that looks like a migration** is superseding a report, and it is
deliberately a capture rather than an edit: a new `--run` at a new date writes a new file, and the
old report stays in the tree as the record of what was true then. Correcting a report by editing it
would break `captured-at`'s meaning, which is the only staleness anchor check mode has.

**Rollback.** Removing the step is a revert of the four mount edits; nothing else in the tree
depends on the report at merge time. That changes at `publish-0-2-0`, which reads the implied
version, and it is why this story lands early in the slice rather than beside it. The wider
posture is the project's: reversibility is bought before the irreversible act, because `cargo yank`
removes a version from resolution and leaves every rendered page exactly as it was
(`_decomposition.md`:657-673).

## Acceptance criteria

Framed from intent, not from capability. This story sits on backbone activity **A2**, *"prove the
promises before making them"* (`_storymap.md`:41), whose two people are **the maintainer at the
release gate** and, one hop downstream, **the consumer whose build initiative AC-11 says must not
break by surprise** (`initiative.md`:338). A2 has no user intent of its own — the storymap says so
in as many words (`:47-50`) — so each criterion below is written as the *maintainer's* goal crossing
the full stack from a clean checkout to a number that can be published, with the consumer named
wherever the outcome reaches them.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The gate proves it without a network, and says that it does.** *GIVEN* a maintainer on a clean checkout with no network and no `cargo-semver-checks` installed — the ordinary case for a contributor and for every `affected` run, *WHEN* they run `cargo xtask ci --fast`, or `cargo xtask affected --base main` on a diff that touches no workspace package (only `CHANGELOG.md`, `Cargo.toml`'s version line or a report), *THEN* the surface-diff step runs — it is in `REQUIRED` with `probe: None`, so it can never be skipped for a missing tool — completes offline from file reads and `git` alone, **writes nothing to the tree**, prints its result without displacing any other step's output, and the two places that state in prose what the gate proves (`xtask/src/main.rs`'s *"What the gate proves"* paragraph `:8-24`, and `print_help` `:718-772`) name it. A step whose existence is not in the gate's own prose is a claim shipped without its evidence. | `xtask/src/surface_diff.rs::tests::check_mode_touches_no_network_and_writes_nothing` (asserts check mode's whole input set is file reads + `git`, over a `tempfile`-free fixture root, and that the fixture root's contents are byte-identical after the run); `xtask/src/surface_diff.rs::tests::required_step_is_mandatory_not_probed` (asserts the `Step` this module contributes carries `probe: None`); `xtask/src/surface_diff.rs::tests::gate_prose_names_this_step` (`include_str!` over `xtask/src/main.rs`, asserting the *"What the gate proves"* paragraph and `print_help`'s body both contain the command name — the same shape `lints.rs` uses to read a file in a test); manual: `cargo xtask ci --fast` green with the network interface down, and `cargo xtask affected --base main` on a `CHANGELOG.md`-only diff |
| AC-002 | **The answer is about the registry, not about the branch point.** *GIVEN* a maintainer who wants to know whether the crate they are about to publish breaks the version consumers are already building against, and *GIVEN* that `.github/workflows/ci.yml`:295-316 already answers a different question on every PR, *WHEN* they run `cargo xtask surface-diff --run --date <YYYY-MM-DD>`, *THEN* the invocation carries `--baseline-version <exact published version>` and `-p <crate>` and **never `--baseline-rev`**, so a break merged several PRs ago — invisible to the branch-point job by construction, as `CONTRIBUTING.md`:295-301 states in its own words — is caught here. The maintainer can tell the two apart from the output alone, because each names its own baseline. | `xtask/src/surface_diff.rs::tests::baseline_is_a_published_version_never_a_rev` (constructs the argv for the three-crate set and asserts `--baseline-version` present with the exact version string, `--baseline-rev` absent — argv is constructible with no network and no tool); `xtask/src/surface_diff.rs::tests::break_merged_before_the_branch_point_is_still_a_finding` (the discover.md scenario: a fixture whose breaking change predates the branch point, asserting the registry-baseline path reports it — the named wrong implementation, rejected directly) |
| AC-003 | **The evidence is a committed artefact a person can open, not a green badge.** *GIVEN* a reviewer, or the maintainer six months later, who wants to know what was actually compared, *WHEN* they open `spec/audits/surface-diff-<YYYY-MM-DD>.md`, *THEN* they find a header carrying `captured-on`, `captured-at`, the `cargo semver-checks` version, and **one row per crate** with baseline version · feature set · verdict · findings · release-version, followed by the `accepted-breaking` block, the verbatim transcripts and the *"what this did not check"* section — and check mode reads only that header, never the prose. **Composition, binding:** every verdict cell carries **words**, never a bare glyph or colour (`_design.md` AP-2, `:646-647`); every count and verdict is attached to a date and a version in the same block (AP-3, `:648-650`); the header is plain Markdown with no raw HTML, inline style or meaning-carrying image (AP-7, `:657-658`). **Density:** the success path prints **exactly one line per crate — three lines**, matching `xtask/src/package.rs`:138-143's one-line-per-crate convention, and the report's header is **≤ 12 lines plus one row per crate**, so it fits one screen before the transcripts begin. | `xtask/src/surface_diff.rs::tests::header_round_trips_through_writer_and_reader` (writer output parsed by the reader, all fields recovered); `xtask/src/surface_diff.rs::tests::missing_header_field_fails_naming_the_field` (each required field removed in turn); `xtask/src/surface_diff.rs::tests::verdict_cell_carries_words_not_a_glyph` (a fixture whose verdict cell is `✅` alone is rejected — AP-2); `xtask/src/surface_diff.rs::tests::check_mode_prints_one_line_per_crate` (asserts the captured stdout line count equals the crate-set size on the success path); the committed `spec/audits/surface-diff-<date>.md` itself, reviewed at the slice review |
| AC-004 | **The version number is a derived fact the maintainer can be argued out of, not a number they typed.** *GIVEN* a maintainer at the release gate who must choose what to publish as, *WHEN* check mode runs, *THEN* it **recomputes** each crate's implied version from that crate's baseline and findings — pre-release-of-the-candidate ⇒ the candidate release, provided every breaking finding is enumerated; stable `0.Y.Z` ⇒ `0.(Y+1).0` on any break, else `0.Y.(Z+1)` — and fails when the report's declared `release-version` differs, **naming which side moved**; and it fails when `happenstance` and `happenstance-core` imply different numbers, because they share one `[workspace.package] version` key (`Cargo.toml`:5-6) and a shared key cannot carry two numbers. A report implying anything outside `0.2.0` is a re-plan, not a rounded-down number (`_storymap.md`:186-191). | `xtask/src/surface_diff.rs::tests::prerelease_baseline_implies_the_candidate_release`; `::tests::stable_baseline_with_a_break_implies_a_minor_bump`; `::tests::stable_baseline_without_a_break_implies_a_patch_bump`; `::tests::declared_version_disagreeing_with_derived_fails_naming_both`; `::tests::shared_workspace_key_implying_two_numbers_fails` (the wrong implementation: computing per crate and never reconciling the two that share a key) |
| AC-005 | **A deliberately breaking change fails it — the instrument can fail.** *GIVEN* project AC-002's own text, *"Running it against a deliberately breaking change fails it"* (`project.md`:229-232), and CLAUDE.md's corollary that a rule no adapter can fail is decorative, *WHEN* a public item is removed or a signature changed and capture mode is run, *THEN* the run reports a breaking verdict for that crate, and check mode then fails unless the finding appears in `accepted-breaking` with a one-line reason. Both directions fail: an unlisted finding is a break nobody read; a listed non-finding is an acceptance carried over from a report that no longer exists. The seeded change is **reverted** — a break the design did not anticipate blocks the release for a re-plan (`_design.md`:76-79), it is not repaired inside a gate-instrument PR — and the real `--run` transcript for both the clean and the seeded case is captured under this story's folder. | `xtask/src/surface_diff.rs::tests::finding_absent_from_accepted_breaking_fails`; `::tests::acceptance_with_no_matching_finding_fails`; `::tests::nonzero_exit_with_zero_scraped_lints_is_schema_drift` (a non-zero exit that yields an empty finding set must fail, never round to *"no findings"* — the call `xtask/src/package.rs`:44-68 already makes about an unrecognised value); manual, recorded: a real `--run` against a seeded removal of a public item, transcript committed under `.bklg/.../registry-surface-diff/`, seed reverted |
| AC-006 | **When it cannot answer, it says so and stops — it never finds nothing and passes.** *GIVEN* a maintainer running the gate in a state the instrument cannot evaluate — nothing published yet, no report, a malformed report name, a `captured-at` that is not an ancestor of `HEAD`, `cargo-semver-checks` absent, the registry unreachable, *WHEN* the step runs, *THEN* it exits non-zero with a message that names **the path, the value it read, and what the reader must do next**, in that one message — no second command to run to find out why, and no context jump into a log (ADR-0010's *"a skip is reported, never silent"*, `.kb/decisions/0010-the-suite-must-prove-itself.md`; `_decomposition.md`:701-707). **The prohibited pass is an empty set compared against an empty set**: a locator that finds zero reports and reports zero problems is this story's single most likely wrong implementation. Where the cause is upstream, the message names the owner (`typed-layer-and-alpha-release`) rather than leaving the maintainer to work it out. | `xtask/src/surface_diff.rs::tests::changelog_with_only_unreleased_fails_naming_the_baseline_owner`; `::tests::no_report_at_all_fails_rather_than_passing_vacuously` (the decorative-gate rejection, stated as a test); `::tests::report_name_that_is_not_an_iso_date_fails_not_skips` (fixture `surface-diff-2026-1-5.md`); `::tests::captured_at_not_an_ancestor_of_head_fails`; `::tests::every_failure_message_names_path_value_and_remedy` (table-driven over the whole catalogue: each message must contain the fixture path, the offending value and an imperative sentence) |
| AC-007 | **The reader is told what was *not* checked, and no sentence in the tree contradicts the tree.** *GIVEN* a consumer or evaluator who will read a claim derived from this report, *WHEN* they reach the report, *THEN* it names its three blind spots — `cargo-semver-checks` does not detect breaking type changes, generic or lifetime changes, or breakage visible only under a feature subset (`standards/rust/40-public-surface-and-evolution.md`:69-71, *checked 2026-08-09, rustc 1.97.1*) — carries the tool version and the feature set each invocation used, and points at the feature-powerset step as the nearest neighbouring check. **No absence is stated without its reason next to it** (`_design.md` AP-13, `:668-669`; RS-40-5). *AND* `CONTRIBUTING.md`:295-301, which today says the registry baseline *"is not available yet"*, is corrected in place to name this step and to keep the two jobs distinct — leaving it is the documentation form of a sentence that is false on the tree that shipped (IQ-7, `_decomposition.md`:305-310), and the same defect class as `_design.md`'s AP-9 known-stale strings. A `CHANGELOG.md` entry names the defect the step detects, matching the discipline `lint-changelog` already enforces. | `xtask/src/surface_diff.rs::tests::report_carries_tool_version_feature_set_and_limits` (a report missing the *"what this did not check"* section, or carrying it with no tool version, is rejected); `::tests::contributing_no_longer_claims_the_baseline_is_unavailable` (`include_str!` over `CONTRIBUTING.md`, asserting the stale phrase is gone and the new command name is present — the same file-reading-in-a-test shape as `xtask/src/lints.rs`); `cargo xtask ci --fast` (the existing `lint-changelog` step, on the new entry) |
| AC-008 | **What is diffed is the decided crate set, and three crates mean three baselines.** *GIVEN* `crate-set-decision`'s recorded answer — three crates, the four-crate alternative rejected (`_decomposition.md`:574-608) — and CF-32's `[FROZEN]` ruling that the testkit carries its own `version` key because *"the contract's is a promise about types, the testkit's is a promise about the bar"* (`spec/SPECIFICATION.md`:8200-8220), *WHEN* the step runs, *THEN* it diffs **per crate against that crate's own published version** and the report carries **three rows, not one verdict**; and the report's crate set must equal `PUBLISHABLE` (`xtask/src/package.rs`:86), so adding or removing a published crate **invalidates** the report rather than silently narrowing it. That is what makes the dependency on `crate-set-decision` mechanical rather than editorial. | `xtask/src/surface_diff.rs::tests::report_crate_set_must_equal_publishable` (a two-crate report against a three-crate `PUBLISHABLE` fails, naming both sets); `::tests::testkit_baseline_is_independent_of_the_workspace_version` (a fixture where the testkit's published version differs from the workspace's, asserting three distinct baselines are used — the wrong implementation is one workspace-wide verdict, which is wrong against a frozen clause); `cargo xtask spec-trace` (CF-32's citation integrity, already a gate step) |

**Coverage of the traced project AC.** This story traces to **AC-002** only (`project.md`:229-232),
and its four clauses map exactly: *"a surface comparison runs against the `0.2.0-alpha.1` registry
baseline"* → AC-002 and AC-008; *"its report is committed in the tree"* → AC-003; *"the published
version number is the one that report's findings imply"* → AC-004; *"running it against a
deliberately breaking change fails it"* → AC-005. AC-001, AC-006 and AC-007 are the deployment
brief's AC-DEP-005 (`_decomposition.md`:766-769) and ADR-0010's fail-closed discipline, which
AC-002 cannot be honestly met without.

## Interaction quality

**This story renders none of `_design.md`'s seven surfaces** (`:104-176`) — those are rendered
registry and docs.rs pages owned by `published-surface-copy` and `rendered-page-preflight`. It does
compose two surfaces of its own, and both are read by a person under time pressure: **the gate
step's terminal output** and **the committed report**. The design's composition rules are written to
be checkable *against a rendered page by someone who cannot read the code* (`_design.md`:643-645),
which is exactly the standard these two must meet, so the applicable anti-patterns bind here too.
Every invariant below is carried by an **AC row in the table above**; this section only says which,
because a bullet here would get no ledger row and would never be gated.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — a failure states path, value and remedy in the one message; the maintainer never has to run a second command to learn why the gate stopped (IQ-1, `_decomposition.md`:232-240) | **AC-006** | `::tests::every_failure_message_names_path_value_and_remedy`, table-driven over the whole fail-closed catalogue |
| **Non-occlusion — a filter must not hide what it filters** (IQ-2, `:241-259`): zero inputs never render as zero problems; a non-zero exit with an empty scraped set is schema drift, not *"no findings"*; a report file that cannot be parsed fails rather than being skipped | **AC-006**, and **AC-005** for the schema-drift arm | `::tests::no_report_at_all_fails_rather_than_passing_vacuously`, `::tests::report_name_that_is_not_an_iso_date_fails_not_skips`, `::tests::nonzero_exit_with_zero_scraped_lints_is_schema_drift` |
| **Reversibility** — check mode writes nothing, so running the gate can never change what the next run reads; superseding a report is a new capture at a new date, never an edit, because `captured-at` is the only staleness anchor check mode has (IQ-4, `:272-288`) | **AC-001** | `::tests::check_mode_touches_no_network_and_writes_nothing`, asserting the fixture root is byte-identical after the run |
| **Preserved position** — the step's output does not displace or interleave with the surrounding steps'; it prints its lines and returns, exactly as `package-check` does | **AC-001**, **AC-003** | `::tests::check_mode_prints_one_line_per_crate` |
| **Reachable without running anything** (IQ-5, `:289-296`) — the evidence is a committed file at a stable path, not *"the gate was green"*; the `accepted-breaking` block is the material `publish-0-2-0`'s changelog section is written from | **AC-003** | the committed `spec/audits/surface-diff-<date>.md`, reviewed at the slice review |
| **Keyboard reachability** — **not applicable.** There is no pointer target on either surface: one is a CLI invocation, the other a Markdown file. Recorded rather than omitted, so a reviewer can tell a considered N/A from a gap | — | — |

**Composition invariants** (from `_design.md`, which binds even though no surface it names is
rendered here — its anti-patterns are phrased for exactly this kind of read-by-a-human artefact)

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the report is a composed document with a machine-read header, a hand-written acceptance block and verbatim transcripts, in that order; a bare dump of `cargo semver-checks` stdout is not a report and does not satisfy AC-003 | **AC-003** | `::tests::header_round_trips_through_writer_and_reader`, `::tests::missing_header_field_fails_naming_the_field` |
| **AP-2 — no status whose only signal is a glyph or a colour** (`_design.md`:646-647): every verdict cell carries words | **AC-003** | `::tests::verdict_cell_carries_words_not_a_glyph` |
| **AP-3 — no count or verdict without a date and a version attached** (`:648-650`): `captured-on` and the per-crate baseline version are required header fields | **AC-003** | `::tests::missing_header_field_fails_naming_the_field` |
| **AP-7 — no raw HTML, inline style, custom colour or meaning-carrying image** (`:657-658`): plain Markdown only | **AC-003** | reviewed against the committed report at the slice review; the writer emits no HTML by construction |
| **AP-13 / RS-40-5 — no absence stated without a reason beside it** (`:668-669`; `standards/rust/40-public-surface-and-evolution.md`:212): the *"what this did not check"* section names the three blind spots and the nearest neighbouring check, in the report, not on another page | **AC-007** | `::tests::report_carries_tool_version_feature_set_and_limits` |
| **AP-9, by class — no sentence in the tree that is false on the tree that shipped** (`:661-663`; IQ-7): `CONTRIBUTING.md`:295-301's *"not available yet"* is corrected in the same change that makes it available | **AC-007** | `::tests::contributing_no_longer_claims_the_baseline_is_unavailable` |
| **Transience** — the *persistent chrome* of this surface is the three per-crate result lines (always printed, success or failure); the transcripts are *opened on demand*, one hop, inside the committed report; the fail-closed remedy is *revealed* in the same message as the failure, never one hop away | **AC-003**, **AC-006** | `::tests::check_mode_prints_one_line_per_crate`, `::tests::every_failure_message_names_path_value_and_remedy` |
| **Density budget, with its numbers** — success output: **exactly one line per crate, three lines**, per `xtask/src/package.rs`:138-143. Report header: **≤ 12 lines plus one row per crate**, so the header and the crate table fit one screen before the transcripts. `accepted-breaking`: **one line per finding**, one reason each — a paragraph per finding turns an enumerated list into prose nobody reconciles | **AC-003** | `::tests::check_mode_prints_one_line_per_crate`; the header budget reviewed against the committed report |
| **Hierarchy** — the crate table is the primary element of the report and appears before the transcripts; **one primary element per screen** (AP-15, `:674-675`), so the transcripts never compete with the verdicts | **AC-003** | reviewed at the slice review against the committed report |

## Error conditions

Each is a fail-closed exit, non-zero, naming the path, the value read and the remedy (AC-006). None
is a warning: a warning in a gate step is an escape hatch that was not decided on.

| id | Condition | Behaviour |
| --- | --- | --- |
| **EC-001** | `CHANGELOG.md` has no released section — nothing has ever been published, so there is no baseline | Fail, naming `CHANGELOG.md`, the newest heading it found, and **`typed-layer-and-alpha-release` as the owner of the missing baseline**. This is the interim state discover.md predicted (`discover.md`, *Questions*), and it must never be a pass |
| **EC-002** | No report under `spec/audits/` matching the surface-diff prefix | Fail, naming the directory searched and the glob. **The decorative-gate failure mode**: zero reports must never mean zero problems |
| **EC-003** | A report file matching the prefix whose date is not a well-formed ISO date (e.g. `surface-diff-2026-1-5.md`) | Fail, naming the file. Skipping it would silently change which report is "newest" |
| **EC-004** | A required header field is missing or empty | Fail, naming the file, the field and the line |
| **EC-005** | The report's crate set differs from `PUBLISHABLE` (`xtask/src/package.rs`:86) | Fail, printing both sets and the symmetric difference. The report is invalid, not narrowed |
| **EC-006** | `captured-at` is not an ancestor of `HEAD` (`git merge-base --is-ancestor`) | Fail, naming the commit and the check. A report captured on an abandoned branch is not evidence |
| **EC-007** | A transcript finding has no entry in `accepted-breaking` | Fail, naming the lint identity and the crate. A break nobody read |
| **EC-008** | An `accepted-breaking` entry matches no finding | Fail, naming the entry. An acceptance carried over from a report that no longer exists |
| **EC-009** | Declared `release-version` differs from the derived one | Fail, printing both and **which side moved** |
| **EC-010** | `happenstance` and `happenstance-core` imply different versions | Fail, citing `Cargo.toml`:5-6 — one key cannot carry two numbers |
| **EC-011** | *Capture mode.* `cargo semver-checks` is absent | Fail, naming the install command. Capture mode is not in the gate, so this is never a gate skip |
| **EC-012** | *Capture mode.* The registry is unreachable, or the named baseline version is not on the registry | Fail, naming the crate, the version requested and the registry response. Never fall back to `--baseline-rev` |
| **EC-013** | *Capture mode.* A non-zero exit yields an empty scraped finding set | Fail as **schema drift**, naming the tool version. The tool's output shape changed; guessing is what `xtask/src/package.rs`:44-68 refuses to do |
| **EC-014** | *Capture mode.* A report already exists for the requested date with different content | Fail, naming the existing file. Capture a new date instead; superseding is a capture, never an edit |

## Non-functional

| id | Requirement | Why, and how it is held |
| --- | --- | --- |
| **NF-001** | **Check mode performs no network I/O and requires no external tool.** Its whole input is three file reads plus two `git` invocations | It is in `REQUIRED` with `probe: None` and runs on every `affected` invocation in this project. A mandatory step that needs a network turns a contributor's offline gate into a flake, and would make AC-DEP-005's premise (*"there is no external tool to probe for"*, `_decomposition.md`:690-694) false rather than honoured |
| **NF-002** | **Check mode adds well under a second to the gate**, being file reads and two `git` calls — no build, no resolve | `cargo xtask ci --fast` is this project's integration gate and `affected` is its story gate; a step that costs a build would be paid on every story in the project |
| **NF-003** | **No clock is read.** The capture date is an argument (`--date`), never `SystemTime::now()` | CF-33 already forbids a conformance rule reading a clock (`xtask/src/main.rs`:28-31); the same discipline makes the report writer deterministic and its tests hermetic. The release date is a decision, not an observation |
| **NF-004** | **Every cargo invocation that resolves dependencies passes `--locked`** | `xtask/src/main.rs`:52-54: a gate that silently updates `Cargo.lock` tested a graph nobody committed. Capture mode's three `cargo semver-checks` runs are exactly such invocations |
| **NF-005** | **Unit tests run with no network and no registry round trip**, against synthetic fixture strings | The testing brief's fixture seam (`_decomposition.md`:507-523), and `xtask/src/package.rs`:417-423's `METADATA` constant as the precedent. A test that needs the registry cannot run in the offline gate it is meant to protect |
| **NF-006** | **Windows-clean.** No shell string, no `sh -c`, no path separator assumptions; `Command` with argument vectors, as `affected.rs` already does | This repository is developed on Windows (`xtask/src/main.rs`:84-88 says so about the env-var workaround), and a gate step that only runs on CI is not a gate |
| **NF-007** | **No new dependency in `xtask/Cargo.toml`.** The header parser is hand-written against a documented, narrow format | `xtask`'s only dependency is `anyhow`, deliberately (`xtask/Cargo.toml`:8-9), and `standards/rust/50-dependency-hygiene.md` is the standing rule. A TOML or YAML crate for a twelve-line header is a dependency bought for a parser this repository already writes by hand three times |

## Implementation notes (non-prescriptive)

Shape suggestions, not requirements — the ACs are the contract.

- **Two modes, one module, the `spec_trace` shape.** `xtask/src/spec_trace.rs` already carries a
  check/write split for a document the gate reads and a command writes; reaching for the same
  `Mode` enum keeps the reader's existing map of this codebase valid. Resist a second module for
  capture mode: the writer and the reader must agree on the header format, and the cheapest way to
  keep them agreeing is to compile them together and round-trip them in a test.
- **Parse narrowly and fail loudly.** `xtask/src/package.rs`:44-68 is the house pattern: recognise
  exactly the shapes you documented, and treat anything else as drift that stops the gate. The
  header is the only machine-read region, so keep it small, keyed, and one field per line.
- **Scrape to *name*, decide from the *exit status*.** The verdict is the process's exit status; the
  transcript is scraped only to label findings for the acceptance list. Keeping those two roles
  separate is what makes EC-013 expressible at all — if the verdict came from the prose, an empty
  scrape would just be a passing crate.
- **`git` via `Command`, as `affected.rs` already does** (`:100-125`). `merge-base --is-ancestor`
  answers EC-006 with an exit status and no parsing.
- **Slice-mate coordination.** `clause-maturity-audit` makes the same four edits to
  `xtask/src/main.rs` and shares `spec/audits/`. Make each mount edit once for both steps; whichever
  story lands first creates the directory. The PR boundary above is narrowed to this story's report
  glob so an interleaved order cannot collide.
- **Visibility, not refactoring, in `package.rs`.** Widen `PUBLISHABLE` to `pub(crate)` and add a
  doc line naming the new consumer. `cargo xtask package-check`'s output must be identical before
  and after — if it is not, something was refactored that this story did not scope.
- **Write the failing tests before the happy path.** Every fixture in B13 names a wrong
  implementation; the module is easiest to get right by making each wrong implementation fail first,
  which is also the order the testing brief's AC-TEST-002 describes.

## Tests and CI (merge gate)

Grounded in the testing brief (`_decomposition.md`, *Testing brief*): AC-002's row assigns **unit +
integration (new gate step)**, AC-TEST-002 requires a `#[cfg(test)] mod tests` in the
`package.rs`/`spec_trace.rs` shape, and the fixture seam is synthetic strings rather than a registry
round trip.

| tier | command / path | proves |
| --- | --- | --- |
| **unit** | `cargo test -p xtask` → `xtask/src/surface_diff.rs` `#[cfg(test)] mod tests` | Every AC's mechanical half: the argv (AC-002), the header round trip and the composition rules (AC-003), both branches of the version rule and the shared-key reconciliation (AC-004), the reconciliation of findings and acceptances plus schema drift (AC-005), the whole fail-closed catalogue with its message content (AC-006), the report's stated limits and `CONTRIBUTING.md`'s correction (AC-007), the crate-set and per-crate-baseline rules (AC-008). Each test names the wrong implementation it rejects in a doc comment, per `xtask/src/package.rs`:408-457 |
| **integration (new gate step)** | `cargo xtask surface-diff` | The step runs end to end against the real tree and the real committed report — the first thing that proves the module's pieces agree with the artefact that shipped |
| **integration (story grain)** | `cargo xtask affected --base main` (wired at `.redkiln/config.yaml`:40) | The second mount: a diff touching no workspace package — a report, `CHANGELOG.md`, a version line — is still gated. This is the case the unconditional block at `xtask/src/affected.rs`:114-125 exists for |
| **integration (project grain)** | `cargo xtask ci --fast` (wired at `.redkiln/config.yaml`:55) | The step is in `REQUIRED`, is picked up by `--fast` (which drops only the two feature powersets, `cargo deny` and the nightly docsrs build), and the whole gate is green with it added |
| **static** | `cargo xtask spec-trace` | CF-32's citations still resolve after this story reads them (`spec/SPECIFICATION.md`:8200-8220) |
| **static** | the existing `lint-changelog` step inside `cargo xtask ci` | CF-29's discipline over the new `CHANGELOG.md` entry naming the defect the step detects |
| **manual, recorded** | a real `cargo xtask surface-diff --run --date <YYYY-MM-DD>` against the registry, twice: clean, and against a seeded breaking change | AC-005's *"running it against a deliberately breaking change fails it"* — the only tier a synthetic fixture cannot discharge, because it is the tool-plus-registry path itself. Both transcripts committed under this story's folder; the seed reverted |
| **process** | `redkiln validate --kb && redkiln doctor` | AC-TEST-003: clean at this story's checkpoint, with exactly the six expected `template-drift` advisories and no `dependency-cycle` |

**Not in this story's gate.** `.github/workflows/ci.yml`'s `semver` job is untouched and keeps
running with `--baseline-rev`; the full `cargo xtask ci` on the publish commit is AC-016's and
`publish-0-2-0`'s (`_decomposition.md`, *Merge-gate commands*); the stranger-install smoke is
`stranger-install-smoke`'s and can only run after publication.

## Risks and coupling (PR-scoped)

| Risk | Why it is live here | Mitigation inside this PR |
| --- | --- | --- |
| **The relabelled branch-point job.** Implementing this with `--baseline-rev` passes every shallow check and leaves AC-002 unmet | It is this story's named wrong implementation (`discover.md`, *The wrong implementation*), and it is the cheaper thing to build | AC-002's argv assertion is a unit test that needs neither network nor tool, plus the pre-branch-point fixture. The two jobs are also kept textually distinct in `CONTRIBUTING.md` (AC-007) so the confusion cannot re-enter through prose |
| **The vacuous pass.** A locator that finds nothing and reports nothing, especially while the baseline does not yet exist | The baseline is upstream (`typed-layer-and-alpha-release`) and may still be absent when this story is implemented | EC-001 and EC-002 are ACs, not notes, and `::tests::no_report_at_all_fails_rather_than_passing_vacuously` is the direct rejection. The gate goes red until the baseline exists — which is the intended state, per ADR-0010 |
| **A mandatory step that needs a network.** Wiring capture mode into `REQUIRED` would make every contributor's gate depend on crates.io | The obvious simplification is one command instead of two | The split is an AC (AC-001, NF-001) and is asserted by a test over check mode's input set. AC-DEP-005's *"there is no external tool to probe for"* premise stays true only because of the split |
| **Slice-mate collision on `xtask/src/main.rs` and `spec/audits/`.** Four stories mount into the same four places | `clause-maturity-audit`, `falsifier-ledger-repair` and `deferred-clause-reread` share this slice and this mount | The slice is implemented in **one context** (`_storymap.md`:161-165): make each mount edit once. The PR boundary's report glob is narrowed to this story's files |
| **The report finds a real break.** `_design.md`:76-79 has already staked a claim that nothing is removed and no signature changes | This instrument is precisely what can falsify that sentence | Out of scope by construction: the finding is **recorded**, not repaired. It blocks the release for a re-plan (`_design.md`:76-79) and the version follows the report (`_storymap.md`:186-191). No public API change is inside this PR's boundary |
| **`cargo-semver-checks` output-format drift** between the version that wrote a report and the version that reads it | The scraper reads a third-party tool's transcript | The verdict comes from the exit status, never the prose; the tool version is a required header field; a non-zero exit with an empty scrape is EC-013, a hard failure |
| **Staleness between `captured-at` and `HEAD`.** Check mode cannot prove the surface has not moved since capture without a network | It is a real limit, and an undocumented limit is read as a guarantee | B9 states it plainly in the module doc and in the report; the PR-grain job covers the interval, and `publish-0-2-0` re-runs capture on the literal publish commit |
| **`package.rs` visibility widening drifting into a refactor** | The step needs `PUBLISHABLE`, and the temptation is to restructure while there | Visibility only, plus one doc line; `cargo xtask package-check`'s output must be byte-identical before and after |

## Dependencies

**Blocks on**

- **`crate-set-decision`** (`HS-S0084`) — the set this story diffs is that story's recorded answer:
  three crates, four-crate alternative rejected (`_decomposition.md`:574-608; AC-DEP-001). The
  coupling is mechanical, not editorial: AC-008 reconciles the report's crate set against
  `PUBLISHABLE`, so diffing a set the decision did not make is a failure rather than a narrower
  report. This is the only blocker — the slice may start as soon as it lands (`_storymap.md`:161).

**Unblocks**

- **`publish-0-2-0`** (`HS-S0096`, `story.md` `blocks:`) — it reads the report's implied version and
  moves `Cargo.toml`:6, the three requirement strings and `crates/happenstance-testkit/Cargo.toml`:14
  to it, then re-runs capture mode on the literal publish commit. Without this story the number it
  would publish is a typed one.
- **`compliance-claim-and-gaps-promise`**, indirectly — the enumerated `accepted-breaking` list is
  the material a published changelog or compliance sentence is written from, and IQ-5 forbids
  *"run our CI"* as a claim's only evidence (`_decomposition.md`:289-296).

**Unordered with** its three slice-mates — `falsifier-ledger-repair`, `clause-maturity-audit`,
`deferred-clause-reread` (`_storymap.md`:161-165) — but sharing their mount point, so they are
implemented in one context and the four `xtask/src/main.rs` edits are made once.

**Upstream substrate, not a story dependency.** The `0.2.0-alpha.1` registry baseline is put there by
`typed-layer-and-alpha-release`'s `publish-0-2-0-alpha-1`. Until it exists the step fails closed with
EC-001's message naming that owner — which is the specified behaviour, not a blocked state
(`_decomposition.md`:701-707).

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Open each at the moment named, not before.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/main.rs` | The composition root. `:89-102` defines what `probe: None` means (mandatory, never skipped for a missing tool) — the sentence AC-001 turns on; `:315-328` is the `spec-trace` step to copy; `:639-706` the dispatch; `:718-772` `print_help`; `:8-24` the *"What the gate proves"* paragraph AC-DEP-005 requires updated in the same change; `:37-41` is the house rule that a check states its own limits | Before the first mount edit, and again before writing AC-007's limits section | AC-001 |
| `xtask/src/package.rs` | Three distinct precedents, all reused verbatim: `:24-42` derive-and-reconcile (why both the derived fact and the hand-written intention are kept, which is AC-004's whole shape); `:44-68` narrow parsing where drift stops the gate (EC-013); `:86` `PUBLISHABLE`; `:138-143` the one-line-per-crate output convention AC-003's density budget cites; `:408-457` the `mod tests` shape every unit test must follow | Before writing the version rule (AC-004) and before the first test (AC-005, AC-008) | AC-004 |
| `xtask/src/affected.rs` | `:100-125` is the second mount — the unconditional block — and the existing precedent for shelling to `git` from a gate step, which is how EC-006's ancestry check is done | When wiring the second mount, and when implementing the `captured-at` check | AC-001 |
| `xtask/src/spec_trace.rs` | The check/write mode split this module copies, and `#[cfg(test)]` blocks applying the fixture-string discipline to a parser rather than to a JSON scanner | Before choosing the module's shape, at the start of implementation | AC-003 |
| `xtask/src/lints.rs` | `:463-481` `entries` is the existing `CHANGELOG.md` parser — do not write a second one to find the newest released heading (EC-001), and it is also the precedent for reading a tree file inside a test (AC-007's `CONTRIBUTING.md` assertion) | When implementing the baseline lookup and AC-007's prose assertion | AC-007 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/registry-surface-diff/discover.md` | Carries the named wrong implementation in full — the relabelled `--baseline-rev` job — including the pre-branch-point scenario the fixture must reproduce, and confirms `--baseline-version` is the supported registry-baseline flag | Before writing the invocation builder or its test | AC-002 |
| `.github/workflows/ci.yml` | `:295-316` is the existing PR-grain `semver` job. Read it to see exactly what this step must **not** duplicate; `:298-308`'s own comment records the registry baseline as deliberately deferred to this phase | Before the invocation builder, alongside `discover.md` | AC-002 |
| `CONTRIBUTING.md` | `:291-296` documents the existing job; `:295-301` is the paragraph that is now false and must be corrected in place, and it states the branch-point limit in the repository's own words | When making the AC-007 prose edit | AC-007 |
| `spec/SPECIFICATION.md` | `:8200-8220` is CF-32, `[FROZEN]`: the testkit's version key means something different from the contract's, which is why there are three baselines and three rows rather than one verdict. `§1.2`:182-184 is why a break in `happenstance-core` is a specification-level event | Before deciding the report's row structure, at the start of implementation | AC-008 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The accepted decision behind the whole fail-closed catalogue: a skip is reported, never silent, and a check that cannot fail is decorative | Before writing the failure paths — read it first, not after the happy path works | AC-006 |
| `standards/rust/40-public-surface-and-evolution.md` | `:69-71` carries the measured blind spots of `cargo-semver-checks` 0.50 with the date and rustc version — the exact text AC-007's *"what this did not check"* section is written from; `:212` is RS-40-5, no absence without a reason | When writing the report's limits section | AC-007 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | Binding on composition even though no surface it names is rendered here: `:76-79` states that nothing is removed and no signature changes, and makes a contrary finding a re-plan; `:641-676` are the anti-patterns AC-003 and AC-007 carry (AP-2, AP-3, AP-7, AP-9, AP-13, AP-15) | Before designing the report's layout, and again if the diff reports a break | AC-003 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The deployment brief's `### CI implication` (`:682-721`) is why both steps are Mandatory rather than probed, and states the premise the two-mode split preserves; `:701-707` is the fail-closed requirement; AC-DEP-005 at `:766-769` | Before the mount edits | AC-001 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The testing brief's fixture seam (`:507-523`) and AC-TEST-002 (`:530-533`): synthetic `cargo-semver-checks` output, a named wrong implementation per test, never a registry round trip | Before writing the first test | AC-005 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | `:161-165` places this story in the slice and names the three slice-mates that share its mount; `:186-191` rules that the version follows the report and that a report implying something outside `0.2.0` is a re-plan | At implementation start, to coordinate the shared mount; again if the derived version is not `0.2.0` | AC-004 |

## Clarifications resolved during spec

- **The AC set is exactly the eight the front half enumerated** — AC-001 … AC-008 — with no
  additions or drops. AC-002's four clauses map onto AC-002, AC-003, AC-004, AC-005; AC-001,
  AC-006 and AC-007 carry AC-DEP-005 and ADR-0010; AC-008 carries CF-32 and the crate-set coupling.
- **Discovery deferred the report's format to spec, and spec settles it** (`discover.md`,
  *Questions*: *"does the report's committed format need to match `package.rs`'s style or
  `spec_trace`'s?"*). Neither: the format is specified here (B5, AC-003), because a gate step reads
  it and an unspecified format read by a parser is a format discovered during implementation. It
  borrows `spec_trace`'s check/write **mode split** and `package.rs`'s narrow-parsing **discipline**
  without copying either's document shape.
- **Discovery asked whether `cargo-semver-checks` supports a registry-version baseline directly. It
  does** (`--baseline-version`), and that is now a normative requirement rather than a confirmation:
  AC-002 asserts the flag on the constructed argv and forbids `--baseline-rev`.
- **The step is split into two modes, which discovery did not anticipate.** Discovery said *"add the
  step to the Mandatory list"*. Taken literally with a single mode, that would put a network call
  and an external-tool dependency into every `cargo xtask ci` — contradicting AC-DEP-005's own
  premise that *"there is no external tool to probe for"* (`_decomposition.md`:690-694). The split
  honours the brief instead of contradicting it: the mandatory half is file-reading and network-free;
  the tool-and-network half is a release-grain command the gate never invokes. Nothing reaches the
  release without the real run.
- **`--date` is an argument, not a clock read.** Not raised in discovery; settled here on CF-33's
  precedent (`xtask/src/main.rs`:28-31) and because a report writer that reads a clock cannot be
  tested hermetically. The release date is a decision.
- **This story renders none of `_design.md`'s seven surfaces, but the design's anti-patterns still
  bind.** They are written to be checked against a rendered artefact by someone who cannot read the
  code (`_design.md`:643-645), and this story ships two such artefacts — the terminal output and the
  committed report. AP-2, AP-3, AP-7, AP-9, AP-13 and AP-15 are therefore carried as AC rows rather
  than waved off as inapplicable. Keyboard reachability is recorded as a considered N/A.
- **Whether a finding is *acceptable* is deliberately not settled here.** `_design.md`:76-79 and
  `_storymap.md`:186-191 already own that call: an unanticipated removal or signature change blocks
  the release for a re-plan, and the version follows the report. This story makes the finding
  unmissable and the number underivable by hand; it does not choose the remedy, and it changes no
  public API in either direction.
