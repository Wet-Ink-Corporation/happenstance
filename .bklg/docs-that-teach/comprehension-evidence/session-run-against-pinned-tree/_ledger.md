---
item: HS-S0165
stage: implement
created: "2026-08-17T13:16:19.552Z"
updated: "2026-08-17T13:16:19.552Z"
---

# Acceptance ledger — Run the session against the named assembled tree

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

`mount_point` throughout is the friction log landed by `friction-log-skeleton` at
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` (the path its spec binds and mounts
at `project.md`'s `## Companions`); where `_design.md` fixes a different path, it substitutes
verbatim, per `spec.md`, `## Integration contract` and EC-010. `verifying_test` values are real
checks against real paths rather than test-file ids — this project has no functions and no test
binary, and per `_decomposition.md`'s testing brief each check asserts on content, not merely on
structural presence.

```yaml
- id: AC-001
  criterion: "GIVEN U3 opens the log six months from now and needs to know whether a stumble still applies, WHEN they read the `## Session record`, THEN the tree walked is a commit or merge SHA that resolves from this repository — together with which sibling projects' work that SHA contains and how it was obtained — and NOT a description such as \"current main\" or \"the docs branch\". The SHA is the post-merge assembled tree: HS-P0022 `application-author-path` and HS-P0023 `reach-and-adapter-path` are confirmed merged forward *into that tree* before the reader is seated, verified rather than assumed."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Session record`, the tree line"
  verifying_test: "Provenance (static): `git log --oneline -1 <tree-sha>` and `git log --oneline <tree-sha> -- crates/happenstance/src/lib.rs docs/README.md` resolve from this worktree and show the sibling merges present"
- id: AC-002
  criterion: "GIVEN U2 must produce a record a stranger can reproduce a stumble against, WHEN the session opens, THEN the `## Session record` already carries the session date and the reader's real run context — platform, toolchain, browser, and any assistive technology actually in use — and the reader's own non-insider declaration is present and resolved (eligible / ineligible / ineligible-but-used-anyway) *before the first task is read out*, NOT back-filled afterwards. Where the state is ineligible-but-used-anyway, the log says so as a stated failure of the artefact and the bar is NOT redefined."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Session record`, the six context fields and the declaration"
  verifying_test: "Provenance + presence (static): `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` predates the recorded session date; `rg -n \"Platform|Toolchain|Browser|Assistive|Declaration\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns a filled value for each with no not-yet-recorded token surviving"
- id: AC-003
  criterion: "GIVEN U1 is trying to get *their own* job done against the library and is never told they are grading it, WHEN they search, click, open, copy or run something, THEN the log's `## Chronological record` carries it as an `FL-###` entry written *at that moment* in `_design.md`'s declared narration mode — search strings verbatim as typed, links followed, files opened, commands run and what they emitted, reactions as they occurred — and NOT a post-session summary, a paraphrase (\"they searched for the error\"), or an opinion about whether they liked it. Where the declared mode is concurrent, the ~17–20% task-time inflation is recorded as a stated property of the run and never presented as a comparable metric."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Chronological record`, the `FL-###` entries' `What happened` fields"
  verifying_test: "Artifact-evidence (ledger-cited reviewer read) that entries are behavioural and verbatim — at least one quoted search string, at least one command with its output — with static support from `rg -n \"^### FL-\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` non-empty and the narration-mode line matching `_design.md` verbatim"
- id: AC-004
  criterion: "GIVEN U3 needs to rank what to fix without asking the logger what an entry meant, WHEN they read any stumble with all styling stripped — in `git diff`, in a plain-text pager, or through a screen reader — THEN its `Severity` field is a word or number drawn from the `## Severity scale` legend `_design.md` named, applied inline as the entry was written, and NOT a colour, an emoji, a swatch carrying the meaning alone, or a mark added in a tidy-up pass after the session."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `Severity` field on every stumble-kind `FL-###` entry, against the `## Severity scale` legend"
  verifying_test: "Plain-text legibility (static): `git show HEAD:.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` read with all styling stripped — every `Severity` value matches a legend token, none is emoji-only or colour-only, and marks are interleaved in chronological position rather than appended as a block"
