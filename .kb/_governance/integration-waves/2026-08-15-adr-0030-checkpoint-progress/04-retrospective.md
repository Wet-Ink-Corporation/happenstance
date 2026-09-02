# Wave `2026-08-15-adr-0030-checkpoint-progress` — retrospective

## How this wave was finished, and by whom

**The same split as `2026-08-13-projection-adrs`, and it is worth reading before anything else.**
The workflow ran all five operations, synced the three map atoms, wired the backlinks and ran both
checks. It then **halted at its own hard gate without committing**, because `redkiln doctor` exited
1. Under that gate three steps did not run: `_intake` was not cleared, this file and
`03-integration-summary.md` were not authored, and no commit was made.

They were completed afterwards, by hand, under an explicit human decision to treat the doctor
failure as out of scope. The five KB operations are the workflow's, unedited; the two summary files
and the intake clearing are not. `NF-004` makes this record the thing a later reader uses to tell
"authored by the process" from "authored to look like it", so the split is stated rather than
smoothed.

**This is the second consecutive wave finished this way.** That is the finding, not the footnote —
see below.

## Why the gate was judged out of scope, again

Nine errors, all one shape — `foundation story 'HS-S####' is consumed by no capability slice in
initiative 'from-contract-to-published-library'` — every one a `.bklg/` story-wiring finding. None
touches `.kb/`.

Established pre-existing **twice, independently**, exactly as last time: the workflow stashed every
file the wave touched, ran `doctor` against bare `HEAD` (`600e1bd`), reproduced the identical nine,
and restored; afterwards `doctor` was run against a **different worktree** at the same commit with
none of the wave's changes present, giving the identical nine with identical story ids.

The second check exists because the first uses `git stash`, whose stack is shared across every
worktree of this repository. **A check that must disturb shared state to prove isolation is worth
repeating without it**, and this is now the second wave in which that repetition was necessary.

The six template-drift warnings are the expected permanent set per `CLAUDE.md`, not findings.

**What is new since 2026-08-13, and it changes the severity.** The `projection-store-freeze` project
review established that **CI's `backlog` job asserts this list is empty**. So the nine errors are no
longer only a gate nuisance for KB waves — they will fail CI on the initiative's pull request. The
defect is filed upstream as `Wet-Ink-Corporation/redkiln#122`, which argues seven of the nine are
false positives produced by a one-hop predicate answering a reachability question. It has become a
release blocker for the initiative rather than tidiness, and it should be settled before
`publication-and-positioning`, not at the PR.

## What merged versus what was created

Three created, two amended, from a single document. The interesting number is not the ratio but the
**spread of layers**: one decision, one open question, one reference. A staging note written to
carry one ADR turned out to carry two further findings, and each belonged somewhere else. A wave
that had ingested only the headline decision would have dropped both.

Both amendments went to atoms that already existed and were already the right owners — no
near-duplicate was minted, and neither body was rewritten. **Five of the seven questions the
2026-08-10 wave parked now have answers**, every one arriving through the ADR pass rather than being
decided by an ingest. The abstention that wave made is still paying.

## Unresolved, and deliberately so

**1. PS-32's correction is filed but still not performed.** Op 4 turns a two-wave-old obligation
into a findable atom; it does **not** author the decision that supersedes ADR-0007, because this
wave received no such ADR and an accepted decision's body may never be edited. The atom's own
finding narrows what is owed: `kb-decision-0007`'s **atom** never repeats the defective sentence —
only `references/adr/0007-projection-runner-decodes.md:36-38` does — so the debt is a correction to
the long-form record, not a supersession forced by the atom's text. Whoever writes it should read
that distinction first; it may be cheaper than two waves of carrying it implied.

**2. `fresh_projection_has_no_checkpoint` is cited by PS-38 and PS-19 and does not exist**,
rendering `†` in §7.2. ADR-0030's atom records the fact. No atom in `.kb/` owns writing the rule and
this wave could not confidently place an owner — correctly, because the owner is a `.bklg` story
(`reset-rules`, HS-S0011), and a KB wave assigning backlog work would be the same category error as
an ingest minting a decision. The rule is what this whole wave exists to unblock.

**3. Claim 7 — PS-3, PS-31 and PS-36 share one shape** (a documentation obligation with no
instrument) and the repository already carries the instrument that would close all three
(`standards/rust/70-rustdoc-obligations.md`). Routed there rather than to a KB atom, and recorded
here so the routing stays visible rather than looking like an omission.

## What this wave says about the process

The 2026-08-13 retrospective ended by observing that a gate which can establish innocence but not
act on it will be routed around by hand every time. **It has now happened twice, in consecutive
waves, on the identical failure.** That is no longer an observation about one gate; it is a
prediction that came true within two days, and the hand-finishing is becoming the normal path rather
than the exception. Two waves is the point at which a workaround stops being a workaround.

The concrete cost so far is small — four files authored by hand that a workflow would have written.
The real cost is that each hand-finish is a place where the audit trail depends on someone choosing
to write down that they finished it manually. This record does. Nothing enforces that it must.
