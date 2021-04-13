//! ABCI requests and request data.
//!
//! The [`Request`] enum records all possible ABCI requests. Requests that
//! contain data are modeled as a separate struct, to avoid duplication of field
//! definitions.

// IMPORTANT NOTE ON DOCUMENTATION:
//
// The documentation for each request type is adapted from the ABCI Methods and
// Types spec document. However, the same logical request may appear three
// times, as a struct with the request data, as a Request variant, and as a
// CategoryRequest variant. Ideally, the documentation would be copied between
// these automatically, but doing this requires using #[doc = include_str!],
// which is unstable. For now, the Request enum is the source of truth; please
// change the docs there and copy as required.
//
// This is also why certain submodules have #[allow(unused)] imports to bring
// items into scope for doc links, rather than changing the doc links -- it
// allows the doc comments to be copied without editing.

use std::convert::{TryFrom, TryInto};

use super::MethodKind;

// bring into scope for doc links
#[allow(unused)]
use super::types::Snapshot;

mod apply_snapshot_chunk;
mod begin_block;
mod check_tx;
mod deliver_tx;
mod echo;
mod end_block;
mod info;
mod init_chain;
mod load_snapshot_chunk;
mod offer_snapshot;
mod query;

pub use apply_snapshot_chunk::ApplySnapshotChunk;
pub use begin_block::BeginBlock;
pub use check_tx::{CheckTx, CheckTxKind};
pub use deliver_tx::DeliverTx;
pub use echo::Echo;
pub use end_block::EndBlock;
pub use info::Info;
pub use init_chain::InitChain;
pub use load_snapshot_chunk::LoadSnapshotChunk;
pub use offer_snapshot::OfferSnapshot;
pub use query::Query;

/// All possible ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Request {
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
    /// Indicates that any pending requests should be completed and their responses flushed.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#flush)
    Flush,
    /// Requests information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Called on genesis to initialize chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Queries for data from the application at current or past height.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
    /// Signals the beginning of a new block.
    ///
    /// Called prior to any [`DeliverTx`]s. The `header` contains the height,
    /// timestamp, and more -- it exactly matches the Tendermint block header.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
    BeginBlock(BeginBlock),
    /// Check whether a transaction should be included in the mempool.
    ///
    /// `CheckTx` is not involved in processing blocks, only in deciding whether a
    /// transaction should be included in the mempool. Every node runs `CheckTx`
    /// before adding a transaction to its local mempool. The transaction may come
    /// from an external user or another node. `CheckTx` need not execute the
    /// transaction in full, but can instead perform lightweight or statateful
    /// validation (e.g., checking signatures or account balances) instead of more
    /// expensive checks (like running code in a virtual machine).
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
    /// Execute a transaction against the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
    DeliverTx(DeliverTx),
    /// Signals the end of a block.
    ///
    /// Called after all transactions, and prior to each `Commit`.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
    EndBlock(EndBlock),
    /// Signals the application that it can write the queued state transitions
    /// from the block to its state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit,
    /// Asks the application for a list of snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots,
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
    OfferSnapshot(OfferSnapshot),
    /// Used during state sync to retrieve snapshot chunks from peers.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Applies a snapshot chunk.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// The application may want to verify each chunk, e.g., by attaching chunk
    /// hashes in [`Snapshot::metadata`] and/or incrementally verifying contents
    /// against `app_hash`.
    ///
    /// When all chunks have been accepted, Tendermint will make an ABCI [`Info`]
    /// request to verify that `last_block_app_hash` and `last_block_height` match
    /// the expected values, and record the `app_version` in the node state. It then
    /// switches to fast sync or consensus and joins the network.
    ///
    /// If Tendermint is unable to retrieve the next chunk after some time (e.g.,
    /// because no suitable peers are available), it will reject the snapshot and try
    /// a different one via `OfferSnapshot`. The application should be prepared to
    /// reset and accept it or abort as appropriate.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
}

impl Request {
    /// Get the method kind for this request.
    pub fn kind(&self) -> MethodKind {
        use Request::*;
        match self {
            Flush => MethodKind::Flush,
            InitChain(_) => MethodKind::Consensus,
            BeginBlock(_) => MethodKind::Consensus,
            DeliverTx(_) => MethodKind::Consensus,
            EndBlock(_) => MethodKind::Consensus,
            Commit => MethodKind::Consensus,
            CheckTx(_) => MethodKind::Mempool,
            ListSnapshots => MethodKind::Snapshot,
            OfferSnapshot(_) => MethodKind::Snapshot,
            LoadSnapshotChunk(_) => MethodKind::Snapshot,
            ApplySnapshotChunk(_) => MethodKind::Snapshot,
            Info(_) => MethodKind::Info,
            Query(_) => MethodKind::Info,
            Echo(_) => MethodKind::Info,
        }
    }
}

