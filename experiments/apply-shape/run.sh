#!/usr/bin/env bash
# The reproducer. A Rust toolchain; Docker only for the Postgres case, which is
# reported as skipped when `APPLY_SHAPE_DATABASE_URL` is unset.
#
#   bash experiments/apply-shape/run.sh
#   APPLY_SHAPE_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:55432/postgres \
#       bash experiments/apply-shape/run.sh        # with a server (see tests/postgres.rs)
#
# Nothing here touches the workspace: no crate, no root Cargo.toml, no gate
# step. Transcripts land in `results/`, with the user's home directory
# replaced by `~` and the repository root by `<repo>`, so no machine path is
# committed.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS="${HERE}/results"
MANIFEST="${HERE}/Cargo.toml"
REPO="$(cd "${HERE}/../.." && pwd)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
mkdir -p "${RESULTS}"

# A Windows home path is all backslashes, and sed dialects disagree on escaping
# them — the first run of this script left home paths in two transcripts that
# way — so the scrub is a few lines of Python with no regex at all.
scrub() { python "${HERE}/scrub.py" "${REPO}"; }

# One transcript per step, headed by the command that produced it. A step's
# exit status is written into its own file rather than stopping the script:
# two of the steps are EXPECTED to fail, and the file is where that is read.
step() {
    local name="$1"; shift
    {
        echo "\$ $*"
        "$@" 2>&1
        echo "exit status: $?"
    } | scrub > "${RESULTS}/${name}.txt"
    tail -n 3 "${RESULTS}/${name}.txt"
}

{
    echo "--- toolchain ---"; rustc -vV; cargo -V
    echo "--- measured tree ---"
    git -C "${HERE}/../.." rev-parse HEAD
    git -C "${HERE}/../.." status --porcelain -- crates Cargo.toml Cargo.lock
    echo "--- docker ---"
    docker info --format '{{.ServerVersion}}' 2>&1 || true
} | scrub > "${RESULTS}/environment.txt"

# The three stores, the edge case, the generic spawn and the doctests.
step test-default cargo test --manifest-path "${MANIFEST}"
# The Postgres case, if a server was named. Otherwise its two tests are listed
# as ignored by the step above, and that is the recorded outcome.
if [ -n "${APPLY_SHAPE_DATABASE_URL:-}" ]; then
    # `--nocapture` so `records_the_server_version` prints the server's
    # `SELECT version()` into the transcript.
    step test-postgres cargo test --manifest-path "${MANIFEST}" --test postgres -- --ignored --test-threads=1 --nocapture
fi
# The proposed trait, the runner and a `!Send` store, for wasm32.
touch "${HERE}/src/lib.rs"
step wasm32-check cargo check --manifest-path "${MANIFEST}" --lib --target wasm32-unknown-unknown --no-default-features
# Lints, pedantic, on everything the default build compiles.
step clippy cargo clippy --manifest-path "${MANIFEST}" --all-targets -- -W clippy::pedantic
# EXPECTED TO FAIL: E0599 (no `position()` on `Delivered`) and E0277
# (`EdgeTally: SendProjection`), the two refusals the doctests claim.
step refusals cargo check --manifest-path "${MANIFEST}" --lib --features demonstrate-refusals
# EXPECTED TO FAIL: E0658. Return-type notation, the one-trait alternative to
# the `trait_variant` pair, is not stable on the pinned toolchain.
# EXPECTED TO FAIL: the two weakened generic spawners of README §3 (nine and
# five E0277s), and F1 on the SHIPPED `happenstance::Projection` (six E0277s).
step spawn-minimal-bounds cargo test --manifest-path "${MANIFEST}" --features demonstrate-spawn-minimal-bounds --test spawn --no-run
step spawn-store-as-projection cargo test --manifest-path "${MANIFEST}" --features demonstrate-spawn-store-as-projection --test spawn --no-run
step spawn-shipped-store-as-projection cargo test --manifest-path "${MANIFEST}" --features demonstrate-shipped-f1 --test shipped_spawn --no-run
step rtn-probe rustc --edition 2024 --crate-type lib --out-dir "${HERE}/target/rtn-probe" "${HERE}/probes/rtn.rs"
