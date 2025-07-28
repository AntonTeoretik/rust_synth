use crate::audio_modules::AudioModule;
use crate::SynthParams;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub struct Gate {
  attack: f32,
  decay: f32,
  sustain: f32,
  release: f32,
  pub envelope: f32,
  state: GateState,
  synth_params: Arc<SynthParams>,
}

#[derive(PartialEq)]
enum GateState {
  Attack,
  Decay,
  Sustain,
  Release,
}

impl Gate {
  pub fn new(
    synth_params: Arc<SynthParams>,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
  ) -> Self {
    Self {
      attack,
      decay,
      sustain,
      release,
      envelope: 0.0,
      state: GateState::Release,
      synth_params,
    }
  }

  fn update_state(&mut self) {
    let has_active_notes = self.synth_params.are_active_notes.load(Ordering::Relaxed);
    match self.state {
      GateState::Release if has_active_notes => self.state = GateState::Attack,
      GateState::Attack | GateState::Decay | GateState::Sustain if !has_active_notes => {
        self.state = GateState::Release
      },
      _ => {},
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
      },
      GateState::Decay => {
        self.envelope -= 1.0 / (self.decay * 44100.0);
        if self.envelope <= self.sustain {
          self.envelope = self.sustain;
          self.state = GateState::Sustain;
        }
      },
      GateState::Sustain => {},
      GateState::Release => {
        self.envelope -= 1.0 / (self.release * 44100.0);
        if self.envelope <= 0.0 {
          self.envelope = 0.0;
        }
      },
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
