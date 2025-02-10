mod audio_module;
mod midi_service;
mod oscillator;
mod gate;
mod gain;
use midi_service::MidiService;
use std::sync::{Arc, Mutex};

use audio_module::AudioModule;
use oscillator::Oscillator;
use gate::Gate;
use gain::Gain;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No audio device found");
    let config = device.default_output_config().unwrap();

    println!("Outputting sound to device: {}", device.name().unwrap());

    let volume = 1.0;

    let (midi_service, _midi_connection) = MidiService::new();
    let oscillator = Arc::new(Mutex::new(Oscillator::new(midi_service.clone(), volume)));
    let gate = Arc::new(Mutex::new(Gate::new(midi_service.clone(), 1.0,1.0, 0.5, 1.0)));
    let gain = Arc::new(Mutex::new(Gain::new(20.0)));


    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                oscillator.lock().unwrap().process(data);
                gate.lock().unwrap().process(data);
                gain.lock().unwrap().process(data);
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .expect("Failed to create audio stream");

    stream.play().expect("Failed to start playback");

    println!("Press Enter to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
