# Pre-publication remediation — handover

**Branch:** `remediation/pre-publication` · **119 commits** over `main` ·
109 files, +21740 / −490
**Source:** `references/evaluation/review-pre-publication-2026-09-03.md` — 96 finding
IDs across 84 entries, pinned to `56ef6c5`
**Last verified at `fce95e2`:** `cargo xtask ci` **exit 0**, all 35 steps run and
none skipped — including the four probe-gated ones, at 214 and 186 feature
configurations, `cargo deny`, and the nightly `--cfg docsrs` build. 116
conformance rules, event-store family 91→93. `cargo hack` restored all 13
manifests (`git status` clean).

> **Read the exit code, not the notification.** Two "completions" in the second
> wave were false: a background wrapper reported its own exit rather than the
> gate's, and a liveness monitor using Git Bash `pgrep` declared a running gate
> dead because `pgrep` there cannot see native Windows processes. Both looked
> exactly like a result. Append `echo "EXIT $?"` to the log and read the log.

---

## Wave two — what closed, and what it cost to learn

The five findings the second wave was asked for — **X-3, L1-1, L2-01, L1-2,
L3-01** — are closed. Five more closed because they fell out of those:
the **cloudflare position fence**, the **`tokio` re-export removal** (ratified),
the **`lint-constitution` blind spot**, the **two `to` composition rules**, and
**X-1 + X-2**.

**Six of eight lanes were sent back at least once, and the returns were the
work.** Nothing below was found by a gate.

- **Two new mutant kinds looked sound and were not.** `Kind::ModelOnlyMutant`'s
  second obligation sat behind `#[cfg(feature = "proptest")]`, so a store with no
  defect at all passed green under `--no-default-features` — the configuration
  `cargo hack` compiles. Repaired with a witness holding `<T as Defect>::select`.
  `Kind::StatedOnlyDefect` was then satisfiable by **two string literals**, its
  row holding plain `fn` pointers with no tie to the subject; the prescribed
  repair was falsified too, and it was **withdrawn** rather than patched —
  `Fixture` is not dyn-compatible under RPITIT, so the tie the soundness needed
  is not available in the language.
- **A named counterexample was wrong and the conclusion survived anyway.**
  Review proposed a store applying the row budget before the upper bound. It is
  not a defect: 4,864 evaluations, zero disagreements — in read order the bound
  and the budget are both prefix operations and two prefixes compose to the
  shorter prefix either way. The second rule is still owed, for a different
  reason: **commuting is about order; the real defect drops the budget because a
  bound is present, which is applicability.**
- **This remediation blinded a gate check and then asserted it did not exist.**
  `cargo xtask lints` is green at `c0c250e` and red by mid-wave: the
  citation-blindspot lane — whose whole subject was citation drift — added
  sixteen lines above a span `.kb/playbooks/` cites and left the cited line
  blank. `citation_ranges_resolve` runs **before** `stated_rule_counts` in the
  same step, so the rule-count census went dark behind it, which is how
  "Ninety rules" survived against a family of 91. Both repaired.

## The scoped gate — four more commands, all learned by going red

The handover's minimum was fmt, clippy, the package's tests, and
`cargo test -p happenstance --lib`. Add:

```
cargo test -p happenstance-testkit --no-default-features --test mutation_coverage
cargo test -p xtask
RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps
```

- `--no-default-features` because a mutant kind was found unsound in exactly that
  configuration and nowhere else.
- `cargo test -p xtask` because editing `spec/SPECIFICATION.md` fires
  `lint_narrative`'s documentation-obligation census — CF-17's new sentence was
  CF-39's verbatim, and the pin classified it neither way.
- **`cargo doc` with `RUSTDOCFLAGS`, and the variable is the whole point.**
  rustdoc does not read `RUSTFLAGS`, so this workspace's ambient `-D warnings`
  denies every rustc lint and **no** rustdoc one. Three rustdoc errors failed the
  full gate that no scoped gate could see. The gate's own step carries a comment
  recording that before that line existed it printed "generated 3 warnings" and
  exited 0 for as long as it ran.

**And a fifth, about running it:** `run_in_background` is **not** detachment if a
timeout still applies. A 600 s cap killed a `cargo xtask ci` mid-run; the tree
survived only because the kill landed in a compile rather than inside
`cargo hack --no-dev-deps`'s manifest rewrite. Launch it under its own process
(`Start-Process`), write the exit code to a sentinel file, and watch the file.

## Citation drift — now measured, and the rule needs a second clause

**Repoint by anchor, and audit the files your diff MOVED — not only the files you
edited by hand.** That distinction alone cost five stale citations in one lane,
one of which was *exactly right before the lane and wrong after it*. Three
further forms turned up:

- **Different ends of one span move by different amounts.** `674-722` became
  `690-735`: +16 and +13. The uniform shift puts the end inside another function.
- **A repair script that is not idempotent.** A second run re-shifted eleven
  citations because the content check passed on coincidentally identical lines
  (`    }`, a blank line, a bare `///`). Restore from git and run exactly once.
