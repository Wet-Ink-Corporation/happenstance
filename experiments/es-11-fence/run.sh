#!/usr/bin/env bash
#
# Extracts the ES-11 fence sweep's rows from `live-neon` job logs and tallies
# them under the decision rule README.md pre-registered, as amended before the
# first counted run (README.md, "Amendment, before the first counted run").
#
# It is NOT a gate step and must never become one (CF-34). The sweep itself runs
# inside CI's `live-neon` job as the `#[ignore]`d test `es11_fence_sweep`; this
# script only reads what that job printed.
#
#   ./run.sh fetch <run-id> <attempt>   download one attempt's log with `gh`, then extract
#   ./run.sh extract <job-log>...       extract rows from saved job logs
#   ./run.sh tally                      write results/tally.md and print the verdict
#
# Raw files, one pair per job attempt:
#   results/raw/<run>-<attempt>.jsonl   every `ES11-SWEEP` row, exactly as printed
#   results/raw/<run>-<attempt>.v1.txt  the two racing rules' result lines (V1)
#
# `RUN_SH_RAW` overrides the raw directory and `RUN_SH_TALLY` the tally file, so
# the script can be exercised on synthetic logs without touching results/.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
raw="${RUN_SH_RAW:-$here/results/raw}"
tally="${RUN_SH_TALLY:-$here/results/tally.md}"

# Pre-registered in README.md, as amended before the first counted run.
# Changing any of these after a counted row exists breaks the rule's claim to
# have been written first.
readonly CELL_TRIALS=250
readonly ROWS_PER_ATTEMPT=$((CELL_TRIALS * 4))
readonly MAX_ERROR_ROWS=25
readonly MAX_ANCHOR_ROWS=50
readonly MAX_ATTEMPTS=3
readonly LIVE_BASELINE_REDS=3
# The row layout the amended rule judges. Rows without it are the pilot's.
readonly ROW_SCHEMA=2
# The pilot: the first CI attempt, on 81eab3b, run under the pre-amendment rule.
# Excluded from the tally whatever it shows, by name as well as by schema.
readonly PILOT_ATTEMPTS=("37591126575-1")
readonly RACING_RULES=(read_result_is_stable_under_concurrent_append query_items_share_one_snapshot)

die() {
  echo "run.sh: $*" >&2
  exit 1
}

need() {
  command -v "$1" >/dev/null 2>&1 || die "needs \`$1\` on PATH"
}

# One job log in, one raw pair out per attempt it contains. The run and attempt
# come from the rows themselves, never from the file name.
extract() {
  local log="$1" rows meta run attempt
  [ -f "$log" ] || die "no such log: $log"
  rows="$(mktemp)"
  # GitHub prefixes every log line with the job, the step and a timestamp, and
  # may end it with a carriage return. The row is everything from the marker.
  tr -d '\r' <"$log" | grep -oE 'ES11-SWEEP \{.*\}$' | sed -E 's/^ES11-SWEEP //' >"$rows" || true
  meta="$(tr -d '\r' <"$log" | grep -oE 'ES11-SWEEP-META \{.*\}$' | sed -E 's/^ES11-SWEEP-META //' | head -1 || true)"
  if [ ! -s "$rows" ]; then
    rm -f "$rows"
    die "no ES11-SWEEP rows in $log; did the job run with --show-output?"
  fi
  mkdir -p "$raw"
  while IFS=$'\t' read -r run attempt; do
    local out="$raw/$run-$attempt"
    jq -c --arg run "$run" --arg attempt "$attempt" \
      'select(.run == $run and .attempt == $attempt)' "$rows" >"$out.jsonl"
    {
      [ -n "$meta" ] && echo "meta: $meta"
      tr -d '\r' <"$log" \
        | grep -oE '(read_result_is_stable_under_concurrent_append|query_items_share_one_snapshot) \.\.\. (ok|FAILED)' \
        || true
    } >"$out.v1.txt"
    echo "extracted $(wc -l <"$out.jsonl") rows -> $out.jsonl"
  done < <(jq -r '[.run, .attempt] | @tsv' "$rows" | sort -u)
  rm -f "$rows"
}

