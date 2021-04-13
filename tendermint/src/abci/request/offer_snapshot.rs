use bytes::Bytes;

use super::super::types::Snapshot;

// bring into scope for doc links
#[allow(unused)]
use super::ApplySnapshotChunk;

/// Offers a list of snapshots to the application.
///
/// `OfferSnapshot` is called when bootstrapping a node using state sync. The
/// application may accept or reject snapshots as appropriate. Upon accepting,
/// Tendermint will retrieve and apply snapshot chunks via
/// [`ApplySnapshotChunk`]. The application may also choose to reject a snapshot
/// in the chunk response, in which case it should be prepared to accept further
/// `OfferSnapshot` calls.
///
/// Only `app_hash` can be trusted, as it has been verified by the light client.
/// Any other data can be spoofed by adversaries, so applications should employ
/// additional verification schemes to avoid denial-of-service attacks. The
/// verified `app_hash` is automatically checked against the restored application
/// at the end of snapshot restoration.
///
/// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
///
/// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OfferSnapshot {
    /// The snapshot offered for restoration.
    pub snapshot: Snapshot,
    /// The light client verified app hash for this height.
    // XXX(hdevalence): replace with apphash
    pub app_hash: Bytes,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// XXX(hdevalence): these all use &'static str for now, this should be fixed
// to align with the crate's error-handling strategy.

use std::convert::{TryFrom, TryInto};
use tendermint_proto::abci as pb;
use tendermint_proto::Protobuf;

impl From<OfferSnapshot> for pb::RequestOfferSnapshot {
    fn from(offer_snapshot: OfferSnapshot) -> Self {
        Self {
            snapshot: Some(offer_snapshot.snapshot.into()),
            app_hash: offer_snapshot.app_hash,
        }
    }
}

impl TryFrom<pb::RequestOfferSnapshot> for OfferSnapshot {
    type Error = crate::Error;

    fn try_from(offer_snapshot: pb::RequestOfferSnapshot) -> Result<Self, Self::Error> {
        Ok(Self {
            snapshot: offer_snapshot
                .snapshot
                .ok_or("missing snapshot")?
                .try_into()?,
            app_hash: offer_snapshot.app_hash,
        })
    }
}

impl Protobuf<pb::RequestOfferSnapshot> for OfferSnapshot {}
