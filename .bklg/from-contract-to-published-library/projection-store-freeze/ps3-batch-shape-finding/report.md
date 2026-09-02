---
item: "HS-S0014"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — The PS-3 evidence written as a finding, not a verdict

## Findings Ledger

**Eight of eight ACs satisfied. Nothing deferred, nothing blocked, nothing
frozen edited, no `.kb/` atom written, and no verdict made.** One document at a
fixed citable path, two mounts — one of them machine-checked on every gate run —
and a per-rule ledger covering the whole enumeration rather than the rules an
author remembered.

| AC | Result | What proves it | Where it is mounted |
| -- | ------ | -------------- | ------------------- |
| **AC-001** | **satisfied** | The finding exists at exactly `references/evaluation/projection-batch-shape-evidence.md`, and its first block carries all three lifecycle facts: date `2026-08-14`, the commit sha of the run it reports (`cfd9231` / `cfd92313cc59e059655fe130f3c8c31b19dfcf08`), and *"immutable evidence. Superseded rather than edited"* with its citation into the directory's own lifecycle rules. `git status --porcelain .kb/` is **empty** for this PR — no decision atom, no reference atom, no `_intake` file — and `redkiln validate --kb` reports `validate passed.` The planner is reading evidence they may still decide against. | `references/evaluation/projection-batch-shape-evidence.md:1-11` |
| **AC-002** | **satisfied** | §1 names the evidence base **before** reasoning from it: both fixtures by their real identifiers in a table (`MemoryProjectionFixture` over `MemoryProjectionStore`, `projection_memory.rs:101`; `BufferingProjectionFixture` over `BufferingProjectionStore`, `…/projection_mutation_coverage/buffering.rs:252`), both harness targets by path, and **one** `cargo xtask ci` invocation with its sha — not two `cargo test` runs reconciled by hand, and it says so. `git cat-file -e cfd92313…` resolves on this branch. One deliberate divergence, in the Notes below. | `…/projection-batch-shape-evidence.md:20-51` |
| **AC-003** | **satisfied** | §2 states all six labels operationally **ahead of** the ledger: D1 per-shape handling · D2 asymmetric declension (naming the capability constants that produce it) · D3 assertion loosened (marked retrospective, read off the diff not the run) · D4 divergent observable behaviour · **agreed** · **not comparable**, with the explicit instruction that the last is never folded into **agreed**. Mechanical: the verdict labels used in the ledger column are `{agreed, D2}` — a subset, no seventh minted. | `…:53-71`, before `…:73` |
| **AC-004** | **satisfied** | Sixteen numbered rows, one per rule, in enumeration order. Compared **mechanically in both directions** against the arms of `for_each_projection_store_rule!` rather than eyeballed: arms 16, rows 16, order identical, in-enum-not-in-ledger `[]`, in-ledger-not-in-enum `[]`, duplicates `[]`. The comparison is recorded as run in `implementation-report.md`'s `## Gates`, with the script. | `…:79-96`; enumeration at `crates/happenstance-testkit/src/projection.rs:1843-1881` |
| **AC-005** | **satisfied** | Two D2 rows, no *not comparable* rows, and both D2 rows are `RuleOutcome`-grounded. Row 3 cites `ProjectionFixture::COMMIT_FAULT` (`contract.rs:640`) and states that the skip is a `RuleOutcome::Skipped` **value** collected by the harness (`contract.rs:473-537`), *"not a remembered line"*. Row 12 cites `ProjectionFixture::RESET_REFUSAL` (`contract.rs:604`) and says the same. The declension text quoted in each is the fixture's own stated reason carried on the constant, not a transcription of stdout. | `…:83` and `…:92` |
| **AC-006** | **satisfied** | §4 exists, is named, and is the **longest** section — which is the shape AC-006 anticipates for this outcome. It states the result as a result (*"on the batch-representation axis … the two shapes did not disagree anywhere"*), then addresses **both** of Note 10 item 2's readings by name. Reading A is supported in part with three cited rows where a rule accepted two different mechanisms without noticing; reading B is supported and **established as a fact** about the tree rather than an interpretation. The conclusion is explicit and sharper than the admissible fallback: reading B holds, and it makes reading A untestable on this evidence, *"because a rule cannot be shown blind to an axis that neither of the two subjects sits on either side of."* Not *"no disagreements observed"*, and not omitted. | `…:121-172` |
| **AC-007** | **satisfied** | Two mandatory statements, both terminal, and no third. §5 states that **PS-2's bar is not met by anything this project built alone**, cites `spec/SPECIFICATION.md:4760-4775`, quotes the `Rejects:` clause **verbatim** including *"the same storage shape wearing two hats — the exact monoculture CLAUDE.md's spread rule exists to catch"*, and closes *"This document makes no claim about when PS-2's bar will be met, or by what."* §7 — the **last** section — states that no exposure verdict is made here, names HS-P0016 (via `projection-port-ship-shape`) as owner of the `unstable-projection` call and HS-P0015 (via `freeze-verdict-document`) as owner of the freeze verdict, and ends *"Neither call is made here, and neither is recommended."* NF-004 lexical sweep over the whole document: the only hits for *should / recommend / suggests / leans / is ready / is proven* are the two negations in §7 that constitute the guard. | `…:174-187` and `…:207-222` |
| **AC-008** | **satisfied** | Both mounts, from both directions a reader travels. Human path: one index entry in the README's *"Later additions, which are neither"* section, in that section's existing shape. Machine path: one additive paragraph in PS-3's body citing the finding by `file:line`, resolved by `cargo xtask spec-trace` — **red first** (*"names a file that does not exist"*, exit 1), then green with 359 citations checked. `cargo xtask ci`: `all checks passed`. `git diff spec/SPECIFICATION.md` is `1 file changed, 4 insertions(+)`, all inside PS-3: PS-2 byte-identical, PS-3's `[PROVISIONAL]` marker and `Rule:` / `Cases:` / `Rejects:` untouched, no other clause, generated §7.1–§7.2 unchanged. | `references/evaluation/README.md:91-112`; `spec/SPECIFICATION.md:4788-4790` |

