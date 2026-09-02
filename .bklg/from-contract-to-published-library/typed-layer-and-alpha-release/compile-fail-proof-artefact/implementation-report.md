---
item: "HS-S0030"
stage: implement
created: "2026-08-16"
updated: "2026-08-16"
---

# Implementation Report — The compile-fail case, its negative control, and its gate row

**All seven ACs are satisfied. Nothing is blocked and nothing is deferred.**

EC-001 did not fire. `trybuild` was absent from the workspace when this story started, and
the story's own PR boundary is what admits it: it is declared once at `Cargo.toml:123`
under `# --- dev / tooling only ---`, consumed only as a `[dev-dependencies]` of a
`publish = false` example, and it clears `deny.toml`'s **unmodified** allowlist. Adoption
was a measurement, not an assumption — the numbers are below. Had `cargo deny` refused, the
response would have been EC-002's escalation and this report would say so.

The claim is now a checked guarantee. `examples/course-subscriptions/tests/ui/unhandled_variant.rs`
restates the worked example's domain enum with one more variant, keeps the `Seats` fold
otherwise verbatim, and the build stops at `error[E0004]: non-exhaustive patterns` with
the `-->` span on the fold's own `match` — pinned byte for byte. Its negative control, one
arm different, compiles and runs. And the discriminator was **observed to discriminate**:
the protection was deliberately removed and the gate went red.

## TDD Evidence

The red here is unusual and worth naming: this story's deliverable *is* an instrument, so
"red" means the instrument was absent, then present-but-unpinned, then pinned.

**Stage one — no target.**

```console
$ cargo test -p course-subscriptions --test ui
error: no test target named `ui`
```

**Stage two — the harness and both fixtures, no snapshot.** `trybuild` refused to pass a
compile-fail case it had no expectation for, and wrote a candidate into `wip/`:

```console
$ cargo test -p course-subscriptions --test ui
test ui::the_negative_control_compiles ... ok
test ui::an_unhandled_variant_fails_to_compile ... FAILED
  panicked at trybuild-1.0.120/src/run.rs:106:13:
  successfully created new stderr files for 1 test cases
```

The negative control was green from the first run, which is the evidence that the fail
case's red was the *missing arm* and not a typo, a bad import or a fixture that could not
build at all.

**Stage three — read the candidate, fix the fixture, regenerate.** The first candidate's
`help:` block quoted this line of the fixture:

```
91 ~             Enrolment::StudentUnsubscribed => self.taken = self.taken.saturating_sub(1),
```

88 columns, which is over AC-007's 80-column budget for *quoted* fixture source. The fix
was to the **fixture**, never to the snapshot: two arms moved to block bodies, which
rustfmt accepts at the 100-column budget and which shortens every line rustc quotes. The
`wip/` candidate was deleted and the snapshot regenerated with `TRYBUILD=overwrite`. That
order — generate, then read, then fix the *source* and regenerate — is the whole of AC-007,
and it is the opposite of the habit that makes snapshot tests worthless.

**Stage four — green, and stable.**

```console
$ cargo test -p course-subscriptions --test ui     # three consecutive runs
test result: ok. 2 passed; 0 failed  (2.26s)
test result: ok. 2 passed; 0 failed  (1.60s)
test result: ok. 2 passed; 0 failed  (1.61s)
```

Three runs rather than one, because two `trybuild::TestCases` share one test target and the
spec flagged concurrency as a flake risk. No serialisation was needed; if it ever is, it
has to live inside the target, because `proof.rs`'s `cargo_args` is a fixed array with
nowhere to put `--test-threads`.

