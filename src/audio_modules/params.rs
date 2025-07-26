use std::collections::HashMap;
use std::fmt;
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Arc, RwLock,
};

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

    pub fn set_oscillator_volume(&self, id: usize, volume: f32) {
        *self
            .oscillator_volumes
            .read()
            .unwrap()
            .get(&id)
            .unwrap()
            .write()
            .unwrap() = volume;
    }
}

impl fmt::Display for SynthParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "SynthParams:")?;
        writeln!(
            f,
            "last_active_note: {}",
            self.last_active_note.load(Ordering::Relaxed)
        )?;
        writeln!(
            f,
            "are_active_notes: {}",
            self.are_active_notes.load(Ordering::Relaxed)
        )?;

        for (id, volume) in self.oscillator_volumes.read().unwrap().iter() {
            writeln!(f, "osc_{}: {}", id, volume.read().unwrap())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillator_volume_management() {
        let params = SynthParams::new();

        // Initialize and test volume for oscillator 0
        params.init_oscillator_volume(0);
        assert_eq!(params.get_oscillator_volume(0), 1.0);

        // Test volume setting
        params.set_oscillator_volume(0, 0.8);
        assert_eq!(params.get_oscillator_volume(0), 0.8);

        // Initialize and test another oscillator
        params.init_oscillator_volume(1);
        assert_eq!(params.get_oscillator_volume(1), 1.0);
    }
}
