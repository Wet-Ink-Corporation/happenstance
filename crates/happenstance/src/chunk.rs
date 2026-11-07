//! How many events one projection commit carries.

use core::num::NonZeroUsize;

/// How many events the projection runner applies between `begin` and `commit`.
///
/// A type rather than a bare [`NonZeroUsize`], so that the call site says what
/// the number is and so that the number has room to grow. The representation
/// is private: a later constructor can carry a different policy behind it,
/// and no caller of [`Chunk::of`] can tell (ADR-0070 §1). Whatever it grows
/// into stays `Copy`, because `Copy` is what lets one value serve every view
/// a fan-out runner drives without a clone per view; that is a promise this
/// type makes and cannot take back.
///
/// There is no `Default`, by ADR-0070 §3. Every size produces the same read
/// model (PS-14), so a default would hide only a cost, and the measurement
/// that would choose one has not been run. What the number trades is under
/// [`run_projection`](crate::run_projection)'s `# Choosing chunk` section.
///
/// ```
/// use happenstance::Chunk;
/// # fn main() -> Result<(), core::num::TryFromIntError> {
/// let per_commit = Chunk::of(64.try_into()?);
/// # let _ = per_commit;
/// # Ok(()) }
/// ```
///
/// A bare count does not compile, which is the point:
///
/// ```compile_fail
/// // compile_fail: chunk_takes_no_bare_count
/// let _ = happenstance::Chunk::of(64);
/// ```
///
/// And there is no default to fall back on:
///
/// ```compile_fail
/// // compile_fail: chunk_has_no_default
/// let _ = happenstance::Chunk::default();
/// ```
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct Chunk {
    size: NonZeroUsize,
}

impl Chunk {
    /// At most `size` events per commit.
    #[must_use]
    pub const fn of(size: NonZeroUsize) -> Self {
        Self { size }
    }

    /// The bound this value carries. Crate-internal, as `Retry`'s is: the
    /// runner reads it, and nothing outside needs a second reading.
    pub(crate) const fn get(self) -> usize {
        self.size.get()
    }
}
