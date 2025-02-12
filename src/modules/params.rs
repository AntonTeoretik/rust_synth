use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};

pub struct SynthParams {
    params: RwLock<HashMap<String, AtomicU32>>,
}

impl SynthParams {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            params: RwLock::new(HashMap::new()),
        })
    }

    pub fn register_param_f32(&self, name: &str, default_value: f32) {
        if let Ok(mut params) = self.params.write() {
            params.insert(name.to_string(), AtomicU32::new(default_value.to_bits()));
        }
    }

    pub fn get_param_f32(&self, name: &str) -> f32 {
        if let Ok(params) = self.params.read() {
            if let Some(p) = params.get(name) {
                return f32::from_bits(p.load(Ordering::Relaxed));
            }
        }
        0.0
    }

    pub fn set_param_f32(&self, name: &str, value: f32) {
        if let Ok(params) = self.params.read() {
            if let Some(param) = params.get(name) {
                param.store(value.to_bits(), Ordering::Relaxed);
            }
        }
    }

    pub fn register_param_u8(&self, name: &str, default_value: u8) {
        if let Ok(mut params) = self.params.write() {
            params.insert(name.to_string(), AtomicU32::new(default_value as u32));
        }
    }

    pub fn get_param_u8(&self, name: &str) -> u8 {
        if let Ok(params) = self.params.read() {
            if let Some(p) = params.get(name) {
                return p.load(Ordering::Relaxed) as u8;
            }
        }
        0
    }

    pub fn set_param_u8(&self, name: &str, value: u8) {
        if let Ok(params) = self.params.read() {
            if let Some(param) = params.get(name) {
                param.store(value as u32, Ordering::Relaxed);
            }
        }
    }
}

impl fmt::Display for SynthParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(params) = self.params.read() {
            writeln!(f, "SynthParams:")?;
            for (key, value) in params.iter() {
                writeln!(f, "{}: {}", key, f32::from_bits(value.load(Ordering::Relaxed)))?;
            }
        }
        Ok(())
    }
}