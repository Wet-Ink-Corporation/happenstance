# Pre-publication review — durable material for the knowledge base

Staged 2026-09-03. Source: [`references/evaluation/review-pre-publication-2026-09-03.md`](../../references/evaluation/review-pre-publication-2026-09-03.md), pinned to `56ef6c5`.

This brief exists because the review document itself is **dated evidence and
ranks below an atom** (`standards/rust/README.md:23-43`). What follows is the
part of it that is *not* dated — mechanisms, transferable practice, and
questions that are now owed an owner. It authors no atom and proposes no ADR
text; the ingest wave adjudicates, and the RUNBOOK's ADR queue decides.

Every claim below is backed by a `results/raw/` file in the named experiment or
by a `path:line` at the pinned commit. Where a claim is a *measurement*, it
carries the conditions it was taken under, because none of them is portable.

---

## Candidate reference atoms — mechanisms that will outlive these findings

### R-1. `cargo test --list` cannot observe `#[ignore]`

`xtask/src/proof.rs:29` rests its argument on *"the names are asserted, out of
`--list`, before the tests run."* Measured in
[`experiments/gate-vacuity/`](../../experiments/gate-vacuity/): libtest's
discovery output is **byte-identical** with and without the attribute
(`results/raw/list-diff.txt` is empty). So an anti-vacuity check built on
`--list` cannot see a silenced test, and all 31 named proof tests can be
`#[ignore = "…"]`-ed with `cargo xtask ci` running all 33 steps and exiting 0,
four of nine artefacts then running zero tests.

The bare `#[ignore]` spelling is caught — but by `clippy::ignore_without_reason`,
a **pedantic** lint nothing in the gate's documentation mentions, and the
spelling clippy's own help text recommends survives it.

**Why it is durable:** it generalises past this repository. Any check that
asserts a test's *existence* as a proxy for its *execution* has this hole, and
the fix is to assert on the run's own output rather than on discovery.

### R-2. Cloning an `Event` costs `t + 2` allocations, and the cheap benchmark spelling cannot see it

`Tags` is `Box<[Tag]>` (`crates/happenstance-core/src/tag.rs:281`) and `Tag` is
`Cow<'static, str>` (`:79`). `Tags::from_pairs` → `Tag::key_value` → `Tag::new`
yields `Cow::Owned`, which clones by allocating; `Tag::from_static` yields
`Cow::Borrowed`, which clones free.

Measured in
[`experiments/event-clone-allocations/`](../../experiments/event-clone-allocations/),
at VT-22's 64-tag floor, `Bytes::from_static` payload:

| tags | via `Tags::from_pairs` | via `Tag::from_static` (control) |
| ---: | ---: | ---: |
| 0 | 1 alloc | 0 |
| 8 | 10 | 1 |
| 64 | **66 allocs / 2,001 B** | **1 alloc / 1,536 B** |

Four sites in the corpus state the cost as two, and one of them is ES-17's own
rationale. The `from_static` arm is **flat in tag count**, and it is the arm a
fixture author writes without choosing to — so a benchmark built the natural way
reports the clone immaterial regardless of the truth.

**Also found and not claimed by any finding:** `Bytes::clone` is a refcount bump
*from the second clone onward*; a payload built with `Bytes::from(Vec<u8>)` —
the shape every decoded payload has — allocates a shared header on its first
clone. One extra allocation per payload for any store that clones each appended
event once.

**Correction to a reasoned figure:** on `x86_64-pc-windows-msvc` at 1.97.1,
`Cow<'static, str>` is **24 bytes, not 32** — `String`'s capacity field carries
the discriminant. So `Tag` and `EventType` are 24, `Tags` 16, `Event` 104,
`SequencedEvent` 144, `QueryItem` 32. A layout budget written from the derived
numbers would have failed on its first run.

### R-3. The busy-timeout margin, measured, and the core axis runs backwards

[`experiments/busy-timeout-margin/`](../../experiments/busy-timeout-margin/).
Worst per-contender wait at the shipped `CONTENDERS = 64` in the gate's own
configuration: **3,628 ms of the 5,000 ms budget, a margin of 1.38x**; worst
cell anywhere 3,828 ms (1.31x) at 8 cores. Defensible claim: **1.3x–1.4x on the
plateau**.

Two mechanisms that a reasonable person would predict are **refuted**:

- **Fewer cores is safer, by ~450x.** 1 core → 8 ms, 2 → 628, 4 → 2,628,
  8 → 3,828, 20 → 3,628. SQLite's handler is a back-off *poll*, not a queue, so
  the pathology needs contenders simultaneously runnable; one core forbids it.
  Constraining cores measures the *safest* cell in the matrix.
- **The build profile barely enters.** Debug (3,628 ms) sits inside release's
  band (3,228 / 3,328 ms). The time is spent asleep, so `--release` has nothing
  to recover.

Conditions, because none of this is portable: i9-13905H (14c/20t), Windows 11,
NTFS/NVMe, rustc 1.97.1 msvc, SQLite 3.53.2, `wal` + `synchronous=normal` read
back off the live connection, on a host running ~20 other agent processes.
Every `wait_ms` is a **lower bound** — the counting handler resolved the same
race ~2.6x faster than SQLite's own, uncalibrated, with one of five ratios
inverted. Storage was never varied, and storage is what a busy handler
ultimately waits on.

### R-4. `park`/`unpark` carries one token per *thread*, so nested `block_on` loses wakeups

