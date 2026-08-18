---
item: "HS-S0141"
stage: implement
created: "2026-08-17T13:16:03.971Z"
updated: "2026-08-17T13:16:03.971Z"
---

# Implementation Report — One resolver for specification clause ids, next to the parser that owns them

## TDD Evidence

The whole `#[cfg(test)] mod tests` was written first — `xtask/src/spec_trace.rs` had none in
2,316 lines, so this PR creates the file's first — against a production half that was the
**named wrong implementations** rather than empty bodies. That is what makes the red run a set
of assertion failures per row instead of one compile error a brand-new item would otherwise
produce:

* `clause_ids` read the document with `fs::read_to_string(root.join(SPEC)).unwrap_or_default()`
  — the swallowed read, which is EC-001's forbidden degradation to a quiet empty set.
* `collect_clause_ids` returned `Ok(BTreeSet::new())` — EC-002's quiet empty, the one that
  makes both consumers blame real pages and real pinned ids.
* The doc comment was a single summary line — no limits, no `# Errors`.

Red run: `cargo test --locked -p xtask --bin xtask tests::` → **120 passed; 8 failed**, with
every one of the eight failing on the missing behaviour rather than on a typo or an import.
Green run, after the real body and the real doc block landed:
`cargo test --locked -p xtask --bin xtask spec_trace::tests` → **8 passed; 0 failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `clause_ids_reads_the_pinned_specification` | RED: `the real document declares clauses; an empty set is the checker being broken` — the swallowed read plus the empty sibling returned `Ok({})` against the real workspace root. GREEN once `clause_ids` became `collect_clause_ids(&read(root, SPEC)?)`: `ES-1` and `VT-1`, declared at `spec/SPECIFICATION.md:2460` and `:563`, are both in the set. |
| AC-002 | `every_section_family_resolves`, `both_declaration_forms_resolve_and_a_cross_reference_does_not` | RED: `the VT- family must resolve; got {}` and `ES-1 is a declaration; got {}`. GREEN on `parse_clauses(spec).into_iter().map(\|clause\| clause.id).collect()` — no regex, no prefix array, so the accepted families are `SECTIONS`' and the declaration forms are `clause_id`'s. The first test builds its slice by **iterating `SECTIONS`**, the shape at `lint_constitution.rs:567`, so a seventh family cannot leave it green by omission. |
| AC-003 | `a_deferred_clause_without_a_suite_still_resolves` | RED: `a deferred clause is declared, so its id resolves; got {}`. GREEN with no maturity filter and no family filter on the path. The test asserts **both** halves — `SY-7` resolves *and* `has_suite("SY-7")` is `false` — so an edit that later routes the accessor through `has_suite` fails here rather than silently dangling every `PS-` and `SY-` citation. |
| AC-004 | `an_unreadable_specification_names_the_file_it_could_not_read`, `a_specification_that_declares_nothing_blames_the_checker` | RED: `an unreadable document must never degrade to an empty set: {}` and `a document that declares nothing must be a hard error: {}`. GREEN on `?`-propagating `read`'s `reading {rel}` context and on the vacuity `bail!` in `wire_rules`' shape (`:1784-1791`). The second test asserts `message.starts_with(SPEC)` — artifact first, so an 80-column soft-wrap cannot push it off the first visual row — and that the message contains no `more`. |
| AC-005 | `a_doubly_declared_id_collapses_to_one`, `the_accessor_documents_its_limits_before_its_errors` | RED: `left: 0 right: 1` and `the doc block states no limits at all: /// The clause ids …`. GREEN in RS-81-1's order — the blind spot proven in the test, *then* stated in the documentation, and stated first. The ordering test reads the file through `read(&workspace_root()?, …)` exactly as `check_harness` reads a source file, isolates the `clause_ids` doc block, and asserts `does not verify` precedes `# Errors`. |
| AC-006 | *(no test, and the spec says so)* | The self-erasure fires in the **consumer's** PR by design, so claiming a test here would be the decorative-check defect CLAUDE.md names. The instrument is the clippy step under `-D warnings`: `cargo clippy --locked -p xtask --all-targets --all-features` is clean in both configurations, and `cargo xtask ci --fast` printed `all required checks passed`. |

One red-to-green iteration inside the green step is worth recording rather than hiding: the
first `clause_ids_doc_block` helper collected the trailing run of `///`-and-`#[`-shaped lines
and stopped dead at the multi-line `#[expect(…)]`, reporting `the doc block states no limits
at all: )]`. The fix was to take everything after the last blank line before the definition —
exact, because a doc comment and its attributes carry no blank line inside them — rather than
to weaken the assertion.

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Spec-trace clause id accessor |