- id: AC-005
  criterion: "GIVEN `route-and-escalate` will hand a sibling project a stumble id and a later reader will land on that cited anchor, WHEN the log is finalised, THEN every `FL-###` id is the one assigned in occurrence order at write time, ids are never reassigned, no gap is closed, no entry is reordered by severity in place of chronology, no heading line is re-worded, and nothing already written during the session is edited — a correction is an appended entry — NOT a tidied, renumbered or severity-sorted record. Two stumbles inside the same minute still get distinct ids."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Chronological record`, the `### FL-###` heading lines and their order"
  verifying_test: "Structure + append-only history (static): `rg -n \"^### FL-\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` yields strictly increasing unique ids; `git log -p --follow .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` shows appends only — no diff hunk rewrites a written `### FL-` heading or body"
- id: AC-006
  criterion: "GIVEN the finding rests on the reader being genuinely unrescued, WHEN the facilitator answers a question, points at a file, narrates on the reader's behalf, or unblocks the session so it can continue, THEN that moment is an `FL-###` entry of `Kind: intervention` carrying its timestamp — and where none occurred, the log states positively that none occurred, NOT leaving the reader to infer it from an absence."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `Kind: intervention` entries in `## Chronological record`, or the explicit zero-intervention assertion in `## Session record`"
  verifying_test: "Content (static): `rg -n \"Kind: intervention\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns timestamped entries, or the explicit zero-intervention assertion is present — the reviewer confirms exactly one of the two is true"
- id: AC-007
  criterion: "GIVEN U1 may be reaching the material with keyboard only or with assistive technology, WHEN they need to find something using the medium's own affordances — rustdoc keyboard search, browser find, intra-doc links — THEN the log records whether they reached it, and a reach that required a pointer is written as a stumble with a severity mark, NOT smoothed over as a session mechanics detail or omitted because it \"wasn't about the docs\"."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the keyboard-reach observation(s) in `## Chronological record`"
  verifying_test: "Artifact-evidence with static support: `rg -ni \"keyboard|browser find|rustdoc search|pointer\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns at least one recorded observation, stated in one of the two directions, with any pointer-only reach carrying a `Severity` token rather than sitting as a bare note"
- id: AC-008
  criterion: "GIVEN the small-sample basis says every problem a real reader hits is already proven worth fixing, WHEN U1 stops — completing the scenario or abandoning at a blocker — THEN the log records the end state explicitly: completed, or abandoned at a named point with the reason, as an `FL-###` entry of `Kind: abandonment`; the partial log is kept as evidence and is NOT discarded, silently re-run, or rescued by coaching the reader past the blocker to \"get more data\"."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the terminal entry of `## Chronological record`"
  verifying_test: "Content (static): the record terminates in a recorded completion of the `## Scenario` or in `rg -n \"Kind: abandonment\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, and `git log --oneline` on the log shows one session's worth of commits with no second session's entries appended"
- id: AC-009
  criterion: "GIVEN U3 must find their items and act without a tool, a rendering step or a conversation, WHEN they open the log as plain markdown, THEN all six of project AC-004's elements tick against the text alone; every stumble is present in the visible-by-default `## Chronological record` so that deleting the whole `## Dispositions index` and every fold or extract would lose nothing; and every stumble's `Disposition` field is present, on the `not yet dispositioned` arm, with its `Revisions` slot present and defaulting to `none` — NOT pre-filled with a \"will fix\", a destination, or a \"noted\", which would pre-empt `disposition-every-stumble` and start the double-disposition failure project AC-006 forbids."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the eight sections as a whole; the `Disposition` and `Revisions` fields on every `FL-###` entry; `## Dispositions index` as a derived, deletable view"
  verifying_test: "Deletion check + presence (static): in a scratch copy delete `## Dispositions index` and every fold, and `rg -c \"^### FL-\"` is unchanged; every entry carries all six labelled fields; `rg -n \"Disposition:\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns only the `not yet dispositioned` arm — zero `fixed:` / `accepted:` / `routed:` / `escalated:`"
```
