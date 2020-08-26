use rodio::Sink;
use std::fs::File;
use std::io::BufReader;


pub struct Soundboard {
    sink_pool: Vec<Sink>
}

impl Soundboard {
    pub fn new() -> Soundboard{
        let device = rodio::default_output_device().unwrap();
        let mut sinks: Vec<Sink> = Vec::new();
        sinks.push(Sink::new(&device));
        sinks.push(Sink::new(&device));
        sinks.push(Sink::new(&device));
        sinks.push(Sink::new(&device));
        sinks.push(Sink::new(&device));
        Self {
            sink_pool: sinks
        }
    }
    pub fn play_sound(&self, file : &str, volume: &f32){
        println!("Playing Sound \"{}\"", file);
        let mut num = 0;
        for sink in &self.sink_pool {
            println!("Sink {} Empty?: {}", num, sink.empty());
            if sink.empty() {
                let file = File::open(file).unwrap();
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                // rodio::play_raw(&device, source.convert_samples());
                sink.set_volume(*volume);
                sink.append(source);
                sink.play();
                break;
            }
            num += 1;
        }
    }
}
