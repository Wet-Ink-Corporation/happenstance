Finish phase 15 — "Reconcile the record" — unattended. I am AFK for the whole session and will not answer questions. Work the queue below to the end, and stop only when it is done or everything that is left is blocked.

## Start

1. Read `runbook/handover.md`, the status table in `runbook/README.md`, `runbook/phases/15-reconcile.md`, and `git log --oneline -10`. Where the handover and `git log` disagree, `git log` wins; say so in the session log.
2. Run `git status`. Three things in the working tree are **not yours to commit**:
   - `.redkiln/telemetry/…jsonl`: staged by an old hook. Leave it staged and untouched.
   - `assets/brand/happenstance-mark.png`: untracked.
   - `.kb/_intake/decisions/*.md`: Weigh-In decision atoms. These are committed by work item 4 and by nothing else.
   - `runbook/phase-15-afk-prompt.md`: this prompt. The owner decides whether it is committed.

   Always commit with explicit pathspecs (`git commit -- <paths>`), never `-a` or `git add .`.
3. If PR #17 (`lane/phase-15-ingest-settled`) is still open, finish it first under the merge policy below.

## Already decided by the owner. Do not re-ask these.

Recorded in the Weigh-In ledger on 2026-09-28:
- **`wi-052920`: `[Unreleased]` rides `0.4.0`.** There is no `0.3.3`. Never tag, publish or bump a version.
- **`wi-38373d`: `.kb` gets an xtask lint.** It diffs every `status: accepted` atom body against the merge base, and it runs in the gate.
- **`wi-ab0a5a`: self-merge on green.** One PR per work item, squash-merged once every check passes.
  - If a check fails in code the PR did not touch, re-run only the failed jobs, once (`gh run rerun <id> --failed`).
  - A second failure, or any failure in code the PR did touch, stops that item. Leave the PR open, record why, and move on to the next independent item.
  - Wait on checks with `gh pr checks <n> --watch` in a background command. Do not poll by hand.

## Work queue: one branch and one PR per item, in this order

Each PR follows the session protocol in `runbook/README.md`. The phase file's box, its session-log line, a `log.md` entry and the rewritten handover all go in the same commit as the work. The first commit on each new branch also fills in the merge commit the previous PR's `log.md` entry left as "recorded here when it merges".

1. **`happenstance-sqlite` says *host only*.** Change the crate root and the README. It does not build for `wasm32-unknown-unknown`: `rusqlite` fails at `libsqlite3_sys`, and reads hop through `spawn_blocking`. Nothing in the gate checks it. See `references/seeds/sqlite-on-wasm.md`. About 36 citations point into these two files by line, so keep line counts or repoint by anchor.
2. **The records read as records.** `HANDOVER.md`, `REMEDIATION-HANDOVER.md` and `SESSION-DECISIONS-0.2.0.md` must not read as current status. They are cited by line, so change only what `RUNBOOK.md`'s frozen banner changed: the title or banner, edited in place, with the line count preserved. Leave the body verbatim.
3. **The `.kb` lint (`wi-38373d`).**
   - An xtask check fails when the body of an accepted decision atom differs from the merge base. Adding `superseded_by` to the frontmatter is legitimate, so a frontmatter-only change must pass.
   - Following the repository's rule, first write the wrong implementation it rejects into its tests.
   - Wire it into `cargo xtask ci`. Update `CLAUDE.md`'s "Nothing validates `.kb` frontmatter" paragraph and the handover's matching trap.
   - **Known snag:** the `gate` job in `.github/workflows/ci.yml` checks out at `fetch-depth: 1`, so the merge base is not in CI's clone. Choose how the lint gets its base (widen the fetch, fetch the base ref, or another option you can defend) through the decision protocol. A lint that silently skips when it has no base is the failure this repository names everywhere. It must either run or fail loudly.
   - Whether it also validates frontmatter *shape* is a two-way call. Decide it the same way.
4. **The KB intake wave, by hand.** Redkiln is retired, so do **not** use `kb-ingest` or any `redkiln` command. Do everything phase 15's intake item lists:
   - Close the open questions that are already answered. Verify `global-versus-per-boundary-visibility-invariant` before closing it.
   - Index the orphan `cf-17-cf-14-maturity-markers-and-the-reopen-must`.
   - Remove `.kb/_intake/remediation-2026-09-04-briefs/README.md`.
   - Write every staged atom in `.kb/_intake/decisions/` into `.kb/`: three older ones and three from 2026-09-28. Copy the frontmatter shape of a neighbouring atom of the same kind, add each to its map in `.kb/maps/`, and remove the intake file.
   - Never edit an accepted decision's body.
