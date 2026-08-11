-- Arm A: a serialised sequence table.
--
-- Allocation is `UPDATE hs_sequence SET n = n + 1 RETURNING n` inside the append
-- transaction, so the row lock is held until commit and allocation order IS
-- commit order. Every writer in the store contends on one row; that is the cost,
-- and arm A at 64 writers is the experiment's second positive control.

CREATE TABLE event (
    position   bigint PRIMARY KEY,
    event_type text   NOT NULL,
    data       bytea  NOT NULL,
    metadata   bytea,
    tags       text[] NOT NULL
);

CREATE INDEX event_tags_idx ON event USING gin (tags);
CREATE INDEX event_type_idx ON event (event_type, position);

CREATE TABLE hs_sequence (id int PRIMARY KEY, n bigint NOT NULL);
INSERT INTO hs_sequence VALUES (1, 0);

CREATE FUNCTION hs_append(p_type text, p_data bytea, p_meta bytea, p_tags text[])
RETURNS bigint LANGUAGE plpgsql AS $$
DECLARE v bigint;
BEGIN
    UPDATE hs_sequence SET n = n + 1 WHERE id = 1 RETURNING n INTO v;
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
    UPDATE hs_sequence SET n = 0 WHERE id = 1;
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
