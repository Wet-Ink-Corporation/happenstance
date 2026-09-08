//! The JSON the `/sql` endpoint answers with.
//!
//! Mirrors node-postgres' result shape, which is what Neon's serverless driver
//! exposes and therefore what the HTTP endpoint emits. It is reproduced here as
//! `Deserialize` types rather than read out of a `serde_json::Value` by hand so
//! that a schema drift is a
//! [`NeonError::MalformedResponse`](crate::NeonError::MalformedResponse) with a
//! `serde_json` error inside it, at a named field, instead of a `None` three
//! layers down.
//!
//! # How every scalar arrives, measured rather than assumed
//!
//! The endpoint renders every value as JSON, and the renderings are not the ones
//! a reader guesses:
//!
//! | Postgres type | JSON |
//! |---|---|
//! | `bigint` | a **string** — `"9223372036854775807"`, which is why nothing here loses precision to `f64` |
//! | `integer` | a number |
//! | `bytea` | a `\x…` hex string, so the adapter selects `encode(col, 'base64')` instead and pays 33% rather than 100% |
//! | `text[]` | a JSON array of strings |
//! | `NULL` | `null`, at any type |
//!
//! Every `SELECT` this crate emits casts its `bigint` columns with `::text`
//! explicitly, so the decoders never have to distinguish "the endpoint chose a
//! string" from "the endpoint chose a number".

use serde::Deserialize;

/// One column of a result set.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct FieldDescription {
    /// The column name.
    pub name: String,
    /// The Postgres OID of the column's type.
    ///
    /// Needed because the endpoint renders everything as JSON: `bigint` arrives
    /// as a *string* to avoid IEEE-754 precision loss, and `bytea` arrives as a
    /// `\x…` hex string, so the OID is the only thing distinguishing them.
    #[serde(rename = "dataTypeID")]
    pub data_type_id: u32,
}

/// The result of one statement.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ResultSet {
    /// The SQL command tag: `SELECT`, `INSERT`, …
    pub command: String,
    /// Rows affected, when the command reports it.
    #[serde(rename = "rowCount")]
    pub row_count: Option<u64>,
    /// The column descriptions.
    pub fields: Vec<FieldDescription>,
    /// The rows, each a JSON object keyed by column name.
    ///
    /// Held as `serde_json::Value` rather than a typed row because the adapter
    /// issues several differently-shaped statements against one decoder, and
    /// because every scalar needs an OID-directed conversion anyway.
    pub rows: Vec<serde_json::Value>,
}

impl ResultSet {
    /// One column of one row, as the endpoint rendered it.
    ///
    /// `None` covers three cases the callers all treat alike: there is no such
    /// row, there is no such column, or the value is SQL `NULL`. Each decoder
    /// that cares about the difference asks a second question rather than having
    /// three variants threaded through every call site.
    #[must_use]
    pub fn cell(&self, row: usize, column: &str) -> Option<&serde_json::Value> {
        self.rows
            .get(row)?
            .get(column)
            .filter(|value| !value.is_null())
    }

    /// One column of one row, as a string.
    ///
    /// The `bigint`-as-string rendering means this is what almost every decode
    /// in this crate wants.
    #[must_use]
    pub fn text(&self, row: usize, column: &str) -> Option<&str> {
        self.cell(row, column)?.as_str()
    }
}

/// The body of a successful `/sql` response.
///
/// The single-statement form answers with a bare [`ResultSet`]; the batch form
/// answers with `{"results": [ … ]}`. Both are accepted here because the adapter
/// uses both.
///
/// # Why this is not `#[serde(untagged)]`
///
/// It was, and the attribute defeated this module's stated reason for existing.
/// Untagged deserialisation tries each variant and, when none fits, reports
/// `data did not match any variant of untagged enum ResponseBody` — with no
/// field, no offset and no indication of which shape was closer. A drift in
/// `ResultSet` would surface as that sentence rather than as
/// `missing field 'rows'`, which is exactly the "a `None` three layers down"
/// failure the module docs above say the typed shapes exist to prevent.
///
/// [`parse`](Self::parse) branches on the presence of `results` first and then
/// deserialises the chosen shape, so a malformed body reports the field it
/// tripped on.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub enum ResponseBody {
    /// The batch form.
    Batch {
        /// One entry per statement, in the order they were sent.
        results: Vec<ResultSet>,
    },
    /// The single-statement form.
    Single(ResultSet),
}

impl ResponseBody {
    /// Reads a 2xx body.
    ///
    /// # Errors
    ///
    /// The `serde_json` error, naming the field that did not fit.
    pub fn parse(body: &[u8]) -> Result<Self, serde_json::Error> {
        let value: serde_json::Value = serde_json::from_slice(body)?;
        if value.get("results").is_some() {
            #[derive(Deserialize)]
            struct Batch {
                results: Vec<ResultSet>,
            }
            let batch: Batch = serde_json::from_value(value)?;
            return Ok(Self::Batch {
                results: batch.results,
            });
        }
        Ok(Self::Single(serde_json::from_value(value)?))
    }

    /// The result sets, however the endpoint chose to wrap them.
    #[must_use]
    pub fn result_sets(&self) -> &[ResultSet] {
        match self {
            Self::Batch { results } => results,
            Self::Single(single) => core::slice::from_ref(single),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ResponseBody;

    /// A body recorded from the live endpoint, single-statement form.
    const SINGLE: &[u8] = br#"{"fields":[{"name":"position","dataTypeID":25}],"rows":[{"position":"1"}],"command":"INSERT","rowCount":1,"rowAsArray":false}"#;

    /// A body recorded from the live endpoint, batch form.
    const BATCH: &[u8] = br#"{"results":[{"fields":[],"rows":[{"conflict":null}],"command":"SELECT","rowCount":1,"rowAsArray":false},{"fields":[],"rows":[{"position":"7"}],"command":"INSERT","rowCount":1,"rowAsArray":false}]}"#;

    #[test]
    fn the_single_form_is_one_result_set() {
        let body = ResponseBody::parse(SINGLE).expect("a recorded body parses");
        assert_eq!(body.result_sets().len(), 1);
        assert_eq!(body.result_sets()[0].text(0, "position"), Some("1"));
        assert_eq!(body.result_sets()[0].row_count, Some(1));
    }

    #[test]
    fn the_batch_form_is_one_result_set_per_statement() {
        let body = ResponseBody::parse(BATCH).expect("a recorded body parses");
        assert_eq!(body.result_sets().len(), 2);
        assert_eq!(
            body.result_sets()[0].cell(0, "conflict"),
            None,
            "a SQL NULL and an absent column are one answer here, on purpose"
        );
        assert_eq!(body.result_sets()[1].text(0, "position"), Some("7"));
    }

    /// The whole reason `untagged` is gone: the error names the field.
    #[test]
    fn a_drifted_body_reports_the_field_it_tripped_on() {
        let error = ResponseBody::parse(br#"{"results":[{"command":"SELECT","fields":[]}]}"#)
            .expect_err("a result set with no `rows` is not a result set");
        let message = error.to_string();
        assert!(
            message.contains("missing field `rows`"),
            "the error must name the missing field, got: {message}"
        );
        assert!(
            !message.contains("untagged"),
            "an untagged enum reports `data did not match any variant`, which is \
             the diagnostic this module exists to avoid: {message}"
        );
    }
}
