use crate::midi_service::MidiService;
use crate::modules::audio_module::AudioModule;
use std::f32::consts::PI;
use std::sync::{Arc, RwLock};

fn midi_note_to_freq(note: u8) -> f32 {
    440.0 * (2.0_f32).powf((note as f32 - 69.0) / 12.0)
}

pub struct Oscillator {
    pub frequency: f32,
    pub volume: f32,
    phase: f32,
    midi_service: Arc<RwLock<MidiService>>,
}

impl Oscillator {
    pub fn new(midi_service: Arc<RwLock<MidiService>>, volume: f32) -> Self {
        Self {
            frequency: 440.0,
            volume,
            phase: 0.0,
            midi_service,
        }
    }

    fn update_frequency(&mut self) {
        if let Some(note) = self.midi_service.read().unwrap().last_note_read() {
            self.frequency = midi_note_to_freq(note);
        }
    }
}

impl AudioModule for Oscillator {
    fn process(&mut self, output: &mut [f32]) {
        self.update_frequency();

        for sample in output.iter_mut() {
            *sample = (self.phase * 2.0 * PI).sin() * self.volume;
            self.phase += self.frequency / 44100.0;
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
        }
    }
}
