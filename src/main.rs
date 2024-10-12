mod audio;


use std::sync::mpsc;
use pipewire::channel::Receiver;
use realfft::RealFftPlanner;

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
        let out = data_receiver.recv().unwrap();

        info!("Received: {:?}", out);

        // try to build an FFT planner
        let mut real_planner = RealFftPlanner::<f32>::new();

        // build the FFT
        let r2c = real_planner.plan_fft_forward(512);
        let mut indat = r2c.make_input_vec();
        for v in 0..512 {
            indat[v] = out.0[v];
        }
        let mut spectrum = r2c.make_output_vec();

        // forward transform
        r2c.process(&mut indat, &mut spectrum).unwrap();

        // create inverse FFT
        let c2r = real_planner.plan_fft_inverse(512);
        let mut outdata = c2r.make_output_vec();
        c2r.process(&mut spectrum, &mut outdata).unwrap();

        info!("Input data: {:?}", indat);
        info!("Forward FFT: {:?}", spectrum);
        info!("Reverse FFT output: {:?}", outdata);

    }
}