- **Green while pointing at the wrong occurrence.** Three citations named a
  closing brace, an `allow = [`, and the line above a field. Each was inside
  `ANCHOR_SLACK` and each was silently wrong.

**Coverage, counted:** only `standards/`'s ~330 citations are anchor-checked.
`references/` (~2,700), `.bklg/` (~25,700), `.kb/_intake` (~1,200) and
`spec/E2E-CASES.md` (~88) have **no citation checker at all**; `.kb/` proper is
checked for existence and non-blankness, never for anchor. `spec/E2E-CASES.md:1043`
was displaced by **this remediation's own** L1-1 lane, traced with `git log -S`.

---

## Scoreboard


| | Count | |
|---|---:|---|
| **Closed** — code landed, gate green | **27** | wave one: `S-1` `S-2` `S-3` `RV-1` `RV-2` `RV-3` `F1-04` `C2-07` `P-4` `U-3` `T1` `AE-5` `H2` `G-1` `V-4` `V-5` `V-6` `F1-05` `D-1` `D-4` · wave two: `X-3` `L1-1` `L2-01` `L1-2` `L3-01` `X-1` `X-2` |
| **Briefed** — decision authored, code awaits ratification | **22 briefs** | see `.kb/_intake/remediation-2026-09-04-briefs/` — nine added in wave two, each stating it did **not** get the two-critic pass the original thirteen had |
| **Untouched** | **45** | listed below — derived from the list, not the arithmetic; see note |
| **Found by us, not in the audit** | **6** | SQLite default-features · repository URL 404s · `SECURITY.md` scope · `lint-constitution` skipped every root-level citation · `cargo xtask lints` blinded **by this remediation** · `spec/E2E-CASES.md` citations checked by nothing |
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

## The 45 untouched, grouped as the audit groups them

> **The count is 45, not 50 − 7.** Five of the seven closed in wave two came off
> this list (`X-3`, `X-1`, `X-2`, `L1-1`, `L2-01`); `L1-2` and `L3-01` were never
> on it — they sat in the *briefed* column, blocked on the fixture-declension
> decision, and were closed once it was ratified. Subtracting seven from fifty
> gives 43 and is wrong. This number is derived by counting the ids below.

```
Typed-layer shapes         B-1 B-2 R-2 Y-5
FROZEN unmet by adapters   Q-01 Q-02 F2-5            (X-3 closed)
Ceilings in wrong unit     R-1 J-5                   (X-1, X-2 closed)
SQLite under load          R-3
Wrong stores pass 89 rules F1-02 M-4                 (L1-1, L2-01 closed; the
                           suite is 116 rules now, event-store 93)
Adapter-author paths       C2-03 M-3 M-5 C2-01 C2-05 V-3 M-2
Rendered surfaces          C2-02 F1-01 O-1 P-5 O-4 N-1 U-2
Unshipped crates           G-2 X-4
Cost numbers               I-5 AE-2
Checks stronger than mech. S-5 Q-03 Q-04 F1-03 K1 K2
Suite self-reports         L1-3 L3-02 L3-04 L3-03 L2-03 L1-4 L2-04
                           (L1-2, L3-01 closed)
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

**Two are still unanswered after wave two and both are cheap now.** The
`repository` 404 was put to the owner and came back unanswered; the `tokio`
question was answered *remove* and is landed. `ANCHOR_SLACK` has been briefed
with a measurement.

- **`repository` 404 — verified independently, 2026-09-04.** The org returns
  **200** and shows one public repo (`praecepta`); `happenstance` returns **404**,
  so it is a visibility answer and not a typo. Sharper than first filed:
  `homepage` and `documentation` are **both `null`** in the workspace manifest, so
  `repository` is not the main outbound link — it is the **only** one. A reader on
  crates.io or docs.rs who wants the spec, the ADRs, or a security contact has one
  door and it is locked, and `SECURITY.md` contains no email, no alternate contact
  and no fallback sentence. Its next paragraph tells reporters not to open a public
  issue, so the instruction it gives when its channel 404s is *silence*. Brief:
  `repository-url-and-security-channel.md`. **One paragraph closes it** and needs
  only an address. Setting `documentation = "https://docs.rs/happenstance"` is one
  line and works under either answer.
- **`ANCHOR_SLACK` — brief: `citation-anchor-slack.md`.** Measured three times
  independently: ~97–101 of ~309–323 citations are green **only** on the slack,
  the distribution does **not** decay (lobes at +4 and +9, two sitting at exactly
  **+10**), the worst-maintained atom is `80-the-gate.md` at **13 of 14**, and
  `xtask/src/spec_trace.rs:395` keeps its **own undocumented `ANCHOR_SLACK = 12`**
  for the same job. Recommendation is exact matching with the checker naming the
  correct line on a miss, staged so the repointings land apart from the four-line
  change. **One addition from wave two:** the 25 citations whose anchor is not
  unique are not an argument against anchors — an anchor matching 66 lines
  (`impl Defect for`) is a badly chosen anchor, and sharpening it is a work item
  the exact-match option surfaces rather than a cost it imposes.



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
