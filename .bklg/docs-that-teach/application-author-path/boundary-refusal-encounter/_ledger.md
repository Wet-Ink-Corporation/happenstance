---
item: "HS-S0185"
stage: implement
created: "2026-08-17T13:16:33.036Z"
updated: "2026-08-17T13:16:33.036Z"
---

# Acceptance ledger — The opening encounter reaches a refused append

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**Four notes specific to this story.** First, `mount_point` is `xtask/src/narrative.rs` on every row
whose surface is `docs/first-encounter.md`, because registration as a `#[cfg(doctest)] mod` there
**is** the mount — a page the harness does not name is *unregistered*, the documentation equivalent
of built-but-unmounted (`.bklg/docs-that-teach/checked-documentation-surface/_design.md:561-562`).
The co-mount `crates/happenstance/src/lib.rs` carries the rows whose surface is the crate root; it
mounts itself, because its doctest is already swept by the `"tests"` REQUIRED step
(`xtask/src/main.rs:143-155`). Second, several rows' `verifying_test` is a **tier**, not a Rust
`#[test]` id: the five tiers are named in
`.bklg/docs-that-teach/application-author-path/_decomposition.md:462-476`, and tier 5 evidence is a
recorded reviewer walk against the signed-off `_design.md`, because `design.capture` is deliberately
absent from `.redkiln/config.yaml` and no perceptual gate exists. Third, evidence for AC-007 must be
**measured on the render** (`target/doc/happenstance/index.html` and the rendered step page), never
counted in the markdown source — hidden lines are removed before the reader sees anything, so source
and rendered counts are different numbers. Fourth, if `xtask/src/narrative.rs` does not exist when
this story is implemented, EC-001 applies: halt and route to HS-P0020, land the crate-root rows, and
leave AC-001/AC-003/AC-006 unsatisfied rather than satisfying them against a parallel tree.

```yaml
- id: AC-001
  criterion: "GIVEN Persona 1 opens `docs/first-encounter.md` at step one wanting the thing the library is *for* rather than a tour of its types, WHEN they read the three steps in order and run step 3's program unchanged, THEN the program **they ran** prints a line of its own output containing the token `ConditionViolated` — reached from inside a matched `Err(AppendError::ConditionViolated(_))` arm, so the line is unreachable unless the store really refused — and the page's output block directly beneath the fence shows exactly that line. The refusal is therefore something the reader *watched happen*, not something a sentence told them: delete every sentence around the fence and the demonstration survives (IQ-7)."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: "tier 3 — `cargo test --locked --workspace --all-features -- --show-output` (`xtask/src/main.rs:143-155`) running step 3's fence through its `#[cfg(doctest)] mod` in `xtask/src/narrative.rs`; the captured stdout line containing `ConditionViolated` and the passing `assert!(matches!(refused, Err(AppendError::ConditionViolated(_))))`; plus the tier-5 confirmation that `docs/first-encounter.md`'s output block is byte-equal to that stdout"

- id: AC-002
  criterion: "GIVEN Persona 1 who has just run `cargo add happenstance` and lands on docs.rs — the page the measured defect is about — WHEN they read `/happenstance/index.html` top to bottom, THEN the first code they meet is `## Watch a boundary refuse`'s fence; it is an executing `#[tokio::main]` program rather than the never-awaited `# async fn example()` wrapper at `crates/happenstance/src/lib.rs:58-67`; its printed refusal sits directly beneath it; the planned-surface roadmap has moved **below** it and the adapter-author redirect is still last; ADR-0006's reasoning at `:20-25` survives verbatim; and the rendered page contains **zero** literal `[happenstance_core]` bracket pairs where four stood before."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "tier 3 — the crate-root doctest executed by the `\"tests\"` REQUIRED step (`xtask/src/main.rs:143-155`); tier 1 — `cargo doc --locked --workspace --all-features --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings` (`xtask/src/main.rs:290-301`); then a render read of `target/doc/happenstance/index.html` recording zero `[<code>` occurrences and the region order inside `details.top-doc > div.docblock` against `.bklg/docs-that-teach/application-author-path/_design.md:415-440`"

- id: AC-003
  criterion: "GIVEN Persona 1 arriving **cold** on `#step-three` from a search result — the arrival DT-4's chosen shape is documented to fail at — WHEN they open that anchor and read only what is on screen, THEN the first element under the heading is the two-line step header block naming what step 2 established and linking to step one in one hop; the fence below it is a *complete program that compiles and runs on its own* rather than a fragment of its predecessors; and the same holds opening `#step-two` cold. Step one carries the answered-need line in line 1's place and omits line 2. The reader can name what they missed and reach it in one click at 1440×900 and at ≤700px, where rustdoc removes the sidebar TOC entirely."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: "tiers 2 and 3 — each step's fence compiled and executed as its own unit through `xtask/src/narrative.rs`, so a fence depending on a previous step's bindings fails to build; plus the tier-5 cold-arrival walk on `#step-two` and `#step-three` at 1440×900, 1024×768 and ≤700px against `.bklg/docs-that-teach/application-author-path/_design.md:142-159`, `:453-459` and anti-pattern 9 (`:868-870`), recorded as a reviewer observation"

