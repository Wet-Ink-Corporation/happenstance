---
item: HS-S0141
stage: implement
created: "2026-08-17T13:16:03.971Z"
updated: "2026-08-17T13:16:03.971Z"
---

# Acceptance ledger — One resolver for specification clause ids, next to the parser that owns them

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
  criterion: "GIVEN a contributor in the same slice writing the narrative checker as a second bin-crate module, WHEN they need to answer \"does clause id `X` exist in `spec/SPECIFICATION.md`\", THEN `pub(crate) fn clause_ids(root: &Path) -> Result<BTreeSet<String>>` is callable from that module without any change to `xtask/src/main.rs` or `xtask/src/lib.rs`, it reads the pinned `SPEC` path through the existing `read` helper, and against the real workspace root it returns a non-empty set containing the ids the document declares — AND the decision half is a private `&str`-taking sibling, so every other assertion in this story is in-memory and adds no dev-dependency."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — already mounted in the xtask bin crate by `mod spec_trace;` at xtask/src/main.rs:70; the xtask lib target declares only `mod constitution;`, so `pub(crate)` reaches the new checker module and nothing else"
  verifying_test: "xtask/src/spec_trace.rs::tests::clause_ids_reads_the_pinned_specification (cargo test -p xtask)"

- id: AC-002
  criterion: "GIVEN a maintainer who later adds a seventh clause family to `SECTIONS`, WHEN they add it in that one place and run the gate, THEN the narrative checker's resolution accepts the new family with no second edit — because `clause_ids` takes its accepted prefixes from `SECTIONS` via `clause_id` rather than from any literal prefix list, and it recognises exactly the declaration forms the parser recognises: the `####`-heading form, the `**PS-1 — …**` bold form and the `**CF-30 is [NON-NORMATIVE] …**` form, while a bare `**ES-40**` cross-reference is not a declaration."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — clause_ids/its private &str-taking sibling, beside all_rules (:1739-1768); prefixes resolved through clause_id (:1391-1430) from SECTIONS (:107-153)"
  verifying_test: "xtask/src/spec_trace.rs::tests::every_section_family_resolves and ::tests::both_declaration_forms_resolve_and_a_cross_reference_does_not (cargo test -p xtask)"

- id: AC-003
  criterion: "GIVEN an application author whose teaching page cites a `PS-` or `SY-` clause that the specification declares as `[DEFERRED]`, WHEN the gate runs, THEN the citation resolves and the page passes — the resolver answers *existence*, never *eligibility*: no maturity filter, no family filter, and `has_suite` is not applied. Ids are stable names that are never renumbered, which is what makes resolution-by-name survive the 521-line divergence on the unmerged sibling branch."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — the private &str-taking sibling of clause_ids; has_suite (:1729-1737) deliberately not on the path"
  verifying_test: "xtask/src/spec_trace.rs::tests::a_deferred_clause_without_a_suite_still_resolves (cargo test -p xtask)"

- id: AC-004
  criterion: "GIVEN a contributor whose checkout has a truncated, moved or unreadable `spec/SPECIFICATION.md`, WHEN the gate runs, THEN the run fails with a message that names the artifact it could not read or could not parse and says the checker is broken rather than the document — never `Ok` with an empty set, which would make both consumers report real pages and real pinned ids as defective. The failure is one composed sentence on the terminal surface with the artifact name first, so soft-wrap at 80 columns cannot push the location off the first visual row, and it never elides."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — clause_ids propagates read's `reading {rel}` context (:2307-2309); the &str-taking sibling carries the vacuity `bail!` in the shape of wire_rules (:1784-1791)"
  verifying_test: "xtask/src/spec_trace.rs::tests::an_unreadable_specification_names_the_file_it_could_not_read and ::tests::a_specification_that_declares_nothing_blames_the_checker (cargo test -p xtask)"

- id: AC-005
  criterion: "GIVEN a contributor who opens the accessor to decide whether they may trust it, WHEN they read its documentation, THEN the *first* thing they meet is what it does not verify — a doubly-declared id collapses and nothing checks for one; existence is not appropriateness; it parses no citations — followed by an `# Errors` section naming the two failure *conditions* rather than a type, and one sentence naming the alternative that lost (a literal prefix list; a `Vec<Clause>` return). No badge, tick or \"verified\" wording claims a resolving citation is a *correct* citation. Prose wraps at ≤ 90 source columns and the hierarchy is carried by position and heading level alone. AND the headline limit is proven rather than asserted."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — the doc comment on clause_ids, beside all_rules (:1739-1752); rendered by cargo xtask ci's docs step"
  verifying_test: "xtask/src/spec_trace.rs::tests::a_doubly_declared_id_collapses_to_one and ::tests::the_accessor_documents_its_limits_before_its_errors (cargo test -p xtask), plus cargo xtask ci's docs step"

- id: AC-006
  criterion: "GIVEN this foundation lands one commit before its two consumers, WHEN the contributor runs the gate at this story's own checkpoint, THEN `cargo xtask ci --fast` is green with a `pub(crate)` function that nothing yet calls — carried by `#[expect(dead_code, reason = …)]` whose reason names `narrative-citation-resolution` and `frozen-documentation-must-pin`, AND the marker is self-erasing: the moment the first consumer calls `clause_ids`, the expectation is unfulfilled, which is itself a warning, which `-D warnings` turns into a failure that forces the attribute's deletion. `#[allow(dead_code)]` appears nowhere in the diff, because `allow` would rot into a permanent exemption."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/spec_trace.rs — the #[expect(dead_code, reason = …)] attribute on clause_ids; gated by the clippy `-D warnings` step of cargo xtask ci --fast"
  verifying_test: "cargo xtask ci --fast (clippy step, -D warnings) at this story's checkpoint commit; reviewed against crates/happenstance-neon/src/event_store.rs:228-240"
```
