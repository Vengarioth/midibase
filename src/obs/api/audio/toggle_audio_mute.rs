use serde_derive::*;

// Toggles the mute of an audio source to either muted or unmuted using "ToggleMute" in OBS-Websocket
// https://github.com/Palakis/obs-websocket/blob/4.x-current/docs/generated/protocol.md#togglemute

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