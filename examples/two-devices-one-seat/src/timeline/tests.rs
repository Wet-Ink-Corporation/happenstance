//! The module's claim, held to account: the same table under any delivery
//! order.
//!
//! The story can never show this. Every log it builds holds a sale before the
//! refund that names it, because central decides the refund after ingesting
//! the sale. So this test writes the logs a store could hold in some other
//! order, the refund ahead of its sale among them, and projects each once.

use std::path::{Path, PathBuf};
use std::{fs, io};

use happenstance::{DecisionModel, DomainEvent, Event, EventStore, Json};

use crate::domain::{BoxOffice, Place, ShowId, ShowLedger, adjudicate, schedule, sell_seat};
use crate::scenario::Site;
use crate::timeline::TimelineView;

/// A directory of this test's own, removed when the test ends.
struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new(test: &str) -> io::Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "happenstance-two-devices-unit-{}-{test}",
            std::process::id()
        ));
        match fs::remove_dir_all(&path) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => return Err(error),
            _ => {}
        }
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.path) {
            eprintln!("could not remove {}: {error}", self.path.display());
        }
    }
}

/// The story's events in miniature, decided by the real decisions: a show of
/// one seat, a sale on each device, and central's refund of the later one.
/// Returned in the order central's log holds them.
fn the_oversell(show: &ShowId) -> Vec<BoxOffice> {
    let empty = ShowLedger::new(show);
    let scheduled = schedule(&empty, 1).expect("scheduling is infallible");
    let on = |at: Place| {
        let mut ledger = empty.clone();
        scheduled.iter().cloned().for_each(|e| ledger.apply(e));
        sell_seat(&ledger, at, at.as_str()).expect("a free seat where it is sold")
    };
    let mut log = scheduled.clone();
    log.extend(on(Place::Tablet));
    log.extend(on(Place::Phone));
    let mut central = empty.clone();
    log.iter().cloned().for_each(|e| central.apply(e));
    log.extend(adjudicate(&central).expect("adjudication is infallible"));
    log
}

/// Appends `events` to a fresh file under `dir` in the order given, projects
/// them once, and reads the view back.
async fn delivered_as(dir: &Path, show: &ShowId, events: &[&BoxOffice]) -> TimelineView {
    fs::create_dir_all(dir).expect("a directory for this order");
    let site = Site::open(dir, Place::Central).expect("a fresh file");
    let encoded: Vec<Event> = events
        .iter()
        .map(|event| {
            let data = event.encode(&Json).expect("the payload encodes");
            Event::new(event.event_type(), data)
                .expect("a declared event type")
                .with_tags(event.tags())
        })
        .collect();
    site.events
        .append(&encoded, None)
        .await
        .expect("an unconditional append");
    site.project(Place::Central, show).await.expect("projected");
    site.view().expect("the view")
}

#[tokio::test]
async fn the_view_is_the_same_under_any_delivery_order() {
    let dir = Scratch::new("delivery-order").expect("a scratch directory");
    let show = ShowId::new("premiere").expect("a valid show");
    let log = the_oversell(&show);
    let [scheduled, sold_t, sold_p, refund] = log.as_slice() else {
        panic!("the oversell is four events: {log:?}");
    };
    assert!(
        matches!(refund, BoxOffice::SaleRefunded { sale, .. } if sale.as_str() == "phone-1"),
        "central refunds the later sale: {refund:?}"
    );

    let in_order = delivered_as(
        &dir.path().join("in-order"),
        &show,
        &[scheduled, sold_t, sold_p, refund],
    )
    .await;
    assert_eq!(
        (in_order.sales(), in_order.held(), in_order.refunds()),
        (2, 1, 1),
        "in order: {in_order:?}"
    );

    for (name, order) in [
        ("refund-first", [refund, scheduled, sold_t, sold_p]),
        ("reversed", [refund, sold_p, sold_t, scheduled]),
        (
            "refund-before-its-sale",
            [scheduled, sold_t, refund, sold_p],
        ),
    ] {
        let view = delivered_as(&dir.path().join(name), &show, &order).await;
        assert_eq!(view, in_order, "{name}: a different table");
    }
}
