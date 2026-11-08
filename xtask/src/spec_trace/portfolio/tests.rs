//! Each test hands [`check`] a document one edit away from the shape the tree
//! has today, and names the wrong input that edit is.

use super::*;
use crate::spec_trace::parse_clauses;

/// The tree's shape at #125, cut to what the check reads: §1.4's prefixes,
/// §1.6's three ports, one open `ES` clause, four axes and their ledger rows.
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

#### ES-40 — a condition over removed history does not reject
`[PROVISIONAL — an adapter that holds only a suffix of its log]`
Rejects: a store that refuses.

#### PS-6 — a buffering adapter's begin resolves at its first poll
`[PROVISIONAL — an adapter that must reserve from its server]`
Rejects: a round trip in begin.

#### The portfolio

| Axis | Near end | Far end | Far end exists? | Kind of instrument needed |
|---|---|---|---|---|
| **Transport** | In-process | One-shot HTTP | **Yes, since phase 10b.** Neon | Adapter |
| **Async flavour** | `Send` | `!Send` | **Fixture yes, adapter no.** Local store | Fixture |
| **Batch shape** (`ProjectionStore`) | Live | Replayed | **Far end yes, near end no.** Sqlite | Adapter |
| **Completeness** | Whole log | A suffix | **No. Planned for phase 14** | Fixture |

#### The freeze ledger

| Axis | Port | Discharged by | Why |
|---|---|---|---|
| Transport | `EventStore` | far end | Neon |
| Async flavour | `EventStore` | ADR-0013 | accepted |
| Batch shape | `ProjectionStore` | far end | Sqlite |
| Completeness | `EventStore` | carried by ES-40 | ES-40's marker |
";

/// The records the tree holds, as far as [`TODAY`] cites them.
fn records(number: &str) -> Option<Record> {
    match number {
        "0013" => Some(Record {
            accepted: true,
            text: "| **Async flavour** | Fixture | **Accepted by this ADR** |".to_owned(),
        }),
        "0042" => Some(Record {
            accepted: false,
            text: "Async flavour, accepted, then superseded".to_owned(),
        }),
        "0050" => Some(Record {
            accepted: true,
            text: "a record about something else entirely".to_owned(),
        }),
        _ => None,
    }
}

/// The findings [`check`] reports against `spec`.
fn findings(spec: &str) -> Vec<Finding> {
    check(spec, &parse_clauses(spec), |n| Ok(records(n)))
        .expect("the stand-in records never fail to read")
        .1
}

/// `spec` with `from` replaced once; panics when `from` is absent, so an edit
/// that silently stops applying cannot turn a test green.
fn edited(from: &str, to: &str) -> String {
    assert!(
        TODAY.contains(from),
        "the fixture no longer carries {from:?}"
    );
    TODAY.replacen(from, to, 1)
}

/// One finding on `line`.
fn at(line: usize, breach: Breach) -> Finding {
    Finding {
        line: Some(line),
        breach,
    }
}

/// The 1-based line of the first line of [`TODAY`]-derived `spec` containing `needle`.
fn line_of(spec: &str, needle: &str) -> usize {
    spec.lines()
        .position(|l| l.contains(needle))
        .expect("the needle is in the fixture")
        + 1
}

fn s(text: &str) -> String {
    text.to_owned()
}

#[test]
fn the_shape_the_tree_has_today_passes_and_is_counted() {
    let (walk, found) =
        check(TODAY, &parse_clauses(TODAY), |n| Ok(records(n))).expect("records read");

    assert_eq!(found, []);
    assert_eq!(
        walk,
        Walk {
            axes: 4,
            frozen: vec![s("EventStore"), s("ProjectionStore")],
            far_end: 2,
            accepted: 1,
            carried: 1,
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
            Breach::NoAdapterAtFarEnd {
                axis: s("Async flavour"),
                port: s("EventStore"),
                verdict: Verdict::FixtureOnly,
            }
        )]
    );
}

/// A verdict softened into words the checker cannot classify must not count
/// as an adapter just because nothing says it is not one.
#[test]
fn a_verdict_outside_the_vocabulary_is_reported_and_discharges_nothing() {
    let spec = edited("**Yes, since phase 10b.**", "**Mostly, we think.**");

    assert_eq!(
        findings(&spec),
        [
            at(
                line_of(&spec, "| **Transport** |"),
                Breach::UnreadableVerdict {
                    axis: s("Transport"),
                    verdict: s("Mostly, we think."),
                }
            ),
            at(
                line_of(&spec, "| Transport | `EventStore`"),
                Breach::NoAdapterAtFarEnd {
                    axis: s("Transport"),
                    port: s("EventStore"),
                    verdict: Verdict::Unreadable,
                }
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
            Breach::CarrierNotOpen {
                axis: s("Completeness"),
                port: s("EventStore"),
                clause: s("ES-40"),
                maturity: s("FROZEN"),
            }
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
            Breach::CarrierOnAnotherPort {
                axis: s("Completeness"),
                port: s("EventStore"),
                clause: s("PS-6"),
            }
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
            Breach::CarrierMissing {
                axis: s("Completeness"),
                port: s("EventStore"),
                clause: s("ES-99"),
            }
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
            Breach::RecordMissing {
                axis: s("Async flavour"),
                port: s("EventStore"),
                adr: s("0099"),
            }
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
            Breach::RecordNotAccepted {
                axis: s("Async flavour"),
                port: s("EventStore"),
                adr: s("0042"),
            }
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
            Breach::RecordSilentOnAxis {
                axis: s("Async flavour"),
                port: s("EventStore"),
                adr: s("0050"),
            }
        )]
    );
}

