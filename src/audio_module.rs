use std::sync::{Arc, Mutex};


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

// 🛠 Автоматически реализуем Shared для всех AudioModule!
impl<T: AudioModule> Shared for T {}