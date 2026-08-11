#!/usr/bin/env bash
# Part 2 - cost. One pgbench script, five arms, four concurrency levels.
#
# The design is PAIRED, and it is paired because the obvious design failed. The
# first pass measured all four levels of the baseline, then all four of each arm,
# then re-ran the baseline last to bound drift - and the drift was 2.7x at one
# client and 3.0x at 64 (results/discarded-sequential/throughput.csv). On Docker
# Desktop's virtual disk, an instance that has been fsyncing for twenty minutes
# is a different instance from the one that started. A drift of 3x is larger
# than two of the three effects being measured, so every ratio computed against
# a session-wide baseline would have been noise wearing a decimal point.
#
# So: at each concurrency level the baseline is re-measured BETWEEN every pair of
# arms, and each arm's ratio is taken against the mean of the baseline
# immediately before it and the baseline immediately after it. The spread among
# the five baselines at a level is reported as the drift the ratio is being
# asked to survive.
set -euo pipefail
. /harness/container/env.sh

BENCH=$HARNESS/bench/append.sql
CSV=$RESULTS/throughput${TAG}.csv
SEQ=0

reset_and_seed() {  # db
    psql -X -q -v ON_ERROR_STOP=1 -d "$1" \
        -c "select hs_reset()" -c "select hs_seed($SEED_ROWS)" > /dev/null
}

run_one() {  # db clients label
    local db=$1 c=$2 label=$3
    local j=$c
    [ "$j" -gt 4 ] && j=4
    SEQ=$((SEQ + 1))

    # Warm up on a table of the intended size, then put the table back to the
    # intended size, so every measured run starts from exactly $SEED_ROWS rows
    # rather than from however many rows the warm-up managed to add - which
    # would charge the fastest arms for being fast. The CHECKPOINT equalises the
    # one piece of server state that otherwise carries over between runs.
    reset_and_seed "$db"
    pgbench -n -M prepared -f "$BENCH" -c "$c" -j "$j" -T "$WARMUP_SECS" -d "$db" \
        > /dev/null 2>&1
    reset_and_seed "$db"
    psql -X -q -d "$db" -c "CHECKPOINT" > /dev/null

    rm -f /tmp/plog.*
    pgbench -n -M prepared -f "$BENCH" -c "$c" -j "$j" -T "$RUN_SECS" \
        -l --log-prefix=/tmp/plog -d "$db" \
        > "$RESULTS/bench${TAG}_${label}_c${c}_s${SEQ}.txt" 2>&1

    local tps p99 mean n
    tps=$(grep -E '^tps = ' "$RESULTS/bench${TAG}_${label}_c${c}_s${SEQ}.txt" | head -1 | awk '{print $3}')
    mean=$(grep -E '^latency average' "$RESULTS/bench${TAG}_${label}_c${c}_s${SEQ}.txt" | head -1 | awk '{print $4}')
    # pgbench -l writes "client_id transaction_no time script_no time_epoch time_us";
    # field 3 is the transaction's elapsed time in microseconds.
    p99=$(cat /tmp/plog.* 2>/dev/null | awk '{print $3}' | sort -n | awk '
        {v[NR] = $1}
        END {
            if (NR == 0) { print "NA"; exit }
            i = int(NR * 0.99); if (i < 1) i = 1
            printf "%.3f", v[i] / 1000
        }')
    n=$(cat /tmp/plog.* 2>/dev/null | wc -l)

    echo "$SEQ,$label,$c,$tps,$mean,$p99,$n" >> "$CSV"
    printf '%3s %-16s c=%-3s tps=%-12s mean_ms=%-9s p99_ms=%-10s n=%s\n' \
        "$SEQ" "$label" "$c" "$tps" "$mean" "$p99" "$n"
}

echo "seq,arm,clients,tps,mean_latency_ms,p99_latency_ms,transactions" > "$CSV"

for c in $LEVELS; do
    run_one baseline    "$c" baseline
    run_one arm_a       "$c" arm_a
    run_one baseline    "$c" baseline
    run_one arm_b_const "$c" arm_b_const
    run_one baseline    "$c" baseline
    run_one arm_b_tag   "$c" arm_b_tag
    run_one baseline    "$c" baseline
    run_one arm_c       "$c" arm_c
    run_one baseline    "$c" baseline
done

echo
echo "=== throughput.csv ==="
cat "$CSV"

# Ratios, paired against the bracketing baselines.
awk -F, 'NR > 1 {
    seq[NR] = $1; arm[NR] = $2; c[NR] = $3; tps[NR] = $4; p99[NR] = $6; n = NR
}
END {
    print "clients,arm,tps,tps_ratio_to_baseline,p99_ms,p99_ratio_to_baseline,bracketing_baseline_tps,baseline_drift_at_this_level"
    # per-level baseline spread
    for (i = 2; i <= n; i++)
        if (arm[i] == "baseline") {
            k = c[i]
            if (!(k in lo) || tps[i] + 0 < lo[k]) lo[k] = tps[i] + 0
            if (!(k in hi) || tps[i] + 0 > hi[k]) hi[k] = tps[i] + 0
        }
    for (i = 2; i <= n; i++) {
        if (arm[i] == "baseline") continue
        # nearest baseline before and after, same level
        bb = 0; ba = 0
        for (j = i - 1; j >= 2; j--) if (arm[j] == "baseline" && c[j] == c[i]) { bb = tps[j] + 0; bbp = p99[j] + 0; break }
        for (j = i + 1; j <= n; j++) if (arm[j] == "baseline" && c[j] == c[i]) { ba = tps[j] + 0; bap = p99[j] + 0; break }
        b  = (bb + ba) / 2
        bp = (bbp + bap) / 2
        printf "%s,%s,%.1f,%.3f,%.2f,%.2f,%.1f,%.2fx\n", c[i], arm[i], tps[i], tps[i] / b, p99[i], p99[i] / bp, b, hi[c[i]] / lo[c[i]]
    }
}' "$CSV" > "$RESULTS/ratios${TAG}.csv"

echo
echo "=== ratios.csv ==="
cat "$RESULTS/ratios${TAG}.csv"
