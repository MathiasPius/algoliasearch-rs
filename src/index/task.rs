use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStatus {
    pub status: String,
    pub pending_task: bool,
}
