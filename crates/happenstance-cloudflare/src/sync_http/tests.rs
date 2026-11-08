//! The pure half of the HTTP binding: which paths are routes, which status a
//! class answers with and back, and how large a request may be. Host and
//! `wasm32` alike; the request path itself runs on the shim, in
//! `event_store/ingest/tests/served.rs`.

use happenstance_sync::wire::Route;
use happenstance_sync::{FailureClass, PeerLimits};

use super::{ENVELOPE_ALLOWANCE, class_for, max_request_bytes, status_for, sync_route};

/// Wrong implementation it rejects: a router that matches on the suffix, and
/// so serves `/anything/pull` or `/sync/pull/extra` as a sync request.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn sync_route_names_exactly_the_two_paths() {
    assert_eq!(sync_route("/sync/pull"), Some(Route::Pull));
    assert_eq!(sync_route("/sync/push"), Some(Route::Push));
    for path in [
        "/sync/",
        "/sync/pull/",
        "/sync/pull/extra",
        "/other/pull",
        "sync/pull",
        "/sync/PULL",
        "/",
    ] {
        assert_eq!(sync_route(path), None, "{path}");
    }
}

/// Wrong implementation it rejects: two tables that drift, so a refusal the
/// object sends as one class is read by its client as another.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn every_class_round_trips_through_its_status() {
    for (class, status) in [
        (FailureClass::Transient, 503),
        (FailureClass::Split, 413),
        (FailureClass::Park, 422),
        (FailureClass::Stop, 400),
    ] {
        assert_eq!(status_for(class), status, "{class:?}");
        assert_eq!(class_for(status), class, "{status}");
    }
}

/// The statuses no class answers with: the platform's failures retry, and a
/// refusal of the request itself stops.
///
/// Wrong implementation it rejects: an inverse that defaults to `Stop` and so
/// halts an edge for good on the 500 a trapped instance answers with, or one
/// that defaults to `Transient` and retries a refused token forever.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn a_status_no_class_maps_to_reads_as_transient_or_stop() {
    for status in [408, 429, 500, 502, 504, 599] {
        assert_eq!(class_for(status), FailureClass::Transient, "{status}");
    }
    for status in [200, 301, 401, 403, 404, 405, 409, 499, 600] {
        assert_eq!(class_for(status), FailureClass::Stop, "{status}");
    }
}

/// Wrong implementation it rejects: a cap of exactly `max_batch_bytes`, which
/// refuses a batch at its declared limit for its envelope, and an addition
/// that wraps for a peer declaring `usize::MAX`.
#[cfg_attr(not(target_arch = "wasm32"), test)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
fn the_request_cap_is_the_batch_plus_the_allowance_and_saturates() {
    let minimum = PeerLimits::minimum();
    assert_eq!(
        max_request_bytes(&minimum),
        minimum.max_batch_bytes + ENVELOPE_ALLOWANCE
    );

    let mut unbounded = PeerLimits::minimum();
    unbounded.max_batch_bytes = usize::MAX;
    assert_eq!(max_request_bytes(&unbounded), usize::MAX);
}
