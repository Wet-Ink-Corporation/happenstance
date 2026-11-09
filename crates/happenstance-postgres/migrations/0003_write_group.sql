-- Migration 3 — write groups (#420), on the event log's lineage.
--
-- Which rows one `append`, or one ingested group with its compensation, wrote
-- together, so a runner and a store peer can read the log back in the writes
-- that made it (SY-39). `write_group` holds the lowest position the write put
-- in, the same value on every row of it; `group_guard` holds the guard an
-- ingested group arrived under, in `happenstance-sync`'s stored form, on that
-- first row only.
--
-- Additive, and idempotent for the reason migration 1 is: `IF NOT EXISTS`
-- throughout. Both columns are nullable and carry no default, so adding them
-- rewrites no row. A row written before this migration has `NULL` in both and
-- reads as a group of its own, named by its position; nothing recorded which
-- rows one earlier call wrote, so there is no backfill to run.
--
-- Apply it before deploying a build that writes these columns: such a build's
-- `INSERT` names `write_group` on every row, and fails against a table without
-- it.
--
-- The index serves the group read, which looks rows up by their group. Partial,
-- because a row from before this migration is found by its position through
-- the primary key and needs no entry. Built inside the migration's implicit
-- transaction, so it holds a lock that blocks writes while it builds; on a
-- large existing table, create it first with `CREATE INDEX CONCURRENTLY` under
-- the same name, and `IF NOT EXISTS` below then leaves it alone.

ALTER TABLE event ADD COLUMN IF NOT EXISTS write_group bigint;

ALTER TABLE event ADD COLUMN IF NOT EXISTS group_guard bytea;

CREATE INDEX IF NOT EXISTS event_write_group_idx
    ON event (write_group)
    WHERE write_group IS NOT NULL;
