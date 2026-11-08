-- Migration 3 — write groups (#420), on the event log's lineage.
--
-- Which rows one `append`, or one ingested group with its compensation, wrote
-- together, so a runner and a store peer can read the log back in the writes
-- that made it (SY-39). `write_group` holds the lowest position the write put
-- in, the same value on every row of it; `group_guard` holds the guard an
-- ingested group arrived under, in `happenstance-sync`'s stored form, on that
-- first row only.
--
-- Rendered and split by `migration.rs`, as migration 1 is: no dollar quote and
-- no semicolon inside a literal, and every identifier a placeholder. Additive
-- and idempotent, `IF NOT EXISTS` throughout. Both columns are nullable with no
-- default, so adding them rewrites no row. A row written before this migration
-- has `NULL` in both and reads as a group of its own, named by its position;
-- there is no backfill.
--
-- Apply it before deploying a build that writes these columns: such a build
-- names `write_group` in every `INSERT`, and fails against a table without it.
--
-- An ingest writes a provisional negative `write_group` and replaces it in the
-- same transaction, so no committed row holds one; no CHECK can say so, because
-- a CHECK is evaluated per statement and the provisional value lives across two.
--
-- The index serves the group read. Partial, because a row from before this
-- migration is found by its position through the primary key. Built inside the
-- migration's batch, so it blocks writes while it builds; on a large existing
-- table, create it first with `CREATE INDEX CONCURRENTLY` under the same name,
-- and `IF NOT EXISTS` below then leaves it alone.

ALTER TABLE @event@ ADD COLUMN IF NOT EXISTS write_group bigint;

ALTER TABLE @event@ ADD COLUMN IF NOT EXISTS group_guard bytea;

CREATE INDEX IF NOT EXISTS @write_group_idx@
    ON @event@ (write_group)
    WHERE write_group IS NOT NULL;
