-- Migration 4 — the removal record (#483, #484, ADR-0110), on the event log's
-- lineage.
--
-- One row per event removed from `event` by any means that keeps the position
-- sequence where it was: an operator's `DELETE`, a retention prune, `psql`. The
-- key is the removed row's origin pair, its `EventId`, so foreign and own events
-- are recorded alike: every row this adapter writes carries a non-null origin.
-- An ingest treats a recorded identity as one the store holds (SY-40): skipped,
-- counted as skipped, never appended again and never refused under SY-38.
--
-- Written by a trigger, so it fires for every client and every session, not only
-- for callers of this crate. `BEFORE DELETE ... FOR EACH ROW`, because the
-- deleting transaction must take the record's lock at its first row, before that
-- row is marked deleted: an ingest holding `SHARE` on the record that meets the
-- row under `ON CONFLICT` then sees a live row and skips it, rather than waiting
-- on a deleter that is itself waiting on the ingest's lock. The `WHEN` mirrors
-- `event_origin_idx`'s predicate: a row with no origin pair was never in the
-- dedupe set, and no peer can offer it back.
--
-- The function names the record through `TG_TABLE_SCHEMA`, the schema of the
-- `event` table the trigger fired on, so it writes to the right record whatever
-- `search_path` the deleting session has. `EXECUTE` rather than a static
-- statement because PL/pgSQL caches a static statement's plan per session, and
-- the plan would bind whichever `event_removed` the first call resolved.
--
-- What does NOT record: `TRUNCATE`, which fires no row trigger (it voids the
-- record for everything it removes, and `TRUNCATE ... RESTART IDENTITY` is a
-- rewind, already outside ES-38); and a removal made before this migration ran,
-- which an operator records by hand:
--
--     INSERT INTO event_removed (origin_store, origin_position)
--     VALUES ('\x...'::bytea, 42) ON CONFLICT DO NOTHING;
--
-- A restore (`pg_restore`, point-in-time recovery, a copy of the database)
-- brings this table back with the log and the sequence, so it rolls back with
-- the allocator, which is what SY-40 requires. Deleting a row from this table
-- lets that identity be ingested again; the only safe time to do so is once no
-- peer can serve it again. The table grows by one small row per removal and is
-- kept for ever.
--
-- Additive and idempotent: `IF NOT EXISTS`, `CREATE OR REPLACE`, and the trigger
-- created only when the table carries none of that name. Apply it before
-- deploying a build that reads it: such a build's ingest locks this table first,
-- and fails against a schema without it.

CREATE TABLE IF NOT EXISTS event_removed (
    origin_store    bytea   NOT NULL,
    origin_position bigint  NOT NULL,
    PRIMARY KEY (origin_store, origin_position)
);

CREATE OR REPLACE FUNCTION event_record_removal() RETURNS trigger
    LANGUAGE plpgsql AS $record$
BEGIN
    EXECUTE format(
        'INSERT INTO %I.event_removed (origin_store, origin_position) '
        'VALUES ($1, $2) ON CONFLICT DO NOTHING',
        TG_TABLE_SCHEMA
    ) USING OLD.origin_store, OLD.origin_position;
    RETURN OLD;
END
$record$;

DO $trigger$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgrelid = 'event'::regclass AND tgname = 'event_record_removal'
    ) THEN
        CREATE TRIGGER event_record_removal
            BEFORE DELETE ON event
            FOR EACH ROW
            WHEN (OLD.origin_store IS NOT NULL AND OLD.origin_position IS NOT NULL)
            EXECUTE FUNCTION event_record_removal();
    END IF;
END
$trigger$;
