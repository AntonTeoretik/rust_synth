use crate::audio_modules::AudioModule;

pub struct Delay {
    buffer: Vec<f32>,
    buffer_index: usize,
    delay_samples: usize,
    feedback: f32,
    wet_mix: f32,
}

impl Delay {
    pub fn new(delay_time_ms: f32, feedback: f32, wet_mix: f32) -> Self {
        const SAMPLE_RATE: f32 = 44100.0;
        let delay_samples = (SAMPLE_RATE * delay_time_ms / 1000.0) as usize;
        Self {
            buffer: vec![0.0; delay_samples],
            buffer_index: 0,
            delay_samples,
            feedback,
            wet_mix,
        }
    }
}

impl AudioModule for Delay {
    fn process(&mut self, output: &mut [f32]) {
        for sample in output.iter_mut() {
            let delayed_sample = self.buffer[self.buffer_index];
            self.buffer[self.buffer_index] = *sample + delayed_sample * self.feedback;
            self.buffer_index = (self.buffer_index + 1) % self.delay_samples;
            *sample = *sample * (1.0 - self.wet_mix) + delayed_sample * self.wet_mix;
        }
    }
}