**Deferred: nothing. Blocked: nothing.**

### The finding is not a null result, and that matters to its two recipients

The story was written to be able to report *"they agreed everywhere"* honestly.
The run produced something more useful, and the vocabulary fixed in advance is
what kept it from being smoothed into either extreme.

**Fourteen agreed, two D2, no D1, no D3, no D4, none *not comparable*.** The two
D2 rows are an asymmetry in what the two runs **assert** — the buffering run
asserts sixteen rules, the reference run asserts fourteen and reports two honest
declensions — and the cause is that one store is shipped and the other is an
instrument. Recording them as **agreed** would have overstated the comparison.

**D3 is zero and provable rather than asserted.** The pinned commit touches
`crates/happenstance-testkit/tests/**` and nothing else; `…/src/**`, where every
rule body and assertion lives, has no diff in it. No assertion was generalised to
let the second shape pass.

**The sharpest result is about the axis, not about the rules.**
`MemoryProjectionStore` is *itself* already a deferred write set
(`crates/happenstance-core/src/projection_memory.rs:280-330`) — it acquires no
handle, opens no transaction and holds no lock between `begin` and `commit`. §4.11
assigned the projection family's CF-5 variant *"the buffering adapter PS-4
permits"* on the understanding that it sat at the other end. It does not. So the
pair spans batch **representation** (an ordered journal versus a delta collapsed
at stage time) plus port flavour, and **not** whether a handle is held across the
batch.

That is stated as a fact with its citation and left there. It is precisely the
kind of thing an evidence document exists to hand a planner, and precisely the
kind of thing a verdict would have foreclosed.

### What a downstream reader can take, and what they cannot

**Can take:** no projection rule assumed a lock was held, a transaction was open,
or a connection existed between `begin` and `commit` — established against three
in-process stores in one run, and it is the sentence PS-4's `Rejects:` clause asks
for.

**Cannot take:** any claim about PS-2, about the `unstable-projection` exposure,
about whether the freeze held, or about when PS-2's bar will be met. Every store
the projection rules have met is in this process; nothing crosses a transaction
manager, a pool under contention, or a server that can refuse half a request.

### Named anti-patterns, and how each was refused

- ***"Both batch shapes passed the whole suite; the port is proven against the
  batch-shape axis"*** (`discover.md:75-87`) — every word true, functions as a
  verdict. Refused twice: §4 establishes that the pair does not span the axis, and
  §5 refuses the PS-2 reading in one direction only.
- **The null result as silence, or as *"no disagreements observed"***
  (`discover.md:89-98`) — refused by §4, which is the document's longest section
  and weighs both candidate readings by name.
- **The finding written as a `.kb/decisions/` atom** (`discover.md:100-107`) —
  refused by construction: it lives in `references/evaluation/`, whose whole
  definition is *evidence kept for citation, binding nothing*, and
  `git status --porcelain .kb/` is empty. HS-P0016 will **decide** PS-3 rather
  than **supersede** a decision this project had no authority to make.

### Notes for the reviewer

**AC-002's criterion says "apply-on-write `MemoryProjectionStore`". The finding
deliberately does not repeat that**, because it is false, and an evidence
document that restates a plan's error in order to match its wording is worth
less than no document. The criterion's operative requirements are all met; the
characterisation is corrected in §4 with its citation. The criterion itself was
not re-worded.

**One in-place edit was made to a citation, and it is the permitted one.**
Inserting the README index entry pushed the supersede-never-edit block from
`:117-119` to `:140-142`, and the finding's header citation was repointed before
the document was committed. That is the single exception
`references/evaluation/README.md` allows, *"because it changes no claim, only
whether a reader can follow one"* — and no reader had followed the old numbers,
because the document had not been committed yet.

**Nothing found here was fixed here.** §4 describes three rules whose reach is
narrower than a reader might assume; §6 states that this is a description of what
a port-level rule asserts rather than a defect filed against it, and that a rule
which turns out to be wrong is repaired in its own story with the reason in the
same change.
