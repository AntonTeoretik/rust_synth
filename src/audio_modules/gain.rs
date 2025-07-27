use crate::audio_modules::AudioModule;

pub struct Gain {
    gain: f32,
}

impl Gain {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }
}

impl AudioModule for Gain {
    fn process(&mut self, output: &mut [f32]) {
        for sample in output.iter_mut() {
            *sample *= self.gain;
            // Мягкое ограничение с использованием сигмоиды
            *sample /= 1.0 + sample.abs();
        }
    }
}
