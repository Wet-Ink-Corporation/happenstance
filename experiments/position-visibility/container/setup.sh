#!/usr/bin/env bash
# Creates one database per arm, applies its schema, and records the settings
# under which every subsequent number was produced.
set -euo pipefail
. /harness/container/env.sh

mkdir -p "$RESULTS"

{
    echo "=== server ==="
    $PSQL -c "select version()"
    echo
    echo "=== settings that decide whether these numbers mean anything ==="
    $PSQL -c "select name || ' = ' || setting || coalesce(' ' || unit, '')
              from pg_settings
              where name in ('fsync','synchronous_commit','wal_sync_method',
                             'full_page_writes','shared_buffers','wal_buffers',
                             'max_connections','max_wal_size','checkpoint_timeout',
                             'wal_level','wal_compression','commit_delay',
                             'data_checksums','server_version')
              order by name"
    echo
    echo "=== container ==="
    echo "nproc = $(nproc)"
    grep -E '^(MemTotal|MemAvailable)' /proc/meminfo
    echo "cgroup cpu.max = $(cat /sys/fs/cgroup/cpu.max 2>/dev/null || echo 'n/a')"
    echo "cgroup memory.max = $(cat /sys/fs/cgroup/memory.max 2>/dev/null || echo 'n/a')"
    echo
    echo "=== pgbench ==="
    pgbench --version
} > "$RESULTS/environment.txt" 2>&1

cat "$RESULTS/environment.txt"

# fsync=off makes all three mechanisms look free. Refuse to produce numbers
# under it rather than produce meaningless ones.
FSYNC=$($PSQL -c "show fsync")
if [ "$FSYNC" != "on" ]; then
    echo "ABORT: fsync is '$FSYNC', not 'on'. Every number this harness would" >&2
    echo "produce under fsync=off is meaningless; the mechanisms differ in how" >&2
    echo "long they hold a lock across a commit, and fsync=off removes the commit." >&2
    exit 1
fi

for arm in $ARMS; do
    echo "--- creating $arm"
    psql -X -q -v ON_ERROR_STOP=1 -c "DROP DATABASE IF EXISTS $arm" postgres
    psql -X -q -v ON_ERROR_STOP=1 -c "CREATE DATABASE $arm" postgres
    psql -X -q -v ON_ERROR_STOP=1 -d "$arm" -f "$(schema_for "$arm")"
done

echo "setup complete"
