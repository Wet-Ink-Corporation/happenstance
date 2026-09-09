#!/usr/bin/env bash
# Runs the adapter re-measurement INSIDE the Postgres container, over the unix
# socket.
#
# # Why this exists rather than just running the binary on the host
#
# The first attempt ran on the host and talked to a mapped TCP port. Its baseline
# drifted 1.13x to 2.39x WITHIN a single paired triple, and arm C's ratio spread
# 0.94 to 1.33 across repeats — against an effect phase 2 measured at 0.99 to
# 1.03. The noise was an order of magnitude larger than the signal, so no number
# from it belonged in a decision record.
#
# Pairing, three repetitions, 10,000 seed rows, a five-second warmup, an
# eight-second inter-pass settle and a fresh container were all tried and none of
# them closed the gap. What phase 2 did differently is the thing left: it ran
# `pgbench` inside the container over `/var/run/postgresql`, so the measurement
# never crossed the host/guest boundary or the Docker Desktop TCP proxy.
#
# This does the same for the Rust harness: build it for the container's platform,
# put the binary inside, and connect over the socket.
set -euo pipefail

# Git Bash rewrites any argument that looks like a unix path into a Windows one,
# so `-w /src/...` reaches docker as `C:/Program Files/Git/src/...` and the run
# fails with "the working directory is invalid". This disables that rewriting for
# the whole script; every path below is a path INSIDE a container.
export MSYS_NO_PATHCONV=1
export MSYS2_ARG_CONV_EXCL='*'

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO="$(cd "$HERE/../../.." && pwd)"
NAME=${HS_PGVIS_CONTAINER:-hs-pgvis-adapter}
IMAGE=${HS_PGVIS_IMAGE:-postgres:17.10}
# Debian trixie, matching `postgres:17.10`'s base (17.10-1.pgdg13+1), so the
# binary's glibc is the one the runtime has.
BUILDER=${HS_BUILDER_IMAGE:-rust:1-trixie}

echo "=== building the harness for the container's platform ==="
# A separate target directory: the host build is a different triple and
# clobbering it would mean the next host run silently links a Linux artefact.
docker run --rm \
  -v "$REPO":/src \
  -w /src/experiments/position-visibility/adapter \
  -e CARGO_TARGET_DIR=/src/experiments/position-visibility/adapter/target-linux \
  "$BUILDER" \
  cargo build --release

echo "=== starting $IMAGE ==="
docker rm -f "$NAME" >/dev/null 2>&1 || true
# `fsync=on` is the default and is asserted by the harness itself; it is the
# setting the whole experiment stands on. `max_connections=200` matches phase 2.
docker run -d --name "$NAME" \
  -e POSTGRES_PASSWORD=postgres \
  -v "$REPO/experiments/position-visibility/adapter/target-linux/release":/harness:ro \
  "$IMAGE" -c max_connections=200 >/dev/null

for _ in $(seq 1 60); do
  if docker exec "$NAME" pg_isready -U postgres >/dev/null 2>&1; then break; fi
  sleep 1
done
echo "server ready"

echo "=== measuring, over the unix socket ==="
# `host=/var/run/postgresql` is the socket directory, so nothing crosses the
# host boundary. Running as `postgres` gives peer authentication.
docker exec -u postgres \
  -e HS_PG_URL="postgres:///postgres?host=/var/run/postgresql" \
  -e HS_RUN_SECS="${HS_RUN_SECS:-20}" \
  -e HS_LEVELS="${HS_LEVELS:-1 8 32}" \
  -e HS_REPEATS="${HS_REPEATS:-3}" \
  -e HS_STALENESS_SAMPLES="${HS_STALENESS_SAMPLES:-9}" \
  -e HS_SETTLE_SECS="${HS_SETTLE_SECS:-8}" \
  -e HS_SEED_ROWS="${HS_SEED_ROWS:-10000}" \
  -e HS_WARMUP_SECS="${HS_WARMUP_SECS:-5}" \
  -e HS_TAG="${HS_TAG:--adapter-socket}" \
  "$NAME" /harness/position-visibility-adapter

echo "=== done; container '$NAME' left running ==="
