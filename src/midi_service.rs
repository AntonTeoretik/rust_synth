use std::sync::{Arc, Mutex};
use midir::{MidiInput, Ignore, MidiInputConnection};

pub struct MidiService {
    is_gate_open: Arc<Mutex<bool>>,
    active_notes: Arc<Mutex<Vec<u8>>>,
    last_note: Arc<Mutex<Option<u8>>>,
    midi_connection: Arc<Mutex<Option<MidiInputConnection<()>>>>,
}

impl MidiService {
    pub fn new() -> Arc<Self> {
        let is_gate_open = Arc::new(Mutex::new(false));
        let active_notes = Arc::new(Mutex::new(Vec::new()));
        let last_note = Arc::new(Mutex::new(None));
        let midi_connection = Arc::new(Mutex::new(None));

        let service = Arc::new(Self {
            is_gate_open: Arc::clone(&is_gate_open),
            active_notes: Arc::clone(&active_notes),
            last_note: Arc::clone(&last_note),
            midi_connection: Arc::clone(&midi_connection),
        });

        Self::start_midi_listener(Arc::clone(&service));

        service
    }

    fn start_midi_listener(service: Arc<Self>) {
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
        let connection = midi_in.connect(
            port,
            "midi_service",
            move |_, message, _| {
                if message.len() < 3 {
                    return;
                }
                let status = message[0];
                let note = message[1];
                let velocity = message[2];

                let mut active_notes = service_clone.active_notes.lock().unwrap();
                let mut is_gate_open = service_clone.is_gate_open.lock().unwrap();

                if (status & 0xF0 == 0x90) && (status & 0x0F == 0) && velocity > 0 {
                    if !active_notes.contains(&note) {
                        active_notes.push(note);
                        *service_clone.last_note.lock().unwrap() = Some(note);
                    }
                    *is_gate_open = true;
                } else if ((status & 0xF0 == 0x80) || ((status & 0xF0 == 0x90) && velocity == 0))
                    && (status & 0x0F == 0)
                {
                    if let Some(pos) = active_notes.iter().position(|&n| n == note) {
                        active_notes.remove(pos);
                    }

                    if let Some(&new_note) = active_notes.last() {
                        *service_clone.last_note.lock().unwrap() = Some(new_note);
                    } else {
                        *service_clone.last_note.lock().unwrap() = None;
                        *is_gate_open = false;
                    }
                }
            },
            (),
        );

        *service.midi_connection.lock().unwrap() = connection.ok();
    }

    pub fn is_open(&self) -> bool {
        *self.is_gate_open.lock().unwrap()
    }

    pub fn last_note(&self) -> Option<u8> {
        *self.last_note.lock().unwrap()
    }

    pub fn active_notes(&self) -> Vec<u8> {
        self.active_notes.lock().unwrap().clone()
    }
}
