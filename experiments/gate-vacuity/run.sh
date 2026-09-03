#!/usr/bin/env bash
#
# Re-derives everything under `results/` from a clean checkout.
#
# The question: how much of `cargo xtask ci` is load-bearing? Three counts —
# how many of the gate's named proof tests can be silenced with `#[ignore]`
# while the gate stays green; how many specification clauses have their
# rule-name check switched off by `spec_trace`'s seven guard substrings; and
# whether a warn-by-default rustc lint inside a constitution fence is caught by
# anything.
#
# It is NOT a gate step and must never become one. CF-34: performance — and, by
# the same argument, any harness that can turn a merge red for a reason the
# author did not choose — is measured separately from the conformance bar.
# Nothing in `.redkiln/config.yaml` or `cargo xtask ci` invokes this, and
# nothing can: this directory has no `Cargo.toml`, so cargo cannot see it at
# all. (`experiments/position-visibility/` is the precedent for a shell-driven
# experiment with no crate of its own.)
#
# **Nothing here writes inside the repository except this file's own
# `results/`.** Every edit is applied to `$WT`, a detached `git worktree` of
# `$COMMIT` living outside the tree, and is reverted with `git checkout -- .`
# between arms.
#
#   bash experiments/gate-vacuity/run.sh              # everything, in order
#   bash experiments/gate-vacuity/run.sh baseline     # just the green control
#   bash experiments/gate-vacuity/run.sh control      # just the red control
#   bash experiments/gate-vacuity/run.sh mechanism    # just the --list diff
#   bash experiments/gate-vacuity/run.sh ignore-all   # just count 1's headline
#   bash experiments/gate-vacuity/run.sh ignore-each  # just count 1's per-name
#   bash experiments/gate-vacuity/run.sh wasm         # count 1 on wasm32
#   bash experiments/gate-vacuity/run.sh guard        # just count 2
#   bash experiments/gate-vacuity/run.sh fence        # just count 3
#   bash experiments/gate-vacuity/run.sh cases        # the E2E-case counts (no build)
#
# Wall clock on the machine in the README: the first `cargo xtask ci` is a cold
# build and is most of it; see README "Conditions".
#
# NF-003: it terminates unattended. Every command below is a cargo invocation
# with no interactive prompt and no watchdog, and the only loop is over a fixed
# 31-line file.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RAW="$HERE/results/raw"
WT=${GATE_VACUITY_WT:-/d/gv-wt}
COMMIT=${GATE_VACUITY_COMMIT:-56ef6c5}
REPO="$(cd "$HERE/../.." && pwd)"
PHASE=${1:-all}

# `cargo xtask ci`, verbatim, and the two subcommands the gate itself shells out
# to. They are written here as the gate writes them so that a reader can check
# the arms were driven by the real thing rather than by a convenient subset.
CI=(cargo xtask ci)
PROOF=(cargo run --locked --quiet -p xtask -- proof-artefact)
SPEC_TRACE=(cargo run --locked --quiet -p xtask -- spec-trace)
DOCS=(cargo test --locked -p xtask --doc)

mkdir -p "$RAW"

log() { printf '\n==> %s\n' "$*"; }

# Runs one command in the worktree, tees it to `results/raw/<label>.txt` with
# its exit status appended, and echoes `<label> <status>` on stdout.
run_arm() {
    local label=$1
    shift
    local out="$RAW/$label.txt"
    (cd "$WT" && "$@") > "$out" 2>&1
    local code=$?
    printf 'EXIT=%s\n' "$code" >> "$out"
    printf '%s exit=%s\n' "$label" "$code"
    return 0
}

# Back to the pinned tree. Called before *and* after every arm: an arm that
# aborts halfway must not leave its edit for the next one to be blamed for.
reset_worktree() { git -C "$WT" checkout -- . ; }

setup() {
    if [ ! -d "$WT" ]; then
        log "worktree: $WT at $COMMIT"
        git -C "$REPO" worktree add --detach "$WT" "$COMMIT"
    fi
    reset_worktree
    git -C "$WT" status --porcelain > "$RAW/worktree-clean.txt"
    git -C "$WT" rev-parse HEAD >> "$RAW/worktree-clean.txt"
}

# ---------------------------------------------------------------------------
# The two controls. Both run before anything is counted, for the reason
# `experiments/append-condition/run.sh` states one level down: an arm that is
# fast and wrong wins every benchmark, and a harness that cannot fail reports
# every arm green.
# ---------------------------------------------------------------------------

