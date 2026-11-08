//! Envelope bytes in postcard, for the two network peers.
//!
//! [`serve_sync`](crate::serve_sync) and [`RemoteSyncPeer`](crate::RemoteSyncPeer)
//! speak the same bytes, so they encode and decode through the same two
//! functions, and a change to one is a change to both.
//!
//! # Why the version is read before the envelope is decoded
//!
//! WF-8, and the measurement on [`WireError::UNSUPPORTED_FORMAT_VERSION`]:
//! postcard's error is fieldless, so a version refusal raised inside
//! [`Envelope`]'s `Deserialize` reaches a postcard caller as a bare
//! `SerdeDeCustom`, indistinguishable from any other malformed buffer. Reading
//! the `u16` off the front with `take_from_bytes` and handing it to
//! [`check_format_version`] is the route that keeps a peer one version ahead
//! apart from garbage. Both are class [`Stop`](FailureClass::Stop): the same
//! bytes fail the same way on every retry. They are separate variants because
//! the remedy differs, and the variant is what tells the operator which one to
//! apply: [`CodecError::UnsupportedVersion`] asks for an upgrade on one side,
//! [`CodecError::Malformed`] for a fix to whatever mangled the buffer.

use alloc::vec::Vec;

use serde::Serialize;
use serde::de::DeserializeOwned;

use super::{Envelope, Route, WireError, check_format_version};
use crate::classify::{Classify, FailureClass};

/// Why envelope bytes could not be read or written.
///
/// Every arm is [`Stop`](FailureClass::Stop). The same bytes fail the same way
/// on every retry, and the remedy — an upgrade on one side, or a fix to the
/// transport that mangled the buffer — is an operator's.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CodecError {
    /// The sender stamped a format version this build does not speak, and
    /// nothing past the version was decoded.
    #[error(transparent)]
    UnsupportedVersion(#[from] WireError),

    /// The bytes are not a postcard envelope holding the message `route`
    /// carries. A mis-routed buffer can land here, or can decode silently as
    /// the wrong message (ADR-0097 §1): routing is the transport's to get right.
    #[error("the {route} message is not a well-formed envelope: {cause}")]
    #[non_exhaustive]
    Malformed {
        /// The route the bytes travelled on.
        route: Route,
        /// What the decoder said.
        cause: PostcardError,
    },

    /// A whole envelope decoded and bytes were left over. Refused rather than
    /// ignored: a buffer that carries more than one message is a transport
    /// that framed it wrong, and whatever the extra bytes were is lost.
    #[error("the {route} message decoded with {extra} bytes left over")]
    #[non_exhaustive]
    TrailingBytes {
        /// The route the bytes travelled on.
        route: Route,
        /// How many bytes followed the envelope.
        extra: usize,
    },

    /// A message could not be encoded.
    #[error("the {route} message could not be encoded: {cause}")]
    #[non_exhaustive]
    Unencodable {
        /// The route the message was for.
        route: Route,
        /// What the encoder said.
        cause: PostcardError,
    },
}

impl Classify for CodecError {
    fn class(&self) -> FailureClass {
        match self {
            Self::UnsupportedVersion(_)
            | Self::Malformed { .. }
            | Self::TrailingBytes { .. }
            | Self::Unencodable { .. } => FailureClass::Stop,
        }
    }
}

/// What postcard reported, carried without making postcard's own error type a
/// part of this crate's public API.
///
/// A newtype with a private field rather than `postcard::Error` in
/// [`CodecError`]'s arms: a public field of a foreign type would tie this
/// crate's semver to postcard's, and a postcard 2 would be a breaking change
/// here. The text is what a log wants, and it is all this exposes.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{0}")]
pub struct PostcardError(postcard::Error);

