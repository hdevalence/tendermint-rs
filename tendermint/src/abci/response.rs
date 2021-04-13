//! ABCI responses and response data.
//!
//! The [`Response`] enum records all possible ABCI responses. Responses that
//! contain data are modeled as a separate struct, to avoid duplication of field
//! definitions.

// IMPORTANT NOTE ON DOCUMENTATION:
//
// The documentation for each response type is adapted from the ABCI Methods and
// Types spec document. However, the same logical response may appear three
// times, as a struct with the response data, as a Response variant, and as a
// CategoryResponse variant. Ideally, the documentation would be copied between
// these automatically, but doing this requires using #[doc = include_str!],
// which is unstable. For now, the Response enum is the source of truth; please
// change the docs there and copy as required.
//
// This is also why certain submodules have #[allow(unused)] imports to bring
// items into scope for doc links, rather than changing the doc links -- it
// allows the doc comments to be copied without editing.

use std::convert::{TryFrom, TryInto};

// bring into scope for doc links
#[allow(unused)]
use super::types::Snapshot;

mod apply_snapshot_chunk;
mod begin_block;
mod check_tx;
mod commit;
mod deliver_tx;
mod echo;
mod end_block;
mod exception;
mod info;
mod init_chain;
mod list_snapshots;
mod load_snapshot_chunk;
mod offer_snapshot;
mod query;

pub use apply_snapshot_chunk::{ApplySnapshotChunk, ApplySnapshotChunkResult};
pub use begin_block::BeginBlock;
pub use check_tx::CheckTx;
pub use commit::Commit;
pub use deliver_tx::DeliverTx;
pub use echo::Echo;
pub use end_block::EndBlock;
pub use exception::Exception;
pub use info::Info;
pub use init_chain::InitChain;
pub use list_snapshots::ListSnapshots;
pub use load_snapshot_chunk::LoadSnapshotChunk;
pub use offer_snapshot::OfferSnapshot;
pub use query::Query;

/// All possible ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Response {
    /// Undocumented, nondeterministic.
    Exception(Exception),
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
    /// Indicates that all pending requests have been completed with their responses flushed.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#flush)
    Flush,
    /// Returns information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Returned on genesis after initializing chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Returns data queried from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
    /// Returns events that occurred when beginning a new block.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
    BeginBlock(BeginBlock),
    /// Returns the result of checking a transaction for mempool inclusion.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
    /// Returns events that occurred while executing a transaction against the
    /// application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
    DeliverTx(DeliverTx),
    /// Returns validator updates that occur after the end of a block.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
    EndBlock(EndBlock),
    /// Returns the result of persisting the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit(Commit),
    /// Returns a list of local state snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots(ListSnapshots),
    /// Returns the application's response to a snapshot offer.
    ///
    /// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
    ///
    /// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
    OfferSnapshot(OfferSnapshot),
    /// Returns a snapshot chunk from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Returns the result of applying a snapshot chunk and associated data.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
}

/// The consensus category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConsensusResponse {
    /// Returned on genesis after initializing chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Returns events that occurred when beginning a new block.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
    BeginBlock(BeginBlock),
    /// Returns events that occurred while executing a transaction against the
    /// application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
    DeliverTx(DeliverTx),
    /// Returns validator updates that occur after the end of a block.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
    EndBlock(EndBlock),
    /// Returns the result of persisting the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit(Commit),
}

impl From<ConsensusResponse> for Response {
    fn from(req: ConsensusResponse) -> Self {
        match req {
            ConsensusResponse::InitChain(x) => Self::InitChain(x),
            ConsensusResponse::BeginBlock(x) => Self::BeginBlock(x),
            ConsensusResponse::DeliverTx(x) => Self::DeliverTx(x),
            ConsensusResponse::EndBlock(x) => Self::EndBlock(x),
            ConsensusResponse::Commit(x) => Self::Commit(x),
        }
    }
}

