mod audio_module;
mod midi_service;
mod oscillator;
use midi_service::MidiService;
use std::sync::{Arc, Mutex};

use audio_module::AudioModule;
use oscillator::Oscillator;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No audio device found");
    let config = device.default_output_config().unwrap();

    println!("Outputting sound to device: {}", device.name().unwrap());

    let volume = 0.1;

    let midi_service = MidiService::new();
    let midi_clone = Arc::clone(&midi_service);

    let module = Arc::new(Mutex::new(Oscillator::new(midi_service, volume)));
    let module_clone = Arc::clone(&module);

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let is_gate_open = midi_clone.is_open();
                if is_gate_open {
                    let mut module = module_clone.lock().unwrap();
                    module.process(&[], data);
                } else {
                    data.fill(0.0);
                }
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .expect("Failed to create audio stream");

    stream.play().expect("Failed to start playback");

    println!("Generating sine wave (440 Hz)... Press Enter to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