5. **Open questions whose own deadline has passed** ("at `0.2.0`", "at phase 12", "at the `ProjectionStore` freeze") each get a new owner in phase 16 or 17. Record each owner in that phase's file, and in `runbook/ledgers.md`'s *Open decisions* table where it belongs there.
6. **`[Unreleased]`.** Tick phase 15's item, citing `wi-052920`. Add a line to `runbook/phases/17-breaking-window.md` saying those entries ship in `0.4.0`. Nothing else.
7. **Found in passing.** Correct the doc comment on `no_accepted_semver_break_outlives_its_reason` (`xtask/src/lints.rs:943`), which still says the registry-baseline step "is `if: false`". Add a work item to `runbook/phases/13-sync.md` to delete `happenstance-sync`'s placeholder `EventId`, `StoreId` and `RecordedAt` from `identity.rs`. The real ones are in `happenstance-core`.
8. **Exit.**
   - Run `cargo xtask ci`. The tests step uses a lot of memory, so if the whole gate cannot finish, run it in slices with `cargo run -p xtask -- <step>` and name each slice you ran.
   - Tick every exit criterion that is actually true. If all are ticked, set phase 15 to `done` in the status table. If any is not, leave it `in progress` and say which criteria are open.
   - The handover's next action is then phase 16.

Items 1, 2, 6 and 7 are small, and you may put them on one branch if they touch no common file. Items 3, 4 and 5 each get their own.

## Decision protocol: Weigh-In

Every assumption, deferral or call you make gets a `weigh-in` block in your message, as the SessionStart hook describes, and goes into the ledger. The CLI is:

```
W="C:/Users/ryanm/.claude-personal/plugins/cache/weigh-in/weigh/0.2.0/libexec/weighin"
sh "$W" --dir "D:/repos/happenstance" batch <<'EOF'
[ ...ops... ]
EOF
```

`/weigh:in` cannot be invoked by the model and must not be. Apply its method yourself:
- Filter first. If the item is already settled by an accepted `.kb` atom or by the list above, dismiss it with `already-decided:<ref>`. If code, tests, docs or git answer it, answer it and dismiss it as `recoverable`.
- Then run the evasion check on your recommendation. "Revisit later", "make it configurable" and "both" are not answers.

**Two-way door, judgment-bound: take your recommendation and record it.** One batch per call:
- `raise` with `"authority":"judgment"`;
- `triage` with `"route":"default"`;
- `default` with `"option"`, `"cause":"two-way"` and a `"reason"` that states the trade-off and the fact that would flip it.

Then act on it. Name the item's id in the commit message and in the phase 15 session log. The owner reviews every default with `/weigh:in digest` on return.

**One-way door, or authority-bound: do not take it.** Authority-bound means a published API or semver surface, a `[FROZEN]` clause, an accepted ADR body, publishing or tagging, deleting a remote branch, `CLAUDE.md`'s binding constraints, or product intent.
- `raise` it with `"door":"one-way"` and `"did":"blocked"`.
- Take no one-way path.
- Carry on with independent work, and list it in the final report.
- Never write a new ADR as a side effect. If an item needs one, record the gap in the phase file and let phase 16's ADR pass own it.

## Guardrails

- **Line-cited files:** `RUNBOOK.md`, `HANDOVER.md`, `spec/SPECIFICATION.md`, the crate sources the spec cites, and `CONTRIBUTING.md` via the constitution. Edit in place with the line count preserved, or repoint every citation by its anchor text, never by an offset. `cargo run -p xtask -- lints`, `cargo xtask lint-constitution` and `cargo xtask spec-trace` catch misses, so run all three before every commit.
- **Line endings:** the working tree is CRLF and the index is LF. A scripted edit must match `\r\n`, and it must not write mixed endings.
- **Scoped gates:** use `cargo xtask affected --base main` for each PR. It is not a substitute for `cargo xtask ci` at the exit.
- **Killed gate runs:** a killed `cargo hack --no-dev-deps` run strips dev-dependencies from every manifest. Check `git status` before believing any result.
- **SQLite flake:** the SQLite concurrency rule flakes on Windows. Do not isolate the test to chase it. Two known CI flakes behave the same way: `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim`, and Neon's `query_items_share_one_snapshot`. They get the one re-run the merge policy allows.
- **Forbidden actions:** no `redkiln` commands or skills. No `--no-verify`. No force-push to `main`. No deleting branches.
- **Counts:** a count in a document is spelled with its members or computed by a tool.

## Final report, printed at the end

- Each PR, with its merge commit on `main`, or the reason it is still open.
- Every Weigh-In default you took, as id, question and choice, in one line each.
- Every blocked one-way or authority item, with what it is waiting on.
- Which phase 15 exit criteria are ticked, and the gate slices you actually ran.
- Anything you assumed and did not verify. Say it in those words.
