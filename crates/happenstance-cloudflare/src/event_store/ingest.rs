//! VT-10's fourth run: `happenstance-sync`'s ingest port, implemented for the
//! Durable Object over the row writer `append` already uses.

// The whole file is `cfg(test)` already, through its declaration in
// `event_store.rs`. Said again here so the module reads as test code where it
// stands.
#[cfg(test)]
mod tests;
