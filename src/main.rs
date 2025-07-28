mod audio_modules;
mod midi_service;

use std::sync::{Arc, Mutex};

use midi_service::MidiService;

use audio_modules::{
  delay::Delay, gain::Gain, gate::Gate, lp_filter::LowPassFilter, oscillator::Oscillator,
  state::SynthState, AudioModule, Shared,
};
use cpal::{
  traits::{DeviceTrait, HostTrait, StreamTrait},
  Device, Stream, SupportedStreamConfig,
};
use midir::MidiInputConnection;

type SharedSynthParams = Arc<SynthState>;
type SynthCore = (SharedSynthParams, MidiInputConnection<()>);

fn init_audio_device() -> (Device, SupportedStreamConfig) {
  let host = cpal::default_host();
  let device = host.default_output_device().expect("No audio device found");
  let config = device.default_output_config().unwrap();

  println!("Outputting sound to device: {}", device.name().unwrap());

  (device, config)
}

fn init_synth_core() -> SynthCore {
  let params = SynthState::new();
  let midi_connection = MidiService::start_midi_listener(Arc::clone(&params));

  (params, midi_connection)
}

fn build_audio_modules(params: &Arc<SynthState>) -> Vec<Arc<Mutex<dyn AudioModule>>> {
  let osc = Oscillator::new(Arc::clone(params), 0).shared();
  let gate = Gate::new(Arc::clone(params), 0.05, 0.2, 0.0, 1.0).shared();
  let gain = Gain::new(1.0).shared();

  let filter_gate = Gate::new(Arc::clone(params), 0.01, 0.1, 0.1, 5.0);
  let lp_filter = LowPassFilter::new(55.0, 0.99, 10.0, filter_gate).shared();

  let delay = Delay::new(1000.0, 0.1, 0.5).shared();

  vec![osc, gate, gain, lp_filter, delay]
}

fn start_audio_stream(
  device: cpal::Device,
  config: cpal::StreamConfig,
  modules: Vec<Arc<Mutex<dyn AudioModule>>>,
) -> Stream {
  device
    .build_output_stream(
      &config,
      move |data: &mut [f32], _| {
        for m in &modules {
          m.lock().unwrap().process(data);
        }
      },
      |err| eprintln!("Stream error: {}", err),
      None,
    )
    .expect("Failed to build stream")
}

fn main() {
  let (device, config) = init_audio_device();
  let (params, _midi_connection) = init_synth_core();

  let modules = build_audio_modules(&params);

  let stream = start_audio_stream(device, config.config(), modules);
  stream.play().unwrap();

  println!("Press Enter to exit.");
  std::io::stdin().read_line(&mut String::new()).unwrap();
}
