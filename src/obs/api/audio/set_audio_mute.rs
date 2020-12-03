use serde_derive::*;

// Sets the mute of an audio source to either muted or unmuted using "SetMute" in OBS-Websocket
// https://github.com/Palakis/obs-websocket/blob/4.x-current/docs/generated/protocol.md#setmute

#[derive(Debug, Serialize, Deserialize)]
pub struct SetAudioMute {
    #[serde(rename = "request-type")]
    request_type: String,
    #[serde(rename = "message-id")]
    message_id: String,
    #[serde(rename = "source")]
    source: String,
    #[serde(rename = "mute")]
    mute: bool,
}

impl SetAudioMute {
    pub fn new(message_id: usize, soruce_name: String, mute: bool) -> Self {
        Self {
            request_type: "SetMute".to_string(),
            message_id: message_id.to_string(),
            source: soruce_name.to_string(),
            mute: mute,
        }
    }
}