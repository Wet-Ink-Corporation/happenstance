-- Baseline (unguarded): position comes from nextval(), which allocates outside
-- the transaction. This arm is expected to FAIL the inversion detector; it is
-- the experiment's first positive control.

CREATE TABLE event (
    position   bigint PRIMARY KEY,
    event_type text   NOT NULL,
    data       bytea  NOT NULL,
    metadata   bytea,
    tags       text[] NOT NULL  -- canonically sorted
);

CREATE SEQUENCE event_position_seq OWNED BY event.position;
ALTER TABLE event ALTER COLUMN position SET DEFAULT nextval('event_position_seq');

CREATE INDEX event_tags_idx ON event USING gin (tags);
CREATE INDEX event_type_idx ON event (event_type, position);

-- The append path. One statement; nextval() is taken by the column default.
CREATE FUNCTION hs_append(p_type text, p_data bytea, p_meta bytea, p_tags text[])
RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    INSERT INTO event (event_type, data, metadata, tags)
    VALUES (p_type, p_data, p_meta, p_tags)
    RETURNING position INTO v;
    RETURN v;
END $$;

-- The append-condition probe: "what is the highest position carrying any of
-- these tags?". Every arm pays its own read cost here, which is the point of
-- routing the probe through a per-arm function rather than inlining it in the
-- pgbench script.
CREATE FUNCTION hs_probe(p_tags text[]) RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    SELECT max(position) INTO v FROM event WHERE tags && p_tags;
    RETURN v;
END $$;

CREATE FUNCTION hs_read() RETURNS TABLE (pos bigint) LANGUAGE plpgsql AS $$
BEGIN
    RETURN QUERY SELECT position FROM event ORDER BY position;
END $$;

CREATE FUNCTION hs_head() RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    SELECT max(position) INTO v FROM event;
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
