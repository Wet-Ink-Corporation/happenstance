# 30 — Citing the specification

> **Load when:** writing a sentence that sounds normative · reaching for a `MUST`

> **See also:** 00 (the declaration) · 20 (the fold line) · 40 (the reviewer's walk)

---

**What nothing here checks.** Whether a cited clause id *resolves* is
`spec/SPECIFICATION.md`'s own traceability step and HS-P0020's `clause_ids`
accessor, never this tree's; a second parser for it is forbidden. And whether a
page **restates a clause it correctly cites** is checked by nothing mechanical at
all: a page can carry a real, resolving id and put the clause's content in the
paragraph underneath, and no byte count sees it. Band 40's paraphrase spot check
is the instrument for that. A green gate is not a claim about paraphrase.

## RP-30-1. Decide clause or page with the falsification question.

**Why.** A page and the specification are two voices, and the risk this band
exists for is a page quietly becoming the second one. One question separates
them, and it has a yes/no answer.

**Do** Ask: *could a conformant adapter written in another language violate this
sentence?* If yes, it is a clause: it belongs in `spec/SPECIFICATION.md` and the
page cites it. If no, it is an idiom about how this repository writes pages, and
the page owns it outright.

**Not** Deciding by how important the sentence feels, or by whether it already
sounds like a rule. Both answer a different question, and both let a normative
claim land in a place nothing traces.

**Rejects.** A page that states, in its own voice, that an append must re-read
its condition before committing. It reads as authoritative, it is correct today,
and the day the clause is amended the page becomes a second answer that nothing
updates — found by the next reader who trusts the page over the specification.

**Evidence.** `standards/rust/README.md:32-36` (the same test, one tree over: an
atom never restates a clause's content) · `spec/SPECIFICATION.md:278-280` (clause
ids and the traceability obligation)

## RP-30-2. Cite the clause id; never restate the clause.

**Why.** A restatement is a copy, and one of two copies goes stale without either
one changing. Citing hands the reader to the voice that is authoritative and
keeps the page's job to explaining why it matters.

**Do**

```text
Appending under a condition re-reads before it commits (ES-25). What that buys
you is the ability to decide against a world you have seen.
```

**Not**

```text
Appending under a condition re-reads before it commits: the store MUST re-run
the condition's query and MUST reject the append when a matching event exists
at or after the stated position.
```

**Rejects.** A page whose explanatory paragraph is a faithful paraphrase of the
clause it cites. Every word is true, the id resolves, the gate is green — and the
first amendment to that clause leaves a page that contradicts the specification
while still citing it, which is worse than a page that never cited anything.

**Evidence.** `xtask/src/spec_trace.rs:107-121` (the traceability machinery a
citation joins) · `standards/rust/README.md:99-107` (the atom shape, and its own
`Evidence.` ordering rule)

## RP-30-3. Write the citation as the stable id, in visible link text.

**Why.** Clause ids are stable names and are never renumbered, so a citation
written as an id survives a specification that grows underneath it. A line number
does not, and a bare link hides which authority the reader is being handed to.

**Do** Write the id itself as the visible link text — `ES-25`, `CF-6`, `PS-3` —
so a reader scanning the page can see, without following anything, that the
sentence is the specification's rather than the page's.

**Not** A line number, a section title, a bare "the specification says", or a
link whose visible text is "here". Each of the four either rots or hides the
authority.

**Rejects.** A page citing `spec/SPECIFICATION.md:1462`. It resolves on the day
it is written and points at an unrelated clause the next time the file grows,
and the reader who follows it reads the wrong rule with no way to tell — the
failure mode stable ids exist to remove.

**Evidence.** `spec/SPECIFICATION.md:280` (clause ids are stable and are never
renumbered) · `standards/rust/README.md:23-29` (a clause outranks this tree, so
a citation is a hand-off upward)
