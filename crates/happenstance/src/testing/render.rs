//! The four-region failure message a failed `then` panics with.
//!
//! It lives in its own function rather than inside [`Decision::then`] so that
//! the string can be asserted directly, at the string level, instead of only
//! through `catch_unwind`. What must **not** move here is the panic itself:
//! `#[track_caller]` does not propagate through a helper's own frame, so the
//! attribute and the `panic!` have to sit together on `then`.
//!
//! [`Decision::then`]: super::Decision::then

use happenstance_core::{EventType, Query, SequencePosition, Tag};

/// The most event rows any one region prints.
const MAX_ROWS: usize = 8;

/// The widest line this message ever emits.
const MAX_COLUMNS: usize = 80;

/// The indent every ordinary row carries.
const ROW: &str = "  ";

/// The marker column region four's rows carry.
///
/// It is the leftmost thing on the row and it never moves, so a reader scanning
/// a terminal dump can find the diagnosis without the region header in view.
const NOT_SELECTED: &str = "  not selected  ";

/// Where a wrapped row's continuation lines start.
const CONTINUATION: &str = "      ";

/// The header sentence, fixed so a `#[should_panic]` match is stable.
pub(crate) const HEADER: &str = "assertion failed: the decision emitted different events";

/// What region four prints when nothing was seeded at all.
///
/// The region **stays and says so**. A region that disappeared would read as
/// "the DSL has nothing to say about the filter" when it means "there was
/// nothing to filter" — the same principle the testkit's declined capabilities
/// already follow, which print their reason rather than vanishing.
pub(crate) const NOTHING_SEEDED: &str = "(nothing was seeded)";

/// One event, as the message renders it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Row {
    /// The position the store assigned it. Never truncated to fit.
    pub(crate) position: SequencePosition,
    /// The type it carries. Never truncated to fit.
    pub(crate) event_type: EventType,
}

impl Row {
    /// `{position}  {event_type}`, before wrapping.
    fn body(&self) -> String {
        format!("{}  {}", self.position, self.event_type.as_str())
    }
}

/// Renders the four labelled regions, in the order the design fixes.
///
/// `expected` and `actual` arrive already `Debug`-rendered, because `Decision`
/// is the only thing that knows the event type and the renderer is deliberately
/// not generic over it.
pub(crate) fn events_differ(
    expected: &[String],
    actual: &[String],
    selected: &[Row],
    unselected: &[Row],
    query: &Query,
) -> String {
    let mut out = String::from(HEADER);
    out.push('\n');

    values(&mut out, "expected:", expected);
    values(&mut out, "actual:", actual);
    rows(&mut out, "selected by the model's query:", selected, ROW);
    seeded_but_not_selected(&mut out, unselected);
    derived_query(&mut out, query);

    out
}

/// A region of already-rendered values.
fn values(out: &mut String, label: &str, items: &[String]) {
    out.push_str(label);
    out.push('\n');
    for item in items.iter().take(MAX_ROWS) {
        wrapped(out, ROW, item);
    }
    overflow(out, items.len());
}

/// A region of event rows.
fn rows(out: &mut String, label: &str, held: &[Row], marker: &str) {
    out.push_str(label);
    out.push('\n');
    for row in held.iter().take(MAX_ROWS) {
        wrapped(out, marker, &row.body());
    }
    overflow(out, held.len());
}

/// The last region, and the reason this message exists.
///
/// It truncates **last**: `selected by the model's query:` yields first,
/// because the diagnosis a reader needs is what their query did *not* pick up.
fn seeded_but_not_selected(out: &mut String, unselected: &[Row]) {
    out.push_str("seeded but NOT selected:");
    out.push('\n');

    if unselected.is_empty() {
        wrapped(out, ROW, NOTHING_SEEDED);
        return;
    }

    for row in unselected.iter().take(MAX_ROWS) {
        wrapped(out, NOT_SELECTED, &row.body());
    }
    overflow(out, unselected.len());
}

/// The recessive closing line: the query that did the filtering.
fn derived_query(out: &mut String, query: &Query) {
    wrapped(out, "", &format!("derived query: {}", describe(query)));
}

/// `… and N more`, never silence.
fn overflow(out: &mut String, total: usize) {
    if total > MAX_ROWS {
        let more = total - MAX_ROWS;
        wrapped(out, ROW, &format!("… and {more} more"));
    }
}

/// The query, named rather than dumped.
///
/// No address, no timestamp and no hash anywhere in it, so a
/// `#[should_panic(expected = …)]` against this message is stable.
fn describe(query: &Query) -> String {
    let Some(items) = query.items() else {
        return "every event (Query::all)".to_owned();
    };

    let mut parts = Vec::with_capacity(items.len());
    for item in items {
        let types: Vec<&str> = item.types().iter().map(EventType::as_str).collect();
        let tags: Vec<&str> = item.tags().iter().map(Tag::as_str).collect();
        parts.push(format!(
            "types [{}] tags [{}]",
            types.join(", "),
            tags.join(", ")
        ));
    }
    parts.join(" OR ")
}

