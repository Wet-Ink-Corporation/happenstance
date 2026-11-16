//! The story, told beat by beat into any writer.
//!
//! Written against `io::Write` rather than to stdout, so the binary hands it a
//! locked stdout and a test hands it a `Vec<u8>`. Nothing printed depends on
//! the run: no store id, no path, no time, so the transcript can be quoted.

use std::io::Write;
use std::path::Path;

use happenstance::CommandOutcome;
use happenstance_sync::FailureClass;

use crate::domain::{Device, Place, ShowLedger};
use crate::error::StoryError;
use crate::readback::Converged;
use crate::scenario::{CAPACITY, SHOW, Scenario, Synced};
use crate::timeline::{Row, Status, TimelineView};
use crate::uplink::Link;

/// Tells the whole story into `out` over three files under `dir`, and returns
/// what the files held when they were read back with every handle dropped.
///
/// # Errors
///
/// Any [`StoryError`]: a file that will not open, a decision or sync that
/// fails in a way the story did not expect, or a write to `out` that fails.
pub async fn tell<W: Write>(dir: &Path, out: &mut W) -> Result<Converged, StoryError> {
    // One scope owns every handle, so all of them are closed before the
    // read-back opens the files again.
    {
        let mut scenario = Scenario::open(dir)?;
        the_show(&mut scenario, out).await?;
        offline(&mut scenario, out).await?;
        back_online(&mut scenario, out).await?;
        converge(&mut scenario, out).await?;
    }
    let converged = Converged::read(dir).await?;
    read_back(&converged, out)?;
    try_again(dir, out).await?;
    Ok(converged)
}

/// Central schedules the show and sells two seats; both devices sync down.
async fn the_show<W: Write>(scenario: &mut Scenario, out: &mut W) -> Result<(), StoryError> {
    writeln!(out, "== the show ==")?;
    scenario.schedule(CAPACITY).await?;
    writeln!(out, "   central schedules {SHOW:?} with {CAPACITY} seats")?;
    for buyer in ["Ada", "Brian"] {
        let sale = scenario.sell(Place::Central, buyer).await?;
        writeln!(out, "   central sells {sale} to {buyer}")?;
    }
    for device in [Device::Tablet, Device::Phone] {
        let synced = scenario.sync(device).await?;
        synced_line(out, device, synced)?;
    }
    for at in Place::ALL {
        if scenario.ledger(at).await?.held() != 2 {
            return Err(StoryError::Contradicted {
                claim: "every place holds two sales",
            });
        }
    }
    writeln!(
        out,
        "   every place holds 2 of {CAPACITY} seats sold; one is left"
    )?;
    Ok(())
}

/// The signal drops, both devices sell the last seat, and a sync fails
/// without losing anything.
async fn offline<W: Write>(scenario: &mut Scenario, out: &mut W) -> Result<(), StoryError> {
    writeln!(out, "\n== the signal drops ==")?;
    for device in [Device::Tablet, Device::Phone] {
        scenario.set_link(device, Link::Down);
        writeln!(out, "   {device}: no signal")?;
    }

    writeln!(out, "\n== both devices sell the last seat ==")?;
    for (device, buyer) in [(Device::Tablet, "Grace"), (Device::Phone, "Linus")] {
        let at = Place::from(device);
        let sale = scenario.sell(at, buyer).await?;
        let held = scenario.ledger(at).await?.held();
        writeln!(
            out,
            "   {device} sells {sale} to {buyer}: accepted, {held} of {CAPACITY} held as the {device} knows it"
        )?;
    }
    match scenario.sync(Device::Tablet).await {
        Err(error) => match error.failure() {
            Some(failure) if failure.class == FailureClass::Transient => writeln!(
                out,
                "   the tablet tries to sync: no signal, {failure};\n   \
                 the edge keeps its place, and the sale waits on the tablet"
            )?,
            _ => return Err(error),
        },
        Ok(_) => {
            return Err(StoryError::Contradicted {
                claim: "a sync with no signal fails",
            });
        }
    }
    Ok(())
}

/// The signal returns: both devices push, central holds four sales for three
/// seats, and central keeps its rule.
async fn back_online<W: Write>(scenario: &mut Scenario, out: &mut W) -> Result<(), StoryError> {
    writeln!(out, "\n== the signal returns ==")?;
    for device in [Device::Tablet, Device::Phone] {
        scenario.set_link(device, Link::Up);
        let synced = scenario.sync(device).await?;
        synced_line(out, device, synced)?;
    }

    writeln!(out, "\n== central sees the whole history ==")?;
    let ledger = scenario.ledger(Place::Central).await?;
    history(out, &ledger)?;
    writeln!(
        out,
        "   {} sales for {CAPACITY} seats: each device was right about what it knew,",
        ledger.held()
    )?;
    writeln!(out, "   and ingest refused neither (SY-1)")?;

    writeln!(out, "\n== central keeps its rule ==")?;
    if scenario.adjudicate().await? == CommandOutcome::Nothing {
        return Err(StoryError::Contradicted {
            claim: "central refunds the seat sold twice",
        });
    }
    let ledger = scenario.ledger(Place::Central).await?;
    for sale in ledger.sales() {
        if let Some(reason) = ledger.refund_reason(&sale.sale) {
            writeln!(
                out,
                "   refunded {} ({}, sold at {}): {reason}",
                sale.sale, sale.buyer, sale.sold_by
            )?;
        }
    }
    if scenario.adjudicate().await? != CommandOutcome::Nothing {
        return Err(StoryError::Contradicted {
            claim: "central's rule, applied twice, refunds once",
        });
    }
    writeln!(out, "   central applies its rule again: nothing to do")?;
    writeln!(
        out,
        "   {} of {CAPACITY} seats held, as central knows it",
        ledger.held()
    )?;
    Ok(())
}

