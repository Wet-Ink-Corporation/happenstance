---
item: HS-S0156
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — rustc's own E0034 output at the site where it fires

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, so a row is not flipped on the wrong proof:

- **AC-007's `verifying_test` is green in *check* mode and is not sufficient on its own.** The
  second half — every re-anchored `store.rs:NNN` citation in `spec/SPECIFICATION.md` still naming
  the same subject — is a human review of `git diff spec/SPECIFICATION.md`, because a tool that
  re-anchors to the wrong subject exits 0. Cite both.
- **AC-005 and AC-006 are jointly unsatisfiable if the pointer is not installed** (EC-002: no rung
  of the href ladder above *nothing* is available). In that state the rows stay `false` and the
  story reports it — filing a P3 row with an empty guard to make AC-006 green is anti-pattern 18 and
  is the failure this ledger exists to catch.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author (Persona 2) at journey state B0, whose build just failed with error[E0034] and whose first move is a *search* — Ctrl-F in the buffer they already have open, or the message pasted into a search box — WHEN they search for any string rustc actually emitted, including a `= note:` candidate line or TraitVariantBlanketType, THEN crates/happenstance-core/src/store.rs is a hit, because the `text` fence in `## Import one flavour, not both` carries rustc 1.97.1's own stderr for a deliberate double-import verbatim — both `= note:` lines and TraitVariantBlanketType included — as contiguous, selectable plain text with no line numbers, gutter decoration or rendered-only markup inside the fence."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs — the module `//!` doc comment, section `## Import one flavour, not both`; rendered at target/doc/happenstance_core/store/index.html via crates/happenstance-core/src/lib.rs:99"
  verifying_test: "rg -n \"= note:\" crates/happenstance-core/src/store.rs (2 hits) and rg -n TraitVariantBlanketType crates/happenstance-core/src/store.rs (>=1 hit) — both return nothing on main; plus the reproduction run's stderr and rustc --version recorded in .bklg/docs-that-teach/reach-and-adapter-path/store-error-site-rewrite/"

- id: AC-002
  criterion: "GIVEN the same reader, *reading* rather than searching — or reading with a screen reader, which linearises `^^^^ multiple` into meaningless characters — WHEN they reach the end of the transcript, THEN the sentences immediately following it state in words which call is ambiguous: the method (`read`), both EventStore and SendEventStore by name, and that both being in scope is the cause — so the diagnosis survives with colour, syntax highlighting and spatial position all removed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs — element (c), immediately after the `text` fence and before the in-place fix; rendered at target/doc/happenstance_core/store/index.html"
  verifying_test: "Read crates/happenstance-core/src/store.rs element (c) with the fence deleted — the remaining prose still names `read`, both trait names and the cause; rg -n \"EventStore\" crates/happenstance-core/src/store.rs confirms both names appear in the section's prose, not only inside the fence (falsifies _design.md anti-pattern 15)"

- id: AC-003
  criterion: "GIVEN a reader at B1 who wants only to get compiling again and will never follow a link, WHEN they read the section top to bottom, THEN they reach the import rule and the fully-qualified escape hatch (SendEventStore::read(&store, &query, options)) before any pointer, with nothing inserted between the plain-words diagnosis (c) and the fix (d) — and deleting the reasoning account entirely leaves store.rs still telling them how to resolve E0034."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs — elements (c) then (d) then (f) in the binding order of _design.md `## Composition`; rendered at target/doc/happenstance_core/store/index.html"
  verifying_test: "Source order in crates/happenstance-core/src/store.rs: the line carrying `SendEventStore::read(&store,` precedes the line carrying the pointer's link text; plus the executed falsification — with the destination page removed from the working tree, the section still resolves E0034 (UX invariant 1, anti-pattern 12)"

- id: AC-004
  criterion: "GIVEN a reader who has just been handed a compiler transcript inside a documentation comment and is deciding how far to trust it, WHEN they read on, THEN the page tells them the narrow truth — the fence is `text` and nothing compiles it; the error *code* is asserted by a compiled rust,compile_fail,E0034 example in the constitution; the wording of the notes and the internal name TraitVariantBlanketType are rustc 1.97.1's rendering and are asserted by nothing — and never the broad falsehood \"nothing checks this\"."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs — element (e), after the in-place fix and before the pointer; rendered at target/doc/happenstance_core/store/index.html"
  verifying_test: "rg -n \"compile_fail,E0034\" standards/rust/20-two-flavour-ports.md standards/rust/00-prime-directives.md (2 hits, proving the cited guard is real) and cargo test --locked -p xtask --doc with RUSTDOCFLAGS=-D warnings (xtask/src/constitution.rs — proving it is compiled)"