/// The consensus category of ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConsensusRequest {
    /// Called on genesis to initialize chain state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#initchain)
    InitChain(InitChain),
    /// Signals the beginning of a new block.
    ///
    /// Called prior to any [`DeliverTx`]s. The `header` contains the height,
    /// timestamp, and more -- it exactly matches the Tendermint block header.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
    BeginBlock(BeginBlock),
    /// Execute a transaction against the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#delivertx)
    DeliverTx(DeliverTx),
    /// Signals the end of a block.
    ///
    /// Called after all transactions, and prior to each `Commit`.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#endblock)
    EndBlock(EndBlock),
    /// Signals the application that it can write the queued state transitions
    /// from the block to its state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#commit)
    Commit,
}

impl From<ConsensusRequest> for Request {
    fn from(req: ConsensusRequest) -> Self {
        match req {
            ConsensusRequest::InitChain(x) => Self::InitChain(x),
            ConsensusRequest::BeginBlock(x) => Self::BeginBlock(x),
            ConsensusRequest::DeliverTx(x) => Self::DeliverTx(x),
            ConsensusRequest::EndBlock(x) => Self::EndBlock(x),
            ConsensusRequest::Commit => Self::Commit,
        }
    }
}

impl TryFrom<Request> for ConsensusRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::InitChain(x) => Ok(Self::InitChain(x)),
            Request::BeginBlock(x) => Ok(Self::BeginBlock(x)),
            Request::DeliverTx(x) => Ok(Self::DeliverTx(x)),
            Request::EndBlock(x) => Ok(Self::EndBlock(x)),
            Request::Commit => Ok(Self::Commit),
            _ => Err("wrong request type"),
        }
    }
}

/// The mempool category of ABCI requests.
#[derive(Clone, PartialEq, Debug)]
pub enum MempoolRequest {
    /// Check whether a transaction should be included in the mempool.
    ///
    /// `CheckTx` is not involved in processing blocks, only in deciding whether a
    /// transaction should be included in the mempool. Every node runs `CheckTx`
    /// before adding a transaction to its local mempool. The transaction may come
    /// from an external user or another node. `CheckTx` need not execute the
    /// transaction in full, but can instead perform lightweight or statateful
    /// validation (e.g., checking signatures or account balances) instead of more
    /// expensive checks (like running code in a virtual machine).
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#checktx)
    CheckTx(CheckTx),
}

impl From<MempoolRequest> for Request {
    fn from(req: MempoolRequest) -> Self {
        match req {
            MempoolRequest::CheckTx(x) => Self::CheckTx(x),
        }
    }
}

impl TryFrom<Request> for MempoolRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::CheckTx(x) => Ok(Self::CheckTx(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The info category of ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum InfoRequest {
    /// Requests information about the application state.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#info)
    Info(Info),
    /// Queries for data from the application at current or past height.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#query)
    Query(Query),
    /// Echoes a string to test an ABCI implementation.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#echo)
    Echo(Echo),
}

impl From<InfoRequest> for Request {
    fn from(req: InfoRequest) -> Self {
        match req {
            InfoRequest::Info(x) => Self::Info(x),
            InfoRequest::Query(x) => Self::Query(x),
            InfoRequest::Echo(x) => Self::Echo(x),
        }
    }
}

impl TryFrom<Request> for InfoRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::Info(x) => Ok(Self::Info(x)),
            Request::Query(x) => Ok(Self::Query(x)),
            Request::Echo(x) => Ok(Self::Echo(x)),
            _ => Err("wrong request type"),
        }
    }
}

/// The snapshot category of ABCI requests.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SnapshotRequest {
    /// Asks the application for a list of snapshots.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#listsnapshots)
    ListSnapshots,
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
    OfferSnapshot(OfferSnapshot),
    /// Used during state sync to retrieve snapshot chunks from peers.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#loadsnapshotchunk)
    LoadSnapshotChunk(LoadSnapshotChunk),
    /// Applies a snapshot chunk.
    ///
    /// The application can choose to refetch chunks and/or ban P2P peers as
    /// appropriate. Tendermint will not do this unless instructed by the
    /// application.
    ///
    /// The application may want to verify each chunk, e.g., by attaching chunk
    /// hashes in [`Snapshot::metadata`] and/or incrementally verifying contents
    /// against `app_hash`.
    ///
    /// When all chunks have been accepted, Tendermint will make an ABCI [`Info`]
    /// request to verify that `last_block_app_hash` and `last_block_height` match
    /// the expected values, and record the `app_version` in the node state. It then
    /// switches to fast sync or consensus and joins the network.
    ///
    /// If Tendermint is unable to retrieve the next chunk after some time (e.g.,
    /// because no suitable peers are available), it will reject the snapshot and try
    /// a different one via `OfferSnapshot`. The application should be prepared to
    /// reset and accept it or abort as appropriate.
    ///
    /// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#applysnapshotchunk)
    ApplySnapshotChunk(ApplySnapshotChunk),
}

