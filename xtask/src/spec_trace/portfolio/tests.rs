//! Each test hands [`check`] a document one edit away from the shape the tree
//! has today, and names the wrong input that edit is.

use super::*;
use crate::spec_trace::parse_clauses;

/// The tree's shape at #125, cut to what the check reads: §1.4's prefixes,
/// §1.6's three ports, one frozen and one open clause per port, four axes and
/// their ledger rows.
const TODAY: &str = "\
### 1.4 Clause identifiers

| Prefix | Surface | Section |
|---|---|---|
| `VT-n` | value types — the vocabulary every port shares | §2 |
| `ES-n` | the `EventStore` port | §3 |
| `PS-n` | the `ProjectionStore` port | §4 |
| `SY-n` | the `SyncPeer` port | §5 |

### 1.6 The three ports

| Port | Where it lives | Maturity | What would freeze it |
|---|---|---|---|
| **`EventStore`** | `store.rs` | **Frozen at 0.1**, conditional | Already frozen |
| **`ProjectionStore`** | `projection.rs` | **Frozen** (ADR-0063) | Frozen |
| **`SyncPeer`** | `lib.rs` | **Shape specified, experiments deferred** | Two real peers |

#### ES-1 — an append is atomic
`[FROZEN]`
Rejects: a half-written batch.

#### ES-40 — a condition over removed history does not reject
`[PROVISIONAL — an adapter that holds only a suffix of its log]`
Rejects: a store that refuses.

#### PS-1 — the read model and the checkpoint commit together
`[FROZEN]`
Rejects: two transactions.

#### PS-6 — a buffering adapter's begin resolves at its first poll
`[PROVISIONAL — an adapter that must reserve from its server]`
Rejects: a round trip in begin.

#### SY-8 — the port describes one peer relationship
`[FROZEN]`
Rejects: a hub type.

#### SY-14 — bulk ingest completes in one round trip
`[DEFERRED — the one-shot HTTP peer, owned by the phase that builds it]`
Rejects: a per-event round trip.

#### The portfolio

| Axis | Near end | Far end | Far end exists? | Kind of instrument needed |
|---|---|---|---|---|
| **Transport** | In-process | One-shot HTTP | **Adapter yes, since phase 10b.** Neon | Adapter |
| **Async flavour** | `Send` | `!Send` | **Fixture yes, adapter no.** Local store | Fixture |
| **Batch shape** (`ProjectionStore`) | Live | Replayed | **Far end adapter yes, near end no.** Sqlite | Adapter |
| **Completeness** | Whole log | A suffix | **No. Planned for phase 14** | Fixture |

#### The freeze ledger

| Axis | Port | Discharged by | Why |
|---|---|---|---|
| Transport | `EventStore` | far end | Neon |
| Transport | `SyncPeer` | carried by SY-14 | the one-shot HTTP peer |
| Async flavour | `EventStore` | ADR-0013 | accepted |
| Batch shape | `ProjectionStore` | far end | Sqlite |
| Completeness | `EventStore` | carried by ES-40 | ES-40's marker |
";

/// The records the tree holds, as far as [`TODAY`] cites them.
fn records(number: &str) -> Option<Record> {
    let (accepted, text) = match number {
        "0013" => (
            true,
            "| **Async flavour** | `EventStore` | **Accepted by this ADR** |",
        ),
        "0042" => (
            false,
            "Async flavour on `EventStore`, accepted, then superseded",
        ),
        "0050" => (true, "a record about something else entirely"),
        "0060" => (true, "Async flavour, accepted for `ProjectionStore` only"),
        _ => return None,
    };
    Some(Record {
        accepted,
        text: text.to_owned(),
    })
}

/// The walk and findings [`check`] reports against `spec`.
fn walked(spec: &str) -> (Walk, Vec<Finding>) {
    check(spec, &parse_clauses(spec), |n| Ok(records(n)))
        .expect("the stand-in records never fail to read")
}

/// The findings [`check`] reports against `spec`.
fn findings(spec: &str) -> Vec<Finding> {
    walked(spec).1
}

/// `base` with `from` replaced once; panics when `from` is absent, so an edit
/// that silently stops applying cannot turn a test green.
fn edit(base: &str, from: &str, to: &str) -> String {
    assert!(
        base.contains(from),
        "the fixture no longer carries {from:?}"
    );
    base.replacen(from, to, 1)
}

