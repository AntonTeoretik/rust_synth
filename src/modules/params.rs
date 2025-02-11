use std::sync::{atomic::{AtomicBool, AtomicU32, Ordering}, Arc};

pub struct SynthParams {
    pub cutoff: AtomicU32,
    pub resonance: AtomicU32,
    pub gain: AtomicU32,
    pub main_freq: AtomicU32,
    pub sample_rate: AtomicU32,
    pub are_active_notes: AtomicBool,
}

impl SynthParams {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            cutoff: AtomicU32::new(1000.0f32.to_bits()),
            resonance: AtomicU32::new(0.5f32.to_bits()),
            gain: AtomicU32::new(1.0f32.to_bits()),
            main_freq: AtomicU32::new(440.0f32.to_bits()),
            sample_rate: AtomicU32::new(44100.0f32.to_bits()),
            are_active_notes: AtomicBool::new(false)
        })
    }

    pub fn set_sample_rate(&self, value: f32) {
        self.sample_rate.store(value.to_bits(), Ordering::Relaxed);
    }

    pub fn get_sample_rate(&self) -> f32 {
        f32::from_bits(self.sample_rate.load(Ordering::Relaxed))
    }
}
