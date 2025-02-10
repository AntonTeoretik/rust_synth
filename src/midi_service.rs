use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use midir::{Ignore, MidiInput, MidiInputConnection};

pub struct MidiService {
    is_gate_open: AtomicBool,
    active_notes: Vec<u8>,
    last_note: Option<u8>,
}

pub type SharedMidiService = Arc<RwLock<MidiService>>;
pub type SharedMidiConnection = Arc<Mutex<Option<MidiInputConnection<()>>>>;

impl MidiService {
    pub fn new() -> (Arc<RwLock<MidiService>>, Arc<Mutex<Option<MidiInputConnection<()>>>>) {
        let service = Arc::new(RwLock::new(Self {
            is_gate_open: AtomicBool::new(false),
            active_notes: Vec::new(),
            last_note: None,
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
                    service.is_gate_open.store(true, Ordering::Relaxed);
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
                        service.is_gate_open.store(false, Ordering::Relaxed);
                    }
                }
            },
            (),
        );

        *connection.lock().unwrap() = conn.ok();
    }

    pub fn is_open(&self) -> bool {
        self.is_gate_open.load(Ordering::Relaxed)
    }

    pub fn last_note_read(&self) -> Option<u8> {
        self.last_note
    }

    pub fn active_notes_read(&self) -> Vec<u8> {
        self.active_notes.clone()
    }
}