impl TryFrom<Response> for ConsensusResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::InitChain(x) => Ok(Self::InitChain(x)),
            Response::BeginBlock(x) => Ok(Self::BeginBlock(x)),
            Response::DeliverTx(x) => Ok(Self::DeliverTx(x)),
            Response::EndBlock(x) => Ok(Self::EndBlock(x)),
            Response::Commit(x) => Ok(Self::Commit(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The mempool category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum MempoolResponse {
    /// Returns the result of checking a transaction for mempool inclusion.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
}

impl From<MempoolResponse> for Response {
    fn from(req: MempoolResponse) -> Self {
        match req {
            MempoolResponse::CheckTx(x) => Self::CheckTx(x),
        }
    }
}

impl TryFrom<Response> for MempoolResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::CheckTx(x) => Ok(Self::CheckTx(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The info category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InfoResponse {
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
    /// Returns information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Returns data queried from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
}

impl From<InfoResponse> for Response {
    fn from(req: InfoResponse) -> Self {
        match req {
            InfoResponse::Echo(x) => Self::Echo(x),
            InfoResponse::Info(x) => Self::Info(x),
            InfoResponse::Query(x) => Self::Query(x),
        }
    }
}

impl TryFrom<Response> for InfoResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::Echo(x) => Ok(Self::Echo(x)),
            Response::Info(x) => Ok(Self::Info(x)),
            Response::Query(x) => Ok(Self::Query(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The snapshot category of ABCI responses.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SnapshotResponse {
    /// Returns a list of local state snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots(ListSnapshots),
    /// Returns the application's response to a snapshot offer.
    ///
    /// See also the [`Snapshot`] data type and the [ABCI state sync documentation][ssd].
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#offersnapshot)
    ///
    /// [ssd]: https://docs.tendermint.com/master/spec/abci/apps.html#state-sync
    OfferSnapshot(OfferSnapshot),
    /// Returns a snapshot chunk from the application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Returns the result of applying a snapshot chunk and associated data.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
}

impl From<SnapshotResponse> for Response {
    fn from(req: SnapshotResponse) -> Self {
        match req {
            SnapshotResponse::ListSnapshots(x) => Self::ListSnapshots(x),
            SnapshotResponse::OfferSnapshot(x) => Self::OfferSnapshot(x),
            SnapshotResponse::LoadSnapshotChunk(x) => Self::LoadSnapshotChunk(x),
            SnapshotResponse::ApplySnapshotChunk(x) => Self::ApplySnapshotChunk(x),
        }
    }
}

impl TryFrom<Response> for SnapshotResponse {
    type Error = &'static str;
    fn try_from(req: Response) -> Result<Self, Self::Error> {
        match req {
            Response::ListSnapshots(x) => Ok(Self::ListSnapshots(x)),
            Response::OfferSnapshot(x) => Ok(Self::OfferSnapshot(x)),
            Response::LoadSnapshotChunk(x) => Ok(Self::LoadSnapshotChunk(x)),
            Response::ApplySnapshotChunk(x) => Ok(Self::ApplySnapshotChunk(x)),
            _ => Err("wrong request type"),
        }
    }
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// XXX(hdevalence): these all use &'static str for now, this should be fixed
// to align with the crate's error-handling strategy.

use tendermint_proto::abci as pb;
use tendermint_proto::Protobuf;

impl From<Response> for pb::Response {
    fn from(response: Response) -> pb::Response {
        use pb::response::Value;
        let value = match response {
            Response::Exception(x) => Some(Value::Exception(x.into())),
            Response::Echo(x) => Some(Value::Echo(x.into())),
            Response::Flush => Some(Value::Flush(Default::default())),
            Response::Info(x) => Some(Value::Info(x.into())),
            Response::InitChain(x) => Some(Value::InitChain(x.into())),
            Response::Query(x) => Some(Value::Query(x.into())),
            Response::BeginBlock(x) => Some(Value::BeginBlock(x.into())),
            Response::CheckTx(x) => Some(Value::CheckTx(x.into())),
            Response::DeliverTx(x) => Some(Value::DeliverTx(x.into())),
            Response::EndBlock(x) => Some(Value::EndBlock(x.into())),
            Response::Commit(x) => Some(Value::Commit(x.into())),
            Response::ListSnapshots(x) => Some(Value::ListSnapshots(x.into())),
            Response::OfferSnapshot(x) => Some(Value::OfferSnapshot(x.into())),
            Response::LoadSnapshotChunk(x) => Some(Value::LoadSnapshotChunk(x.into())),
            Response::ApplySnapshotChunk(x) => Some(Value::ApplySnapshotChunk(x.into())),
        };
        pb::Response { value }
    }
}

impl TryFrom<pb::Response> for Response {
    type Error = crate::Error;

    fn try_from(response: pb::Response) -> Result<Self, Self::Error> {
        use pb::response::Value;
        match response.value {
            Some(Value::Exception(x)) => Ok(Response::Exception(x.try_into()?)),
            Some(Value::Echo(x)) => Ok(Response::Echo(x.try_into()?)),
            Some(Value::Flush(_)) => Ok(Response::Flush),
            Some(Value::Info(x)) => Ok(Response::Info(x.try_into()?)),
            Some(Value::InitChain(x)) => Ok(Response::InitChain(x.try_into()?)),
            Some(Value::Query(x)) => Ok(Response::Query(x.try_into()?)),
            Some(Value::BeginBlock(x)) => Ok(Response::BeginBlock(x.try_into()?)),
            Some(Value::CheckTx(x)) => Ok(Response::CheckTx(x.try_into()?)),
            Some(Value::DeliverTx(x)) => Ok(Response::DeliverTx(x.try_into()?)),
            Some(Value::EndBlock(x)) => Ok(Response::EndBlock(x.try_into()?)),
            Some(Value::Commit(x)) => Ok(Response::Commit(x.try_into()?)),
            Some(Value::ListSnapshots(x)) => Ok(Response::ListSnapshots(x.try_into()?)),
            Some(Value::OfferSnapshot(x)) => Ok(Response::OfferSnapshot(x.try_into()?)),
            Some(Value::LoadSnapshotChunk(x)) => Ok(Response::LoadSnapshotChunk(x.try_into()?)),
            Some(Value::ApplySnapshotChunk(x)) => Ok(Response::ApplySnapshotChunk(x.try_into()?)),
            None => Err("no response in proto".into()),
        }
    }
}

impl Protobuf<pb::Response> for Response {}
