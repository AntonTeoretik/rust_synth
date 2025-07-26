use std::collections::HashMap;
use std::sync::{atomic::AtomicU8, Arc, RwLock};

pub struct SynthParams {
    // MIDI parameters
    pub last_active_note: AtomicU8,
    pub are_active_notes: AtomicU8,

    // Oscillator volumes (wrapped in RwLock for thread-safe mutation)
    oscillator_volumes: RwLock<HashMap<usize, RwLock<f32>>>,
}

impl SynthParams {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            last_active_note: AtomicU8::new(0),
            are_active_notes: AtomicU8::new(0),
            oscillator_volumes: RwLock::new(HashMap::new()),
        })
    }

    pub fn init_oscillator_volume(&self, id: usize) {
        self.oscillator_volumes
            .write()
            .unwrap()
            .insert(id, RwLock::new(1.0));
    }

    pub fn get_oscillator_volume(&self, id: usize) -> f32 {
        *self
            .oscillator_volumes
            .read()
            .unwrap()
            .get(&id)
            .unwrap()
            .read()
            .unwrap()
    }
}