| AC | test / artefact | red | green |
| --- | --- | --- | --- |
| AC-001 | `ui::an_unhandled_variant_fails_to_compile` | `no test target named ui`, then `successfully created new stderr files` | ✅ E0004 pinned byte for byte |
| AC-002 | the committed `.stderr`, asserted by the AC-001 test | same | ✅ first block is E0004; span is `tests/ui/unhandled_variant.rs:88:15` |
| AC-003 | `ui::the_negative_control_compiles` | `no test target named ui` | ✅ compiles *and runs*; one arm is the whole delta |
| AC-004 | the recorded mutation | n/a — it *is* the red | ✅ observed red, reverted, re-run green |
| AC-005 | `cargo xtask proof-artefact` | the target was in no row, so it could be renamed or emptied in silence | ✅ `course-subscriptions/ui: 2 named tests present` |
| AC-006 | `cargo deny` × 2, `cargo xtask ci --fast` | `trybuild` absent from `Cargo.toml`/`Cargo.lock` | ✅ `licenses ok`, `advisories ok`, `ci --fast` exit 0 |
| AC-007 | the fixture and the snapshot, read | the 88-column quoted line above | ✅ every quoted line ≤ 28 columns; `rustfmt --check` silent |

### AC-004 — the recorded mutation, in full

The protection is the *absence* of a `_ =>` arm. It was removed:

```diff
--- a/examples/course-subscriptions/tests/ui/unhandled_variant.rs
+++ b/examples/course-subscriptions/tests/ui/unhandled_variant.rs
@@ fn apply(&mut self, event: Self::Event) {
             Enrolment::StudentUnsubscribed => {
                 self.taken = self.taken.saturating_sub(1);
             }
+            _ => {}
         }
```

```console
$ cargo xtask proof-artefact
test tests/ui/handled_variant.rs ... ok
test ui::the_negative_control_compiles ... ok

test tests/ui/unhandled_variant.rs ... error
Expected test case to fail to compile, but it succeeded.

test ui::an_unhandled_variant_fails_to_compile ... FAILED
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

xtask failed: `course-subscriptions`'s `ui` proof artefact failed with exit code: 101
```

Two things in that output matter beyond the word FAILED. The message is *specific* — it
says the case compiled when it was expected not to, which is the mutation and nothing else.
And the **negative control stayed green**, which is what makes the red attributable: a
fixture broken for an unrelated reason would have taken both cases down.

The edit was then reverted from a byte-for-byte copy (**not** `git restore` — on Windows
that re-checks-out CRLF and rustfmt then rejects the whole file), and the gate re-run:

```console
$ cargo xtask proof-artefact
course-subscriptions/ui: 2 named tests present
test result: ok. 2 passed; 0 failed
```

## Commits

One checkpoint commit, carrying the whole story — the dependency, the harness, both
fixtures, the snapshot, the gate row, this report and the ledger:

```
feat(typed-layer-and-alpha-release): Compile-fail proof artefact
Story: typed-layer-and-alpha-release/compile-fail-proof-artefact
```

Its short SHA is reported in this run's slice digest and is recoverable here with
`git log --grep "Story: typed-layer-and-alpha-release/compile-fail-proof-artefact" --oneline`.

One follow-up commit repairs a constitution citation this PR's `Cargo.toml` edit moved —
see *Notes*. It is separate on purpose, so this story's own diff stays inside its declared
PR boundary; the same shape as `d95c760` and `b77cffb` earlier on this branch.

## Changes

