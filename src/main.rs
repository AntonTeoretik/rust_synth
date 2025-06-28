mod audio_modules;
mod midi_service;

use std::sync::{Arc, Mutex, RwLock};

use midi_service::MidiService;

use audio_modules::{
    delay::Delay, gain::Gain, gate::Gate, lp_filter::LowPassFilter, oscillator::Oscillator,
    params::SynthParams, AudioModule, Shared,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Stream, SupportedStreamConfig,
};
use midir::MidiInputConnection;

fn init_audio_device() -> (Device, SupportedStreamConfig) {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No audio device found");
    let config = device.default_output_config().unwrap();

    println!("Outputting sound to device: {}", device.name().unwrap());

    (device, config)
}

fn init_synth_core() -> (
    Arc<SynthParams>,
    Arc<RwLock<MidiService>>,
    Arc<Mutex<Option<MidiInputConnection<()>>>>,
) {
    let params = SynthParams::new();
    let (midi_service, _midi_connection) = MidiService::new(Arc::clone(&params));

    (params, midi_service, _midi_connection)
}

fn build_audio_modules(
    params: &Arc<SynthParams>,
    midi_service: &Arc<RwLock<MidiService>>,
) -> Vec<Arc<Mutex<dyn AudioModule>>> {
    let osc = Oscillator::new(Arc::clone(&params), 0).shared();
    let gate = Gate::new(midi_service.clone(), 0.1, 1.0, 1.0, 20.0).shared();
    let gain = Gain::new(20.0).shared();

    let filter_gate = Gate::new(midi_service.clone(), 0.3, 10.0, 1.0, 5.0);
    let lp_filter = LowPassFilter::new(55.0, 0.99, 10.0, filter_gate).shared();

    let delay = Delay::new(1000.0, 0.4, 0.4).shared();

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
    let (params, midi_service, _midi_connection) = init_synth_core();

    let modules = build_audio_modules(&params, &midi_service);

    let stream = start_audio_stream(device, config.config(), modules);
    stream.play().unwrap();

    println!("Press Enter to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
