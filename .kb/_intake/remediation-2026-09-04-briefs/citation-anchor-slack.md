# Should `lint-constitution` require an exact anchor match, now that a third of the constitution's citations are green only on `ANCHOR_SLACK`?

Decision record: **GATE-02-anchor-slack**. Brief only — no ADR prose, no atom, no
change to the checker. Found while repairing a *different* defect in the same step
(`Span::rfind_path_colon` recognised a citation by looking for a slash, so every
citation into a repository-root file was silently skipped); **not in the audit**,
which did not examine the citation checker at all.

The blind spot is fixed and seventeen dead citations are repointed. This brief is
about the tolerance that the fix left standing, and it is owed because the fix is
what made the tolerance measurable for the first time.

---

## Why this is owed

`ANCHOR_SLACK` is the constant that decides whether a citation is stale.
`xtask/src/lint_constitution.rs:105-111`:

> *"How far from its stated line an anchor may sit before the citation is stale.
> Not zero: a citation that has to be re-numbered for every edit above it is a
> citation people stop maintaining. Ten lines is enough to survive ordinary
> editing and far too little to survive a function moving, which is the drift this
> catches."*

That rationale has never been checked against the corpus it governs. The whole
argument is a prediction about a distribution — *ordinary editing* stays inside
ten lines, *a function moving* does not — and nobody has ever taken the
distribution. This brief takes it.

The reason it matters is the same reason the slash bug mattered. **A green that is
read as coverage and is not coverage is worse than no check**, because it retires
the reader's own vigilance. `standards/rust/README.md:118-119` describes this step
to every reader of the corpus as checking that *"citations resolve to a line that
still contains their anchor"* — which is not what it checks, and is not what it can
promise at a tolerance of ten. This repository's remediation has been broken three
times by citation drift.

## What is true today

**Measured 2026-09-04 against `lane/citation-blindspot` at `cc9f21a`**, by
re-implementing the checker's own window arithmetic (`:723-724`) over every
citation in `standards/rust/`, after the seventeen repairs. The corpus is
27 atoms and **323 citations**.

| | count | share |
|---|---|---|
| the cited line **contains** the anchor | 222 | 69% |
| the cited line does **not**, and a nearer line inside the window does | **101** | **31%** |

The 101 are green. Every one of them is a citation that has already drifted and
that the gate has already declined to report.

**The offset distribution**, nearest occurrence minus cited line:

```
-8: 1   -3: 2   -2: 1   -1: 15    0: 222    +1: 15   +2: 7   +3: 4   +4: 17
+5: 9   +6: 1   +7: 5   +8: 5    +9: 17   +10: 2
```

Three things in it that the rationale did not anticipate:

- **The distribution is not concentrated near zero.** It has a second lobe at
  +4 (17 citations) and a third at +9 (17 citations). Drift is not decaying with
  distance inside the window; it is piling up against the wall.
- **Two citations sit at exactly +10**, one inserted line from red:
  `13-sealing-and-exhaustiveness.md:97` → `crates/happenstance-core/src/query.rs:194
  (pub fn items)`, actually at 204, and `40-public-surface-and-evolution.md:166` →
  `crates/happenstance-core/src/event.rs:406 (not public API)`, actually at 416.
  Twenty-five are at `|offset| ≥ 8`.
- **The worst-maintained atom is the one about the gate.** `80-the-gate.md` is
  **13 of 14** citations stale-but-green; `81-checks-that-cannot-be-types.md` is
  9 of 14. The atoms that document the instruments are the atoms whose citations
  the instruments have stopped holding.

**The wrong-occurrence hazard is real, and it was realised.** Eleven citations had
their anchor occurring more than once inside the window. Three of them were
pointing at a line that did not contain the anchor at all while a *different*
occurrence sat inside the tolerance, and were repaired in this lane:

- `40-public-surface-and-evolution.md:167` → `event.rs:426 (non_exhaustive)` — the
  attribute is at 435 and its doc comment at 432; 426 is a closing brace.
