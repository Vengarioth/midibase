<<<<<<< HEAD
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;

pub struct Soundboard {
    sink_pool: Vec<Sink>,
}

impl Soundboard {
    pub fn new(sink_count: u64) -> Soundboard {
        let device = rodio::default_output_device().unwrap();
        let mut sinks: Vec<Sink> = Vec::new();
        for _ in 1..(sink_count + 1) {
            sinks.push(Sink::new(&device));
        }
        Self { sink_pool: sinks }
    }
    pub fn play_sound(&self, file: &str, volume: &f32) {
        println!("Playing Sound \"{}\"", file);
        let mut num = 0;
        let mut found_sink = false;
        for sink in &self.sink_pool {
            if sink.empty() {
                let file = File::open(file).unwrap();
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                // rodio::play_raw(&device, source.convert_samples());
                sink.set_volume(*volume);
                sink.append(source);
                sink.play();
                found_sink = true;
                break;
            }
            num += 1;
        }
        if !found_sink {
            println!("`Too many sounds are playing at once,\
                No sound was played, please wait for sounds to end,\
                or increase the number of audio pools by using the\
                \"--sinks\" option at startup. See \"midibase run --help\"\
                for more information on the \"--sinks\" option"
            );
        }
=======
use rodio::Source;
use std::fs::File;
use std::io::BufReader;


pub struct Soundboard {}

impl Soundboard {
    pub fn play_sound(file : &str){
        println!("Playing Sound \"{}\"", file);
        let device = rodio::default_output_device().unwrap();
        
        let file = File::open(file).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        rodio::play_raw(&device, source.convert_samples());
>>>>>>> d41269db2af93c4f508f96674a4d8e9d1698f02e
    }
}
