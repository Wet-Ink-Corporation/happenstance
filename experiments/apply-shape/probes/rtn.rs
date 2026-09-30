//! Is return-type notation usable on the pinned toolchain?
//!
//! If it were, a generic spawner could write `P: Projection<apply(..): Send>`
//! and ONE trait would suffice, with no `trait_variant` pair. Expected on
//! 1.97.1: `error[E0658]: return type notation is experimental`.
//!
//! Built by `run.sh` as `rustc --edition 2024 --crate-type lib probes/rtn.rs`.

pub trait Projection {
    fn apply(&mut self, x: u32) -> impl Future<Output = Result<(), ()>>;
}

pub fn spawnable<P>(projection: P)
where
    P: Projection<apply(..): Send> + Send + 'static,
{
    let _ = projection;
}
