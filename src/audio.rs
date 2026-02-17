// Audio system using SDL2_mixer
use crate::resources;
use sdl2::mixer::{Channel, Chunk, AUDIO_S16LSB, DEFAULT_CHANNELS};

/// AudioSystem - manages SDL2_mixer and sound playback
pub struct AudioSystem {
    click_sound: Chunk,
    crunch_sound: Chunk,
}

impl AudioSystem {
    /// Initialize the audio system and load sound effects
    pub fn new() -> Result<Self, String> {
        // Initialize SDL2_mixer with 44.1kHz, 16-bit, stereo, 1024 byte chunks
        sdl2::mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)?;

        // Allocate 4 mixing channels for sound effects
        sdl2::mixer::allocate_channels(4);

        // Load sound effects
        let click_sound = resources::load_click_sound()?;
        let crunch_sound = resources::load_crunch_sound()?;

        Ok(AudioSystem {
            click_sound,
            crunch_sound,
        })
    }

    /// Play menu click sound
    pub fn play_click(&self) {
        let _ = Channel::all().play(&self.click_sound, 0);
    }

    /// Play food crunch sound
    pub fn play_crunch(&self) {
        let _ = Channel::all().play(&self.crunch_sound, 0);
    }
}
