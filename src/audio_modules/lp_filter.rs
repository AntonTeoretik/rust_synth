use crate::audio_modules::AudioModule;
use crate::audio_modules::gate::Gate;

pub struct LowPassFilter {
    cutoff: f32,
    res_factor: f32,
    prev_output: f32,
    mod_depth: f32,
    gate: Gate,
}

impl LowPassFilter {
    pub fn new(cutoff: f32, res_factor: f32, mod_depth: f32, gate: Gate) -> Self {
        
        Self {
            cutoff,
            res_factor,
            prev_output: 0.0,
            mod_depth,
            gate,
        }
    }
}

impl AudioModule for LowPassFilter {
    fn process(&mut self, output: &mut [f32]) {

        const SAMPLE_RATE: f32 = 44100.0;
        
        for sample in output.iter_mut() {
            self.gate.next_envelope_value();
            let modulated_cutoff = self.cutoff * (2.0_f32).powf(self.gate.envelope * self.mod_depth);
            let alpha = 2.0 * std::f32::consts::PI * modulated_cutoff / (SAMPLE_RATE + 2.0 * std::f32::consts::PI * modulated_cutoff);
        
            let filtered = alpha * *sample + (1.0 - alpha) * self.prev_output;
            self.prev_output = filtered + (filtered - self.prev_output) * self.res_factor;
            *sample = self.prev_output;
        }
    }
}
