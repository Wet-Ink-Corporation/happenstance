-- Arm B (tag-derived key): pg_advisory_xact_lock(hashtext(tags[1])) before
-- nextval().
--
-- This is the version the crate's ES-10 note calls "the interesting one",
-- because it does not serialise the whole store. It is also the version whose
-- guarantee is narrower than the clause: two writers holding DISJOINT keys are
-- not ordered against each other at all, so the inversion is expected to
-- reproduce exactly as on the baseline whenever the two appends touch different
-- tags. The detector runs a shared-key pair and a disjoint-key pair for exactly
-- this reason.

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
    -- The key is derived from the first (canonically sorted) tag. A real
    -- adapter would derive it from the AppendCondition's query, which is the
    -- part of this mechanism that pushes work back toward the port.
    PERFORM pg_advisory_xact_lock(hashtext(p_tags[1])::bigint);
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
