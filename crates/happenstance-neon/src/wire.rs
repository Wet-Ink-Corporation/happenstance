//! The JSON the `/sql` endpoint answers with.
//!
//! Mirrors node-postgres' result shape, which is what Neon's serverless driver
//! exposes and therefore what the HTTP endpoint emits. It is reproduced here as
//! `Deserialize` types rather than read out of a `serde_json::Value` by hand so
//! that a schema drift is a
//! [`NeonError::MalformedResponse`](crate::NeonError::MalformedResponse) with a
//! `serde_json` error inside it, at a named field, instead of a `None` three
//! layers down.

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

/// The body of a successful `/sql` response.
///
/// The single-statement form answers with a bare [`ResultSet`]; the batch form
/// answers with `{"results": [ … ]}`. Both are accepted here because the
/// adapter uses both.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
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
    /// The result sets, however the endpoint chose to wrap them.
    pub fn result_sets(&self) -> &[ResultSet] {
        match self {
            Self::Batch { results } => results,
            Self::Single(single) => core::slice::from_ref(single),
        }
    }
}
