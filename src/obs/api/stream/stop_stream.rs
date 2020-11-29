use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopStream {
    #[serde(rename = "request-type")]
    request_type: String,
    #[serde(rename = "message-id")]
    message_id: String
}

impl StopStream {
    pub fn new(message_id: usize) -> Self {
        Self {
            request_type: "StopStream".to_string(),
            message_id: message_id.to_string(),
        }
    }
}