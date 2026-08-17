---
item: HS-S0166
stage: implement
created: "2026-08-17T13:16:20.065Z"
updated: "2026-08-17T13:16:20.065Z"
---

# Acceptance ledger — Exactly one disposition per stumble, readable at the stumble

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

`mount_point` throughout is the friction log landed by `friction-log-skeleton` at
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — specifically the `Disposition:` and
`Revisions:` fields of each `### FL-###` entry in `## Chronological record`, and the derived
`## Dispositions index`, reachable in one hop from `project.md`'s `## Companions` list. Where
`_design.md` fixes a different path, it substitutes verbatim, per `spec.md`, `## Integration
contract` and EC-001. `verifying_test` values are real checks against real paths rather than
test-file ids — this project has no functions and no test binary — and per `_decomposition.md`'s
testing brief each static check asserts on **content**, never on mere structural presence: a check
that would pass `"noted"` as readily as `"routed: HS-P0022"` rejects no wrong implementation.

```yaml
- id: AC-001
  criterion: "GIVEN U3 receives the log at hand-off and must be able to trust that nothing was quietly left undecided, WHEN they read `## Chronological record` end to end, THEN every entry carrying a severity token from the `## Severity scale` legend has exactly one disposition arm with its payload — `fixed: <ref>` | `accepted: <reason>` | `routed: <id>` | `escalated: DT-<n>` — zero entries still read `not yet dispositioned`, and no entry carries two arms; and NOT a log where the \"obvious\" items were left blank, nor one where an item is both fixed *and* routed \"in case it comes back\", which is a stumble with two owners and no accountability."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `Disposition:` field of every `### FL-###` entry in `## Chronological record`"
  verifying_test: "Count + arm shape (static): `rg -c \"^### FL-\" .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` equals `rg -c \"^Disposition:\" ...`; `rg -n \"not yet dispositioned\" ...` returns nothing on a severity-marked entry; each `Disposition:` value carries exactly one arm token and no line carries two. Artifact-evidence: reviewer reconciles severity-marked count against dispositioned count, cited by file:line"
- id: AC-002
  criterion: "GIVEN U3's stated constraint is acting *without asking the logger what an entry meant*, WHEN they open any single `### FL-###` entry, THEN the arm and its payload are legible on the `Disposition:` line inside that entry's own block, so the outcome is known without leaving the entry — any outward move is by id, optional, and at most one hop; and NOT a `Disposition:` field reading \"see the index below\", nor a disposition that exists only in `## Dispositions index`."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `Disposition:` line inside each `### FL-###` block"
  verifying_test: "Self-containment (static): every `### FL-` block contains its own `Disposition:` line carrying an arm and payload; a scan of those lines for the cross-reference words see / below / above / index / as noted returns nothing. Artifact-evidence: ledger cites two entries by file:line — one `routed:`, one non-`routed:`"
- id: AC-003
  criterion: "GIVEN a reviewer six months out is deciding whether an accepted stumble should now be reopened, WHEN they read an entry dispositioned `accepted:`, THEN the payload states what is being accepted and why that is acceptable — the cost borne, and the reason bearing it is right — in terms a stranger to the session can evaluate; and NOT \"noted\", \"minor\", \"by design\", \"wontfix\", a mood, or a reason that only makes sense to someone who was in the room."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `accepted:` payloads on `### FL-###` entries in `## Chronological record`"
  verifying_test: "Artifact-evidence (reviewer-read, ledger-cited) — `_decomposition.md`'s testing brief forbids automating this into a presence check. Static support only: every `accepted:` payload is a full clause and matches none of the deny-list noted / minor / ok / n/a / by design / wontfix alone. Each `accepted:` entry cited by file:line"
- id: AC-004
  criterion: "GIVEN `content-fixes-from-dispositions` must land the fix this arm asserts without re-deriving the finding, WHEN an entry is dispositioned `fixed:`, THEN the payload names what will change and where — a real path that resolves, and the section, item or doc comment within it — and NOT a past-tense claim about an edit made in this PR: the diff contains no file under `crates/`, `docs/`, `examples/`, `spec/` or `standards/`, including the one-line doc-comment fix that would take less time than writing the disposition."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `fixed:` payloads on `### FL-###` entries; and the PR diff as a whole"
  verifying_test: "Boundary + payload typing (static): `git diff --name-only main...HEAD` intersected with `crates/ docs/ examples/ spec/ standards/` is empty; every `fixed:` payload contains a path that `test -f` or `test -d` resolves from this worktree. Artifact-evidence: reviewer confirms the payload is specific enough to act on cold"
