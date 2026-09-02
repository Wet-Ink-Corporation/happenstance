# Clause-citation audit — every normative claim this project's pages make

Companion to `spec.md`, beside `_inventory.md` and `_ledger.md`. It carries **`§ Claims`**,
**`§ Restatement`** and **`§ Drills`**, and it is read cold at closeout (NF-002).

`_inventory.md` answers *is this fence exercised*. This file answers *does this sentence point
at a clause that exists, and does the page defer to it rather than becoming a second copy of
it*. The split is the slice's own: what a gate step can decide, and what only a reader can.

**Dated 2026-08-19**, against `initiative/docs-that-teach` at `9dc139a`, toolchain **1.97.1**.
Re-derivation commands are `git grep -n` or a `cargo` subcommand, for the reason
`merge-forward-preflight/_baseline.md:90-95` gives: `rg` is not on this machine's `PATH`, and a
command that only runs in one sandbox is a claim rather than a check.

**Id first, line second** (NF-003). Clause **ids** are stable and are never renumbered
(`spec/SPECIFICATION.md:300`); clause **lines** are not, and they already rotted once at the
merge. Every merged location below is read from `merge-forward-preflight/_baseline.md § Anchors`
where that record carries the id, and re-derived with the same `git grep -nF` form where it does
not — ES-8 and CF-8 are cited by pages that did not exist when the preflight ran, so their rows
are new anchors rather than re-reads.

---

## § Claims

Every sentence on the four surfaces that states a rule the reader is expected to obey, with the
clause it cites, that clause's maturity marker **verbatim from `spec/SPECIFICATION.md`**, and —
per page, never per project — **what actually resolves the citation**.

### The resolver, stated per page before any claim is listed

| surface | path | the resolver for a clause id **cited by this page** | can it fail? |
| --- | --- | --- | --- |
| `crate-root-encounter` | `crates/happenstance/src/lib.rs` | **nothing.** Outside `TREE = "docs"` (`xtask/src/lint_narrative.rs:239`), so `lint_narrative::check_citations` never opens it; `spec_trace` reads only `spec/SPECIFICATION.md` (`xtask/src/spec_trace.rs`); a relative markdown link to the specification is not an intra-doc link, so `broken_intra_doc_links` under `RUSTDOCFLAGS=-D warnings` does not see it | **no** — observed, `§ Drills` D-4. Routed as `_inventory.md § Routing` R-5 |
| `opening-encounter` | `docs/first-encounter.md` | HS-P0020's `every narrative page is checked` step, via `clause_ids` → `lint_narrative::check_citations` (`xtask/src/lint_narrative.rs:1197`) | **yes** — observed, `§ Drills` D-3 |
| `conceptual-bridge` | `docs/carry-your-invariant.md` | the same step | **yes** |
| `worked-example-handoff` | `docs/read-the-worked-example.md` | the same step — vacuously, the page cites no clause | **yes**, if it ever did |
| (`overview.md`, inventoried with them) | `examples/course-subscriptions/src/overview.md` | **nothing**, on the same corpus boundary as the crate root | **no**; vacuous today — the file cites no clause |

`cargo xtask spec-trace` appears in neither column, and that is the correction this table exists
to make. It is a `REQUIRED` step (`xtask/src/main.rs:369`) and it runs at this story's grain
(`.redkiln/config.yaml:48`), but what it checks is `spec/SPECIFICATION.md`'s **internal**
traceability — every normative clause against the conformance rule that discharges it — and it
never opens a narrative page. D-3 measures that directly: on a tree with `ES-999` on a page,
`spec-trace` exits 0.

### The claims

