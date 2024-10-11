use std::thread::{JoinHandle, spawn};
use pipewire::main_loop::MainLoop;
use pipewire::context::Context;
use pipewire::core::Core;
use pipewire::port;
use pipewire::registry::GlobalObject;
use pipewire::spa::pod::Object;
use pipewire::spa::utils::dict::DictRef;
use pipewire::types::ObjectType;



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


pub fn audio_main() {
    // set up all of the audio stuff we need
    let mut ar = AudioReceiver::new();
        
    // build our main loop to receive messages from pipewire
    pipewire::init();
    let mainloop = MainLoop::new(None).unwrap();
    let context: Context = Context::new(&mainloop).unwrap();
    let core: Core = context.connect(None).unwrap();
    let registry = core.get_registry().unwrap();    

    let _listener = registry
        .add_listener_local()
        .global(|global|  {
            if global.type_ == ObjectType::Port {
                let props = global.props.as_ref().unwrap();
                let port_name = props.get("port.name").unwrap();
                if port_name == "monitor_FR" || port_name == "monitor_FL" {
                    let port_alias = props.get("port.alias");
                    let object_path = props.get("object.path");
                    let format_dsp = props.get("format.dsp");
                    let audio_channel = props.get("audio.channel");
                    let port_id = props.get("port.id");
                    let port_direction = props.get("port.direction");
                    println!("Port: Name: {:?} Alias: {:?}  Id: {:?} Direction: {:?} AudioChannel: {:?} Object Path: {:?} FormatDsp: {:?}",
                        port_name,
                        port_alias,
                        port_id,port_direction,audio_channel,object_path,format_dsp
                    );
                }
            } else if global.type_ == ObjectType::Node {
                let props = global.props.as_ref().unwrap();
                println!("{:?}", props)
            }
        })
    .register();

    // create a new node and endpoints
    // look here: https://gitlab.freedesktop.org/pipewire/pipewire-rs/-/blob/main/pipewire/examples/audio-capture.rs?ref_type=heads

    

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
    let mut props = pipewire::properties::properties! {
        *pipewire::keys::MEDIA_TYPE => "Audio",
        *pipewire::keys::MEDIA_CATEGORY => "Capture",
        *pipewire::keys::MEDIA_ROLE => "Music",
    };

    props.insert(*pipewire::keys::STREAM_CAPTURE_SINK, "true");

    // uncomment if you want to capture from the sink monitor ports
    // props.insert(*pw::keys::STREAM_CAPTURE_SINK, "true");

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

            println!(
                "capturing rate:{} channels:{}",
                user_data.format.rate(),
                user_data.format.channels()
            );
        })
        .process(|stream, user_data| match stream.dequeue_buffer() {
            None => println!("out of buffers"),
            Some(mut buffer) => {
                let datas = buffer.datas_mut();
                if datas.is_empty() {
                    return;
                }

                let data = &mut datas[0];
                let n_channels = user_data.format.channels();
                let n_samples = data.chunk().size() / (std::mem::size_of::<f32>() as u32);

                if let Some(samples) = data.data() {
                    if user_data.cursor_move {
                        print!("\x1B[{}A", n_channels + 1);
                    }
                    println!("captured {} samples", n_samples / n_channels);
                    for c in 0..n_channels {
                        let mut max: f32 = 0.0;
                        for n in (c..n_samples).step_by(n_channels as usize) {
                            let start = n as usize * std::mem::size_of::<f32>();
                            let end = start + std::mem::size_of::<f32>();
                            let chan = &samples[start..end];
                            let f = f32::from_le_bytes(chan.try_into().unwrap());
                            max = max.max(f.abs());
                        }

                        let peak = ((max * 30.0) as usize).clamp(0, 39);

                        println!(
                            "channel {}: |{:>w1$}{:w2$}| peak:{}",
                            c,
                            "*",
                            "",
                            max,
                            w1 = peak + 1,
                            w2 = 40 - peak
                        );
                    }
                    user_data.cursor_move = true;
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

pub fn spawn_audio_thread() -> JoinHandle<()> {
    spawn(||{
        audio_main();
    })
}
