use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct StartStream {
    #[serde(rename = "request-type")]
    request_type: String,
    #[serde(rename = "message-id")]
    message_id: String
}

impl StartStream {
    pub fn new(message_id: usize) -> Self {
        Self {
            request_type: "StartStream".to_string(),
            message_id: message_id.to_string(),
        }
    }
}