| id | page and location | the sentence's own claim | cited id | maturity marker, verbatim | resolver | link form |
| --- | --- | --- | --- | --- | --- | --- |
| **C-1** | `docs/first-encounter.md:41-43` | events come back in the order the store assigned, and every other reader of the store sees that same order | **ES-8** (`spec/SPECIFICATION.md:2781`) | `**[FROZEN]**` at `:2787`; index row `\| ES-8 \| FROZEN \|` at `:9082` | tree-page checker | inline `([ES-8](../spec/SPECIFICATION.md#es-8--ordering))`, last position in the sentence, ordinary link |
| **C-2** | `docs/first-encounter.md:82-84` | `after` is exclusive, and an event at exactly that position never rejects | **ES-26** (`:3814`) | `**[FROZEN]**` at `:3820`; index row at `:9100` | tree-page checker | inline, last position, ordinary link |
| **C-3** | `docs/first-encounter.md:127-130` | the store is required to refuse and required to report the refusal under exactly the name the program just printed | **ES-25** (`:3756`) | `**[FROZEN]**` at `:3764`; index row at `:9099` | tree-page checker | inline, last position, ordinary link |
| **C-4** | `docs/carry-your-invariant.md:36-40` | items are OR'd across a query, so it returns everything either half of the rule depends on | **ES-27** (`:3855`) | `**[FROZEN]**` at `:3861`; index row at `:9101` | tree-page checker | inline, last position, ordinary link |
| **C-5** | `docs/carry-your-invariant.md:86-88` | the condition is the query already written, bounded at the position the read reached, and a matching event above that boundary refuses it | **ES-25** (`:3756`) | `**[FROZEN]**` | tree-page checker | inline, last position, ordinary link |
| **C-6** | `docs/carry-your-invariant.md:90-93` | carrying several independently bounded guards in one condition is described but is **not settled**, so nothing above leans on it | **VT-30** (`:1861`) | `[PROVISIONAL — falsified if no adapter can push a multi-guard condition into a single round trip]` at `:1869`; index row `\| VT-30 \| PROVISIONAL \|` at `:9048` | tree-page checker | inline, **not** the sentence's final token — see the note below |
| **C-7** | `docs/carry-your-invariant.md:134-137` | a condition carrying a tag no stored event carries does not reject the append, even when a stored event matches the condition's types | **CF-8** (`:7704`) | index row `\| CF-8 \| FROZEN \|` at `:9212` | tree-page checker | inline, last position, ordinary link |
| **C-8** | `docs/carry-your-invariant.md:139-142` | the mirror mistake — dropping the tags and guarding on types alone — fails loudly, by refusing commands it should admit | **CF-7** (`:7670`) | index row `\| CF-7 \| FROZEN \|` at `:9211` | tree-page checker | inline, **not** the sentence's final token — see the note below |
| **C-9** | `docs/carry-your-invariant.md:144-146` | both follow from the same rule: a condition is matched on tags and not only on types | **ES-27** (`:3855`) | `**[FROZEN]**` | tree-page checker | inline, last position, ordinary link |

**Nine claims, nine citations, nine ids that resolve.** No page carries a normative claim
without a citation, and no citation names an id `spec/SPECIFICATION.md` does not define — proved
by the checker rather than by this table: `cargo xtask narrative` is green over all five pages,
and `check_citations` reports by `path:line` when it is not (D-3).

**The two surfaces that cite nothing, and why that is a result rather than an omission.**
`crates/happenstance/src/lib.rs` and `docs/read-the-worked-example.md` carry **zero** clause
citations. Both were tested sentence by sentence with RP-30-1's falsification question —
*could a conformant adapter written in another language violate this sentence?*
(`standards/pages/30-citing-the-specification.md:20-26`) — and every candidate came out **no**:
the crate root's claims are about this workspace's crate split, its feature table and its
re-export surface, and the handoff page's one API claim is that `happenstance::commit` derives
the `Query` and the `AppendCondition` from a `DecisionModel`. None of them is a clause, so none
of them owes a citation. Recorded rather than left blank, because a page with no citations and a
page nobody checked look identical in a count.

### Every link target resolves, and that was checked rather than assumed

The checker resolves the **id**. It does not resolve the **fragment** the link points at, so
that half was resolved separately by slugging every `#`-heading of `spec/SPECIFICATION.md` and
matching each link's fragment against it:

