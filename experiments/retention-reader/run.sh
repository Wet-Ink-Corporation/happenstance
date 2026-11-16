#!/usr/bin/env bash
# The reproducer. A Rust toolchain and nothing else: every store is in memory
# or a SQLite file under the system temporary directory, and no scenario needs
# a network, a server or a tokio runtime.
#
#   bash experiments/retention-reader/run.sh
#
# Nothing here touches the workspace: no crate, no root Cargo.toml, no gate
# step. Transcripts land in `results/`, with the target directory replaced by
# `<target>`, the repository root by `<repo>` and the home directory by `~`,
# so no machine path is committed.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS="${HERE}/results"
MANIFEST="${HERE}/Cargo.toml"
REPO="$(cd "${HERE}/../.." && pwd)"
mkdir -p "${RESULTS}"

TARGET="${CARGO_TARGET_DIR:-${HERE}/target}"
scrub() { sed -e "s|${TARGET}|<target>|g" -e "s|${REPO}|<repo>|g" -e "s|${HOME}|~|g"; }

# One transcript per step, headed by the command that produced it, with the
# step's exit status written into the file rather than stopping the script.
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
    git -C "${REPO}" rev-parse HEAD
    git -C "${REPO}" status --porcelain -- crates Cargo.toml Cargo.lock
    echo "--- os ---"; uname -srm
} | scrub > "${RESULTS}/environment.txt"

# Every scenario, one at a time, with each test's name in the transcript.
step test-all cargo test --locked --manifest-path "${MANIFEST}" -- --test-threads=1
# The lints the manifest copies from the workspace, warnings denied.
step clippy cargo clippy --locked --manifest-path "${MANIFEST}" --all-targets -- -D warnings
