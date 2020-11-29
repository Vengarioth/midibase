use serde_derive::*;

pub mod audio;
pub mod scene;
pub mod stream;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GetVersionRequest {
//     #[serde(rename = "request-type")]
//     pub request_type: String,
//     #[serde(rename = "message-id")]
//     pub message_id: u32,
// }

// impl GetVersionRequest {
//     pub fn new(message_id: u32) -> Self {
//         Self {
//             request_type: "GetVersion".to_string(),
//             message_id,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Error {
        #[serde(rename = "message-id")]
        message_id: String,

        error: String,
    },
    Ok {
        #[serde(rename = "message-id")]
        message_id: String,
    },
}
