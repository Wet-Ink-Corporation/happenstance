#!/usr/bin/env bash
# Shared configuration for every script that runs inside the container.

export PGUSER=postgres
export PGHOST=/var/run/postgresql
export PSQL="psql -X -q -v ON_ERROR_STOP=1 -A -t"

HARNESS=/harness
RESULTS=/results

# arm key -> database name -> schema file
ARMS="baseline arm_a arm_b_const arm_b_tag arm_c"

schema_for() {
    case "$1" in
        baseline)    echo "$HARNESS/schema/baseline.sql" ;;
        arm_a)       echo "$HARNESS/schema/arm-a.sql" ;;
        arm_b_const) echo "$HARNESS/schema/arm-b-const.sql" ;;
        arm_b_tag)   echo "$HARNESS/schema/arm-b-tag.sql" ;;
        arm_c)       echo "$HARNESS/schema/arm-c.sql" ;;
        *)           echo "" ;;
    esac
}

# Rows present before every measurement, so no arm is measured against an empty
# table and all arms are measured against the same one.
SEED_ROWS=${HS_SEED_ROWS:-10000}

# Throughput run shape. Overridable so the single-client level can be re-measured
# with a longer run without editing the harness: at c=1 this workload is one
# fsync at a time and the virtual disk's jitter is larger than every effect,
# which 30 seconds does not average out.
#
#   HS_LEVELS=1 HS_RUN_SECS=90 HS_TAG=-c1long bash throughput.sh
LEVELS=${HS_LEVELS:-"1 8 32 64"}
WARMUP_SECS=${HS_WARMUP_SECS:-5}
RUN_SECS=${HS_RUN_SECS:-30}

# Suffix for this pass's output files, so a re-measurement does not overwrite
# the pass it is being compared with.
TAG=${HS_TAG:-}
