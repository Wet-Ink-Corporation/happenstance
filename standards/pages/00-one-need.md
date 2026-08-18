# 00 — One need per page

> **Load when:** starting a narrative page · a page that seems to answer two questions

> **See also:** 10 (the need set) · 20 (the fold line) · 40 (the reviewer's walk)

---

Every governed page answers one reader's question and says which one, in a line
that a reader and a check both read. *Which* needs exist is band 10's subject;
this band fixes the count, the form, the position, and that the line is never
hidden.

## RP-00-1. Declare exactly one need on every governed page.

**Why.** A page that quietly answers two questions reads as complete and is not.
The reader who came for the second question gets a partial answer with no signal
that it is partial, and nothing makes the overload visible until someone rewrites
the page.

**Do**

```text
# Append conditions

> **Answers:** `explanation` — Why does a write re-read what it decided on?
```

**Not**

```text
# Append conditions

> **Answers:** `explanation` — Why does a write re-read what it decided on?
> **Answers:** `how-to` — How do I append under a condition?
```

**Rejects.** A page that declares two needs, or none. The reader who came for the
second need reads the page's shape, concludes the answer is missing and leaves;
the author finds out at the next rewrite, which is exactly the moment the split
would have been cheapest. A page with no declaration is the worse half: nothing
says what it is for, so nothing about it can be wrong.

**Evidence.** `standards/rust/README.md:32-36` (the same one-owner test one level
up — an atom never restates a clause) ·
`.bklg/docs-that-teach/page-need-discipline/_design.md` (S1) ·
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` (the
open-ended-sink anti-pattern, first entry)

## RP-00-2. Put the declaration immediately after the H1 and interpose nothing.

**Why.** The test a reader applies is *read nothing but the region above the first
prose paragraph and name the need*. Any element between the title and the
declaration makes that region ambiguous, so the position carries as much of the
meaning as the token does.

**Do**

```text
# <Page title>

> **Answers:** `token` — <question>?

<first prose paragraph>
```

The line sits immediately after the page's `# Title`, with one blank line above
and one below. The token is one member of band 10's set, written in backticks;
the clause after ` — ` is a question in the reader's voice, ending in `?`. The
whole line is at most 96 characters, and 80 is the target, because 80 is what a
plain-text pager and a `git diff` show without wrapping. When the line will not
fit, the question shortens and the token never does — and if the question still
will not fit, the page is answering more than one need:
the overflow is the diagnosis.

**Not**

```text
# Append conditions

_Last updated 2026-08-18_

> **Answers:** `explanation` — Why does a write re-read what it decided on?
```

Nothing may be interposed: no badge row, no table of contents, no admonition and
no "last updated" line.

**Rejects.** A page whose declaration is correct and sits under a status badge, a
table of contents or an "updated on" line. It passes every reading of the words
and fails the only test that matters — a reader scanning the head now has three
candidates for *what this page is for*, and the authoritative one is the one they
cannot tell apart from the other two.

**Evidence.** `standards/rust/00-prime-directives.md:3-9` (this repository's only
enforced line-level, human-visible, machine-read page head) ·
`xtask/src/lint_constitution.rs:247-255` (the parser that reads it) ·
`.bklg/docs-that-teach/page-need-discipline/_design.md` (Composition — S1; the
Density budget)

## RP-00-3. Keep the declaration visible in every state.

**Why.** The declaration's whole function is being read before the page is. A
label that appears only after something is opened has not been read by the reader
who did not open it, and that is every reader in a plain-text pager, in a print
view and in a `git diff`.

**Do** The declaration is rendered in the document by default, in every state and
in every medium, and it may never sit behind a fold, a tab, an inactive panel or
a disclosure control. *What counts as such a mechanism* — including a wrapper the
renderer supplies and opens by default — is band 20's answer, and this band does
not settle it.

**Not** A page whose declaration sits inside a collapsed "About this page"
section, so the page opens on its first prose paragraph. This atom names that
shape in words rather than showing it in a fence: an atom writing the rule
against hiding may not contain the markup it forbids.

**Rejects.** A page that is conformant read in a browser with everything expanded
and unlabelled in every other medium — a pager, a print view, a diff, or a reader
who collapsed the section once and never opened it again. The author finds out
when a reviewer running band 40's walk records `indeterminate`, which is a defect
in the page rather than in the procedure.

**Evidence.** `standards/rust/81-checks-that-cannot-be-types.md:11` (a check whose
limits are undocumented is read as a guarantee — the same failure applied to a
label a reader never sees) ·
`.bklg/docs-that-teach/page-need-discipline/_design.md` (Transience policy, the
S1 declaration row; DT-8 Part 2, class 1)
