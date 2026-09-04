# Pre-publication remediation — handover

**Branch:** `remediation/pre-publication` · **71 commits** over `main` ·
46 files, +5730 / −231
**Source:** `references/evaluation/review-pre-publication-2026-09-03.md` — 96 finding
IDs across 84 entries, pinned to `56ef6c5`
**Last verified:** 101 test targets, **2,177 passed, 0 failed**; fmt, clippy
`-D warnings`, and all eight `xtask` checks green; every fence holds.

---

## Scoreboard

| | Count | |
|---|---:|---|
| **Closed** — code landed, gate green | **20** | `S-1` `S-2` `S-3` `RV-1` `RV-2` `RV-3` `F1-04` `C2-07` `P-4` `U-3` `T1` `AE-5` `H2` `G-1` `V-4` `V-5` `V-6` `F1-05` `D-1` `D-4` |
| **Briefed** — decision authored, code awaits ratification | **26** | see `.kb/_intake/remediation-2026-09-04-briefs/` |
| **Untouched** | **50** | listed below |
| **Found by us, not in the audit** | **3** | SQLite default-features; repository URL 404s; `SECURITY.md` scope |
| Audit's "unopened" items closed | 4 of 7 | #1 CI · #2 re-exports · #5 suite cost · #6 registry facts · #7 runners |

---

## What is done, and why these first

**The instruments, before anything else.** Every later claim of "the gate is green"
was worth less until these landed.

- `S-1` — the gate could be silenced by `#[ignore = "…"]` on all 31 named proof tests
  while exiting 0. `proof.rs` now reads the run's own report, not `cargo test --list`,
  which the `gate-vacuity` experiment measured as byte-identical either way.
- `S-2`/`S-2b` — the story grain dropped `lint-rule-counts`. It now derives its set
  from the gate's step table with a **named exclusion list**, so a new file-reading
  check must be *decided about*. This proved itself in the wild: a different agent in
  a different lane added `lint_workflows.rs` and the check forced it onto the grain.
- `F1-04` — ES-13's `[FROZEN]` pin was a `compile_fail` fence in an integration-test
  target, which cargo never hands to a compiler. Proven inert (breaking the fenced
  code changed nothing), moved into the lib, verified running by doctest count.
- `RV-2` — a `HashMap`-bucket-dependent counterexample reddening the gate ~1 run in 126.
- `RV-3` — `experiments/` was inert to `affected` and indexed by `spec-trace`.

**Supply chain** (`UNOPENED-1`): `permissions: contents: read`; **19/19 actions pinned
to 40-char SHAs** with versions in trailing comments; `dtolnay/rust-toolchain@master`
— a *branch* — gone; new `lint-workflows` gate step so it cannot regress.

**Driver re-exports** (`D-1`/`D-4`, ratified): `futures_core` in core; `rusqlite`,
`tokio`, `happenstance_core` in sqlite; `worker` + `happenstance_core` in cloudflare;
`happenstance_core` in the typed layer. Closes the audit's unopened item #2 —
`pub use happenstance_core;` previously appeared in **no** crate.

---

## Three defects the audit never had

1. **`cargo test -p happenstance-sqlite` does not compile under default features.**
   `tests/migration.rs:560` reaches `projection_store::` while `default = ["event-store"]`.
   The crate asserts the opposite in two places — `tests/conformance.rs:41` says the
   flagless command *"is therefore the whole command"*. **Fixed.** Invisible to every
   gate step because they all pass `--all-features`, and `cargo hack check` has no
   `--all-targets`.
2. **Three published crates point `repository` at a URL that 404s anonymously.** The
   org resolves (200); the repo does not — i.e. it is private. `SECURITY.md`'s only
   vulnerability-reporting channel is on it. **Not fixed — yours to decide** (publish
   the repo, or give `SECURITY.md` an out-of-band contact).
3. **`SECURITY.md` lists `happenstance-sqlite` as published** at its "published
   versions"; the registry says `0.0.0`. The document already handles this exact case
   for `happenstance-cloudflare` three lines below. **Not fixed.**

Also measured (`UNOPENED-5`): a stranger's first `cargo test` on the falsifier crate
is **≈14 s**, `happenstance-sqlite --all-features` **≈60 s**, and the 112-rule macro
contributes **≈1.3 s**. The audit's fear was ten minutes. The moat is the ordinary
dependency graph, not the macro.

---

## The 50 untouched, grouped as the audit groups them

```
Typed-layer shapes         B-1 B-2 R-2 Y-5
FROZEN unmet by adapters   Q-01 X-3 Q-02 F2-5
Ceilings in wrong unit     X-1 X-2 R-1 J-5
SQLite under load          R-3
Wrong stores pass 89 rules L1-1 L2-01 F1-02 M-4
Adapter-author paths       C2-03 M-3 M-5 C2-01 C2-05 V-3 M-2
Rendered surfaces          C2-02 F1-01 O-1 P-5 O-4 N-1 U-2
Unshipped crates           G-2 X-4
Cost numbers               I-5 AE-2
Checks stronger than mech. S-5 Q-03 Q-04 F1-03 K1 K2
Suite self-reports         L1-3 L3-02 L3-04 L3-03 L2-03 L1-4 L2-04
Rendered examples          P-3 Y-6
```

