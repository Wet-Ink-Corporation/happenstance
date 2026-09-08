//! Translating a [`Query`] into a `WHERE` clause the `/sql` endpoint accepts.
//!
//! One translation, two callers: the read path's single `SELECT` and the
//! conditional append's conflict guard. They are the same predicate over the same
//! table, and writing it twice is how the two acquire different bugs.
//!
//! # The semantics being reproduced
//!
//! Within a [`QueryItem`], **types are OR** (an empty list means any type) and
//! **tags are AND** (an empty set means any tags, and a superset matches).
//! **Across items it is OR.** `happenstance_core::Query::matches` is the
//! reference implementation and the conformance suite holds this to it.
//!
//! # Why `@>` and not a join table
//!
//! `Tags` is canonically sorted and deduplicated at construction, so the
//! containment operator expresses "the event's tags are a superset of the item's
//! tags" in one operator against one GIN index.
//!
//! # Where this differs from `happenstance-postgres`'s copy
//!
//! In one place, and it is a wire fact rather than a semantic one: parameters
//! here are [`serde_json::Value`], because the endpoint takes a JSON array and
//! infers Postgres types from it. A tag set therefore travels as a JSON array of
//! strings and is cast with `$n::text[]` — measured against the live endpoint,
//! which renders a JSON array as a Postgres array literal. `sqlx`'s typed binds
//! have no analogue to reproduce.
//!
//! # Parameters, not interpolation
//!
//! Every *value* is bound. Not because a tag could contain a quote — `Tag` and
//! `EventType` both reject control characters at construction — but because the
//! moment a value reaches SQL as text, the next person to add a value type has no
//! interpolation-free example to copy. Identifiers are a different problem and
//! [`NeonConfig`](crate::NeonConfig) answers it separately: they cannot be bound
//! at all, so they are quoted.

use happenstance_core::{Query, QueryItem};

/// A `WHERE`-clause fragment and the parameters it binds, in order.
///
/// The fragment is always parenthesised as a unit, so a caller may `AND` it with
/// its own predicates without thinking about precedence.
#[derive(Debug, Clone)]
pub(crate) struct Predicate {
    /// SQL, using `$n` placeholders numbered from the caller's starting index.
    sql: String,
    /// The values to bind, in placeholder order.
    params: Vec<serde_json::Value>,
}

impl Predicate {
    /// The SQL fragment.
    pub(crate) fn sql(&self) -> &str {
        &self.sql
    }

    /// The parameters, in placeholder order.
    pub(crate) fn into_params(self) -> Vec<serde_json::Value> {
        self.params
    }
}

/// Builds the `WHERE`-clause fragment matching `query`, numbering placeholders
/// from `next` and leaving `next` pointing past the last one used.
///
/// [`Query::All`] yields `TRUE` rather than an empty string: a caller can always
/// write `WHERE {fragment} AND …` without a special case, and `TRUE` is what the
/// planner removes for free.
pub(crate) fn predicate(query: &Query, next: &mut usize) -> Predicate {
    let Some(items) = query.items() else {
        return Predicate {
            sql: "TRUE".to_owned(),
            params: Vec::new(),
        };
    };

    if items.is_empty() {
        // Unreachable through today's public API — `Query::from_items([])` is
        // rejected as `InvalidQuery::NoItems`, and the test below asserts that
        // rather than leaving this looking untested. Kept because `Query` is
        // `#[non_exhaustive]` and `items()` hands back a plain slice. `FALSE`
        // rather than `TRUE`: an empty disjunction is empty, and getting it
        // backwards turns a query matching nothing into one matching the whole
        // log — and an append condition into one that refuses every append.
        return Predicate {
            sql: "FALSE".to_owned(),
            params: Vec::new(),
        };
    }

    let mut params = Vec::new();
    let arms: Vec<String> = items
        .iter()
        .map(|item| item_sql(item, next, &mut params))
        .collect();

    Predicate {
        // Each arm is parenthesised, and so is the disjunction. The core crate's
        // `AppendCondition` docs warn about exactly this: an adapter generating
        // SQL must parenthesise each guard explicitly, because the precedence bug
        // that already threatens the single-boundary form becomes n times more
        // likely here.
        sql: format!("({})", arms.join(" OR ")),
        params,
    }
}