/// Emits `text` under `first`, wrapping so that no line exceeds the budget.
///
/// It breaks at a space where one is available inside the budget and mid-word
/// where none is. Nothing is ever cut: a position and an event type are the
/// identity of a row, so what yields is the **line**, never the value.
fn wrapped(out: &mut String, first: &str, text: &str) {
    let held: Vec<char> = text.chars().collect();
    let mut start = 0_usize;
    let mut prefix = first;

    loop {
        let room = MAX_COLUMNS.saturating_sub(prefix.chars().count()).max(1);

        if held.len() - start <= room {
            out.push_str(prefix);
            out.extend(held[start..].iter());
            out.push('\n');
            return;
        }

        // The last space inside the budget, or a hard break where the word
        // itself is wider than the line.
        let window = &held[start..=(start + room)];
        let cut = match window.iter().rposition(|held| *held == ' ') {
            Some(0) | None => room,
            Some(at) => at,
        };

        out.push_str(prefix);
        out.extend(held[start..start + cut].iter());
        out.push('\n');

        start += cut;
        while held.get(start) == Some(&' ') {
            start += 1;
        }
        prefix = CONTINUATION;
    }
}

#[cfg(test)]
mod tests {
    use happenstance_core::{EventType, Query, QueryItem, SequencePosition, Tags};

    use super::{HEADER, NOTHING_SEEDED, Row, events_differ};

    fn position(raw: u64) -> SequencePosition {
        SequencePosition::new(raw).expect("a non-zero position")
    }

    fn row(raw: u64, event_type: &'static str) -> Row {
        Row {
            position: position(raw),
            event_type: EventType::from_static(event_type),
        }
    }

    fn query() -> Query {
        Query::from_item(
            QueryItem::new(
                ["Deposited"],
                Tags::from_pairs([("account", "a1")]).expect("a valid tag pair"),
            )
            .expect("a constrained item"),
        )
    }

    /// The four labels appear, once each, in the order the design fixes.
    ///
    /// Asserted by byte offset rather than by presence: four labels in the
    /// wrong order satisfy four `contains` calls perfectly.
    #[test]
    fn the_four_regions_are_in_order() {
        let message = events_differ(
            &["Deposited".to_owned()],
            &["Withdrawn".to_owned()],
            &[row(1, "Deposited")],
            &[row(2, "Audited")],
            &query(),
        );

        let at = |label: &str| message.find(label).expect("every label is present");
        assert!(message.starts_with(HEADER));
        assert!(at("expected:") < at("actual:"));
        assert!(at("actual:") < at("selected by the model's query:"));
        assert!(at("selected by the model's query:") < at("seeded but NOT selected:"));
        assert!(at("seeded but NOT selected:") < at("derived query:"));
    }

    /// Nothing seeded: the region stays and says so.
    #[test]
    fn the_empty_state_keeps_its_region() {
        let message = events_differ(&[], &[], &[], &[], &query());

        assert!(message.contains("seeded but NOT selected:"));
        assert!(message.contains(NOTHING_SEEDED));
    }

    /// `selected` truncates first; the diagnosis is still complete.
    ///
    /// The oracle is spelled from the inputs the test already holds — twenty
    /// rows in, `20 - 8 = 12` more — rather than from the renderer's own
    /// arithmetic.
    #[test]
    fn selected_truncates_first_and_the_diagnosis_last() {
        let selected: Vec<Row> = (1..=20).map(|n| row(n, "Deposited")).collect();
        let unselected = vec![row(21, "Audited"), row(22, "Audited")];

        let message = events_differ(&[], &[], &selected, &unselected, &query());
        let tail = message
            .split("seeded but NOT selected:")
            .nth(1)
            .expect("region four");

        assert!(message.contains("… and 12 more"));
        assert!(
            !tail.contains("… and"),
            "region four carries the diagnosis and must truncate last: {tail}"
        );
        assert_eq!(
            tail.matches("not selected").count(),
            2,
            "both unselected rows must be printed: {tail}"
        );
    }

    /// Every line fits, and nothing was cut to make it fit.
    #[test]
    fn a_long_event_type_wraps_rather_than_truncating() {
        let long = "DepositedIntoAnAccountWhoseNameIsFarLongerThanAnybodyWouldEverReasonablyChoose";
        let message = events_differ(&[], &[], &[], &[row(4096, long)], &query());

        for line in message.lines() {
            assert!(
                line.chars().count() <= 80,
                "over budget ({}): {line}",
                line.chars().count()
            );
        }

        let squashed: String = message.chars().filter(|c| !c.is_whitespace()).collect();
        assert!(
            squashed.contains(long),
            "the event type was truncated to fit: {message}"
        );
        assert!(
            squashed.contains("4096"),
            "the position was truncated to fit: {message}"
        );
    }

    /// The closing line names the query rather than dumping it.
    #[test]
    fn the_closing_line_names_the_derived_query() {
        let message = events_differ(&[], &[], &[], &[], &query());
        let last = message.lines().last().expect("a closing line");

        assert!(last.starts_with("derived query:"));
        assert!(last.contains("Deposited"));
        assert!(last.contains("account:a1"));
    }

    /// `Query::all` is named, not rendered as an empty item list.
    #[test]
    fn an_unconstrained_query_is_named() {
        let message = events_differ(&[], &[], &[], &[], &Query::all());
        assert!(message.contains("every event (Query::all)"));
    }
}