# GREEN control: the pinned tree, unedited, must reproduce the claim the whole
# review rests on — `cargo xtask ci` green at 56ef6c5. If this is red, no number
# below means anything.
phase_baseline() {
    log "GREEN control: cargo xtask ci on the unedited tree (expect exit 0)"
    reset_worktree
    run_arm baseline-ci "${CI[@]}"
}

# RED control: rename one test `proof.rs` names, so `--list` no longer prints
# it. The gate MUST go red. Without this, a table of greens is
# indistinguishable from a harness that cannot fail — which is the failure mode
# being measured, so it has to be excluded by measurement rather than by
# assertion.
phase_control() {
    log "RED control: rename mutants_fail_exactly_their_declared_rules (expect non-zero)"
    reset_worktree
    python "$HERE/mutate.py" rename \
        "$WT/crates/happenstance-testkit/tests/mutation_coverage.rs" \
        mutants_fail_exactly_their_declared_rules \
        mutants_fail_exactly_their_declared_rules_renamed
    run_arm control-rename-ci "${CI[@]}"
    reset_worktree
}

# ---------------------------------------------------------------------------
# Count 1 — `#[ignore]`
# ---------------------------------------------------------------------------

# The mechanism, shown rather than argued: libtest's discovery output for a
# target with one test ignored, diffed against the same target unedited.
phase_mechanism() {
    log "mechanism: cargo test -- --list with and without an #[ignore]"
    reset_worktree
    (cd "$WT" && cargo test --locked -p happenstance-testkit --all-features \
        --test projection_harness_parity -- --list) > "$RAW/list-clean.txt" 2>&1
    python "$HERE/mutate.py" ignore \
        "$WT/crates/happenstance-testkit/tests/projection_harness_parity.rs" \
        no_harness_lists_a_rule_by_hand
    (cd "$WT" && cargo test --locked -p happenstance-testkit --all-features \
        --test projection_harness_parity -- --list) > "$RAW/list-ignored.txt" 2>&1
    # Cargo's own `Compiling`/`Finished` lines are on stdout and differ between
    # the two runs for a reason that is not the subject — the second one has a
    # file to rebuild. The listing itself is the lines libtest prints, and those
    # are what is compared.
    grep ": test$" "$RAW/list-clean.txt" > "$RAW/list-clean-names.txt"
    grep ": test$" "$RAW/list-ignored.txt" > "$RAW/list-ignored-names.txt"
    diff "$RAW/list-clean-names.txt" "$RAW/list-ignored-names.txt" \
        > "$RAW/list-diff.txt" 2>&1
    printf 'list-diff exit=%s (0 means libtest printed the ignored test identically)\n' $?
    reset_worktree
}

# The headline: every one of the 31 names silenced at once, then the whole gate.
# The maximal case rather than 31 separate gates, and it is the stronger claim:
# the baseline is green with all 31 running, so if the gate is also green with
# all 31 skipped, no step in it is reading their result.
#
# Both spellings, because the first run of this arm found the difference between
# them: a *bare* `#[ignore]` is refused by `clippy::ignore_without_reason`, which
# `pedantic = "warn"` (`Cargo.toml:168`) enables and `-D warnings` promotes, so
# the gate goes red at step 2 for a reason no part of it documents. The spelling
# clippy's own help line tells the author to reach for -- `#[ignore = ".."]` --
# is the one the count is about.
phase_ignore_all() {
    for spelling in ignore ignore-reason; do
        log "count 1 ($spelling): all 31 named proof tests silenced, then the whole gate"
        reset_worktree
        local n=0
        while read -r _pkg _tgt name file; do
            case "$_pkg" in \#* | "") continue ;; esac
            python "$HERE/mutate.py" "$spelling" "$WT/$file" "${name##*::}" || return 1
            n=$((n + 1))
        done < "$HERE/names.txt"
        echo "$n tests silenced with $spelling" > "$RAW/ignore-all-$spelling-count.txt"
        git -C "$WT" diff --stat >> "$RAW/ignore-all-$spelling-count.txt"
        run_arm "ignore-all-$spelling-ci" "${CI[@]}"
        reset_worktree
    done
}

