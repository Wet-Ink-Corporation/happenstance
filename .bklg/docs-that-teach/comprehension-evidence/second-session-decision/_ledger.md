---
item: HS-S0170
stage: implement
created: "2026-08-17T13:16:22.711Z"
updated: "2026-08-17T13:16:22.711Z"
---

# Acceptance ledger — Answer the second-session question explicitly

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

`mount_point` throughout is the second-session verdict slot inside `## Hand-off` in the friction log
landed by `friction-log-skeleton` at
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`; where `_design.md` or the as-built
skeleton fixed a different path or slot name, it substitutes verbatim, per `spec.md`,
`## Integration contract` and EC-007. `verifying_test` values are real checks against real paths
rather than test-file ids — this project has no functions and no test binary, and per
`_decomposition.md`'s testing brief each check asserts on content, not merely on structural presence.

```yaml
- id: AC-001
  criterion: "GIVEN U3 opens `<log>`'s `## Hand-off` to find out how much evidence they are inheriting, and has never read this story, WHEN they look for the second-session question, THEN the slot carries exactly one of `run` or `declined — <reason>`, readable in place with no hop and no fold, with the skeleton's not-yet-recorded token gone from that slot — and NOT absent, blank, deleted, `TBD`, `n/a`, `noted`, \"no second session planned\", or stated only in a story report, a commit message or a companion note. The verdict appears in exactly one place in the repository; `handoff-note-to-closeout` cites this slot rather than restating it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Hand-off`, the second-session verdict slot"
  verifying_test: "Content (static): `rg -n \"second.session\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns one arm (`run` or `declined — <reason>`); the not-yet-recorded token no longer matches inside `## Hand-off`; `rg -ni \"TBD|not mentioned|noted\"` over that block returns nothing"
- id: AC-002
  criterion: "GIVEN a reviewer six months out must be able to disagree with the verdict rather than defer to it, WHEN they read the block immediately above the verdict, THEN the assessment states all four dominance inputs as facts read off this log — (a) the count of severity-marked stumbles, (b) how many carry the blocking-end token of the scale `_design.md` named, naming the token, (c) whether the record terminates in an entry of `Kind: abandonment`, (d) whether the reader reached the stated goal of `## Scenario` — each count citing the `FL-###` ids it was computed from, so the reviewer can recompute without asking the logger anything. It is NOT a conclusion presented alone, a mood (\"the session went fine\"), an appeal to time or reader availability dressed as an assessment, or a count with no ids behind it."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Hand-off`, the assessment block above the verdict"
  verifying_test: "Content (static) + recount: the four inputs are present with values; `rg -o \"FL-[0-9]{3}\"` over the assessment block resolves against `rg -n \"^### FL-\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md`; a fresh count of severity-marked entries and blocking-end tokens matches the recorded numbers"
- id: AC-003
  criterion: "GIVEN the verdict must be a decision and not a default (`discover.md`, `## The wrong implementation`), WHEN the reviewer reads the assessment, THEN the dominance test is written out beside it in one place — one stumble at the blocking end of the named scale both accounts for the session ending (abandonment at that stumble, or the stated goal never reached) and leaves the remaining severity-marked stumbles too few to carry the narrow claim on their own — and the recorded verdict follows from applying that test to the four inputs, visibly and in the same block. It is NOT a test stated only in this spec, re-worded to fit the conclusion, or replaced by a bare \"we judged that…\"."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Hand-off`, the stated dominance test beside the assessment"
  verifying_test: "Artifact-evidence (reviewer-read, ledger-cited) with static support: `rg -ni \"dominat\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns the stated test inside `## Hand-off`; the reviewer confirms test → inputs → verdict is entailed as written; `git log -p --follow` shows the test and verdict landing together, not the test edited after the verdict"