/// [`TODAY`] with one edit.
fn edited(from: &str, to: &str) -> String {
    edit(TODAY, from, to)
}

/// One finding on `line`.
fn at(line: usize, breach: Breach) -> Finding {
    Finding {
        line: Some(line),
        breach,
    }
}

/// The 1-based line of the first line of `spec` containing `needle`.
fn line_of(spec: &str, needle: &str) -> usize {
    spec.lines()
        .position(|l| l.contains(needle))
        .expect("the needle is in the fixture")
        + 1
}

fn s(text: &str) -> String {
    text.to_owned()
}

/// A frozen port's row on `axis` that does not discharge it.
fn row(axis: &str, port: &str, problem: RowProblem) -> Breach {
    Breach::Row {
        axis: s(axis),
        port: s(port),
        problem,
    }
}

/// SY-8 and SY-14 made open, so `SyncPeer` carries no `[FROZEN]` clause.
fn sync_peer_unfrozen() -> String {
    edited(
        "#### SY-8 — the port describes one peer relationship\n`[FROZEN]`",
        "#### SY-8 — the port describes one peer relationship\n`[PROVISIONAL — a hub that needs a type of its own]`",
    )
}

#[test]
fn the_shape_the_tree_has_today_passes_and_is_counted() {
    let (walk, found) = walked(TODAY);

    assert_eq!(found, []);
    assert_eq!(
        walk,
        Walk {
            axes: 4,
            frozen: vec![s("EventStore"), s("ProjectionStore"), s("SyncPeer")],
            far_end: 2,
            accepted: 1,
            carried: 2,
            unwalked: 0,
        }
    );
}

/// CF-26's own `Rejects:` — reading a fixture tick as coverage.
#[test]
fn a_fixture_only_verdict_does_not_put_an_adapter_at_the_far_end() {
    let spec = edited(
        "| Async flavour | `EventStore` | ADR-0013 |",
        "| Async flavour | `EventStore` | far end |",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Async flavour | `EventStore`"),
            row(
                "Async flavour",
                "EventStore",
                RowProblem::NoAdapterAtFarEnd {
                    verdict: Verdict::FixtureOnly,
                },
            )
        )]
    );
}

/// A bare "yes" does not say whether a fixture or an adapter sits there, and
/// a verdict softened into words the checker cannot classify must not count
/// as an adapter just because nothing says it is not one.
#[test]
fn a_bare_yes_is_not_an_adapter_and_discharges_nothing() {
    let spec = edited(
        "**Adapter yes, since phase 10b.**",
        "**Yes, since phase 10b.**",
    );

    assert_eq!(
        findings(&spec),
        [
            at(
                line_of(&spec, "| **Transport** |"),
                Breach::UnreadableVerdict {
                    axis: s("Transport"),
                    verdict: s("Yes, since phase 10b."),
                }
            ),
            at(
                line_of(&spec, "| Transport | `EventStore`"),
                row(
                    "Transport",
                    "EventStore",
                    RowProblem::NoAdapterAtFarEnd {
                        verdict: Verdict::Unreadable,
                    },
                )
            ),
        ]
    );
}

/// The freeze #125 exists to catch: the clause carrying an axis's exposure is
/// frozen, and with it the only disclosure the axis had.
#[test]
fn a_carrying_clause_that_freezes_fails_its_ports_row() {
    let spec = edited(
        "`[PROVISIONAL — an adapter that holds only a suffix of its log]`",
        "`[FROZEN]`",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Completeness | `EventStore`"),
            row(
                "Completeness",
                "EventStore",
                RowProblem::CarrierNotOpen {
                    clause: s("ES-40"),
                    maturity: s("FROZEN"),
                },
            )
        )]
    );
}

#[test]
fn an_open_clause_of_another_port_does_not_carry_the_exposure() {
    let spec = edited("carried by ES-40", "carried by PS-6");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Completeness | `EventStore`"),
            row(
                "Completeness",
                "EventStore",
                RowProblem::CarrierOnAnotherPort { clause: s("PS-6") },
            )
        )]
    );
}

#[test]
fn a_carrying_clause_the_document_does_not_declare_is_reported() {
    let spec = edited("carried by ES-40", "carried by ES-99");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Completeness | `EventStore`"),
            row(
                "Completeness",
                "EventStore",
                RowProblem::CarrierMissing { clause: s("ES-99") },
            )
        )]
    );
}