# Per name, so the table can be read a row at a time. The step run here is the
# gate's own `proof-artefact` — the step whose entire purpose is to notice this —
# and the spelling is the one that survives clippy, so a green row is a row that
# survives the whole gate rather than only this step.
phase_ignore_each() {
    log "count 1: one name at a time, then cargo xtask proof-artefact"
    : > "$RAW/ignore-each.tsv"
    while read -r pkg tgt name file; do
        case "$pkg" in \#* | "") continue ;; esac
        reset_worktree
        python "$HERE/mutate.py" ignore-reason "$WT/$file" "${name##*::}" || continue
        local out="$RAW/each-${name//:/_}.txt"
        (cd "$WT" && "${PROOF[@]}") > "$out" 2>&1
        local code=$?
        printf 'EXIT=%s\n' "$code" >> "$out"
        printf '%s\t%s\t%s\t%s\n' "$pkg" "$tgt" "$name" "$code" \
            | tee -a "$RAW/ignore-each.tsv"
    done < "$HERE/names.txt"
    reset_worktree
}

# The wasm32 half of count 1. `proof.rs:1726` and `:2408` and `main.rs:403` all
# say the *runner* is where an `#[ignore]` on a wasm32 target is caught, so the
# host measurement above says nothing about those rows on its own. One name out
# of `CLOUDFLARE_UNIT_TESTS`, silenced, against the gate step that runs them.
#
# It carries the same probe the gate step does (`main.rs:481`). Without one, a
# machine with no `wasm-bindgen-test-runner` reports a red here that says
# nothing about `#[ignore]` — which is this experiment's own failure mode, one
# level up.
phase_wasm() {
    log "count 1 (wasm32): one #[ignore = \"…\"] under cargo xtask wasm-conformance"
    if ! wasm-bindgen-test-runner --version > /dev/null 2>&1; then
        echo "skipped: no wasm-bindgen-test-runner" | tee "$RAW/wasm-ignore-conformance.txt"
        return 0
    fi
    reset_worktree
    python "$HERE/mutate.py" ignore-reason \
        "$WT/crates/happenstance-cloudflare/src/lib.rs" \
        the_probe_is_not_vacuous '#[wasm_bindgen_test]'
    git -C "$WT" diff > "$RAW/wasm-ignore-diff.txt"
    run_arm wasm-ignore-conformance \
        cargo run --locked --quiet -p xtask -- wasm-conformance
    reset_worktree
}

# ---------------------------------------------------------------------------
# Count 2 — spec_trace's seven guard substrings
# ---------------------------------------------------------------------------

phase_guard() {
    log "count 2: spec-trace with the seven guard terms deleted"
    reset_worktree
    (cd "$WT" && "${SPEC_TRACE[@]}") > "$RAW/spec-trace-clean.txt" 2>&1
    printf 'EXIT=%s\n' $? >> "$RAW/spec-trace-clean.txt"

    python "$HERE/mutate.py" guard "$WT/xtask/src/spec_trace.rs"
    (cd "$WT" && "${SPEC_TRACE[@]}") > "$RAW/spec-trace-no-guard.txt" 2>&1
    printf 'EXIT=%s\n' $? >> "$RAW/spec-trace-no-guard.txt"
    reset_worktree

    # Each term alone, so the table can attribute the count rather than report
    # a total. The dagger is expected to contribute nothing at this commit — no
    # clause `Rule:` line carries one — and a term that contributes nothing is
    # exactly what this arm exists to find.
    for term in unit-test compile-test meta-test paren-new dagger leading-new space-new-tick; do
        reset_worktree
        python "$HERE/mutate.py" guard1 "$WT/xtask/src/spec_trace.rs" "$term"
        (cd "$WT" && "${SPEC_TRACE[@]}") > "$RAW/spec-trace-drop-$term.txt" 2>&1
        printf 'EXIT=%s\n' $? >> "$RAW/spec-trace-drop-$term.txt"
    done
    reset_worktree

    # Derived from the raw output above and written back into `raw/`, so that
    # re-running this file reproduces every file the tables were read off. The
    # maturity marker is read out of the clause's own section in the pinned tree.
    grep "names rule" "$RAW/spec-trace-no-guard.txt" \
        | sed 's/.*— \([A-Z]*-[0-9]*\) names rule `\([^`]*\)`.*/\1 \2/' \
        | sort > "$RAW/guard-newly-reported.txt"
    for term in unit-test compile-test meta-test paren-new dagger leading-new space-new-tick; do
        grep "names rule" "$RAW/spec-trace-drop-$term.txt" \
            | sed 's/.*— \([A-Z]*-[0-9]*\) names rule `\([^`]*\)`.*/\1 \2/'
    done | sort -u > "$RAW/guard-single-term-union.txt"
    : > "$RAW/guard-clause-maturity.txt"
    for c in $(awk '{print $1}' "$RAW/guard-newly-reported.txt" | sort -u); do
        line=$(grep -n -m1 -E "(^#### |^\*\*)$c\b" "$WT/spec/SPECIFICATION.md" | cut -d: -f1)
        marker=$(sed -n "${line},$((line + 60))p" "$WT/spec/SPECIFICATION.md" \
            | grep -m1 -oE "\[(FROZEN|PROVISIONAL|DEFERRED|NON-NORMATIVE)")
        printf '%s %s]\n' "$c" "$marker" >> "$RAW/guard-clause-maturity.txt"
    done
}