`crates/happenstance-testkit/src/registry.rs:338-342` parks with no notified
flag. `concurrency.rs:890` → `:980` nests a second `block_on` inside a rule
already driven by one. Measured deterministically over four runs
(`experiments/busy-timeout-margin/tests/lost_wakeup.rs`): baseline completes,
nested-without-collision completes, nested-with-collision **hangs past 10 s**.

**Why it is durable:** CF-33 `[FROZEN]` forbids the suite a watchdog, so a hang
names no rule — and this workspace now has *two* mechanisms that produce a
stopped CI job naming no rule. `crates/happenstance-sqlite/tests/concurrency.rs:45-47`
currently instructs the reader that a hang is evidence about the busy timeout,
which is sound only while it is the sole candidate.

---

## Candidate playbook — verifying a citation says what the citing sentence claims

`cargo xtask spec-trace` verifies that a cited location **resolves**. It cannot
verify that the location **says what the citing sentence claims**, and
`references/evaluation/review-citation-drift.md` §1 already recorded that gap
recurring once after phase 2 closed it.

This review found a fresh instance, in the document it modelled its own format
on: `references/evaluation/phase-7-contract-defects.md:10` cites
`README.md:83-85` for the lifecycle's citation-repointing rule; `:83-85` is the
*erratum* exception, and the rule is at `README.md:228-230`. It resolves, so
nothing catches it — and because every later evaluation document copies that
header, the error propagates by imitation rather than by edit.

The repair is the **one permitted in-place change** the sentence itself
describes, so this needs no superseding document.

The transferable practice: an anchored citation (`path:line (anchor-text)`, the
form `xtask/src/lint_constitution.rs` already enforces for
`standards/rust/`) is checkable where a bare `path:line` is not. The
`standards/` corpus has this; `references/evaluation/` and `spec/` do not.

---

## Candidate governance atom — what may refute a finding

Derived from `standards/rust/README.md:23-43` and RS-01-3, and worth stating as
an atom because this review had to derive it and got it wrong first.

The precedence ladder is SPECIFICATION clause > accepted ADR > constitution atom
> `CLAUDE.md` > `references/evaluation/*`. Three consequences that are easy to
miss when someone is auditing the tree:

1. **`references/evaluation/*` and `RUNBOOK.md` may annotate a finding and may
   never refute one.** They are dated evidence, not rules. A review that lets
   them refute suppresses true findings using documents the repository itself
   declares non-binding.
2. **A refutation needs a quoted sentence answering the same question.** A
   filename is not a refutation; a section number is not a refutation. Absence
   of a citation in a ~40,000-line corpus is a search failure, not a verdict.
3. **An ADR's currency is computable, not a matter of judgement.**
   `git log <adr-commit>..HEAD -- <the cited files>`: empty means the decision
   was taken against exactly this code and refutes; non-empty means it refutes
   only if a line at HEAD still implements it. Where that line has moved or
   vanished, the right verdict is **an accepted decision may have silently
   drifted** — which is the highest-value class available and the one a naive
   "the ADR covers this" lens discards. Four candidates surfaced under this rule
   in this review. Specification clauses are exempt: `spec-trace` is a gate step,
   so their citations resolve at HEAD by construction.

---

## Questions now owed an owner

Named, not answered. Each has evidence attached and none has a number in
`RUNBOOK.md:262`'s ADR queue.

1. **Does `Event::metadata` get a floor, share one with `data`, or is it
   declared deliberately unbounded?** It is a fourth opaque payload with no
   `MIN_SUPPORTED_*`, no `StoreLimit` variant and therefore no conformant
   refusal (VT-25 `[FROZEN]` forecloses `AppendError::Store`). `happenstance-cloudflare`
   is already carving its margin out of `data` to compensate. ADR-0015 is
   accepted and immutable, so this needs a superseding atom.
2. **Is a read page budgeted in rows, in bytes, or by the caller?** Measured:
   one page of 512 ceiling-sized events is 512.2 MiB resident; raising
   `PAGE_SIZE` is the measured win for aggregate lock time (2,929 s → 99 s over
   a 10⁶ replay) and is exactly what the residency says is unsafe. One knob, two
   consequences, not co-optimisable.
3. **Does the testkit grow a fixture-declared tolerance for transient
   contention?** A busy store and a broken store are currently the same
   `Attempt`, and CF-33 `[FROZEN]` guarantees the suite cannot tell them apart.
   ADR-0022 §12 says the contender count is not its call.
4. **Does an adapter taking VT-6's documented-procedure branch owe an in-process
   check?** `remint_identity`'s precondition is trust-only even for the case a
   program can see.
5. **Does the query plan chunk on bound parameters as well as UNION arms?**
   `PARAMETER_BUDGET` exists, carries the arithmetic, and has one caller where
   it should have three.

## Falsifiers that fired, and should be recorded as fired

- **ADR-0022 §11** — *"re-open if any run ever reports `busy > 0`"*. `busy > 0`
  observed at the shipped `CONTENDERS = 64`, one launch in seven. First nonzero
  busy count anywhere in this tree. Note precisely: `busy=1` and `busy=2` with
  `exhausted=0` — attempts that *entered* the handler, not attempts that ran the
  budget out.
- **ADR-0022 §9** — *"re-open if a deployment shows the captured `Handle`
  costing something the inline path does not."* The capture is unconditional and
  irreversible, and makes `NoRuntime` unreachable for a store that outlives its
  construction runtime.
- **ADR-0022 §16's falsifier for §8** cannot fire as written, because the shape
  it names to re-open on — the `GROUP BY … HAVING COUNT(DISTINCT tag)` aggregate
  — is not what the adapter emits. Measured, the chain that does ship loses to
  that aggregate in 9 of 9 two-tag cells.