#[test]
fn an_accepting_record_that_does_not_exist_is_reported() {
    let spec = edited("| ADR-0013 |", "| ADR-0099 |");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Async flavour | `EventStore`"),
            row(
                "Async flavour",
                "EventStore",
                RowProblem::RecordMissing { adr: s("0099") },
            )
        )]
    );
}

/// A superseded acceptance is a record that stopped accepting.
#[test]
fn a_superseded_record_no_longer_accepts_the_risk() {
    let spec = edited("| ADR-0013 |", "| ADR-0042 |");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Async flavour | `EventStore`"),
            row(
                "Async flavour",
                "EventStore",
                RowProblem::RecordNotAccepted { adr: s("0042") },
            )
        )]
    );
}

/// CF-25's second route is "name the axis and the ADR that accepts it". A
/// record that never mentions the axis names nothing.
#[test]
fn a_record_that_never_names_the_axis_does_not_accept_it() {
    let spec = edited("| ADR-0013 |", "| ADR-0050 |");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Async flavour | `EventStore`"),
            row(
                "Async flavour",
                "EventStore",
                RowProblem::RecordSilentOnAxis { adr: s("0050") },
            )
        )]
    );
}

/// CF-25's *pro forma* sentence: one port's acceptance does not discharge
/// another port's row.
#[test]
fn a_record_accepting_the_axis_for_another_port_does_not_discharge_this_one() {
    let spec = edited("| ADR-0013 |", "| ADR-0060 |");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Async flavour | `EventStore`"),
            row(
                "Async flavour",
                "EventStore",
                RowProblem::RecordSilentOnPort { adr: s("0060") },
            )
        )]
    );
}

/// CF-25's `Rule:` line: a `[FROZEN]` port clause is the trigger, whatever
/// §1.6 says. `SyncPeer` carries one, so a missing row fails.
#[test]
fn a_port_with_a_frozen_clause_and_no_ledger_row_fails() {
    let spec = edited(
        "| Transport | `SyncPeer` | carried by SY-14 | the one-shot HTTP peer |\n",
        "",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "#### SY-8"),
            Breach::UnwalkedFreeze {
                port: s("SyncPeer"),
                cause: Cause::Clauses {
                    first: s("SY-8"),
                    count: 1,
                },
            }
        )]
    );
}

/// §1.6 stays consistent with the clauses: a port declared **Frozen** owes
/// rows even when no clause of it is frozen.
#[test]
fn a_port_declared_frozen_with_no_ledger_row_fails() {
    let spec = edit(
        &edit(
            &sync_peer_unfrozen(),
            "**Shape specified, experiments deferred**",
            "**Frozen**",
        ),
        "| Transport | `SyncPeer` | carried by SY-14 | the one-shot HTTP peer |\n",
        "",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| **`SyncPeer`** |"),
            Breach::UnwalkedFreeze {
                port: s("SyncPeer"),
                cause: Cause::Declared,
            }
        )]
    );
}

/// A §1.6 lead the check cannot classify must not read as "not frozen".
#[test]
fn a_maturity_lead_outside_the_vocabulary_is_reported() {
    let spec = edited(
        "**Shape specified, experiments deferred**",
        "**Mostly settled**",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| **`SyncPeer`** |"),
            Breach::UnreadableMaturity {
                port: s("SyncPeer"),
                maturity: s("Mostly settled"),
            }
        )]
    );
}

/// A port with nothing frozen owes nothing yet, so its rows are counted and
/// not walked — even a placeholder no discharge form reads.
#[test]
fn a_port_with_nothing_frozen_has_its_rows_counted_and_not_walked() {
    let spec = edit(&sync_peer_unfrozen(), "carried by SY-14", "nothing yet");

    let (walk, found) = walked(&spec);

    assert_eq!(found, []);
    assert_eq!(walk.unwalked, 1);
}

/// A gap written down is still a gap: `undischarged` fails the gate.
#[test]
fn an_undischarged_row_of_a_frozen_port_fails() {
    let spec = edited("carried by SY-14", "undischarged");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Transport | `SyncPeer`"),
            row("Transport", "SyncPeer", RowProblem::Undischarged)
        )]
    );
}

