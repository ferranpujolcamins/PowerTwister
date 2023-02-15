use midir::{MidiInput, MidiInputPort};

mod state_machine;

const CONTROL_DEVICE_NAME: &str = "Midi Fighter Twister";
fn main() {
    let midi_in = MidiInput::new("midir test input").unwrap();

    println!("Connecting to {}...", CONTROL_DEVICE_NAME);
    let mut control_device_port: Option<MidiInputPort> = None;
    for p in midi_in.ports().iter() {
        if midi_in.port_name(p).unwrap().starts_with(CONTROL_DEVICE_NAME) {
            control_device_port = Some(p.clone());
        }
    }

    let _connection = midi_in.connect(&(control_device_port.unwrap()), "control_input_conection", move |stamp, message, _| {
        println!("{}: {:?} (len = {})", stamp, message, message.len());
    }, ());

    loop {};
}
// TODO: setup rustfmt
