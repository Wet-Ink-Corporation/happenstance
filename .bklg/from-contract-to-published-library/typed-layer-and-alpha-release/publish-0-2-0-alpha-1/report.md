---
item: "HS-S0033"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — 0.2.0-alpha.1 on the registry, with its churn mitigations

## Findings Ledger

**Outcome, as amended 2026-08-16: seven of seven ACs satisfied.** This section was
written when AC-007 was BLOCKED; the block has since been cleared and the text below is
kept rather than rewritten, because a report that quietly becomes a success story is
worth less than one that shows what was outstanding and when it landed.

**What was blocking, and what unblocked it.** `cargo publish` against the live crates.io
index is a **human handoff** — declared as one by `_storymap.md:163-166`, and unreachable
by any gate step (DEP-006, `_decomposition.md:964-977`). It has now been run: all three
crates are live at `0.2.0-alpha.1` (`_release-log.md` §5.1), and the three read-back
checks that a successful upload does *not* discharge have been run separately — the
outside-workspace resolution and write-then-read cycle (§5.3), the rendered crates.io
page (§5.4) and docs.rs (§5.5). See *The blocker, and how it cleared*.

**One thing read back badly and is routed rather than swallowed** (`_release-log.md`
§5.6): every documentation link on the published page 404s, because the GitHub repository
is not publicly reachable. Assessed against EC-007 and deliberately not yanked — a new
version number contains the identical strings and fixes nothing — and routed to HS-P0016,
which this story's PR boundary already names as the owner of registry presentation.

Everything up to the cut is done, and the release gate is green **whole**.

| AC | Result | Proven by | Notes |
| --- | --- | --- | --- |
| **AC-001** | satisfied | `cargo metadata` exits 0 with **no** internal requirement on `^0.2.0`; `cargo publish --dry-run -p happenstance-core` green | The check **earned its place** — it found a sixth manifest the boundary list did not enumerate, and the workspace had stopped resolving. See *The finding* |
| **AC-002** | satisfied | `crates/happenstance-testkit/Cargo.toml:21` is a **literal** `version = "0.2.0-alpha.1"`; CF-32's gate step green in the full run | The comment above it is preserved and **extended** with why the number moves *down*: CF-32 gives the crate an independent number, not an independent maturity |
| **AC-003** | satisfied | `rg -n '^#'` region order; `rg -n 'facade'` → nothing; first fence at `:30` | The blockquote is **deleted**, not annotated. The fence is at exactly the line it was on before — see *The presentation constraint that bit* |
| **AC-004** | satisfied | `crates/happenstance/README.md:15-20`; no-hedge check over the section returns nothing | Exactly three claims: the phase in the reader's terms with the consequence attached, the CHANGELOG link, and the yank policy **named** rather than hedged |
| **AC-005** | satisfied | `## [0.2.0-alpha.1] — 2026-08-16` at `:25`; `rg 'Unreleased'` → nothing; `lint-changelog` green after the rename | CF-29's parser survived, and this is the run that **verifies** it rather than assuming it. The `unstable-projection` claim now names both crates, read off the manifests at cut time |
| **AC-006** | satisfied | `cargo xtask ci` **whole**: `all checks passed`, transcript in `_release-log.md` §4 | **Four of four `OPTIONAL` steps RAN**, none skipped — which is the whole difference between the release bar and `--fast`'s project bar |
| **AC-007** | satisfied *(2026-08-16, after the handoff)* | Three `cargo publish` transcripts (`_release-log.md` §5.1); a scratch project outside this workspace resolving all three at an explicit pre-release, its lockfile naming `registry+…crates.io-index` with a checksum on each, and its stdout showing `committed: position=1` → `read back: 1 event(s)` → `position=1 type=SeatTaken decoded=Taken` (§5.3) | All three conjuncts discharged. Negative control observed: `"0.2"` fails to select the pre-release. See *The blocker, and how it cleared* |

### The blocker, and how it cleared

**As written at implement time**, AC-007 was a conjunction of three things and it had two.

**Discharged, and recorded before the cut.** The publish **order** is fixed with its
reason — `happenstance-core` first, because `happenstance` carries a registry requirement
on it, while `happenstance-testkit` has no ordering constraint since `happenstance`
dev-depends on it by *versionless path*, which cargo strips from the published manifest
entirely (`_release-log.md` §5). And the **yank-and-republish instruction** is written into
this story's folder **before** any cut (`_release-log.md` §1), which is the half that
becomes unwritable afterwards: three `cargo yank` commands in reverse dependency order, the
statement that a yank keeps existing lock files resolving while stopping new selections,
and the refusal to ever publish over a version.

**Missing: the act.** Three `cargo publish` invocations; a scratch project outside this
workspace resolving all three from the registry at an **explicit pre-release requirement**
(a bare `"0.2"` will not select a pre-release); the rendered crates.io page; a docs.rs check
within the hour. `_release-log.md` §5 is the four-item checklist whoever publishes fills in,
and the ledger row is flipped only when those four are in the file.

