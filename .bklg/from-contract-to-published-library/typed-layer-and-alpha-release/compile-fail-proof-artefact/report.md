---
item: "HS-S0030"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — The compile-fail case, its negative control, and its gate row

## Findings Ledger

**Outcome: seven of seven ACs satisfied. Nothing deferred, nothing blocked.**

**EC-001 did not fire, and that is the first thing a reviewer will want to know.**
`trybuild` was absent from `Cargo.toml` and `Cargo.lock` when this story started. It is now
declared once at `Cargo.toml:123` under `# --- dev / tooling only ---` and consumed only as
a `[dev-dependencies]` of the `publish = false` example — which is what the story's own PR
boundary provides for. It was adopted on a measurement: `cargo deny check licenses` →
`licenses ok`, `cargo deny check advisories` → `advisories ok`, both against a `deny.toml`
that was **not edited**. Ten dev-only nodes enter the lockfile and no published crate's
graph, feature set or MSRV floor moves. Had `cargo deny` refused, the answer would have been
EC-002's escalation, not a widened allowlist.

The mount point is `xtask/src/proof.rs` — `COMPILE_FAIL_PAIR` at `:236` and the `Artefact`
row at `:279-284`, reached by the existing REQUIRED step at `xtask/src/main.rs:178-190`. No
new gate step was needed and none was added. Nothing else in the tree references these two
fixtures, which is precisely the argument for mounting here.

| AC | result | proved by | notes |
| --- | --- | --- | --- |
| AC-001 | satisfied | `ui::an_unhandled_variant_fails_to_compile` | The fixture restates the worked example's enum plus `CourseClosed` and keeps the `Seats` fold verbatim with **no `_ =>` arm**. `error[E0004]: non-exhaustive patterns`, pinned byte for byte by a checked-in `.stderr`. |
| AC-002 | satisfied | the committed `.stderr`, asserted by the AC-001 test | Line 1 is the E0004 block; **nothing precedes it** — no warning, no unrelated error. Line 2 is `--> tests/ui/unhandled_variant.rs:88:15`, the fold's own `match` line inside `examples/course-subscriptions/`. Never a macro body, never under `crates/`. Single-error-surface was designed for: `event_type` **is** extended for the new variant and only `apply` is not, because a second missing arm would have printed a second error and buried this one. |
| AC-003 | satisfied | `ui::the_negative_control_compiles` | `diff` between the two fixtures prints **exactly one line**. `trybuild`'s `pass` compiles *and runs* it, so its assertions on the folded model execute. Any unrelated breakage turns both cases red at once. |
| AC-004 | satisfied | the **recorded mutation**, performed and observed | `_ => {}` added to the fail fixture → `cargo xtask proof-artefact` exited **101** with `Expected test case to fail to compile, but it succeeded`. The negative control stayed green throughout, which is what makes the red attributable to the mutation. Reverted from a byte copy; re-run green. Full transcript in `implementation-report.md`. |
| AC-005 | satisfied | `cargo xtask proof-artefact`, **not** `cargo xtask affected` | The row names the **tests**, asserted out of `cargo test -- --list` before anything runs, so deletion, truncation, `#[ignore]` and rename each fail. `course-subscriptions/ui: 2 named tests present`. Exercised end to end by `cargo xtask ci --fast`, exit 0. |
| AC-006 | satisfied | `cargo deny` × 2 against an unmodified `deny.toml`; the `ci --fast` packaging step | One declaration, workspace-wide; dev-only; `publish = false` consumer. The diff touches **no file under `crates/`** — no `pub` item, no re-export, no feature, no `_design.md` `## Items` row. |
| AC-007 | satisfied | the fixture and the snapshot, read as artefacts | Generated with `TRYBUILD=overwrite` under the pinned 1.97.1, then read. **Never hand-edited**: when the first candidate's `help:` block quoted an 88-column fixture line, the *fixture* was changed and the snapshot regenerated. `rustfmt --edition 2024 --check` on both fixtures is silent — run explicitly, because files under `tests/ui/` are not cargo targets and `cargo fmt` never reaches them. Every quoted source line is ≤ 28 columns. |

**Gate evidence.** `cargo xtask proof-artefact` green (cited separately from `affected`, per
the spec's own warning that the story gate is silent about this story's deliverable).
`cargo xtask affected --base main` → `affected gate passed`. `cargo xtask ci --fast` →
**exit 0**, the project's declared integration bar. `cargo test -p course-subscriptions
--test ui` run three consecutive times, 2 passed each, which is also the check that two
`TestCases` sharing one target do not race.

**Two deviations, both reported here rather than absorbed.**

1. **The `_design.md` route conflict is bound toward the rule.** `:80` routes the surface to
   `crates/happenstance/tests/ui/`; the same file's selector and anti-pattern 13 require the
   span to land in `examples/course-subscriptions/` and forbid *"anything under `crates/`"*.
   A `trybuild` fixture's span is its own path, so both cannot hold. The harness is at
   `examples/course-subscriptions/tests/ui.rs`, which satisfies the selector, AC-U07 and
   anti-pattern 13 at once — and which is the only mechanically workable placement anyway.
   **`_design.md` was not edited and anti-pattern 13 was not softened.** For the slice
   review to confirm.
2. **One file outside the PR boundary, in a separate commit.** Adding `trybuild` to the
   dev/tooling block moved `Cargo.toml:136` to `:154`, and
   `standards/rust/90-skeletons-and-todo.md:269` cites that line — so
   `cargo xtask lint-constitution`, a REQUIRED step, went red. The citation is repaired in a
   follow-up commit so this story's own diff stays inside its declared boundary; the same
   shape as `d95c760` and `b77cffb` earlier on this branch. `lint-constitution` then reports
   `27 atoms, all consistent`.

**One risk carried forward, named rather than closed.** The fixtures are a *mirror* of
`examples/course-subscriptions/src/main.rs` and nothing forces them to stay one. Both
fixtures' module docs name what they mirror with `file:line` pointers, and both name their
two simplifications (variants carry only what the fold reads; `tags` returns the empty set,
because restating the example's two validated identity newtypes would add failure surface to
a file whose whole value is a single-error output). The *guarantee* being pinned —
exhaustiveness of a fold over `Self::Event` — stays true under drift, so this is a
review-grade fidelity risk rather than a correctness one.

**EC-005 did not fire.** The slice-mate's three folds are all `match`es over `Self::Event`
with no wildcard arm, asserted independently by `runs::the_event_set_is_named_once`, so the
added variant is genuinely non-exhaustive and no finding against the mate was needed.

**Left alone deliberately.** `spec/SPECIFICATION.md`'s PS-36 disposition row states that
pinning a compile-fail diagnostic *"needs a `trybuild`-style stderr snapshot"* — exactly the
fact this PR lands, and exactly the edit that is out of scope. It is HS-P0016's, and
`cargo xtask spec-trace` stays green without it. `deny.toml` untouched. `CHANGELOG.md`
untouched.
