---
item: HS-S0120
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — DT-7's signal shape and the redaction question, answered against the code

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN the application author on *Choose a contract before a database* needs this project's answer to \"how is a reader told the log it is reading is incomplete\" to be a decision rather than a scheduled intention, WHEN they open `.bklg/…/retention-and-incomplete-logs/_design.md`, THEN a top-level `## DT-7 — how a reader is told the log is incomplete` section states which of (a) one undifferentiated incompleteness signal or (b) the transient / benign-permanent / meaningful-permanent distinction is adopted, names the option that lost, and gives the reason it lost — a section naming a winner without naming a loser does not satisfy this, per the brief's own AC-009 row."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md — a new top-level `## DT-7 — how a reader is told the log is incomplete` section appended after the existing `## Sign-off` block"
  verifying_test: "static content review of `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` § `DT-7 …`, read against `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:604-612` (AC-009 row) and `project.md:228-230`; diff gated by `cargo xtask affected --base main` (`.redkiln/config.yaml:40`)"

- id: AC-002
  criterion: "GIVEN the evaluator on *Decide in one sitting* must be able to see that the retention answer came from the evidence this project built and not from the charter's one-line framing, WHEN they read the DT-7 section, THEN both branches are weighed by explicit path citation against the recorded CF-27 pass list from `cf-27-experiment-and-recorded-pass-list` (what the suite can distinguish at all) and against the two reader observations from `decision-model-and-ingest-observed` (the confidently wrong `read_decision_model` fold; `IngestStore::holds` answering `false` for an event this store minted), and the losing option is refuted by name — the Ecotone production evidence if (a) wins, the reader complexity if (b) wins."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md — the DT-7 section's evidence paragraphs"
  verifying_test: "static content review resolving every citation in the DT-7 section against `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/cf-27-experiment-and-recorded-pass-list/spec.md`, `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/decision-model-and-ingest-observed/discover.md`, and `.bklg/from-contract-to-published-library/_discovery/research/07-retention-deletion-and-incomplete-log-semantics.md:98-107`"

- id: AC-003
  criterion: "GIVEN the adapter author on *Learn when you are finished* would otherwise read a retention vocabulary and go looking for the channel that reports it, WHEN they reach the point in the DT-7 section where the adopted words are introduced, THEN the section has already told them no such channel exists — the statement precedes the vocabulary rather than following it — naming all three channel-less return shapes (`read_decision_model` → `(Vec<SequencedEvent>, Option<SequencePosition>)`, `crates/happenstance-core/src/store.rs:321-331`; `Guard::is_violated_by` → `bool` from a two-arm `match`, `crates/happenstance-core/src/append.rs:239-253`; `contains_event_id` → `Result<bool, _>` whose `false` conflates \"never had it\" with \"had it and forgot\", `store.rs:268`) and stating the adopted shape is a vocabulary a future primitive could carry, never a signal a store reports today."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md — the DT-7 section, in the position before the adopted words are introduced"
  verifying_test: "static content review for section ordering (limit stated before vocabulary) and for all three citations present, each line reference opened against `crates/happenstance-core/src/store.rs` and `crates/happenstance-core/src/append.rs`; rejects the named wrong implementation `ThreeKindsOfNothing`"

- id: AC-004
  criterion: "GIVEN ES-39 rejects a floor because a regulated purge is *scattered, not a prefix* and \"ships looking correct until a claim runs long\", and specifying a reporting shape would settle ES-39 by implication from an artifact no ADR authorises, WHEN the DT-7 section describes incompleteness, THEN it uses no boundary-position framing (\"history below position N is gone\") and specifies no reporting shape; and where the honest resolution would need one, that is recorded as the AC-012 escalation it is — pointing at DA-7's four-row option table (floor / retained ranges / third condition outcome / tri-state `contains_event_id`) and at `surface-diff-and-the-ac-012-escalation` — with no change made."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md — the DT-7 section; and the absence of any diff under `crates/happenstance-core/src/`"
  verifying_test: "static content review against `spec/SPECIFICATION.md:4325-4349` (esp. `:4336-4341`) and `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:363-379` (DA-7), plus `git diff --name-only` showing no change to `crates/happenstance-core/src/store.rs` or `crates/happenstance-core/src/append.rs`"

