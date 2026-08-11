-- Arm C: keep nextval(), and move the invariant to the read side.
--
-- Each row carries the appending transaction's xid8. A reader admits only rows
-- whose xid8 is below pg_snapshot_xmin(pg_current_snapshot()) -- the frontier
-- beneath which no transaction can still be in flight. Writers never serialise.
-- The cost lands on reads: every read gains a predicate, head reports the
-- frontier rather than max(position), and a freshly committed event is
-- invisible until the frontier passes it. That last part is read-side
-- STALENESS, and it is measured separately.

CREATE TABLE event (
    position   bigint PRIMARY KEY,
    event_type text   NOT NULL,
    data       bytea  NOT NULL,
    metadata   bytea,
    tags       text[] NOT NULL,
    xact_id    xid8   NOT NULL DEFAULT pg_current_xact_id()
);

CREATE SEQUENCE event_position_seq OWNED BY event.position;
ALTER TABLE event ALTER COLUMN position SET DEFAULT nextval('event_position_seq');

CREATE INDEX event_tags_idx ON event USING gin (tags);
CREATE INDEX event_type_idx ON event (event_type, position);
-- The visibility predicate's own index. Without it every read is a seq scan,
-- which would charge arm C for the absence of an index rather than for the
-- mechanism.
CREATE INDEX event_xact_idx ON event (xact_id);

-- The append path is the baseline's. nextval() is untouched.
CREATE FUNCTION hs_append(p_type text, p_data bytea, p_meta bytea, p_tags text[])
RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    INSERT INTO event (event_type, data, metadata, tags)
    VALUES (p_type, p_data, p_meta, p_tags)
    RETURNING position INTO v;
    RETURN v;
END $$;

-- Every read carries the frontier predicate. This is the structural cost.
CREATE FUNCTION hs_probe(p_tags text[]) RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    SELECT max(position) INTO v FROM event
    WHERE tags && p_tags
      AND xact_id < pg_snapshot_xmin(pg_current_snapshot());
    RETURN v;
END $$;

CREATE FUNCTION hs_read() RETURNS TABLE (pos bigint) LANGUAGE plpgsql AS $$
BEGIN
    RETURN QUERY
        SELECT position FROM event
        WHERE xact_id < pg_snapshot_xmin(pg_current_snapshot())
        ORDER BY position;
END $$;

CREATE FUNCTION hs_head() RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    SELECT max(position) INTO v FROM event
    WHERE xact_id < pg_snapshot_xmin(pg_current_snapshot());
    RETURN coalesce(v, 0);
END $$;

CREATE FUNCTION hs_reset() RETURNS void LANGUAGE plpgsql AS $$
BEGIN
    TRUNCATE event;
    PERFORM setval('event_position_seq', 1, false);
END $$;

CREATE FUNCTION hs_seed(n int) RETURNS void LANGUAGE plpgsql AS $$
DECLARE i int;
BEGIN
    FOR i IN 1..n LOOP
        PERFORM hs_append(
            'CourseCapacityChanged',
            '\x0102030405060708'::bytea,
            NULL,
            ARRAY['course:' || ((i % 1000) + 1)::text]
        );
    END LOOP;
END $$;
