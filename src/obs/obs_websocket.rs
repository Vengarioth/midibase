use websocket::{ClientBuilder, Message, sync::Writer};
use anyhow::Error;
use std::net::TcpStream;
use crossbeam_channel::unbounded;

use super::api::*;
use crate::config::*;

pub struct ObsWebsocket {
    receiver: crossbeam_channel::Receiver<websocket::OwnedMessage>,
    writer: Writer<TcpStream>,
    next_message_id: usize,
}

impl ObsWebsocket {
    pub fn new(url: &str) -> Result<Self, Error> {

        let client = ClientBuilder::new(url)?.connect_insecure()?;
        let (mut reader, writer) = client.split()?;
        let (sender, receiver) = unbounded();

        std::thread::spawn(move || loop {
            let message = reader.recv_message().unwrap();
            sender.send(message).unwrap();
        });

        Ok(Self {
            receiver,
            writer,
            next_message_id: 0,
        })
    }

    pub fn set_current_scene(&mut self, scene_name: impl Into<String>) -> Result<(), Error> {
        let scene_name = scene_name.into();
        println!("Switching scene to \"{}\"", scene_name);
        let message_id = self.next_message_id;
        self.next_message_id += 1;

        let request = scene::set_current_scene_request::SetCurrentSceneRequest::new(scene_name, message_id);
        let serialized = serde_json::to_string(&request)?;
        self.writer.send_message(&Message::text(serialized))?;

        Ok(())
    }

    pub fn toggle_audio_mute(&mut self, source_name: impl Into<String>) -> Result<(), Error> {
        let source_name = source_name.into();
        println!("Toggling Audio Mute: \"{}\"", source_name);
        let message_id = self.next_message_id;
        self.next_message_id += 1;
        let request = audio::toggle_audio_mute::ToggleAudioMute::new(message_id, source_name);
        let serialized = serde_json::to_string(&request)?;
        self.writer.send_message(&Message::text(serialized))?;
        Ok(())
    }

    pub fn set_audio_mute(&mut self, source_name: impl Into<String>, mute: bool) -> Result<(), Error> {
        let source_name = source_name.into();
        println!("Setting \"{}\" mute to \"{}\"", source_name, mute);
        let message_id = self.next_message_id;
        self.next_message_id += 1;

        let request = audio::set_audio_mute::SetAudioMute::new(message_id, source_name, mute);
        let serialized = serde_json::to_string(&request)?;
        self.writer.send_message(&Message::text(serialized))?;
        Ok(())
    }

    pub fn start_stream(&mut self) -> Result<(), Error> {
        let message_id = self.next_message_id;
        self.next_message_id += 1;

        let request = stream::start_stream::StartStream::new(message_id);
        let serialized = serde_json::to_string(&request)?;
        self.writer.send_message(&Message::text(serialized))?;
        Ok(())
    }

    pub fn stop_stream(&mut self) -> Result<(), Error> {
        let message_id = self.next_message_id;
        self.next_message_id += 1;

        let request = stream::stop_stream::StopStream::new(message_id);
        let serialized = serde_json::to_string(&request)?;
        self.writer.send_message(&Message::text(serialized))?;
        Ok(())
    }

    pub fn set_scene_item_properties(&mut self, scene_name: impl Into<String>, item: impl Into<String>, options: &Vec<SceneItemOption>) -> Result<(), Error> {
        let message_id = self.next_message_id;
        self.next_message_id += 1;
        let scene_name = scene_name.into();
        let item = item.into();
        println!("Updating properties of {} in {}", item, scene_name);
        let mut set_properties = scene::scene_item_properties::SetSceneItemProperties::new(&scene_name, &item, message_id);
        for option in options {
            match option.option_name.as_ref() {
                "visible" => {
                    match option.option_value.as_ref() {
                        "true" => set_properties.set_visible(true),
                        "false" => set_properties.set_visible(false),
                        _ => {}
                    }
                },
                "locked" => {
                    match option.option_value.as_ref() {
                        "true" => set_properties.set_locked(true),
                        "false" => set_properties.set_locked(false),
                        _ => {}
                    }
                }
                "scale" => {
                    let v: Vec<&str> =  option.option_value.split(' ').collect();
                    if v.len() == 2 { 
                        set_properties.set_scale(
                            v[0].parse::<u32>()?, 
                            v[1].parse::<u32>()?
                        );
                    } else {
                        println!("Scale requires 2 values for {} SetSceneItemProperties, please see documentation.", item);
                    }
                }
                "crop" => {
                    let v: Vec<&str> =  option.option_value.split(' ').collect();
                    if v.len() == 4 { 
                        set_properties.set_crop(
                            v[0].parse::<u32>()?, 
                            v[1].parse::<u32>()?, 
                            v[2].parse::<u32>()?, 
                            v[3].parse::<u32>()?
                        );
                    } else {
                        println!("Crop requires 4 values for {} SetSceneItemProperties, please see documentation.", item);
                    }
                }
                "position" => {
                    let v: Vec<&str> =  option.option_value.split(' ').collect();
                    if v.len() == 4 { 
                        set_properties.set_position(
                            v[0].parse::<u32>()?, 
                            v[1].parse::<u32>()?, 
                            v[2].parse::<u32>()?
                        );
                    } else {
                        println!("Position requires 4 values for {} SetSceneItemProperties, please see documentation.", item);
                    }
                }
                "set_bounds" => {
                    let v: Vec<&str> =  option.option_value.split(' ').collect();
                    if v.len() == 4 { 
                        set_properties.set_bounds(
                            v[0].parse::<u32>()?, 
                            v[1].parse::<u32>()?, 
                            v[2].parse::<u32>()?, 
                            v[3].to_string(),
                        );
                    } else {
                        println!("Bounds requires 4 values for {} SetSceneItemProperties, please see documentation.", item);
                    }
                }
                _ => {}
            }
        }
        let set = serde_json::to_string(&set_properties)?;
        self.writer.send_message(&Message::text(set))?;
        Ok(())
    }
    
    pub fn poll(&self) -> Option<String> {
        if let Ok(message) = self.receiver.try_recv() {
            match message {
                websocket::OwnedMessage::Text(text) => {

                    if let Ok(response) = serde_json::from_str::<super::Response>(&text) {
                        if let super::Response::Error{ error, .. } = response {
                            println!("Error: {}", error);
                        }
                    }

                    Some(text)
                },
                websocket::OwnedMessage::Binary(_) => None,
                websocket::OwnedMessage::Close(close_data) => {
                    println!("remote socket closed");
                    if let Some(close_data) = close_data {
                        println!("    [{}] {}", close_data.status_code, close_data.reason);
                    }
                    None
                },
                websocket::OwnedMessage::Ping(_) => None,
                websocket::OwnedMessage::Pong(_) => None,
            }
        } else {
            None
        }
    }
}
