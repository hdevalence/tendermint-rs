/// Requests information about the application state.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Info {
    /// The Tendermint software semantic version.
    pub version: String,
    /// The Tendermint block protocol version.
    pub block_version: u64,
    /// The Tendermint p2p protocol version.
    pub p2p_version: u64,
    /// The Tendermint ABCI semantic version.
    pub abci_version: String,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// XXX(hdevalence): these all use &'static str for now, this should be fixed
// to align with the crate's error-handling strategy.

use std::convert::TryFrom;
use tendermint_proto::abci as pb;
use tendermint_proto::Protobuf;

impl From<Info> for pb::RequestInfo {
    fn from(info: Info) -> Self {
        Self {
            version: info.version,
            block_version: info.block_version,
            p2p_version: info.p2p_version,
            abci_version: info.abci_version,
        }
    }
}

impl TryFrom<pb::RequestInfo> for Info {
    type Error = &'static str;

    fn try_from(info: pb::RequestInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            version: info.version,
            block_version: info.block_version,
            p2p_version: info.p2p_version,
            abci_version: info.abci_version,
        })
    }
}

impl Protobuf<pb::RequestInfo> for Info {}
