use std::{fs::File, io::BufReader};

use rodio::{MixerDeviceSink, Player};

pub struct AudioManager {
    sink: MixerDeviceSink,
    background_music: Option<Player>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let sink = rodio::DeviceSinkBuilder::open_default_sink()?;
        Ok(Self {
            sink,
            background_music: None,
        })
    }

    pub fn play_background_music(&mut self, filepath: &str) {
        self.background_music = self.play_music(filepath);
    }

    pub fn play_sound(&self, filepath: &str) {
        if let Some(player) = self.play_music(filepath) {
            player.detach();
        }
    }

    pub fn play_music(&self, filepath: &str) -> Option<Player> {
        let file = File::open(filepath).ok()?;
        let reader = BufReader::new(file);
        let source = rodio::Decoder::new(reader).ok()?;

        let player = rodio::Player::connect_new(self.sink.mixer());
        player.append(source);
        Some(player)
    }
}
