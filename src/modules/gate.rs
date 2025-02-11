use crate::midi_service::MidiService;
use crate::modules::AudioModule;
use std::sync::{Arc, RwLock};

pub struct Gate {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    pub envelope: f32,
    state: GateState,
    midi_service: Arc<RwLock<MidiService>>,
}

#[derive(PartialEq)]
enum GateState {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Gate {
    pub fn new(midi_service: Arc<RwLock<MidiService>>, attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            envelope: 0.0,
            state: GateState::Release,
            midi_service,
        }
    }

    fn update_state(&mut self) {
        let has_active_notes = !self.midi_service.read().unwrap().active_notes_read().is_empty();
        match self.state {
            GateState::Release if has_active_notes => self.state = GateState::Attack,
            GateState::Attack | GateState::Decay | GateState::Sustain if !has_active_notes => self.state = GateState::Release,
            _ => {}
        }
    }

    pub fn next_envelope_value(&mut self) {
        self.update_state();
        match self.state {
            GateState::Attack => {
                self.envelope += 1.0 / (self.attack * 44100.0);
                if self.envelope >= 1.0 {
                    self.envelope = 1.0;
                    self.state = GateState::Decay;
                }
            }
            GateState::Decay => {
                self.envelope -= 1.0 / (self.decay * 44100.0);
                if self.envelope <= self.sustain {
                    self.envelope = self.sustain;
                    self.state = GateState::Sustain;
                }
            }
            GateState::Sustain => {}
            GateState::Release => {
                self.envelope -= 1.0 / (self.release * 44100.0);
                if self.envelope <= 0.0 {
                    self.envelope = 0.0;
                }
            }
        }
    }
}

impl AudioModule for Gate {
    fn process(&mut self, output: &mut [f32]) {
        for sample in output.iter_mut() {
            self.next_envelope_value();
            *sample *= self.envelope;
        }
    }
}