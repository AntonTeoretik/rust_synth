use crate::audio_modules::params::{SynthParams, MAX_OSCILLATORS};
use crate::audio_modules::AudioModule;
use std::f32::consts::PI;
use std::sync::atomic::Ordering;
use std::sync::Arc;

const SAMPLE_RATE: f32 = 44100.0;

fn midi_note_to_freq(note: u8) -> f32 {
    440.0 * (2.0_f32).powf((note as f32 - 69.0) / 12.0)
}

pub struct Oscillator {
    params: Arc<SynthParams>,
    phase: f32,
    id: usize,
}

impl Oscillator {
    pub fn new(params: Arc<SynthParams>, id: usize) -> Self {
        // ID must be within array bounds
        assert!(id < MAX_OSCILLATORS);

        Self {
            params,
            phase: 0.0,
            id,
        }
    }

    pub fn get_volume(&self) -> f32 {
        let raw_volume =
            self.params.oscillator_volumes[self.id].load(std::sync::atomic::Ordering::Relaxed);
        raw_volume as f32 / 255.0
    }
}

impl AudioModule for Oscillator {
    fn process(&mut self, output: &mut [f32]) {
        let last_note = self.params.last_active_note.load(Ordering::Relaxed);
        let frequency = midi_note_to_freq(last_note);

        let volume = self.get_volume();
        let sample_rate = SAMPLE_RATE;

        for sample in output.iter_mut() {
            *sample += (self.phase * 2.0 * PI).sin() * volume;
            self.phase += frequency / sample_rate;
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
        }
    }
}