fetch() {
  local run="$1" attempt="$2" log
  need gh
  log="$(mktemp)"
  gh run view "$run" --attempt "$attempt" --log >"$log"
  extract "$log"
  rm -f "$log"
}

# The Clopper-Pearson 95% interval for k successes in n, by bisection on the
# binomial tail in log space (awk has no beta quantile).
clopper_pearson() {
  awk -v k="$1" -v n="$2" '
    function logpmf(i, p) {
      return lg[n] - lg[i] - lg[n - i] + i * log(p) + (n - i) * log(1 - p)
    }
    # P(X >= k | p), or P(X <= k | p) when upper is set.
    function tail(p, upper,   i, m, s, x) {
      m = -1e300
      for (i = 0; i <= n; i++) if ((upper && i <= k) || (!upper && i >= k)) {
        x = logpmf(i, p); if (x > m) m = x
      }
      s = 0
      for (i = 0; i <= n; i++) if ((upper && i <= k) || (!upper && i >= k)) s += exp(logpmf(i, p) - m)
      return exp(m) * s
    }
    function solve(upper,   lo, hi, mid, t, it) {
      lo = 1e-12; hi = 1 - 1e-12
      for (it = 0; it < 80; it++) {
        mid = (lo + hi) / 2; t = tail(mid, upper)
        # The lower bound solves P(X >= k) = 0.025, rising with p; the upper
        # solves P(X <= k) = 0.025, falling with p.
        if ((!upper && t < 0.025) || (upper && t > 0.025)) lo = mid; else hi = mid
      }
      return mid
    }
    BEGIN {
      lg[0] = 0; for (i = 1; i <= n; i++) lg[i] = lg[i - 1] + log(i)
      if (n == 0) { print "n/a"; exit }
      low = (k == 0) ? 0 : solve(0)
      high = (k == n) ? 1 : solve(1)
      printf "%.4f%% to %.4f%%", 100 * low, 100 * high
    }'
}

# The raw files' attempt keys (`<run>-<attempt>`), in attempt order: run ID,
# then attempt number, both numerically. Glob order would put attempt 10
# before attempt 2.
attempts_in_order() {
  (
    shopt -s nullglob
    cd "$raw" 2>/dev/null || exit 0
    for f in *.jsonl; do echo "${f%.jsonl}"; done
  ) | sort -t- -k1,1n -k2,2n
}

is_pilot() {
  local key="$1" pilot
  for pilot in "${PILOT_ATTEMPTS[@]}"; do
    [ "$key" = "$pilot" ] && return 0
  done
  return 1
}

# Why an attempt's V1 lines do not number one per racing rule, or nothing.
v1_problem() {
  local v="$1" rule lines
  [ -f "$v" ] || { echo "no V1 file"; return; }
  for rule in "${RACING_RULES[@]}"; do
    lines="$(grep -cE "(^|[^a-z_])$rule \.\.\. (ok|FAILED)\$" "$v" || true)"
    if [ "$lines" -ne 1 ]; then
      echo "$lines result line(s) for \`$rule\` rather than 1"
      return
    fi
  done
}