impl From<SnapshotRequest> for Request {
    fn from(req: SnapshotRequest) -> Self {
        match req {
            SnapshotRequest::ListSnapshots => Self::ListSnapshots,
            SnapshotRequest::OfferSnapshot(x) => Self::OfferSnapshot(x),
            SnapshotRequest::LoadSnapshotChunk(x) => Self::LoadSnapshotChunk(x),
            SnapshotRequest::ApplySnapshotChunk(x) => Self::ApplySnapshotChunk(x),
        }
    }
}

impl TryFrom<Request> for SnapshotRequest {
    type Error = &'static str;
    fn try_from(req: Request) -> Result<Self, Self::Error> {
        match req {
            Request::ListSnapshots => Ok(Self::ListSnapshots),
            Request::OfferSnapshot(x) => Ok(Self::OfferSnapshot(x)),
            Request::LoadSnapshotChunk(x) => Ok(Self::LoadSnapshotChunk(x)),
            Request::ApplySnapshotChunk(x) => Ok(Self::ApplySnapshotChunk(x)),
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

impl From<Request> for pb::Request {
    fn from(request: Request) -> pb::Request {
        use pb::request::Value;
        let value = match request {
            Request::Echo(x) => Some(Value::Echo(x.into())),
            Request::Flush => Some(Value::Flush(Default::default())),
            Request::Info(x) => Some(Value::Info(x.into())),
            Request::InitChain(x) => Some(Value::InitChain(x.into())),
            Request::Query(x) => Some(Value::Query(x.into())),
            Request::BeginBlock(x) => Some(Value::BeginBlock(x.into())),
            Request::CheckTx(x) => Some(Value::CheckTx(x.into())),
            Request::DeliverTx(x) => Some(Value::DeliverTx(x.into())),
            Request::EndBlock(x) => Some(Value::EndBlock(x.into())),
            Request::Commit => Some(Value::Commit(Default::default())),
            Request::ListSnapshots => Some(Value::ListSnapshots(Default::default())),
            Request::OfferSnapshot(x) => Some(Value::OfferSnapshot(x.into())),
            Request::LoadSnapshotChunk(x) => Some(Value::LoadSnapshotChunk(x.into())),
            Request::ApplySnapshotChunk(x) => Some(Value::ApplySnapshotChunk(x.into())),
        };
        pb::Request { value }
    }
}

impl TryFrom<pb::Request> for Request {
    type Error = crate::Error;

    fn try_from(request: pb::Request) -> Result<Self, Self::Error> {
        use pb::request::Value;
        match request.value {
            Some(Value::Echo(x)) => Ok(Request::Echo(x.try_into()?)),
            Some(Value::Flush(pb::RequestFlush {})) => Ok(Request::Flush),
            Some(Value::Info(x)) => Ok(Request::Info(x.try_into()?)),
            Some(Value::InitChain(x)) => Ok(Request::InitChain(x.try_into()?)),
            Some(Value::Query(x)) => Ok(Request::Query(x.try_into()?)),
            Some(Value::BeginBlock(x)) => Ok(Request::BeginBlock(x.try_into()?)),
            Some(Value::CheckTx(x)) => Ok(Request::CheckTx(x.try_into()?)),
            Some(Value::DeliverTx(x)) => Ok(Request::DeliverTx(x.try_into()?)),
            Some(Value::EndBlock(x)) => Ok(Request::EndBlock(x.try_into()?)),
            Some(Value::Commit(pb::RequestCommit {})) => Ok(Request::Commit),
            Some(Value::ListSnapshots(pb::RequestListSnapshots {})) => Ok(Request::ListSnapshots),
            Some(Value::OfferSnapshot(x)) => Ok(Request::OfferSnapshot(x.try_into()?)),
            Some(Value::LoadSnapshotChunk(x)) => Ok(Request::LoadSnapshotChunk(x.try_into()?)),
            Some(Value::ApplySnapshotChunk(x)) => Ok(Request::ApplySnapshotChunk(x.try_into()?)),
            None => Err("no request in proto".into()),
        }
    }
}

impl Protobuf<pb::Request> for Request {}
