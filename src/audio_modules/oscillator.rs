use crate::midi_service::P_LAST_ACTIVE_NOTE;
use crate::audio_modules::params::SynthParams;
use crate::audio_modules::AudioModule;
use std::f32::consts::PI;
use std::sync::Arc;

const SAMPLE_RATE : f32 = 44100.0;

fn midi_note_to_freq(note: u8) -> f32 {
    440.0 * (2.0_f32).powf((note as f32 - 69.0) / 12.0)
}

pub struct Oscillator {
    params: Arc<SynthParams>,
    phase: f32,
    sample_rate: f32,
    pub volume_param: String,
}

impl Oscillator {
    pub fn new(params: Arc<SynthParams>, id: usize) -> Self {
        let volume_param = format!("osc_{}_volume", id);
        
        params.register_param_f32(&volume_param, 440.0);
        
        Self {
            params,
            phase: 0.0,
            sample_rate: 0.0,
            volume_param
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.params.get_param_f32(&self.volume_param)
    }
}

impl AudioModule for Oscillator {
    fn process(&mut self, output: &mut [f32]) {
        let last_note = self.params.get_param_u8(P_LAST_ACTIVE_NOTE);
        let frequency = midi_note_to_freq(last_note);

        let volume = self.get_volume();
        let sample_rate = SAMPLE_RATE;

        for sample in output.iter_mut() {
            *sample = (self.phase * 2.0 * PI).sin() * volume;
            self.phase += frequency / sample_rate;
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
        }
    }
}
