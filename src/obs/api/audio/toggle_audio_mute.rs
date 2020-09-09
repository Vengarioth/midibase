use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToggleAudioMute {
    #[serde(rename = "request-type")]
    request_type: String,
    #[serde(rename = "message-id")]
    message_id: String,
    #[serde(rename = "source")]
    source: String,
}

impl ToggleAudioMute {
    pub fn new(message_id: usize, soruce_name: String) -> Self {
        Self {
            request_type: "ToggleMute".to_string(),
            message_id: message_id.to_string(),
            source: soruce_name.to_string(),
        }
    }
}