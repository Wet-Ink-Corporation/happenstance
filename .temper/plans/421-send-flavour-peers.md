# 421 · A Send-flavour story for StoreSyncPeer and RemoteSyncPeer

## Goal
Generic code bound on `SendSyncPeer` can drive a `StoreSyncPeer` and a
`RemoteSyncPeer` inside a real `tokio::spawn` (`Edge::step`, `Edge::run`).
Additive: no existing signature changes.

## Design
- RS-20-4: neither type can implement `SendSyncPeer` beside its bare impl
  (E0119 against `trait_variant`'s blanket impl), so the Send flavour is a
  second type. One newtype, `send_peer::SendPeer<P>`, rather than two named
  twins: one name to learn, room for the next peer, and both impls are
  coherent because their `P`s are distinct concrete heads.
  - `impl<S, I> SendSyncPeer for SendPeer<StoreSyncPeer<S, I>>` where
    `S: SendWriteGroupStore + Sync`, `I: SendIngestStore + Sync`.
  - `impl<X> SendSyncPeer for SendPeer<RemoteSyncPeer<X>>` where
    `X: SendExchange + Sync`.
  - Each method delegates to the bare impl. The future is `Send` because the
    inner store/exchange futures resolve through `trait_variant`'s blanket
    impls to the `Send` flavour's (the same reasoning `edge_spawns_from_generic`
    already proves for `Edge::step`). `Sync` because `&self` crosses awaits.
- Lost: `impl<P: SyncPeer + Send + Sync> SendSyncPeer for SendPeer<P>`, generic
  over any peer. It cannot be written on stable: the bare `P::pull` future has
  no `Send` bound to name without return-type notation, so the compiler cannot
  prove the delegation `Send`.
- Lost: deciding the spawning runner is generic over concrete peers. That
  pushes every application to spawn concretely and leaves the acceptance
  (generic `SendSyncPeer` spawn) unmet.
- Surface (OWN-01/02): `SendPeer::new(peer: P) -> Self` (stores it),
  `get_ref(&self) -> &P`, `into_inner(self) -> P`. `Debug, Clone` derived,
  no bounds on the struct (API-21).
- `lib.rs`: module and re-export appended at the end, so cited lines hold.

## Tests
- [x] `a_spawned_edge_runs_a_store_peer_from_generic_code`: `edge_runs_from_generic`
      over `SendPeer<StoreSyncPeer<..>>` syncs in a multi-thread task.
- [x] `a_spawned_edge_steps_a_remote_peer_from_generic_code`: `edge_spawns_from_generic`
      over `SendPeer<RemoteSyncPeer<Loopback>>` (a `SendExchange` calling `serve_sync`).
- [x] `a_spawned_edge_runs_a_remote_peer_from_generic_code`: the same via `Edge::run`.
- [x] Red: the same calls with the bare types fail E0277 (`SendSyncPeer` not implemented).

## HITL
- Public API addition (additive, scoped by #421 itself). No new dependency.

## Progress
- Red: `cargo test -p happenstance-sync --all-features --test flavours`, the
  three tests over the bare types: `error[E0277]: the trait bound
  StoreSyncPeer<Shared, Shared>: SendSyncPeer is not satisfied` (flavours.rs:877)
  and `RemoteSyncPeer<Loopback>: SendSyncPeer is not satisfied` (:895, :912).
- Green: `SendPeer` with the two impls; 9 of 9 in `flavours`.
- First green run caught a wrong expectation in my own new step test (pushed
  2, not 1: the pulled hub event is relayed back as part of the device log and
  the confirmed position is the device's head). Rewritten to assert against
  the head the store reports, not a literal.
- Mutant: `X: SendExchange` without `+ Sync` fails to compile, "future cannot
  be sent between threads safely": the `Sync` bounds are load-bearing.
- Gate GREEN; `cargo xtask spec-trace` clean; wasm32 check of the crate clean.