- id: AC-004
  criterion: "GIVEN Persona 1 whose stated fear is a boundary that looks right and is quietly empty, WHEN they read step 3's fence and then remove **either** the tag join from the guard's `Query` **or** the `after_opt(upto)` from the `AppendCondition`, THEN the scenario stops refusing — both are load-bearing — and nowhere on either surface does prose claim a real consistency boundary over code constructing an empty one. Concretely: the guard's query is tagged to the invariant being held **and the racing append carries the same tags**, because `QueryItem::matches` is `type_ok && tags.contains_all(&self.tags)` (`crates/happenstance-core/src/query.rs:112-115`) — an untagged event does not match a tagged query item, and a scenario built that way would refuse nothing while appearing to."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: "tier 3 — the same executed assertion as AC-001, which yields `Ok` rather than `ConditionViolated` and fails outright if the guard is not load-bearing; plus the tier-5 prose half (`.bklg/docs-that-teach/application-author-path/_decomposition.md:488-490`): a reviewer reads every sentence adjacent to a fence on both surfaces and records that no boundary claim sits over `Tags::empty()` or an omitted `after`"

- id: AC-005
  criterion: "GIVEN Persona 1 reading the **rendered** page rather than its markdown source — the only artefact they will ever see — WHEN they attempt to rebuild the boundary from what is visible on screen, THEN they can: no `#`-prefixed line on either surface carries a `Query`, `QueryItem`, `Tags`, `Guard`, `AppendCondition`, the append call, the read call, or any assertion. Hiding is used only for a `use` block already shown visibly earlier on the same page, `Ok(())`, and struct-literal filler. This is binary rather than a fold — `#`-prefixed lines are absent from the rendered DOM entirely, with no hover, focus or toggle that recovers them (`_design.md:524`)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "tier 5 with a mechanical aid — an enumeration of every `#`-prefixed line in the authored spans of `crates/happenstance/src/lib.rs` and `docs/first-encounter.md`, each classified against the permitted list in `.bklg/docs-that-teach/application-author-path/_design.md:524`; followed by IQ-2.1's destructive render test (`.bklg/docs-that-teach/application-author-path/_decomposition.md:243-251`): read `target/doc/happenstance/index.html` and the rendered step page and write out the boundary from the visible lines alone"

- id: AC-006
  criterion: "GIVEN Persona 1 will only ever meet this page if the repository keeps it alive, and a maintainer must not be able to break it silently, WHEN the gate runs on this branch, THEN `docs/first-encounter.md` is registered as its own `#[cfg(doctest)] mod` in `xtask/src/narrative.rs` (one module per page, so a failure names the page) and carries its row in `docs/README.md`'s narrative table — neither *unregistered* nor a *dangling registration* — and **every** fence this story authored on both surfaces is compiled *and executed*, with no `ignore`, no `no_run`, no `compile_fail`, and nothing added to `IGNORE_ALLOWANCES`. A page in `docs/` the harness does not name is the documentation equivalent of built-but-unmounted."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: "tier 2 (HS-P0020's compiled-fence REQUIRED step, reached through this page's registration) and tier 3 (`cargo test --locked --workspace --all-features`, `xtask/src/main.rs:143-155`); plus the wiring evidence — the module line in `xtask/src/narrative.rs`, the row in `docs/README.md`, a search over both authored spans returning no `ignore`/`no_run`/`compile_fail` fence attribute, a `git diff` showing no addition to `IGNORE_ALLOWANCES`, and `cargo xtask ci --fast` green (`.redkiln/config.yaml:55`)"

- id: AC-007
  criterion: "GIVEN Persona 1 reading on a 1024×768 laptop and on a phone, whose attention is the scarce resource this project spends, WHEN they render either surface, THEN the composition the signed-off design fixed is what they meet: no fence exceeds **68 columns** (72 is where `overflow-x` engages on the 696px fence at 1024×768); no step fence exceeds **24 rendered lines** and the crate-root fence **32**; no `##` heading exceeds **22 characters** (the 200px sidebar TOC clips with an ellipsis — it never wraps); no paragraph exceeds **435 characters**; each step carries exactly the seven budgeted elements; exactly **one** answered-need line sits above everything and above the first fence on each surface, in HS-P0021's notation and no second notation; the pages introduce **zero** interactive affordances — no tab strip, accordion, disclosure triangle, nav widget, badge, button, CSS, JS or diagram, and nothing lands inside a `details` that is not `open`; and neither surface contains the words \"aggregate\", \"your aggregates\", \"one stream per entity\" or \"which stream\"."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "tier 5 against `.bklg/docs-that-teach/application-author-path/_design.md:532-617` and `:831-882` — widest-line and rendered-line counts read off `target/doc/happenstance/index.html` and the rendered step page at 1024×768, heading lengths counted, paragraph lengths counted, a case-insensitive search for the four forbidden phrases over both authored spans returning nothing, IQ-6's affordance enumeration whose correct answer is none (`_decomposition.md:287-291`), and an answered-need count of exactly one per surface above the first fence"

- id: AC-008
  criterion: "GIVEN Persona 1 needs to know that what they just watched is the library's *promise* and not this page's opinion, WHEN they reach the last sentence of step 3 and of the crate-root section, THEN the normative weight is a citation to **ES-25** by its stable clause id — the clause requiring the store to reject the append if and only if it holds a matching event strictly beyond the guard's `after`, and to report the rejection as `AppendError::ConditionViolated` — carried as an ordinary inline link in last position, never a tooltip or popover; no sentence containing \"MUST\" or \"MUST NOT\" appears on either surface that is not a link to a clause id; and the citation is provenance the reader may skip, not required reading — the encounter completes without opening `spec/SPECIFICATION.md` or the crate source."
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/narrative.rs"
  verifying_test: "tier 1 — `cargo xtask spec-trace` (REQUIRED step, `xtask/src/main.rs:315-327`; also inside `reachability_static`, `.redkiln/config.yaml:48`) resolving every citation this story adds; plus tier 5 — every \"MUST\" occurrence in the authored spans confirmed to be a clause link (anti-pattern 11, `.bklg/docs-that-teach/application-author-path/_design.md:865-866`), and IQ-1's falsification (`_decomposition.md:234-239`): strike every off-page link and confirm the encounter still completes"
```
