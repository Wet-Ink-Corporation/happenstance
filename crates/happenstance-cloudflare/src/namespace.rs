//! [`TableNamespace`], and the table names a store renders its statements with.
//!
//! Private: the type is re-exported at the crate root, which is its one public
//! path.

use std::borrow::Cow;

/// A validated prefix that keeps one log in its own tables.
///
/// A Durable Object has exactly one SQL database, and
/// [`CloudflareEventStore::new`](crate::CloudflareEventStore::new) keeps its log
/// in tables with fixed names. A store built by
/// [`CloudflareEventStore::namespaced`](crate::CloudflareEventStore::namespaced)
/// keeps its log in `{namespace}_event`, `{namespace}_event_tag` and
/// `{namespace}_store_meta`, indexed by `{namespace}_event_type_idx`, so several
/// logs live in one object and share nothing: no rows, no position sequence and
/// no incarnation. The `workerd` conformance harness uses it to give each
/// fixture instance its own log; an application can use it for one log per
/// tenant.
///
/// # The alphabet
///
/// The namespace is spliced into SQL as part of an identifier, so it is
/// validated rather than quoted:
///
/// * **Lowercase ASCII letters, digits and `_`.** SQLite compares identifiers
///   case-insensitively, so admitting `Tenant` beside `tenant` would admit two
///   namespaces that compare unequal here and share a log there.
/// * **A letter first.** An identifier that starts with a digit has to be
///   quoted.
/// * **Not beginning `sqlite`.** SQLite reserves every name beginning `sqlite_`.
/// * **At most [`MAX_LEN`](Self::MAX_LEN) bytes.**
///
/// # No two namespaces share a name
///
/// Each of the four names ends in a word the other three do not end in
/// (`event`, `tag`, `meta`, `idx`), so two names can be equal only if they are
/// the same kind of name, and then only if their namespaces are equal. No
/// prefixed name equals an unprefixed one, because the namespace is never
/// empty. Indexes share SQLite's schema namespace with tables, which is why the
/// index is part of the argument.
///
/// # Examples
///
/// ```
/// use happenstance_cloudflare::TableNamespace;
///
/// let tenant = TableNamespace::new("tenant_7")?;
/// assert_eq!(tenant.as_str(), "tenant_7");
///
/// assert!(TableNamespace::new("Tenant").is_err());
/// assert!(TableNamespace::new("sqlite_x").is_err());
/// # Ok::<(), happenstance_cloudflare::InvalidTableNamespace>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TableNamespace(String);

impl TableNamespace {
    /// The longest namespace accepted, in bytes.
    ///
    /// Long enough for a UUID tenant key mapped into the alphabet (37 bytes),
    /// and a bound so that a namespace cannot dominate the statement length a
    /// Durable Object caps. A query names the tag table once per tag term, so
    /// the prefix's cost scales with the terms in one statement, which the
    /// parameter cap bounds: at `workerd`'s 100 parameters that is about 6.4 KB
    /// of the 100,000 bytes it allows. Raising this later is additive; lowering
    /// it is not.
    pub const MAX_LEN: usize = 64;

