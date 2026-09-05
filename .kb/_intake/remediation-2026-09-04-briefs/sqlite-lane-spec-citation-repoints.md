# Ten `spec/SPECIFICATION.md` citations into `happenstance-sqlite` that this lane's diff moved and did not repoint

Record: **sqlite-ceilings-spec-citation-repoints**. Not a decision — a **handoff**, and it
is here because `spec/SPECIFICATION.md` is another lane's file and this lane was
instructed to brief rather than edit it.

**This brief did not get the author → two-critic → revision pass the original thirteen
had.** It contains no judgement to critique; every line below is mechanical and checkable.

---

## What happened

The `lane/sqlite-ceilings` lane edited three files under
`crates/happenstance-sqlite/src/` — `projection_store.rs` (X-4), `query_sql.rs` (I-5) and
`event_store.rs` (R-1+J-5, R-3) — and every addition was documentation or a test, so the
line numbers below every insertion moved. Ten `file:line` citations in
`spec/SPECIFICATION.md` point into those files and are now stale.

**One of them fails the gate.** `cargo xtask spec-trace` anchor-checks a subset of its
citations, and `spec/SPECIFICATION.md:5315` is in it:

```
spec/SPECIFICATION.md:5315 — citation `crates/happenstance-sqlite/src/projection_store.rs:552`
is evidence for `begin`, and `begin` is not within 12 lines of
crates/happenstance-sqlite/src/projection_store.rs:552. The citation points at the wrong
place, or the sentence attributes it to the wrong thing.
```

That is the **only** traceability problem the lane introduced; the three "rule claimed by
no clause" entries beside it in the same output are pre-existing and belong to other
lanes. `cargo xtask lints` fails on the same one, because it runs `spec-trace` as its
first check.

The other nine are silently stale: they are not anchor-checked, so nothing reports them.

## The repoints

Resolved by **anchor**, not by offset — each end of each range was matched by its exact
line content in the base blob and located uniquely in the current file. The deltas are not
uniform, and two ranges move by different amounts at their two ends, so applying a modal
offset would leave outliers wrong and green.

Base for every "from" below is `bd11598` (`remediation/pre-publication`); the "to" is
`lane/sqlite-ceilings` at its tip, and it was recomputed after the last commit that moved
a line rather than carried forward from an earlier one.

| in `spec/SPECIFICATION.md` | from | to | anchor at the range's start |
|---|---|---|---|
| `:392` | `projection_store.rs:529-679` | `:609-759` | `impl SendProjectionStore for SqliteProjectionStore {` |
| `:1616` | `event_store.rs:284` | `:317` | `pub const MAX_EVENT_DATA_LEN: usize = 1_048_576;` |
| `:2600` | `event_store.rs:998` | `:1141` | `.map_err(\|_\| SqliteEventStoreError::MalformedIdentity { len: raw…` |
| `:2691` | `event_store.rs:1011-1104` | `:1154-1279` | `pub enum SqliteEventStoreError {` |
| `:2999` | `event_store.rs:1337` | `:1512` | `Unsampled,` |
| `:4072` | `event_store.rs:1106` | `:1281` | `impl SendEventStore for SqliteEventStore {` |
| `:4715` | `projection_store.rs:529` | `:609` | `impl SendProjectionStore for SqliteProjectionStore {` |
| `:4740` | `projection_store.rs:534-542` | `:614-622` | `// The `E0195` transcript this line used to carry is now discharged…` |
| `:5315` | `projection_store.rs:552` | `:632` | `fn begin(&self) -> Self::Batch {` — **this is the one the gate fails on** |
| `:8654` | `projection_store.rs:552-679` | `:632-759` | `fn begin(&self) -> Self::Batch {` |

Four citations into these files did **not** move and must be left alone:
`event_store.rs:57`, `event_store.rs:62-67` (four occurrences, at `:3432`, `:3951`,
`:7671`, `:7738`) and `projection_store.rs:19-43`. All sit above every insertion.

`crates/happenstance-sqlite/src/query_sql.rs` is cited by `spec/SPECIFICATION.md` **not at
all**, so I-5 moved nothing there.

## What was repointed rather than briefed

For completeness, so that whoever applies the table above knows what has already been
done and does not redo it:

- **`standards/rust/`** — 20 citations across atoms `22`, `23`, `24`, `25`, `30` and `90`,
  repointed in place. `cargo xtask lint-constitution` verifies them and reports *"27
  atoms, all consistent"*.
- **`.kb/_intake/remediation-2026-09-04-briefs/`** — 63 citations across ten briefs,
  repointed in place. Nothing in the gate scans this directory, so these were the lane's
  own responsibility and are stated here rather than assumed.

  **With one limitation worth naming, because it looks like a defect this lane
  introduced and is not.** Anchor repointing preserves whatever a citation *pointed at*;
  it cannot fix one that was already aimed wrongly. Several citations in these briefs were
  already stale at `bd11598`, from `X-1`'s roughly +38/+41 drift in `event_store.rs` and
  earlier — `sqlite-blocking-seam.md:217` cites `append`'s signature at a line that has
  been inside `SqliteEventStoreError::MalformedIdentity` since before this lane existed,
  and `:708` cites `PAGE_SIZE` at `:141`, which was already `:180` at `bd11598`. They were
  moved faithfully and **not** re-aimed: re-aiming another lane's evidence is a guess at
  its intent, which is the thing the anchor rule exists to forbid. Whoever owns those
  briefs owns those rows.

  One consequence **is** this lane's, and is a quotation rather than a line number:
  `sqlite-blocking-seam.md:708` quotes `PAGE_SIZE`'s doc calling it *"a placeholder until
  it is measured"*, and `R-1+J-5` deleted that sentence — the measurement came back. The
  brief's open question (*whether `PAGE_SIZE = 512` stays*) is unchanged and is now
  answered at greater length in `read-page-budget-rows-bytes-or-caller.md`; the quotation
  beside it is simply no longer in the tree.
- **`docs/`, `examples/`, `RUNBOOK.md`, `CONTRIBUTING.md`, `README.md`** — searched;
  none of them cites these three files by line.

## Why the lane did not simply fix the ten

`spec/SPECIFICATION.md` is owned by another lane in this remediation wave, and the
instruction under which this lane worked is explicit: *move no maturity marker, change no
`[FROZEN]` clause text, do not edit `spec/SPECIFICATION.md` — if a clause is implicated,
brief it.* A line-number repoint is not a clause change, and a reasonable reading would
have permitted it; the flat prohibition was followed rather than argued with, because two
lanes editing one file concurrently is the failure this wave's worktree discipline exists
to prevent.

The consequence is stated plainly rather than dressed up: **the `lane/sqlite-ceilings`
branch does not pass `cargo xtask spec-trace` or `cargo xtask lints` on its own.** It
passes every other command in the scoped gate, and the single failure is the row marked
above. Applying that one row makes both green.