**`X-3` is the one to do first** and it is decision-free: `unsigned_abs()` decodes a
stored position, so a corrupt negative maps onto its positive twin and forges a
duplicate position **and** a duplicate `EventId`. The correct rejecting decoder is
already written one file away at `projection_store.rs:892-902`. **The audit undercounts
it** — it cites `row.rs:105,116`; there are **seven** sites in `happenstance-sqlite`
(`row.rs:105,116`, `event_store.rs:671,749,1077,1250`) plus one in
`happenstance-cloudflare` (`event_store.rs:950`), and `event_store.rs:1077`
additionally swallows the error and returns `None`.

`L1-1`, `L2-01`, `L1-2`, `L3-01` have their **Red pre-written**:
`experiments/suite-against-wrong-adapters/src/stores.rs` holds four wrong stores that
pass all 89 rules, written in the testkit's own `MutantStore` idiom and citing the
mutants file they would be promoted into. Promote, register in `REGISTRY`, watch
`every_rule_has_a_mutant` go red, then write the rule.

---

## How to run this (the method, and what it cost to learn)

**Worktree per lane, one writer per path.** `../happenstance-remediation` is the trunk;
`../hs-prose` and `../hs-supply` are lanes off it, merged back by hand. Lanes run in
parallel; findings *inside* a lane run strictly sequentially.

**Per finding:** read only that entry (`Read` with `offset`/`limit`) plus 1–3 named
constitution atoms · Red committed alone and proven to fail · implement · **revert-proof**
(revert only the implementation in a scratch worktree; the test must still fail) ·
scoped gate · state your own cheapest self-refutation · adversarial refuter that
**re-executes** and defaults to `refuted: true` when uncertain · judge runs the whole
gate once per wave.

### Five things that will bite you, all learned the hard way

1. **A scoped gate must be a prefix of the real gate.** I omitted clippy (one wave
   landed red and three findings stacked on it), then fmt (gate halts at step 1), then
   the doc-density budgets in `crates/happenstance/src/tests.rs`. Minimum:
   `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features --
   -D warnings`, the package's own tests, **and** `cargo test -p happenstance --lib` if
   you touched its docs.
2. **`cargo hack --no-dev-deps` rewrites all 13 manifests in place** and restores them
   on exit. Kill it mid-run — the 10-minute foreground cap does this — and the tree is
   left stripped of `[dev-dependencies]`, after which every compile-based check fails
   for an unrelated reason. It cost one judge its entire verdict. **Always run
   `cargo xtask ci` detached, and `git checkout -- .` before believing anything after
   an interruption.**
3. **Citation drift is the dominant cost of working here.** It broke the branch three
   times from our own edits: F1-04's +146 lines (4 citations), H2's +259 lines (10),
   and the audit's own commit staling `RV-1`'s proposed remedy. **Repoint by anchor,
   never by offset** — in the ten-citation case eight moved +14 and one moved +22, and
   the modal offset would have left that one wrong *and green*, still inside the
   checker's 10-line tolerance. There is a repointer recipe in commit `a4ac713`:
   parse `lint-constitution`'s own output, find the anchor it reports missing, rewrite
   the number. `--write` does **not** do this; it regenerates the index.
4. **Never verify inside a worktree an agent holds.** I did, got a false "12/12
   falsified" reading off a mid-edit tree, and had to withdraw it. That is the audit's
   own correction C-8.
5. **Keep schemas flat.** Deeply nested `additionalProperties: false` schemas blew the
   StructuredOutput retry cap and killed two workflows. The single richest result of
   the session came from an agent with **no schema at all**.

---

## Open judgements — yours, not mine

- **`repository` 404 + `SECURITY.md` scope** (above). Publish the repo, or add an
  out-of-band security contact.
- **`tokio` in the re-export set.** Landed, but it is the weakest member: sqlite takes
  it at `features = ["rt"]`, so a consumer reaching tokio *only* through the re-export
  gets a partial tokio. The caveat is written at the site and in `RS-40-4`. Removing it
  is a one-line change and free until `0.2.0`.
- **`S-2b` and `S-3` remain refuted** on a paraphrase bound — a *reworded* falsehood
  evades a text-matching check. I ruled them bounded residuals rather than iterate a
  third time. The durable answer is **generation, not pinning**: regenerate the claim
  from the fact and compare whole, the mechanism `SPECIFICATION.md §7.2` and the
  constitution index already use. That converts published prose into generated content
  on a live crate — a design decision, deliberately not slipped in.
- **`C2-07b` likewise**: its refuter defeated the pin by moving the falsehood one blank
  line *closer* to the true sentence. Coexistence, not paraphrase. Same routing.
- **`--all-targets` on the feature powerset stays OFF.** Measured: **68 of 214
  configurations fail** — 64 in `happenstance` (test code assuming its crate's default
  features, a defect class 16× larger than the sqlite instance) and 4 in sqlite, now
  fixed. It cannot be added green, and a step softened with a skip list is worse than
  none (RS-80-2). The number, date and toolchain are recorded in `migration.rs`'s header.

---

## Not done, and deliberately

No ADR written. No `.kb/decisions/` atom created or edited. No `redkiln` command run.
No `.bklg/` frontmatter touched. Nothing published. `references/evaluation/` has exactly
one change — `RV-1`'s citation repoint, which is the one in-place change that tree's own
lifecycle permits.
