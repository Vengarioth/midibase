use serde_derive::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Crop {
    top: u32,
    bottom: u32,
    left: u32,
    right: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bounds {
    #[serde(rename = "type")]
    bounds_type: String,
    alignment: u32,
    x: u32,
    y: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    x: u32,
    y: u32,
    alignment: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    x: u32,
    y: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetSceneItemProperties {
    #[serde(rename = "request-type")]
    request_type: String,
    #[serde(rename = "message-id")]
    message_id: String,
    #[serde(rename = "scene-name")]
    scene_name: String,
    item: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scale: Option<Scale>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crop: Option<Crop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bounds: Option<Bounds>,
}

//by default all properties are optional, and set to none. 
impl SetSceneItemProperties {
    pub fn new(scene_name: &String, item: &String, message_id: usize) -> Self {
        Self {
            request_type: "SetSceneItemProperties".to_string(),
            message_id: message_id.to_string(),
            scene_name: scene_name.to_string(),
            item: item.to_string(),
            visible: None,
            locked: None,
            position: None,
            scale: None,
            crop: None,
            bounds: None,
        }
    }

    pub fn set_visible(&mut self, visible: bool){
        self.visible = Some(visible);
    }

    pub fn set_locked(&mut self, locked: bool){
        self.locked = Some(locked);
    }

    pub fn set_position(&mut self, x: u32, y: u32, alignment: u32 ){
        self.position = Some(Position {
            x,
            y,
            alignment
        });
    }

    pub fn set_scale(&mut self, x: u32, y: u32){
        self.scale = Some(Scale {
            x,
            y,
        });
    }

    pub fn set_crop(&mut self, top: u32, bottom: u32, left: u32, right: u32 ){
        self.crop = Some(Crop {
            top,
            bottom,
            left,
            right,
        });
    }
    
    pub fn set_bounds(&mut self, x: u32, y: u32, alignment: u32, bounds_type: String){
        self.bounds = Some(Bounds {
            x,
            y,
            alignment,
            bounds_type
        });
    }
}

//TODO: Figure out how to Recieve and Deserialize GetSceneItemProperties, Useful for toggling properties
// #[derive(Debug, Serialize, Deserialize)]
// pub struct GetSceneItemPropertiesRequest{
//     #[serde(rename = "request-type")]
//     request_type: String,
//     #[serde(rename = "message-id")]
//     message_id: String,
//     #[serde(rename = "scene-name")]
//     scene_name: String,
//     #[serde(rename = "item")]
//     item: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct GetSceneItemPropertiesResponse{}

// impl GetSceneItemProperties {
//     pub fn new(scene_name: String, item: String, message_id: usize) -> Self {
//         Self {
//             request_type: "GetSceneItemProperties".to_string(),
//             message_id: message_id.to_string(),
//             scene_name,
//             item
//         }
//     }
// }