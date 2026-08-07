-- One pgbench script, every arm. The arms differ only in what hs_probe() and
-- hs_append() do, which is what makes the ratio between them mean something:
-- the client-side work, the number of round trips, the transaction shape and
-- the tag distribution are identical.
--
-- The transaction is probe-then-append because that is the shape of a DCB
-- append: read the consistency boundary, then write under it. A single-statement
-- append would understate every mechanism's cost, because the interval a lock is
-- held over is what the mechanisms actually charge for.
\set tagid random(1, 1000)
BEGIN;
SELECT hs_probe(ARRAY['course:' || :tagid::text]);
SELECT hs_append('CourseCapacityChanged', '\x0102030405060708'::bytea, NULL, ARRAY['course:' || :tagid::text]);
END;