- `50-dependency-hygiene.md:165` → `deny.toml:10 (version = 2)` — `[licenses]
  version = 2` is at 9 and `[advisories] version = 2` at 5; 10 is `allow = [`.
- `60-what-a-test-must-prove.md:270` → `mutants.rs:266 (defect: PhantomData)` —
  the field is at 267 and the initialiser at 275; 266 is the line above.

Each was green for as long as it was wrong. That is the failure mode stated in
the abstract in `81-checks-that-cannot-be-types.md`'s RS-81-1, occurring in the
checker that atom is cited by.

**Demonstrated, not argued.** In a scratch worktree at `cc9f21a`, moving
`00-prime-directives.md`'s `Cargo.toml:160 (unsafe_code = "forbid")` to
`Cargo.toml:170` — a deliberately false line — leaves the step printing
`27 atoms, all consistent`.

**The anchor is often not discriminating on its own.** Of the 101 stale-but-green
citations, **25** have an anchor that occurs more than once in the target file, so
the line number is the only thing pinning them. The extreme case is
`01-standard-of-evidence.md:81` → `mutants.rs:432 (impl Defect for)`, whose anchor
occurs **66 times** in that file. Any option that weakens the line number in favour
of the anchor has to answer this.

**A sibling checker disagrees about the number and nobody noticed.**
`xtask/src/spec_trace.rs:395` sets its own `const ANCHOR_SLACK: usize = 12`, for the
same job over `spec/SPECIFICATION.md`. Two instruments, two tolerances, one
undocumented difference.

## Options

### Option A — Exact match: the cited line must contain the anchor

Set the window to zero. Costs a one-time repointing of 101 citations — 76 of which
have a unique anchor in their target file and are therefore a mechanical
`grep -n` away; the other 25 need a human to choose the occurrence, which is
exactly the judgement the slack currently hides.

The standing cost is the one the rationale names: every insertion above a cited
line turns the gate red, and the repair is renumbering. Against that, `--write`
already exists on this subcommand for the router's generated region, so a
`lint-constitution --repoint` that rewrites unambiguous citations from their
anchors is a small, obvious extension — and where the anchor is ambiguous it must
*refuse*, because that is the case a slack silently guesses at.

### Option B — Keep the window, and report the offset

Leave the tolerance and make the step print the drift it is tolerating: *"`Cargo.toml:102`
is 58 lines from `unsafe_code = "forbid"` — inside tolerance"* becomes visible
output rather than nothing. Cheapest possible change, and it converts a silent
allowance into a stated one, which is what `RS-80-2` asks of a probed step.

It does not fix anything. A number nobody has to act on is a number nobody reads,
and this repository has 101 of them ready to print on every run — noise that will
train a reader to skim the step's output, which is how the *real* findings stop
being seen.

### Option C — Exact match, with the anchor as the repair, and no window at all

Option A plus: on a miss, the checker searches the whole file for the anchor and
*names the line it should be*. A wrong citation then costs one keystroke to repair
rather than a `grep`, which is the whole of the maintenance objection. Where the
anchor is not unique the message lists the candidates and refuses to choose.

Strictly more work than A, and it is A's argument carried to its conclusion: the
reason a citation carries an anchor at all is so that the line number is
*recoverable*, and a checker that can recover it should say so instead of shrugging
within ten lines.

### Option D — Tighten rather than close: `ANCHOR_SLACK = 2` or `3`

Splits the difference and repoints 63 citations at `2`, or 57 at `3`, instead of
101. It has no principle
behind it, which is its whole problem: the number would be chosen to fit the
distribution measured today, and the next measurement moves it again. It is worth
naming only because it is what a hurried reading of the table above suggests.

## Recommendation

**Option C**, staged: land the repointing of all 101 first as its own change, then
close the window and add the anchor-search repair message in a second. Reconcile
`spec_trace.rs`'s 12 to the same rule in the same pass, or write down why it
differs.

