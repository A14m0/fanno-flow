use std::thread::{JoinHandle, spawn};
use pipewire::main_loop::MainLoop;
use pipewire::context::Context;

pub struct AudioReceiver{

}

impl AudioReceiver{

    pub fn new() -> Self {
        Self{}
    }

}

pub fn audio_main() {
    // set up all of the audio stuff we need
    let mut ar = AudioReceiver::new();
        
    // build our main loop to receive messages from pipewire
    let mainloop = MainLoop::new(None).unwrap();
    let context = Context::new(&mainloop).unwrap();
    let core = context.connect(None).unwrap();
    let registry = core.get_registry().unwrap();    

    let _listener = registry
        .add_listener_local()
        .global(|global| println!("New global: {:?}", global))
        .register();
            
    // run it
    mainloop.run();

}

pub fn spawn_audio_thread() -> JoinHandle<()> {
    spawn(||{
        audio_main();
    })
}
