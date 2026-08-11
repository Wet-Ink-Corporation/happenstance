# `_intake` — the staging area for KB ingestion

Drop documents here that you want folded into the permanent knowledge base. This directory is
the **default input to `/redkiln:kb-ingest`**: run it with no argument and it ingests all of
`.kb/_intake/*.md`; pass a file or a glob to narrow the wave to part of it.

Nothing staged here is an atom yet. These are raw material — a design note, a pasted transcript,
a hand-written brain-dump — and they are not held to `KbFrontmatter`. The ingest run extracts the
claims, adjudicates them against the corpus that already exists (biased hard towards **merging
into an existing atom** over spawning a near-duplicate, and towards **superseding** an accepted
decision over editing it), and writes what survives as real atoms in the permanent layers.

## A successful ingest clears this directory

That is the contract, and it is what makes `_intake` readable: **a file still sitting here after
a run is a file that run did not ingest.** Clearing it loses nothing. The whole wave — the new
and amended atoms _and_ the removal of the staged sources — is committed together on its own
worktree branch, so the source document stays in git history, and every atom the document fed
cites its `.kb/_intake/...` path in `source_paths`.

## `_`-prefixed directories are reserved

`_intake/` and `_templates/` are redkiln's, and `redkiln validate --kb` skips any directory whose
name begins with `_`. That skip is deliberate rather than incidental: a draft you are still
staging must not have to satisfy the atom schema to sit here, so validation has to be blind to
this whole tree. Do not put atoms in a `_`-prefixed directory expecting them to be checked, and
do not name a real KB layer with a leading underscore.

This README is skipped by the same rule, which is what lets the scaffolded file be a README rather
than a `.gitkeep`: it is free to say what the directory is for without having to pass as an atom.

**Why a file at all?** Not because the copy would drop an empty directory — `init`'s `copyTree`
creates its destination before it iterates, so an empty bundled directory _is_ scaffolded, nested
ones included. The reason is one step earlier: **git does not record empty directories.** An empty
`kb-skeleton/_intake/` would be absent from every clone of the redkiln repository, so there would be
nothing for the copy to find. A tracked file is also what makes the directory visible to `doctor`'s
`kb-skeleton-absent` check, which walks the bundled skeleton for _files_ — a directory-only entry
could never be reported missing.
