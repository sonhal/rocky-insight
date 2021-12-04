use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaDiskUseMessage {
    pub snapshot: Vec<Snapshot>,
    pub action: String,
    pub name: String,
    pub host_identifier: String,
    pub calendar_time: String,
    pub unix_time: i64,
    pub epoch: i64,
    pub counter: i64,
    pub numerics: bool,
    pub decorations: Decorations,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    #[serde(rename = "bytes_used_by_kafka")]
    pub bytes_used_by_kafka: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decorations {
    #[serde(rename = "host_uuid")]
    pub host_uuid: String,
    pub username: String,
}
