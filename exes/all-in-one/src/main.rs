use all_in_one::ffi::{create_instance, load_config, stop_instance};
use std::sync::mpsc::channel;
use ctrlc;

fn main() {
    let c = load_config();
    let comp = create_instance(c);

    // wait for signal
    let (tx, rx) = channel();
    
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    rx.recv().unwrap();

    println!("Exiting.");

    stop_instance(comp);
}