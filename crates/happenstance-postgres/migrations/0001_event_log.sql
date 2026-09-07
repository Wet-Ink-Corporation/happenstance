-- Migration 1 — the event log.
--
-- Applied by `include_str!` and one `execute`, not by `sqlx::migrate!`; the
-- manifest records why. It is idempotent (`IF NOT EXISTS` throughout) so that
-- applying it twice to one schema is a no-op rather than an error, because the
-- alternative — a fixture that fails on its second `connect()` — reports a
-- broken environment as an adapter defect.
--
-- The visibility mechanism is `xid8` + `pg_snapshot_xmin` -- the arm ADR-0013's
-- phase-2 experiment measured at 0.99-1.03x baseline while the two serialising
-- arms cost 16x and 30x at 64 writers, and the tag-keyed advisory lock bought a
-- per-boundary invariant where ES-10 states a global one
-- (`experiments/position-visibility/README.md`). It is wired here on that prior
-- authority; the *choice*, its structural bill and a re-measurement against this
-- adapter belong to the story that owns them, and nothing in this file cites a
-- decision record that does not exist yet.
--
-- `position` still carries NO column default. The sequence is real and is read
-- explicitly by the INSERT, which keeps `information_schema` reporting
-- `column_default IS NULL` and `is_identity = 'NO'` -- so "position is not
-- bigserial" stays a checkable fact rather than a turn of phrase, and the guard
-- that asserts it survives the mechanism landing.

CREATE TABLE IF NOT EXISTS event (
    -- Deliberately not `bigserial`, and the distinction is not cosmetic even
    -- though the allocator is the same. `nextval()` DOES allocate outside the
    -- transaction, and that IS where position order stops being commit order --
    -- arm C does not deny it, it stops treating position order as VISIBILITY
    -- order and puts the frontier predicate on the read side instead. Keeping
    -- the default off the column is what makes the allocation site explicit at
    -- the one INSERT that performs it, rather than implicit in the DDL.
    position        bigint  PRIMARY KEY,
    event_type      text    NOT NULL,
    -- Payloads are opaque bytes. The contract crate never sees them decoded
    -- (ADR-0003), so neither does this schema.
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
    -- admits times *before* it — there is a core test named for that case. A
    -- `timestamptz` column would be a lossy conversion the port never asked for.
    recorded_at     bigint  NOT NULL,
    -- The mechanism. The appending transaction stamps its own id, and a reader
    -- admits only rows beneath `pg_snapshot_xmin(pg_current_snapshot())` -- the
    -- frontier below which no transaction can still be in flight. Writers never
    -- serialise; the cost is paid on the read side and it is structural rather
    -- than incremental: `head` reports a frontier rather than a maximum,
    -- read-your-own-writes does not hold, and staleness is bounded by the
    -- longest open write transaction anywhere in the cluster.
    --
    -- A DEFAULT rather than an explicit value in the INSERT, and the asymmetry
    -- with `position` above is deliberate: `pg_current_xact_id()` must be
    -- evaluated by the writing transaction itself, so a default is the one
    -- spelling that cannot be got wrong by a caller assembling the statement.
    xact_id         xid8    NOT NULL DEFAULT pg_current_xact_id()
);

-- Read explicitly by the INSERT, never as a column default. `OWNED BY` ties its
-- lifetime to the column so a dropped table does not leave it behind.
CREATE SEQUENCE IF NOT EXISTS event_position_seq OWNED BY event.position;

-- Tag matching. `text[]` + GIN was chosen over a join table and over `jsonb`:
-- it is the shape the crate's own prose schema already leaned to, it is the
-- shape `experiments/position-visibility/` ran its numbers against, and `@>`
-- expresses a query item's "tags are AND, and a superset matches" directly
-- rather than as a self-join per tag.
CREATE INDEX IF NOT EXISTS event_tags_idx ON event USING gin (tags);

-- Type-and-position, in that order, so a query item that names types is a range
-- scan rather than a filter over one.
CREATE INDEX IF NOT EXISTS event_type_idx ON event (event_type, position);

-- One store never mints one `EventId` twice, and this is where that is a fact
-- rather than an intention: `event_ids_are_unique_within_a_store` is a
-- conformance rule, and an adapter that enforces it only in Rust enforces it
-- only until two connections race. A partial index because the pair is nullable
-- until a row is stamped, and `NULL`s do not collide in a plain unique index —
-- which would make the constraint quietly weaker than it reads.
CREATE UNIQUE INDEX IF NOT EXISTS event_origin_idx
    ON event (origin_store, origin_position)
    WHERE origin_store IS NOT NULL AND origin_position IS NOT NULL;

-- The frontier predicate's own index. Without it every read is a sequential
-- scan, which would charge the mechanism for the absence of an index rather
-- than for what it actually costs -- the experiment's schema says so in the same
-- words, and its numbers were taken with this index present.
CREATE INDEX IF NOT EXISTS event_xact_idx ON event (xact_id);

-- The store's own identity, which is the `StoreId` half of every `EventId` this
-- store mints. One row, inserted once and never updated by the adapter.
--
-- `gen_random_uuid()` rather than an extension: it is core since PostgreSQL 13
-- and gives exactly the sixteen bytes `StoreId` holds, so `uuid_send` is a
-- reinterpretation rather than a conversion. `ON CONFLICT DO NOTHING` makes the
-- insert idempotent, which is what lets two handles race a first `connect()` and
-- both end up agreeing about who this store is -- the loser reads the winner's
-- row rather than keeping the id it generated.
CREATE TABLE IF NOT EXISTS store_meta (
    k text  PRIMARY KEY,
    v bytea NOT NULL
);

INSERT INTO store_meta (k, v)
VALUES ('store_id', uuid_send(gen_random_uuid()))
ON CONFLICT (k) DO NOTHING;