# ---------------------------------------------------------------------------
# Count 3 — a warn-by-default rustc lint inside a constitution fence
# ---------------------------------------------------------------------------

phase_fence() {
    log "count 3: a non_snake_case binding in a constitution atom's fence"
    reset_worktree

    # The control first, and it is the one narrative.rs's own re-measurement
    # used: the same snippet must warn under plain rustc, or the arm below is
    # measuring a lint that was never going to fire.
    cat > "$RAW/lint-control.rs" <<'RS'
fn main() {
    let notSnakeCase = 1u8;
    assert_eq!(notSnakeCase, 1);
}
RS
    # `-o lint-control.meta` and not `-o /dev/null`: on Windows rustc renames its
    # output into place and cannot rename across drives, so `/dev/null` makes the
    # control exit 1 for a reason that is not the lint.
    (cd "$RAW" && rustc --edition 2024 --emit=metadata -o lint-control.meta \
        lint-control.rs) > "$RAW/lint-control.txt" 2>&1
    printf 'EXIT=%s\n' $? >> "$RAW/lint-control.txt"
    rm -f "$RAW/lint-control.meta"

    # The reachability control: the same insertion point, a type error instead of
    # a lint violation. The docs step MUST go red here, or a green result below
    # means only that the edit was never handed to a compiler.
    python "$HERE/mutate.py" fence-typeerror \
        "$WT/standards/rust/00-prime-directives.md"
    (cd "$WT" && RUSTDOCFLAGS="-D warnings" "${DOCS[@]}") \
        > "$RAW/fence-control-typeerror.txt" 2>&1
    printf 'EXIT=%s\n' $? >> "$RAW/fence-control-typeerror.txt"
    reset_worktree

    python "$HERE/mutate.py" fence "$WT/standards/rust/00-prime-directives.md"
    git -C "$WT" diff > "$RAW/fence-diff.txt"
    (cd "$WT" && RUSTDOCFLAGS="-D warnings" "${DOCS[@]}") \
        > "$RAW/fence-docs-step.txt" 2>&1
    printf 'EXIT=%s\n' $? >> "$RAW/fence-docs-step.txt"

    # And the same edit under the whole gate rather than the one step, because
    # the claim under test is about `cargo xtask ci` and not about `cargo test -p
    # xtask --doc`. Two steps read these files, and `lint-constitution` — which
    # reads them as text — runs first, so this is what says whether *anything* in
    # the gate answers a warn-by-default lint inside a fence.
    (cd "$WT" && "${CI[@]}") > "$RAW/fence-ci.txt" 2>&1
    printf 'EXIT=%s\n' $? >> "$RAW/fence-ci.txt"
    reset_worktree
}

# ---------------------------------------------------------------------------
# The one arm that needs no worktree and no build: two counts over the two
# specification documents as text. It is here rather than in its own file
# because Q-04 is the same shape of question as count 2 — a [FROZEN] clause
# whose obligation `spec-trace` does not check — and a reader comparing them
# should not have to run two things.
# ---------------------------------------------------------------------------
phase_cases() {
    log "E2E cases: CF-37's per-case clause naming, and the inverse relation"
    python "$HERE/cases.py" "$REPO" | tee "$RAW/e2e-cases.txt"
}

case "$PHASE" in
all)
    setup
    phase_baseline
    phase_control
    phase_mechanism
    phase_ignore_all
    phase_ignore_each
    phase_wasm
    phase_guard
    phase_fence
    phase_cases
    ;;
baseline) setup && phase_baseline ;;
control) setup && phase_control ;;
mechanism) setup && phase_mechanism ;;
ignore-all) setup && phase_ignore_all ;;
ignore-each) setup && phase_ignore_each ;;
wasm) setup && phase_wasm ;;
guard) setup && phase_guard ;;
fence) setup && phase_fence ;;
cases) phase_cases ;;
*)
    echo "unknown phase: $PHASE" >&2
    exit 2
    ;;
esac

echo
echo "Raw output is under results/raw/. The tables in results/*.md are written"
echo "by hand from it, because a table nobody read is a table nobody checked."
