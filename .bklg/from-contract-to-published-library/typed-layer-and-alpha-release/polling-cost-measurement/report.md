---
item: "HS-S0028"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — The polling cost ES-32 imposes, as a number

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing deferred, nothing blocked.**

The mount point is `experiments/polling-cost/Cargo.toml` — the one file that both binds the
harness to the **real** `happenstance::run_projection` (a path dependency with
`features = ["unstable-projection", "memory", "json"]`) and holds it *out* of the workspace
(a bare `[workspace]` table under a comment saying why). That is CF-34 expressed in git
rather than in a promise: a correct implementation of this story cannot touch a file the
gate compiles, and this one does not.

**The number.** Delivery amplification **32.00** at 32 overlapping views — exactly the
fan-out, no economy of scale — and **1.00** disjoint. 64 reads either way. Observation
latency is the poll interval at every fan-out, with no backlog. All 144 rows, with every
condition attached, are in `experiments/polling-cost/results/pass-001/records.ndjson`.

| AC | result | proved by | notes |
| --- | --- | --- | --- |
| AC-001 | satisfied | `one_cell.rs` (2 tests) + a `run.sh` transcript | One command, a Rust toolchain, nothing else. The harness calls the published runner and binds `EventStore`, never `SendEventStore`; the source assertion strips comments first, because the harness's own page names the flavour it does not bind. |
| AC-002 | satisfied | `amplification.rs` (3) + `sweep_shape.rs` (3) | The headline is a ratio of two observed counts, not a duration. Both arms present, every declared axis value present. **The denominator deviates from AC-002's literal words** — see *Deviations*. |
| AC-003 | satisfied | `record_conditions.rs` (4) | 17 fields on every record, 14 on the once-per-pass manifest, none null or empty. The committed pass names revision `55a2370` with `git_dirty: false` and `profile: release`. |
| AC-004 | satisfied | `staleness.rs` (4), **including the named mutant** | Latency between two observed instants; backlog by comparing positions. The subtracting mutant is written into the test file and asserted to answer **2 and 14** where the observer answers **2 and 2**, against `GappyMemoryStore` at stride 7. `head()` appears nowhere in the harness. |
| AC-005 | satisfied | `schema.rs` (4) | NDJSON under `results/`, validated line by line against schemas committed in `schema/`. `schema_version` and `seed` on every record; `projection_store` is one of exactly two named values and is HS-P0010's `MemoryProjectionStore`, which had landed. |
| AC-006 | satisfied | `readme.rs::the_gap_is_named_in_the_falsifiers_own_words`, `::the_verdict_quotes_the_figure_and_stops_there`, `::the_headline_figure_matches_the_committed_corpus` | Both falsifier terms quoted, the word **floor** present, the verdict recommending nothing — and the strong check: the printed `32.00` is recomputed from the corpus, so prose and numbers cannot drift. |
| AC-007 | satisfied | `gate_inertness.rs` (5) + `cargo xtask affected --base main` + `cargo xtask ci --fast` | No package selected, no step added, root `members` and `[workspace.dependencies]` untouched, no `crates/**` manifest changed. |
| AC-008 | satisfied | `no_verdict.rs` (3) | An absurd figure still returns `Ok`. No threshold constant, no `assert!`/`panic!` on the measured path, and `HarnessError` has no arm for a number being large. ES-32's marker is not moved and `spec/` is untouched by this story. |
| AC-009 | satisfied | `readme.rs` (3) + `output_format.rs` (4) | Sections in the declared order, ratio before the first timing figure, `… and 30 more` on both long tables. Console output is plain lines, **truncated** at 80 columns rather than wrapped, with control characters flattened. |
| AC-010 | satisfied | `rerun_is_additive.rs` (2), and demonstrated for real | `HS_TAG=pass-001 bash run.sh` against the committed pass exits non-zero naming it, and the committed bytes are unchanged. A new tag lands beside it; undoing a re-run is deleting one directory. |

### What a reviewer should look at first

1. **`staleness.rs::the_subtracting_mutant_fails_the_same_comparison`.** The mutant is in
   the test file and is *asserted to disagree*. Without it, the gap test would pass against
   an implementation that subtracts, which is the failure mode `head()`'s own documentation
   warns about.
2. **The `## 4. What this does not prove` section of the README.** This is a floor, and it
   says so. The read count — 2N in both arms — is the figure most likely to change
   character entirely on a store where a read is a round trip, and the section says that in
   the falsifier's own terms.
3. **`experiments/polling-cost/Cargo.toml`'s first six lines.** The bare `[workspace]`
   table with its reason. Delete the comment and the next reader deletes the table.

### Deviations, all recorded in the implementation report

- **The headline's denominator.** AC-002 spells it *"events the projections applied"*.
  Under a derived query and a filtering store that ratio is identically **1.0** in both arms
  at every fan-out — it measures nothing, and it would make the two selectivity arms report
  the same figure, defeating the reason for having two. The denominator implemented is the
  **distinct** events those deliveries carried, observed rather than assumed, which gives
  32.00 and 1.00. Both raw counts are on every record so either ratio is recomputable, and
  the schema and README document the choice on the field.
- **`src/store.rs` became `src/counting.rs`.** A second `store.rs` anywhere in the tree
  makes `spec-trace`'s bare-name citation resolution ambiguous, and produced 19 traceability
  problems in the affected gate. The alternative fix lived in `xtask/src/spec_trace.rs`,
  outside this story's boundary.
- **`git_dirty` is scoped to the measured tree** (`crates/`, `Cargo.toml`, `Cargo.lock`)
  rather than the whole worktree, and the schema says so. Without that scoping no committed
  pass could ever carry `false`, because the pass and the README quoting it are necessarily
  uncommitted while the sweep runs.
- **One measurement artefact was found and the first sweep discarded.** Observers were
  initially told about events their view's query excludes, inflating disjoint-arm staleness
  and manufacturing a backlog. Fixed by telling only the views that nominated the event; the
  committed pass is the corrected run.

### Handed forward

- **HS-S0032 (`defect-log-and-macros-verdict`)** receives one project AC-012 entry: this
  story is the first consumer of `run_projection` from outside the workspace, and
  `&mut P` makes N views cost N reads *at the API level* — correct for the alpha, and worth
  recording as a shape the tail-seam decision will meet.
- **HS-P0016 and `projection-clause-verdicts`** own ES-32's disposition. This story supplies
  evidence toward its falsifier and moves no marker; §5 of the README quotes the figures and
  stops.
- **HS-P0012 (`sqlite-durable-store`)** owns the *"on a real deployment"* term. The
  reproducer is one command and the record contract is versioned, so a second pass against a
  durable adapter writes beside this one under a new tag rather than replacing it.
- **The project closeout (DoD 8)** has a quotable headline: *delivery amplification 32.00 at
  32 overlapping views, 1.00 disjoint, 64 reads either way* — cited from
  `experiments/polling-cost/README.md`, not restated.
