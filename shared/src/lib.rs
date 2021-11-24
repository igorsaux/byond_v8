use serde::{Deserialize, Serialize};

pub mod ipc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    ExecuteCode { code: String },
}
