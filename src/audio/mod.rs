use std::thread::{JoinHandle, spawn};
use pipewire::main_loop::MainLoop;
use pipewire::context::Context;
use pipewire::core::Core;
use pipewire::port;
use pipewire::registry::GlobalObject;
use pipewire::spa::pod::Object;
use pipewire::spa::utils::dict::DictRef;
use pipewire::types::ObjectType;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use log::{info, debug, warn, error};




pub struct AudioReceiver{

}

impl AudioReceiver{

    pub fn new() -> Self {
        Self{}
    }

}

// fn create_connection(target: &DictRef) -> 

struct UserData {
    format: libspa::param::audio::AudioInfoRaw,
    cursor_move: bool,
}


struct CurrentStore {
    channel_zero: [f32; 512],
    channel_one: [f32; 512]
}

pub fn audio_main(
    data_sender: mpsc::Sender<([f32; 512], [f32; 512])>,
    requester: pipewire::channel::Receiver<()>
) {
    // build our main loop to receive messages from pipewire
    pipewire::init();
    let mainloop = MainLoop::new(None).unwrap();
    // whenever we get a request for data, send back dual channel f32LE data
    let store = Arc::new(Mutex::new(CurrentStore{channel_zero: [0.0; 512], channel_one: [0.0; 512]}));
    let _receiver = requester.attach(mainloop.loop_(), {
        let store = store.clone();
        move |_| {
            let store = store.lock().unwrap();
            debug!("Audio receiver: Trying to send {:?} {:?}", store.channel_zero, store.channel_one);
            data_sender.send((store.channel_zero, store.channel_one)).unwrap();
        }
    });
    let context: Context = Context::new(&mainloop).unwrap();
    let core: Core = context.connect(None).unwrap();    

    let data = UserData {
        format: Default::default(),
        cursor_move: false,
    };

    /* Create a simple stream, the simple stream manages the core and remote
     * objects for you if you don't need to deal with them.
     *
     * If you plan to autoconnect your stream, you need to provide at least
     * media, category and role properties.
     *
     * Pass your events and a user_data pointer as the last arguments. This
     * will inform you about the stream state. The most important event
     * you need to listen to is the process event where you need to produce
     * the data.
     */
    let props = pipewire::properties::properties! {
        *pipewire::keys::MEDIA_TYPE => "Audio",
        *pipewire::keys::MEDIA_CATEGORY => "Capture",
        *pipewire::keys::MEDIA_ROLE => "Music",
        *pipewire::keys::STREAM_CAPTURE_SINK => "true"
    }; 

    let stream = pipewire::stream::Stream::new(&core, "audio-capture", props).unwrap();

    let _listener = stream
        .add_local_listener_with_user_data(data)
        .param_changed(|_, user_data, id, param| {
            // NULL means to clear the format
            let Some(param) = param else {
                return;
            };
            if id != pipewire::spa::param::ParamType::Format.as_raw() {
                return;
            }

            let (media_type, media_subtype) = match libspa::param::format_utils::parse_format(param) {
                Ok(v) => v,
                Err(_) => return,
            };

            // only accept raw audio
            if media_type != libspa::param::format::MediaType::Audio || media_subtype != libspa::param::format::MediaSubtype::Raw {
                return;
            }

            // call a helper function to parse the format for us.
            user_data
                .format
                .parse(param)
                .expect("Failed to parse param changed to AudioInfoRaw");

            info!(
                "capturing rate:{} channels:{}",
                user_data.format.rate(),
                user_data.format.channels()
            );
        })
        .process(move |stream, user_data| match stream.dequeue_buffer() {
            None => warn!("out of buffers"),
            Some(mut buffer) => {
                // pull data out of buffer
                let datas = buffer.datas_mut();
                if datas.is_empty() {
                    return;
                }

                // looking only at the first chunk of data, parse # channels and samples
                let data = &mut datas[0];
                let n_channels = user_data.format.channels();
                let n_samples = data.chunk().size() / (std::mem::size_of::<f32>() as u32);

                // don't update stuff if we don't have enough samples
                if n_samples < 512 {
                    return;
                }
                
                // make sure we have actually gotten data
                if let Some(samples) = data.data() {
                    // for each one of the received channels...
                    for c in 0..n_channels {
                        // for each sample index for that channel...
                        for n in (c..n_samples).step_by(n_channels as usize) {
                            // parse the f32 value for it
                            let start = n as usize * std::mem::size_of::<f32>();
                            let end = start + std::mem::size_of::<f32>();
                            let chan = &samples[start..end];
                            let f = f32::from_le_bytes(chan.try_into().unwrap());
                            let mut store = store.lock().unwrap();
                            if n < 512 * 2 {
                                match c {
                                    0 => store.channel_zero[n as usize/2] = f,
                                    1 => store.channel_one[(n as usize - 1)/2] = f,
                                    _ => debug!("Ignoring channel {c}")
                                }
                            }
                        }
                    }
                }
            }
        })
        .register().unwrap();

    /* Make one parameter with the supported formats. The SPA_PARAM_EnumFormat
     * id means that this is a format enumeration (of 1 value).
     * We leave the channels and rate empty to accept the native graph
     * rate and channels. */
    let mut audio_info = pipewire::spa::param::audio::AudioInfoRaw::new();
    audio_info.set_format(pipewire::spa::param::audio::AudioFormat::F32LE);
    let obj = pipewire::spa::pod::Object {
        type_: pipewire::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
        id: pipewire::spa::param::ParamType::EnumFormat.as_raw(),
        properties: audio_info.into(),
    };
    let values: Vec<u8> = pipewire::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pipewire::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();

    let mut params = [libspa::pod::Pod::from_bytes(&values).unwrap()];

    /* Now connect this stream. We ask that our process function is
     * called in a realtime thread. */
    stream.connect(
        libspa::utils::Direction::Input,
        None,
        pipewire::stream::StreamFlags::AUTOCONNECT
            | pipewire::stream::StreamFlags::MAP_BUFFERS
            | pipewire::stream::StreamFlags::RT_PROCESS,
        &mut params,
    ).unwrap();


            
    // run it
    mainloop.run();

}

pub fn spawn_audio_thread(
    data_sender: mpsc::Sender<([f32; 512], [f32; 512])>,
    requester: pipewire::channel::Receiver<()>
) -> JoinHandle<()> {
    spawn(move ||{
        audio_main(data_sender, requester);
    })
}
