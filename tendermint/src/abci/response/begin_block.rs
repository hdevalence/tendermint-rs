// XXX(hdevalence): fix this import
use super::super::event::Event;

/// Returns events that occurred when beginning a new block.
///
/// [ABCI documentation](https://docs.tendermint.com/master/spec/abci/abci.html#beginblock)
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct BeginBlock {
    /// Events that occurred while beginning the block.
    pub events: Vec<Event>,
}

// =============================================================================
// Protobuf conversions
// =============================================================================

// XXX(hdevalence): these all use &'static str for now, this should be fixed
// to align with the crate's error-handling strategy.

use std::convert::{TryFrom, TryInto};
use tendermint_proto::abci as pb;
use tendermint_proto::Protobuf;

impl From<BeginBlock> for pb::ResponseBeginBlock {
    fn from(begin_block: BeginBlock) -> Self {
        Self {
            events: begin_block.events.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<pb::ResponseBeginBlock> for BeginBlock {
    type Error = crate::Error;

    fn try_from(begin_block: pb::ResponseBeginBlock) -> Result<Self, Self::Error> {
        Ok(Self {
            events: begin_block
                .events
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Protobuf<pb::ResponseBeginBlock> for BeginBlock {}
