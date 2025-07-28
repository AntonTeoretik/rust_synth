use std::sync::{
  atomic::{AtomicBool, AtomicU8},
  Arc,
};

pub const MAX_OSCILLATORS: usize = 8;
pub const MAX_VOLUME: u8 = 255;

pub struct SynthState {
  // MIDI parameters
  pub last_active_note: AtomicU8,
  pub are_active_notes: AtomicBool,

  // Oscillator volumes (fixed-size array)
  pub oscillator_volumes: [AtomicU8; MAX_OSCILLATORS],
}

impl SynthState {
  pub fn new() -> Arc<Self> {
    let volumes = [(); MAX_OSCILLATORS].map(|_| AtomicU8::new(MAX_VOLUME));

    Arc::new(Self {
      last_active_note: AtomicU8::new(0),
      are_active_notes: AtomicBool::new(false),
      oscillator_volumes: volumes,
    })
  }
}