/// A new axis must not land attributed to no port, which is how a port
/// sitting on it would escape the check without anyone deciding it does not.
#[test]
fn a_portfolio_axis_with_no_ledger_row_fails() {
    let spec = edited(
        "| **Completeness** |",
        "| **Durability** | Volatile | Survives a reopen | **Fixture yes, adapter yes.** | Fixture |\n\
         | **Completeness** |",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| **Durability** |"),
            Breach::UnattributedAxis {
                axis: s("Durability"),
            }
        )]
    );
}

/// A repeated axis would let the second verdict silently replace the first.
#[test]
fn a_portfolio_axis_carried_twice_fails() {
    let spec = edited(
        "| **Completeness** |",
        "| **Transport** | x | y | **No.** | z |\n| **Completeness** |",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| **Transport** | x |"),
            Breach::DuplicateAxis {
                axis: s("Transport"),
            }
        )]
    );
}

#[test]
fn a_ledger_row_naming_an_axis_the_portfolio_lacks_fails() {
    let spec = edited(
        "| Transport | `EventStore` |",
        "| Transprot | `EventStore` |",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Transprot |"),
            Breach::UnknownAxis {
                axis: s("Transprot"),
            }
        )]
    );
}

#[test]
fn a_ledger_row_naming_a_port_no_table_lists_fails() {
    let spec = edited(
        "| Batch shape | `ProjectionStore` |",
        "| Batch shape | `ProjectorStore` |",
    );

    assert_eq!(
        findings(&spec),
        [
            at(
                line_of(&spec, "| Batch shape | `ProjectorStore`"),
                Breach::UnknownPort {
                    port: s("ProjectorStore"),
                }
            ),
            at(
                line_of(&spec, "#### PS-1"),
                Breach::UnwalkedFreeze {
                    port: s("ProjectionStore"),
                    cause: Cause::Clauses {
                        first: s("PS-1"),
                        count: 1,
                    },
                }
            ),
        ]
    );
}

#[test]
fn a_discharge_in_none_of_the_forms_fails() {
    let spec = edited("| ADR-0013 |", "| accepted, see phase 4 |");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Async flavour | `EventStore`"),
            row(
                "Async flavour",
                "EventStore",
                RowProblem::Unrecognised {
                    cell: s("accepted, see phase 4"),
                },
            )
        )]
    );
}

#[test]
fn two_rows_for_one_port_on_one_axis_fail() {
    let spec = edited(
        "| Transport | `EventStore` | far end | Neon |\n",
        "| Transport | `EventStore` | far end | Neon |\n\
         | Transport | `EventStore` | ADR-0013 | again |\n",
    );

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Transport | `EventStore` | ADR-0013"),
            Breach::DuplicateRow {
                axis: s("Transport"),
                port: s("EventStore"),
            }
        )]
    );
}

/// A check that finds no ledger must not pass by having nothing to walk.
#[test]
fn a_missing_ledger_fails_rather_than_passing_vacuously() {
    let spec = edited(LEDGER, "#### Some other heading");

    assert_eq!(
        findings(&spec),
        [Finding {
            line: None,
            breach: Breach::MissingTable {
                what: "freeze ledger under `#### The freeze ledger` with `Axis`, `Port` and \
                       `Discharged by` columns",
            },
        }]
    );
}

/// Every bold lead the live portfolio opens a `Far end exists?` cell with,
/// read the way a person reads it, and the bare forms CF-26 refuses.
#[test]
fn each_verdict_the_live_portfolio_carries_reads_as_written() {
    let leads = [
        (
            "**Fixture yes, adapter no.** Pre-commit store",
            Verdict::FixtureOnly,
        ),
        (
            "**Adapter yes, since phase 10b, and the far end answered.**",
            Verdict::Adapter,
        ),
        (
            "**Far end adapter yes, near end no — and the suite.**",
            Verdict::Adapter,
        ),
        ("**No. Planned for phase 14** (CF-27)", Verdict::Nothing),
        (
            "**Fixture yes, adapter yes — pooling still empty.**",
            Verdict::Adapter,
        ),
        ("**Yes, since phase 10b.**", Verdict::Unreadable),
        ("**Far end yes, near end no.**", Verdict::Unreadable),
        ("Adapter yes, but not in bold", Verdict::Unreadable),
    ];

    let read: Vec<Verdict> = leads.iter().map(|(cell, _)| verdict(cell)).collect();

    assert_eq!(read, leads.map(|(_, v)| v));
}

