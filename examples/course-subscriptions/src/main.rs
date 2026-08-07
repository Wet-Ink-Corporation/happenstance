//! The canonical DCB worked example: students subscribing to courses.
//!
//! Three invariants, none of which fits inside a single aggregate:
//!
//! 1. a course may not be defined twice;
//! 2. a course may not exceed its capacity;
//! 3. a student may not subscribe to the same course twice.
//!
//! Invariant 2 spans every subscription for a course. Invariant 3 spans one
//! student *and* one course. Classical event sourcing forces a choice here:
//! make `Course` the aggregate and invariant 3 needs a read model plus a saga,
//! or make the pair the aggregate and invariant 2 has nowhere to live.
//!
//! DCB dissolves the problem. Each handler reads exactly the events its
//! decision depends on, notes where it read to, and appends conditioned on
//! nothing matching that same query having appeared since. The consistency
//! boundary is drawn per decision, and it is drawn by the query.
//!
//! Run with `cargo run -p course-subscriptions`.

#![allow(clippy::print_stdout)]

use anyhow::{Result, bail};
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, MemoryEventStore, Query, QueryItem,
    SequencePosition, Tags, read_decision_model,
};

const COURSE_DEFINED: &str = "CourseDefined";
const STUDENT_SUBSCRIBED: &str = "StudentSubscribed";
const STUDENT_UNSUBSCRIBED: &str = "StudentUnsubscribed";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let store = MemoryEventStore::new();

    println!("== defining course c1 with capacity 2 ==");
    define_course(&store, "c1", 2).await?;
    println!("   defined");

    println!("\n== defining course c1 again ==");
    match define_course(&store, "c1", 5).await {
        Ok(()) => bail!("a duplicate course definition should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    println!("\n== subscribing s1 and s2 ==");
    subscribe(&store, "c1", "s1").await?;
    subscribe(&store, "c1", "s2").await?;
    println!("   both subscribed");

    println!("\n== subscribing s1 again ==");
    match subscribe(&store, "c1", "s1").await {
        Ok(()) => bail!("a duplicate subscription should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    println!("\n== subscribing s3, which would exceed capacity ==");
    match subscribe(&store, "c1", "s3").await {
        Ok(()) => bail!("exceeding capacity should have been rejected"),
        Err(err) => println!("   rejected: {err}"),
    }

    println!("\n== s1 unsubscribes, freeing a seat ==");
    unsubscribe(&store, "c1", "s1").await?;
    subscribe(&store, "c1", "s3").await?;
    println!("   s3 subscribed into the freed seat");

    println!("\n== final log ==");
    for event in store.snapshot() {
        println!(
            "   {:>3}  {:<22} {:?}",
            event.position.get(),
            event.event_type().as_str(),
            event.tags()
        );
    }

    Ok(())
}

/// Defines a course, rejecting a second definition of the same one.
///
/// The consistency boundary is "any `CourseDefined` event tagged with this
/// course" — the smallest set of events that could invalidate the decision.
async fn define_course(store: &MemoryEventStore, course: &str, capacity: u32) -> Result<()> {
    let query = Query::from_item(QueryItem::new(
        [COURSE_DEFINED],
        Tags::from_pairs([("course", course)])?,
    )?)?;

    let (existing, last_seen) = read_decision_model(store, &query).await?;
    if !existing.is_empty() {
        bail!("course {course} is already defined");
    }

    let event = Event::new(
        COURSE_DEFINED,
        format!("{{\"capacity\":{capacity}}}").into_bytes(),
    )?
    .with_tags(Tags::from_pairs([("course", course)])?);

    commit(store, &[event], &query, last_seen).await
}

/// Subscribes a student, enforcing capacity and no-double-subscription
/// together.
///
/// This is the case that motivates DCB. One query spans the course definition,
/// every subscription to the course, and this student's own history — three
/// things that would be three aggregates — and one append condition covers all
/// of it.
async fn subscribe(store: &MemoryEventStore, course: &str, student: &str) -> Result<()> {
    let query = Query::from_items([
        // The capacity, and everyone currently holding a seat.
        QueryItem::new(
            [COURSE_DEFINED, STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED],
            Tags::from_pairs([("course", course)])?,
        )?,
        // This student's history with this course.
        QueryItem::new(
            [STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED],
            Tags::from_pairs([("course", course), ("student", student)])?,
        )?,
    ])?;

    let (events, last_seen) = read_decision_model(store, &query).await?;

    // Fold the events into the decision model.
    let mut capacity = None;
    let mut seats_taken = 0i64;
    let mut already_subscribed = false;

    for sequenced in &events {
        let tags = sequenced.event.tags();
        let is_this_student = tags
            .iter()
            .any(|tag| tag.key() == Some("student") && tag.value() == Some(student));

        match sequenced.event_type().as_str() {
            COURSE_DEFINED => capacity = Some(parse_capacity(sequenced.event.data())),
            STUDENT_SUBSCRIBED => {
                seats_taken += 1;
                if is_this_student {
                    already_subscribed = true;
                }
            }
            STUDENT_UNSUBSCRIBED => {
                seats_taken -= 1;
                if is_this_student {
                    already_subscribed = false;
                }
            }
            _ => {}
        }
    }

    let Some(capacity) = capacity else {
        bail!("course {course} does not exist");
    };
    if already_subscribed {
        bail!("student {student} is already subscribed to {course}");
    }
    if seats_taken >= i64::from(capacity) {
        bail!("course {course} is full ({seats_taken}/{capacity})");
    }

    let event = Event::new(STUDENT_SUBSCRIBED, &b"{}"[..])?.with_tags(Tags::from_pairs([
        ("course", course),
        ("student", student),
    ])?);

    commit(store, &[event], &query, last_seen).await
}

/// Releases a student's seat.
async fn unsubscribe(store: &MemoryEventStore, course: &str, student: &str) -> Result<()> {
    let query = Query::from_item(QueryItem::new(
        [STUDENT_SUBSCRIBED, STUDENT_UNSUBSCRIBED],
        Tags::from_pairs([("course", course), ("student", student)])?,
    )?)?;

    let (events, last_seen) = read_decision_model(store, &query).await?;

    let subscribed = events
        .last()
        .is_some_and(|event| event.event_type().as_str() == STUDENT_SUBSCRIBED);
    if !subscribed {
        bail!("student {student} is not subscribed to {course}");
    }

    let event = Event::new(STUDENT_UNSUBSCRIBED, &b"{}"[..])?.with_tags(Tags::from_pairs([
        ("course", course),
        ("student", student),
    ])?);

    commit(store, &[event], &query, last_seen).await
}

/// Appends `events`, conditioned on nothing matching `query` having appeared
/// since `last_seen`.
///
/// This is the second half of every DCB command handler, and it is identical
/// every time — which is exactly why it belongs in the typed layer rather than
/// in each handler. That is `happenstance`'s job (ADR-0006); it is a facade over
/// the contract today, so the loop is spelled out here.
async fn commit(
    store: &MemoryEventStore,
    events: &[Event],
    query: &Query,
    last_seen: Option<SequencePosition>,
) -> Result<()> {
    let condition = AppendCondition::new(query.clone()).after_opt(last_seen);

    match store.append(events, Some(&condition)).await {
        Ok(_) => Ok(()),
        // Under real contention this is where a retry loop would go: rebuild
        // the decision model from the current state and try again.
        Err(AppendError::ConditionViolated(_)) => {
            bail!("concurrent modification: the decision model is stale, retry")
        }
        Err(err) => bail!("append failed: {err}"),
    }
}

/// Extracts `capacity` from a `CourseDefined` payload.
///
/// Hand-rolled because the contract layer stores opaque bytes and this example
/// deliberately takes no serialisation dependency — decoding is the typed
/// layer's job, and that layer does not exist yet.
fn parse_capacity(data: &[u8]) -> u32 {
    core::str::from_utf8(data)
        .ok()
        .and_then(|text| text.split(':').nth(1))
        .and_then(|tail| tail.trim_end_matches('}').trim().parse().ok())
        .unwrap_or(0)
}
