//! The four probes ADR-0025 cannot be written without.
//!
//! P0 — what a second handle is: two `Database::new` on one directory, or one
//!      `Arc<Database>` with two `Connection`s.
//! P1 — does `UINT64` round-trip a value above `i64::MAX`? (decides whether the
//!      checkpoint column is `UINT64` or `INT64`, and whether
//!      `PositionOutOfRange` is a real variant or a decorative one)
//! P2 — does a transaction give read-your-own-writes *within* itself? (decides
//!      PS-4's Cypher-level condition, which is the whole of phase 11's
//!      contribution)
//! P3 — is there an injectable fault that aborts the LAST statement of a
//!      transaction after the earlier ones succeeded? (decides COMMIT_FAULT)

use std::sync::Arc;

fn line(tag: &str, body: impl std::fmt::Display) {
    println!("[{tag}] {body}");
}

fn fresh(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("lbug-probe-{name}"));
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn main() {
    // ---------------------------------------------------------------- P0
    {
        let dir = fresh("p0");
        let first = lbug::Database::new(&dir, lbug::SystemConfig::default());
        match &first {
            Ok(_) => line("P0", "first Database::new: ok"),
            Err(e) => line("P0", format!("first Database::new FAILED: {e}")),
        }
        let second = lbug::Database::new(&dir, lbug::SystemConfig::default());
        match &second {
            Ok(_) => line("P0", "second Database::new on the SAME directory: ok"),
            Err(e) => line("P0", format!("second Database::new on the SAME directory: REFUSED — {e}")),
        }

        let db = Arc::new(first.expect("first database"));
        let a = lbug::Connection::new(&db);
        let b = lbug::Connection::new(&db);
        line(
            "P0",
            format!(
                "two Connections over one Arc<Database>: {} / {}",
                if a.is_ok() { "ok" } else { "FAILED" },
                if b.is_ok() { "ok" } else { "FAILED" }
            ),
        );
        if let (Ok(a), Ok(b)) = (a, b) {
            a.query("CREATE NODE TABLE T(id STRING, PRIMARY KEY(id))").ok();
            a.query("CREATE (:T {id: 'from-a'})").ok();
            match b.query("MATCH (t:T) RETURN count(t) AS n") {
                Ok(mut r) => {
                    let seen = r.next().map(|row| format!("{row:?}"));
                    line("P0", format!("connection B sees A's committed write: {seen:?}"));
                }
                Err(e) => line("P0", format!("connection B query FAILED: {e}")),
            };
        };
    }

    // ---------------------------------------------------------------- P1
    {
        let dir = fresh("p1");
        let db = lbug::Database::new(&dir, lbug::SystemConfig::default()).expect("db");
        let c = lbug::Connection::new(&db).expect("conn");
        c.query("CREATE NODE TABLE C(k STRING, pos UINT64, PRIMARY KEY(k))")
            .expect("ddl");
        // Above i64::MAX, which is the whole question: SequencePosition is a
        // NonZeroU64 and `bigint` cannot hold the top half of that domain.
        let big: u64 = u64::MAX - 1;
        match c.query(&format!("CREATE (:C {{k: 'a', pos: {big}}})")) {
            Ok(_) => line("P1", format!("stored {big} into a UINT64 column: ok")),
            Err(e) => line("P1", format!("storing {big} FAILED: {e}")),
        }
        match c.query("MATCH (c:C) RETURN c.pos") {
            Ok(mut r) => line("P1", format!("read back: {:?}", r.next())),
            Err(e) => line("P1", format!("read back FAILED: {e}")),
        };
    }

    // ---------------------------------------------------------------- P2
    {
        let dir = fresh("p2");
        let db = lbug::Database::new(&dir, lbug::SystemConfig::default()).expect("db");
        let c = lbug::Connection::new(&db).expect("conn");
        c.query("CREATE NODE TABLE R(k STRING, v INT64, PRIMARY KEY(k))")
            .expect("ddl");

        match c.query("BEGIN TRANSACTION") {
            Ok(_) => line("P2", "BEGIN TRANSACTION accepted as a query"),
            Err(e) => line("P2", format!("BEGIN TRANSACTION REFUSED: {e}")),
        }
        c.query("CREATE (:R {k: 'x', v: 1})").expect("create inside txn");
        match c.query("MATCH (r:R {k: 'x'}) RETURN r.v") {
            Ok(mut r) => {
                let row = r.next();
                line(
                    "P2",
                    format!("READ-YOUR-OWN-WRITES inside the transaction: {row:?}"),
                );
            }
            Err(e) => line("P2", format!("read inside the transaction FAILED: {e}")),
        }
        c.query("COMMIT").expect("commit");
        match c.query("MATCH (r:R) RETURN count(r)") {
            Ok(mut r) => line("P2", format!("after COMMIT: {:?}", r.next())),
            Err(e) => line("P2", format!("after COMMIT FAILED: {e}")),
        }

        // And the rollback half, which `rollback` and PS-7 both rest on.
        c.query("BEGIN TRANSACTION").expect("begin 2");
        c.query("CREATE (:R {k: 'y', v: 2})").expect("create 2");
        match c.query("ROLLBACK") {
            Ok(_) => line("P2", "ROLLBACK accepted"),
            Err(e) => line("P2", format!("ROLLBACK REFUSED: {e}")),
        }
        match c.query("MATCH (r:R) RETURN count(r)") {
            Ok(mut r) => line("P2", format!("after ROLLBACK (expect 1): {:?}", r.next())),
            Err(e) => line("P2", format!("post-rollback read FAILED: {e}")),
        };
    }

    // ---------------------------------------------------------------- P3
    {
        let dir = fresh("p3");
        let db = lbug::Database::new(&dir, lbug::SystemConfig::default()).expect("db");
        let c = lbug::Connection::new(&db).expect("conn");
        c.query("CREATE NODE TABLE P(k STRING, v INT64, PRIMARY KEY(k))")
            .expect("ddl");
        c.query("CREATE NODE TABLE K(k STRING, PRIMARY KEY(k))")
            .expect("ddl2");
        c.query("CREATE (:K {k: 'planted'})").expect("plant");

        c.query("BEGIN TRANSACTION").expect("begin");
        c.query("CREATE (:P {k: 'read-model', v: 1})")
            .expect("read-model write inside the transaction");
        // A duplicate primary key on a pre-planted row: the candidate injection
        // for COMMIT_FAULT. `CREATE`, not `MERGE` — a MERGE matches and cannot
        // conflict, which is the error the design panel's critique caught.
        let faulted = c.query("CREATE (:K {k: 'planted'})");
        line(
            "P3",
            match &faulted {
                Ok(_) => "duplicate-key CREATE SUCCEEDED — not an injection".to_owned(),
                Err(e) => format!("duplicate-key CREATE raised: {e}"),
            },
        );
        // What happened to the transaction, and to the read-model write in it?
        let after = c.query("MATCH (p:P) RETURN count(p)");
        line(
            "P3",
            match after {
                Ok(mut r) => format!("read-model rows visible after the fault: {:?}", r.next()),
                Err(e) => format!("post-fault read FAILED: {e}"),
            },
        );
        let rolled = c.query("ROLLBACK");
        line(
            "P3",
            format!("ROLLBACK after the fault: {}", if rolled.is_ok() { "ok" } else { "refused" }),
        );
        match c.query("MATCH (p:P) RETURN count(p)") {
            Ok(mut r) => line("P3", format!("read-model rows finally (expect 0): {:?}", r.next())),
            Err(e) => line("P3", format!("final read FAILED: {e}")),
        };
    }

    println!("\nprobes complete");
}