The row records the checkpoint **as first written**. A commit cannot contain its own SHA, so
the reachable commit is reported in the slice digest and recorded on the item by
`redkiln record-links --sha` at the advance seam.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/spec_trace.rs:1770-1845` | The accessor and its private `&str`-taking sibling, placed **beside `all_rules`** as the design signed off: `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` reads `SPEC` through the existing `read` helper and calls `collect_clause_ids`, which maps `parse_clauses`' `Clause` values to their ids, collects into a `BTreeSet<String>`, and `bail!`s when the set is empty. Read and decide are separate for one reason: every assertion in this story is then an assertion about a `&str`, which is what keeps the dev-dependency count at zero (DR-12). |
| `xtask/src/spec_trace.rs:1770-1802` | The doc block: the design's verbatim summary and sibling sentence, `# What this does not verify` **first** with the three limits, one sentence naming the two alternatives that lost (a literal prefix list; a `Vec<Clause>` return), then `# Errors` phrased as conditions. |
| `xtask/src/spec_trace.rs:1803-1817` | The self-erasing dead-code marker, `#[cfg_attr(not(test), expect(dead_code, reason = …))]`, whose `reason` names both consumer stories. |
| `xtask/src/spec_trace.rs:2394-2622` | The file's first `#[cfg(test)] mod tests`, in `lint_constitution.rs:827-841`'s shape including the scoped `#![allow(clippy::unwrap_used, reason = "test code, per the house style")]`. Eight tests; six are in-memory over `&str` and two touch the real tree through `workspace_root` and `read`. |
| `.bklg/…/spec-trace-clause-id-accessor/_ledger.md` | Six rows flipped to `satisfied: true` with `file:line` and test-id evidence. |

**Mount point.** There is no composition-root edit, and that is the story rather than an
omission: `mod spec_trace;` at `xtask/src/main.rs:70` already mounts the module in the *bin*
crate, and `xtask/src/lib.rs` declares only `mod constitution;`, so `pub(crate)` is exactly
the visibility that reaches `lint_narrative` and nothing else. `xtask/src/main.rs` and
`xtask/src/lib.rs` are unchanged, as the PR boundary requires.

**What did not move**, checked rather than asserted: `run` (`:604-646`) keeps its own parse and
its own empty guard, the generated §7.1–§7.2 region is untouched, nothing writes, and
`xtask/Cargo.toml` gains no entry.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask tests::` (red) | `120 passed; 8 failed` — the eight rows of this story, each failing on its missing behaviour |
| `cargo test --locked -p xtask --bin xtask spec_trace::tests` (green) | `8 passed; 0 failed` |
| `cargo clippy --locked -p xtask --all-targets --all-features` | clean, `-D warnings`-safe; and clean again without `--all-targets`, which is the configuration the `expect` covers |
| `cargo fmt -p xtask -- --check` | clean |
| `cargo xtask affected --base main` | `affected gate passed` — `231 passed; 0 failed; 3 ignored` |
| `cargo xtask spec-trace` | `200 clauses (139 FROZEN, 49 PROVISIONAL, 10 DEFERRED, 2 NON-NORMATIVE), 95 conformance rules, 58 e2e cases, 358 citations checked`; `traceability: no problems found; §7.1–§7.2 matches the checker` — byte-identical to the pre-commit run (NF-004) |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)` — the merge bar, and AC-006's instrument |

## Notes

**One deviation from the spec, and it is in the dead-code marker's spelling.** The spec asks
for `#[expect(dead_code, reason = …)]`; what landed is
`#[cfg_attr(not(test), expect(dead_code, reason = …))]`. The reason is mechanical and was
observed rather than predicted: this story's own test module is a *caller* of `clause_ids`, so
under `cfg(test)` the item is live, the expectation is unfulfilled, and
`unfulfilled_lint_expectations` fires — `warning: this lint expectation is unfulfilled` on the
first `cargo test` run. The gate's clippy step is `clippy (all targets, all features)`
(`xtask/src/main.rs:118-121`), so that warning is a `-D warnings` failure, and a bare `expect`
would have made this story's own checkpoint red.

Everything AC-006 asks the marker to *do* survives the narrowing. It is still `expect` and not
`allow`, so it cannot rot into a permanent exemption; the `reason` still names both consumer
stories; and the self-erasure still fires on the first non-test call site, which is exactly
what `narrative-citation-resolution` adds. The alternative — deleting the tests that call the
accessor so a bare `expect` would hold — would have bought the literal spelling by removing
the evidence, which is the trade the spec's own risk table refuses.

**Two things the spec asked to be said explicitly, said.** No conformance rule is added, no
port changes and nothing here is adapter-observable, so CF-29's `CHANGELOG.md` obligation
(`xtask/src/lints.rs:504-580`) is not triggered. And no `[FROZEN]` clause is edited, amended or
restated, so no ADR is owed; the one divergence from in-repo precedent — hard-erroring on a
vacuous parse where `spec_trace::citations` skips — is discharged by a sentence in the item's
own `# Errors` section, which is what architecture Note 6 asks for.

**Hand-off to the two consumers.** `clause_ids` is called **once** per gate run by the
narrative checker, and the resulting `BTreeSet<String>` is passed to both the citation check
and the pin check. Two calls would read and parse a 9,070-line document twice inside one step
for no new information (EC-006). The first consumer to land must delete the `cfg_attr` block
at `xtask/src/spec_trace.rs:1803-1817` in the same change, because clippy will report the
unfulfilled expectation the moment the call site exists.
