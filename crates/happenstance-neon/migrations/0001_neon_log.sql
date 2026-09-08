-- Migration 1 — the event log, as Neon's `/sql` endpoint has to receive it.
--
-- This is NOT `crates/happenstance-postgres/migrations/0001_event_log.sql`
-- copied. It carries the same columns, the same indexes and the same reasoning,
-- and it differs in the two ways this endpoint forces:
--
-- 1. **Every identifier is a placeholder**, rendered by `crate::migration`.
--    The Postgres adapter hard-codes unqualified `event`, `event_position_seq`
--    and `store_meta` and leans on a pooled `search_path` to place them. There
--    is no session here to hold a `search_path` in, and the `/sql` proxy
--    discards `options=-c search_path=…` from the connection string — measured,
--    `SHOW search_path` still answers `"$user", public`. So a name that is not
--    written out is a name resolved against whatever backend the proxy picked,
--    and the fixture's one-schema-per-instance isolation would be a lie. Every
--    `@…@` below is substituted with an already-quoted, already-qualified
--    identifier by `NeonConfig`.
--
-- 2. **One statement per statement.** The endpoint's `query` field takes exactly
--    one; a multi-statement string is refused with `Database request failed`
--    and no detail. `crate::migration` splits this file on its statement
--    terminators and sends the pieces as one non-interactive batch, so the whole
--    migration is still one round trip and still atomic.
--
-- Idempotent (`IF NOT EXISTS` throughout) for the reason the Postgres file
-- gives: one fixture instance is connected to more than once, and a migration
-- that fails on its second application reports a broken environment as an
-- adapter defect.
--
-- The visibility mechanism is `xid8` + `pg_snapshot_xmin`, inherited from
-- ADR-0024 and RE-MEASURED here rather than assumed, because that ADR's numbers
-- were taken against a dedicated container and this is a shared Neon branch with
-- a pooler and Neon's own compute processes on it. Ten append-then-read cycles
-- against the live endpoint: the frontier had already passed the appended
-- position on the FIRST read every time, ten of ten, with the read landing
-- 80-100 ms after the append's commit. The mechanism is bought here, not
-- borrowed.
--
-- `position` still carries NO column default. The sequence is real and is read
-- explicitly by the INSERT, which keeps `information_schema` reporting
-- `column_default IS NULL` and `is_identity = 'NO'` — so "position is not
-- bigserial" stays a checkable fact rather than a turn of phrase.

CREATE SCHEMA IF NOT EXISTS @schema@;

CREATE TABLE IF NOT EXISTS @event@ (
    -- Deliberately not `bigserial`, and the distinction is not cosmetic even
    -- though the allocator is the same. `nextval()` DOES allocate outside the
    -- transaction, and that IS where position order stops being commit order.
    -- The frontier predicate on the read side is what answers it; keeping the
    -- default off the column is what makes the allocation site explicit at the
    -- one INSERT that performs it, rather than implicit in the DDL.
    position        bigint  PRIMARY KEY,
    event_type      text    NOT NULL,
    -- Payloads are opaque bytes. The contract crate never sees them decoded
    -- (ADR-0003), so neither does this schema. On the wire they are base64
    -- rather than `\x` hex: both survive the round trip, and base64 costs 33%
    -- expansion against `MAX_RESPONSE_BYTES` where hex costs 100%.
    data            bytea   NOT NULL,
    -- Nullable on purpose: `None` and `Some(empty)` are different values and the
    -- suite has a rule that says so. A `NOT NULL DEFAULT ''` here would collapse
    -- them and pass every other rule.
    metadata        bytea,
    -- `Tags` is canonically sorted, which is what keeps the containment operator
    -- `@>` available against the GIN index below.
    tags            text[]  NOT NULL,
    -- The two halves of `EventId`, which is `(StoreId, SequencePosition)`.
    -- `StoreId` is `[u8; 16]`, and the CHECK is what makes that a fact about the
    -- table rather than a convention in the encoder. Nullable because a
    -- replication ingest may hold rows minted elsewhere before this store stamps
    -- its own.
    origin_store    bytea   CONSTRAINT event_origin_store_is_16_bytes
                            CHECK (origin_store IS NULL OR octet_length(origin_store) = 16),
    origin_position bigint,
    -- Milliseconds since the epoch, and signed because `RecordedAt` is `i64` and
    -- admits times *before* it. A `timestamptz` column would be a lossy
    -- conversion the port never asked for.
    recorded_at     bigint  NOT NULL,
    -- The mechanism. The appending transaction stamps its own id, and a reader
    -- admits only rows beneath `pg_snapshot_xmin(pg_current_snapshot())` — the
    -- frontier below which no transaction can still be in flight. Writers never
    -- serialise; the cost is paid on the read side and it is structural rather
    -- than incremental: `head` reports a frontier rather than a maximum, and
    -- read-your-own-writes is not promised.
    --
    -- A DEFAULT rather than an explicit value in the INSERT, and the asymmetry
    -- with `position` above is deliberate: `pg_current_xact_id()` must be
    -- evaluated by the writing transaction itself, so a default is the one
    -- spelling that cannot be got wrong by a caller assembling the statement.
    xact_id         xid8    NOT NULL DEFAULT pg_current_xact_id()
);

