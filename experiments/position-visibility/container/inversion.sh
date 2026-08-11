#!/usr/bin/env bash
# Part 1 - the inversion detector.
#
# Three held sessions, driven deterministically. There is no sleep that
# ESTABLISHES the interleaving: every step is followed by a barrier that waits
# for psql to close a marker file, which it can only do once the preceding
# statement has finished. The 50ms poll inside the barrier is how the shell
# notices the file, not how the schedule is arranged. The one deliberate wait is
# lock_timeout, which is what turns "session B blocks forever" into a scored
# result instead of a hang.
#
#   A: BEGIN, append, do NOT commit
#   B: BEGIN, append, COMMIT          (may block on A - that is a result)
#   C: fresh READ COMMITTED snapshot; record head H and the visible position set
#   A: COMMIT
#   C: fresh snapshot again; anything at a position <= H that was not in the
#      first observation is an ES-10 violation.
set -uo pipefail
. /harness/container/env.sh

WORK=/tmp/inv
LOCK_TIMEOUT=3000          # ms. B is expected to hit this under arms A and B-const.
BARRIER_TICKS=600          # 600 * 0.05s = 30s, comfortably above lock_timeout
SEED=5
MARK=0

wait_for_file() {
    local f=$1 i=0
    while [ ! -e "$f" ]; do
        i=$((i + 1))
        if [ "$i" -gt "$BARRIER_TICKS" ]; then
            echo "BARRIER-TIMEOUT waiting for $f" >&2
            return 1
        fi
        sleep 0.05
    done
}

open_session() {  # name db fd
    local name=$1 db=$2 fd=$3
    mkfifo "$WORK/$name.in"
    psql -X -q -A -t -v ON_ERROR_STOP=0 -d "$db" \
        < "$WORK/$name.in" > "$WORK/$name.out" 2>&1 &
    eval "exec $fd>\"$WORK/$name.in\""
}

close_session() {  # name fd
    local name=$1 fd=$2
    eval "exec $fd>&-"
}

# Send SQL and block until psql has finished executing it. The marker is a file
# psql closes with \o, so its existence proves the statement returned; stdout
# buffering cannot forge it.
send() {  # name fd sql...
    local name=$1 fd=$2
    shift 2
    MARK=$((MARK + 1))
    local mf="$WORK/$name.mark.$MARK"
    {
        printf '%s\n' "$*"
        printf '\\o %s\n' "$mf"
        printf "select 'done';\n"
        printf '\\o\n'
    } >&"$fd"
    wait_for_file "$mf"
}

# As send, but the statement's result lands in $out.
send_capture() {  # name fd out sql...
    local name=$1 fd=$2 out=$3
    shift 3
    MARK=$((MARK + 1))
    local mf="$WORK/$name.mark.$MARK"
    {
        printf '\\o %s\n' "$out"
        printf '%s\n' "$*"
        printf '\\o\n'
        printf '\\o %s\n' "$mf"
        printf "select 'done';\n"
        printf '\\o\n'
    } >&"$fd"
    wait_for_file "$mf"
}

