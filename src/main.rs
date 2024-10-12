mod audio;


use std::sync::mpsc;
use pipewire::channel::Receiver;

use std::io::{stdin,stdout,Write};
use log::info;

fn main() {
    // initialize our logger
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    // set up MPSC channels for getting data from the audio thread
    let (data_sender, data_receiver) = mpsc::channel();
    let (request_sender, request_receiver) = pipewire::channel::channel();

    // spawn the audio backend thread
    let audio_thread = audio::spawn_audio_thread(data_sender, request_receiver);

    // DEMO: Wait for user input and get data from the audio channel
    loop {
        // wait for user to enter 
        let mut _s = String::new();
        let _=stdout().flush();
        stdin().read_line(&mut _s).expect("Failed to get string");

        // request the data from the audio backend
        request_sender.send(()).unwrap();
        
        // wait for the data to be received
        let out = data_receiver.recv();

        info!("Received: {:?}", out);

    }
}
