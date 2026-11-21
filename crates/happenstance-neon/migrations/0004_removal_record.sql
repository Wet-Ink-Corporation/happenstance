-- Migration 4 — the removal record (#483, #484, ADR-0110), on the event log's
-- lineage. The table only; its trigger is `migration.rs`'s
-- `removal_trigger_statements`.
--
-- One row per event removed from the event table by any means that keeps the
-- position sequence where it was: an operator's `DELETE`, a retention prune,
-- `psql`, the Neon console. The key is the removed row's origin pair, its
-- `EventId`, so foreign and own events are recorded alike: every row this
-- adapter writes carries a non-null origin. An ingest treats a recorded identity
-- as one the store holds (SY-40): skipped, never appended again and never
-- refused under SY-38. Every ingest locks this table `IN SHARE MODE` first, so a
-- removal and an ingest wait for each other.
--
-- Rendered and split by `migration.rs`, as migration 1 is: no dollar quote and no
-- semicolon inside a literal, and every identifier a placeholder. The trigger
-- that writes the record has a `plpgsql` body, which this splitter would cut in
-- half, so it is built as whole statements in Rust and sent beside this file's
-- in the same batch.
--
-- What does not record: `TRUNCATE`, which fires no row trigger; a removal made
-- before this migration ran, which an operator records by hand with an `INSERT`
-- of the origin pair; and a branch reset or a restore, which brings this table
-- back with the log and the sequence, so it rolls back with the allocator, as
-- SY-40 requires. Deleting a row from this table lets that identity be ingested
-- again. The table grows by one small row per removal and is kept for ever.
--
-- Additive and idempotent: `IF NOT EXISTS`. Apply it, and the trigger, before
-- deploying a build that reads it: such a build's ingest locks this table, and
-- fails against a schema without it.

CREATE TABLE IF NOT EXISTS @removed@ (
    origin_store    bytea   NOT NULL,
    origin_position bigint  NOT NULL,
    PRIMARY KEY (origin_store, origin_position)
);
