use kira::{AudioError, Value, instance::*, manager::*, sound::*};
pub struct Soundboard {
    audio_manager: AudioManager,
}

impl Soundboard {
    pub fn new() -> Result<Soundboard, AudioError> {
        let audio_manager = AudioManager::<()>::new(AudioManagerSettings::default())?;
        Ok(Self {
            audio_manager: audio_manager,
        })
    }
    pub fn play_sound(&mut self, file : &str, volume : f64) -> Result<(), AudioError> {
        let sound_id = self.audio_manager.add_sound(Sound::from_file(file, SoundSettings::default())?)?;
        self.audio_manager.play_sound(sound_id, InstanceSettings{
            volume: Value::from(volume),
            ..InstanceSettings::default()
        })?;
        Ok(())
    }
}

