use crate::modules::params::SynthParams;
use crate::modules::AudioModule;
use std::f32::consts::PI;
use std::sync::Arc;
use std::sync::atomic::Ordering;

fn midi_note_to_freq(note: u8) -> f32 {
    440.0 * (2.0_f32).powf((note as f32 - 69.0) / 12.0)
}

pub struct Oscillator {
    params: Arc<SynthParams>,
    phase: f32,
}

impl Oscillator {
    pub fn new(params: Arc<SynthParams>) -> Self {
        Self {
            params,
            phase: 0.0,
        }
    }
}

impl AudioModule for Oscillator {
    fn process(&mut self, output: &mut [f32]) {
        let frequency = f32::from_bits(self.params.main_freq.load(Ordering::Relaxed));
        let volume = f32::from_bits(self.params.gain.load(Ordering::Relaxed));
        let sample_rate = f32::from_bits(self.params.sample_rate.load(Ordering::Relaxed));

        for sample in output.iter_mut() {
            *sample = (self.phase * 2.0 * PI).sin() * volume;
            self.phase += frequency / sample_rate;
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
        }
    }
}
