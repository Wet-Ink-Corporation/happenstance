-- Arm B (constant key): pg_advisory_xact_lock(0) before nextval().
--
-- The lock is released by the commit, so the interval [allocate, commit] is
-- mutually exclusive across writers and allocation order is commit order. With
-- a constant key this is arm A wearing a different hat: one lock, every writer.

CREATE TABLE event (
    position   bigint PRIMARY KEY,
    event_type text   NOT NULL,
    data       bytea  NOT NULL,
    metadata   bytea,
    tags       text[] NOT NULL
);

CREATE SEQUENCE event_position_seq OWNED BY event.position;

CREATE INDEX event_tags_idx ON event USING gin (tags);
CREATE INDEX event_type_idx ON event (event_type, position);

CREATE FUNCTION hs_append(p_type text, p_data bytea, p_meta bytea, p_tags text[])
RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    PERFORM pg_advisory_xact_lock(0::bigint);
    v := nextval('event_position_seq');
    INSERT INTO event (position, event_type, data, metadata, tags)
    VALUES (v, p_type, p_data, p_meta, p_tags);
    RETURN v;
END $$;

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