- id: AC-005
  criterion: "GIVEN a reader at B2 who is compiling again but still cannot tell \"the port is wrong for me\" from \"I have not understood the port yet\" — Persona 2's stated goal verbatim — WHEN they finish the section, THEN its last sentence offers exactly one hop to the adapter reasoning account: recessive, carrying no heading of its own, with self-describing link text of >= 3 words naming the destination and the need, in the highest available rung of the href ladder, never an intra-doc link and never a bare URL — and it resolves in all three gate rustdoc builds."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs — element (f), the last sentence of `## Import one flavour, not both`; built by the three rustdoc passes in xtask/src/main.rs, of which `documentation (no default features)` (xtask/src/main.rs:502) constrains its form"
  verifying_test: "cargo xtask ci --fast green — the `documentation (no default features)` step under rustdoc::broken_intra_doc_links = \"deny\" (Cargo.toml:134); plus exactly one pointer in the section and link text carrying no \"here\"/\"this\"/\"docs\"/bare https:// (EC-002 governs the not-installed case)"

- id: AC-006
  criterion: "GIVEN a maintainer six months on who must find every pointer this project installed without reading every file, WHEN they read the pointer register landed by pointer-policy-and-inventory, THEN row P3 is present, naming the surface (crates/happenstance-core/src/store.rs module doc), the destination, the ladder rung actually taken, and a non-empty guard stating what fails the build if the destination moves."
  satisfied: false
  evidence: ""
  mount_point: "the pointer register landed by the slice-mate story pointer-policy-and-inventory (path bound there; build-time data, never a rendered page per _design.md `## Transience policy`) — row P3"
  verifying_test: "The register file contains a P3 entry whose guard cell is non-empty (anti-pattern 18), and cargo xtask lints && cargo xtask spec-trace — .redkiln/config.yaml's reachability_static — is green over the tree that carries it"

- id: AC-007
  criterion: "GIVEN the repository owner, who must know a doc-comment rewrite inside happenstance-core did not silently un-discharge a frozen documentation MUST, WHEN they read this story's folder after the merge, THEN three artefacts are recorded — the HS-P0020 clause-id set read before the first edit, the toolchain the transcript was reproduced on, and the cargo xtask spec-trace result — and spec-trace is green in check mode over the resulting tree, with every displaced store.rs:NNN citation in spec/SPECIFICATION.md re-anchored to the same subject."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md — the line-anchored store.rs citations the insertion displaces (store.rs:93-268 at line 371, plus :101, :119-123, :131-139, :213-217, :321-331 and others), repaired through xtask/src/main.rs:671-673; and .bklg/docs-that-teach/reach-and-adapter-path/store-error-site-rewrite/ for the three artefacts"
  verifying_test: "cargo xtask spec-trace exits 0 in check mode, AND a line-by-line human review of git diff spec/SPECIFICATION.md confirming each re-anchored citation's named subject is unchanged; plus the repair-vs-gap verdict of .kb/playbooks/repairing-a-frozen-clause-without-amending-it.md written down (a verdict of gap fires EC-009 and this row stays false)"

- id: AC-008
  criterion: "GIVEN a reviewer holding _design.md who cannot read Rust, WHEN they open the rendered happenstance_core::store page at 1440x900 and at 1024x768 and read the diff beside it, THEN the surface obeys the signed-off design: exactly four heading entries and no fifth, no `##` whose entire body is one sentence, the section <= 36 source lines against a measured ~34, every added element persistent chrome with no fold, tab, accordion or <details>, no raw HTML and no inline style=, no \"See also\" / \"Next steps\" / \"Further reading\" block, no skipped heading level, and nothing clipped rather than reflowed — the error[E0034] fence's horizontal scrollbar excepted and expected at both viewports."
  satisfied: false
  evidence: ""
  mount_point: "the rendered surface store-module-error-site (_design.md `## Surfaces`): target/doc/happenstance_core/store/index.html, selector `#main-content > .docblock` — confirmed for this crate after cargo doc -p happenstance-core (EC-008 if it does not match)"
  verifying_test: "cargo doc -p happenstance-core --no-deps then the built page read at both widths, confirming the module doc is not rendered inside a collapsed details.toggle.top-doc body; section source-line count taken from the diff; _design.md anti-patterns 5, 9, 10, 13 and 16 each checked as a screenshot/diff question"

- id: AC-009
  criterion: "GIVEN a reader whose only handle is the string rustc printed and who types it into rustdoc's search box rather than into the file, WHEN they search E0034 or TraitVariantBlanketType on happenstance_core's documentation, THEN the search index carries them to the two traits — three #[doc(alias)] attributes added under a stated rule (the string is one rustc, the specification, or a recorded reader question actually emits, and is not the item's own name or a substring of it), establishing that convention in a workspace that has zero occurrences of the attribute today, and recording the rejected alternatives."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/store.rs:93-94 — the attributes on `pub trait EventStore` and on the trait_variant-derived SendEventStore; surfaced through rustdoc's search index on the page mounted by crates/happenstance-core/src/lib.rs:99"
  verifying_test: "rg -n 'doc\\(alias' crates/happenstance-core/src/store.rs returns exactly 3 and rg -n 'doc\\(alias' crates/ standards/ xtask/ returns only those 3; cargo xtask ci --fast green (all three rustdoc builds and the four wasm32 steps); rustdoc search exercised by hand on the built page; the rule and its rejected alternatives written in the same change"
```
