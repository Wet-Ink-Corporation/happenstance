-- Migration 1 — the event log.
--
-- Applied by `include_str!` and one `execute`, not by `sqlx::migrate!`; the
-- manifest records why. It is idempotent (`IF NOT EXISTS` throughout) so that
-- applying it twice to one schema is a no-op rather than an error, because the
-- alternative — a fixture that fails on its second `connect()` — reports a
-- broken environment as an adapter defect.
--
-- Every column below is settled independently of ADR-0024. The
-- position-visibility mechanism is a *deliberate* deliverable of a later story,
-- and this file must not answer it by accident: there is no column default on
-- `position`, no `serial`/`bigserial`/`GENERATED … AS IDENTITY`, and no
-- visibility-mechanism column. Under the `xid8` arm this table gains one; under
-- either lock arm it gains none. Adding one here would make a decision record
-- describe a schema that had already chosen.

CREATE TABLE IF NOT EXISTS event (
    -- Deliberately not `bigserial`. A `serial` column is `nextval()` evaluated
    -- as a default, and `nextval()` allocating outside the transaction is
    -- precisely where ES-10's invariant is lost. How this value is produced is
    -- ADR-0024's to settle; that it is not produced by a column default is
    -- already decided.
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
    recorded_at     bigint  NOT NULL
);

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
