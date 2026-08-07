#!/usr/bin/env bash
# Host-side driver. Starts a throwaway postgres:17, copies this directory in,
# runs the three phases, copies the results back out, and removes the container.
#
# Nothing here touches the workspace: no crate, no Cargo.toml, no gate step.
# `cargo xtask ci` must stay runnable with no Docker at all.
#
#   bash docs/experiments/position-visibility/run.sh              # everything
#   bash docs/experiments/position-visibility/run.sh inversion    # part 1 only
#   bash docs/experiments/position-visibility/run.sh throughput   # part 2 only
#   bash docs/experiments/position-visibility/run.sh staleness    # arm C only
#   bash docs/experiments/position-visibility/run.sh keep         # leave it up
set -euo pipefail

# Git Bash on Windows rewrites arguments that look like POSIX paths into
# Windows paths before handing them to a native .exe, so `docker exec … bash
# /harness/x.sh` arrives inside the container as `C:/Program Files/harness/x.sh`.
# This is the switch that turns that off; it is a no-op everywhere else.
export MSYS_NO_PATHCONV=1

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NAME=${HS_PGVIS_CONTAINER:-hs-pgvis}
IMAGE=${HS_PGVIS_IMAGE:-postgres:17}
PHASE=${1:-all}

cleanup() {
    if [ "$PHASE" != keep ]; then
        docker rm -f "$NAME" > /dev/null 2>&1 || true
    fi
}

docker rm -f "$NAME" > /dev/null 2>&1 || true

# fsync stays at its default of ON. Do not "helpfully" add -c fsync=off here:
# it makes all three mechanisms look free, because what they cost is the length
# of the interval a lock is held across a durable commit.
docker run -d --name "$NAME" \
    -e POSTGRES_PASSWORD=postgres \
    -e POSTGRES_HOST_AUTH_METHOD=trust \
    "$IMAGE" \
    -c max_connections=200 \
    -c log_min_messages=warning > /dev/null

trap cleanup EXIT

# Wait for the server, without a fixed sleep.
for _ in $(seq 1 60); do
    if docker exec "$NAME" pg_isready -q -U postgres > /dev/null 2>&1; then
        break
    fi
    sleep 1
done
docker exec "$NAME" pg_isready -U postgres

# Named destinations, not `/harness/`. `docker cp dir container:/harness/`
# copies dir's *contents* when /harness does not yet exist, which silently
# flattens the first directory copied into the mount point.
docker exec "$NAME" bash -c 'mkdir -p /harness /results'
docker cp "$HERE/schema"    "$NAME:/harness/schema"
docker cp "$HERE/bench"     "$NAME:/harness/bench"
docker cp "$HERE/container" "$NAME:/harness/container"
docker exec "$NAME" bash -c 'chmod +x /harness/container/*.sh'

run_as_postgres() { docker exec -u postgres "$NAME" bash "$1"; }

docker exec "$NAME" bash -c 'chown -R postgres /results'
run_as_postgres /harness/container/setup.sh

case "$PHASE" in
    inversion)  run_as_postgres /harness/container/inversion.sh ;;
    throughput) run_as_postgres /harness/container/throughput.sh ;;
    staleness)
        run_as_postgres /harness/container/staleness.sh
        run_as_postgres /harness/container/staleness-pinned.sh
        ;;
    all|keep)
        run_as_postgres /harness/container/inversion.sh
        run_as_postgres /harness/container/throughput.sh
        # The single-client level again, longer. At c=1 this workload is one
        # fsync at a time; 30-second runs leave a baseline spread of 3.7x, which
        # is larger than any effect at that level.
        docker exec -u postgres \
            -e HS_LEVELS=1 -e HS_RUN_SECS=90 -e HS_WARMUP_SECS=10 -e HS_TAG=-c1long \
            "$NAME" bash /harness/container/throughput.sh
        run_as_postgres /harness/container/staleness.sh
        run_as_postgres /harness/container/staleness-pinned.sh
        ;;
    *) echo "unknown phase: $PHASE" >&2; exit 2 ;;
esac

mkdir -p "$HERE/results"
docker cp "$NAME:/results/." "$HERE/results/"
echo "results in $HERE/results"