/// The phase 13 freeze: `SyncPeer` declared frozen and nothing walked.
#[test]
fn declaring_a_port_frozen_with_no_ledger_row_fails() {
    let spec = edited("**Shape specified, experiments deferred**", "**Frozen**");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| **`SyncPeer`** |"),
            Breach::UnwalkedFreeze {
                port: s("SyncPeer"),
            }
        )]
    );
}

/// A clause count is not the trigger (§1.3): an undeclared port's rows owe
/// nothing yet, so a placeholder there is not walked.
#[test]
fn an_undeclared_ports_rows_are_counted_and_not_walked() {
    let spec = edited(
        "| Completeness | `EventStore` | carried by ES-40 | ES-40's marker |\n",
        "| Completeness | `EventStore` | carried by ES-40 | ES-40's marker |\n\
         | Transport | `SyncPeer` | nothing yet | no network peer |\n",
    );

    let (walk, found) =
        check(&spec, &parse_clauses(&spec), |n| Ok(records(n))).expect("records read");

    assert_eq!(found, []);
    assert_eq!(walk.unwalked, 1);
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

#[test]
fn a_ledger_row_naming_an_axis_the_portfolio_lacks_fails() {
    let spec = edited(
        "| Transport | `EventStore` |",
        "| Transprot | `EventStore` |",
    );

    assert_eq!(
        findings(&spec),
        [
            at(
                line_of(&spec, "| Transprot |"),
                Breach::UnknownAxis {
                    axis: s("Transprot"),
                }
            ),
            at(
                line_of(&spec, "| **Transport** |"),
                Breach::UnattributedAxis {
                    axis: s("Transport"),
                }
            ),
        ]
    );
}

#[test]
fn a_ledger_row_naming_a_port_section_1_6_lacks_fails() {
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
                line_of(&spec, "| **`ProjectionStore`** |"),
                Breach::UnwalkedFreeze {
                    port: s("ProjectionStore"),
                }
            ),
        ]
    );
}

#[test]
fn a_discharge_in_none_of_the_three_forms_fails() {
    let spec = edited("| ADR-0013 |", "| accepted, see phase 4 |");

    assert_eq!(
        findings(&spec),
        [at(
            line_of(&spec, "| Async flavour | `EventStore`"),
            Breach::UnrecognisedDischarge {
                axis: s("Async flavour"),
                port: s("EventStore"),
                cell: s("accepted, see phase 4"),
            }
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
/// read the way a person reads it.
#[test]
fn each_verdict_the_live_portfolio_carries_reads_as_written() {
    let leads = [
        (
            "**Fixture yes, adapter no.** Pre-commit store",
            Verdict::FixtureOnly,
        ),
        (
            "**Yes, since phase 10b, and the far end answered.**",
            Verdict::Adapter,
        ),
        (
            "**Far end yes, near end no — and the suite.**",
            Verdict::Adapter,
        ),
        ("**No. Planned for phase 14** (CF-27)", Verdict::Nothing),
        (
            "**Fixture yes, adapter yes — pooling still empty.**",
            Verdict::Adapter,
        ),
        (
            "**Fixture yes, adapter yes — fault far end still empty.**",
            Verdict::Adapter,
        ),
        ("Yes, but not in bold", Verdict::Unreadable),
    ];

    let read: Vec<Verdict> = leads.iter().map(|(cell, _)| verdict(cell)).collect();

    assert_eq!(read, leads.map(|(_, v)| v));
}

#[test]
fn a_superseded_atoms_frontmatter_does_not_read_accepted() {
    let atom = "---\nid: kb-decision-0042\nstatus: superseded\n---\n\nstatus: accepted\n";

    assert_eq!(status_of(atom), Some("superseded"));
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
        "CF-25 portfolio: 7 axes; 2 port(s) declared frozen (EventStore, ProjectionStore), \
         7 ledger row(s) walked: 4 adapter at the far end, 2 accepted by a record, 1 carried \
         by an open clause; 0 row(s) of undeclared ports not walked. Not read: the near end, \
         an axis a port sits on with no row, and what a record says beyond naming the axis\n"
    );
}
