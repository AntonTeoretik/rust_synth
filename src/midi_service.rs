use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::{Arc, Mutex, RwLock};

use crate::audio_modules::params::SynthParams;

pub struct MidiService {
    active_notes: Vec<u8>,
    last_note: Option<u8>,
    params: Arc<SynthParams>,
}

pub type SharedMidiService = Arc<RwLock<MidiService>>;
pub type SharedMidiConnection = Arc<Mutex<Option<MidiInputConnection<()>>>>;

pub const P_LAST_ACTIVE_NOTE: &str = "LastActiveNote";
pub const P_ARE_ACTIVE_NOTES: &str = "AreActiveNotes";

impl MidiService {
    pub fn new(
        params: Arc<SynthParams>,
    ) -> (
        Arc<RwLock<MidiService>>,
        Arc<Mutex<Option<MidiInputConnection<()>>>>,
    ) {
        params.register_param_u8(P_LAST_ACTIVE_NOTE, 0);
        params.register_param_u8(P_ARE_ACTIVE_NOTES, 0);

        let service = Arc::new(RwLock::new(Self {
            active_notes: Vec::new(),
            last_note: None,
            params,
        }));

        let midi_connection = Arc::new(Mutex::new(None)); // Изначально соединения нет
        Self::start_midi_listener(Arc::clone(&service), Arc::clone(&midi_connection));

        (service, midi_connection)
    }

    fn start_midi_listener(service: SharedMidiService, connection: SharedMidiConnection) {
        let mut midi_in = MidiInput::new("MIDI Service").expect("Failed to open MIDI input");
        midi_in.ignore(Ignore::None);

        let ports = midi_in.ports();
        if ports.is_empty() {
            println!("No MIDI input devices found.");
            return;
        }

        let port = &ports[0];
        println!("Using MIDI device: {}", midi_in.port_name(port).unwrap());

        let service_clone = Arc::clone(&service);
        let conn = midi_in.connect(
            port,
            "midi_service",
            move |_, message, _| {
                Self::handle_message(message, &service_clone);
            },
            (),
        );

        *connection.lock().unwrap() = conn.ok();
    }

    fn handle_message(message: &[u8], service_clone: &SharedMidiService) {
        if message.len() < 3 {
            return;
        }
        let status = message[0];
        let note = message[1];
        let velocity = message[2];

        let mut service = service_clone.write().unwrap();

        if (status & 0xF0 == 0x90) && (status & 0x0F == 0) && velocity > 0 {
            if !service.active_notes.contains(&note) {
                service.active_notes.push(note);
                service.last_note = Some(note);
            }
        } else if ((status & 0xF0 == 0x80) || ((status & 0xF0 == 0x90) && velocity == 0))
            && (status & 0x0F == 0)
        {
            if let Some(pos) = service.active_notes.iter().position(|&n| n == note) {
                service.active_notes.remove(pos);
            }

            if let Some(&new_note) = service.active_notes.last() {
                service.last_note = Some(new_note);
            } else {
                service.last_note = None;
            }
        }

        service.params.set_param_u8(P_LAST_ACTIVE_NOTE, note);
        service
            .params
            .set_param_u8(P_ARE_ACTIVE_NOTES, !service.active_notes.is_empty() as u8);
    }

    pub fn active_notes_read(&self) -> Vec<u8> {
        self.active_notes.clone()
    }
}