/// A body line is prose about some other atom, even when the frontmatter
/// states no status of its own.
#[test]
fn a_status_line_below_the_frontmatter_is_not_the_atoms_status() {
    let atom = "---\nid: kb-decision-0042\n---\n\nstatus: accepted\n";

    assert_eq!(status_of(atom), None);
}

#[test]
fn a_superseded_atoms_frontmatter_reads_superseded() {
    let atom = "---\nid: kb-decision-0042\nstatus: superseded\n---\n\nstatus: accepted\n";

    assert_eq!(status_of(atom), Some("superseded"));
}

/// A root under the system temp directory, unique to this process and label,
/// removed when dropped. `xtask` carries no `tempfile` (DR-12).
struct Scratch(PathBuf);

impl Scratch {
    fn new(label: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("hs-portfolio-{label}-{}", std::process::id()));
        std::fs::create_dir_all(path.join(".kb/decisions")).expect("a temp dir is writable");
        std::fs::create_dir_all(path.join("references/adr")).expect("a temp dir is writable");
        Self(path)
    }

    fn write(&self, rel: &str, text: &str) {
        std::fs::write(self.0.join(rel), text).expect("a temp dir is writable");
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // Best effort: a leftover directory under temp is harmless, and a
        // panic in `drop` during an unwinding test would abort the run.
        if let Err(e) = std::fs::remove_dir_all(&self.0) {
            eprintln!("could not remove {}: {e}", self.0.display());
        }
    }
}

/// The production lookup: an accepted atom reads accepted and carries its
/// long form's text; a superseded one does not read accepted; a number with
/// no atom is `None`.
#[test]
fn records_under_reads_status_from_the_atom_and_text_from_both_files() {
    let root = Scratch::new("records");
    root.write(
        ".kb/decisions/0013-accepted.md",
        "---\nstatus: accepted\n---\nThe atom.\n",
    );
    root.write(
        "references/adr/0013-accepted.md",
        "| Async flavour | `EventStore` | accepted |\n",
    );
    root.write(
        ".kb/decisions/0042-superseded.md",
        "---\nstatus: superseded\n---\nAsync flavour on `EventStore`.\n",
    );
    let lookup = records_under(&root.0);

    let accepted = lookup("0013").expect("readable").expect("present");
    let superseded = lookup("0042").expect("readable").expect("present");
    let absent = lookup("0099").expect("readable");

    assert_eq!(
        (accepted.accepted, accepted.text.as_str()),
        (
            true,
            "---\nstatus: accepted\n---\nThe atom.\n| Async flavour | `EventStore` | accepted |\n"
        )
    );
    assert!(!superseded.accepted, "a superseded atom does not accept");
    assert_eq!(absent.map(|r| r.text), None);
}

/// Two records sharing a number are an error, not whichever one `read_dir`
/// happens to list first.
#[test]
fn two_records_with_one_number_are_refused() {
    let root = Scratch::new("duplicate");
    root.write(".kb/decisions/0013-one.md", "---\nstatus: accepted\n---\n");
    root.write(".kb/decisions/0013-two.md", "---\nstatus: accepted\n---\n");

    let found = numbered(&root.0.join(".kb/decisions"), "0013");

    assert!(
        found.is_err(),
        "a duplicate number must not pick a record: {found:?}"
    );
}

#[test]
fn the_summary_names_what_the_walk_did_not_read() {
    let mut out = Vec::new();
    let walk = Walk {
        axes: 7,
        frozen: vec![s("EventStore"), s("ProjectionStore")],
        far_end: 4,
        accepted: 2,
        carried: 1,
        unwalked: 0,
    };

    summarise(&mut out, &walk).expect("a Vec accepts every write");

    assert_eq!(
        String::from_utf8(out).expect("the summary is UTF-8"),
        "CF-25 portfolio: 7 axes; 2 frozen port(s) (EventStore, ProjectionStore), 7 ledger \
         row(s) discharged: 4 adapter at the far end, 2 accepted by a record, 1 carried by an \
         open clause; 0 row(s) of ports with nothing frozen not walked. Not read: the near end, \
         an axis a port sits on with no row, and what a record says beyond naming the axis and \
         the port\n"
    );
}