/// `message` in an envelope stamped [`FORMAT_VERSION`](super::FORMAT_VERSION),
/// as postcard bytes.
pub(crate) fn encode<T: Serialize>(route: Route, message: &T) -> Result<Vec<u8>, CodecError> {
    postcard::to_stdvec(&Envelope::new(message)).map_err(|cause| CodecError::Unencodable {
        route,
        cause: PostcardError(cause),
    })
}

/// The message inside an envelope that `bytes` holds whole.
///
/// The version is read and checked first (WF-8), so a buffer from a newer peer
/// is [`CodecError::UnsupportedVersion`], never [`CodecError::Malformed`].
pub(crate) fn decode<T: DeserializeOwned>(route: Route, bytes: &[u8]) -> Result<T, CodecError> {
    let malformed = |cause| CodecError::Malformed {
        route,
        cause: PostcardError(cause),
    };
    let (found, _) = postcard::take_from_bytes::<u16>(bytes).map_err(malformed)?;
    check_format_version(found)?;
    let (envelope, rest) = postcard::take_from_bytes::<Envelope<T>>(bytes).map_err(malformed)?;
    if !rest.is_empty() {
        return Err(CodecError::TrailingBytes {
            route,
            extra: rest.len(),
        });
    }
    Ok(envelope.into_message())
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use super::{CodecError, decode, encode};
    use crate::wire::{FORMAT_VERSION, PullRequest, ResumeToken, Route, WireError};
    use crate::{Classify, FailureClass};

    #[test]
    fn a_message_survives_the_round_trip() {
        let request = PullRequest::after(ResumeToken::new("7"));

        let bytes = encode(Route::Pull, &request).expect("a pull request encodes");

        assert_eq!(decode::<PullRequest>(Route::Pull, &bytes), Ok(request));
    }

    /// A peer one version ahead: the typed refusal, not a malformed buffer.
    #[test]
    fn an_unknown_version_is_refused_by_number_before_the_message() {
        // Version 999 as a postcard varint, then bytes that are no message at
        // all: if the decoder read past the version it would say `Malformed`.
        let mut bytes = postcard::to_stdvec(&999_u16).expect("a u16 encodes");
        bytes.extend([0xFF, 0xFF, 0xFF]);

        let refused = decode::<PullRequest>(Route::Pull, &bytes).expect_err("version 999");

        assert_eq!(
            refused,
            CodecError::UnsupportedVersion(WireError::UnsupportedFormatVersion {
                found: 999,
                supported: FORMAT_VERSION,
            })
        );
        assert_eq!(refused.class(), FailureClass::Stop);
    }

    #[test]
    fn bytes_that_are_no_envelope_are_malformed() {
        for garbage in [vec![], vec![0x80], vec![FORMAT_VERSION_BYTE, 0x01, 0xFF]] {
            let refused = decode::<PullRequest>(Route::Pull, &garbage).expect_err("garbage");

            assert!(
                matches!(
                    refused,
                    CodecError::Malformed {
                        route: Route::Pull,
                        ..
                    }
                ),
                "{garbage:?} gave {refused:?}"
            );
            assert_eq!(refused.class(), FailureClass::Stop);
        }
    }

    #[test]
    fn bytes_past_the_envelope_are_refused() {
        let mut bytes = encode(Route::Pull, &PullRequest::from_start()).expect("encodes");
        bytes.push(0x00);

        let refused = decode::<PullRequest>(Route::Pull, &bytes).expect_err("one byte extra");

        assert_eq!(
            refused,
            CodecError::TrailingBytes {
                route: Route::Pull,
                extra: 1,
            }
        );
    }

    /// `FORMAT_VERSION` as its one-byte postcard varint, for building buffers
    /// by hand. Pinned by `the_version_byte_is_the_format_version`.
    const FORMAT_VERSION_BYTE: u8 = 0x02;

    #[test]
    fn the_version_byte_is_the_format_version() {
        let bytes = postcard::to_stdvec(&FORMAT_VERSION).expect("a u16 encodes");

        assert_eq!(bytes, vec![FORMAT_VERSION_BYTE]);
    }
}
