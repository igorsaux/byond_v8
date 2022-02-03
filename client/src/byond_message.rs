use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum ByondMessage {
    NewIsolate { uuid: String },
    Error { message: String },
}
