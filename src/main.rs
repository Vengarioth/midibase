use midir::{
    MidiInput,
};
use anyhow::Error;
use crossbeam_channel::unbounded;
use crossbeam_utils::sync::Parker;

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
            println!("available midi devices:");
            for (index, port) in midi.ports().iter().enumerate() {
                println!("{}: {}", index, midi.port_name(&port)?);
            }
            Ok(())
        },
        Command::Run { device, url, config, poll } => {
            let config = Configuration::load_from(&config)?;

            let midi = MidiInput::new("midibase")?;
            let ports = midi.ports();

            let parker = Parker::new();
            let unparker = parker.unparker().clone();
            let (sender, receiver) = unbounded();
            let _connection = midi.connect(&ports[device], "midibase input", move |_timestamp, message, _| {
                let down = message[0] == 144;
                let button = message[1];
                println!("midi input {} {}", down, button);
                sender.send((down, button)).unwrap();
                unparker.unpark();
            }, ());

            let mut obs = ObsWebsocket::new(&url)?;

            loop {
                parker.park_timeout(std::time::Duration::from_millis(poll));
                
                for (down, button) in receiver.try_recv() {
                    let pressed = button as usize;
                    for command in config.commands.iter() {
                        match command {
                            config::Command::SetCurrentScene { button, scene } => {
                                if down && *button == pressed {
                                    println!("Switching scene to \"{}\"", scene);
                                    obs.set_current_scene(scene)?;
                                }
                            },
                        }
                    }
                }

                if let Some(message) = obs.poll() {
                    dbg!(message);
                }
            }
        }
    }
}