/// Both devices pull the refund, and every place projects the same view.
async fn converge<W: Write>(scenario: &mut Scenario, out: &mut W) -> Result<(), StoryError> {
    writeln!(out, "\n== everyone converges ==")?;
    for device in [Device::Tablet, Device::Phone] {
        let synced = scenario.sync(device).await?;
        synced_line(out, device, synced)?;
    }

    writeln!(out, "\n== the same view, three places ==")?;
    scenario.project().await?;
    for at in Place::ALL {
        let view = scenario.view(at)?;
        view_line(out, at, &view)?;
    }
    let view = scenario.view(Place::Central)?;
    for row in view
        .rows
        .iter()
        .filter(|row| row.status == Status::Refunded)
    {
        writeln!(out, "   audit: {}", audit(row))?;
    }
    Ok(())
}

/// What the three files hold, read back with every handle dropped.
fn read_back<W: Write>(converged: &Converged, out: &mut W) -> Result<(), StoryError> {
    writeln!(out, "\n== every handle dropped; reading the files back ==")?;
    for (at, replica) in converged.replicas() {
        if replica
            .refunds
            .iter()
            .any(|id| id.store() != converged.central.store)
        {
            return Err(StoryError::Contradicted {
                claim: "central wrote every refund (SY-7)",
            });
        }
        let refunds = u32::try_from(replica.refunds.len()).unwrap_or(u32::MAX);
        writeln!(
            out,
            "   {at:<8} {} events, {}, written by central",
            replica.ids.len(),
            plural(refunds, "refund", "refunds")
        )?;
    }
    if !converged.agrees() {
        return Err(StoryError::Contradicted {
            claim: "the three files agree",
        });
    }
    writeln!(
        out,
        "   the three logs hold the same {} events; the three read models are identical",
        converged.central.ids.len()
    )?;
    Ok(())
}

/// The phone, reopened and offline, decides against the complete history.
async fn try_again<W: Write>(dir: &Path, out: &mut W) -> Result<(), StoryError> {
    writeln!(out, "\n== the phone tries again ==")?;
    let scenario = Scenario::open(dir)?;
    scenario.set_link(Device::Phone, Link::Down);
    match scenario.sell(Place::Phone, "Margaret").await {
        Err(error) => match error.refusal() {
            Some(refusal) => writeln!(
                out,
                "   the phone, offline, sells to Margaret: refused, {refusal}"
            )?,
            None => return Err(error),
        },
        Ok(_) => {
            return Err(StoryError::Contradicted {
                claim: "the phone refuses a seat central has kept",
            });
        }
    }
    writeln!(
        out,
        "   no round trip: the history it holds is the complete one"
    )?;
    Ok(())
}

fn synced_line<W: Write>(out: &mut W, device: Device, synced: Synced) -> Result<(), StoryError> {
    writeln!(
        out,
        "   {device} syncs: {} in, {} out",
        events(synced.pulled),
        events(synced.landed)
    )?;
    Ok(())
}

fn history<W: Write>(out: &mut W, ledger: &ShowLedger) -> Result<(), StoryError> {
    for sale in ledger.sales() {
        writeln!(
            out,
            "   {:<10} {:<6} sold at {}",
            sale.sale.as_str(),
            sale.buyer,
            sale.sold_by
        )?;
    }
    Ok(())
}

fn view_line<W: Write>(out: &mut W, at: Place, view: &TimelineView) -> Result<(), StoryError> {
    let Some(capacity) = view.capacity else {
        writeln!(out, "   {at:<8} the show is not scheduled here")?;
        return Ok(());
    };
    writeln!(
        out,
        "   {at:<8} {} of {capacity} seats held, {} sales, {}",
        view.held(),
        view.sales(),
        plural(view.refunds(), "refund", "refunds")
    )?;
    Ok(())
}

/// The audit line, from the row's own data: the reason travelled with the
/// refund, so every replica prints the same words. A refund whose sale has not
/// arrived yet has no buyer or place, and says so.
fn audit(row: &Row) -> String {
    let sold = match (&row.buyer, row.sold_by) {
        (Some(buyer), Some(at)) => format!("{buyer}, sold at {at}"),
        _ => "its sale not yet here".to_owned(),
    };
    let reason = row.reason.as_deref().unwrap_or("no reason recorded");
    format!("{} ({sold}): {reason}", row.sale)
}

fn events(n: usize) -> String {
    if n == 1 {
        "1 event".to_owned()
    } else {
        format!("{n} events")
    }
}

fn plural(n: u32, one: &str, many: &str) -> String {
    if n == 1 {
        format!("1 {one}")
    } else {
        format!("{n} {many}")
    }
}
