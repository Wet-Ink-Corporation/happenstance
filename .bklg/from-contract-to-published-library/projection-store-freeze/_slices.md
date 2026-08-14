---
item: HS-P0010
stage: implementation
created: 2026-08-14T01:25:19.009Z
updated: 2026-08-14T01:25:19.009Z
template_sig: 4c5f37d6
rendered_sig: e468b480
---

# Slice ledger — Freeze ProjectionStore behind a suite that can fail

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| decisions-and-design-record | approved | ps-clause-pairing-sweep f77f183, projection-decision-atoms 9520b28, projection-api-design-record 0df2c1c | (this commit) |
| projection-port-and-probe | changes-requested | owned-batch-port-shape 2eade38, projection-probe-conformance-feature cb495ee, memory-projection-store 5fd62c6 | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### projection-port-and-probe

- `crates/happenstance-core/src/projection.rs:495-499` publishes a `[dev-dependencies]` block for
  `happenstance-core = { features = ["conformance"] }`, which contradicts the orphan-rule paragraph
  three lines above it (:486-491) and cannot work. Because `impl ProjectionProbe for TheirStore` is
  orphan-rule-rejected in the adapter's `tests/` crate, the impl must live in the adapter's `src/`; a
  dev-dependency does not make `ProjectionProbe` exist for the lib build, and Rust cannot `#[cfg]` on
  a dependency's feature. The story spec anticipated this precisely — AC-002 reads "`[dependencies]`
  (not `[dev-dependencies]`)" — and the ledger records the wrong form as satisfying evidence for
  AC-006 ("with the `[dev-dependencies]` cost stated"). This is published guidance on the crate's
  public page, aimed at the exact persona (P2, the adapter author) the project exists to serve.
  Fix: rewrite the snippet to the shape that actually compiles and still demonstrates "one flag on a
  dependency they already have, no new edge": `[dependencies] happenstance-core = "…"` /
  `[features] conformance = ["happenstance-core/conformance"]` /
  `[dev-dependencies] happenstance-testkit = "…"`. Then correct the AC-006 evidence line in
  `.bklg/from-contract-to-published-library/projection-store-freeze/projection-probe-conformance-feature/_ledger.md:64`
  so the ledger stops asserting the wrong artefact.

- `crates/happenstance-core/src/projection.rs:504-507` states "The falsifier is the story that builds
  an outside author's fixture from the documentation alone, and it is named here" — but no name
  appears. probe AC-006 requires the trait's doc to be "naming `documented-extension-surface` as the
  story that can falsify it". The sentence is self-refuting as written, and the ledger claims the
  requirement met.
  Fix: either name it — "…the falsifier is `documented-extension-surface` (HS-S0015), which builds an
  outside author's fixture from the documentation alone" — or, if naming an internal backlog slug in
  published rustdoc is judged wrong, drop the "it is named here" clause, record the deviation and its
  reason in the ledger's AC-006 row rather than reporting the AC as met.

- **Added by the orchestrator after the run, not by the slice reviewer.**
  `owned-batch-port-shape`'s checkpoint `2eade38` writes
  `crates/happenstance-ladybug/src/stand_in.rs`, which is outside the boundary its own `spec.md`
  declares. Found by `redkiln verify --item HS-S0004 --grain story`, whose `boundary` check reads
  `links.commits`; the slice review did not report it. The consequence is not cosmetic:
  `redkiln advance HS-S0004 --to report` runs `implement`'s command gate on the way out, the gate is
  red, and the story cannot reach its `report` review gate at all — so **HS-S0004 is held at `plan`
  and no human verdict could be recorded against it**, unlike its two slice-mates.
  Settle which of the two it is, and do not paper over it: either the `spec.md` boundary is too
  narrow for work `owned-batch-port-shape` legitimately owns — in which case widen it deliberately
  and say why, as `cc0f158` did for its own widening — or the implementer reached into the ladybug
  skeleton it should not have, in which case the `stand_in.rs` change belongs to a story that owns
  that file. Widening a boundary to match what was written, with no reason given, converts the check
  into a rubber stamp for whatever the implementer happened to touch.

## Fix passes

Claims for the next reviewer to verify, not facts to trust. The findings above are left exactly as
the reviewer wrote them; what each one was answered with is recorded here.

### projection-port-and-probe — fix pass 2026-08-14