- id: AC-004
  criterion: "GIVEN a count taken over a moving finding set is arithmetic about nothing, WHEN the assessment is computed, THEN `<log>` already carries exactly one disposition arm on every severity-marked stumble — zero entries reading `not yet dispositioned` — and the counts the assessment records match what a reviewer counts in the log as it stands at merge; NOT computed against a partially dispositioned record, and NOT repaired afterwards by editing a severity mark or a disposition to change a count."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `Disposition:` field on every `FL-###` entry, read by the `## Hand-off` assessment"
  verifying_test: "Static: `rg -n \"Disposition:\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns zero `not yet dispositioned` arms; the recorded counts equal a fresh `rg` recount; `git log -p --follow` shows this PR's diff touching no `Severity:` and no `Disposition:` line"
- id: AC-005
  criterion: "GIVEN BR-14 binds every summary of this evidence to \"real stumbles were captured and are traceable\" and never to exhaustiveness (`../project.md`, AC-008), WHEN the verdict is `declined`, THEN its reason states what the decline costs — that the evidence rests on one session and the narrow claim stands unchanged — is compatible with `## Scope of the claim` without restating it, and asserts nothing about coverage; and where a second session was owed but could not be run, that is the stated reason and the record says the evidence is correspondingly thinner. It is NOT \"one session was enough to find the problems\", \"nothing important was missed\", a decline justified by cost or scheduling alone, or a quietly relaxed bar."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Hand-off`, the `declined — <reason>` arm, read against `## Scope of the claim`"
  verifying_test: "Static (grep) + Artifact-evidence: `rg -ni \"exhaustiv|complete coverage|all the problems|nothing.{0,20}missed\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns nothing outside a disclaiming sentence; the reviewer confirms the reason names its consequence, is compatible with `## Scope of the claim`, and reads honestly quoted alone"
- id: AC-006
  criterion: "GIVEN one commit or merge point is one session's fixture and a second session opens a new fixture instance (`../_decomposition.md`, `## Testing brief`, fixtures and seams; `CLAUDE.md`, `## The rule that matters`), WHEN the verdict is `run`, THEN the verdict records the decision and names where the second session's own record will live, and no second-session entry is appended to `<log>`'s `## Chronological record`, no `FL-###` id is continued into it, and no second session is run inside this PR — NOT reopened as a continuation of the first log."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Hand-off` (the `run` arm and its named destination) against an unchanged `## Chronological record`"
  verifying_test: "Static: `rg -n \"^### FL-\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` unchanged by this PR's diff; `git diff --stat` shows `## Hand-off` only; where the arm is `run`, the ledger cites the line naming the second record's destination, and where it is `declined` the reviewer records the row as vacuously satisfied and says so"
- id: AC-007
  criterion: "GIVEN two sibling projects have already declared they will cite into this log by reference, and a re-worded heading breaks a citation with no error message, WHEN this PR is diffed, THEN the change is confined to the second-session slot inside `## Hand-off`: no section added, renamed or reordered, no `FL-###` heading or body altered, no id renumbered; the verdict is composed from the repository's existing `## Shape decision` primitive (the decision with the alternative that lost) rather than a hand-rolled block; it is complete as static text — no colour, emoji, fold, `<details>`, widget or rendering step, and any checkbox on one line; and a verdict revised after first being written appends beside the original with the earlier verdict and the reason still legible. It is NOT an in-place overwrite, a new heading vocabulary, a bespoke verdict widget, or a tidy-up of the rest of the log."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the whole file as a diff surface; the `## Shape decision`-shaped verdict block inside `## Hand-off`"
  verifying_test: "Static: `rg -n \"^## \" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns the same eight headings in the same order; `git diff -- <log>` shows hunks only inside `## Hand-off`; `rg -n \"^### FL-\"` unchanged; `git show HEAD:<log>` read with all styling stripped carries the whole verdict; `rg -n \"<details>|<summary>\"` returns nothing; any `- [ ]` box is on one line; `git log -p --follow` shows no in-place rewrite of a written verdict"
```
