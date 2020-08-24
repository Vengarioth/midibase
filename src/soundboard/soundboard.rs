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
    }
}
