use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCurrentSceneRequest {
    #[serde(rename = "request-type")]
    pub request_type: String,
    #[serde(rename = "scene-name")]
    pub scene_name: String,
    #[serde(rename = "message-id")]
    pub message_id: u32,
}

impl SetCurrentSceneRequest {
    pub fn new(scene_name: String, message_id: u32) -> Self {
        Self {
            request_type: "SetCurrentScene".to_string(),
            scene_name,
            message_id,
        }
    }
}
