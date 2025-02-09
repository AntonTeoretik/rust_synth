pub trait AudioModule : Send + Sync {
    fn process(&mut self, input: &[f32], output: &mut [f32]);
}
