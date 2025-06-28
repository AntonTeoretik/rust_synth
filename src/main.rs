mod audio_modules;
mod midi_service;

use std::sync::Arc;

use midi_service::MidiService;

use audio_modules::{
    delay::Delay, gain::Gain, gate::Gate, lp_filter::LowPassFilter, oscillator::Oscillator,
    params::SynthParams, AudioModule, Shared,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No audio device found");
    let config = device.default_output_config().unwrap();

    println!("Outputting sound to device: {}", device.name().unwrap());

    let params = SynthParams::new();

    let (midi_service, _midi_connection) = MidiService::new(Arc::clone(&params));
    let oscillator = Oscillator::new(Arc::clone(&params), 0).shared();
    let gate = Gate::new(midi_service.clone(), 0.1, 1.0, 1.0, 20.0).shared();
    let gain = Gain::new(20.0).shared();

    let filter_gate = Gate::new(midi_service.clone(), 0.3, 10.0, 1.0, 5.0);
    let lp_filter = LowPassFilter::new(55.0, 0.99, 10.0, filter_gate).shared();

    let delay = Delay::new(1000.0, 0.4, 0.4).shared();

    println!("{}", &oscillator.lock().unwrap().volume_param);
    params.set_param_f32(&oscillator.lock().unwrap().volume_param, 1.0);

    println!("{}", params);

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