```
docs/first-encounter.md:43    [ES-8]  -> #es-8--ordering                        -> SPECIFICATION.md:2781
docs/first-encounter.md:84    [ES-26] -> #es-26--the-ac3-boundary-…             -> SPECIFICATION.md:3814
docs/first-encounter.md:130   [ES-25] -> #es-25--condition-semantics            -> SPECIFICATION.md:3756
docs/carry-your-invariant.md:40   [ES-27] -> #es-27--a-condition-matches-…      -> SPECIFICATION.md:3855
docs/carry-your-invariant.md:88   [ES-25] -> #es-25--condition-semantics        -> SPECIFICATION.md:3756
docs/carry-your-invariant.md:92   [VT-30] -> #vt-30--an-appendcondition-is-…    -> SPECIFICATION.md:1861
docs/carry-your-invariant.md:137  [CF-8]  -> #62-the-measured-gaps              -> SPECIFICATION.md:7663
docs/carry-your-invariant.md:141  [CF-7]  -> #62-the-measured-gaps              -> SPECIFICATION.md:7663
docs/carry-your-invariant.md:146  [ES-27] -> #es-27--a-condition-matches-…      -> SPECIFICATION.md:3855
```

**Nine of nine resolve.** Two land on a **section** rather than on a clause heading, and that is
correct rather than sloppy: CF-7 and CF-8 are not `####` headings — they are bold paragraphs
`**CF-7.**` and `**CF-8.**` inside `## 6.2 The measured gaps` (`spec/SPECIFICATION.md:7663`), so
`#62-the-measured-gaps` is the nearest anchor that exists. Recorded so a later reader does not
read the difference as a defect and "repair" it to a fragment the document does not emit.

Re-derivation: slug every `^#{1,6}` heading of `spec/SPECIFICATION.md` (lowercase, strip
backticks and punctuation, spaces to hyphens) and match each page's
`](../spec/SPECIFICATION.md#…)` fragment against the set. The rule is the one
`tension-resolutions/_resolutions.md:377-384` already verified by example for this repository's
two renderers.

### The two maturity findings

**VT-30 is `[PROVISIONAL]`, and C-6's wording survives it.** `_design.md:400-405` binds this:
no page may imply the shape is frozen, and no page may be written in a way that would need
rewriting if VT-30 changes. C-6 reads *"carrying several independently bounded guards in one
condition is described but is not settled ([VT-30]), so nothing above leans on it."* It names
the shape as **described and unsettled**, and it states that the page's own material does not
depend on it. If VT-30 were amended, withdrawn or frozen in a different shape, that sentence
would still be true. **Verdict: passes.**

**C-6 and C-8 are the two citations that are not the final token of their sentence**, and both
are recorded rather than repaired. `_design.md:556`'s transience row asks for *"last position in
a sentence, ordinary link"*, and the mechanism it protects is that a citation must not interrupt
the reading or sit behind a hover. Both are ordinary inline links, both are recessive, and both
close the clause that makes the claim; what follows each is the page's own qualification —
C-6's *"so nothing above leans on it"* is the `[PROVISIONAL]` hedge `_design.md:400-405`
requires, and C-8's *"; this fence is deliberately not that one"* is what stops a reader
mistaking the wrong-guard fence for the type-only failure. Moving either citation to the
sentence's end would orphan or delete the clause that makes it correct, which is a rewrite and
not a one-line repair (EC-008). **Disposition: no repair; recorded here so the tier-5 reviewer
meets the two sentences rather than a tick.**

---

## § Restatement

Two passes, in the order `_design.md`'s anti-pattern 11 and HS-P0021's RP-40-2 put them.
**No second procedure was invented**; inventing one is DR-14's defect in another medium
(`project.md:224-226`).

### Pass 1 — the mechanical sweep (anti-pattern 11, `_design.md:903-905`)

*A sentence containing "MUST" or "MUST NOT" that is not a link to a clause id.* Greppable, and
pasted as one:

```
$ git grep -n "MUST" -- crates/happenstance/src/lib.rs docs/first-encounter.md \
      docs/carry-your-invariant.md docs/read-the-worked-example.md \
      examples/course-subscriptions/src/overview.md
$ echo $?
1
```

