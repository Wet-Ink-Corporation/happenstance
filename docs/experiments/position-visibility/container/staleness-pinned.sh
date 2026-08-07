#!/usr/bin/env bash
# Arm C's real failure mode, measured rather than asserted.
#
# pg_snapshot_xmin is the oldest transaction id still in flight ANYWHERE IN THE
# CLUSTER, not in this store. So one long-lived write transaction - a migration,
# a batch job, an application that left a transaction open, a store in a
# different database entirely - pins arm C's frontier and every reader of the
# event log stops seeing new events for as long as it is held. The staleness
# figures under load say nothing about this; that is why it gets its own probe.
#
# The holder deliberately runs in the `baseline` database, so what is
# demonstrated is that the frontier is cluster-wide and not workload-local.
set -euo pipefail
. /harness/container/env.sh

HOLD_SECS=${HS_HOLD_SECS:-5}
OUT=$RESULTS/staleness_pinned.txt

cat > /tmp/holder.sql <<SQL
BEGIN;
INSERT INTO event (event_type, data, metadata, tags)
VALUES ('Holder', '\x00'::bytea, NULL, ARRAY['holder:1']);
SELECT pg_sleep($HOLD_SECS);
COMMIT;
SQL

{
    echo "holder: an open write transaction in database 'baseline', held ${HOLD_SECS}s"
    echo "probe : arm_c, unrelated database, unrelated tags"
    echo

    echo "--- control: no holder ---"
    psql -X -q -v ON_ERROR_STOP=1 -d arm_c -f "$HARNESS/container/staleness.sql" 2>&1 \
        | grep -oE 'STALENESS_MS [0-9.]+'

    echo
    echo "--- with holder ---"
    psql -X -q -v ON_ERROR_STOP=1 -d baseline -f /tmp/holder.sql > /dev/null 2>&1 &
    HOLDER=$!
    sleep 1   # let the holder's INSERT assign an xid before the probe commits
    psql -X -q -v ON_ERROR_STOP=1 -d arm_c -f "$HARNESS/container/staleness.sql" 2>&1 \
        | grep -oE 'STALENESS_MS [0-9.]+'
    wait $HOLDER || true
} | tee "$OUT"
