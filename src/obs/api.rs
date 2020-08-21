use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetVersionRequest {
    #[serde(rename = "request-type")]
    pub request_type: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
}

impl GetVersionRequest {
    pub fn new(message_id: u32) -> Self {
        Self {
            request_type: "GetVersion".to_string(),
            message_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCurrentSceneRequest {
    #[serde(rename = "request-type")]
    request_type: String,
    #[serde(rename = "message-id")]
    message_id: String,
    #[serde(rename = "scene-name")]
    scene_name: String,
}

impl SetCurrentSceneRequest {
    pub fn new(scene_name: String, message_id: usize) -> Self {
        Self {
            request_type: "SetCurrentScene".to_string(),
            message_id: message_id.to_string(),
            scene_name,
        }
    }
}

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