| File | Shape of the change |
| --- | --- |
| `Cargo.toml` | `trybuild = "1.0"` at `:123`, in the `# --- dev / tooling only ---` block, with the licence/advisory measurement and the ten-node lockfile delta written into the comment above it. |
| `examples/course-subscriptions/Cargo.toml` | A `[dev-dependencies]` section with `trybuild.workspace = true`. |
| `examples/course-subscriptions/tests/ui.rs` | **New.** Two `#[test]`s inside `mod ui`, each building its own `trybuild::TestCases`. Its module doc carries the argument the design settled — why not a `compile_fail` doctest, why the fixtures live beside the example, and why two tests rather than one. |
| `examples/course-subscriptions/tests/ui/unhandled_variant.rs` | **New.** The fail fixture: the example's domain plus `CourseClosed`, the `Seats` fold with no wildcard arm, a `fn main` that folds all four variants. |
| `examples/course-subscriptions/tests/ui/handled_variant.rs` | **New.** The negative control. `diff` against the fixture above prints exactly one line. |
| `examples/course-subscriptions/tests/ui/unhandled_variant.stderr` | **New.** rustc's own render, generated under the pinned 1.97.1 and read before committing. |
| `xtask/src/proof.rs` | `const COMPILE_FAIL_PAIR` (`:236`) and one `Artefact` row (`:279-284`). The module doc's target count moved from six to seven. No new gate step. |
| `Cargo.lock` | Ten new nodes, all dev-only, all inside the unmodified allowlist. |

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p course-subscriptions --test ui` ×3 | 2 passed each time; no flake from the two `TestCases` sharing a target |
| `cargo xtask proof-artefact` | green; `course-subscriptions/ui: 2 named tests present` — **this is the AC-005 evidence**, because `cargo xtask affected` runs no proof-artefact step (`xtask/src/affected.rs:119-125`) |
| `cargo xtask proof-artefact` with the mutation applied | **exit 101**, `Expected test case to fail to compile, but it succeeded` |
| `cargo deny check licenses` | `licenses ok` against an **unmodified** `deny.toml` |
| `cargo deny check advisories` | `advisories ok` |
| `rustfmt --edition 2024 --check tests/ui/*.rs` | silent — run explicitly, because files under `tests/ui/` are not cargo targets and `cargo fmt` never reaches them |
| `cargo fmt --all --check` | clean |
| `cargo xtask affected --base main` | **`affected gate passed`** |
| `cargo xtask ci --fast` | **exit 0** — the project's declared integration bar, including all four `wasm32` steps, the proof artefacts and the packaging step |

## Notes

**Deviation 1 — the `_design.md` route conflict, bound toward the rule and reported here.**
`_design.md:80` routes the `compile-fail-diagnostic` surface to `crates/happenstance/tests/ui/`,
while the same file's selector (`:212-214`) and anti-pattern 13 (`:999-1001`) require the
span to land in `examples/course-subscriptions/` and forbid *"anything under `crates/`"*. A
`trybuild` fixture's span is its own path, so the two cannot both be literally true. The
harness is at `examples/course-subscriptions/tests/ui.rs` with its fixtures under
`tests/ui/`, which satisfies the selector, AC-U07 and anti-pattern 13 at once — and which is
in any case the only placement that works mechanically, because a variant cannot be added to
an imported enum and this package is a binary crate with no lib target. **`_design.md` was
not edited and anti-pattern 13 was not softened.** Raised here for the slice review.

**Deviation 2 — one file outside the PR boundary, in a separate commit.**
`standards/rust/90-skeletons-and-todo.md:269` cites `./Cargo.toml:136` for the phrase *"the
allow protected nothing"*. Adding `trybuild` to the dev/tooling block moved that line to
`:154`, and `cargo xtask lint-constitution` — a REQUIRED step of `cargo xtask ci --fast` —
went red naming the drift. The citation is repaired to `:154` in a follow-up commit rather
than in the story checkpoint, so the story's own diff stays inside its declared boundary.
`cargo xtask lint-constitution` then reports `27 atoms, all consistent`.

**Two simplifications inside the fixture, both deliberate and both named in the file
itself.** The mirrored enum's variants carry only what the fold reads, and `tags` returns
the empty set. The example's own tags come from two validated identity newtypes
(`src/main.rs:112-181`); restating those in the fixture would add imports, serde attributes
and conversions to a file whose entire value is that the **first** thing rustc prints is the
missing arm. Separately, the fixture's `event_type` **is** extended for the new variant and
only `apply` is not — because `EVENT_TYPES`/`event_type` agreement is not compiler-enforced,
so leaving that arm out too would have printed a second error and buried the one being
pinned. Both are recorded in the fixtures' own module docs with a `file:line` pointer to
what they mirror, which is the mitigation the spec asks for the drift risk.

**EC-005 did not fire.** The slice-mate's fold is a `match` over `Self::Event` with no
wildcard arm in all three decision models — asserted independently by
`runs::the_event_set_is_named_once` — so the added variant is genuinely non-exhaustive and
no finding against the mate was needed.

**Left alone deliberately.** `spec/SPECIFICATION.md`'s PS-36 disposition row says a
compile-fail diagnostic *"needs a `trybuild`-style stderr snapshot"*, which is exactly the
fact this PR lands. It is not edited: the clause ledger is HS-P0016's, and
`cargo xtask spec-trace` stays green without touching it. `deny.toml` is not edited. No
`crates/**` source file is touched at all — no `pub` item, no re-export, no feature.
