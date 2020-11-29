use serde_derive::*;
use anyhow::Error;
use std::path::Path;


#[derive(Debug, Serialize, Deserialize)]
pub struct MidibaseEvent {
    pub button: usize,
    pub on_down: bool,
    pub commands: Vec<Command>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command")]
pub enum Command {
    SetCurrentScene {
        scene: String,
    },
    PlaySound {
        file: String,
        volume: f32,
    },
    ToggleAudioMute {
        audio_source: String,
    },
    SetAudioMute {
        audio_source: String,
        mute: bool
    },
    StartStream,
    StopStream,
    SetSceneItemProperties{
        scene_name: String,
        item: String,
        options: Vec<SceneItemOption>
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneItemOption{
    pub option_name: String,
    pub option_value: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub midibase_events: Vec<MidibaseEvent>
}

impl Configuration {
    pub fn load_from(path: &Path) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path)?;
        let deserialized = serde_json::from_str(&contents)?;
        Ok(deserialized)
    }
}