**Zero hits across all four surfaces and `overview.md`** — `git grep` exits 1 on no match, so
the empty output is a result rather than a mistyped path. Not one sentence on any of these pages
contains `MUST` or `MUST NOT` at all, which is a stronger result than anti-pattern 11 asks for:
the pattern permits a `MUST` sentence that *is* a clause link, and this set has none of either.

The sweep's own limit, stated: it is a word search. A clause can be restated perfectly without
the word `MUST`, which is the whole reason pass 2 exists and the reason a grep alone would be
the decorative check CLAUDE.md's corollary names.

### Pass 2 — HS-P0021's paraphrase spot check, RP-40-2

The procedure is `standards/pages/40-reviewing-a-page.md:68-90`, executed as written and not
adapted: *walk the corpus file by file; for each sentence that states a rule, ask whether its
authority is a visible, resolving clause id or a restatement of clause content; record one
verdict per file, with the number of normative sentences examined.* RP-40-2's named wrong output
is *"a sweep recorded as 'no paraphrase found' with no file list and no sentence count"*, so the
counts below are the procedure's own requirement rather than decoration.

**The walker.** This story's implementation context. It authored **none** of the four surfaces —
`docs/first-encounter.md` is HS-S0185's and HS-S0186's (`9493276`, `cc9c4a4`),
`docs/carry-your-invariant.md` is HS-S0187's (`b24d2cd`), `docs/read-the-worked-example.md` and
`overview.md` are HS-S0188's (`5805623`), and `crates/happenstance/src/lib.rs`'s module doc is
HS-S0185's — which is the constraint `page-need-discipline/project.md:233-235` fixes. Sources
consulted for the verdicts: the rendered and source pages, `spec/SPECIFICATION.md`, and
`standards/pages/`. **Not** consulted: any authoring story's spec, and no page's git history —
the commit shas above are attribution for this record, not verdict input. The tier-5 human
sign-off that closes AC-005 is the project's review gate, and this record is what it reads.