-- Read explicitly by the INSERT, never as a column default. `OWNED BY` ties its
-- lifetime to the column so a dropped table does not leave it behind.
CREATE SEQUENCE IF NOT EXISTS @sequence@ OWNED BY @event@.position;

-- Tag matching. `text[]` + GIN, as in the Postgres adapter and for the same
-- reason: `@>` expresses a query item's "tags are AND, and a superset matches"
-- directly rather than as a self-join per tag.
--
-- It carries a second load here that it does not carry there. A conditional
-- append is `SERIALIZABLE`, so its guard read has to be predicate-locked for SSI
-- to see the write-skew at all; Postgres has supported predicate locking on GIN
-- since 11, and the server behind this endpoint is 18.6.
CREATE INDEX IF NOT EXISTS @tags_idx@ ON @event@ USING gin (tags);

-- Type-and-position, in that order, so a query item that names types is a range
-- scan rather than a filter over one.
CREATE INDEX IF NOT EXISTS @type_idx@ ON @event@ (event_type, position);

-- One store never mints one `EventId` twice, and this is where that is a fact
-- rather than an intention: `event_ids_are_unique_within_a_store` is a
-- conformance rule, and an adapter that enforces it only in Rust enforces it
-- only until two requests race — which here is every request, because there is
-- no session and nothing is serialised anywhere. A partial index because the
-- pair is nullable until a row is stamped, and NULLs do not collide in a plain
-- unique index.
CREATE UNIQUE INDEX IF NOT EXISTS @origin_idx@
    ON @event@ (origin_store, origin_position)
    WHERE origin_store IS NOT NULL AND origin_position IS NOT NULL;

-- The frontier predicate's own index. Without it every read is a sequential
-- scan, which would charge the mechanism for the absence of an index rather than
-- for what it actually costs.
CREATE INDEX IF NOT EXISTS @xact_idx@ ON @event@ (xact_id);

-- The store's own identity, which is the `StoreId` half of every `EventId` this
-- store mints. One row, inserted once and never updated by the adapter.
--
-- `gen_random_uuid()` rather than an extension: it is core since PostgreSQL 13
-- and gives exactly the sixteen bytes `StoreId` holds, so `uuid_send` is a
-- reinterpretation rather than a conversion. `ON CONFLICT DO NOTHING` makes the
-- insert idempotent, which is what lets two handles race a first `connect()` and
-- both end up agreeing about who this store is.
CREATE TABLE IF NOT EXISTS @meta@ (
    k text  PRIMARY KEY,
    v bytea NOT NULL
);

INSERT INTO @meta@ (k, v)
VALUES ('store_id', uuid_send(gen_random_uuid()))
ON CONFLICT (k) DO NOTHING;
