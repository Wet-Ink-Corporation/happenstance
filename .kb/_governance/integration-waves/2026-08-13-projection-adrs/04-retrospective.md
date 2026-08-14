# Wave `2026-08-13-projection-adrs` — retrospective

## How this wave was finished, and by whom

**Read this first; it is the wave's own provenance.** The ingest workflow ran all six operations,
synced the three map atoms, wired the reciprocal backlinks and ran both checks. It then **halted at
its own hard gate without committing**, because `redkiln doctor` exited 1. Under that gate, three
steps did not run: `_intake` was not cleared, this file and `03-integration-summary.md` were not
authored, and no commit was made.

They were completed afterwards, by hand, under an explicit human decision to treat the doctor
failure as out of scope for this wave. That decision is recorded here rather than smoothed over,
because `NF-004` makes this record the thing a later reader uses to tell "authored by the process"
from "authored to look like it". The six KB operations themselves are the workflow's, unedited; the
two summary files and the intake clearing are not.

## Why the gate was judged out of scope

The nine doctor errors are all one shape — `foundation story 'HS-S####' is consumed by no capability
slice in initiative 'from-contract-to-published-library'` — across `HS-S0002`, `-0034`, `-0035`,
`-0067`, `-0074`, `-0075`, `-0100`, `-0108`, `-0120`. Every one is a `.bklg/` story-wiring finding
in an in-flight initiative. None touches `.kb/`.

It was established as pre-existing **twice, independently**:

1. The workflow's own isolation check stashed every file the wave touched, ran `doctor` against bare
   `HEAD` (`511f206`), reproduced the identical nine errors, and restored.
2. Afterwards, `redkiln doctor` was run against a *different worktree* sitting at the same commit
   with none of the wave's changes present. Identical nine errors, identical story ids.

The second check exists because the first used `git stash`, whose stack is shared across every
worktree of this repository — a check that has to disturb shared state to prove isolation is a check
worth repeating without it.

The six template-drift warnings (`discover.md`, `gates/discover.md`, `gates/intake.md`, `spec.md`,
`_design.md`, `_intake-brief.md`) are the expected permanent set per `CLAUDE.md` and are not
findings. The set is exactly six, which is itself the assertion the `backlog` CI job makes.

**The debt is not closed by this wave and is not this wave's to close.** It is routed as a finding
against the initiative's own backlog, where the storymap wiring lives.

## What merged versus what was created

Three created, three amended — and the ratio is the point. The 2026-08-10 wave created twelve atoms
and amended none, because the corpus was empty. This wave found owners for half its work: every
question it answered already had an atom waiting, and the authority model's merge-and-link bias
resolved to real merges rather than to near-duplicates.

The clearest instance is **Op 5**, a genuine cross-file merge: `ps-19-scope-narrower-than-its-rule`
received material from *two* intake documents — the ADR-0018 record and the clause-pairing sweep —
which arrived at the same finding independently. One atom, two sources, no second atom minted to
hold the second arrival. **Op 6** is the same shape: the sweep and ADR-0019 reached the
intent-versus-sentence cause by different routes (the sweep by census, ADR-0019 through PS-29), and
both landed in the one existing question rather than in a new one.

## Unresolved, and deliberately so

Six items came back unresolved. They are not failures of the wave; five are refusals it was right to
make, and the first is owed to a human.

**1. A supersession of `kb-decision-0007` is OWED and was not performed.** ADR-0017's record states
that it corrects one sentence of ADR-0007's Context per PS-32, and that the correction is a
superseding atom's work rather than an edit's. Both halves were honoured — 0007's body was not
edited, and no superseding atom was written — because authoring that atom would be this wave taking
a decision nobody signed. **A human must decide whether the correcting atom is owed now or at
HS-P0011.** This is the one item here that is a live obligation rather than a closed refusal.

**2. `experiments/live-handle-projection-batch/` does not exist.** `kb-decision-0017` records the
disposition ADR-0017 decided — move `LiveHandleProjectionStore` there rather than delete it — but
the move is a `crates/**` and `experiments/**` change this wave is forbidden to make. The path was
deliberately kept out of `source_paths` so no atom cites a directory that is not there.

**3. Four pairing defects got no atom, and two identical ones already have atoms.** PS-8, PS-13,
PS-28 and PS-29 are each named in the decision atom whose clause range contains them and attributed
to an owner that exists in `.bklg/` — `unstable-projection-gate-and-clause-disposition` (HS-P0010,
AC-014) or HS-P0011 — because the open-questions README bars filing work as a question. PS-1 and
PS-19 have atoms for the identical shape. **The asymmetry is real**, and a human may want it
corrected in either direction.

**4. No reference atom was minted for `references/evaluation/ps-clause-pairing-sweep.md`**, though
the option was live and `kb-reference-phase-4-5-spec-reconciliation-001` is the precedent. Declined
on the source document's own stated position, on the sweep being registered evidence pinned to
`2136dde` and citable by path today, and on authority rule 3's bias against new atoms. Five
destinations in this wave cite it by path. If a later wave mints it, both the PS-1 and PS-19
questions should gain the edge, and the atom should carry the verdict, threshold and tally — not the
census.

**5. Two candidate questions the sweep named and declined**, recorded so the declination stays
visible: (a) PS-3, PS-31 and PS-36 carry a documentation obligation with no instrument, routed to
`standards/rust/70-rustdoc-obligations.md` rather than to a KB atom; (b) `rebuild_is_chunk_size_invariant`
is ungated while `batch_reads_reflect_pending_writes` carries `READS_THROUGH_BATCH`, so a
write-behind adapter blessed by PS-4 and PS-12's second arm cannot pass it — routed to ADR-0017 and
the suite stories. Neither has a KB destination in this plan.

**6. `kb-open-question-adr-status-vocabulary-001` is untouched for the third wave running** while
its workaround is applied for the third wave running. The ADR-0017 intake forbids answering it by
acting on it, and nothing in this plan gives it an owner or a forcing event. Three waves of silent
compliance is the point at which a workaround stops being temporary; it is named here so the fourth
wave inherits the observation rather than rediscovering it.

## What this wave says about the process

The 2026-08-10 retrospective parked thirteen `requires-new-decision` claims as seven questions rather
than minting decisions an ingest had no authority to make. This wave is the return leg, and it
worked: three of those questions were answered by decisions that arrived through the ADR pass with
records, alternatives and a sign-off behind them. The abstention cost one wave of latency and bought
a corpus where every accepted decision can name who decided it.

The one thing that did not work is the gate. A hard gate on `redkiln doctor` means any unrelated
`.bklg` debt anywhere in the repository blocks every KB wave, and the wave's own isolation check —
which correctly proved the failure was not its own — could not act on what it had proved. A gate that
can establish innocence but not accept it will be routed around by hand every time, which is exactly
what happened here.
