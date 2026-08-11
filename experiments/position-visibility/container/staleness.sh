#!/usr/bin/env bash
# Arm C's staleness: time from COMMIT to admitted-by-a-reader, measured under
# the same concurrency levels the throughput runs use. Under load the frontier
# lags by roughly the lifetime of the longest concurrent write transaction, so
# this number is a property of the workload as much as of the mechanism.
set -euo pipefail
. /harness/container/env.sh

SAMPLES=20
LOAD_SECS=40
CSV=$RESULTS/staleness.csv

echo "clients,samples,min_ms,median_ms,p95_ms,max_ms" > "$CSV"

psql -X -q -v ON_ERROR_STOP=1 -d arm_c \
    -c "select hs_reset()" -c "select hs_seed($SEED_ROWS)" > /dev/null

for c in $LEVELS; do
    j=$c
    [ "$j" -gt 4 ] && j=4

    pgbench -n -M prepared -f "$HARNESS/bench/append.sql" -c "$c" -j "$j" \
        -T "$LOAD_SECS" -d arm_c > "$RESULTS/staleness_load_c${c}.txt" 2>&1 &
    LOAD_PID=$!
    sleep 5   # let the load reach steady state before sampling

    : > /tmp/staleness_c${c}.raw
    for i in $(seq 1 $SAMPLES); do
        psql -X -q -v ON_ERROR_STOP=1 -d arm_c -f "$HARNESS/container/staleness.sql" 2>&1 \
            | grep -oE 'STALENESS_MS [0-9.]+' | awk '{print $2}' >> /tmp/staleness_c${c}.raw
    done

    wait $LOAD_PID || true
    cp /tmp/staleness_c${c}.raw "$RESULTS/staleness_samples_c${c}.txt"

    sort -n /tmp/staleness_c${c}.raw | awk -v c="$c" '
        {v[NR] = $1}
        END {
            if (NR == 0) { printf "%s,0,NA,NA,NA,NA\n", c; exit }
            med = (NR % 2) ? v[int(NR/2) + 1] : (v[NR/2] + v[NR/2 + 1]) / 2
            p95 = v[int(NR * 0.95) < 1 ? 1 : int(NR * 0.95)]
            printf "%s,%d,%.3f,%.3f,%.3f,%.3f\n", c, NR, v[1], med, p95, v[NR]
        }' >> "$CSV"

    tail -1 "$CSV"
done

echo
echo "=== staleness.csv ==="
cat "$CSV"
