---
item: HS-S0164
stage: implement
created: "2026-08-17T13:16:19.002Z"
updated: "2026-08-17T13:16:19.002Z"
---

# Acceptance ledger — Recruit a reader who is verifiably outside, and record the declaration

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes for whoever flips these rows.

**The mount point is the same for all seven, and that is the point.** Every criterion is reachable
through the **logger identity / declaration section of the single friction log**,
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, that `friction-log-skeleton`
landed. A row whose evidence cites a second log file, a copy kept in this story's folder, or a
summary that restates the declaration is **not** satisfied — it is the failure IQ-2 and IQ-3 exist
to reject (`../_decomposition.md`, lines 188–200). **Resolve the real filename from `_design.md`'s
protocol section before citing it**; if it differs, correct the spec's `## Integration contract` and
`## PR boundary` deliberately (spec `EC-005`) rather than editing this column quietly.

**There is no automated test suite here, and that is not a gap in rigor.** This project's proof
artifact is a dated markdown record, so `verifying_test` names the **real command or reviewer
instrument** that can fail — a `git` provenance comparison, an `rg` content assertion that rejects a
blank or blanket declaration, or the AC-003 test performed literally by someone who did not write
the declaration. `evidence` is a `file:line` into the log itself (`../_decomposition.md`,
`## Testing brief`, opening paragraph). A structural-presence check alone satisfies none of these
rows: it would pass an empty declaration, which is exactly the decorative rule `CLAUDE.md` forbids.

**Two rows can only be satisfied by a *negative* result on `_design.md`.** AC-001 requires
`git diff --stat <base>...HEAD -- .bklg/docs-that-teach/comprehension-evidence/_design.md` to be
**empty**, and AC-003 requires that file's commit timestamp to strictly precede the declaration
date. If either has to be argued rather than shown, the provenance chain is broken and the answer is
spec `EC-007` — escalate — never a back-dated date or a tidied diff.

