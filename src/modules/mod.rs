use std::sync::{Arc, Mutex};

pub mod oscillator;
pub mod gate;
pub mod gain;
pub mod lp_filter;
pub mod delay;

pub trait AudioModule : Send + Sync {
    fn process(&mut self, output: &mut [f32]);
}

pub trait Shared {
    fn shared(self) -> Arc<Mutex<Self>>
    where
        Self: Sized,
    {
        Arc::new(Mutex::new(self))
    }
}

impl<T: AudioModule> Shared for T {}