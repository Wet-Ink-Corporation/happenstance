-- Arm C's cost is not a stall and not an inversion; it is read-side staleness.
-- This measures it directly: append, commit, then poll a FRESH snapshot until
-- the frontier admits the row, and report the elapsed time.
--
-- The COMMIT inside the loop is what makes each iteration a new transaction, so
-- pg_snapshot_xmin is re-evaluated rather than frozen. The probing session
-- never holds an xid of its own, so it does not hold back the frontier it is
-- waiting on.
DO $do$
DECLARE
    p  bigint;
    t0 timestamptz;
    ms double precision;
BEGIN
    p := hs_append('StalenessProbe', '\x00'::bytea, NULL, ARRAY['staleness:probe']);
    COMMIT;
    t0 := clock_timestamp();
    LOOP
        IF EXISTS (
            SELECT 1 FROM event
            WHERE position = p
              AND xact_id < pg_snapshot_xmin(pg_current_snapshot())
        ) THEN
            EXIT;
        END IF;
        COMMIT;
        PERFORM pg_sleep(0.0005);
        IF clock_timestamp() - t0 > interval '60 seconds' THEN
            RAISE EXCEPTION 'staleness probe did not converge within 60s';
        END IF;
    END LOOP;
    ms := extract(epoch from (clock_timestamp() - t0)) * 1000;
    RAISE NOTICE 'STALENESS_MS %', ms;
END
$do$;
