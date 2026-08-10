# A cross-reference checker must verify the referent, and report its own coverage

## Where this expects to land

**Layer: none of the three scaffolded ones.** This is a transferable engineering practice
about tooling, not a persona (`product/`), an interaction pattern (`design/`), or something
unsettled (`open-questions/`). It belongs at the **KB root**, or in a `playbooks/` layer if
one is created. **Kind: `playbook`** — it prescribes what to do when building or reviewing a
cross-reference check.

`source_paths`: `xtask/src/spec_trace.rs`, `xtask/src/lint_constitution.rs`,
`docs/architecture/SPECIFICATION.md`, `docs/RUNBOOK.md`,
`docs/evaluation/phase-4-5-reconciliation.md`, and this intake file.

## The claim

A checker over cross-references has two independent obligations, and satisfying the first
while silently failing the second produces a green step that means nothing:

1. **Verify the referent, not the address.** That a `file:line` resolves — the file exists,
   the line is within bounds — says nothing about whether the thing the sentence attributes to
   that location is there.
2. **Report your own coverage.** A check that does not state what fraction of the corpus it
   parsed is **indistinguishable from one that sees all of it**, and a reader has no way to
   tell the two apart from the outside.

The second is the sharper of the two, because a checker with a narrow filter degrades in
exactly the way that produces confidence: it never reports a false positive, it passes on
every run, and nobody ever has cause to look at it.

## The measurement this comes from

`xtask/src/spec_trace.rs` runs as a step of `cargo xtask ci` and, among other things, verifies
that the ``file:line`` citations in `docs/architecture/SPECIFICATION.md` resolve. Its parser
(`fn citations`, `xtask/src/spec_trace.rs:2199`) filtered on two conditions before this pass:
the citation had to be path-qualified, and it had to name a `.rs` or a `.toml`. The old filter,
recorded verbatim in that function's own doc comment:

```text
!path.contains('/') || !(has_ext(".rs") || has_ext(".toml"))
```

Against the document as it then stood:

| | count |
| --- | --- |
| citations in the document | 338 |
| satisfying **both** conditions, and therefore checked | **84** |
| bare file names (`memory.rs:154`), never parsed | 200 |
| pointing into Markdown, never parsed | 56 |

`check_citations` was **not a weak check over the corpus. It was a correct check over a
quarter of it**, and three quarters of the specification's evidence sat unverified behind a
green step. The step printed "no problems found" the entire time.

That is not a hypothetical exposure. An earlier commit, `2e4407b`, had found eight citations
that were "green and wrong" and repaired them by hand — the same defect, in the quarter of
the corpus anyone could see.

## Both halves of the fix, and why the second is the durable one

`a843b99` widened the parser: bare names resolve through a workspace index built by walking
the tree (`fn workspace_index`, `xtask/src/spec_trace.rs:2153`), with a hand-checked
`BARE_NAME_MAP` (`:2037`) for the four basenames the workspace defines more than once.
Coverage went from 84 parsed to 358 checked.

`84dcc67` then added the referent check: a citation must be *about* the identifier the
sentence attributes to it. That mechanism is its own lesson —
see `lesson-anchoring-citations-in-a-long-lived-document.md`.

But the change worth generalising is the smallest one. The summary line now carries the
number:

> … 95 conformance rules, 58 e2e cases, **358 citations checked (69 anchored to their
> subject, 12 external)**

The code says why, at `xtask/src/spec_trace.rs:291-296` and again at `:985-988`:

> Returns how many it looked at, because a check that does not state its own coverage is
> indistinguishable from one that sees everything — which is exactly how this step reported
> success while parsing a quarter of the corpus.

> The coverage number is in the summary rather than in a comment because the failure this step
> spent a phase inside was not a wrong check, it was a check whose scope nobody could see. A
> reader who is told "338 citations" can notice that the document has more.

**A reader who is told a number can falsify it. A reader who is told "no problems found"
cannot.** That is the whole of the mechanism: the coverage figure converts an unfalsifiable
claim into a falsifiable one, at the cost of one `write!`.

## Generalisation

For any checker over cross-references — citations, links, IDs, symbol references, schema
`$ref`s, test-to-requirement traceability:

- **Print the denominator.** How many candidates existed, how many you parsed, how many you
  actually verified. Three numbers, not one. `spec-trace` prints all three because "checked"
  and "anchored to their subject" are different populations and collapsing them would restore
  the original defect at a higher coverage level.
- **State the population you declined.** Twelve of the 358 are external
  (`EXTERNAL_CITATIONS`); one is deliberately unanchored because §2.7 quotes a claim in order
  to call it false (`UNANCHORED_CITATIONS`, `xtask/src/spec_trace.rs:389-392`). An exemption
  that is counted and named is a decision; an exemption that is a filter is a blind spot.
- **A narrowing filter is a silent scope reduction.** Every `continue` in a parser is an
  unverified population. If you cannot name the size of what each one skips, you do not know
  what your check covers.
- **Suspect a check that has never failed.** The absence of findings over a long period is
  evidence either that the corpus is clean or that the check is not looking. Only the
  coverage number distinguishes them.

## The counter-consideration, stated honestly

Coverage reporting does not make a check correct — it makes its scope legible. 358 of 358
citations are now checked for addressing, but only 69 are checked for referent, because a
derivable subject exists for only that many. **81% of the corpus is still verified only for
addressing.** The summary line says so, which is the point; a summary that reported "358
citations verified" and stopped would have re-created the original problem one level up.
