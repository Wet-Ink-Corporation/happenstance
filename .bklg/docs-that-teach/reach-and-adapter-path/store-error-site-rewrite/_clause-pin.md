# The clause-id set this edit was made against, and the repair-versus-gap verdict

AC-007's **first artefact**, and N-6 step 1. A doc-comment rewrite inside
`happenstance-core` can silently un-discharge a frozen documentation MUST, and the
only thing that stops it is reading the pinned set before touching the file.

## EC-001 does not fire — the pin exists and is enumerated

HS-P0020's clause-id pin is committed and readable at
`xtask/src/lint_narrative.rs:1354`, as `const FROZEN_DOC_MUSTS: &[DocumentationMust]`.
It enumerates **21 clause ids**, each with a `Disposition` that is either `Pinned`
(discharged in the contract's own documentation, at a `site`, by a verbatim
`anchor` phrase) or `Excluded` (with the reason it is not the contract's
obligation):

```
VT-13  VT-15  VT-17  VT-21  VT-22  VT-24  VT-32  VT-33
ES-19  ES-23  ES-24  ES-26  ES-35  ES-40
PS-31  PS-34  PS-36  SY-32  CF-35  CF-39  CF-40
```

The set is enumerated rather than derived, so the story does not block
(`project.md`'s risk table; EC-001).

## The three entries pinned to this file

`crates/happenstance-core/src/store.rs` is the `site` for exactly three, all of
them `Pinned` by a **verbatim phrase** rather than by a line number — which is why
a doc comment growing above them cannot displace them at all:

| Clause | Anchor phrase | Where it lives now | Inside this story's edit? |
| --- | --- | --- | --- |
| **ES-19** | ``It is not a sound `after` for a follow-up condition`` | `store.rs:179` — the `append` doc comment | no |
| **ES-23** | `# Cancellation` | `store.rs:194` — the `append` doc comment | no |
| **ES-24** | `at-most-once under verbatim reissue` | `store.rs:217` — the `append` doc comment | no |

All three sit in **item** documentation, below `pub trait EventStore`. This story
edits the **module** `//!` comment at `store.rs:1-74` and adds two attributes at
`:139-140`; it changes no line any of the three anchors is on, and
`cargo xtask lint-narrative` — a mandatory gate step — re-reads all 21 after the
edit and reports them discharged.

`ES-26` is the fourth `ES` entry naming this port and is `Excluded`, with a reason
that names this project: *"both halves are documented but nothing says the
asymmetry is deliberate. Anchoring it needs a doc-comment edit, which is HS-P0023's
under `.kb/governance/rewrite-the-referent-never-the-reasoning.md`."* It is
**recorded and not taken**: no acceptance criterion of this story asks for it, its
subject is `append`'s error asymmetry rather than the `E0034` site, and doing it
here would be an unasked edit to a doc comment inside the pin. It is carried
forward in `## Notes` of this story's implementation report.

## Order, stated honestly

N-6 requires the set be read **before the first edit**. This story was implemented
across two sessions, and the truthful account is:

1. An earlier session (workflow `wf_37c6741d-fe1`) made the first edit to
   `store.rs` and was stopped mid-story at the human's request. Its work is in
   `10e99b2`, deliberately carrying **no** `Story:` trailer so the story re-runs,
   and its own record does not say whether the pin was read first.
2. This session read `FROZEN_DOC_MUSTS` in full **before its first edit** to
   `store.rs`, identified the three entries above, and confirmed each anchor phrase
   is still present and still outside the edited region — before the fence was
   re-pasted or a test was written.

Recorded this way rather than claimed cleanly, because the protocol's value is the
reading and not the sentence about it, and because a later reader comparing
`10e99b2`'s timestamp with this file would find the gap anyway.

## Repair or gap — the verdict is **repair**

The primary test is `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md`:
a correction is a **repair** if the set of implementations the clause admits is
unchanged, and a **gap** otherwise — and a gap is an ADR's business, not a
documentation story's.

Applied to each thing this change does:

| Change | Does it move what any clause admits? |
| --- | --- |
| Restoring both `= note:` lines and `TraitVariantBlanketType` to the `text` fence | **No.** The fence is an uncompiled illustration of a diagnostic. No clause cites it, and no adapter can implement it rightly or wrongly. |
| Adding the plain-words diagnosis, the narrow-limit sentence and the pointer | **No.** Three sentences of module prose. No clause's obligation is on module prose, and none of the three states a normative requirement — the limit sentence cites a guard that already exists rather than creating one. |
| Two `#[doc(alias)]` attributes on `pub trait EventStore` | **No.** Erased before type-checking; they add no path a caller can write, no bound, no signature and no feature. Identical on both flavours and on `wasm32`. |
| Bumping 49 `store.rs:NNN` citations by one line | **No.** Line numbers only — proved mechanically: every changed line is identical to its predecessor once the number is masked. No clause **text** changed. |

Verdict: **repair** on all four. EC-009 does not fire and no ADR is owed.

The secondary test is `.kb/governance/rewrite-the-referent-never-the-reasoning.md`,
and the story says plainly what it is doing with it: all three of that atom's
worked instances are ADR-record renames, so applying it to a doc comment is an
**analogy sanctioned by name in `CLAUDE.md`** — not a fourth worked instance, and
not a licence to widen the atom. The reasoning (which clause admits what) is
untouched; only the referent (which line a citation names) moved.