/// One query item: types OR'd, tags AND'd, the two AND'd together.
fn item_sql(item: &QueryItem, next: &mut usize, params: &mut Vec<serde_json::Value>) -> String {
    let mut conjuncts: Vec<String> = Vec::new();

    if !item.types().is_empty() {
        let placeholders: Vec<String> = item
            .types()
            .iter()
            .map(|event_type| {
                params.push(serde_json::Value::String(event_type.as_str().to_owned()));
                let placeholder = format!("${next}");
                *next += 1;
                placeholder
            })
            .collect();
        conjuncts.push(format!("event_type IN ({})", placeholders.join(", ")));
    }

    if !item.tags().is_empty() {
        params.push(serde_json::Value::Array(
            item.tags()
                .iter()
                .map(|tag| serde_json::Value::String(tag.as_str().to_owned()))
                .collect(),
        ));
        // `@>` is containment: the row's tags must be a superset of the item's.
        // `<@` would be the other direction and would silently turn a superset
        // match into an exact-subset one.
        conjuncts.push(format!("tags @> ${next}::text[]"));
        *next += 1;
    }

    if conjuncts.is_empty() {
        // Also unreachable, and for a stronger reason: `QueryItem` rejects an
        // item constraining neither types nor tags as
        // `InvalidQuery::UnconstrainedItem`, so "matches everything" is spelled
        // `Query::All` and has nowhere else to come from. Same `#[non_exhaustive]`
        // justification for keeping it; same test below.
        return "(TRUE)".to_owned();
    }

    format!("({})", conjuncts.join(" AND "))
}

#[cfg(test)]
mod tests {
    use happenstance_core::{Query, QueryItem, Tags};

    use super::predicate;

    fn build(query: &Query) -> (String, Vec<serde_json::Value>) {
        let mut next = 1;
        let built = predicate(query, &mut next);
        (built.sql().to_owned(), built.into_params())
    }

    #[test]
    fn query_all_is_true_and_binds_nothing() {
        let (sql, params) = build(&Query::all());
        assert_eq!(sql, "TRUE");
        assert!(params.is_empty());
    }

    /// The `FALSE` branch above is unreachable through the public API, and this
    /// is the test that says so rather than leaving it looking like a gap.
    #[test]
    fn a_query_with_no_items_cannot_be_built_at_all() {
        assert!(
            Query::from_items([]).is_err(),
            "the core crate rejects an empty query; the FALSE branch is a guard, not a path"
        );
    }

    #[test]
    fn types_within_an_item_are_or() {
        let item = QueryItem::of_types(["A", "B"]).expect("valid types");
        let (sql, params) = build(&Query::from_item(item));
        assert_eq!(sql, "((event_type IN ($1, $2)))");
        assert_eq!(params, vec![serde_json::json!("A"), serde_json::json!("B")]);
    }

    #[test]
    fn tags_within_an_item_are_one_containment_test() {
        let tags = Tags::from_pairs([("course", "c1"), ("student", "s1")]).expect("valid tags");
        let item = QueryItem::tagged(tags).expect("valid item");
        let (sql, params) = build(&Query::from_item(item));
        assert_eq!(sql, "((tags @> $1::text[]))");
        // Canonically sorted by `Tags` itself, which is what keeps `@>` usable.
        assert_eq!(params, vec![serde_json::json!(["course:c1", "student:s1"])]);
    }

    #[test]
    fn types_and_tags_within_an_item_are_and() {
        let tags = Tags::from_pairs([("course", "c1")]).expect("valid tags");
        let item = QueryItem::new(["A"], tags).expect("valid item");
        let (sql, _) = build(&Query::from_item(item));
        assert_eq!(sql, "((event_type IN ($1) AND tags @> $2::text[]))");
    }

    /// Across items it is OR, and every arm is parenthesised.
    ///
    /// The unparenthesised form is `a AND b OR c AND d`, which Postgres reads as
    /// `(a AND b) OR (c AND d)` — correct here by luck. Add a boundary predicate
    /// with `AND` at the caller and the luck runs out, which is why the whole
    /// disjunction is wrapped too.
    #[test]
    fn items_are_or_and_every_arm_is_parenthesised() {
        let first = QueryItem::of_types(["A"]).expect("valid");
        let second =
            QueryItem::tagged(Tags::from_pairs([("k", "v")]).expect("valid")).expect("valid");
        let (sql, _) = build(&Query::from_items([first, second]).expect("valid"));
        assert_eq!(sql, "((event_type IN ($1)) OR (tags @> $2::text[]))");
    }

    /// The numbering the append's second statement depends on.
    ///
    /// Its guard predicate is numbered from `3`, because `$1` is the event batch
    /// and `$2` is `recorded_at`; the standalone probe numbers the same predicate
    /// from `1`. Two callers, two starting indexes, one function — and getting
    /// this wrong binds the events as an event type without a type error anywhere.
    #[test]
    fn placeholder_numbering_continues_from_the_callers_index() {
        let item = QueryItem::of_types(["A"]).expect("valid");
        let query = Query::from_item(item);
        let mut next = 3;
        let built = predicate(&query, &mut next);
        assert_eq!(built.sql(), "((event_type IN ($3)))");
        assert_eq!(next, 4, "the caller must be told what it may use next");
    }

    /// The `(TRUE)` branch in `item_sql` is likewise unreachable, and for a
    /// stronger reason than the one above.
    #[test]
    fn an_item_constraining_nothing_cannot_be_built_at_all() {
        assert!(
            QueryItem::tagged(Tags::default()).is_err(),
            "the core crate rejects an unconstrained item; `Query::All` is how that is spelled"
        );
    }
}