    /// Validates `name` as a namespace.
    ///
    /// # Errors
    ///
    /// Returns an [`InvalidTableNamespace`] naming the first rule `name` breaks.
    pub fn new(name: &str) -> Result<Self, InvalidTableNamespace> {
        let Some(first) = name.chars().next() else {
            return Err(InvalidTableNamespace::Empty);
        };
        if name.len() > Self::MAX_LEN {
            return Err(InvalidTableNamespace::TooLong { len: name.len() });
        }
        if !first.is_ascii_lowercase() {
            return Err(InvalidTableNamespace::Start { character: first });
        }
        if let Some((index, character)) = name
            .char_indices()
            .find(|&(_, c)| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'))
        {
            return Err(InvalidTableNamespace::Character { index, character });
        }
        if name.starts_with("sqlite") {
            return Err(InvalidTableNamespace::Reserved);
        }
        Ok(Self(name.to_owned()))
    }

    /// The namespace as it was given.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Why a string is not a [`TableNamespace`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidTableNamespace {
    /// The string is empty.
    #[error("a table namespace is empty")]
    Empty,
    /// The string is longer than [`TableNamespace::MAX_LEN`] bytes.
    #[error("a table namespace is {len} bytes, above the maximum of {max}", max = TableNamespace::MAX_LEN)]
    TooLong {
        /// The string's length in bytes.
        len: usize,
    },
    /// The first character is not a lowercase ASCII letter.
    #[error("a table namespace starts with {character:?}, not a lowercase ASCII letter")]
    Start {
        /// The offending character.
        character: char,
    },
    /// A character outside lowercase ASCII letters, digits and `_`.
    #[error("a table namespace has {character:?} at byte {index}")]
    Character {
        /// The byte offset of the character.
        index: usize,
        /// The offending character.
        character: char,
    },
    /// The string begins `sqlite`, whose tables SQLite reserves.
    #[error("a table namespace begins \"sqlite\", which SQLite reserves")]
    Reserved,
}

/// The names one store's statements are rendered with.
///
/// `Cow` so the unprefixed set is a `const` and the store built by
/// [`CloudflareEventStore::new`](crate::CloudflareEventStore::new) renders
/// exactly the statements it always has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Tables {
    pub(crate) event: Cow<'static, str>,
    pub(crate) event_tag: Cow<'static, str>,
    pub(crate) store_meta: Cow<'static, str>,
    pub(crate) event_type_idx: Cow<'static, str>,
}

impl Tables {
    /// The names [`CloudflareEventStore::new`](crate::CloudflareEventStore::new)
    /// has always used.
    pub(crate) const UNPREFIXED: Self = Self {
        event: Cow::Borrowed("event"),
        event_tag: Cow::Borrowed("event_tag"),
        store_meta: Cow::Borrowed("store_meta"),
        event_type_idx: Cow::Borrowed("event_type_idx"),
    };

    /// The names for `namespace`'s log.
    pub(crate) fn namespaced(namespace: &TableNamespace) -> Self {
        let prefixed = |table: &str| Cow::Owned(format!("{}_{table}", namespace.as_str()));
        Self {
            event: prefixed("event"),
            event_tag: prefixed("event_tag"),
            store_meta: prefixed("store_meta"),
            event_type_idx: prefixed("event_type_idx"),
        }
    }
}

