// Audio system - placeholder for now
// Will be implemented with SDL2 audio in Phase 2

pub struct AudioSystem {
    // TODO: Add SDL2 audio context and sound effects
}

impl AudioSystem {
    pub fn new() -> Result<Self, String> {
        Ok(AudioSystem {})
    }

    pub fn play_click(&self) {
        // TODO: Play menu click sound
        println!("[Audio] Click");
    }

    pub fn play_crunch(&self) {
        // TODO: Play food crunch sound
        println!("[Audio] Crunch");
    }
}
