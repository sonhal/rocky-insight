use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerContainersMessage {
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
    pub container: String,
    #[serde(rename = "image_id")]
    pub image_id: String,
    pub name: String,
    pub state: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decorations {
    #[serde(rename = "host_uuid")]
    pub host_uuid: String,
    pub username: String,
}