```yaml
- id: AC-001
  criterion: "GIVEN the disqualifying criteria are already frozen in `_design.md` at a commit that predates this story, WHEN U2 the facilitator screens a real named candidate, THEN every criterion is applied exactly as written — none added, dropped, reworded or softened while the candidate is in hand — and `_design.md` is unchanged by this PR, so a reviewer can see the criteria could not have been fitted to the person who answered them."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the logger identity / declaration section landed by `friction-log-skeleton`, filename authoritative per `_design.md`'s protocol section"
  verifying_test: "Static (provenance + immutability), two checks that must both hold: `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` strictly earlier than the declaration date in the log, AND `git diff --stat <base>...HEAD -- .bklg/docs-that-teach/comprehension-evidence/_design.md` empty; plus `redkiln verify --grain story`, whose PR-boundary check fails any edit to `_design.md`"
- id: AC-002
  criterion: "GIVEN U3 opens the friction log six months later with nobody left to ask, WHEN they read the declaration section, THEN they find the reader's own first-person answer to each written criterion — one answer per criterion, tied to that criterion's own label in `_design.md` rather than restating its text — and can resolve eligibility from that section plus `_design.md` alone. A facilitator's third-person paraphrase, a single blanket assertion, or one answer covering the criteria as an undifferentiated set each fail this criterion."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the logger identity / declaration section landed by `friction-log-skeleton`, filename authoritative per `_design.md`'s protocol section"
  verifying_test: "Static (content-asserting): `rg -n` over the declaration section returns exactly one answer line per criterion label present in `_design.md`'s protocol section — a count mismatch fails, which is what rejects the blanket-assertion shape; plus the reviewer-read AC-003 test performed literally, by someone who did not write the declaration, without contacting the reader or the facilitator"
- id: AC-003
  criterion: "GIVEN the project's AC-002 fixes a provenance chain — criteria commit → declaration → session — WHEN U2 records the declaration, THEN it carries the date it was given (not the date it was transcribed), that date is strictly later than the `_design.md` criteria commit and strictly earlier than any session date the log later records, and no date is back-filled or inferred."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the logger identity / declaration section landed by `friction-log-skeleton`, filename authoritative per `_design.md`'s protocol section"
  verifying_test: "Static (provenance) date comparison run from this worktree: `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` < the declaration date recorded in the log. The third term (declaration date < session date) does not exist until `session-run-against-pinned-tree` and is asserted here as a bound that story must not violate — do not mark this row satisfied on a two-term comparison without saying so in the evidence"
- id: AC-004
  criterion: "GIVEN a stumble is only reproducible against the context that produced it, WHEN the declaration is captured, THEN U1's environment of record — platform, toolchain, browser, assistive technology — is recorded at declaration time, all four present, with `none` stated explicitly where one does not apply rather than the field being silently omitted. Reconstructed-after-the-session values fail: the same objection the UX brief raises to retrospective narration applies to context."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the logger identity / declaration section landed by `friction-log-skeleton`, filename authoritative per `_design.md`'s protocol section"
  verifying_test: "Static (presence, four terms): `rg -n` over the declaration section returns all four environment field labels the skeleton declares, each with a value — an omitted field and an explicit `none` are distinguishable, and only the second passes; plus ledger `file:line` evidence citing the block, checked against `../_decomposition.md` UX-AC-004 (lines 250–254)"
- id: AC-005
  criterion: "GIVEN eligibility is three-valued by design, WHEN U2 records the verdict, THEN the log carries exactly one of `eligible` / `ineligible` / `ineligible-but-used-anyway` as a plain-text token legible with all styling stripped and in a `git diff`; the third arm additionally states the artefact is unmet, names which criterion was breached, and does not redefine that criterion to fit; and a verdict changed after first being written is recorded as a revision beside the original, with the earlier verdict and the reason still legible, never an in-place overwrite. Absence of an objection is not a verdict."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the logger identity / declaration section landed by `friction-log-skeleton`, filename authoritative per `_design.md`'s protocol section"
  verifying_test: "Static (token presence): `rg -n` over the declaration section returns exactly one of the three tokens — zero and two both fail, and a token carried by colour, emoji or bolding alone fails the plain-text read (`../_decomposition.md`, accessibility floor, lines 144–160). Revision rule checked by `git log -p` over the log showing no verdict line replaced without its predecessor surviving in the text, per `.kb/governance/rewrite-the-referent-never-the-reasoning.md` and IQ-4"
- id: AC-006
  criterion: "GIVEN U1 must remain resolvable to a stranger and must not be coached, WHEN identity and prior exposure are recorded, THEN the declaration names a referent a reviewer can resolve without asking the facilitator — a name, or a stable pseudonym plus who can vouch for it — and any prior exposure the candidate volunteers is recorded as given, not negotiated toward the answer that keeps them eligible. A declaration produced by walking the candidate to the eligible answer is agreement, not evidence."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the logger identity / declaration section landed by `friction-log-skeleton`, filename authoritative per `_design.md`'s protocol section"
  verifying_test: "Reviewer-read against the UX brief's own intent test — a stranger six months later resolves the referent without asking anyone a question (`../_decomposition.md`, lines 20–23): a bare first name with no vouching route fails, and so does an identity resolvable only through the facilitator. Non-interference is checked by the declaration stating how the criteria were put to the candidate, the way IQ-5 (lines 208–213) requires interventions to be recorded rather than assumed absent"
- id: AC-007
  criterion: "GIVEN `friction-log-skeleton` has already landed the single friction log, WHEN the declaration is mounted, THEN it appears inside that log's existing declaration section, composed into the skeleton's own field vocabulary and the repository's document primitives — never a pasted raw transcript blob, a bespoke format, or a second log file — with no new or renamed heading, no id renumbered, no section reordered, nothing added only to a summary, a fold or this story's folder, and the whole section reachable and readable as plain text by browser find with no widget, script or rendering tool. Any checkbox occupies exactly one line."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/comprehension-evidence/_friction-log.md — the logger identity / declaration section landed by `friction-log-skeleton`, filename authoritative per `_design.md`'s protocol section"
  verifying_test: "Static (structure stability), three checks: `git diff -U0 <base>...HEAD -- <log path>` shows no removed heading lines and no heading lines added outside the declaration section; `rg -n \"<details|<summary\"` over the log returns nothing; and the UX-AC-006 deletion test (lines 258–260) — delete every roll-up and fold and the declaration is still complete. One-line-checkbox conformance per `.redkiln/templates/gates/` and `CLAUDE.md`. Repository gates for the same diff: `cargo xtask affected --base main`, `cargo xtask ci --fast`, `redkiln validate --kb && redkiln doctor`"
```
