---
item: HS-S0097
stage: discover
created: 2026-08-12T13:03:06.859Z
updated: 2026-08-12T13:03:06.859Z
template_sig: 86ce4036
rendered_sig: 25e5c0a9
---

# Discover — A stranger installs it from the registry and it works

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:69 | "The proof artefact: from a scratch project outside this workspace, `cargo add` the published crate from the registry — no path dependency, no workspace feature unification — run the README's own quick-start write-then-read cycle against the resolved version, and record the run." |
| AC-008 | `project.md`:254-257 | "From a scratch project outside this workspace, adding the published crate from the registry and running the smallest write-then-read cycle succeeds against the published version, not a path dependency. This is the project's proof artefact." |
| DR-8 | `project.md`:188-190 | Same requirement, restated as a derived requirement: "no path dependency and no workspace feature unification." |
| `dependsOn: publish-0-2-0` | manifest, `_storymap.md`:173 | "the only step that can only run *after*" — this story is structurally impossible before the crate is actually on the registry. |
| Testing brief's classification | `_decomposition.md`, Testing brief AC table, AC-008 row | "**e2e**, the project's proof artefact" — the only true end-to-end instrument in this project, per the Testing brief's Intent (433-441): "that is the only claim in this project a compiled test suite cannot stand in for." |
| Testing brief's named wrong implementation | `_decomposition.md`, Testing brief AC table, AC-008 row | "a smoke that resolves a path dependency or inherits workspace feature unification instead of the published registry version — this is exactly what 'not a path dependency' in AC-008's own text forecloses." |
| Fixtures-and-seams guidance | `_decomposition.md`, Testing brief "Fixtures and seams to mock" (520-523) | "AC-008's smoke is the one seam that must not be mocked. It needs a real `cargo add` against the real registry, which only resolves after publish — it is inherently a post-publish verification, not a pre-merge gate step, and should be recorded as such rather than folded into `cargo xtask ci`." |
| The text this story runs | `_design.md`:511, 660-695, AC-UX-012 | The same fence as `crates/happenstance/README.md`'s quick start, compiled as `happenstance`'s own doctest via `crates/happenstance/src/lib.rs:10`. `guarantees-and-docs-rs-presentation` is the story that makes that identity hold; this story is what actually runs the resulting text against the registry. |
| IQ-5, reachable evidence | `_decomposition.md`, UX brief (289-295) | "No claim on a published surface may have 'run our CI' or 'check out the repo' as its only evidence... the quick start must be copy-pasteable into an empty project and work with **no path dependency and no workspace feature unification**." |
| Workspace feature unification, the specific trap | Cargo's own resolver behaviour (general Rust knowledge, corroborated by AC-008's explicit wording) | A workspace member depending on the crate via a path dependency inherits every other member's enabled features through Cargo's default unification within one resolve — a smoke run *inside* this workspace could pass with a feature combination a truly external `cargo add` would never produce. |

## Questions

- **What is "the smallest write-then-read cycle" concretely — is it exactly the doctest example, or a separate minimal program?** `_design.md`'s doctest (660-695) is explicitly framed as "the same text three times over," including "the program `stranger-install-smoke` runs against the registry version." Answered: it is the same 19-line example, not a separately maintained script — AC-UX-012 and AP-12 both forbid a second, independently-drifting copy.
- **Where does the scratch project live, and is its run recorded manually or automated?** Not specified by any brief beyond "outside this workspace." Deferred to spec: likely a throwaway `cargo new` outside the repository root (or in a CI job with no workspace context), with its `Cargo.toml`, output, and the date recorded as the proof artefact — mirroring the "committed artefact with a date" pattern DoD item 3 already establishes for the other two instruments.

## Decision

After `publish-0-2-0`, a scratch Cargo project created outside this workspace — with no path dependency and no workspace-level feature unification — adds `happenstance` from the registry via ordinary `cargo add`, and runs the exact write-then-read example that `crates/happenstance/README.md`'s quick start shows and `crates/happenstance/src/lib.rs`'s doctest compiles. The run is recorded as a dated artefact. This is the project's only true end-to-end check and cannot be mocked or folded into `cargo xtask ci`, because it depends on the registry resolution that only exists after the irreversible act. Spec pins where the scratch project lives and how its run is recorded.

## The wrong implementation

A "stranger-install smoke" implemented as a workspace member — a new crate under `examples/` or a dev-dependency in an existing workspace `Cargo.toml` — depending on `happenstance` by `path = "../crates/happenstance"` (or even by version, but still inside the workspace's `Cargo.lock` resolve), which then runs the write-then-read example and passes. It satisfies a naive reading of AC-008 ("a scratch project... adding the published crate... running a write-then-read cycle succeeds") because the code runs and the assertions hold. It is wrong for the precise reason AC-008's own text forecloses it: a path dependency never resolves against the registry at all — it could pass even if `0.2.0` were never actually published, or if the published tarball were missing a file `cargo package --list` would have caught, because Cargo never fetches the tarball for a path dependency. Worse, if it stays inside the workspace, Cargo's default feature unification means every other workspace member's enabled features leak into the resolve, so the smoke could pass under a feature combination no external `cargo add happenstance` user would ever actually get. This is exactly the testing brief's own named wrong implementation for AC-008, and it is also the prompt's own worked example of the class of defect this project is watching for. What catches it: the scratch project must live genuinely outside the workspace (no ancestor `Cargo.toml` with a `[workspace]` table reaching it) and its `Cargo.toml` must declare `happenstance = "0.2.0"` with no `path` or `git` key — checked by inspecting the manifest and the resolved `Cargo.lock`'s source, which for a registry dependency reads `registry+https://github.com/rust-lang/crates.io-index` rather than a filesystem path.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