- **The `[dev-dependencies]` snippet.** Rewritten to the shape that compiles and still carries the
  argument, at `crates/happenstance-core/src/projection.rs:496-509`: `[dependencies]
  happenstance-core = "…"`, `[features] conformance = ["happenstance-core/conformance"]`,
  `[dev-dependencies] happenstance-testkit = "…"`. The forwarding entry carries a comment saying why
  the dev-dependency spelling cannot work — the impl lives in the adapter's `src/` and a crate cannot
  `cfg` on a dependency's feature — so the wrong form is refuted on the page rather than merely
  absent from it. The prose above it (`:491-494`) now says the impl goes in `src/`, which is what
  the orphan-rule paragraph three lines above always implied.
- **The unnamed falsifier.** Named, at `:514-517`: "the falsifier is `documented-extension-surface`
  (HS-S0015), which builds an outside author's fixture from this documentation alone". The first arm
  of the reviewer's remedy was taken rather than the second: probe AC-006 asks for the name, the
  slug is the identifier the outside-author story is known by in every other artefact, and a
  placement argument whose falsifier is anonymous is one nobody can go and check.
- **The AC-006 row.** `projection-probe-conformance-feature/_ledger.md` AC-006 records the
  withdrawal and the re-assertion, quotes both false descriptions it previously carried, and states
  the evidence against the corrected text with current line numbers.
- **The stale adapter module docs** — a defect of the same class, found by this pass and not by the
  slice review. Four rendered module docs described the deleted GAT port in the present tense; they
  now read, in order, `crates/happenstance-neon/src/projection_store.rs:3-22` and `:51-53`,
  `crates/happenstance-sqlite/src/projection_store.rs:11-18`, and
  `crates/happenstance-postgres/src/projection_store.rs:7-12`. All four are restated in the past
  tense with the port's current shape and the clause that settled it (PS-4, PS-5, PS-6, ADR-0017).
  This is what `owned-batch-port-shape/spec.md:180` put those files in the boundary to prevent, and
  it had been done for the `//` comments and the crate-level `//!` docs but not for the module `//!`
  docs a docs.rs reader actually meets.
- **The duplicated `error[E0195]` narrative.** Trimmed at the module doc (`projection.rs:25-36`) to
  an intra-doc pointer at `ProjectionStore`'s `# Implementing it` — the item whose unusual shape
  bought it, and the one carrying the compiling doctest — leaving the narrative stated once, per
  RS-70-5 and memory AC-009. The rustc ICE, which is the *other* of the two compiled results and is
  stated nowhere else, stays in the module doc.
- **`stand_in.rs`, the undeclared boundary file.** Settled as a deliberate widening of
  `owned-batch-port-shape/spec.md`, recorded in the paragraph under its PR-boundary fence with the
  reason: the edit was forced by an entry the boundary already admits (ADR-0017's removal of the
  `live_handle` module, which `crates/happenstance-ladybug/src/lib.rs` is in the list for), and an
  intra-doc link into a removed module is a hard error under `broken_intra_doc_links = "deny"`. The
  alternative was not "leave the file alone" but "do not execute ADR-0017".

## Authorised boundary widenings

Widenings taken outside any single story's declared boundary, recorded where a later reader can find
them. A commit message asserting its own authorisation is not an authorisation record.

- **`cc0f158` — `spec/SPECIFICATION.md`, `standards/rust/**`, `CLAUDE.md`, `xtask/src/main.rs`.**
  None of the three story specs in this slice lists those paths, and `owned-batch-port-shape`'s
  EC-004 explicitly requires stop-and-report rather than editing `SPECIFICATION.md` inside the story.
  The stop-and-report *was* filed — `owned-batch-port-shape/_reviewed-diff.md` §7 — and `cc0f158`
  executed its remedy (a) at the slice boundary rather than inside a story, which is where a finding
  larger than one story is supposed to be discharged. The forcing reason: `xtask/src/affected.rs:117-125`
  runs `spec-trace` first and unconditionally and `.redkiln/config.yaml:40` wires
  `cargo xtask affected --base {{base}}` as the story gate, so while the seven stale citations stood,
  **no** story in this project could pass its own declared gate. The re-points changed `file:line`
  targets only — no clause text and no maturity marker moved — and the two retirements (RS-22-4 and
  RS-92-1) were taken at each rule's own written PROVISIONAL settlement trigger, which PS-5 met when
  ADR-0017 landed. RS-21-1 was amended, not retired, because only its spelling died. Ratified here
  on 2026-08-14; the remaining exposure is that it was ratified *after* the fact, which is what this
  section exists to stop happening twice.
