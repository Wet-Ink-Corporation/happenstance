//! [`StoreResume`], the token a store peer mints, and its text form.

use happenstance_core::SequencePosition;

use crate::classify::{Classify, FailureClass};
use crate::wire::ResumeToken;

/// Where a [`StoreSyncPeer`](crate::StoreSyncPeer) pull left off: the last
/// far-store position it forwarded, or the start.
///
/// Owned and `Copy`, so it outlives the peer and the store handle (SY-16), and
/// its text form is a [`ResumeToken`] for the wire: the decimal position, and
/// `"0"` for the start — a value no position takes, because positions start at
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StoreResume {
    after: Option<SequencePosition>,
}

impl StoreResume {
    /// The token that reads the far store from its first event.
    #[must_use]
    pub const fn start() -> Self {
        Self { after: None }
    }

    /// The token that reads the far store above `position`.
    #[must_use]
    pub const fn after(position: SequencePosition) -> Self {
        Self {
            after: Some(position),
        }
    }

    /// The last position forwarded, or `None` from the start.
    #[must_use]
    pub const fn last_forwarded(self) -> Option<SequencePosition> {
        self.after
    }

    /// The first position the next pull reads, or `None` if this token
    /// already names the last position a store can number.
    pub(super) const fn next_position(self) -> Option<SequencePosition> {
        match self.after {
            Some(position) => position.next(),
            None => Some(SequencePosition::FIRST),
        }
    }
}

impl From<StoreResume> for ResumeToken {
    fn from(resume: StoreResume) -> Self {
        let position = resume.after.map_or(0, SequencePosition::get);
        Self::new(alloc::format!("{position}"))
    }
}

/// Reads back the text [`From<StoreResume>`](ResumeToken#impl-From<StoreResume>-for-ResumeToken)
/// wrote, and nothing else.
///
/// Strict, so the text form is one-to-one: digits only, no sign, no leading
/// zero but the start's `"0"`, and within `u64`.
impl TryFrom<&ResumeToken> for StoreResume {
    type Error = ForeignToken;

    fn try_from(token: &ResumeToken) -> Result<Self, Self::Error> {
        let text = token.as_str();
        let canonical = !text.is_empty()
            && text.bytes().all(|byte| byte.is_ascii_digit())
            && (text == "0" || !text.starts_with('0'));
        let parsed = canonical
            .then(|| text.parse::<u64>().ok())
            .flatten()
            // Cloned into the error, which outlives the request the borrowed
            // token came in: a runner reports it after the buffer is gone.
            .ok_or_else(|| ForeignToken {
                token: token.clone(),
            })?;
        Ok(Self {
            after: SequencePosition::new(parsed),
        })
    }
}

/// A [`ResumeToken`] that no [`StoreSyncPeer`](crate::StoreSyncPeer) minted.
///
/// [`Stop`](FailureClass::Stop): the token came from somewhere else — another
/// kind of peer, a corrupted cursor column, a hand-edited request — and the
/// same token fails the same way on every retry. The remedy is to drop it and
/// pull from the start, which is the operator's or runner's call, not this
/// peer's.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("resume token {:?} was not minted by a store peer", token.as_str())]
pub struct ForeignToken {
    token: ResumeToken,
}

impl ForeignToken {
    /// The token that was refused.
    #[must_use]
    pub const fn token(&self) -> &ResumeToken {
        &self.token
    }

    /// The token that was refused, handed back.
    #[must_use]
    pub fn into_token(self) -> ResumeToken {
        self.token
    }
}

impl Classify for ForeignToken {
    fn class(&self) -> FailureClass {
        FailureClass::Stop
    }
}
