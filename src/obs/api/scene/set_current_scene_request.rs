use serde_derive::*;

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