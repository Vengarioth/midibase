use websocket::{ClientBuilder, Message, sync::Writer};
use anyhow::Error;
use std::net::TcpStream;
use crossbeam_channel::unbounded;

use super::api::*;

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
        let message_id = self.next_message_id;
        self.next_message_id += 1;

        let request = SetCurrentSceneRequest::new(scene_name.into(), message_id);
        let serialized = serde_json::to_string(&request)?;
        dbg!(&serialized);
        self.writer.send_message(&Message::text(serialized))?;

        Ok(())
    }

    pub fn poll(&self) -> Option<String> {
        if let Ok(message) = self.receiver.try_recv() {
            match message {
                websocket::OwnedMessage::Text(text) => Some(text),
                websocket::OwnedMessage::Binary(_) => None,
                websocket::OwnedMessage::Close(_) => None,
                websocket::OwnedMessage::Ping(_) => None,
                websocket::OwnedMessage::Pong(_) => None,
            }
        } else {
            None
        }
    }
}