The staging matters more than the choice. A change that closes the window *and*
repoints in one commit is 101 line-number edits inside a diff whose interesting
part is four lines of `xtask`, and no reviewer will read it.

### The strongest argument against, in its own words

*The slack's rationale is right and the measurement proves it. 101 citations
drifted and none of them ended up pointing at the wrong thing — the check did its
job, which was to catch a function moving, and it caught nothing because nothing
moved. Closing the window buys precision nobody needs and imposes a red gate on
every contributor who inserts a paragraph above a cited line. The corpus is
documentation; ten lines of imprecision costs a reader one scroll.*

Two of its three claims are false, and the measurement is what falsifies them.
**Three citations did end up pointing at the wrong thing** — a closing brace, an
`allow = [`, and the line above a field — and each was green. And the drift is not
"nothing moved": the workspace lint table moved **+58** lines, and the check
reported nothing, because the citations into it were in a file with no slash in its
name and were never examined at all. That last point is the one that decides it:
the argument for the slack was always *"it catches the drift that matters"*, and
the drift that mattered most in this corpus was drift the step could not see. Its
green was never evidence about the tolerance; it was evidence about the blind spot.

The third claim — that a red gate on an ordinary insertion is a real cost — is
true, and it is the reason the recommendation is C and not A. The cost is paid in
the repair, and C makes the repair one keystroke.

## Cost of delay

Low and slowly compounding, with one asymmetry. Nothing is published from
`standards/rust/`, so no consumer is misled today. But the 101 grow: two are already
at +10 and will turn red on an unrelated edit, at which point a contributor with no
context repoints one citation and learns nothing about the other hundred. The
cheap moment is now, while the measurement exists and the reason for it is one
commit away.

The asymmetry is that the *reader's* trust in the step is spent whether or not the
decision is taken. Every time someone reads "27 atoms, all consistent" and takes it
for what `README.md:118` says it means, the eventual correction gets more expensive.

## What this does not settle

- **Whether `spec-trace`'s window should move with it.** Its subject is a
  specification with clause ids rather than a corpus of Rust anchors, and 12 may be
  right there for reasons nobody has written down. Same question, different corpus,
  and it needs its own measurement.
- **Whether `standards/rust/README.md:118-119` should be corrected in the
  meantime.** It currently describes a stricter check than the one that runs. That
  is a one-sentence edit and it is deliberately not made here, because which
  sentence is true depends on which option is taken.
- **Whether an anchor should be required to be unique in its target file.** The
  `impl Defect for` case (66 occurrences) suggests some anchors are not doing the
  job an anchor exists to do, and no option above makes that a rule.
- **What `--repoint` should do about a citation whose anchor has been deleted
  outright.** One was found in this lane —
  `52-wasm32-and-target-cfg.md`'s `.github/workflows/ci.yml:198`, whose anchor text
  was removed at `8ea7bb7` — and no mechanical repair can find a line that is not
  there. It needs a human, and the checker should say so rather than guess.
- **Whether a `--repoint` must be idempotent, and whether a content check is
  enough to make it so.** *(Added by the query-ceilings lane, 2026-09-04, and
  not part of this brief's own two-critic pass.)* That lane repointed 58
  citations with a script that derived each new line from `git diff -U0` hunks
  and verified it by comparing the cited line's content before and after.
  Running it a **second** time re-shifted eleven of them, and the content check
  did not stop it: the already-corrected number was compared against a line whose
  content happened to be identical — `    }`, a blank line, a bare `///` — so the
  guard passed and the shift was applied twice. The repair had become a
  corruption, and it was caught by re-reading the output rather than by any
  check. Restoring from git and running exactly once is the discipline that
  worked; whether a shipped `--repoint` can do better than that discipline is a
  question this brief's options do not reach. It is the same shape as the
  tolerance measured above — *a check that passes on a coincidence* — which is
  the third form this effort has met it in, and the third argument for the exact
  option. The account is in `query-partition-public-surface.md`.
