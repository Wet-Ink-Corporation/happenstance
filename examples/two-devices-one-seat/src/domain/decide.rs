//! The decisions: scheduling and selling on any store, and central's rule over
//! the sales that reach it.
//!
//! Each is a plain function from the history a store holds to the events it
//! decides, so the same model runs on a device, offline, and on central.

use core::convert::Infallible;

use happenstance::DecisionModel;

use super::{BoxOffice, Place, RefundReason, Refusal, Rule, SaleId, ShowLedger};

/// The decision behind scheduling the show: one `ShowScheduled`, or nothing
/// if this store already holds one, so scheduling twice is not an error.
///
/// # Errors
///
/// None: the error type is [`Infallible`].
pub fn schedule(ledger: &ShowLedger, capacity: u32) -> Result<Vec<BoxOffice>, Infallible> {
    Ok(match ledger.capacity {
        Some(_) => Vec::new(),
        None => vec![BoxOffice::ShowScheduled {
            show: ledger.show.clone(),
            capacity,
        }],
    })
}

/// The decision behind a sale at `at`: one `SeatSold` if a seat is free in the
/// history this store holds, a [`Refusal`] if not.
///
/// The sale's id is minted here, from the same fold, so a retried decision
/// mints it again from the history it re-read.
///
/// # Errors
///
/// [`Refusal::NotScheduled`] before the show reaches this store, and
/// [`Refusal::SoldOut`] when every seat is held.
pub fn sell_seat(ledger: &ShowLedger, at: Place, buyer: &str) -> Result<Vec<BoxOffice>, Refusal> {
    let Some(capacity) = ledger.capacity else {
        return Err(Refusal::NotScheduled {
            show: ledger.show.to_string(),
        });
    };
    let held = ledger.held();
    if held >= capacity {
        return Err(Refusal::SoldOut { capacity, held });
    }
    let made_here = ledger.sales().filter(|sale| sale.sold_by == at).count();
    Ok(vec![BoxOffice::SeatSold {
        show: ledger.show.clone(),
        sale: SaleId::mint(at, made_here.saturating_add(1)),
        buyer: buyer.to_owned(),
        sold_by: at,
    }])
}

/// Central's rule, asked about one group of events a device pushed: one
/// `SaleRefunded` for each sale in `arriving` that lands past capacity, and
/// nothing otherwise.
///
/// `ledger` is the history central holds. `earlier` is what lands before
/// `arriving` in the same push, which central does not hold yet either. The
/// landing order is that history, then `earlier`, then `arriving`: the first
/// sales to reach central keep their seats.
///
/// An event central already holds is not folded twice, and a sale it already
/// holds is never refunded here, because it was judged when it first landed.
/// So a group delivered again answers nothing, whether or not the runner
/// skips it.
///
/// Takes the ledger by value: the rule folds the push into it and has no use
/// for it afterwards.
#[must_use]
pub fn adjudicate(
    mut ledger: ShowLedger,
    earlier: impl IntoIterator<Item = BoxOffice>,
    arriving: impl IntoIterator<Item = BoxOffice>,
) -> Vec<BoxOffice> {
    for event in earlier {
        land(&mut ledger, event);
    }
    let mut landed_now = Vec::new();
    for event in arriving {
        // Cloned because the id is needed twice: once in the ledger the
        // event moves into, once to know which refunds this group earns.
        if let Some(sale) = land(&mut ledger, event) {
            landed_now.push(sale);
        }
    }
    let Some(capacity) = ledger.capacity else {
        return Vec::new();
    };
    let sales_seen = ledger.held();
    let keep = usize::try_from(capacity).unwrap_or(usize::MAX);
    ledger
        .holding()
        .skip(keep)
        .filter(|sale| landed_now.contains(&sale.sale))
        .map(|sale| BoxOffice::SaleRefunded {
            show: ledger.show.clone(),
            sale: sale.sale.clone(),
            reason: RefundReason {
                capacity,
                sales_seen,
                rule: Rule::FirstToReachCentralKeepsTheSeat,
            },
        })
        .collect()
}

/// Folds `event` into `ledger` unless the ledger already holds it, and hands
/// back the id of a sale that landed now.
fn land(ledger: &mut ShowLedger, event: BoxOffice) -> Option<SaleId> {
    let (held, sold) = match &event {
        BoxOffice::ShowScheduled { .. } => (ledger.capacity.is_some(), None),
        BoxOffice::SeatSold { sale, .. } => (
            ledger.sales.iter().any(|known| known.sale == *sale),
            Some(sale.clone()),
        ),
        BoxOffice::SaleRefunded { sale, .. } => (ledger.refunds.contains_key(sale), None),
    };
    if held {
        return None;
    }
    ledger.apply(event);
    sold
}

#[cfg(test)]
mod tests;
