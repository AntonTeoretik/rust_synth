use std::sync::{Arc, Mutex};

pub mod delay;
pub mod gain;
pub mod gate;
pub mod lp_filter;
pub mod oscillator;
pub mod state;

pub trait AudioModule: Send + Sync {
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