| file | normative sentences examined | cited | restated | verdict |
| --- | --- | --- | --- | --- |
| `crates/happenstance/src/lib.rs` | **0** of 14 candidate claim sentences; all 14 answered *no* to RP-30-1 (they are about this crate's split, features and re-exports, not about store behaviour any adapter could violate) | — | **0** | **no paraphrase** |
| `docs/first-encounter.md` | **3** (`:41-43`, `:82-84`, `:127-130`) | 3 | **0** | **no paraphrase** |
| `docs/carry-your-invariant.md` | **6** (`:36-40`, `:86-88`, `:90-93`, `:134-137`, `:139-142`, `:144-146`) | 6 | **0** | **no paraphrase** |
| `docs/read-the-worked-example.md` | **0** of 3 candidate claim sentences | — | **0** | **no paraphrase** |
| `examples/course-subscriptions/src/overview.md` | **0** of 4 candidate claim sentences (they describe what the example's own program does) | — | **0** | **no paraphrase** |

**Nine normative sentences across the set; nine carry a visible, resolving clause id; none
restates clause content.** No page has become a second specification.

**The two closest calls, quoted verbatim, because a verdict without the sentence it turned on is
a tick.** RP-30-2's discriminator is not staleness — its own permitted example goes stale too if
the clause is amended — it is whether the page has *spelled out the normative structure* so a
reader could follow the page instead of the clause. Its named wrong output is a sentence
carrying the clause's `MUST`s in the page's own voice.

> **`docs/first-encounter.md:82-84`** — *"A matching event already sits at exactly the position
> you read to, and the append is still admitted: `after` is exclusive, and an event at exactly
> that position never rejects ([ES-26])."*
>
> ES-26 reads: *"`AppendCondition::after` MUST be **exclusive**: an event at exactly `after`
> MUST NOT reject the append."* The page's sentence is the nearest thing in the set to a
> paraphrase. **Verdict `cites`**, on three grounds: it carries no `MUST` or `MUST NOT`, it
> omits the clause's second half entirely (`ReadOptions::from` is inclusive, and the asymmetry
> is deliberate and must be documented as such), and it is describing the output the reader has
> just seen printed two lines above — `1 seen, up to Some(SequencePosition(1)), at
> SequencePosition(2)` — rather than legislating. A reader who followed this sentence instead of
> ES-26 would not thereby know anything ES-26 says that the page does not send them to it for.

> **`docs/first-encounter.md:127-130`** — *"That line is printed from inside the matched arm, so
> it cannot appear unless the store really refused — which it is required to do, and required to
> report under exactly that name ([ES-25])."*
>
> ES-25 reads, in part: *"A rejection MUST be reported as `AppendError::ConditionViolated`,
> never as `AppendError::Store`."* **Verdict `cites`.** The sentence asserts *that* an obligation
> exists and hands the reader to the clause for what it says; it does not carry ES-25's
> if-and-only-if, its `after: None` case, or its nothing-was-written guarantee. *"Under exactly
> that name"* points at the name the program printed rather than naming it a second time in the
> page's own voice, which is the difference RP-30-2's `Do`/`Not` pair draws.

Both pages already carry a machine instrument for pass 1 —
`xtask/tests/first_encounter.rs:716` (`no_sentence_states_a_rule_in_the_pages_own_words`) — and
it is scoped to `docs/first-encounter.md` alone. `docs/carry-your-invariant.md` and
`docs/read-the-worked-example.md` have **no** such test; for those two the mechanical pass is
the sweep above and nothing else. Recorded because a coverage claim that averages over the set
is the artifact this story exists to refuse.

---

## § Drills

**D-3 and D-4, the clause-id pair.** D-1 and D-2 — the opt-out pair — are in
`_inventory.md § Drills`, split by artifact so each headline claim sits beside its own
falsification (`spec.md § Clarifications resolved during spec`, item 3). Four fields each: the
exact edit, the exact command, the exact output, and the revert with `git hash-object` shown
identical. Each asserts on the checker's `path:line — message` shape, never on a bare non-zero
exit.

Pre-drill hashes, shared with `_inventory.md § Drills`:

```
git hash-object docs/first-encounter.md crates/happenstance/src/lib.rs
63073630c1c167e022193a989bfc6c3b1b5c32ea    docs/first-encounter.md
05c69822bc1512bbea295cdea33400d7a240318b    crates/happenstance/src/lib.rs
```

### D-3 — retag one cited clause id, on a page inside `TREE`. Expected: **red**.

Subject: **C-3**, `docs/first-encounter.md:130` — the ES-25 citation that closes step 3.

**The edit.**

```diff
--- a/docs/first-encounter.md
+++ b/docs/first-encounter.md
@@ -127,7 +127,7 @@ guarded above Some(SequencePosition(1)): ConditionViolated
 That line is printed from inside the matched arm, so it cannot appear unless the
 store really refused — which it is required to do, and required to report under
 exactly that name
-([ES-25](../spec/SPECIFICATION.md#es-25--condition-semantics)).
+([ES-999](../spec/SPECIFICATION.md#es-25--condition-semantics)).
```

**The command, and the output.**

```
$ cargo xtask narrative
  docs/first-encounter.md:130 — cites `ES-999`, which SPECIFICATION.md does not define

xtask failed: 1 problem(s) in docs
error: process didn't exit successfully: `target\debug\xtask.exe narrative` (exit code: 1)
```

That is the predicted string to the character
(`checked-documentation-surface/_design.md:337`, implemented at
`xtask/src/lint_narrative.rs:1231`).

**And the command the storymap's one-liner is shorthand for, on the same broken tree:**

```
$ cargo xtask spec-trace
201 clauses (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE), 112 conformance rules, 58 e2e cases, 401 citations checked (80 anchored to their subject, 12 external)
traceability: no problems found; §7.1–§7.2 matches the checker
                                                 # exit 0
```

`spec-trace` is **green against a page citing a clause that does not exist**. That is the
measurement behind the resolver table above, and it is why this audit names a resolver per page
rather than one per project.

**The revert.**

```
$ git hash-object docs/first-encounter.md
63073630c1c167e022193a989bfc6c3b1b5c32ea        # identical to pre-drill
$ git status --porcelain
                                                 # empty
$ cargo xtask narrative
  5 pages, all consistent
```

**Verdict: the mechanism fired, named the page and the line, and the tree returned
byte-identical.**

### D-4 — an undefined clause id on the crate root. Expected: **green, and that is the finding**.

The crate root cites **no** clause at all (`§ Claims`), so the drill *injects* one rather than
retagging one. It is written as the bare `ES-999` token, which is exactly what
`lint_narrative::clause_citations` (`xtask/src/lint_narrative.rs:1092`) scans for on a tree page,
link or no link — so both halves of the drill present the checker with the same input. The edit
**replaces** an existing line rather than adding one, so that
`crates/happenstance/tests/doc_budget.rs:122`'s `MODULE_DOC_LINES = 130` — currently met at
exactly 130 — cannot fire and disguise a citation result as a density result.

**The edit.**

```diff
--- a/crates/happenstance/src/lib.rs
+++ b/crates/happenstance/src/lib.rs
@@ -83,7 +83,7 @@
 //!
 //! Everything the contract crate exports is available here under the same
 //! paths, so nothing above has to be rewritten when the rest of the typed
-//! layer lands.
+//! layer lands (ES-999).
```

**The commands, and the outputs. Four instruments, all green.**

```
$ cargo xtask lints
=== every narrative page is checked ===
  5 pages, all consistent
=== every page declares one need ===
  5 pages, 16 rules, all consistent
                                                 # exit 0

$ cargo xtask spec-trace
traceability: no problems found; §7.1–§7.2 matches the checker
                                                 # exit 0

$ RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps
   Generated …/target/doc/happenstance/index.html
                                                 # exit 0

$ cargo test -p happenstance --tests
                                                 # 16 binaries, every one "test result: ok"
```

**The revert.**

```
$ git hash-object crates/happenstance/src/lib.rs
05c69822bc1512bbea295cdea33400d7a240318b        # identical to pre-drill
$ git status --porcelain
                                                 # empty
```

**Verdict: failed to fail, as expected, and recorded as *observed green* rather than skipped.**
A clause citation on the crate root is resolved by nothing. It is the same corpus boundary D-2
found for fences, met from the other side: `spec_trace` reads only `spec/SPECIFICATION.md`,
`lint_narrative` sweeps `TREE = "docs"`, and a relative markdown link is not an intra-doc link.
**Routed as `_inventory.md § Routing` R-5**, with the reason it is not designed around
(`crates/happenstance/src/lib.rs:7-9`) recorded there rather than restated here.

---

## Re-deriving this record

```
git grep -n "MUST" -- crates/happenstance/src/lib.rs docs/first-encounter.md \
    docs/carry-your-invariant.md docs/read-the-worked-example.md \
    examples/course-subscriptions/src/overview.md     # § Restatement pass 1; exits 1, no output
git grep -nE '\(\[[A-Z]{2}-[0-9]+\]' -- docs crates/happenstance/src/lib.rs
                                                      # every citation, by page and line
cargo xtask narrative                                 # every cited id resolves
cargo xtask spec-trace                                # the specification's own traceability
git grep -nF "#### ES-25 " -- spec/SPECIFICATION.md    # one anchor row; repeat per id
```

`§ Claims`' maturity column is re-derived from two places that must agree: the `**[FROZEN]**` /
`[PROVISIONAL …]` marker in the clause's own body, and the index row in
`spec/SPECIFICATION.md`'s maturity table (`:9048`, `:9082`, `:9099-9101`, `:9211-9212`). Both
were read for every id above and both agree; a disagreement would be a finding about the
specification, not about a page.

`§ Restatement` pass 2 is tier 5 — a person, recorded once (`_decomposition.md:486`) — and is
re-derived by re-executing `standards/pages/40-reviewing-a-page.md:68-90` over the five files,
not by a command.
