//! Error types

use crate::account;
use crate::vote;
use alloc::string::String;
use core::num::TryFromIntError;
use flex_error::{define_error, DisplayOnly};
use serde::{Deserialize, Serialize};

define_error! {
    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    Error {
        Crypto
            |_| { format_args!("cryptographic error") },

        InvalidKey
            { detail: String }
            |e| { format_args!("invalid key: {}", e) },

        Length
            |_| { format_args!("length error") },

        Parse
            { data: String }
            | e | { format_args!("error parsing data: {}", e.data) },

        ParseInt
            { data: String }
            [ DisplayOnly<core::num::ParseIntError>]
            | e | { format_args!("error parsing int data: {}", e.data) },

        Protocol
            { detail: String }
            |e| { format_args!("protocol error: {}", e.detail) },

        // When the oldtime feature is disabled, the chrono::oldtime::OutOfRangeError
        // type is private and cannot be referred:
        // https://github.com/chronotope/chrono/pull/541
        DurationOutOfRange
            |_| { format_args!("duration value out of range") },

        EmptySignature
            |_| { format_args!("empty signature") },

        SignatureInvalid
            { detail: String }
            |e| { format_args!("bad signature: {}", e.detail) },

        InvalidMessageType
            |_| { format_args!("invalid message type") },

        NegativeHeight
            [ DisplayOnly<TryFromIntError> ]
            |_| { format_args!("negative height") },

        NegativeRound
            [ DisplayOnly<TryFromIntError> ]
            |_| { format_args!("negative round") },

        NegativePolRound
            |_| { format_args!("negative POL round") },

        NegativeValidatorIndex
            [ DisplayOnly<TryFromIntError> ]
            |_| { format_args!("negative validator index") },

        InvalidHashSize
            |_| { format_args!("invalid hash: expected hash size to be 32 bytes") },

        NonZeroTimestamp
            | _ | { "absent commitsig has non-zero timestamp" },

        InvalidAccountIdLength
            |_| { format_args!("invalid account ID length") },

        InvalidSignatureIdLength
            |_| { format_args!("invalid signature ID length") },

        IntegerOverflow
            [ DisplayOnly<TryFromIntError> ]
            |_| { format_args!("integer overflow") },

        TimestampOverflow
            [ DisplayOnly<TryFromIntError> ]
            |_| { format_args!("timestamp overflow") },

        TimestampConversion
            |_| { format_args!("timestamp conversion error") },

        NoVoteFound
            |_| { format_args!("no vote found") },

        NoProposalFound
            |_| { format_args!("no proposal found") },

        InvalidAppHashLength
            |_| { format_args!("invalid app hash length") },

        InvalidPartSetHeader
            { detail : String }
            |_| { format_args!("invalid part set header") },

        MissingHeader
            |_| { format_args!("missing header field") },

        MissingData
            |_| { format_args!("missing data field") },

        MissingEvidence
            |_| { format_args!("missing evidence field") },

        MissingTimestamp
            |_| { format_args!("missing timestamp field") },

        InvalidTimestamp
            { reason: String }
            | e | { format_args!("invalid timestamp: {}", e.reason) },

        InvalidBlock
            { reason: String }
            | e | { format_args!("invalid block: {}", e.reason) },

        MissingVersion
            |_| { format_args!("missing version") },

        InvalidFirstHeader
            |_| { format_args!("last_block_id is not null on first height") },

        InvalidSignature
            { reason: String }
            | e | { format_args!("invalid signature: {}", e.reason) },

        InvalidValidatorAddress
            |_| { format_args!("invalid validator address") },

        InvalidSignedHeader
            |_| { format_args!("invalid signed header") },

        InvalidEvidence
            |_| { format_args!("invalid evidence") },

        BlockIdFlag
            |_| { format_args!("invalid block id flag") },

        NegativePower
            [ DisplayOnly<TryFromIntError> ]
            |_| { format_args!("negative power") },

        UnsupportedKeyType
            |_| { format_args!("unsupported key type" ) },

        RawVotingPowerMismatch
            { raw: vote::Power, computed: vote::Power }
            |e| { format_args!("mismatch between raw voting ({0:?}) and computed one ({1:?})", e.raw, e.computed) },

        MissingPublicKey
            |_| { format_args!("missing public key") },

        InvalidValidatorParams
            |_| { format_args!("invalid validator parameters") },

        InvalidVersionParams
            |_| { format_args!("invalid version parameters") },

        NegativeMaxAgeNum
            [ DisplayOnly<TryFromIntError> ]
            |_| { format_args!("negative max_age_num_blocks") },

        MissingMaxAgeDuration
            |_| { format_args!("missing max_age_duration") },

        ProposerNotFound
            { account: account::Id }
            |e| { format_args!("proposer with address '{0}' no found in validator set", e.account) },

        ChronoParse
            [ DisplayOnly<chrono::ParseError> ]
            |_| { format_args!("chrono parse error") },

        SubtleEncoding
            [ DisplayOnly<subtle_encoding::Error> ]
            |_| { format_args!("subtle encoding error") },

        Signature
            [ DisplayOnly<signature::Error> ]
            |_| { format_args!("signature error") },

        TrustThresholdTooLarge
            |_| { "trust threshold is too large (must be <= 1)" },

        UndefinedTrustThreshold
            |_| { "undefined trust threshold (denominator cannot be 0)" },

        TrustThresholdTooSmall
            |_| { "trust threshold too small (must be >= 1/3)" },

        Other
            { msg: &'static str }
            | e | { format_args!("error: {}", e.msg) },
    }
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Error {
        Error::other(msg)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_never: std::convert::Infallible) -> Error {
        unreachable!("Infallible can never be constructed")
    }
}
