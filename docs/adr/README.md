# The ADRs moved to `.kb/decision/`

Every architecture decision record now lives in the knowledge base, one atom per
ADR, at [`.kb/decision/`](../../.kb/decision/). The filenames are unchanged, so
`docs/adr/0016-the-wire-format.md` is now `.kb/decision/0016-the-wire-format.md`
and `git log --follow` resolves across the move.

This file is the only thing left here. It exists because git history, older
commit messages and any link somebody saved point at this directory, and a reader
who lands on a 404 has no way to find where the ADRs went.

## What changed, and what did not

**Not the content.** The bodies are byte-identical to what was here; each gained
a YAML frontmatter block above its `# ADR-NNNN:` heading so that
`redkiln validate --kb` can read it.

**Not the authority.** An ADR still records *why* a decision was taken and
*when*, and [`docs/architecture/SPECIFICATION.md`](../architecture/SPECIFICATION.md)
still records *what is true now*. Where the two disagree, the ADR is history and
the specification is current.

**What did change** is that the immutability rule is now enforced rather than
observed. An accepted decision atom cannot be edited — `redkiln validate --kb`
checks each one against `HEAD` — so correcting one means writing a new atom that
supersedes it, which is what the ADR discipline always asked for and nothing
previously checked.

## Do not add a file here

`cargo xtask lint-adr-paths` fails the gate on any reference to `docs/adr/` anywhere in the repository, unless the same line also names `.kb/decision/` — because a link to a directory holding only this note is a link that has silently stopped working.

A line naming both paths is a line *about* the move, like the one above. That is the exemption, rather than a list of excused files: excluding files would have blinded the check to the two `xtask` modules that carried links to the old path in the first place. It is per **line**, so a sentence mentioning both has to keep them on one line — this paragraph is wrapped that way deliberately.

A new ADR is a new atom in `.kb/decision/`; see the queue in
[`../RUNBOOK.md`](../RUNBOOK.md) for the next free number.
