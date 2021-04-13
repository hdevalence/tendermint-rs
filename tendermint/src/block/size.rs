//! Block size parameters

use {
    crate::serializers,
    serde::{Deserialize, Serialize},
};

/// Block size parameters
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Size {
    /// Maximum number of bytes in a block
    #[serde(with = "serializers::from_str")]
    pub max_bytes: u64,

    /// Maximum amount of gas which can be spent on a block
    #[serde(with = "serializers::from_str")]
    pub max_gas: i64,

    /// This parameter has no value anymore in Tendermint-core
    #[serde(with = "serializers::from_str", default = "Size::default_time_iota_ms")]
    pub time_iota_ms: i64,
}

impl Size {
    /// The default value for the `time_iota_ms` parameter.
    pub fn default_time_iota_ms() -> i64 {
        1000
    }
}