**Nothing was stubbed to hide this.** No test was skipped, no evidence was summarised in
place of a transcript, and the ledger row read `satisfied: false` with the missing
dependency named.

**Cleared, 2026-08-16.** All four checklist items are now in `_release-log.md` and the
ledger row is `satisfied: true` against them.

- **The act**, §5.1 — three `cargo publish` transcripts pasted verbatim. The middle one
  *proves* the ordering argument rather than restating it: verifying `happenstance`
  printed `Downloaded happenstance-core v0.2.0-alpha.1` and compiled against it, which is
  cargo resolving the requirement from the **registry** and is only possible because core
  went first. `happenstance-testkit` compiled `happenstance-core` and **not**
  `happenstance`, confirming NF-006/CF-32 from the other side.
- **The resolution and the cycle**, §5.3 — a `cargo new` project outside this workspace
  with no `[workspace]` above it, no `[patch]`, no `path =` and no user-level cargo
  config, built after `cargo clean` *and* deleting `Cargo.lock` so the resolution is
  fresh. Of 24 packages in the regenerated lockfile the only one without a registry
  source is the scratch crate itself, and the substring `path` appears **0** times in the
  file. The program commits one event through the command loop, reads it straight back
  out and asserts the reader's position is the writer's. `happenstance-testkit` is
  linked and *used*, not merely resolved. Negative control observed: a bare `"0.2"`
  requirement fails with `candidate versions found which didn't match: 0.2.0-alpha.1`.
- **The rendered page**, §5.4 — read as crates.io's own HTML rather than as the markdown
  source, because AC-003 is about the output. `<h2 id="user-content-stability">` renders
  above the first `<pre>`, the designed region order is intact, and *facade* appears 0
  times.
- **docs.rs**, §5.5 — green. EC-006 not triggered.

**And one thing read back badly**, §5.6 — recorded rather than omitted, assessed against
EC-007, deliberately not yanked, and routed to HS-P0016.

### The finding — the release story's own EC-001, caught locally

The version move alone stopped the workspace resolving:

> `error: failed to select a version for the requirement happenstance-core = "^0.2.0"` ·
> `candidate versions found which didn't match: 0.2.0-alpha.1` ·
> `required by package outside-projection-adapter`

`examples/outside-projection-adapter/Cargo.toml` writes its requirements long-hand on
purpose — it models an outside author's manifest — and was not in the spec's fenced list.
**This is exactly EC-001's failure class**, arriving locally and for free instead of on the
*second* `cargo publish`, after `happenstance-core` was live and unremovable and a new
version number was the only way out. That is why AC-001 makes the `cargo metadata` read a
criterion rather than a step, and it is the single most useful thing this story did.

Fixed with two lines in a `publish = false` example, so no published artefact changes. The
boundary widening is declared rather than absorbed.

### The presentation constraint that bit

AC-003 requires the first code block to sit **no lower than it does today** (`:30`). The
first draft of `## Stability` — three paragraphs — put it at `:38`, which is the failure a
reviewer sees and no assertion does. The section was compressed to a three-item list; the
fence is back at exactly `:30`, the blockquote's removal paying for the new section, which
is what *deleting* rather than annotating it was for. The mechanism a reader may want next
— a yank keeps existing lock files working — is one hop away in `_release-log.md` §1 rather
than a fourth claim the design's three-item cap forbids.

### Mount point

**`Cargo.toml`'s `[workspace.package] version`** — the single root that decides what
`happenstance` and `happenstance-core` publish as, inherited at each crate's `:4`. The
registry artefact does not exist until that line moves, and it has moved. Co-mounts, all
live: `Cargo.toml`'s three `[workspace.dependencies]` requirement strings,
`crates/happenstance-testkit/Cargo.toml`'s CF-32 number, `crates/happenstance/README.md`
(the `crate-readme` surface, compiled as a doctest through
`crates/happenstance/src/lib.rs:10`) and `CHANGELOG.md`.

### Two red gates, both repaired rather than tolerated

`lint-constitution` reported two citations whose anchors moved when this story added comment
lines to `Cargo.toml` and the testkit's manifest; re-anchored in their own commit, following
this branch's precedent, and `27 atoms, all consistent`. Earlier in the slice
`cargo deny check bans` was already red on this tree before the release story began, and was
repaired inside `edge-flavour-and-wasm-claim`'s fence — a release story is the worst place to
discover a gate that was never green.

### Nothing deferred that could have been done

No `crates/*/src/**` path is in the diff, so no public item was added, removed or gated — a
release story that grows a feature flag has changed the artefact it was cutting. `Cbor`
ships: `cargo deny check` passed against the **unmodified** allowlist, so EC-003's fallback
did not fire. No `.github/workflows/` change, no MSRV promise, no `cargo-semver-checks`
verdict, no registry presentation — all HS-P0016's, and this release creates the baseline
they will diff against while making none of the promises.
