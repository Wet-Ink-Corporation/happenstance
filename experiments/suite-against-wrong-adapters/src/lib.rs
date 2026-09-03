//! Four wrong event stores, and the correct core they are one step away from.
//!
//! This crate is the *subject* half of the experiment. It depends on
//! `happenstance-core` and on nothing else, exactly as an out-of-tree adapter
//! would: the conformance suite is a **dev**-dependency, reached from `tests/`,
//! so nothing in `src/` can be written to please it.
//!
//! # What is here
//!
//! [`correct`] is the repository's own correct core, copied verbatim from the
//! testkit's `tests/mutation_coverage/correct.rs`. [`stores`] holds four stores
//! built out of it with exactly one step changed each — the four wrong
//! implementations the pre-publication review named in findings L1-1, L1-2,
//! L3-01 and F2-5.
//!
//! # The second defect, and why every store carries a switch for it
//!
//! Each store is generic over `const INJECTED: bool`. With `false` it is the
//! store the finding describes. With `true` it *also* carries the testkit's own
//! `InnerJoinTagStore` defect — untagged events dropped by the read filter,
//! which the testkit's registry declares is caught by eighteen rules.
//!
//! That switch is the experiment's first control, and it is the house rule made
//! concrete: **a store that fails nothing is only evidence if a store that
//! should fail something does.** A fixture the harness silently declined, a
//! store whose `connect` handed back an empty log, a `const` that turned the
//! rules off — every one of those also produces "fails nothing", and none of
//! them is a statement about the suite. The injected arm rules them out.
#![forbid(unsafe_code)]

pub mod correct;
pub mod stores;