/// The isolation [`CloudflareEventStore::namespaced`](crate::CloudflareEventStore::namespaced)
/// promises, observed on one Durable Object through the public port.
#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests {
    use core::future::poll_fn;
    use core::pin::pin;

    use futures_core::Stream;
    use happenstance_core::{
        AppendCondition, AppendError, Event, EventStore, Query, QueryItem, ReadOptions,
        SequencePosition, Tag, Tags,
    };
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::TableNamespace;
    use crate::host::{arm_throw, durable_object};
    use crate::sql_storage::{SqlStorage, SqlValue};
    use crate::{CloudflareEventStore, CloudflareEventStoreError};

    fn namespaced(sql: &SqlStorage, name: &str) -> CloudflareEventStore {
        let namespace = TableNamespace::new(name).expect("a valid namespace");
        let store = CloudflareEventStore::namespaced(sql.clone(), &namespace);
        store.migrate().expect("the namespace's schema applies");
        store
    }

    fn event(event_type: &str) -> Event {
        Event::new(event_type.to_owned(), &b"payload"[..]).expect("a valid event type")
    }

    async fn types_in(store: &CloudflareEventStore) -> Vec<String> {
        let all = Query::all();
        let mut stream = pin!(store.read(&all, ReadOptions::default()));
        let mut out = Vec::new();
        while let Some(item) = poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
            out.push(
                item.expect("the read succeeds")
                    .event_type()
                    .as_str()
                    .to_owned(),
            );
        }
        out
    }

    async fn append(store: &CloudflareEventStore, event_type: &str) -> SequencePosition {
        store
            .append(&[event(event_type)], None)
            .await
            .expect("the append lands")
    }

    /// Two namespaces and the unprefixed log on one object: each reads back only
    /// its own events, and each `head` is its own.
    #[wasm_bindgen_test]
    async fn namespaces_on_one_object_share_no_events() {
        let sql = durable_object();
        let plain = CloudflareEventStore::new(sql.clone());
        plain.migrate().expect("the schema applies");
        let left = namespaced(&sql, "left");
        let right = namespaced(&sql, "right");

        append(&left, "Left").await;
        let left_head = append(&left, "Left").await;
        let right_head = append(&right, "Right").await;

        assert_eq!(types_in(&left).await, ["Left", "Left"]);
        assert_eq!(types_in(&right).await, ["Right"]);
        assert_eq!(types_in(&plain).await, Vec::<String>::new());
        assert_eq!(left.head().await.expect("head reads"), Some(left_head));
        assert_eq!(right.head().await.expect("head reads"), Some(right_head));
        assert_eq!(plain.head().await.expect("head reads"), None);
    }

    /// An identity minted in one namespace is not a member of another, so the
    /// incarnations differ rather than being shared through one `store_meta`.
    #[wasm_bindgen_test]
    async fn an_event_id_is_a_member_only_of_its_own_namespace() {
        let sql = durable_object();
        let left = namespaced(&sql, "left");
        let right = namespaced(&sql, "right");
        append(&left, "Left").await;
        append(&right, "Right").await;

        let all = Query::all();
        let mut stream = pin!(left.read(&all, ReadOptions::default()));
        let id = poll_fn(|cx| stream.as_mut().poll_next(cx))
            .await
            .expect("one event")
            .expect("the read succeeds")
            .id;

        assert!(left.contains_event_id(id).await.expect("the probe runs"));
        assert!(!right.contains_event_id(id).await.expect("the probe runs"));
    }

    /// Two handles under one namespace are one log, as two `new` handles are.
    #[wasm_bindgen_test]
    async fn two_handles_under_one_namespace_are_one_log() {
        let sql = durable_object();
        let first = namespaced(&sql, "shared");
        let second = namespaced(&sql, "shared");

        let position = append(&first, "Shared").await;

        assert_eq!(second.head().await.expect("head reads"), Some(position));
        assert_eq!(types_in(&second).await, ["Shared"]);
    }

    fn tagged(event_type: &str, tags: &[&str]) -> Event {
        event(event_type).with_tags(tag_set(tags))
    }

    fn tag_set(tags: &[&str]) -> Tags {
        tags.iter()
            .map(|tag| Tag::new((*tag).to_owned()).expect("a valid tag"))
            .collect()
    }

    fn by_tag(tag: &str) -> Query {
        Query::from_item(
            QueryItem::new(core::iter::empty::<String>(), tag_set(&[tag])).expect("a valid item"),
        )
    }

    async fn types_matching(store: &CloudflareEventStore, query: &Query) -> Vec<String> {
        let mut stream = pin!(store.read(query, ReadOptions::default()));
        let mut out = Vec::new();
        while let Some(item) = poll_fn(|cx| stream.as_mut().poll_next(cx)).await {
            out.push(
                item.expect("the read succeeds")
                    .event_type()
                    .as_str()
                    .to_owned(),
            );
        }
        out
    }

    fn rows_in(sql: &SqlStorage, table: &str) -> i64 {
        let mut cursor = sql
            .exec(&format!("SELECT count(*) FROM {table}"), &[])
            .expect("the count runs");
        match cursor.next_row().map(|row| row.expect("the row decodes")) {
            Some(row) => match row.values() {
                [SqlValue::Integer(count)] => *count,
                other => panic!("unexpected count row: {other:?}"),
            },
            None => panic!("a count returns one row"),
        }
    }

    /// Every statement a namespaced store issues names its own tables. The
    /// object holds **no** unprefixed table, so a statement that still named
    /// `event`, `event_tag` or `store_meta` fails with "no such table" here:
    /// the tag rows, a tag query, a tag-guarded condition both refused and
    /// admitted, `head`, and `contains_event_id`.
    #[wasm_bindgen_test]
    async fn a_namespaced_store_touches_only_its_own_tables() {
        let sql = durable_object();
        let store = namespaced(&sql, "solo");

        let first = store
            .append(&[tagged("Opened", &["account:1"])], None)
            .await
            .expect("a tagged append lands");
        store
            .append(&[tagged("Other", &["account:2"])], None)
            .await
            .expect("a second tagged append lands");

        assert_eq!(
            types_matching(&store, &by_tag("account:1")).await,
            ["Opened"]
        );

        let guard = AppendCondition::new(by_tag("account:1"));
        let refused = store
            .append(&[tagged("Again", &["account:1"])], Some(&guard))
            .await;
        assert!(
            matches!(refused, Err(AppendError::ConditionViolated(_))),
            "the guard sees its own namespace's tag rows: {refused:?}"
        );
        let unseen = AppendCondition::new(by_tag("account:3"));
        let admitted = store
            .append(&[tagged("Fresh", &["account:3"])], Some(&unseen))
            .await
            .expect("a guard over an unheld tag admits the append");

        assert_eq!(store.head().await.expect("head reads"), Some(admitted));
        let query = by_tag("account:1");
        let mut stream = pin!(store.read(&query, ReadOptions::default()));
        let id = poll_fn(|cx| stream.as_mut().poll_next(cx))
            .await
            .expect("one event")
            .expect("the read succeeds")
            .id;
        assert_eq!(id.position(), first);
        assert!(store.contains_event_id(id).await.expect("the probe runs"));
        assert_eq!(rows_in(&sql, "solo_event_tag"), 3);
    }

    /// The same tag in two namespaces and the unprefixed log: each tag query
    /// and each tag guard sees only its own log.
    #[wasm_bindgen_test]
    async fn a_tag_in_one_namespace_is_invisible_to_another() {
        let sql = durable_object();
        let plain = CloudflareEventStore::new(sql.clone());
        plain.migrate().expect("the schema applies");
        let left = namespaced(&sql, "left");
        let right = namespaced(&sql, "right");

        left.append(&[tagged("Left", &["shared:tag"])], None)
            .await
            .expect("the append lands");

        assert_eq!(types_matching(&left, &by_tag("shared:tag")).await, ["Left"]);
        assert_eq!(
            types_matching(&right, &by_tag("shared:tag")).await,
            Vec::<String>::new()
        );
        assert_eq!(
            types_matching(&plain, &by_tag("shared:tag")).await,
            Vec::<String>::new()
        );

        let guard = AppendCondition::new(by_tag("shared:tag"));
        right
            .append(&[tagged("Right", &["shared:tag"])], Some(&guard))
            .await
            .expect("another namespace's tag does not violate this guard");
        plain
            .append(&[tagged("Plain", &["shared:tag"])], Some(&guard))
            .await
            .expect("nor the unprefixed log's");
    }

    /// A batch that fails part way through is swept out of its own tables. The
    /// object holds no unprefixed table, so a discard that named `event` or
    /// `event_tag` would fail and leave the row behind.
    #[wasm_bindgen_test]
    async fn a_failed_batch_is_discarded_from_its_own_tables() {
        let sql = durable_object();
        let store = namespaced(&sql, "solo");
        arm_throw(
            &sql,
            "INSERT INTO solo_event_tag",
            "no space left on device",
        );

        let failed = store
            .append(&[tagged("Doomed", &["account:1"])], None)
            .await;

        assert!(
            matches!(
                failed,
                Err(AppendError::Store(CloudflareEventStoreError::Sql(_)))
            ),
            "the armed throw fails the batch, and its discard succeeds: {failed:?}"
        );
        assert_eq!(rows_in(&sql, "solo_event"), 0);
        assert_eq!(rows_in(&sql, "solo_event_tag"), 0);
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::{InvalidTableNamespace, TableNamespace, Tables};

    #[test]
    fn a_lowercase_identifier_is_accepted() {
        let namespace = TableNamespace::new("tenant_7").expect("valid");
        assert_eq!(namespace.as_str(), "tenant_7");
    }

    #[test]
    fn each_rule_is_refused_by_name() {
        assert_eq!(TableNamespace::new(""), Err(InvalidTableNamespace::Empty));
        assert_eq!(
            TableNamespace::new(&"a".repeat(TableNamespace::MAX_LEN + 1)),
            Err(InvalidTableNamespace::TooLong {
                len: TableNamespace::MAX_LEN + 1
            })
        );
        assert_eq!(
            TableNamespace::new("7tenant"),
            Err(InvalidTableNamespace::Start { character: '7' })
        );
        assert_eq!(
            TableNamespace::new("Tenant"),
            Err(InvalidTableNamespace::Start { character: 'T' })
        );
        assert_eq!(
            TableNamespace::new("ten-ant"),
            Err(InvalidTableNamespace::Character {
                index: 3,
                character: '-'
            })
        );
        assert_eq!(
            TableNamespace::new("tenAnt"),
            Err(InvalidTableNamespace::Character {
                index: 3,
                character: 'A'
            })
        );
        assert_eq!(
            TableNamespace::new("sqlite_x"),
            Err(InvalidTableNamespace::Reserved)
        );
    }

    #[test]
    fn the_longest_namespace_is_accepted() {
        let longest = "a".repeat(TableNamespace::MAX_LEN);
        assert_eq!(
            TableNamespace::new(&longest).map(|namespace| namespace.as_str().len()),
            Ok(TableNamespace::MAX_LEN)
        );
    }

    /// The common tenant key: a UUID, hyphens mapped to `_` and a letter in
    /// front because a UUID may start with a digit. 37 bytes.
    #[test]
    fn a_mapped_uuid_fits() {
        let tenant = "t6f1c2d3e_4b5a_4c6d_8e7f_9a0b1c2d3e4f";
        assert_eq!(tenant.len(), 37);
        assert_eq!(
            TableNamespace::new(tenant).map(|namespace| namespace.as_str().to_owned()),
            Ok(tenant.to_owned())
        );
    }

    #[test]
    fn a_namespace_prefixes_every_name() {
        let tables = Tables::namespaced(&TableNamespace::new("t").expect("valid"));
        assert_eq!(tables.event, "t_event");
        assert_eq!(tables.event_tag, "t_event_tag");
        assert_eq!(tables.store_meta, "t_store_meta");
        assert_eq!(tables.event_type_idx, "t_event_type_idx");
    }

    #[test]
    fn distinct_namespaces_share_no_table() {
        let names = |tables: &Tables| {
            [
                tables.event.to_string(),
                tables.event_tag.to_string(),
                tables.store_meta.to_string(),
                tables.event_type_idx.to_string(),
            ]
        };
        // The pairs a careless suffix scheme collides on: one namespace that is
        // another's prefix plus a table name.
        let a = Tables::namespaced(&TableNamespace::new("a").expect("valid"));
        let a_event = Tables::namespaced(&TableNamespace::new("a_event").expect("valid"));
        for left in names(&a) {
            assert!(!names(&a_event).contains(&left), "{left} is shared");
            assert!(
                !names(&Tables::UNPREFIXED).contains(&left),
                "{left} is shared"
            );
        }
    }
}