run_case() {  # db tag_a tag_b label
    local db=$1 tag_a=$2 tag_b=$3 label=$4

    rm -rf "$WORK"; mkdir -p "$WORK"
    MARK=0

    psql -X -q -v ON_ERROR_STOP=1 -d "$db" \
        -c "select hs_reset()" -c "select hs_seed($SEED)" > /dev/null

    open_session a "$db" 3
    open_session b "$db" 4
    open_session c "$db" 5

    send a 3 "SET lock_timeout = '${LOCK_TIMEOUT}ms'; SET statement_timeout = '${LOCK_TIMEOUT}ms';"
    send b 4 "SET lock_timeout = '${LOCK_TIMEOUT}ms'; SET statement_timeout = '${LOCK_TIMEOUT}ms';"

    send a 3 "BEGIN;"
    send_capture a 3 "$WORK/a_pos.txt" \
        "SELECT hs_append('A', '\\x00'::bytea, NULL, ARRAY['$tag_a']);"

    send b 4 "BEGIN;"
    send_capture b 4 "$WORK/b_pos.txt" \
        "SELECT hs_append('B', '\\x00'::bytea, NULL, ARRAY['$tag_b']);"
    send b 4 "COMMIT;"

    send c 5 "BEGIN ISOLATION LEVEL READ COMMITTED;"
    send_capture c 5 "$WORK/h1.txt" "SELECT hs_head();"
    send_capture c 5 "$WORK/obs1.txt" \
        "SELECT coalesce(string_agg(pos::text, ',' ORDER BY pos), '') FROM hs_read();"
    send c 5 "COMMIT;"

    send a 3 "COMMIT;"

    send c 5 "BEGIN ISOLATION LEVEL READ COMMITTED;"
    send_capture c 5 "$WORK/h2.txt" "SELECT hs_head();"
    send_capture c 5 "$WORK/obs2.txt" \
        "SELECT coalesce(string_agg(pos::text, ',' ORDER BY pos), '') FROM hs_read();"
    send c 5 "COMMIT;"

    close_session a 3
    close_session b 4
    close_session c 5
    wait 2>/dev/null

    local a_pos b_pos h1 h2 obs1 obs2 blocked inv verdict
    a_pos=$(cat "$WORK/a_pos.txt" 2>/dev/null | tr -d '\r\n')
    b_pos=$(cat "$WORK/b_pos.txt" 2>/dev/null | tr -d '\r\n')
    h1=$(cat "$WORK/h1.txt" 2>/dev/null | tr -d '\r\n')
    h2=$(cat "$WORK/h2.txt" 2>/dev/null | tr -d '\r\n')
    obs1=$(cat "$WORK/obs1.txt" 2>/dev/null | tr -d '\r\n')
    obs2=$(cat "$WORK/obs2.txt" 2>/dev/null | tr -d '\r\n')

    blocked=no
    if grep -qiE 'canceling statement due to (lock|statement) timeout' "$WORK/b.out"; then
        blocked=yes
    fi

    inv=$(awk -v h="$h1" -v o1="$obs1" -v o2="$obs2" 'BEGIN {
        n = split(o1, a, ","); for (i = 1; i <= n; i++) seen[a[i] + 0] = 1
        m = split(o2, b, ","); bad = ""
        for (i = 1; i <= m; i++)
            if (b[i] + 0 <= h + 0 && !((b[i] + 0) in seen))
                bad = bad (bad == "" ? "" : " ") (b[i] + 0)
        print bad
    }')

    if [ "$blocked" = yes ]; then
        verdict="pass-by-serialisation"
    elif [ -n "$inv" ]; then
        verdict="INVERSION"
    else
        verdict="pass"
    fi

    {
        echo "case      : $label"
        echo "database  : $db"
        echo "tags      : A=[$tag_a] B=[$tag_b]"
        echo "A position: ${a_pos:-<none: append failed>}"
        echo "B position: ${b_pos:-<none: append failed>}"
        echo "B blocked : $blocked"
        echo "head H    : $h1   (observation 1, A still uncommitted)"
        echo "obs 1     : $obs1"
        echo "head after: $h2"
        echo "obs 2     : $obs2"
        echo "appeared <= H and not in obs 1: ${inv:-<none>}"
        echo "verdict   : $verdict"
        echo
        echo "--- session A transcript ---"; cat "$WORK/a.out"
        echo "--- session B transcript ---"; cat "$WORK/b.out"
        echo "--- session C transcript ---"; cat "$WORK/c.out"
    } > "$RESULTS/inversion_${label}.txt"

    printf '%-28s %-24s %s\n' "$label" "$verdict" "B blocked=$blocked A=$a_pos B=$b_pos H=$h1 late=${inv:-none}" \
        | tee -a "$RESULTS/inversion_summary.txt"
}

rm -f "$RESULTS/inversion_summary.txt"

# Every arm is run with both a shared-key writer pair and a disjoint-key pair.
# Only arm B-tag is expected to distinguish them; running all ten cases is what
# makes that claim a measurement rather than an expectation.
for arm in $ARMS; do
    run_case "$arm" "course:shared"  "course:shared" "${arm}__shared_tag"
    run_case "$arm" "course:alpha"   "course:beta"   "${arm}__disjoint_tags"
done

echo
echo "=== inversion summary ==="
cat "$RESULTS/inversion_summary.txt"