- id: AC-005
  criterion: "GIVEN the application author must know, before choosing this contract, whether an event's queryable tags can ever be erased for E2E-49's regulated-deletion case, WHEN they read the redaction answer in `_design.md`, THEN it answers against the code, each fact cited by path and line: `Tag` is `Tag(Cow<'static, str>)` whose hand-written `PartialEq`/`Eq`/`Ord`/`Hash` all delegate to `as_str()`, so tag equality is string equality with no digest field to swap in (`crates/happenstance-core/src/tag.rs:79`, `:170-193`); `EventParts` separates opaque `data` from queryable `tags`, so shredding reaches `data` and cannot reach `tags` (`crates/happenstance-core/src/event.rs`); and `EventStore` exposes no store-side update path (`crates/happenstance-core/src/store.rs`)."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md — the redaction section appended after the existing `## Sign-off` block"
  verifying_test: "static content review of the redaction section against `spec/E2E-CASES.md:1288-1311` (E2E-49) and `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_decomposition.md:446-466` (DA-9), with every cited `file:line` in `crates/happenstance-core/src/tag.rs`, `event.rs` and `store.rs` opened and confirmed"

- id: AC-006
  criterion: "GIVEN CF-38 makes a provisional claim with an empty falsifier a gate failure and AC-010 requires the deferral to name an experiment, WHEN the redaction answer is read and the `experiments/` path it names is opened, THEN the answer is an explicit deferral stating why — redaction needs either a new store operation (a published-surface change gate decision 4 forbids) or a change to what `Tag` equality means (a silent re-keying of every tag index, invisible to `cargo-semver-checks` because no signature moves) — and the named path resolves to a charter `README.md` that exists in the tree, and ES-37's `[FROZEN]` text is unchanged."
  satisfied: false
  evidence: ""
  mount_point: "experiments/<named-experiment>/README.md — the deferral's falsifier, made real; referenced from the redaction section of `.bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md`"
  verifying_test: "`test -f experiments/<name>/README.md` (the named path resolves), static content review of the deferral against `spec/SPECIFICATION.md:8318` (CF-38) and `.bklg/from-contract-to-published-library/_decomposition.md:239-249` (gate decision 4), and `git diff` showing `spec/SPECIFICATION.md:4274-4297` (ES-37) untouched; rejects `DeferredToNothing` and `DigestTagsQuietly`"

- id: AC-007
  criterion: "GIVEN a maintainer six months from now must be able to run the falsifier rather than re-derive it — the failure `experiments/wire-format/README.md` records, where \"measured, probe W5\" was a label and not a citation — WHEN they open the charter, THEN it states the hypothesis (a digest-keyed `Tag`), what it would measure including the tag-index migration cost, the two registered rules whose observable behaviour a tag-equality change would move — `query_item_tags_are_and` (`crates/happenstance-testkit/src/suite.rs:409`) and `query_item_tags_match_supersets` (`:439`), both in `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:112-113`) — what result would settle the question either way, and in terms that it has not been run; and the directory is neither a workspace member nor a gate step."
  satisfied: false
  evidence: ""
  mount_point: "experiments/<named-experiment>/README.md — the charter, modelled on `experiments/position-visibility/README.md` and isolated per `experiments/wire-format/README.md`"
  verifying_test: "static content review of `experiments/<name>/README.md` against the two named rule sites (`crates/happenstance-testkit/src/suite.rs:409`, `:439`; `crates/happenstance-testkit/src/registry.rs:112-113`), both of which must resolve; plus a grep of `Cargo.toml`'s `members` and `xtask/src/main.rs` showing the new directory in neither"

- id: AC-008
  criterion: "GIVEN the repository owner approved `_design.md`'s no-surface determination on 2026-08-12 at the `/redkiln:plan` design gate and that approval must still mean exactly what it meant, WHEN this story's change is applied, THEN the DT-7 and redaction content is appended as new top-level sections after the existing `## Sign-off` block, carrying its own dated attribution to HS-S0120, with every line of the approved text (`:20-36`, `:86-95`) byte-identical and the CLI-owned frontmatter untouched; the added sections are composed in the artifact's own house shape — `##` sections with by-path citations, not a bare bullet dump — within the density budget (DT-7 ≤ 70 lines, redaction ≤ 50 lines, charter README ≤ 150 lines); and the diff touches nothing outside the PR boundary: no `.kb/` write, no `spec/` edit, no conformance rule, no published signature, leaving ADR-0028 to `adr-0028-and-the-open-question-wave`."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md — appended below `## Sign-off` (`:86-95`), leaving `:20-36` and the approval paragraph unchanged"
  verifying_test: "`git diff .bklg/from-contract-to-published-library/retention-and-incomplete-logs/_design.md` showing additions only below `## Sign-off`; `git diff --name-only` checked against the PR boundary's fenced glob list; line counts of the two added sections and the charter against the density budget; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green"
```
