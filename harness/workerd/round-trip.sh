#!/usr/bin/env bash
# The SQLite round trip (#130; tests/sqlite_round_trip/) against one running
# harness Worker, strict:
#
#   HARNESS_TOKEN=... ./round-trip.sh <label> <base-url> <run-prefix>
#
# The live tests are `#[ignore]`d so a machine with no Worker stays green; this
# runs exactly those, and fails unless every one the binary lists executed and
# passed, none was left ignored, and the list is at least FLOOR long (a floor
# rather than an equality, for live-neon's reason: the commit that adds a test
# must stay green). The CI job's `workerd` steps call it twice: against
# `wrangler dev` (local workerd) and against the deployed Worker.
set -euo pipefail

FLOOR=4

label=${1:?usage: round-trip.sh <label> <base-url> <run-prefix>}
url=${2:?usage: round-trip.sh <label> <base-url> <run-prefix>}
run=${3:?usage: round-trip.sh <label> <base-url> <run-prefix>}
: "${HARNESS_TOKEN:?HARNESS_TOKEN is required}"

target=(cargo test --locked -p happenstance-workerd-harness --test sqlite_round_trip)

"${target[@]}" -- --ignored --list | tee "round-trip-$label-list.txt"
listed=$(grep -c ': test$' "round-trip-$label-list.txt" || true)

status=0
WORKERD_HARNESS_URL=$url WORKERD_HARNESS_RUN=$run \
  "${target[@]}" -- --ignored --test-threads=1 | tee "round-trip-$label.txt" || status=$?

result=$(grep -E '^test result:' "round-trip-$label.txt" | tail -1 || true)
count() { grep -oE "[0-9]+ $1" <<<"$result" | grep -oE '[0-9]+' || echo 0; }
passed=$(count passed)
failed=$(count failed)
ignored=$(count ignored)
echo "round trip ($label): listed $listed; passed $passed, failed $failed, left ignored $ignored"

if [ "$status" -ne 0 ] || [ "$listed" -lt "$FLOOR" ] || [ "$passed" -ne "$listed" ] \
  || [ "$failed" -ne 0 ] || [ "$ignored" -ne 0 ]; then
  echo "::error::the $label round trip did not execute and pass every listed test (floor $FLOOR)"
  exit 1
fi