- id: AC-005
  criterion: "GIVEN U3 opens the log to *find the items that are theirs*, WHEN they read a `routed:` or `escalated:` entry, THEN the payload is an id token — `HS-P0020` … `HS-P0023`, the `support` initiative (`.redkiln/config.yaml:5`), or a named staged deferral hand-off — respectively `DT-<n>` drawn from the ten-row ownership table; and a stumble whose fix would change a design tension a sibling already resolved takes the `escalated:` arm rather than being absorbed as `fixed:` here; and NOT \"route to the docs team\", \"the docs owner\", or any prose destination, which fails as surely as an undispositioned item."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `routed:` and `escalated:` payloads on `### FL-###` entries, and their rows in `## Dispositions index`"
  verifying_test: "Payload typing + id existence (static): every `routed:` payload matches `HS-[PI][0-9]{4}` or names `support` and resolves (`test -f .bklg/support/initiative.md`, `test -f .bklg/docs-that-teach/<sibling>/project.md`); every `escalated:` payload matches `DT-([1-9]|10)` and appears in `rg -n \"^\\| DT-\" .bklg/docs-that-teach/_decomposition.md`. Artifact-evidence: reviewer confirms no DT-shaped finding took the `fixed:` arm"
- id: AC-006
  criterion: "GIVEN an escalation or acceptance must be safe to make because it can be withdrawn without the trail vanishing, WHEN a disposition written earlier in this pass is changed, THEN the entry's `Revisions:` field gains a dated line carrying the earlier arm and payload verbatim plus the reason for the change, the `Disposition:` line shows the current arm, and the earlier one stays legible; and NOT an in-place edit that leaves no trace, nor a `Revisions:` slot retrofitted at the moment of first change — it exists from first write, defaulting to `none`, precisely so the first revision has somewhere to go."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `Revisions:` field of each `### FL-###` entry"
  verifying_test: "Append-only revisions (static, git): `git log -p --follow .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` over this story's commits shows no `Disposition:` rewrite without a same-commit `Revisions:` addition; every entry carries a `Revisions:` line (`none` where unrevised). Artifact-evidence: each revised entry cited by file:line, or a positive assertion that none was revised"
- id: AC-007
  criterion: "GIVEN \"zero undispositioned\" means nothing unless the denominator is honest, WHEN the pass sweeps the record, THEN every entry of a stumble kind carrying a legend severity token owes an arm; entries the shape exempts (`Kind: intervention`, and any entry whose `Severity:` is `n/a`) record `n/a` on `Disposition:` explicitly, so exemption is an assertion rather than an inference from a blank; and no `Severity:` value is erased, downgraded or re-scaled during this pass; and NOT a shrunken denominator produced by demoting a mark now that the fix looks hard, nor by leaving exempt entries blank so that a blank has to be read as an exemption. An abandonment entry is a finding, not a void: if the session ended there it carries a severity and therefore an arm."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the `Severity:` and `Disposition:` fields of every `### FL-###` entry"
  verifying_test: "Immutability + exemption (static): `git diff main...HEAD -- .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` shows zero changed `Severity:` lines; every `### FL-` block carries a non-empty `Disposition:` value, `n/a` exactly where `Kind: intervention` or `Severity: n/a`. Artifact-evidence: reviewer confirms each `n/a` matches an exempt kind"
- id: AC-008
  criterion: "GIVEN `route-and-escalate` will hand `FL-004` to HS-P0022 and there is no redirect, alias table or forwarding mechanism behind that id, WHEN this PR's diff on the log is read, THEN it is confined to `Disposition:` lines, `Revisions:` lines and the `## Dispositions index` block — no id renumbered, no gap closed, no heading label re-worded, no entry reordered by severity in place of chronology, no `Time`, `Kind`, `Severity` or `What happened` text changed, and no section added, renamed or reordered; and NOT a tidy-up pass that makes the record read better next to its dispositions, which breaks a citation silently and with no error message."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Chronological record`, the `### FL-###` heading lines and the frozen entry fields"
  verifying_test: "Immutability of the record (static, git): `git diff -U0 main...HEAD -- .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` shows every changed line as a `Disposition:` line, a `Revisions:` line, or inside `## Dispositions index`; `rg -n \"^### FL-\"` yields the identical id sequence before and after; `rg -n \"^## \"` yields the same eight headings in the same order"
- id: AC-009
  criterion: "GIVEN a reviewer arriving cold at `redkiln board`, WHEN they open the project card, follow one `## Companions` link and open any entry, THEN they see its disposition with no second document, no filter, no script and no rendering step: `## Dispositions index` is regenerated from the entries in `docs/README.md`'s two-column shape, states in its own first line that it is derived and that `## Chronological record` is authoritative, and deleting the whole section loses no stumble and no disposition; no `<details>`, fold, widget or colour/emoji-only carrier is introduced and any checkbox stays on one line; and NOT a disposition reachable only through the index, which is the labour-saving move that converts every future review into a context jump."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — `## Dispositions index`, reached in one hop from `.bklg/docs-that-teach/comprehension-evidence/project.md`'s `## Companions` row"
  verifying_test: "Deletion check + reachability (static): in a scratch copy delete `## Dispositions index` and every fold — the `### FL-` count and the `Disposition:` line count are unchanged; `rg -n \"<details>|<summary>|<script>\" <log>` returns nothing; the index's first line asserts derivation; `rg -n \"_friction-log\" .bklg/docs-that-teach/comprehension-evidence/project.md` returns the `## Companions` row"
```
