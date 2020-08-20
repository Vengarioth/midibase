use midir::{
    MidiInput,
};
use anyhow::Error;
use websocket::ClientBuilder;
use crossbeam_channel::unbounded;

mod obs;
mod command;
mod config;

use obs::*;
use command::*;
use config::Configuration;

fn main() -> Result<(), Error> {
    match Command::parse() {
        Command::List => {
            let midi = MidiInput::new("midibase")?;
            for (index, port) in midi.ports().iter().enumerate() {
                println!("{}: {}", index, midi.port_name(&port)?);
            }
            Ok(())
        },
        Command::Run { device, url, config } => {
            let config = Configuration::load_from(&config)?;

            let midi = MidiInput::new("midibase")?;
            let ports = midi.ports();

            let (sender, receiver) = unbounded();
            let _connection = midi.connect(&ports[device], "midibase input", move |_timestamp, message, _| {
                let down = message[0] == 144;
                let button = message[1];

                sender.send((down, button)).unwrap();
            }, ());

            let mut client = ClientBuilder::new(&url)?.connect_insecure()?;

            loop {
                for (down, button) in receiver.try_recv() {
                    let pressed = button as usize;
                    for command in config.commands.iter() {
                        match command {
                            config::Command::SetCurrentScene { button, scene } => {
                                if down && *button == pressed {
                                    println!("Switching scene to \"{}\"", scene);
                                    let request = SetCurrentSceneRequest::new(scene.to_string(), 0);
                                    let serialized = serde_json::to_string(&request)?;
                                    client.send_message(&websocket::Message::text(serialized))?;
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