tally() {
  need jq
  local keys=()
  mapfile -t keys < <(attempts_in_order)
  [ "${#keys[@]}" -gt 0 ] || die "no raw rows under $raw; run extract or fetch first"

  local pooled_files=() excluded_notes="" key f n errors anchors schema_ok problem
  for key in "${keys[@]}"; do
    f="$raw/$key.jsonl"
    n="$(wc -l <"$f" | tr -d ' ')"
    schema_ok="$(jq -s --argjson s "$ROW_SCHEMA" 'length > 0 and all(.[]; .schema == $s)' "$f")"
    errors="$(jq -s '[.[] | select(.outcome == "error")] | length' "$f")"
    anchors="$(jq -s '[.[] | select(.outcome == "anchor")] | length' "$f")"
    problem="$(v1_problem "$raw/$key.v1.txt")"
    if is_pilot "$key" || [ "$schema_ok" != "true" ]; then
      excluded_notes+="* \`$key\`: pilot, run under the pre-amendment rule (row schema other than $ROW_SCHEMA); excluded whatever it shows"$'\n'
    elif [ "$n" -ne "$ROWS_PER_ATTEMPT" ]; then
      excluded_notes+="* \`$key\`: void, $n rows rather than $ROWS_PER_ATTEMPT"$'\n'
    elif [ "$errors" -gt "$MAX_ERROR_ROWS" ]; then
      excluded_notes+="* \`$key\`: void, $errors error rows (more than $MAX_ERROR_ROWS)"$'\n'
    elif [ "$anchors" -gt "$MAX_ANCHOR_ROWS" ]; then
      excluded_notes+="* \`$key\`: void, $anchors anchor rows (more than $MAX_ANCHOR_ROWS, 5%)"$'\n'
    elif [ -n "$problem" ]; then
      excluded_notes+="* \`$key\`: void, V1 incomplete: $problem"$'\n'
    elif [ "${#pooled_files[@]}" -ge "$MAX_ATTEMPTS" ]; then
      excluded_notes+="* \`$key\`: valid but beyond the first $MAX_ATTEMPTS valid attempts; not pooled"$'\n'
    else
      pooled_files+=("$f")
    fi
  done

  local pooled
  pooled="$(mktemp)"
  if [ "${#pooled_files[@]}" -gt 0 ]; then
    cat "${pooled_files[@]}" >"$pooled"
  fi

  # A red counts only with a complete `before`; `judge` already makes every
  # other red an anchor, and this filter says so again where it is counted.
  # Classes, for red rows only. C3 first: it is the one the fence leaves
  # possible, and `>=` puts a tie at microsecond resolution there, so a true
  # C3 is never filed as a spike defect.
  local defs='
    def red: .outcome == "red" and .before_complete == true;
    def class:
      if (.t_append_dispatch_us == null or .t_read_answer_us == null
          or .t_append_send_us == null or .t_read_send_us == null) then "unclassified"
      elif .t_append_dispatch_us >= .t_read_answer_us then "C3"
      elif .t_append_send_us < .t_read_send_us then "C1"
      else "C2" end;
    def judged: .outcome == "pass" or red;'

  local counts
  counts="$(jq -rs "$defs"'
    group_by([.arm, .shape])[]
    | [ .[0].arm, .[0].shape,
        ([.[] | select(.outcome == "pass")] | length),
        ([.[] | select(red)] | length),
        ([.[] | select(.outcome == "anchor")] | length),
        ([.[] | select(.outcome == "error")] | length),
        ([.[] | select(red and .reason == "late_in_drained")] | length),
        ([.[] | select(red and .reason == "drained_ne_before")] | length),
        ([.[] | select(red) | select(class == "C1")] | length),
        ([.[] | select(red) | select(class == "C2")] | length),
        ([.[] | select(red) | select(class == "C3")] | length),
        ([.[] | select(red) | select(class == "unclassified")] | length) ]
    | @tsv' "$pooled")"

  local base_n base_red base_anchor fence_n fence_red fence_anchor fence_c3 fence_c12
  base_n="$(jq -s "$defs"'[.[] | select(.arm == "baseline" and judged)] | length' "$pooled")"
  base_red="$(jq -s "$defs"'[.[] | select(.arm == "baseline" and red)] | length' "$pooled")"
  base_anchor="$(jq -s '[.[] | select(.arm == "baseline" and .outcome == "anchor")] | length' "$pooled")"
  fence_n="$(jq -s "$defs"'[.[] | select(.arm == "fence" and judged)] | length' "$pooled")"
  fence_red="$(jq -s "$defs"'[.[] | select(.arm == "fence" and red)] | length' "$pooled")"
  fence_anchor="$(jq -s '[.[] | select(.arm == "fence" and .outcome == "anchor")] | length' "$pooled")"
  fence_c3="$(jq -s "$defs"'[.[] | select(.arm == "fence" and red) | select(class == "C3")] | length' "$pooled")"
  fence_c12=$((fence_red - fence_c3))

  # Every pooled attempt carries exactly one line per racing rule; checked
  # above, so `v1_seen` is 2 per pooled attempt and never 0 beside a verdict.
  local v1_failed=0 v1_seen=0 v
  for f in "${pooled_files[@]}"; do
    v="${f%.jsonl}.v1.txt"
    v1_seen=$((v1_seen + $(grep -cE '\.\.\. (ok|FAILED)$' "$v" || true)))
    v1_failed=$((v1_failed + $(grep -c 'FAILED$' "$v" || true)))
  done

  local verdict
  if [ "${#pooled_files[@]}" -eq 0 ]; then
    verdict="NOT YET DECIDABLE: no valid attempt to pool. Run an attempt."
  elif [ "$fence_red" -gt 0 ] && [ "$fence_c3" -gt 0 ]; then
    verdict="THE FENCE FAILS: $fence_c3 fence-arm red(s) of class C3. ADR-0087's falsifier 1 fires. Stop and ask the owner."
  elif [ "$fence_red" -gt 0 ]; then
    verdict="SPIKE DEFECT: $fence_c12 fence-arm red(s) of class C1/C2/unclassified, so the fence was not engaged. Check the trace, fix, re-run, and record the defect in README.md."
  elif [ "$base_red" -lt "$LIVE_BASELINE_REDS" ] && [ "${#pooled_files[@]}" -ge "$MAX_ATTEMPTS" ]; then
    verdict="INCONCLUSIVE: $base_red baseline red(s) in ${#pooled_files[@]} attempts; the race was not reproduced at this N. Report to the owner; claim nothing."
  elif [ "$base_red" -lt "$LIVE_BASELINE_REDS" ]; then
    verdict="NOT YET DECIDABLE: $base_red baseline red(s) in ${#pooled_files[@]} valid attempt(s); fewer than $LIVE_BASELINE_REDS. Run another attempt (at most $MAX_ATTEMPTS pooled)."
  elif [ "$v1_failed" -gt 0 ]; then
    verdict="NOT 'WORKS': the sweep is clean but V1 has $v1_failed red result(s) for the two racing rules. Read them before anything else."
  else
    verdict="THE FENCE WORKS under the pre-registered rule: instrument live ($base_red baseline reds), 0 fence reds in $fence_n judged fence trials, V1 green ($v1_seen result lines over ${#pooled_files[@]} attempt(s))."
  fi

  {
    echo "# ES-11 fence sweep: tally"
    echo
    echo "Generated by \`run.sh tally\` from \`results/raw/\`. Do not edit by hand."
    echo
    echo "Valid attempts pooled: ${#pooled_files[@]} (the first $MAX_ATTEMPTS valid attempts in attempt order)."
    for f in "${pooled_files[@]}"; do echo "* \`$(basename "$f" .jsonl)\`"; done
    if [ -n "$excluded_notes" ]; then
      echo
      echo "Excluded attempts:"
      echo
      printf '%s' "$excluded_notes"
    fi
    echo
    echo "| arm | shape | pass | red | anchor | error | red late_in_drained | red drained_ne_before | red C1 | red C2 | red C3 | red unclassified |"
    echo "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"
    if [ -n "$counts" ]; then
      while IFS=$'\t' read -r arm shape pass red anchor err late ne c1 c2 c3 un; do
        echo "| $arm | $shape | $pass | $red | $anchor | $err | $late | $ne | $c1 | $c2 | $c3 | $un |"
      done <<<"$counts"
    fi
    echo
    echo "* Baseline: $base_red red in $base_n judged trials ($base_anchor anchor, excluded); Clopper-Pearson 95%: $(clopper_pearson "$base_red" "$base_n")."
    if [ "$fence_n" -gt 0 ]; then
      echo "* Fence: $fence_red red in $fence_n judged trials ($fence_anchor anchor, excluded); rule-of-three 95% upper bound: $(awk -v n="$fence_n" 'BEGIN { printf "%.2f%%", 300 / n }') (meaningful only at 0 reds)."
    fi
    echo "* V1: $v1_seen result line(s) for the two racing rules, $v1_failed FAILED."
    echo
    echo "**Verdict:** $verdict"
  } >"$tally"

  rm -f "$pooled"
  cat "$tally"
}

case "${1:-tally}" in
  fetch)
    [ "$#" -eq 3 ] || die "usage: run.sh fetch <run-id> <attempt>"
    need jq
    fetch "$2" "$3"
    ;;
  extract)
    [ "$#" -ge 2 ] || die "usage: run.sh extract <job-log>..."
    need jq
    shift
    for log in "$@"; do extract "$log"; done
    ;;
  tally)
    tally
    ;;
  *)
    die "unknown command \`$1\`; see the header of this file"
    ;;
esac
