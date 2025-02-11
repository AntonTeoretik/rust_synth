mod audio_module;
mod midi_service;
mod oscillator;

mod gate;
mod gain;
mod delay;
mod lp_filter;

use midi_service::MidiService;
use std::sync::{Arc, Mutex};

use audio_module::{AudioModule, Shared};
use oscillator::Oscillator;
use gate::Gate;
use gain::Gain;
use delay::Delay;
use lp_filter::LowPassFilter;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No audio device found");
    let config = device.default_output_config().unwrap();

    println!("Outputting sound to device: {}", device.name().unwrap());

    let volume = 1.0;

    let (midi_service, _midi_connection) = MidiService::new();
    let oscillator = Oscillator::new(midi_service.clone(), volume).shared();
    let gate = Gate::new(midi_service.clone(), 0.1, 1.0, 1.0, 20.0).shared();
    let gain = Gain::new(20.0).shared();

    let filter_gate = Gate::new(midi_service.clone(), 0.3, 10.0, 1.0, 5.0);
    let lp_filter = LowPassFilter::new(55.0, 0.99, 10.0, filter_gate).shared();

    let delay = Delay::new(1000.0, 0.4, 0.4).shared();

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                oscillator.lock().unwrap().process(data);
                gate.lock().unwrap().process(data);
                gain.lock().unwrap().process(data);
                lp_filter.lock().unwrap().process(data);
                delay.lock().unwrap().process(data);
            },
            |err| eprintln!("Stream error: {}", err),
            None,
        )
        .expect("Failed to create audio stream");

    stream.play().expect("Failed to start playback");

    println!("Press Enter to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
