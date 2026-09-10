#!/usr/bin/env bash
# The reproducer. One command, a Rust toolchain, and nothing else — no Docker,
# no database, no network.
#
#   bash experiments/polling-cost/run.sh            # the whole sweep
#   HS_TAG=pass-002 bash .../run.sh                 # a second pass, beside the first
#   HS_CELLS=6 bash .../run.sh                      # a smoke run of the first six cells
#
# Nothing here touches the workspace: no crate, no root Cargo.toml, no gate step.
# `cargo xtask ci` must stay runnable whether or not this has ever been run, and
# `cargo xtask affected --base main` over a diff touching only this directory
# must keep selecting no package.
#
# A pass is a directory under `results/`. Re-running under a tag that already
# exists is **refused**, naming the pass that is in the way: a re-measurement
# writes beside the pass it is being compared with, so undoing one is deleting
# one directory and a committed pass is never edited in place.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TAG=${HS_TAG:-pass-001}

echo "polling-cost: tag ${TAG}"

# The environment, captured where a reader will look for it rather than only
# inside the JSON. The manifest line in the pass carries the machine-readable
# copy; this is the transcript.
echo "--- toolchain ---"
rustc -vV
cargo -V
echo "--- measured tree ---"
git -C "${HERE}/../.." rev-parse HEAD
git -C "${HERE}/../.." status --porcelain -- crates Cargo.toml Cargo.lock || true
echo "--- host ---"
uname -a 2>/dev/null || echo "uname unavailable"

# `--release`, always. A debug build measures the optimiser, and the manifest
# records the profile so a reader can tell.
cargo build --release --manifest-path "${HERE}/Cargo.toml"

HS_TAG="${TAG}" cargo run --release --manifest-path "${HERE}/Cargo.toml"

echo "--- written ---"
ls -1 "${HERE}/results/${TAG}"
