use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::atomic::Ordering;
use std::sync::{Arc, RwLock};

use crate::audio_modules::params::SynthParams;

pub struct MidiService {
    active_notes: Vec<u8>,
    params: Arc<SynthParams>,
}

type SharedMidiService = Arc<RwLock<MidiService>>;

pub type SharedMidiConnection = Arc<MidiInputConnection<()>>;

impl MidiService {
    pub fn initialize(params: Arc<SynthParams>) -> SharedMidiConnection {
        let service = Self {
            active_notes: Vec::new(),
            params,
        };

        Self::start_midi_listener(service)
    }

    fn start_midi_listener(service: Self) -> SharedMidiConnection {
        let mut midi_in = MidiInput::new("MIDI Service").expect("Failed to open MIDI input");
        midi_in.ignore(Ignore::None);

        let ports = midi_in.ports();
        if ports.is_empty() {
            panic!("No MIDI input devices found");
        }

        let port = &ports[0];
        println!("Using MIDI device: {}", midi_in.port_name(port).unwrap());

        let shared_service = Arc::new(RwLock::new(service));

        let conn = midi_in
            .connect(
                port,
                "midi_service",
                move |_, message, _| {
                    Self::handle_message(message, &shared_service);
                },
                (),
            )
            .expect("Failed to create MIDI connection");

        Arc::new(conn)
    }

    fn handle_message(message: &[u8], shared_service: &SharedMidiService) {
        if message.len() < 3 {
            return;
        }
        let status = message[0];
        let note = message[1];
        let velocity = message[2];

        let mut service = shared_service.write().unwrap();

        if (status & 0xF0 == 0x90) && (status & 0x0F == 0) && velocity > 0 {
            if !service.active_notes.contains(&note) {
                service.active_notes.push(note);
            }
        } else if ((status & 0xF0 == 0x80) || ((status & 0xF0 == 0x90) && velocity == 0))
            && (status & 0x0F == 0)
        {
            if let Some(pos) = service.active_notes.iter().position(|&n| n == note) {
                service.active_notes.remove(pos);
            }
        }

        service
            .params
            .last_active_note
            .store(note, Ordering::Relaxed);
        service
            .params
            .are_active_notes
            .store(!service.active_notes.is_empty() as u8, Ordering::Relaxed);
    }
}
